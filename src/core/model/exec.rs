use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;

use bitvec::prelude::*;

use crate::model::{ir::*, *};

struct EventCal {
    events: HashMap<u64, VecDeque<DevEvent>>,
}

impl EventCal {
    fn enq_new_events(&mut self, curr_time: u64, dev: DevCtx) {
        // TODO: because NewTimer event is "executed" by putting a TimerExpired event on the calendar in the future, all of these just
        // enqueue the dev_ctx's new events immediately.
        // A more clear distinction should be made between event-creation and event dispatching.
        for new_event in dev.new_events {
            // println!("Enqueuing {new_event} at {curr_time}.");
            let curr_events = self
                .events
                .entry(curr_time)
                .or_insert_with(|| VecDeque::new());
            curr_events.push_back(new_event);
        }
    }
}

pub struct StepEmu<P: Processor> {
    sys: System<P>,
    event_cal: EventCal,
    timers: HashMap<TimerId, usize>,
    instrm: InstrmCollection,
}

impl<P: Processor> StepEmu<P> {
    pub fn new(sys: System<P>) -> Self {
        Self {
            sys,
            event_cal: EventCal {
                events: HashMap::new(),
            },
            timers: HashMap::new(),
            instrm: InstrmCollection::new(),
        }
    }

    pub fn instrument(&mut self, instrm: impl Instrumentation) -> InstrmInfo {
        let instrm_id = self.instrm.instrm_states.len();
        let mut decl = InstrmDecl::new(instrm_id);
        instrm.declare(&mut decl);
        self.instrm.instrm_states.push(Box::new(instrm));

        if let Some(on_br) = decl.on_br {
            self.instrm.on_br_cbs.push(on_br);
        }
        if let Some(on_cbr) = decl.on_cbr {
            self.instrm.on_cbr_cbs.push(on_cbr);
        }

        InstrmInfo { id: instrm_id }
    }

    pub fn uninstrument<I: Instrumentation>(&mut self, instrm: InstrmInfo) -> I {
        *self
            .instrm
            .instrm_states
            .remove(instrm.id)
            .downcast()
            .unwrap()
    }

    pub fn take(self) -> System<P> {
        self.sys
    }

    pub fn run(&mut self, stop_time: Option<u64>) {
        let mut time = 0; // TODO: should probably be a part of System itself

        loop {
            if stop_time.is_some_and(|stop_time| time >= stop_time) {
                break;
            }

            // Step each processor
            for (id, ((proc, proc_data), ir_ctx)) in self
                .sys
                .procs
                .iter()
                .zip(self.sys.proc_data.iter_mut())
                .zip(self.sys.ir_ctxs.iter())
                .enumerate()
            {
                // Handle events for this cycle except the processor step (currently just device interrupt changes and timer create/modify/delete)
                while let Some(mut curr_events) = self.event_cal.events.remove(&time) {
                    while let Some(event) = curr_events.pop_front() {
                        // println!("Processing {event} at {time}.");
                        match event {
                            DevEvent::SetIntr { pin, level } => {
                                // Set the new level in the IntrTree
                                let (edge, _child) = self.sys.intrs.intr_conn(pin);
                                // println!("Before SetIntr {0}", Dot::new(&self.sys.intrs.tree));
                                // println!("level = {level}.");
                                self.sys.intrs.tree.edge_weight_mut(edge).unwrap().level = level;
                                // println!("After SetIntr {0}", Dot::new(&self.sys.intrs.tree));
                                // Call the handler if this is a device
                                if let IntrIface::DevInput {
                                    dev: dev_id,
                                    handler,
                                    ..
                                } = self.sys.intrs.tree.node_weight_mut(pin.idx).unwrap()
                                {
                                    let dev = self.sys.devs.get_mut(*dev_id).unwrap().as_mut();
                                    let dcac = self.sys.intrs.dev_downcasts.get(&pin.idx).unwrap();
                                    let mut dev_ctx = DevCtx {
                                        id: *dev_id,
                                        new_events: Vec::new(),
                                        count: time,
                                    };
                                    // println!("At device intr_p dcac, level = {level}.");
                                    dcac(&mut (**handler), dev, pin.n, level, &mut dev_ctx);
                                    // Enqueue any resulting events
                                    self.event_cal.enq_new_events(time, dev_ctx);
                                }
                            }
                            DevEvent::NewTimer {
                                dev_id,
                                expiry,
                                handler,
                                dcac,
                            } => {
                                let events_at_expiry = self
                                    .event_cal
                                    .events
                                    .entry(expiry) // This means expiry is an absolute tick time
                                    .or_insert_with(|| VecDeque::new());
                                // println!("Inserted timer expiry at {expiry}.");
                                events_at_expiry.push_back(DevEvent::TimerExpired {
                                    dev_id,
                                    handler,
                                    dcac,
                                });

                                // Timer ID's
                                self.timers
                                    .insert(TimerId { dev_id, expiry }, events_at_expiry.len() - 1);
                            }
                            DevEvent::ModTimer {
                                id: timer_id,
                                new_expiry,
                            } => {
                                // Delete old
                                let index_in_expiry_events = self.timers.get(&timer_id).unwrap();
                                // println!("Modifying timer expiry at {0}.", timer_id.expiry);
                                let deleted = self
                                    .event_cal
                                    .events
                                    .get_mut(&timer_id.expiry)
                                    .unwrap()
                                    .remove(*index_in_expiry_events);
                                assert!(deleted.is_some());
                                let (handler, dcac) =
                                    if let DevEvent::TimerExpired { handler, dcac, .. } =
                                        deleted.unwrap()
                                    {
                                        (handler, dcac)
                                    } else {
                                        panic!("Unreachable");
                                    };

                                // Create new
                                let events_at_expiry = self
                                    .event_cal
                                    .events
                                    .entry(new_expiry) // This means expiry is an absolute tick time
                                    .or_insert_with(|| VecDeque::new());
                                events_at_expiry.push_back(DevEvent::TimerExpired {
                                    dev_id: timer_id.dev_id,
                                    handler,
                                    dcac,
                                });

                                // Timer ID's
                                self.timers.insert(
                                    TimerId {
                                        dev_id: timer_id.dev_id,
                                        expiry: new_expiry,
                                    },
                                    events_at_expiry.len() - 1,
                                );
                            }
                            DevEvent::DelTimer(timer_id) => {
                                // println!("Delete timer expiry at {0}.", timer_id.expiry);
                                let index_in_expiry_events = self.timers.get(&timer_id).unwrap();
                                let deleted = self
                                    .event_cal
                                    .events
                                    .get_mut(&timer_id.expiry)
                                    .unwrap()
                                    .remove(*index_in_expiry_events);
                                assert!(deleted.is_some());
                            }
                            DevEvent::TimerExpired {
                                dev_id,
                                mut handler,
                                dcac,
                            } => {
                                // println!("Timer expired: {time} (dev = {dev_id})");
                                let dev = self.sys.devs.get_mut(dev_id).unwrap().as_mut();
                                let mut dev_ctx = DevCtx {
                                    id: dev_id,
                                    new_events: Vec::new(),
                                    count: time,
                                };
                                dcac(handler.as_mut(), dev, &mut dev_ctx);
                                // Enqueue any resulting events
                                self.event_cal.enq_new_events(time, dev_ctx);
                            }
                        }
                    }
                }

                // Single-step
                let mut ctx = ProcCtx {
                    reg_data: &mut proc_data.reg_data,
                    reg_offsets: &proc_data.reg_offsets,
                    reg_mapped: &mut proc_data.reg_mapped,
                    tlb: &mut proc_data.tlb,
                    devs: &mut self.sys.devs,
                    mem: &mut self.sys.mem,
                    count: proc_data.count,
                    rng_fn: proc_data.rng_fn,
                    new_dev_ctxs: Vec::new(),
                    next_mem_write_cb_id: self.instrm.on_mem_write_cbs.len(),
                    instrm: &mut self.instrm,
                    new_instrm_ctxs: Vec::new(),
                };

                // TODO: re-implement on_step callbacks
                // if let Some(on_step) = self.instrm.on_step.as_mut() {
                //     on_step(time, &mut ctx);
                // }

                // Take pending interrupts
                let mut any_intr_taken = false;
                for proc_node in self.sys.intrs.proc_nodes.get(&id).unwrap() {
                    let mut children = self.sys.intrs.tree.children(*proc_node);
                    while let Some((conn, _dev_node)) = children.walk_next(&self.sys.intrs.tree) {
                        let conn = self.sys.intrs.tree.edge_weight(conn).unwrap().clone();

                        if let IntrIface::ProcInput { handler, .. } =
                            self.sys.intrs.tree.node_weight_mut(*proc_node).unwrap()
                        {
                            let downcast_and_call =
                                self.sys.intrs.proc_downcasts.get_mut(proc_node).unwrap();
                            let handler = handler.as_mut();

                            // println!("At proc intr_p dcac, level = {0}.", conn.level);
                            let intr_taken =
                                downcast_and_call(handler, proc, conn.n, conn.level, &mut ctx);
                            any_intr_taken = intr_taken || any_intr_taken;
                        }
                    }
                }

                // If none taken, execute an instruction
                if !any_intr_taken {
                    // println!("Intr not taken.");

                    let ip = proc.ip(&mut ctx);
                    let (insn, ir_g) = proc
                        .fetch_decode(ip, ir_ctx, &mut ctx)
                        .unwrap_or_else(|| panic!("Fetch error: 0x{ip:x}"));

                    // TODO: re-implement on_insn callbacks
                    // if let Some(on_insn) = self.instrm.on_insn.as_mut() {
                    //     on_insn(ip, insn, &ir_g, &mut ctx);
                    // }

                    StepEmu::exec(ir_g, proc, &mut ctx);
                } else {
                    // println!("Intr taken so no instruction.");
                }

                // Enqueue the events from this processor step for the next cycle (time + 1)
                // TODO: cylce = time assumption
                while let Some(dev_ctx) = ctx.new_dev_ctxs.pop() {
                    self.event_cal.enq_new_events(time + 1, dev_ctx)
                }
                let mut new_instrm_ctxs = ctx.new_instrm_ctxs;
                while let Some(instrm_ctx) = new_instrm_ctxs.pop() {
                    self.instrm.handle_new_instrm_events(instrm_ctx);
                }

                proc_data.count += 1;
            }
            time += 1;
        }
    }

    fn exec(ir_graph: IRGraph<P>, proc: &P, ctx: &mut ProcCtx<P>) {
        // TODO: right now every IRGraph is a single node so we don't have to iterate
        let ir_block = ir_graph.pop();

        let mut interp = Interpreter::new();
        for (ir_res, ir_op) in ir_block {
            match ir_op {
                // System control
                IROp::Br(target, attrs) => {
                    let target = interp.addr(target);
                    // Execute all instrumentations with an on_br callback
                    let origin = proc.ip(ctx);
                    for (instrm_id, dcac, instrm_br) in &ctx.instrm.on_br_cbs {
                        let instrm = ctx.instrm.instrm_states.get_mut(*instrm_id).unwrap();
                        dcac(instrm_br.as_ref(), instrm.as_mut(), origin, target);
                    }
                    proc.br(target, attrs, ctx)
                }
                IROp::Cbr(cond, t_target, f_target, attrs) => {
                    let (cond, t_target, f_target) = (
                        interp.bool(cond),
                        interp.addr(t_target),
                        interp.addr(f_target),
                    );
                    // Execute all instrumentations with an on_cbr callback
                    let origin = proc.ip(ctx);
                    for (instrm_id, dcac, instrm_cbr) in &ctx.instrm.on_cbr_cbs {
                        let instrm = ctx.instrm.instrm_states.get_mut(*instrm_id).unwrap();
                        dcac(
                            instrm_cbr.as_ref(),
                            instrm.as_mut(),
                            origin,
                            cond,
                            t_target,
                            f_target,
                        );
                    }
                    proc.cbr(cond, t_target, f_target, attrs, ctx);
                }
                // IROp::RaiseExcp(excp) => proc.raise_exception(excp),
                // System access
                IROp::RReg(op) => interp.read_reg(ir_res, op, ctx),
                IROp::WReg(op) => interp.write_reg(op, ctx),
                IROp::RMem(op) => interp.read_mem(ir_res, op, proc, ctx),
                IROp::WMem(op) => interp.write_mem(op, proc, ctx),
                // IR control
                IROp::Call(op) => interp.call(ir_res, op, proc, ctx),
                IROp::Nop => continue,
                // Data-processing
                // IROp::Addr(op) => interp.addr_op(ir_res, op),
                IROp::Bool(op) => interp.bool_op(ir_res, op),
                IROp::Int8(op) => interp.int8_op(ir_res, op),
                IROp::Int16(op) => interp.int16_op(ir_res, op),
                IROp::Int32(op) => interp.int32_op(ir_res, op),
                IROp::Int64(op) => interp.int64_op(ir_res, op),
                IROp::Int128(op) => interp.int128_op(ir_res, op),
                IROp::Float16(op) => interp.float16_op(ir_res, op),
                IROp::Float32(op) => interp.float32_op(ir_res, op),
                IROp::Float64(op) => interp.float64_op(ir_res, op),
            }
        }
    }
}

struct Interpreter<P: Processor> {
    // addr_vals: HashMap<Def, usize>,
    bit_vals: HashMap<Def, bool>,
    bv8_vals: HashMap<Def, u8>,
    bv16_vals: HashMap<Def, u16>,
    bv32_vals: HashMap<Def, u32>,
    bv64_vals: HashMap<Def, u64>,
    bv128_vals: HashMap<Def, u128>,
    proc: PhantomData<P>,
}

impl<P: Processor> Interpreter<P> {
    fn new() -> Self {
        Self {
            // addr_vals: HashMap::new(),
            bit_vals: HashMap::new(),
            bv8_vals: HashMap::new(),
            bv16_vals: HashMap::new(),
            bv32_vals: HashMap::new(),
            bv64_vals: HashMap::new(),
            bv128_vals: HashMap::new(),
            proc: PhantomData,
        }
    }

    fn addr(&self, val: Addr) -> usize {
        self.u64(val) as usize
        // match val {
        //     Addr::Lit(val) => val,
        //     Addr::Var(def) => self
        //         .addr_vals
        //         .get(&def)
        //         .expect("Builder guarantees well-formed IR")
        //         .clone(),
        // }
    }

    fn bool(&self, val: Bit) -> bool {
        match val {
            Bit::Lit(val) => val,
            Bit::Var(def) => self
                .bit_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .clone(),
        }
    }

    fn _bv8(&mut self, val: Bv8) -> BitVec {
        let mut new_bv = bitvec!(0; 8);
        match val {
            Bv8::Lit(val) => new_bv.store(val),
            Bv8::Var(def) => self
                .bv8_vals
                .get(&def)
                .and_then(|val| {
                    new_bv.store(*val);
                    Some(())
                })
                .expect("Builder guarantees well-formed IR"),
        };
        new_bv
    }

    fn _bv16(&mut self, val: Bv16) -> BitVec {
        let mut new_bv = bitvec!(0; 16);
        match val {
            Bv16::Lit(val) => new_bv.store(val),
            Bv16::Var(def) => self
                .bv16_vals
                .get(&def)
                .and_then(|val| {
                    new_bv.store(*val);
                    Some(())
                })
                .expect("Builder guarantees well-formed IR"),
        };
        new_bv
    }

    fn _bv32(&mut self, val: Bv32) -> BitVec {
        let mut new_bv = bitvec!(0; 32);
        match val {
            Bv32::Lit(val) => new_bv.store(val),
            Bv32::Var(def) => self
                .bv32_vals
                .get(&def)
                .and_then(|val| {
                    new_bv.store(*val);
                    Some(())
                })
                .expect("Builder guarantees well-formed IR"),
        };
        new_bv
    }

    fn _bv64(&mut self, val: Bv64) -> BitVec {
        let mut new_bv = bitvec!(0; 64);
        match val {
            Bv64::Lit(val) => new_bv.store(val),
            Bv64::Var(def) => self
                .bv64_vals
                .get(&def)
                .and_then(|val| {
                    new_bv.store(*val);
                    Some(())
                })
                .expect("Builder guarantees well-formed IR"),
        };
        new_bv
    }

    fn _bv128(&mut self, val: Bv128) -> BitVec {
        let mut new_bv = bitvec!(0; 128);
        match val {
            Bv128::Lit(val) => new_bv.store(val),
            Bv128::Var(def) => self
                .bv128_vals
                .get(&def)
                .and_then(|val| {
                    new_bv.store(*val);
                    Some(())
                })
                .expect("Builder guarantees well-formed IR"),
        };
        new_bv
    }

    fn i8(&self, val: Bv8) -> i8 {
        match val {
            Bv8::Lit(val) => val.view_bits::<Lsb0>().load(),
            Bv8::Var(def) => self
                .bv8_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .view_bits::<Lsb0>()
                .load(),
        }
    }

    fn i16(&self, val: Bv16) -> i16 {
        match val {
            Bv16::Lit(val) => val.view_bits::<Lsb0>().load(),
            Bv16::Var(def) => self
                .bv16_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .view_bits::<Lsb0>()
                .load(),
        }
    }

    fn i32(&self, val: Bv32) -> i32 {
        match val {
            Bv32::Lit(val) => val.view_bits::<Lsb0>().load(),
            Bv32::Var(def) => self
                .bv32_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .view_bits::<Lsb0>()
                .load(),
        }
    }

    fn i64(&self, val: Bv64) -> i64 {
        match val {
            Bv64::Lit(val) => val.view_bits::<Lsb0>().load(),
            Bv64::Var(def) => self
                .bv64_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .view_bits::<Lsb0>()
                .load(),
        }
    }

    fn i128(&self, val: Bv128) -> i128 {
        // u128 doesn't implement BitStore, so we need to copy the value into a
        // u64-backed 128-bit array and then load the i128
        let val = match val {
            Bv128::Lit(val) => val,
            Bv128::Var(def) => self
                .bv128_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .clone(),
        };
        let mut bv = BitArray::<[u64; 2], Lsb0>::ZERO;
        bv.fill_with(|idx| (val >> idx) & 1 != 0);
        bv.as_bitslice().load()
    }

    fn u8(&self, val: Bv8) -> u8 {
        match val {
            Bv8::Lit(val) => val,
            Bv8::Var(def) => self
                .bv8_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .clone(),
        }
    }

    fn u16(&self, val: Bv16) -> u16 {
        match val {
            Bv16::Lit(val) => val,
            Bv16::Var(def) => self
                .bv16_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .clone(),
        }
    }

    fn u32(&self, val: Bv32) -> u32 {
        match val {
            Bv32::Lit(val) => val,
            Bv32::Var(def) => self
                .bv32_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .clone(),
        }
    }

    fn u64(&self, val: Bv64) -> u64 {
        match val {
            Bv64::Lit(val) => val,
            Bv64::Var(def) => self
                .bv64_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .clone(),
        }
    }

    fn u128(&self, val: Bv128) -> u128 {
        match val {
            Bv128::Lit(val) => val,
            Bv128::Var(def) => self
                .bv128_vals
                .get(&def)
                .expect("Builder guarantees well-formed IR")
                .clone(),
        }
    }

    // fn f16(&self, val: Float16) -> u16 {
    //     match val {
    //         Float16::Lit(val) => val,
    //         Float16::Var(def) => self
    //             .f16_vals
    //             .get(&def)
    //             .expect("Builder guarantees well-formed IR")
    //             .clone(),
    //     }
    // }

    fn f32(&self, val: Bv32) -> f32 {
        match val {
            Bv32::Lit(val) => f32::from_bits(val),
            Bv32::Var(def) => f32::from_bits(
                self.bv32_vals
                    .get(&def)
                    .expect("Builder guarantees well-formed IR")
                    .clone(),
            ),
        }
    }

    fn f64(&self, val: Bv64) -> f64 {
        match val {
            Bv64::Lit(val) => f64::from_bits(val),
            Bv64::Var(def) => f64::from_bits(
                self.bv64_vals
                    .get(&def)
                    .expect("Builder guarantees well-formed IR")
                    .clone(),
            ),
        }
    }

    // fn new_addr(&mut self, def: Def, val: usize) {
    //     self.addr_vals
    //         .insert(def, val)
    //         .expect("Builder guarantees well-formed IR");
    // }

    fn new_bool(&mut self, def: Def, val: bool) {
        assert!(self.bit_vals.insert(def, val).is_none());
    }

    fn new_i8(&mut self, def: Def, val: i8) {
        let mut new_val = 0;
        new_val.view_bits_mut::<Lsb0>().store(val);
        assert!(self.bv8_vals.insert(def, new_val).is_none());
    }

    fn new_i16(&mut self, def: Def, val: i16) {
        let mut new_val = 0;
        new_val.view_bits_mut::<Lsb0>().store(val);
        assert!(self.bv16_vals.insert(def, new_val).is_none());
    }

    fn new_i32(&mut self, def: Def, val: i32) {
        let mut new_val = 0;
        new_val.view_bits_mut::<Lsb0>().store(val);
        assert!(self.bv32_vals.insert(def, new_val).is_none());
    }

    fn new_i64(&mut self, def: Def, val: i64) {
        let mut new_val = 0;
        new_val.view_bits_mut::<Lsb0>().store(val);
        assert!(self.bv64_vals.insert(def, new_val).is_none());
    }

    fn new_i128(&mut self, def: Def, val: i128) {
        let mut new_val = BitArray::<[u64; 2], Lsb0>::ZERO;
        new_val.fill_with(|idx| (val >> idx) & 1 != 0);
        assert!(self.bv128_vals.insert(def, new_val.load()).is_none());
    }

    fn new_u8(&mut self, def: Def, val: u8) {
        assert!(self.bv8_vals.insert(def, val).is_none());
    }

    fn new_u16(&mut self, def: Def, val: u16) {
        assert!(self.bv16_vals.insert(def, val).is_none());
    }

    fn new_u32(&mut self, def: Def, val: u32) {
        assert!(self.bv32_vals.insert(def, val).is_none());
    }

    fn new_u64(&mut self, def: Def, val: u64) {
        assert!(self.bv64_vals.insert(def, val).is_none());
    }

    fn new_u128(&mut self, def: Def, val: u128) {
        assert!(self.bv128_vals.insert(def, val).is_none());
    }

    // fn new_f16(&mut self, def: Def, val: u16) {
    //     self.f16_vals
    //         .insert(def, val)
    //         .expect("Builder guarantees well-formed IR");
    // }

    // fn new_f32(&mut self, def: Def, val: f32) {
    //     self.bv32_vals
    //         .insert(def, val.to_bits())
    //         .expect("Builder guarantees well-formed IR");
    // }

    // fn new_f64(&mut self, def: Def, val: f64) {
    //     self.bv64_vals
    //         .insert(def, val.to_bits())
    //         .expect("Builder guarantees well-formed IR");
    // }

    fn read_reg(&mut self, res: IRRes, op: RRegOp, ctx: &ProcCtx<P>) {
        // TODO: this fails for reg-mapped devices
        use RRegOp::*;
        match op {
            Bit(reg) => self.new_bool(res.def0.unwrap(), ctx.reg_data[reg.0]),
            Bv8(reg) => self.new_u8(res.def0.unwrap(), ctx.reg_data[reg.0..(reg.0 + 8)].load()),
            Bv16(reg) => self.new_u16(res.def0.unwrap(), ctx.reg_data[reg.0..(reg.0 + 16)].load()),
            Bv32(reg) => self.new_u32(res.def0.unwrap(), ctx.reg_data[reg.0..(reg.0 + 32)].load()),
            Bv64(reg) => self.new_u64(res.def0.unwrap(), ctx.reg_data[reg.0..(reg.0 + 64)].load()),
            Bv128(reg) => {
                self.new_u128(res.def0.unwrap(), ctx.reg_data[reg.0..(reg.0 + 128)].load())
            }
        };
    }

    fn write_reg(&mut self, op: WRegOp, ctx: &mut ProcCtx<P>) {
        // TODO: this fails for reg-mapped devices
        // TODO: this ignore reg-write instrumentation
        use WRegOp::*;
        match op {
            Bit(reg, val) => *ctx.reg_data.get_mut(reg.0).unwrap() = self.bool(val),
            Bv8(reg, val) => {
                (&mut ctx.reg_data[reg.0..(reg.0 + 8)]).store(self.u8(val));
            }
            Bv16(reg, val) => {
                (&mut ctx.reg_data[reg.0..(reg.0 + 16)]).store(self.u16(val));
            }
            Bv32(reg, val) => {
                (&mut ctx.reg_data[reg.0..(reg.0 + 32)]).store(self.u32(val));
            }
            Bv64(reg, val) => {
                (&mut ctx.reg_data[reg.0..(reg.0 + 64)]).store(self.u64(val));
            }
            Bv128(reg, val) => {
                (&mut ctx.reg_data[reg.0..(reg.0 + 128)]).store(self.u128(val));
            }
        }
    }

    fn read_mem(&mut self, res: IRRes, op: RMemOp<P>, proc: &P, ctx: &mut ProcCtx<P>) {
        use RMemOp::*;
        match op {
            Bv8(addr, attrs) => {
                let read = proc.ld(self.addr(addr), 1, attrs, ctx).0.unwrap();
                let mut data = [0; 1];
                data.copy_from_slice(&read[0..1]);
                self.new_u8(res.def0.unwrap(), u8::from_le_bytes(data))
            }
            Bv16(addr, attrs) => {
                let read = proc.ld(self.addr(addr), 2, attrs, ctx).0.unwrap();
                let mut data = [0; 2];
                data.copy_from_slice(&read[0..2]);
                self.new_u16(res.def0.unwrap(), u16::from_le_bytes(data))
            }
            Bv32(addr, attrs) => {
                let read = proc.ld(self.addr(addr), 4, attrs, ctx).0.unwrap();
                let mut data = [0; 4];
                data.copy_from_slice(&read[0..4]);
                self.new_u32(res.def0.unwrap(), u32::from_le_bytes(data))
            }
            Bv64(addr, attrs) => {
                let read = proc.ld(self.addr(addr), 8, attrs, ctx).0.unwrap();
                let mut data = [0; 8];
                data.copy_from_slice(&read[0..8]);
                self.new_u64(res.def0.unwrap(), u64::from_le_bytes(data))
            }
            Bv128(addr, attrs) => self.new_u128(
                res.def0.unwrap(),
                u128::from_le_bytes(proc.ld(self.addr(addr), 16, attrs, ctx).0.unwrap()),
            ),
        }
    }

    fn write_mem(&mut self, op: WMemOp<P>, proc: &P, ctx: &mut ProcCtx<P>) {
        use WMemOp::*;
        match op {
            Bv8(addr, val, attrs) => proc
                .st(self.addr(addr), &self.u8(val).to_le_bytes(), attrs, ctx)
                .0
                .unwrap(),
            Bv16(addr, val, attrs) => proc
                .st(self.addr(addr), &self.u16(val).to_le_bytes(), attrs, ctx)
                .0
                .unwrap(),
            Bv32(addr, val, attrs) => proc
                .st(self.addr(addr), &self.u32(val).to_le_bytes(), attrs, ctx)
                .0
                .unwrap(),
            Bv64(addr, val, attrs) => proc
                .st(self.addr(addr), &self.u64(val).to_le_bytes(), attrs, ctx)
                .0
                .unwrap(),
            Bv128(addr, val, attrs) => proc
                .st(self.addr(addr), &self.u128(val).to_le_bytes(), attrs, ctx)
                .0
                .unwrap(),
        }
    }

    fn eval_call_args(&self, args: Vec<CallParam>) -> Vec<CallArg> {
        args.into_iter()
            .map(|arg| match arg {
                CallParam::Bit(val) => CallArg::Bit(self.bool(val)),
                CallParam::Bv8(val) => CallArg::Bv8(self.u8(val)),
                CallParam::Bv16(val) => CallArg::Bv16(self.u16(val)),
                CallParam::Bv32(val) => CallArg::Bv32(self.u32(val)),
                CallParam::Bv64(val) => CallArg::Bv64(self.u64(val)),
                CallParam::Bv128(val) => CallArg::Bv128(self.u128(val)),
            })
            .collect()
    }

    fn call(&mut self, res: IRRes, op: CallOp<P>, proc: &P, ctx: &mut ProcCtx<P>) {
        use CallOp::*;
        match op {
            Unit { mut f, args, .. } => f(proc, ctx, self.eval_call_args(args)),
            Bit { mut f, args, .. } => {
                self.new_bool(res.def0.unwrap(), f(proc, ctx, self.eval_call_args(args)))
            }
            Bv8 { mut f, args, .. } => {
                self.new_u8(res.def0.unwrap(), f(proc, ctx, self.eval_call_args(args)))
            }
            Bv16 { mut f, args, .. } => {
                self.new_u16(res.def0.unwrap(), f(proc, ctx, self.eval_call_args(args)))
            }
            Bv32 { mut f, args, .. } => {
                self.new_u32(res.def0.unwrap(), f(proc, ctx, self.eval_call_args(args)))
            }
            Bv64 { mut f, args, .. } => {
                self.new_u64(res.def0.unwrap(), f(proc, ctx, self.eval_call_args(args)))
            }
            Bv128 { mut f, args, .. } => {
                self.new_u128(res.def0.unwrap(), f(proc, ctx, self.eval_call_args(args)))
            }
        }
    }

    // fn addr_op(&mut self, res: IRRes, op: AddrOp) {
    //     use AddrOp::*;
    //     match op {
    //         // Arithmetic
    //         Add(lhs, rhs) => self.new_addr(
    //             res.def0.unwrap(),
    //             self.addr(lhs).overflowing_add(self.addr(rhs)).0,
    //         ),
    //         Sub(lhs, rhs) => self.new_addr(
    //             res.def0.unwrap(),
    //             self.addr(lhs).overflowing_sub(self.addr(rhs)).0,
    //         ),
    //         // Conversion
    //         FromBv16(val) => self.new_addr(res.def0.unwrap(), self.i16(val) as usize),
    //         FromBv32(val) => self.new_addr(res.def0.unwrap(), self.i32(val) as usize),
    //         FromBv64(val) => self.new_addr(res.def0.unwrap(), self.i64(val) as usize),
    //         ToBv16(val) => self.new_u16(res.def0.unwrap(), self.addr(val) as u16),
    //         ToBv32(val) => self.new_u32(res.def0.unwrap(), self.addr(val) as u32),
    //         ToBv64(val) => self.new_u64(res.def0.unwrap(), self.addr(val) as u64),
    //     }
    // }

    fn bool_op(&mut self, res: IRRes, op: BoolOp) {
        use BoolOp::*;
        match op {
            // Logic
            And(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.bool(lhs) & self.bool(rhs)),
            Not(val) => self.new_bool(res.def0.unwrap(), !self.bool(val)),
            Or(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.bool(lhs) | self.bool(rhs)),
            Xor(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.bool(lhs) ^ self.bool(rhs)),
            // Bool equality
            Eq(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.bool(lhs) == self.bool(rhs)),
            Ne(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.bool(lhs) != self.bool(rhs)),
            // Ternary
            Select(cond, t_case, f_case) => {
                if self.bool(cond) {
                    self.new_bool(res.def0.unwrap(), self.bool(t_case));
                } else {
                    self.new_bool(res.def0.unwrap(), self.bool(f_case));
                }
            }
            // Int equality
            EqInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u8(lhs) == self.u8(rhs)),
            NeInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u8(lhs) != self.u8(rhs)),
            EqInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u16(lhs) == self.u16(rhs)),
            NeInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u16(lhs) != self.u16(rhs)),
            EqInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u32(lhs) == self.u32(rhs)),
            NeInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u32(lhs) != self.u32(rhs)),
            EqInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u64(lhs) == self.u64(rhs)),
            NeInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u64(lhs) != self.u64(rhs)),
            // Int8 ordering
            SLtInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i8(lhs) < self.i8(rhs)),
            SLeInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i8(lhs) <= self.i8(rhs)),
            SGtInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i8(lhs) > self.i8(rhs)),
            SGeInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i8(lhs) >= self.i8(rhs)),
            ULtInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u8(lhs) < self.u8(rhs)),
            ULeInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u8(lhs) <= self.u8(rhs)),
            UGtInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u8(lhs) > self.u8(rhs)),
            UGeInt8(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u8(lhs) >= self.u8(rhs)),
            // Int16 ordering
            SLtInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i16(lhs) < self.i16(rhs)),
            SLeInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i16(lhs) <= self.i16(rhs)),
            SGtInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i16(lhs) > self.i16(rhs)),
            SGeInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i16(lhs) >= self.i16(rhs)),
            ULtInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u16(lhs) < self.u16(rhs)),
            ULeInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u16(lhs) <= self.u16(rhs)),
            UGtInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u16(lhs) > self.u16(rhs)),
            UGeInt16(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u16(lhs) >= self.u16(rhs)),
            // Int32 ordering
            SLtInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i32(lhs) < self.i32(rhs)),
            SLeInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i32(lhs) <= self.i32(rhs)),
            SGtInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i32(lhs) > self.i32(rhs)),
            SGeInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i32(lhs) >= self.i32(rhs)),
            ULtInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u32(lhs) < self.u32(rhs)),
            ULeInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u32(lhs) <= self.u32(rhs)),
            UGtInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u32(lhs) > self.u32(rhs)),
            UGeInt32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u32(lhs) >= self.u32(rhs)),
            // Int64 ordering
            SLtInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i64(lhs) < self.i64(rhs)),
            SLeInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i64(lhs) <= self.i64(rhs)),
            SGtInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i64(lhs) > self.i64(rhs)),
            SGeInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.i64(lhs) >= self.i64(rhs)),
            ULtInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u64(lhs) < self.u64(rhs)),
            ULeInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u64(lhs) <= self.u64(rhs)),
            UGtInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u64(lhs) > self.u64(rhs)),
            UGeInt64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.u64(lhs) >= self.u64(rhs)),
            // Float equality
            EqFloat16(_lhs, _rhs) => todo!("Float16 support"),
            NeFloat16(_lhs, _rhs) => todo!("Float16 support"),
            EqFloat32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f32(lhs) == self.f32(rhs)),
            NeFloat32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f32(lhs) != self.f32(rhs)),
            EqFloat64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f64(lhs) == self.f64(rhs)),
            NeFloat64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f64(lhs) != self.f64(rhs)),
            // Float ordering
            LtFloat16(_lhs, _rhs) => todo!("Float16 support"),
            LeFloat16(_lhs, _rhs) => todo!("Float16 support"),
            GtFloat16(_lhs, _rhs) => todo!("Float16 support"),
            GeFloat16(_lhs, _rhs) => todo!("Float16 support"),
            LtFloat32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f32(lhs) < self.f32(rhs)),
            LeFloat32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f32(lhs) <= self.f32(rhs)),
            GtFloat32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f32(lhs) > self.f32(rhs)),
            GeFloat32(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f32(lhs) >= self.f32(rhs)),
            LtFloat64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f64(lhs) < self.f64(rhs)),
            LeFloat64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f64(lhs) <= self.f64(rhs)),
            GtFloat64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f64(lhs) > self.f64(rhs)),
            GeFloat64(lhs, rhs) => self.new_bool(res.def0.unwrap(), self.f64(lhs) >= self.f64(rhs)),
            // Bit access
            NthBitBv8(val, n) => self.new_bool(
                res.def0.unwrap(),
                self.u8(val)
                    .checked_shr(self.u8(n) as u32)
                    .and_then(|v| Some(v & 1 != 0))
                    .unwrap_or(false),
            ),
            NthBitBv16(val, n) => self.new_bool(
                res.def0.unwrap(),
                self.u16(val)
                    .checked_shr(self.u16(n) as u32)
                    .and_then(|v| Some(v & 1 != 0))
                    .unwrap_or(false),
            ),
            NthBitBv32(val, n) => self.new_bool(
                res.def0.unwrap(),
                self.u32(val)
                    .checked_shr(self.u32(n) as u32)
                    .and_then(|v| Some(v & 1 != 0))
                    .unwrap_or(false),
            ),
            NthBitBv64(val, n) => self.new_bool(
                res.def0.unwrap(),
                self.u64(val)
                    .checked_shr(self.u64(n) as u32)
                    .and_then(|v| Some(v & 1 != 0))
                    .unwrap_or(false),
            ),
        }
    }

    fn int8_op(&mut self, res: IRRes, op: Int8Op) {
        use Int8Op::*;
        match op {
            // Logic
            And(lhs, rhs) => self.new_u8(res.def0.unwrap(), self.u8(lhs) & self.u8(rhs)),
            Not(val) => self.new_u8(res.def0.unwrap(), !self.u8(val)),
            Or(lhs, rhs) => self.new_u8(res.def0.unwrap(), self.u8(lhs) | self.u8(rhs)),
            Xor(lhs, rhs) => self.new_u8(res.def0.unwrap(), self.u8(lhs) ^ self.u8(rhs)),
            // Bit manipulation
            Asr(lhs, rhs) => self.new_i8(
                res.def0.unwrap(),
                self.i8(lhs).overflowing_shr(self.u8(rhs) as u32).0,
            ),
            Fshl(hi, lo, shift) => {
                let (hi, lo) = (self.u8(hi) as u16, self.u8(lo) as u16);
                let concat = (hi << 8) | lo;
                let shift = shift % 8;
                self.new_u8(
                    res.def0.unwrap(),
                    (concat.overflowing_shl(shift as u32).0 >> u8::BITS) as u8,
                );
            }
            Fshr(hi, lo, shift) => {
                let (hi, lo) = (self.u8(hi) as u16, self.u8(lo) as u16);
                let concat = (hi << 8) | lo;
                let shift = shift % 8;
                self.new_u8(
                    res.def0.unwrap(),
                    (concat.overflowing_shr(shift as u32).0 & u8::MAX as u16) as u8,
                );
            }
            Lsl(lhs, rhs) => self.new_u8(
                res.def0.unwrap(),
                self.u8(lhs).overflowing_shl(self.u8(rhs) as u32).0,
            ),
            Lsr(lhs, rhs) => self.new_u8(
                res.def0.unwrap(),
                self.u8(lhs).overflowing_shr(self.u8(rhs) as u32).0,
            ),
            Ror(lhs, rhs) => self.new_u8(
                res.def0.unwrap(),
                self.u8(lhs).rotate_right(self.u8(rhs) as u32),
            ),
            BitRev(val) => self.new_u8(res.def0.unwrap(), self.u8(val).reverse_bits() as u8),
            Ctls(val) => {
                let val = self.u8(val);
                let result = if (val >> 7) & 1 != 0 {
                    val.leading_ones() - 1
                } else {
                    val.leading_zeros() - 1
                } as u8;
                self.new_u8(res.def0.unwrap(), result);
            }
            Ctlz(val) => self.new_u8(res.def0.unwrap(), self.u8(val).leading_zeros() as u8),
            Ctset(val) => self.new_u8(res.def0.unwrap(), self.u8(val).count_ones() as u8),
            Cttz(val) => self.new_u8(res.def0.unwrap(), self.u8(val).trailing_zeros() as u8),
            // Arithmetic
            Abs(val) => {
                let val = self.i8(val).abs();
                self.new_i8(res.def0.unwrap(), val);
            }
            SAdd(lhs, rhs) => {
                let lhs = self.i8(lhs);
                let rhs = self.i8(rhs);

                let (result, overflow) = lhs.overflowing_add(rhs);
                self.new_i8(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SDiv(lhs, rhs) => {
                let lhs = self.i8(lhs);
                let rhs = self.i8(rhs);

                let result = lhs.checked_div(rhs).unwrap_or(0);
                self.new_i8(res.def0.unwrap(), result);
            }
            SMul(lhs, rhs) => {
                let lhs = self.i8(lhs);
                let rhs = self.i8(rhs);

                let (result, overflow) = lhs.overflowing_mul(rhs);
                self.new_i8(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SRem(lhs, rhs) => {
                let lhs = self.i8(lhs);
                let rhs = self.i8(rhs);

                let result = lhs.checked_rem(rhs).unwrap_or(0);
                self.new_i8(res.def0.unwrap(), result);
            }
            SSub(lhs, rhs) => {
                let lhs = self.i8(lhs);
                let rhs = self.i8(rhs);

                let (result, overflow) = lhs.overflowing_sub(rhs);
                self.new_i8(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UAdd(lhs, rhs) => {
                let (result, overflow) = self.u8(lhs).overflowing_add(self.u8(rhs));
                self.new_u8(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UDiv(lhs, rhs) => {
                let result = self.u8(lhs).checked_div(self.u8(rhs)).unwrap_or(0);
                self.new_u8(res.def0.unwrap(), result);
            }
            UMul(lhs, rhs) => {
                let (result, overflow) = self.u8(lhs).overflowing_mul(self.u8(rhs));
                self.new_u8(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            URem(lhs, rhs) => {
                let result = self.u8(lhs).checked_rem(self.u8(rhs)).unwrap_or(0);
                self.new_u8(res.def0.unwrap(), result);
            }
            USub(lhs, rhs) => {
                let (result, overflow) = self.u8(lhs).carrying_add(!self.u8(rhs), true);
                // let (result, overflow) = self.u8(lhs).overflowing_sub(self.u8(rhs));
                // TODO: this uses a nightly feature
                self.new_u8(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            // Min/max
            SMax(lhs, rhs) => {
                self.new_i8(res.def0.unwrap(), max(self.i8(lhs), self.i8(rhs)));
            }
            SMin(lhs, rhs) => {
                self.new_i8(res.def0.unwrap(), min(self.i8(lhs), self.i8(rhs)));
            }
            UMax(lhs, rhs) => {
                self.new_u8(res.def0.unwrap(), max(self.u8(lhs), self.u8(rhs)));
            }
            UMin(lhs, rhs) => {
                self.new_u8(res.def0.unwrap(), min(self.u8(lhs), self.u8(rhs)));
            }
            // Ternary
            Select(cond, t_case, f_case) => {
                if self.bool(cond) {
                    self.new_u8(res.def0.unwrap(), self.u8(t_case));
                } else {
                    self.new_u8(res.def0.unwrap(), self.u8(f_case));
                }
            }
            // Conversion
            All(bit) => {
                let val = if self.bool(bit) { u8::MAX } else { 0 };
                self.new_u8(res.def0.unwrap(), val);
            }
            FromBit(bit) => {
                let val = if self.bool(bit) { 1 } else { 0 };
                self.new_u8(res.def0.unwrap(), val);
            }
            ReplBit(bit, n) => {
                let val = if self.bool(bit) {
                    let n = self.u8(n);
                    if n >= 8 {
                        u8::MAX
                    } else {
                        (1 << n) - 1
                    }
                } else {
                    0
                };
                self.new_u8(res.def0.unwrap(), val);
            }
            Split(val) => {
                let val = self.u16(val);
                self.new_u8(
                    res.def0.unwrap(),
                    ((val >> u8::BITS as u16) & u8::MAX as u16) as u8,
                );
                self.new_u8(res.def1.unwrap(), (val & u8::MAX as u16) as u8);
            }
            TruncBv16(val) => {
                let val = self.u16(val);
                self.new_u8(res.def0.unwrap(), (val & u8::MAX as u16) as u8);
            }
            TruncBv32(val) => {
                let val = self.u32(val);
                self.new_u8(res.def0.unwrap(), (val & u8::MAX as u32) as u8);
            }
            TruncBv64(val) => {
                let val = self.u64(val);
                self.new_u8(res.def0.unwrap(), (val & u8::MAX as u64) as u8);
            }
        }
    }

    fn int16_op(&mut self, res: IRRes, op: Int16Op) {
        use Int16Op::*;
        match op {
            // Logic
            And(lhs, rhs) => self.new_u16(res.def0.unwrap(), self.u16(lhs) & self.u16(rhs)),
            Not(val) => self.new_u16(res.def0.unwrap(), !self.u16(val)),
            Or(lhs, rhs) => self.new_u16(res.def0.unwrap(), self.u16(lhs) | self.u16(rhs)),
            Xor(lhs, rhs) => self.new_u16(res.def0.unwrap(), self.u16(lhs) ^ self.u16(rhs)),
            // Bit manipulation
            Asr(lhs, rhs) => self.new_i16(
                res.def0.unwrap(),
                self.i16(lhs).overflowing_shr(self.u16(rhs) as u32).0,
            ),
            Fshl(hi, lo, shift) => {
                let (hi, lo) = (self.u16(hi) as u32, self.u16(lo) as u32);
                let concat = (hi << 16) | lo;
                let shift = shift % 16;
                self.new_u16(
                    res.def0.unwrap(),
                    (concat.overflowing_shl(shift as u32).0 >> u16::BITS) as u16,
                );
            }
            Fshr(hi, lo, shift) => {
                let (hi, lo) = (self.u16(hi) as u32, self.u16(lo) as u32);
                let concat = (hi << 16) | lo;
                let shift = shift % 16;
                self.new_u16(
                    res.def0.unwrap(),
                    (concat.overflowing_shr(shift as u32).0 & u16::MAX as u32) as u16,
                );
            }
            Lsl(lhs, rhs) => self.new_u16(
                res.def0.unwrap(),
                self.u16(lhs).overflowing_shl(self.u16(rhs) as u32).0,
            ),
            Lsr(lhs, rhs) => self.new_u16(
                res.def0.unwrap(),
                self.u16(lhs).overflowing_shr(self.u16(rhs) as u32).0,
            ),
            Ror(lhs, rhs) => self.new_u16(
                res.def0.unwrap(),
                self.u16(lhs).rotate_right(self.u16(rhs) as u32),
            ),
            BitRev(val) => self.new_u16(res.def0.unwrap(), self.u16(val).reverse_bits() as u16),
            ByteRev(val) => self.new_u16(res.def0.unwrap(), self.u16(val).swap_bytes() as u16),
            Ctls(val) => {
                let val = self.u16(val);
                let result = if (val >> 15) & 1 != 0 {
                    val.leading_ones() - 1
                } else {
                    val.leading_zeros() - 1
                } as u16;
                self.new_u16(res.def0.unwrap(), result);
            }
            Ctlz(val) => self.new_u16(res.def0.unwrap(), self.u16(val).leading_zeros() as u16),
            Ctset(val) => self.new_u16(res.def0.unwrap(), self.u16(val).count_ones() as u16),
            Cttz(val) => self.new_u16(res.def0.unwrap(), self.u16(val).trailing_zeros() as u16),
            // Arithmetic
            Abs(val) => {
                let val = self.i16(val).abs();
                self.new_i16(res.def0.unwrap(), val);
            }
            SAdd(lhs, rhs) => {
                let lhs = self.i16(lhs);
                let rhs = self.i16(rhs);

                let (result, overflow) = lhs.overflowing_add(rhs);
                self.new_i16(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SDiv(lhs, rhs) => {
                let lhs = self.i16(lhs);
                let rhs = self.i16(rhs);

                let result = lhs.checked_div(rhs).unwrap_or(0);
                self.new_i16(res.def0.unwrap(), result);
            }
            SMul(lhs, rhs) => {
                let lhs = self.i16(lhs);
                let rhs = self.i16(rhs);

                let (result, overflow) = lhs.overflowing_mul(rhs);
                self.new_i16(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SRem(lhs, rhs) => {
                let lhs = self.i16(lhs);
                let rhs = self.i16(rhs);

                let result = lhs.checked_rem(rhs).unwrap_or(0);
                self.new_i16(res.def0.unwrap(), result);
            }
            SSub(lhs, rhs) => {
                let lhs = self.i16(lhs);
                let rhs = self.i16(rhs);

                let (result, overflow) = lhs.overflowing_sub(rhs);
                self.new_i16(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UAdd(lhs, rhs) => {
                let (result, overflow) = self.u16(lhs).overflowing_add(self.u16(rhs));
                self.new_u16(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UDiv(lhs, rhs) => {
                let result = self.u16(lhs).checked_div(self.u16(rhs)).unwrap_or(0);
                self.new_u16(res.def0.unwrap(), result);
            }
            UMul(lhs, rhs) => {
                let (result, overflow) = self.u16(lhs).overflowing_mul(self.u16(rhs));
                self.new_u16(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            URem(lhs, rhs) => {
                let result = self.u16(lhs).checked_rem(self.u16(rhs)).unwrap_or(0);
                self.new_u16(res.def0.unwrap(), result);
            }
            USub(lhs, rhs) => {
                let (result, overflow) = self.u16(lhs).carrying_add(!self.u16(rhs), true);
                // let (result, overflow) = self.u16(lhs).overflowing_sub(self.u16(rhs));
                // TODO: this uses a nightly feature
                self.new_u16(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            // Min/max
            SMax(lhs, rhs) => {
                self.new_i16(res.def0.unwrap(), max(self.i16(lhs), self.i16(rhs)));
            }
            SMin(lhs, rhs) => {
                self.new_i16(res.def0.unwrap(), min(self.i16(lhs), self.i16(rhs)));
            }
            UMax(lhs, rhs) => {
                self.new_u16(res.def0.unwrap(), max(self.u16(lhs), self.u16(rhs)));
            }
            UMin(lhs, rhs) => {
                self.new_u16(res.def0.unwrap(), min(self.u16(lhs), self.u16(rhs)));
            }
            // Ternary
            Select(cond, t_case, f_case) => {
                if self.bool(cond) {
                    self.new_u16(res.def0.unwrap(), self.u16(t_case));
                } else {
                    self.new_u16(res.def0.unwrap(), self.u16(f_case));
                }
            }
            // Conversion
            All(bit) => {
                let val = if self.bool(bit) { u16::MAX } else { 0 };
                self.new_u16(res.def0.unwrap(), val);
            }
            FromBit(bit) => {
                let val = if self.bool(bit) { 1 } else { 0 };
                self.new_u16(res.def0.unwrap(), val);
            }
            ReplBit(bit, n) => {
                let val = if self.bool(bit) {
                    let n = self.u16(n);
                    if n >= 16 {
                        u16::MAX
                    } else {
                        (1 << n) - 1
                    }
                } else {
                    0
                };
                self.new_u16(res.def0.unwrap(), val);
            }
            Concat(hi, lo) => {
                let hi = self.u8(hi) as u16;
                let lo = self.u8(lo) as u16;
                self.new_u16(res.def0.unwrap(), ((hi << u8::BITS as u32) | lo) as u16);
            }
            Split(val) => {
                let val = self.u32(val);
                self.new_u16(
                    res.def0.unwrap(),
                    ((val >> u16::BITS as u32) & u16::MAX as u32) as u16,
                );
                self.new_u16(res.def1.unwrap(), (val & u16::MAX as u32) as u16);
            }
            SextBv8(val) => {
                let val = self.i8(val) as i16;
                self.new_i16(res.def0.unwrap(), val);
            }
            TruncBv32(val) => {
                let val = self.u32(val);
                self.new_u16(res.def0.unwrap(), (val & u16::MAX as u32) as u16);
            }
            TruncBv64(val) => {
                let val = self.u64(val);
                self.new_u16(res.def0.unwrap(), (val & u16::MAX as u64) as u16);
            }
            ZextBv8(val) => {
                let val = self.u8(val) as u16;
                self.new_u16(res.def0.unwrap(), val);
            }
        }
    }

    fn int32_op(&mut self, res: IRRes, op: Int32Op) {
        use Int32Op::*;
        match op {
            // Logic
            And(lhs, rhs) => self.new_u32(res.def0.unwrap(), self.u32(lhs) & self.u32(rhs)),
            Not(val) => self.new_u32(res.def0.unwrap(), !self.u32(val)),
            Or(lhs, rhs) => self.new_u32(res.def0.unwrap(), self.u32(lhs) | self.u32(rhs)),
            Xor(lhs, rhs) => self.new_u32(res.def0.unwrap(), self.u32(lhs) ^ self.u32(rhs)),
            // Bit manipulation
            Asr(lhs, rhs) => self.new_i32(
                res.def0.unwrap(),
                self.i32(lhs).overflowing_shr(self.u32(rhs) as u32).0,
            ),
            Fshl(hi, lo, shift) => {
                let (hi, lo) = (self.u32(hi) as u64, self.u32(lo) as u64);
                let concat = (hi << 32) | lo;
                let shift = shift % 32;
                self.new_u32(
                    res.def0.unwrap(),
                    (concat.overflowing_shl(shift as u32).0 >> u32::BITS) as u32,
                );
            }
            Fshr(hi, lo, shift) => {
                let (hi, lo) = (self.u32(hi) as u64, self.u32(lo) as u64);
                let concat = (hi << 32) | lo;
                let shift = shift % 32;
                self.new_u32(
                    res.def0.unwrap(),
                    (concat.overflowing_shr(shift as u32).0 & u32::MAX as u64) as u32,
                );
            }
            Lsl(lhs, rhs) => self.new_u32(
                res.def0.unwrap(),
                self.u32(lhs).overflowing_shl(self.u32(rhs) as u32).0,
            ),
            Lsr(lhs, rhs) => self.new_u32(
                res.def0.unwrap(),
                self.u32(lhs).overflowing_shr(self.u32(rhs) as u32).0,
            ),
            Ror(lhs, rhs) => self.new_u32(
                res.def0.unwrap(),
                self.u32(lhs).rotate_right(self.u32(rhs) as u32),
            ),
            BitRev(val) => self.new_u32(res.def0.unwrap(), self.u32(val).reverse_bits() as u32),
            ByteRev(val) => self.new_u32(res.def0.unwrap(), self.u32(val).swap_bytes() as u32),
            Ctls(val) => {
                let val = self.u32(val);
                let result = if (val >> 31) & 1 != 0 {
                    val.leading_ones() - 1
                } else {
                    val.leading_zeros() - 1
                };
                self.new_u32(res.def0.unwrap(), result);
            }
            Ctlz(val) => self.new_u32(res.def0.unwrap(), self.u32(val).leading_zeros() as u32),
            Ctset(val) => self.new_u32(res.def0.unwrap(), self.u32(val).count_ones() as u32),
            Cttz(val) => self.new_u32(res.def0.unwrap(), self.u32(val).trailing_zeros() as u32),
            // Arithmetic
            Abs(val) => {
                let val = self.i32(val).abs();
                self.new_i32(res.def0.unwrap(), val);
            }
            SAdd(lhs, rhs) => {
                let lhs = self.i32(lhs);
                let rhs = self.i32(rhs);

                let (result, overflow) = lhs.overflowing_add(rhs);
                self.new_i32(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SDiv(lhs, rhs) => {
                let lhs = self.i32(lhs);
                let rhs = self.i32(rhs);

                let result = lhs.checked_div(rhs).unwrap_or(0);
                self.new_i32(res.def0.unwrap(), result);
            }
            SMul(lhs, rhs) => {
                let lhs = self.i32(lhs);
                let rhs = self.i32(rhs);

                let (result, overflow) = lhs.overflowing_mul(rhs);
                self.new_i32(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SRem(lhs, rhs) => {
                let lhs = self.i32(lhs);
                let rhs = self.i32(rhs);

                let result = lhs.checked_rem(rhs).unwrap_or(0);
                self.new_i32(res.def0.unwrap(), result);
            }
            SSub(lhs, rhs) => {
                let lhs = self.i32(lhs);
                let rhs = self.i32(rhs);

                let (result, overflow) = lhs.overflowing_sub(rhs);
                self.new_i32(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UAdd(lhs, rhs) => {
                let (result, overflow) = self.u32(lhs).overflowing_add(self.u32(rhs));
                self.new_u32(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UDiv(lhs, rhs) => {
                let result = self.u32(lhs).checked_div(self.u32(rhs)).unwrap_or(0);
                self.new_u32(res.def0.unwrap(), result);
            }
            UMul(lhs, rhs) => {
                let (result, overflow) = self.u32(lhs).overflowing_mul(self.u32(rhs));
                self.new_u32(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            URem(lhs, rhs) => {
                let result = self.u32(lhs).checked_rem(self.u32(rhs)).unwrap_or(0);
                self.new_u32(res.def0.unwrap(), result);
            }
            USub(lhs, rhs) => {
                let (result, overflow) = self.u32(lhs).carrying_add(!self.u32(rhs), true);
                // let (result, overflow) = self.u32(lhs).overflowing_sub(self.u32(rhs));
                // TODO: this uses a nightly feature
                self.new_u32(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            // Min/max
            SMax(lhs, rhs) => {
                self.new_i32(res.def0.unwrap(), max(self.i32(lhs), self.i32(rhs)));
            }
            SMin(lhs, rhs) => {
                self.new_i32(res.def0.unwrap(), min(self.i32(lhs), self.i32(rhs)));
            }
            UMax(lhs, rhs) => {
                self.new_u32(res.def0.unwrap(), max(self.u32(lhs), self.u32(rhs)));
            }
            UMin(lhs, rhs) => {
                self.new_u32(res.def0.unwrap(), min(self.u32(lhs), self.u32(rhs)));
            }
            // Ternary
            Select(cond, t_case, f_case) => {
                if self.bool(cond) {
                    self.new_i32(res.def0.unwrap(), self.i32(t_case));
                } else {
                    self.new_i32(res.def0.unwrap(), self.i32(f_case));
                }
            }
            // Conversion
            All(bit) => {
                let val = if self.bool(bit) { u32::MAX } else { 0 };
                self.new_u32(res.def0.unwrap(), val);
            }
            FromBit(bit) => {
                let val = if self.bool(bit) { 1 } else { 0 };
                self.new_u32(res.def0.unwrap(), val);
            }
            ReplBit(bit, n) => {
                let val = if self.bool(bit) {
                    let n = self.u32(n);
                    if n >= 32 {
                        u32::MAX
                    } else {
                        (1 << n) - 1
                    }
                } else {
                    0
                };
                self.new_u32(res.def0.unwrap(), val);
            }
            Concat(hi, lo) => {
                let hi = self.i16(hi) as u32;
                let lo = self.i16(lo) as u32;
                self.new_u32(res.def0.unwrap(), ((hi << u16::BITS as u32) | lo) as u32);
            }
            Split(val) => {
                let val = self.u64(val);
                self.new_u32(
                    res.def0.unwrap(),
                    ((val >> u32::BITS as u64) & u32::MAX as u64) as u32,
                );
                self.new_u32(res.def1.unwrap(), (val & u32::MAX as u64) as u32);
            }
            SextBv8(val) => {
                let val = self.i8(val) as i32;
                self.new_i32(res.def0.unwrap(), val);
            }
            SextBv16(val) => {
                let val = self.i16(val) as i32;
                self.new_i32(res.def0.unwrap(), val);
            }
            TruncBv64(val) => {
                let val = self.u64(val);
                self.new_u16(res.def0.unwrap(), (val & u16::MAX as u64) as u16);
            }
            ZextBv8(val) => {
                let val = self.u8(val) as u32;
                self.new_u32(res.def0.unwrap(), val);
            }
            ZextBv16(val) => {
                let val = self.u16(val) as u32;
                self.new_u32(res.def0.unwrap(), val);
            }
        }
    }

    fn int64_op(&mut self, res: IRRes, op: Int64Op) {
        use Int64Op::*;
        match op {
            // Logic
            And(lhs, rhs) => self.new_u64(res.def0.unwrap(), self.u64(lhs) & self.u64(rhs)),
            Not(val) => self.new_u64(res.def0.unwrap(), !self.u64(val)),
            Or(lhs, rhs) => self.new_u64(res.def0.unwrap(), self.u64(lhs) | self.u64(rhs)),
            Xor(lhs, rhs) => self.new_u64(res.def0.unwrap(), self.u64(lhs) ^ self.u64(rhs)),
            // Bit manipulation
            Asr(lhs, rhs) => self.new_i64(
                res.def0.unwrap(),
                self.i64(lhs)
                    .overflowing_shr(self.i64(rhs).try_into().unwrap_or(u32::MAX))
                    .0,
            ),
            Fshl(hi, lo, shift) => {
                let (hi, lo) = (self.u64(hi) as u128, self.u64(lo) as u128);
                let concat = (hi << 64) | lo;
                let shift = shift % 64;
                self.new_u64(
                    res.def0.unwrap(),
                    (concat
                        .overflowing_shl(shift.try_into().unwrap_or(u32::MAX))
                        .0
                        >> u64::BITS) as u64,
                );
            }
            Fshr(hi, lo, shift) => {
                let (hi, lo) = (self.u64(hi) as u128, self.u64(lo) as u128);
                let concat = (hi << 64) | lo;
                let shift = shift % 64;
                self.new_u64(
                    res.def0.unwrap(),
                    (concat.overflowing_shr(shift as u32).0 & u64::MAX as u128) as u64,
                );
            }
            Lsl(lhs, rhs) => self.new_u64(
                res.def0.unwrap(),
                self.u64(lhs)
                    .overflowing_shl(self.u64(rhs).try_into().unwrap_or(u32::MAX))
                    .0,
            ),
            Lsr(lhs, rhs) => self.new_u64(
                res.def0.unwrap(),
                self.u64(lhs)
                    .overflowing_shr(self.u64(rhs).try_into().unwrap_or(u32::MAX))
                    .0,
            ),
            Ror(lhs, rhs) => self.new_u64(
                res.def0.unwrap(),
                self.u64(lhs)
                    .rotate_right(self.u64(rhs).try_into().unwrap_or(u32::MAX)),
            ),
            BitRev(val) => self.new_u64(res.def0.unwrap(), self.u64(val).reverse_bits() as u64),
            ByteRev(val) => self.new_u64(res.def0.unwrap(), self.u64(val).swap_bytes() as u64),
            Ctls(val) => {
                let val = self.u64(val);
                let result = if (val >> 63) & 1 != 0 {
                    val.leading_ones() - 1
                } else {
                    val.leading_zeros() - 1
                } as u64;
                self.new_u64(res.def0.unwrap(), result);
            }
            Ctlz(val) => self.new_u64(res.def0.unwrap(), self.u64(val).leading_zeros() as u64),
            Ctset(val) => self.new_u64(res.def0.unwrap(), self.u64(val).count_ones() as u64),
            Cttz(val) => self.new_u64(res.def0.unwrap(), self.u64(val).trailing_zeros() as u64),
            // Arithmetic
            Abs(val) => {
                let val = self.i64(val).abs();
                self.new_i64(res.def0.unwrap(), val);
            }
            SAdd(lhs, rhs) => {
                let lhs = self.i64(lhs);
                let rhs = self.i64(rhs);

                let (result, overflow) = lhs.overflowing_add(rhs);
                self.new_i64(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SDiv(lhs, rhs) => {
                let lhs = self.i64(lhs);
                let rhs = self.i64(rhs);

                let result = lhs.checked_div(rhs).unwrap_or(0);
                self.new_i64(res.def0.unwrap(), result);
            }
            SMul(lhs, rhs) => {
                let lhs = self.i64(lhs);
                let rhs = self.i64(rhs);

                let (result, overflow) = lhs.overflowing_mul(rhs);
                self.new_i64(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SRem(lhs, rhs) => {
                let lhs = self.i64(lhs);
                let rhs = self.i64(rhs);

                let result = lhs.checked_rem(rhs).unwrap_or(0);
                self.new_i64(res.def0.unwrap(), result);
            }
            SSub(lhs, rhs) => {
                let lhs = self.i64(lhs);
                let rhs = self.i64(rhs);

                let (result, overflow) = lhs.overflowing_sub(rhs);
                self.new_i64(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UAdd(lhs, rhs) => {
                let lhs = self.u64(lhs);
                let rhs = self.u64(rhs);

                let (result, overflow) = lhs.overflowing_add(rhs);
                self.new_u64(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UDiv(lhs, rhs) => {
                let result = self.u64(lhs).checked_div(self.u64(rhs)).unwrap_or(0);
                self.new_u64(res.def0.unwrap(), result);
            }
            UMul(lhs, rhs) => {
                let (result, overflow) = self.u64(lhs).overflowing_mul(self.u64(rhs));
                self.new_u64(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            URem(lhs, rhs) => {
                let result = self.u64(lhs).checked_rem(self.u64(rhs)).unwrap_or(0);
                self.new_u64(res.def0.unwrap(), result);
            }
            USub(lhs, rhs) => {
                let (result, overflow) = self.u64(lhs).carrying_add(!self.u64(rhs), true);
                // let (result, overflow) = self.u64(lhs).overflowing_sub(self.u64(rhs));
                // TODO: this uses a nightly feature
                self.new_u64(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            // Min/max
            SMax(lhs, rhs) => {
                self.new_i64(res.def0.unwrap(), max(self.i64(lhs), self.i64(rhs)));
            }
            SMin(lhs, rhs) => {
                self.new_i64(res.def0.unwrap(), min(self.i64(lhs), self.i64(rhs)));
            }
            UMax(lhs, rhs) => {
                self.new_u64(res.def0.unwrap(), max(self.u64(lhs), self.u64(rhs)));
            }
            UMin(lhs, rhs) => {
                self.new_u64(res.def0.unwrap(), min(self.u64(lhs), self.u64(rhs)));
            }
            // Ternary
            Select(cond, t_case, f_case) => {
                if self.bool(cond) {
                    self.new_u64(res.def0.unwrap(), self.u64(t_case));
                } else {
                    self.new_u64(res.def0.unwrap(), self.u64(f_case));
                }
            }
            // Conversion
            All(bit) => {
                let val = if self.bool(bit) { u64::MAX } else { 0 };
                self.new_u64(res.def0.unwrap(), val);
            }
            FromBit(bit) => {
                let val = if self.bool(bit) { 1 } else { 0 };
                self.new_u64(res.def0.unwrap(), val);
            }
            Split(val) => {
                let val = self.u128(val);
                self.new_u64(
                    res.def0.unwrap(),
                    ((val >> u64::BITS as u128) & u64::MAX as u128) as u64,
                );
                self.new_u64(res.def1.unwrap(), (val & u64::MAX as u128) as u64);
            }
            ReplBit(bit, n) => {
                let val = if self.bool(bit) {
                    let n = self.u64(n);
                    if n >= 64 {
                        u64::MAX
                    } else {
                        (1 << n) - 1
                    }
                } else {
                    0
                };
                self.new_u64(res.def0.unwrap(), val);
            }
            Concat(hi, lo) => {
                let hi = self.i32(hi) as u64;
                let lo = self.i32(lo) as u64;
                self.new_u64(res.def0.unwrap(), ((hi << u32::BITS as u64) | lo) as u64);
            }
            SextBv8(val) => {
                let val = self.i8(val) as i64;
                self.new_i64(res.def0.unwrap(), val);
            }
            SextBv16(val) => {
                let val = self.i16(val) as i64;
                self.new_i64(res.def0.unwrap(), val);
            }
            SextBv32(val) => {
                let val = self.i32(val) as i64;
                self.new_i64(res.def0.unwrap(), val);
            }
            TruncBv128(val) => {
                let val = self.u128(val);
                self.new_u64(res.def0.unwrap(), (val & u64::MAX as u128) as u64);
            }
            ZextBv8(val) => {
                let val = self.u8(val) as u64;
                self.new_u64(res.def0.unwrap(), val);
            }
            ZextBv16(val) => {
                let val = self.u16(val) as u64;
                self.new_u64(res.def0.unwrap(), val);
            }
            ZextBv32(val) => {
                let val = self.u32(val) as u64;
                self.new_u64(res.def0.unwrap(), val);
            }
        }
    }

    fn int128_op(&mut self, res: IRRes, op: Int128Op) {
        use Int128Op::*;
        match op {
            Lsr(lhs, rhs) => self.new_u128(
                res.def0.unwrap(),
                self.u128(lhs)
                    .overflowing_shr(self.u128(rhs).try_into().unwrap_or(u32::MAX))
                    .0,
            ),
            SMul(lhs, rhs) => {
                let lhs = self.i128(lhs);
                let rhs = self.i128(rhs);

                let (result, overflow) = lhs.overflowing_mul(rhs);
                self.new_i128(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            UMul(lhs, rhs) => {
                let (result, overflow) = self.u128(lhs).overflowing_mul(self.u128(rhs));
                self.new_u128(res.def0.unwrap(), result);
                self.new_bool(res.def1.unwrap(), overflow);
            }
            SextBv8(val) => {
                let val = self.i8(val) as i128;
                self.new_i128(res.def0.unwrap(), val);
            }
            SextBv16(val) => {
                let val = self.i16(val) as i128;
                self.new_i128(res.def0.unwrap(), val);
            }
            SextBv32(val) => {
                let val = self.i32(val) as i128;
                self.new_i128(res.def0.unwrap(), val);
            }
            SextBv64(val) => {
                let val = self.i64(val) as i128;
                self.new_i128(res.def0.unwrap(), val);
            }
            ZextBv8(val) => {
                let val = self.u8(val) as u128;
                self.new_u128(res.def0.unwrap(), val);
            }
            ZextBv16(val) => {
                let val = self.u16(val) as u128;
                self.new_u128(res.def0.unwrap(), val);
            }
            ZextBv32(val) => {
                let val = self.u32(val) as u128;
                self.new_u128(res.def0.unwrap(), val);
            }
            ZextBv64(val) => {
                let val = self.u64(val) as u128;
                self.new_u128(res.def0.unwrap(), val);
            }
            _ => panic!("Most Int128Op's aren't supported yet"),
        }
    }

    fn float16_op(&mut self, _res: IRRes, _op: Float16Op) {
        todo!("Float16 support")
    }

    fn float32_op(&mut self, _res: IRRes, _op: Float32Op) {
        todo!("Float32 support")
    }

    fn float64_op(&mut self, _res: IRRes, _op: Float64Op) {
        todo!("Float64 support")
    }
}
