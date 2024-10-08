use crate::model::ir::{self, ProcIRCtx};
use crate::model::proc::*;

pub mod config {
    mod features;
    pub use features::Feat;
    mod impdef;
    pub use impdef::Impdef;
    mod version;
    pub use version::Version;
}
pub mod registers;
use bitvec::order::Lsb0;
use bitvec::view::BitView;
use registers::{aarch64::*, *};

use config::*;
mod insns {
    mod a64;
    mod helpers;
}
mod pseudoc;
pub use pseudoc::EL;
use pseudoc::*;

struct ArmCtx<'cpu, 'ctx, 'a: 'cpu + 'ctx> {
    cpu: &'cpu ArmCpu,
    ctx: &'ctx mut ProcCtx<'a, ArmCpu>,
}

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    fn is_feat_impl(&self, feat: Feat) -> bool {
        self.cpu.is_feat_impl(feat)
    }

    fn physical_count(&self) -> u64 {
        self.ctx.count()
    }

    fn curr_el(&mut self) -> EL {
        let curr_el = self.read::<pstate::EL>();
        let (el_hi, el_lo) = (
            curr_el.view_bits::<Lsb0>()[1],
            curr_el.view_bits::<Lsb0>()[0],
        );
        EL::from_bits(el_hi, el_lo)
    }

    fn set_el(&mut self, el: EL) {
        self.write::<pstate::EL>(el.as_num());
    }

    fn btype(&mut self) -> BType {
        let btype = self.read::<pstate::BTYPE>();
        let (hi, lo) = (btype.view_bits::<Lsb0>()[1], btype.view_bits::<Lsb0>()[0]);
        BType::from_bits(hi, lo)
    }

    fn set_btype(&mut self, btype: BType) {
        self.write::<pstate::BTYPE>(btype.as_num());
    }

    fn read<T>(&mut self) -> u64
    where
        T: RegisterView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        self.ctx.read::<T>()
    }

    fn write<T>(&mut self, val: u64)
    where
        T: RegisterView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        self.ctx.write::<T>(val);
    }

    fn read_elem<T>(&self, idx: usize) -> u64
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        self.ctx.read_elem::<T>(idx)
    }

    fn write_elem<T>(&mut self, idx: usize, val: u64)
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        self.ctx.write_elem(idx, val);
    }
}

struct ArmIRCtx<'cpu, 'ctx> {
    cpu: &'cpu ArmCpu,
    ctx: &'ctx ProcIRCtx,
}

impl<'cpu, 'ctx> ArmIRCtx<'cpu, 'ctx> {
    fn is_feat_impl(&self, feat: Feat) -> bool {
        self.cpu.is_feat_impl(feat)
    }

    fn reg1<T>(&self) -> ir::Reg1
    where
        T: RegisterView,
        Assert<{ T::LEN == 1 }>: IsTrue,
    {
        self.ctx.reg1::<T>()
    }

    pub fn arr8<T>(&self, idx: u32) -> ir::Reg8
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 8 }>: IsTrue,
    {
        self.ctx.arr8::<T>(idx as usize)
    }

    pub fn arr16<T>(&self, idx: u32) -> ir::Reg16
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 16 }>: IsTrue,
    {
        self.ctx.arr16::<T>(idx as usize)
    }

    fn arr32<T>(&self, idx: u32) -> ir::Reg32
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 32 }>: IsTrue,
    {
        self.ctx.arr32::<T>(idx as usize)
    }

    fn reg64<T>(&self) -> ir::Reg64
    where
        T: RegisterView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        self.ctx.reg64::<T>()
    }

    fn arr64<T>(&self, idx: u32) -> ir::Reg64
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        self.ctx.arr64::<T>(idx as usize)
    }

    pub fn arr128<T>(&self, idx: u32) -> ir::Reg128
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 128 }>: IsTrue,
    {
        self.ctx.arr128(idx as usize)
    }

    fn wsp(&self, el: EL) -> ir::Reg32 {
        match el {
            EL::EL0 => self.ctx.reg32::<WSP_EL0>(),
            EL::EL1 => self.ctx.reg32::<WSP_EL1>(),
            EL::EL2 => self.ctx.reg32::<WSP_EL2>(),
            EL::EL3 => self.ctx.reg32::<WSP_EL3>(),
        }
    }

    fn sp(&self, el: EL) -> ir::Reg64 {
        match el {
            EL::EL0 => self.ctx.reg64::<SP_EL0>(),
            EL::EL1 => self.ctx.reg64::<SP_EL1>(),
            EL::EL2 => self.ctx.reg64::<SP_EL2>(),
            EL::EL3 => self.ctx.reg64::<SP_EL3>(),
        }
    }

    fn elr(&self, el: EL) -> ir::Reg64 {
        match el {
            EL::EL0 => panic!("No ELR_ELx for EL0"),
            EL::EL1 => self.ctx.reg64::<ELR_EL1>(),
            EL::EL2 => self.ctx.reg64::<ELR_EL2>(),
            EL::EL3 => self.ctx.reg64::<ELR_EL3>(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ArmBrAttrs {
    General {
        branch_type: BranchType,
        branch_conditional: bool,
    },
    Eret {
        pac: bool,
        use_key_a: bool,
        auth_then_branch: bool,
    },
}

#[derive(Clone, Copy)]
pub enum ArmLdstAttrs {
    ExLDST {
        acqrel: bool,
        tagchecked: bool,
        s: u32,
    },
    LOR {
        tagchecked: bool,
    },
    AcqRel {
        tagchecked: bool,
    },
    GPR {
        nontemporal: bool,
        unpriv_instr: bool, // Indicates whether privileged = PSTATE.EL != EL0
        tagchecked: bool,
    },
    ASIMD {
        nontemporal: bool,
        tagchecked: bool,
    },
}

pub struct ArmCpu {
    pub impdef: Impdef,
    version: Version,
    tlb_enabled: bool,
}

impl ArmCpu {
    pub fn v9p4a() -> Self {
        Self {
            impdef: Impdef::default(),
            version: Version::Armv9p4a,
            tlb_enabled: false,
        }
    }

    pub fn is_feat_impl(&self, feat: Feat) -> bool {
        self.version.is_feat_impl(feat)
    }

    fn ctx<'cpu, 'ctx, 'a: 'cpu + 'ctx>(
        &'cpu self,
        ctx: &'ctx mut ProcCtx<'a, ArmCpu>,
    ) -> ArmCtx<'cpu, 'ctx, 'a> {
        ArmCtx { cpu: self, ctx }
    }

    fn ir_ctx<'cpu, 'ctx>(&'cpu self, ir_ctx: &'ctx ProcIRCtx) -> ArmIRCtx<'cpu, 'ctx> {
        ArmIRCtx {
            cpu: self,
            ctx: ir_ctx,
        }
    }

    fn handle_fiq<'a>(&self, _n: u32, level: bool, _ctx: &mut ProcCtx<'a, ArmCpu>) -> bool {
        if !level {
            return false;
        }
        todo!()
    }

    fn handle_irq<'a>(&self, _n: u32, level: bool, ctx: &mut ProcCtx<'a, ArmCpu>) -> bool {
        // println!(
        //     "handle_irq: {level}, mask = {0}.",
        //     ctx.read::<pstate::I>() != 0
        // );
        if level && (ctx.read::<pstate::I>() == 0) {
            let mut ctx = self.ctx(ctx);
            ctx.aarch64_take_physical_irq_exception();
            return true;
        } else {
            return false;
        }
    }

    fn handle_serror<'a>(&self, _n: u32, level: bool, _ctx: &mut ProcCtx<'a, ArmCpu>) -> bool {
        if !level {
            return false;
        }
        todo!()
    }
}

impl Processor for ArmCpu {
    fn declare(&self, decl: &mut ProcDecl<Self>)
    where
        Self: Sized,
    {
        decl.intr_in("FIQ", 1, ArmCpu::handle_fiq)
            .intr_in("IRQ", 1, ArmCpu::handle_irq)
            .intr_in("SError", 1, ArmCpu::handle_serror)
            .reg::<PC>()
            .reg_array::<R>()
            .reg_array::<Z>();

        self.decl_pstate(decl);
        self.decl_aarch32_sysregs(decl);
        self.decl_aarch64_sysregs(decl);
    }

    fn ip(&self, ctx: &mut ProcCtx<ArmCpu>) -> usize {
        ctx.read::<PC>() as usize
    }

    fn set_ip(&self, ctx: &mut ProcCtx<ArmCpu>, addr: usize) {
        ctx.write::<PC>(addr as u64);
    }

    type Insn = u32;

    fn fetch_decode<'a>(
        &self,
        addr: usize,
        ir_ctx: &ProcIRCtx,
        ctx: &mut ProcCtx<'a, ArmCpu>,
    ) -> Option<(Self::Insn, ir::IRGraph<Self>)>
    where
        Self: Sized,
    {
        let mut ctx = self.ctx(ctx);
        let ir_ctx = self.ir_ctx(ir_ctx);

        let accdesc = ctx.create_acc_desc_ifetch();
        let insn = u32::from_le_bytes(
            ctx.mem_read(addr as u64, 4, accdesc)
                .and_then(|res| {
                    let mut insn = [0; 4];
                    insn.copy_from_slice(&res[0..4]);
                    Some(insn)
                })
                .expect("All test systems are valid"),
        );

        ctx.decode_a64(insn, &ir_ctx)
            .and_then(|ir_graph| Some((insn, ir_graph)))
    }

    type BrAttrs = ArmBrAttrs;

    fn br<'a>(&self, addr: usize, attrs: Self::BrAttrs, ctx: &mut ProcCtx<'a, ArmCpu>) {
        let mut ctx = self.ctx(ctx);
        match attrs {
            ArmBrAttrs::General {
                branch_type,
                branch_conditional,
            } => {
                ctx.branch_to(addr, branch_type, branch_conditional);
            }
            ArmBrAttrs::Eret { .. } => {
                let spsr = match ctx.curr_el() {
                    EL::EL0 => panic!("Unreachable"),
                    EL::EL1 => ctx.read::<SPSR_EL1>(),
                    EL::EL2 => ctx.read::<SPSR_EL2>(),
                    EL::EL3 => ctx.read::<SPSR_EL3>(),
                };
                ctx.aarch64_exception_return(addr as u64, spsr);
            }
        }
    }

    fn cbr<'a>(
        &self,
        cond: bool,
        t_addr: usize,
        f_addr: usize,
        attrs: Self::BrAttrs,
        ctx: &mut ProcCtx<'a, ArmCpu>,
    ) {
        let mut ctx = self.ctx(ctx);
        if let ArmBrAttrs::General {
            branch_type,
            branch_conditional,
        } = attrs
        {
            if cond {
                ctx.branch_to(t_addr, branch_type, branch_conditional);
            } else {
                ctx.branch_not_taken(branch_type, branch_conditional);
                ctx.write::<PC>(f_addr as u64);
            }
        } else {
            panic!("Unreachable")
        }
    }

    type LdstAttrs = ArmLdstAttrs;

    fn ld<'a>(
        &self,
        addr: usize,
        size: usize,
        attrs: Self::LdstAttrs,
        ctx: &mut ProcCtx<'a, ArmCpu>,
    ) -> MemReadResult {
        let mut ctx = self.ctx(ctx);
        let accdesc = match attrs {
            ArmLdstAttrs::GPR {
                nontemporal,
                unpriv_instr,
                tagchecked,
            } => {
                let privileged = if unpriv_instr {
                    ctx.aarch64_is_unpriv_access_priv()
                } else {
                    ctx.curr_el() != EL::EL0
                };
                ctx.create_acc_desc_gpr(MemOp::LOAD, nontemporal, privileged, tagchecked)
            }
            ArmLdstAttrs::ASIMD {
                nontemporal,
                tagchecked,
            } => ctx.create_acc_desc_asimd(MemOp::LOAD, nontemporal, tagchecked),
            ArmLdstAttrs::AcqRel { tagchecked } => {
                ctx.create_acc_desc_acq_rel(MemOp::LOAD, tagchecked)
            }
            ArmLdstAttrs::ExLDST {
                acqrel, tagchecked, ..
            } => ctx.create_acc_desc_ex_ldst(MemOp::LOAD, acqrel, tagchecked),

            ArmLdstAttrs::LOR { tagchecked } => {
                ctx.create_acc_desc_acq_rel(MemOp::LOAD, tagchecked)
            }
        };

        let ret = ctx.mem_read(addr as u64, size, accdesc).unwrap();
        // println!("read vaddr 0x{addr:x} {ret:?}");
        MemReadResult(Some(ret))
    }

    fn st<'a>(
        &self,
        addr: usize,
        data: &[u8],
        attrs: Self::LdstAttrs,
        ctx: &mut ProcCtx<'a, ArmCpu>,
    ) -> MemWriteResult {
        let mut ctx = self.ctx(ctx);
        let accdesc = match attrs {
            ArmLdstAttrs::GPR {
                nontemporal,
                unpriv_instr,
                tagchecked,
            } => {
                let privileged = if unpriv_instr {
                    ctx.aarch64_is_unpriv_access_priv()
                } else {
                    ctx.curr_el() != EL::EL0
                };
                ctx.create_acc_desc_gpr(MemOp::STORE, nontemporal, privileged, tagchecked)
            }
            ArmLdstAttrs::ASIMD {
                nontemporal,
                tagchecked,
            } => ctx.create_acc_desc_asimd(MemOp::STORE, nontemporal, tagchecked),
            ArmLdstAttrs::AcqRel { tagchecked } => {
                ctx.create_acc_desc_acq_rel(MemOp::STORE, tagchecked)
            }
            ArmLdstAttrs::ExLDST {
                acqrel, tagchecked, ..
            } => ctx.create_acc_desc_ex_ldst(MemOp::STORE, acqrel, tagchecked),

            ArmLdstAttrs::LOR { tagchecked } => {
                ctx.create_acc_desc_acq_rel(MemOp::STORE, tagchecked)
            }
        };

        if let ArmLdstAttrs::ExLDST { s, .. } = attrs {
            // Rs == 0 means the operation updated memory
            ctx.write_elem::<X>(s as usize, 0);
        }

        // println!("write vaddr 0x{addr:x} {0:?}", data);
        ctx.mem_write(addr as u64, data.len(), data, accdesc);
        MemWriteResult(Some(()))
    }

    type TLBKey = TLBContext;

    type TLBValue = TLBLine;

    type ExcepAttrs = Option<()>;

    fn handle_excep<'a>(&self, _excep: Self::ExcepAttrs, _ctx: &mut ProcCtx<'a, ArmCpu>) {
        todo!()
    }
}
