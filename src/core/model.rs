use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use bitvec::field::BitField;
use bitvec::prelude::{BitVec, Lsb0};
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use daggy::petgraph::dot::Dot;
use daggy::{walker::Recursive, Dag, EdgeIndex, NodeIndex, Walker};
use rand::Rng;

/// APIs for emulating a System
pub mod exec;

/// APIs for building and manipulating the instruction intermediate representation
pub mod ir;
use ir::ProcIRCtx;

/// Re-export of everything needed to model a peripheral device
pub mod dev {
    pub use super::{
        DevCtx, DevDecl, DevId, DevInfo, Device, IntrInput, MemParent, MemReadResult,
        MemWriteResult, TimerExpiryResult, TimerId,
    };
}

/// Re-export of everything needed to model a processor
pub mod proc {
    pub use super::{
        Assert, IsTrue, MemReadResult, MemWriteResult, ProcCtx, ProcData, ProcDecl, ProcId,
        ProcInfo, Processor, Register, RegisterArray, RegisterArrayView, RegisterView,
    };
}

/// Provides the API for defining a System and initializing it.
pub struct System<P: Processor> {
    name: String,
    procs: Vec<P>,
    proc_names: Vec<String>,
    proc_data: Vec<ProcData<P>>,
    ir_ctxs: Vec<ir::ProcIRCtx>,
    devs: Vec<Box<dyn Any>>,
    dev_names: Vec<String>,
    mem: MemoryMap,
    intrs: InterruptTree<P>,
}

impl<P: Processor> System<P> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            procs: Vec::new(),
            proc_names: Vec::new(),
            proc_data: Vec::new(),
            ir_ctxs: Vec::new(),
            devs: Vec::new(),
            dev_names: Vec::new(),
            mem: MemoryMap::new(),
            intrs: InterruptTree::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn proc(&self, id: ProcId) -> Option<&ProcData<P>> {
        self.proc_data.get(id)
    }

    pub fn proc_mut(&mut self, id: ProcId) -> Option<&mut ProcData<P>> {
        self.proc_data.get_mut(id)
    }

    pub fn sysbus(&self) -> MemParent {
        self.mem.root()
    }

    pub fn add_proc<S>(&mut self, name: S, proc: P) -> ProcInfo<P>
    where
        S: Into<String>,
    {
        let proc_id = self.procs.len();
        let proc_name = name.into();
        let mut decl = ProcDecl::new(proc_id);
        proc.declare(&mut decl);

        // Add the processor itself
        self.procs.push(proc);
        self.proc_names.push(proc_name.clone());

        // Add its registers
        let mut reg_offsets = HashMap::new();
        let mut total_reg_bits = 0;
        for (type_id, (_name, len)) in decl.regs {
            reg_offsets.insert(type_id, total_reg_bits);
            total_reg_bits += len;
        }
        for (type_id, (_name, elems, elem_len)) in decl.reg_arrays {
            reg_offsets.insert(type_id, total_reg_bits);
            total_reg_bits += elems * elem_len;
        }
        let mut reg_data = BitVec::with_capacity(total_reg_bits);
        reg_data.resize(total_reg_bits, false);
        self.proc_data.push(ProcData {
            reg_data,
            reg_offsets: reg_offsets.clone(),
            reg_mapped: HashMap::new(),
            tlb: HashMap::new(),
            rng_fn: None,
            count: 0,
        });
        self.ir_ctxs.push(ProcIRCtx {
            offsets: reg_offsets,
        });

        // Add its interrupt inputs and collect them for the ProcInfo
        let mut intr_inputs = HashMap::new();
        let mut proc_nodes = Vec::new();
        for (name, n, handler, downcast) in decl.intr_inputs {
            let idx = self.intrs.tree.add_node(IntrIface::ProcInput {
                proc: proc_id,
                name: name.clone(),
                handler,
            });
            self.intrs.proc_downcasts.insert(idx, downcast);
            proc_nodes.push(idx);
            intr_inputs.insert(name, (n, idx));
        }
        self.intrs.proc_nodes.insert(proc_id, proc_nodes);

        ProcInfo {
            id: proc_id,
            name: proc_name,
            intr_inputs,
            p: PhantomData,
        }
    }

    pub fn add_dev<S, D>(&mut self, name: S, dev: D) -> DevInfo
    where
        S: Into<String>,
        D: Device,
    {
        let dev_id = self.devs.len();
        let dev_name = name.into();
        let mut decl = DevDecl::new(dev_id);
        dev.declare(&mut decl);

        // Add the device itself and a function
        let dev = Box::new(dev);
        self.devs.push(dev);
        self.dev_names.push(dev_name.clone());

        // Add any register mappings this device creates with a particular processor
        for (proc_id, reg, mapping) in decl.reg_mapped {
            self.proc_data
                .get_mut(proc_id)
                .unwrap()
                .reg_mapped
                .insert(reg, mapping);
        }

        // Add its memory regions and collect any containers to be placed in the DevInfo
        let mut containers = HashMap::new();
        for (parent, range, region, downcast_fns) in decl.mem_regions {
            let (_, idx) = self.mem.map.add_child(parent.0, range, region);
            if let Some((r_downcast, w_downcast)) = downcast_fns {
                self.mem
                    .mmio_downcasts
                    .insert(idx, (r_downcast, w_downcast));
            }

            if let MemRegion::Container { name, .. } = self.mem.map.node_weight(idx).unwrap() {
                containers.insert(name.clone(), idx);
            }
        }

        // Add its interrupt connections
        for input in decl.intr_conns {
            self.intrs.tree.add_child(
                input.idx,
                IntrConn {
                    level: false,
                    n: input.n,
                },
                IntrIface::DevOutput(dev_id),
            );
        }

        // Add its interrupt input pins and collect them for the DevInfo
        let mut intr_inputs = HashMap::new();
        for (name, num_pins, handler, downcast) in decl.intr_inputs {
            let idx = self.intrs.tree.add_node(IntrIface::DevInput {
                dev: dev_id,
                name: name.clone(),
                num_pins,
                handler,
            });
            self.intrs.dev_downcasts.insert(idx, downcast);
            intr_inputs.insert(name, (num_pins, idx));
        }

        return DevInfo {
            name: dev_name.clone(),
            containers,
            intr_inputs,
        };
    }

    pub fn add_ram(&mut self, name: impl Into<String>, base: usize, size: usize) {
        let mut mem = Vec::with_capacity(size);
        mem.resize(size, 0);
        self.mem.map.add_child(
            self.mem.root,
            AddrRange { base, size },
            MemRegion::Ram {
                dev: None,
                name: name.into(),
                mem,
            },
        );
    }

    pub fn add_rom(&mut self, name: impl Into<String>, base: usize, size: usize) {
        let mut mem = Vec::with_capacity(size);
        mem.resize(size, 0);
        self.mem.map.add_child(
            self.mem.root,
            AddrRange { base, size },
            MemRegion::Rom {
                dev: None,
                name: name.into(),
                mem,
            },
        );
    }

    pub fn init_mem(&mut self, addr: usize, data: &[u8]) {
        let (addr_range, region) = self
            .mem
            .route_to_leaf(addr)
            .expect("Address not in the memory map.");

        let base = self.mem.map.edge_weight(addr_range).unwrap().base;
        let offset = addr - base;
        let region = self.mem.map.node_weight_mut(region).unwrap();

        if let MemRegion::Ram { mem, .. } = region {
            mem[offset..(offset + data.len())].copy_from_slice(data);
        } else if let MemRegion::Rom { mem, .. } = region {
            mem[offset..(offset + data.len())].copy_from_slice(data);
        } else {
            panic!("Only RAM or ROM regions can be initialized with byte data.");
        }
    }
}

// Processor model

pub trait Processor: 'static {
    fn declare(&self, decl: &mut ProcDecl<Self>)
    where
        Self: Sized;

    fn ip(&self, ctx: &mut ProcCtx<Self>) -> usize
    where
        Self: Sized;
    fn set_ip(&self, ctx: &mut ProcCtx<Self>, addr: usize)
    where
        Self: Sized;

    type Insn: Sized;
    fn fetch_decode<'a>(
        &self,
        addr: usize,
        ir_ctx: &ProcIRCtx,
        ctx: &mut ProcCtx<'a, Self>,
    ) -> Option<(Self::Insn, ir::IRGraph<Self>)>
    where
        Self: Sized;

    type BrAttrs;
    fn br<'a>(&self, addr: usize, attrs: Self::BrAttrs, ctx: &mut ProcCtx<'a, Self>)
    where
        Self: Sized;
    fn cbr<'a>(
        &self,
        cond: bool,
        t_addr: usize,
        f_addr: usize,
        attrs: Self::BrAttrs,
        ctx: &mut ProcCtx<'a, Self>,
    ) where
        Self: Sized;

    type LdstAttrs;
    fn ld<'a>(
        &self,
        addr: usize,
        size: usize,
        attrs: Self::LdstAttrs,
        ctx: &mut ProcCtx<'a, Self>,
    ) -> MemReadResult
    where
        Self: Sized;
    fn st<'a>(
        &self,
        addr: usize,
        data: &[u8],
        attrs: Self::LdstAttrs,
        ctx: &mut ProcCtx<'a, Self>,
    ) -> MemWriteResult
    where
        Self: Sized;

    type TLBKey: Hash + Eq + PartialEq;
    type TLBValue;

    type ExcepAttrs;
    fn handle_excep<'a>(&self, excep: Self::ExcepAttrs, ctx: &mut ProcCtx<'a, Self>)
    where
        Self: Sized;
}

pub struct ProcInfo<P: Processor> {
    id: ProcId,
    name: String,
    // TODO: a list of externally-mappable registers?
    intr_inputs: HashMap<String, (u32, NodeIndex)>,
    p: PhantomData<P>,
}

impl<P: Processor> ProcInfo<P> {
    pub fn id(&self) -> ProcId {
        self.id
    }

    pub fn intr_in(&self, name: impl AsRef<str>, n: u32) -> IntrInput {
        self.intr_inputs
            .get(name.as_ref())
            .and_then(|(num_pins, idx)| {
                // If pin name exists, check if n is in range of the pin numbers
                if n >= *num_pins {
                    None
                } else {
                    Some(IntrInput {
                        idx: idx.clone(),
                        n: n.clone(),
                    })
                }
            })
            .unwrap_or_else(|| {
                panic!(
                    "Unknown interrupt input for processor {0}: [{1}:{n}]",
                    self.name,
                    name.as_ref()
                )
            })
    }
}

pub type ProcId = usize;

pub struct ProcDecl<P: Processor> {
    id: ProcId,
    regs: HashMap<TypeId, (&'static str, usize)>,
    reg_arrays: HashMap<TypeId, (&'static str, usize, usize)>,
    intr_inputs: Vec<(String, u32, Box<dyn Any>, DowncastAndCallProcIntr<P>)>,
    p: PhantomData<P>,
}

impl<P: Processor> ProcDecl<P> {
    fn new(id: usize) -> Self {
        Self {
            id,
            regs: HashMap::new(),
            reg_arrays: HashMap::new(),
            intr_inputs: Vec::new(),
            p: PhantomData,
        }
    }

    pub fn reg<T: Register>(&mut self) -> &mut Self
    where
        Assert<{ T::LEN > 0 }>: IsTrue,
    {
        let prev = self.regs.insert(TypeId::of::<T>(), (T::NAME, T::LEN));
        if prev.is_some() {
            panic!("Duplicate Register declaration for {0}.", T::NAME);
        }
        self
    }

    pub fn reg_array<T: RegisterArray>(&mut self) -> &mut Self
    where
        Assert<{ T::LEN > 0 }>: IsTrue,
    {
        let prev = self
            .reg_arrays
            .insert(TypeId::of::<T>(), (T::NAME, T::ELEMS, T::ELEM_LEN));
        if prev.is_some() {
            panic!("Duplicate RegisterArray declaration for {0}.", T::NAME);
        }
        self
    }

    pub fn intr_in<S, H>(&mut self, name: S, n: u32, handler: H) -> &mut Self
    where
        S: Into<String>,
        H: 'static + for<'a> FnMut(&P, u32, bool, &mut ProcCtx<'a, P>) -> bool,
    {
        self.intr_inputs.push((
            name.into(),
            n,
            Box::new(handler),
            Box::new(|erased_fn, erased_proc, n, level, ctx| {
                let handler_fn = erased_fn
                    .downcast_mut::<H>()
                    .expect("Failed to downcast Proc interrupt handler to original type.");

                let proc = erased_proc
                    .downcast_ref::<P>()
                    .expect("Failed to downcast Proc struct to original type.");

                handler_fn(proc, n, level, ctx)
            }),
        ));
        self
    }
}

pub trait Register: 'static {
    const NAME: &'static str;
    const LEN: usize;
}

pub trait RegisterView: 'static {
    type Register: Register;
    const NAME: &'static str;
    const OFFSET: usize;
    const LEN: usize;
}

impl<T: Register> RegisterView for T {
    type Register = Self;
    const NAME: &'static str = <Self as Register>::NAME;
    const OFFSET: usize = 0;
    const LEN: usize = <Self as Register>::LEN;
}

pub trait RegisterArray: 'static {
    const NAME: &'static str;
    const ELEMS: usize;
    const ELEM_LEN: usize;
}

pub trait RegisterArrayView: 'static {
    type RegisterArray: RegisterArray;
    const NAME: &'static str;
    const OFFSET: usize;
    const LEN: usize;
}

impl<T: RegisterArray> RegisterArrayView for T {
    type RegisterArray = Self;
    const NAME: &'static str = <Self as RegisterArray>::NAME;
    const OFFSET: usize = 0;
    const LEN: usize = <Self as RegisterArray>::ELEM_LEN;
}

// Use these to validate conditions on Register/RegisterView associated constants at compile-time.
// TODO: I think these can be circumvented if a user does impl IsTrue for Assert<false> {}. Let's see if we can prevent this
// Hopefully const_generic_exprs will eventually make this easier: #76560 <https://github.com/rust-lang/rust/issues/76560>

pub struct Assert<const P: bool>;
pub trait IsTrue {}
impl IsTrue for Assert<true> {}

pub struct ProcData<P: Processor> {
    reg_data: BitVec<usize, Lsb0>,
    reg_offsets: HashMap<TypeId, usize>,
    reg_mapped: HashMap<TypeId, RegMapping>,
    tlb: HashMap<P::TLBKey, P::TLBValue>,
    rng_fn: Option<fn() -> u64>,
    count: u64,
}

impl<P: Processor> ProcData<P> {
    pub fn read<T>(&self) -> u64
    where
        T: RegisterView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        // The (uncommon) case of a register being mapped to a device, e.g. the ARM generic timer and interrrupt controller
        if self.reg_mapped.contains_key(&TypeId::of::<T::Register>()) {
            panic!("Cannot read from device-mapped registers from the ProcData struct!");
        } else {
            // The (common) case of a register corresponding to concrete binary data
            let offset = self
                .reg_offsets
                .get(&TypeId::of::<T::Register>())
                .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
                .clone();

            self.reg_data
                .get((offset + T::OFFSET)..(offset + T::OFFSET + T::LEN))
                .unwrap()
                .load()
        }
    }

    pub fn write<T>(&mut self, val: u64)
    where
        T: RegisterView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        // The (uncommon) case of a register being mapped to a device, e.g. the ARM generic timer and interrrupt controller
        if self.reg_mapped.contains_key(&TypeId::of::<T::Register>()) {
            panic!("Cannot write to device-mapped registers from the ProcData struct!");
        } else {
            // The (common) case of a register corresponding to concrete binary data
            let offset = self
                .reg_offsets
                .get(&TypeId::of::<T::Register>())
                .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
                .clone();
            self.reg_data
                .get_mut((offset + T::OFFSET)..(offset + T::OFFSET + T::LEN))
                .unwrap()
                .store(val);
        }
    }

    pub fn read_ext<T>(&mut self) -> u128 {
        todo!()
    }

    pub fn write_ext<T>(&mut self, _val: u128) {
        todo!()
    }

    pub fn read_elem<T>(&self, idx: usize) -> u64
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        let offset = self
            .reg_offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }

        self.reg_data
            .get(
                (offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
                    ..(offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET + T::LEN),
            )
            .unwrap()
            .load()
    }

    pub fn write_elem<T>(&mut self, idx: usize, val: u64)
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        let offset = self
            .reg_offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }

        self.reg_data
            .get_mut(
                (offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
                    ..(offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET + T::LEN),
            )
            .unwrap()
            .store(val);
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn set_rng(&mut self, f: fn() -> u64) {
        self.rng_fn = Some(f);
    }
}

pub struct ProcCtx<'a, P: Processor> {
    reg_data: &'a mut BitSlice<usize, Lsb0>,
    reg_offsets: &'a HashMap<TypeId, usize>,
    reg_mapped: &'a mut HashMap<TypeId, RegMapping>,
    mem: &'a mut MemoryMap,
    devs: &'a mut [Box<dyn Any>],
    tlb: &'a mut HashMap<P::TLBKey, P::TLBValue>,
    count: u64,
    rng_fn: Option<fn() -> u64>,
    new_dev_ctxs: Vec<DevCtx>,
}

impl<'a, P: Processor> ProcCtx<'a, P> {
    pub fn read<T>(&mut self) -> u64
    where
        T: RegisterView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        // The (uncommon) case of a register being mapped to a device, e.g. the ARM generic timer and interrrupt controller
        if let Some(reg_map) = self.reg_mapped.get_mut(&TypeId::of::<T::Register>()) {
            // println!("Dev reg mapping for a RT read!");

            let mut dev = self.devs.get_mut(reg_map.dev);
            let dev = dev.as_mut().unwrap().as_mut();
            let mut dev_ctx = DevCtx {
                id: reg_map.dev,
                new_events: Vec::new(),
                count: self.count,
            };
            let data = (reg_map.read_dac)(reg_map.read_fn.as_mut(), dev, &mut dev_ctx);
            self.new_dev_ctxs.push(dev_ctx);
            data
        } else {
            // The (common) case of a register corresponding to concrete binary data
            let offset = self
                .reg_offsets
                .get(&TypeId::of::<T::Register>())
                .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
                .clone();

            self.reg_data
                .get((offset + T::OFFSET)..(offset + T::OFFSET + T::LEN))
                .unwrap()
                .load()
        }
    }

    pub fn write<T>(&mut self, val: u64)
    where
        T: RegisterView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        // The (uncommon) case of a register being mapped to a device, e.g. the ARM generic timer and interrrupt controller
        if let Some(reg_map) = self.reg_mapped.get_mut(&TypeId::of::<T::Register>()) {
            // println!("Dev reg mapping for a RT write!");

            let mut dev = self.devs.get_mut(reg_map.dev);
            let dev = dev.as_mut().unwrap().as_mut();
            let mut dev_ctx = DevCtx {
                id: reg_map.dev,
                new_events: Vec::new(),
                count: self.count,
            };
            (reg_map.write_dac)(reg_map.write_fn.as_mut(), dev, val, &mut dev_ctx);
            self.new_dev_ctxs.push(dev_ctx);
        } else {
            // The (common) case of a register corresponding to concrete binary data
            let offset = self
                .reg_offsets
                .get(&TypeId::of::<T::Register>())
                .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
                .clone();
            self.reg_data
                .get_mut((offset + T::OFFSET)..(offset + T::OFFSET + T::LEN))
                .unwrap()
                .store(val);
        }
    }

    pub fn read_ext<T>(&mut self) -> u128 {
        todo!()
    }

    pub fn write_ext<T>(&mut self, _val: u128) {
        todo!()
    }

    pub fn read_elem<T>(&self, idx: usize) -> u64
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        let offset = self
            .reg_offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }

        self.reg_data
            .get(
                (offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
                    ..(offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET + T::LEN),
            )
            .unwrap()
            .load()
    }

    pub fn write_elem<T>(&mut self, idx: usize, val: u64)
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        let offset = self
            .reg_offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }

        self.reg_data
            .get_mut(
                (offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
                    ..(offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET + T::LEN),
            )
            .unwrap()
            .store(val);
    }

    pub fn mem_read(&mut self, addr: usize, size: usize) -> MemReadResult {
        let leaf = self.mem.route_to_leaf(addr);
        if leaf.is_none() {
            panic!("Could not route address 0x{addr:x} to leaf node.");
            // return MemReadResult(None);
        }
        let (leaf_e, leaf_n) = leaf.unwrap();

        let region_base = self.mem.map.edge_weight(leaf_e).unwrap().base;
        let region = self.mem.map.node_weight_mut(leaf_n).unwrap();
        match region {
            MemRegion::Mmio {
                dev: dev_id,
                read_fn,
                ..
            } => {
                let dev = self.devs.get_mut(*dev_id).unwrap().as_mut();
                let offset = addr - region_base;
                let (downcast_and_call, _) = self
                    .mem
                    .mmio_downcasts
                    .get(&leaf_n)
                    .expect("Failed to locate MMIO read downcast-and-call fn.");
                self.new_dev_ctxs.push(DevCtx {
                    id: dev_id.clone(),
                    new_events: Vec::new(),
                    count: self.count,
                });
                let data = downcast_and_call(
                    &mut (**read_fn),
                    dev,
                    offset,
                    size,
                    self.new_dev_ctxs.last_mut().unwrap(),
                );
                data
            }
            MemRegion::Ram { mem, .. } => {
                let offset = addr - region_base;
                let res = mem.get(offset..(offset + size)).and_then(|data| {
                    let mut res = [0; 16];
                    (&mut res[0..size]).copy_from_slice(&data);
                    Some(res)
                });
                MemReadResult(res)
            }
            MemRegion::Rom { mem, .. } => {
                let offset = addr - region_base;
                let res = mem.get(offset..(offset + size)).and_then(|data| {
                    let mut res = [0; 16];
                    (&mut res[0..size]).copy_from_slice(&data);
                    Some(res)
                });
                MemReadResult(res)
            }
            _ => panic!("Unreachable"),
        }
    }

    pub fn mem_write(&mut self, addr: usize, data: &[u8]) -> MemWriteResult {
        let leaf = self.mem.route_to_leaf(addr);
        if leaf.is_none() {
            return MemWriteResult(None);
        }
        let (leaf_e, leaf_n) = leaf.unwrap();

        let region_base = self.mem.map.edge_weight(leaf_e).unwrap().base;
        let region = self.mem.map.node_weight_mut(leaf_n).unwrap();
        match region {
            MemRegion::Mmio {
                dev: dev_id,
                write_fn,
                ..
            } => {
                let offset = addr - region_base;
                let dev = self.devs.get_mut(*dev_id).unwrap().as_mut();
                let (_, downcast_and_call) = self
                    .mem
                    .mmio_downcasts
                    .get(&leaf_n)
                    .expect("Failed to locate MMIO write downcast-and-call fn.");
                self.new_dev_ctxs.push(DevCtx {
                    id: dev_id.clone(),
                    new_events: Vec::new(),
                    count: self.count,
                });
                downcast_and_call(
                    &mut (**write_fn),
                    dev,
                    offset,
                    data,
                    self.new_dev_ctxs.last_mut().unwrap(),
                )
            }
            MemRegion::Ram { mem, .. } => {
                let offset = addr - region_base;
                let mem_mut = mem.get_mut(offset..offset + data.len()).unwrap();
                mem_mut.copy_from_slice(data);
                MemWriteResult(Some(()))
            }
            MemRegion::Rom { .. } => {
                panic!("Attempt write to ROM region.");
            }
            _ => panic!("Unreachable"),
        }
    }

    pub fn tlb_get(&self, k: &P::TLBKey) -> Option<&P::TLBValue> {
        self.tlb.get(k)
    }

    pub fn tlb_insert(&mut self, k: P::TLBKey, v: P::TLBValue) -> Option<P::TLBValue> {
        self.tlb.insert(k, v)
    }

    pub fn tlb_remove(&mut self, k: &P::TLBKey) -> Option<P::TLBValue> {
        self.tlb.remove(k)
    }

    /// Remove all entries from the TLB for which f returns true
    pub fn tlb_flush<F>(&mut self, mut f: F)
    where
        F: FnMut(&P::TLBKey, &mut P::TLBValue) -> bool,
    {
        self.tlb.retain(|k, v| !f(k, v));
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn rng(&self) -> u64 {
        self.rng_fn
            .as_ref()
            .and_then(|rng_fn| Some(rng_fn()))
            .unwrap_or_else(|| rand::thread_rng().r#gen())
    }
}

// Device model

pub trait Device: 'static {
    fn declare(&self, decl: &mut DevDecl<Self>)
    where
        Self: Sized;
}

pub type DevId = usize;

pub struct DevInfo {
    name: String,
    containers: HashMap<String, NodeIndex>,
    intr_inputs: HashMap<String, (u32, NodeIndex)>,
}

impl DevInfo {
    pub fn container(&self, name: impl AsRef<str>) -> MemParent {
        let idx = self.containers.get(name.as_ref()).unwrap_or_else(|| {
            panic!(
                "Unknown memory container for device {0}: {1}",
                self.name,
                name.as_ref()
            )
        });
        MemParent(idx.clone())
    }

    pub fn intr_in(&self, name: impl AsRef<str>, n: u32) -> IntrInput {
        self.intr_inputs
            .get(name.as_ref())
            .and_then(|(num_pins, idx)| {
                // If pin name exists, check if n is in range of the pin numbers
                if n >= *num_pins {
                    None
                } else {
                    Some(IntrInput {
                        idx: idx.clone(),
                        n: n.clone(),
                    })
                }
            })
            .unwrap_or_else(|| {
                panic!(
                    "Unknown interrupt input for device {0}: [{1}:{n}]",
                    self.name,
                    name.as_ref()
                )
            })
    }
}

type DowncastAndCallRegRead = Box<dyn FnMut(&mut dyn Any, &mut dyn Any, &mut DevCtx) -> u64>;
type DowncastAndCallRegWrite = Box<dyn Fn(&mut dyn Any, &mut dyn Any, u64, &mut DevCtx)>;

struct RegMapping {
    dev: DevId,
    read_fn: Box<dyn Any>,
    read_dac: DowncastAndCallRegRead,
    write_fn: Box<dyn Any>,
    write_dac: DowncastAndCallRegWrite,
}

pub struct DevDecl<D: Device> {
    id: usize,
    reg_mapped: Vec<(ProcId, TypeId, RegMapping)>,
    mem_regions: Vec<(
        MemParent,
        AddrRange,
        MemRegion,
        Option<(DowncastAndCallMMIORead, DowncastAndCallMMIOWrite)>,
    )>,
    intr_conns: Vec<IntrInput>,
    intr_inputs: Vec<(String, u32, Box<dyn Any>, DowncastAndCallDevIntr)>,
    p: PhantomData<D>,
}

impl<D: Device> DevDecl<D> {
    fn new(id: usize) -> Self {
        Self {
            id,
            reg_mapped: Vec::new(),
            mem_regions: Vec::new(),
            intr_conns: Vec::new(),
            intr_inputs: Vec::new(),
            p: PhantomData,
        }
    }

    pub fn map_reg<T, R, W>(&mut self, proc: ProcId, read_fn: R, write_fn: W) -> &mut Self
    where
        T: RegisterView,
        R: 'static + FnMut(&mut D, &mut DevCtx) -> u64,
        W: 'static + FnMut(&mut D, u64, &mut DevCtx),
    {
        self.reg_mapped.push((
            proc,
            TypeId::of::<T::Register>(),
            RegMapping {
                dev: self.id,
                read_fn: Box::new(read_fn),
                read_dac: Box::new(|erased_fn, erased_dev, ctx| {
                    let read_fn = erased_fn
                        .downcast_mut::<R>()
                        .expect("Failed to downcast Reg read fn to original type.");

                    let dev = erased_dev
                        .downcast_mut::<D>()
                        .expect("Failed to downcast device struct to original type.");

                    read_fn(dev, ctx)
                }),
                write_fn: Box::new(write_fn),
                write_dac: Box::new(|erased_fn, erased_dev, val, ctx| {
                    let write_fn = erased_fn
                        .downcast_mut::<W>()
                        .expect("Failed to downcast Reg write fn to original type.");

                    let dev = erased_dev
                        .downcast_mut::<D>()
                        .expect("Failed to downcast device struct to original type.");

                    write_fn(dev, val, ctx)
                }),
            },
        ));
        self
    }

    pub fn container<S>(
        &mut self,
        name: S,
        parent: MemParent,
        base: usize,
        size: usize,
    ) -> &mut Self
    where
        S: Into<String>,
    {
        self.mem_regions.push((
            parent,
            AddrRange { base, size },
            MemRegion::Container {
                dev: Some(self.id),
                name: name.into(),
            },
            None,
        ));
        self
    }

    pub fn mmio<S, R, W>(
        &mut self,
        name: S,
        bus: MemParent,
        base: usize,
        size: usize,
        read_fn: R,
        write_fn: W,
    ) -> &mut Self
    where
        S: Into<String>,
        R: 'static + FnMut(&mut D, usize, usize, &mut DevCtx) -> MemReadResult,
        W: 'static + FnMut(&mut D, usize, &[u8], &mut DevCtx) -> MemWriteResult,
    {
        self.mem_regions.push((
            bus,
            AddrRange { base, size },
            MemRegion::Mmio {
                dev: self.id,
                name: name.into(),
                read_fn: Box::new(read_fn),
                write_fn: Box::new(write_fn),
            },
            Some((
                Box::new(|erased_fn, erased_dev, addr, size, ctx| {
                    let read_fn = erased_fn
                        .downcast_mut::<R>()
                        .expect("Failed to downcast MMIO read fn to original type.");

                    let dev = erased_dev
                        .downcast_mut::<D>()
                        .expect("Failed to downcast device struct to original type.");

                    read_fn(dev, addr, size, ctx)
                }),
                Box::new(|erased_fn, erased_dev, addr, data, ctx| {
                    let write_fn = erased_fn
                        .downcast_mut::<W>()
                        .expect("Failed to downcast MMIO write fn to original type.");

                    let dev = erased_dev
                        .downcast_mut::<D>()
                        .expect("Failed to downcast device struct to original type.");

                    write_fn(dev, addr, data, ctx)
                }),
            )),
        ));
        self
    }

    pub fn ram<S>(&mut self, name: S, parent: MemParent, base: usize, size: usize) -> &mut Self
    where
        S: Into<String>,
    {
        let mut mem = Vec::with_capacity(size);
        mem.resize(size, 0);
        self.mem_regions.push((
            parent,
            AddrRange { base, size },
            MemRegion::Ram {
                dev: Some(self.id),
                name: name.into(),
                mem,
            },
            None,
        ));
        self
    }

    pub fn rom<S>(&mut self, name: S, parent: MemParent, base: usize, size: usize) -> &mut Self
    where
        S: Into<String>,
    {
        let mut mem = Vec::with_capacity(size);
        mem.resize(size, 0);
        self.mem_regions.push((
            parent,
            AddrRange { base, size },
            MemRegion::Rom {
                dev: Some(self.id),
                name: name.into(),
                mem,
            },
            None,
        ));
        self
    }

    pub fn intr_conn(&mut self, pin: IntrInput) -> &mut Self {
        self.intr_conns.push(pin);
        self
    }

    pub fn intr_in<S, H>(&mut self, name: S, n: u32, handler: H) -> &mut Self
    where
        S: Into<String>,
        H: 'static + FnMut(&mut D, u32, bool, &mut DevCtx),
    {
        self.intr_inputs.push((
            name.into(),
            n,
            Box::new(handler),
            Box::new(|erased_fn, erased_dev, n, level, ctx| {
                let handler_fn = erased_fn
                    .downcast_mut::<H>()
                    .expect("Failed to downcast interrupt handler fn to original type.");

                let dev = erased_dev
                    .downcast_mut::<D>()
                    .expect("Failed to downcast device struct to original type.");

                handler_fn(dev, n, level, ctx)
            }),
        ));
        self
    }
}

enum DevEvent {
    SetIntr {
        pin: IntrInput,
        level: bool,
    },
    NewTimer {
        dev_id: DevId,
        expiry: u64,
        handler: Box<dyn Any>,
        dcac: DowncastAndCallTimer,
    },
    ModTimer {
        id: TimerId,
        new_expiry: u64,
    },
    DelTimer(TimerId),
    TimerExpired {
        dev_id: DevId,
        handler: Box<dyn Any>,
        dcac: DowncastAndCallTimer,
    },
}

impl std::fmt::Display for DevEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DevEvent::SetIntr { level, .. } => write!(f, "DevEvent::SetIntr {level}"),
            DevEvent::NewTimer { expiry, .. } => write!(f, "DevEvent::NewTimer {expiry}"),
            DevEvent::ModTimer { new_expiry, .. } => write!(f, "DevEvent::ModTimer {new_expiry}"),
            DevEvent::DelTimer(_) => write!(f, "DevEvent::DelTimer"),
            DevEvent::TimerExpired { .. } => write!(f, "DevEvent::TimerExpired"),
        }
    }
}

pub struct DevCtx {
    id: DevId,
    new_events: Vec<DevEvent>,
    count: u64,
}

impl DevCtx {
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn set_intr(&mut self, pin: IntrInput, level: bool) {
        self.new_events.push(DevEvent::SetIntr { pin, level });
    }

    pub fn new_timer<D, H>(&mut self, handler: H, expiry: u64) -> TimerId
    where
        D: Device,
        H: 'static + FnMut(&mut D, &mut DevCtx),
    {
        // println!(
        //     "New timer event requested by {0} (expiry = {expiry}).",
        //     self.id
        // );
        self.new_events.push(DevEvent::NewTimer {
            dev_id: self.id,
            expiry,
            handler: Box::new(handler),
            dcac: Box::new(|erased_fn, erased_dev, ctx| {
                let handler_fn = erased_fn
                    .downcast_mut::<H>()
                    .expect("Failed to downcast interrupt handler fn to original type.");

                let dev = erased_dev
                    .downcast_mut::<D>()
                    .expect("Failed to downcast device struct to original type.");

                handler_fn(dev, ctx)
            }),
        });

        TimerId {
            dev_id: self.id,
            expiry,
        }
    }

    pub fn mod_timer(&mut self, id: TimerId, new_expiry: u64) -> TimerId {
        self.new_events.push(DevEvent::ModTimer { id, new_expiry });

        TimerId {
            dev_id: self.id,
            expiry: new_expiry,
        }
    }

    pub fn del_timer(&mut self, id: TimerId) {
        self.new_events.push(DevEvent::DelTimer(id));
    }
}

// Memory model

pub struct MemReadResult(pub Option<[u8; 16]>);

pub struct MemWriteResult(pub Option<()>);

#[derive(Clone, Copy)]
pub struct MemParent(NodeIndex);

// Dynamic typing for Devices, MMIOs, and interrupts
type DowncastAndCallMMIORead =
    Box<dyn Fn(&mut dyn Any, &mut dyn Any, usize, usize, &mut DevCtx) -> MemReadResult>;

type DowncastAndCallMMIOWrite =
    Box<dyn Fn(&mut dyn Any, &mut dyn Any, usize, &[u8], &mut DevCtx) -> MemWriteResult>;

struct MemoryMap {
    map: Dag<MemRegion, AddrRange>,
    root: NodeIndex,
    mmio_downcasts: HashMap<NodeIndex, (DowncastAndCallMMIORead, DowncastAndCallMMIOWrite)>,
    rando_buf: Vec<u8>,
}

impl MemoryMap {
    fn new() -> Self {
        let mut map = Dag::new();
        let root = map.add_node(MemRegion::Container {
            dev: None,
            name: String::from("/"),
        });
        Self {
            map,
            root,
            mmio_downcasts: HashMap::new(),
            rando_buf: vec![0xff; 1000],
        }
    }

    fn root(&self) -> MemParent {
        MemParent(self.root)
    }

    fn route_to_leaf(&self, addr: usize) -> Option<(EdgeIndex, NodeIndex)> {
        let addr_router = self.addr_router(addr);

        for (e, n) in addr_router.iter(&self.map) {
            if matches!(
                self.map[n],
                MemRegion::Mmio { .. } | MemRegion::Ram { .. } | MemRegion::Rom { .. }
            ) {
                return Some((e, n));
            }
        }
        // The case where this is None means an address is directed to a container with no Buf or Mmio regions
        None
    }

    fn addr_router(
        &self,
        addr: usize,
    ) -> Recursive<
        Dag<MemRegion, AddrRange>,
        impl FnMut(&Dag<MemRegion, AddrRange>, NodeIndex) -> Option<(EdgeIndex, NodeIndex)>,
    > {
        let ret = self.map.recursive_walk(self.root, move |g, n| {
            g.children(n).iter(g).find(|&(e, _n)| g[e].contains(addr))
        });
        ret
    }

    fn region(&mut self, idx: NodeIndex) -> Option<&mut MemRegion> {
        self.map.node_weight_mut(idx)
    }
}

struct AddrRange {
    base: usize,
    size: usize,
}

impl AddrRange {
    fn contains(&self, addr: usize) -> bool {
        self.base <= addr && addr <= self.base + self.size
    }
}

enum MemRegion {
    Container {
        dev: Option<DevId>,
        name: String,
    },
    Mmio {
        dev: DevId,
        name: String,
        read_fn: Box<dyn Any>,
        write_fn: Box<dyn Any>,
    },
    Ram {
        dev: Option<DevId>,
        name: String,
        mem: Vec<u8>,
    },
    Rom {
        dev: Option<DevId>,
        name: String,
        mem: Vec<u8>,
    },
}

// Interrupt model

#[derive(Clone, Copy, Debug)]
pub struct IntrInput {
    idx: NodeIndex,
    n: u32,
}

type DowncastAndCallProcIntr<P: Processor> =
    Box<dyn for<'a> FnMut(&mut dyn Any, &dyn Any, u32, bool, &mut ProcCtx<'a, P>) -> bool>;

type DowncastAndCallDevIntr = Box<dyn Fn(&mut dyn Any, &mut dyn Any, u32, bool, &mut DevCtx)>;

struct InterruptTree<P: Processor> {
    tree: Dag<IntrIface, IntrConn>,
    proc_nodes: HashMap<ProcId, Vec<NodeIndex>>,
    proc_downcasts: HashMap<NodeIndex, DowncastAndCallProcIntr<P>>,
    dev_downcasts: HashMap<NodeIndex, DowncastAndCallDevIntr>,
}

impl<P: Processor> InterruptTree<P> {
    fn new() -> Self {
        Self {
            tree: Dag::new(),
            proc_nodes: HashMap::new(),
            proc_downcasts: HashMap::new(),
            dev_downcasts: HashMap::new(),
        }
    }

    fn intr_conn(&self, input: IntrInput) -> (EdgeIndex, NodeIndex) {
        let node_w = self.tree.node_weight(input.idx).unwrap();
        // println!("{node_w} {0}", input.n);

        self.tree
            .children(input.idx)
            .iter(&self.tree)
            .find(|(e, _c)| self.tree.edge_weight(*e).unwrap().n == input.n)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to map intrinput to an edge. Graph: {0}",
                    Dot::new(&self.tree)
                )
            })
    }
}

enum IntrIface {
    ProcInput {
        proc: ProcId,
        name: String,
        handler: Box<dyn Any>,
    },
    DevInput {
        dev: DevId,
        name: String,
        num_pins: u32,
        handler: Box<dyn Any>,
    },
    DevOutput(DevId),
}

impl std::fmt::Display for IntrIface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntrIface::ProcInput { name, .. } => write!(f, "ProcInput {name}"),
            IntrIface::DevInput { name, num_pins, .. } => write!(f, "DevInput {name} ({num_pins})"),
            IntrIface::DevOutput(a) => write!(f, "DevOutput ({a})"),
        }
    }
}

#[derive(Clone, Copy)]
struct IntrConn {
    level: bool,
    n: u32,
}

impl std::fmt::Display for IntrConn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0} ({1})", self.n, self.level)
    }
}

// Timer model

pub struct TimerExpiryResult(pub Option<()>);

// TODO: This assumes no device will have multiple timers for the same expiry.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct TimerId {
    dev_id: DevId,
    expiry: u64,
}

pub type DowncastAndCallTimer = Box<dyn Fn(&mut dyn Any, &mut dyn Any, &mut DevCtx)>;
