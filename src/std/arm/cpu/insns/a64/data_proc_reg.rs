use crate::core::model::ir::*;
use crate::std::arm::cpu::insns::helpers::*;
use crate::std::arm::cpu::registers::*;
use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
use bitvec::prelude::*;

enum ShiftType {
    ASR,
    LSL,
    LSR,
    ROR,
}

impl ShiftType {
    fn decode(hi: bool, lo: bool) -> Self {
        match (hi, lo) {
            (false, false) => ShiftType::LSL,
            (false, true) => ShiftType::LSR,
            (true, false) => ShiftType::ASR,
            (true, true) => ShiftType::ROR,
        }
    }
}

// Note: adds 2 IROps to block
fn shift_reg_i32<'a>(
    reg: u32,
    shift_type: ShiftType,
    amount: Bv32,
    ctx: &ArmIRCtx,
    block: &mut IrBlock<ArmCpu>,
) -> Bv32 {
    let result = x32(reg, ctx, block);
    match shift_type {
        ShiftType::LSL => block.lsl_i32(result, amount),
        ShiftType::LSR => block.lsr_i32(result, amount),
        ShiftType::ASR => block.asr_i32(result, amount),
        ShiftType::ROR => block.ror_i32(result, amount),
    }
}

// Note: adds 2 IROps to block
fn shift_reg_i64<'a>(
    reg: u32,
    shift_type: ShiftType,
    amount: Bv64,
    ctx: &ArmIRCtx,
    block: &mut IrBlock<ArmCpu>,
) -> Bv64 {
    let result = x64(reg, ctx, block);
    match shift_type {
        ShiftType::LSL => block.lsl_i64(result, amount),
        ShiftType::LSR => block.lsr_i64(result, amount),
        ShiftType::ASR => block.asr_i64(result, amount),
        ShiftType::ROR => block.ror_i64(result, amount),
    }
}

pub mod two_source {
    use super::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};

    macro_rules! decode {
        ($rd:ident, $rn:ident, $rm:ident, $ops:literal => $d:ident, $n:ident, $m:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();
            let $m: u32 = $rm.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn udiv32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 7 => d, n, m, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let is_zero = block.eq_i32(operand2, Bv32::zero());
        let result = block.udiv_i32(operand1, operand2);
        let result = block.select_i32(is_zero, Bv32::zero(), result);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn sdiv32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 7 => d, n, m, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let is_zero = block.eq_i32(operand2, Bv32::zero());
        let result = block.sdiv_i32(operand1, operand2);
        let result = block.select_i32(is_zero, Bv32::zero(), result);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn lslv32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 6 => d, n, m, graph, block);

        let shift_type = ShiftType::LSL;
        let operand2 = x32(m, ctx, block);
        let shift_amount = block.urem_i32(operand2, Bv32::Lit(32));
        let result = shift_reg_i32(n, shift_type, shift_amount, ctx, block);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn lsrv32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 6 => d, n, m, graph, block);

        let shift_type = ShiftType::LSR;
        let operand2 = x32(m, ctx, block);
        let shift_amount = block.urem_i32(operand2, Bv32::Lit(32));
        let result = shift_reg_i32(n, shift_type, shift_amount, ctx, block);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn asrv32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 6 => d, n, m, graph, block);

        let shift_type = ShiftType::ASR;
        let operand2 = x32(m, ctx, block);
        let shift_amount = block.urem_i32(operand2, Bv32::Lit(32));
        let result = shift_reg_i32(n, shift_type, shift_amount, ctx, block);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn rorv32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 6 => d, n, m, graph, block);

        let shift_type = ShiftType::ROR;
        let operand2 = x32(m, ctx, block);
        let shift_amount = block.urem_i32(operand2, Bv32::Lit(32));
        let result = shift_reg_i32(n, shift_type, shift_amount, ctx, block);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    // TODO: crc here

    pub fn smax32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 5 => d, n, m, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let result = block.smax_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn umax32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 5 => d, n, m, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let result = block.umax_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn smin32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 5 => d, n, m, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let result = block.smin_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn umin32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 5 => d, n, m, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let result = block.umin_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn subp<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn udiv64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 6 => d, n, m, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let is_zero = block.eq_i64(operand2, Bv64::zero());
        let result = block.udiv_i64(operand1, operand2);
        let result = block.select_i64(is_zero, Bv64::zero(), result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn sdiv64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 6 => d, n, m, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let is_zero = block.eq_i64(operand2, Bv64::zero());
        let result = block.sdiv_i64(operand1, operand2);
        let result = block.select_i64(is_zero, Bv64::zero(), result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn irg<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn gmi<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn lslv64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 5 => d, n, m, graph, block);

        let shift_type = ShiftType::LSL;
        let operand2 = x64(m, ctx, block);
        let shift_amount = block.urem_i64(operand2, Bv64::Lit(64));
        let result = shift_reg_i64(n, shift_type, shift_amount, ctx, block);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn lsrv64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 5 => d, n, m, graph, block);

        let shift_type = ShiftType::LSR;
        let operand2 = x64(m, ctx, block);
        let shift_amount = block.urem_i64(operand2, Bv64::Lit(64));
        let result = shift_reg_i64(n, shift_type, shift_amount, ctx, block);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn asrv64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 5 => d, n, m, graph, block);

        let shift_type = ShiftType::ASR;
        let operand2 = x64(m, ctx, block);
        let shift_amount = block.urem_i64(operand2, Bv64::Lit(64));
        let result = shift_reg_i64(n, shift_type, shift_amount, ctx, block);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn rorv64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 5 => d, n, m, graph, block);

        let shift_type = ShiftType::ROR;
        let operand2 = x64(m, ctx, block);
        let shift_amount = block.urem_i64(operand2, Bv64::Lit(64));
        let result = shift_reg_i64(n, shift_type, shift_amount, ctx, block);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn pacga<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    // TODO: more crc

    pub fn smax64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 4 => d, n, m, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let result = block.smax_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn umax64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 4 => d, n, m, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let result = block.umax_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn smin64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 4 => d, n, m, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let result = block.smin_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn umin64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 4 => d, n, m, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let result = block.umin_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn subps<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }
}

pub mod one_source {
    use crate::core::model::ir::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($rd:ident, $rn:ident, $ops:literal => $d:ident, $n:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn rbit32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 4 => d, n, graph, block);

        let operand = x32(n, ctx, block);
        let result = block.bit_rev_i32(operand);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    // Rev16 - 32-bit
    pub fn rev1632<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 7 => d, n, graph, block);

        let operand = x32(n, ctx, block);
        let (hi16, lo16) = block.split_i32_to_i16(operand);
        let hi16 = block.byte_rev_i16(hi16);
        let lo16 = block.byte_rev_i16(lo16);
        let result = block.concat_i16_to_i32(hi16, lo16);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    // Rev - 32-bit
    pub fn rev32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 4 => d, n, graph, block);

        let operand = x32(n, ctx, block);
        let result = block.byte_rev_i32(operand);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn clz32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 4 => d, n, graph, block);

        let operand = x32(n, ctx, block);
        let result = block.ctlz_i32(operand);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn cls32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 4 => d, n, graph, block);

        let operand = x32(n, ctx, block);
        let result = block.ctls_i32(operand);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn ctz32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 4 => d, n, graph, block);

        let operand = x32(n, ctx, block);
        let result = block.cttz_i32(operand);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn cnt32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 4 => d, n, graph, block);

        let operand = x32(n, ctx, block);
        let result = block.ctset_i32(operand);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn abs32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 4 => d, n, graph, block);

        let operand = x32(n, ctx, block);
        let result = block.abs_i32(operand);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn rbit64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 3 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let result = block.bit_rev_i64(operand);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    // Rev16 - 64-bit
    pub fn rev1664<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 12 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let (hi32, lo32) = block.split_i64_to_i32(operand);

        let (hi32_hi16, hi32_lo16) = block.split_i32_to_i16(hi32);
        let new_hi32_hi16 = block.byte_rev_i16(hi32_hi16);
        let new_hi32_lo16 = block.byte_rev_i16(hi32_lo16);
        let new_hi32 = block.concat_i16_to_i32(new_hi32_hi16, new_hi32_lo16);

        let (hi16, lo16) = block.split_i32_to_i16(lo32);
        let new_lo32_hi16 = block.byte_rev_i16(hi16);
        let new_lo32_lo16 = block.byte_rev_i16(lo16);
        let new_lo32 = block.concat_i16_to_i32(new_lo32_hi16, new_lo32_lo16);

        let result = block.concat_i32_to_i64(new_hi32, new_lo32);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    // Rev32
    pub fn rev3264<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 6 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let (hi32, lo32) = block.split_i64_to_i32(operand);
        let hi32 = block.byte_rev_i32(hi32);
        let lo32 = block.byte_rev_i32(lo32);
        let result = block.concat_i32_to_i64(hi32, lo32);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    // Rev - 64-bit
    pub fn rev64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 3 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let result = block.byte_rev_i64(operand);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn clz64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 3 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let result = block.ctlz_i64(operand);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn cls64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 3 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let result = block.ctls_i64(operand);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn ctz64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 3 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let result = block.cttz_i64(operand);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn cnt64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 3 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let result = block.ctset_i64(operand);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn abs64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, 3 => d, n, graph, block);

        let operand = x64(n, ctx, block);
        let result = block.abs_i64(operand);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }
}

pub mod logical {
    use super::*;

    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};

    macro_rules! decode {
        ($rd:ident, $rn:ident, $rm:ident, $shift:ident, $imm6:ident, $ops:literal => $d:ident, $n:ident, $m:ident, $shift_type:ident, $shift_amount:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();
            let $m: u32 = $rm.load();
            let $shift_type = ShiftType::decode($shift[1], $shift[0]);
            let $shift_amount = $imm6.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn and32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 6 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);

        let result = block.and_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn bic32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 7 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let shifted = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);
        let operand2 = block.not_i32(shifted);

        let result = block.and_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn orr32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 6 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);

        let result = block.or_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn orn32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 7 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let shifted = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);
        let operand2 = block.not_i32(shifted);

        let result = block.or_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn eor32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 6 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);

        let result = block.xor_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn eon32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 7 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let shifted = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);
        let operand2 = block.not_i32(shifted);

        let result = block.xor_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn ands32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 12 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);

        let result = block.and_i32(operand1, operand2);

        let n = block.slt_i32(result, Bv32::zero());
        let z = block.eq_i32(result, Bv32::zero());
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), Bit::zero());
        block.write_reg1(ctx.reg1::<pstate::V>(), Bit::zero());

        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn bics32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 13 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let shifted = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);
        let operand2 = block.not_i32(shifted);

        let result = block.and_i32(operand1, operand2);

        let n = block.slt_i32(result, Bv32::zero());
        let z = block.eq_i32(result, Bv32::zero());
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), Bit::zero());
        block.write_reg1(ctx.reg1::<pstate::V>(), Bit::zero());

        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn and64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 5 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);

        let result = block.and_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn bic64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 6 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let shifted = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);
        let operand2 = block.not_i64(shifted);

        let result = block.and_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn orr64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 5 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);

        let result = block.or_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn orn64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 6 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let shifted = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);
        let operand2 = block.not_i64(shifted);

        let result = block.or_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn eor64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 5 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);

        let result = block.xor_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn eon64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 6 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let shifted = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);
        let operand2 = block.not_i64(shifted);

        let result = block.xor_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn ands64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 11 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);

        let result = block.and_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        let n = block.slt_i64(result, Bv64::zero());
        let z = block.eq_i64(result, Bv64::zero());
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), Bit::Lit(false));
        block.write_reg1(ctx.reg1::<pstate::V>(), Bit::Lit(false));

        inc_pc(ctx, block);
        graph
    }

    pub fn bics64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 12 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let shifted = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);
        let operand2 = block.not_i64(shifted);

        let result = block.and_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        let n = block.slt_i64(result, Bv64::zero());
        let z = block.eq_i64(result, Bv64::zero());
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), Bit::zero());
        block.write_reg1(ctx.reg1::<pstate::V>(), Bit::zero());

        inc_pc(ctx, block);
        graph
    }
}

pub mod add_sub_shifted {
    use super::*;

    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};

    macro_rules! decode {
        ($rd:ident, $rn:ident, $rm:ident, $shift:ident, $imm6:ident, $ops:literal => $d:ident, $n:ident, $m:ident, $shift_type:ident, $shift_amount:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();
            let $m: u32 = $rm.load();
            let $shift_type = ShiftType::decode($shift[1], $shift[0]);
            let $shift_amount = $imm6.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn add32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 6 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);
        let (result, _) = block.uadd_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn adds32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 13 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);
        let (result, overflow) = block.uadd_i32(operand1, operand2);

        let n = block.slt_i32(result, Bv32::zero());
        let z = block.eq_i32(result, Bv32::zero());
        let c = overflow;
        let (_, v) = block.sadd_i32(operand1, operand2);
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn sub32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 6 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);
        let (result, _) = block.usub_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn subs32<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 13 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = shift_reg_i32(m, shift_type, Bv32::Lit(shift_amount), ctx, block);
        let (result, overflow) = block.usub_i32(operand1, operand2);

        let n = block.slt_i32(result, Bv32::zero());
        let z = block.eq_i32(result, Bv32::zero());
        let c = overflow;
        let (_, v) = block.ssub_i32(operand1, operand2);
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn add64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 5 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);
        let (result, _) = block.uadd_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn adds64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 12 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);
        let (result, overflow) = block.uadd_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        let n = block.slt_i64(result, Bv64::zero());
        let z = block.eq_i64(result, Bv64::zero());
        let c = overflow;
        let (_, v) = block.sadd_i64(operand1, operand2);
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);
        inc_pc(ctx, block);
        graph
    }

    pub fn sub64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 5 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);
        let (result, _) = block.usub_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn subs64<'a>(
        ctx: &ArmIRCtx,
        shift: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm6: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, shift, imm6, 12 => d, n, m, shift_type, shift_amount, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = shift_reg_i64(m, shift_type, Bv64::Lit(shift_amount), ctx, block);
        let (result, overflow) = block.usub_i64(operand1, operand2);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        let n = block.slt_i64(result, Bv64::zero());
        let z = block.eq_i64(result, Bv64::zero());
        let c = overflow;
        let (_, v) = block.ssub_i64(operand1, operand2);
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);
        inc_pc(ctx, block);
        graph
    }
}

pub mod add_sub_extended {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::pseudoc::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($rd:ident, $rn:ident, $rm:ident, $imm3:ident, $option:ident, $ops:literal => $d:ident, $n:ident, $m:ident, $shift:ident, $extend_type:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();
            let $m: u32 = $rm.load();
            let $shift = $imm3.load();
            let $extend_type = ExtendType::decode($option[2], $option[1], $option[0]);

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn add32<'a>(
        ctx: &ArmIRCtx,
        option: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm3: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, imm3, option, 11 => d, n, m, shift, extend_type, graph, block);

        let operand1 = if n == 31 {
            wsp(el, pstate_sp, ctx, block)
        } else {
            x32(n, ctx, block)
        };
        let operand2 = extend_reg_i32(m, extend_type, shift, ctx, block);
        let (result, _) = block.uadd_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d == 31 {
            block.write_reg64(
                if !pstate_sp {
                    ctx.sp(EL::EL0)
                } else {
                    ctx.sp(el)
                },
                result,
            );
        } else {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn adds32<'a>(
        ctx: &ArmIRCtx,
        option: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm3: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, imm3, option, 18 => d, n, m, shift, extend_type, graph, block);

        let operand1 = if n == 31 {
            wsp(el, pstate_sp, ctx, block)
        } else {
            x32(n, ctx, block)
        };
        let operand2 = extend_reg_i32(m, extend_type, shift, ctx, block);
        let (result, new_c) = block.uadd_i32(operand1, operand2);
        let (_, new_v) = block.sadd_i32(operand1, operand2);

        let n = block.slt_i32(result, Bv32::zero());
        let z = block.eq_i32(result, Bv32::zero());
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), new_c);
        block.write_reg1(ctx.reg1::<pstate::V>(), new_v);

        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn sub32<'a>(
        ctx: &ArmIRCtx,
        option: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm3: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, imm3, option, 11 => d, n, m, shift, extend_type, graph, block);

        let operand1 = if n == 31 {
            wsp(el, pstate_sp, ctx, block)
        } else {
            x32(n, ctx, block)
        };
        let operand2 = extend_reg_i32(m, extend_type, shift, ctx, block);
        let (result, _) = block.usub_i32(operand1, operand2);
        let result = block.zext_i32_to_i64(result);
        if d == 31 {
            block.write_reg64(
                if !pstate_sp {
                    ctx.sp(EL::EL0)
                } else {
                    ctx.sp(el)
                },
                result,
            );
        } else {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn subs32<'a>(
        ctx: &ArmIRCtx,
        option: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm3: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, imm3, option, 18 => d, n, m, shift, extend_type, graph, block);

        let operand1 = if n == 31 {
            wsp(el, pstate_sp, ctx, block)
        } else {
            x32(n, ctx, block)
        };
        let operand2 = extend_reg_i32(m, extend_type, shift, ctx, block);
        let (result, new_c) = block.usub_i32(operand1, operand2);
        let (_, new_v) = block.ssub_i32(operand1, operand2);

        let n = block.slt_i32(result, Bv32::zero());
        let z = block.eq_i32(result, Bv32::zero());
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), new_c);
        block.write_reg1(ctx.reg1::<pstate::V>(), new_v);

        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn add64<'a>(
        ctx: &ArmIRCtx,
        option: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm3: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, imm3, option, 10 => d, n, m, shift, extend_type, graph, block);

        let operand1 = if n == 31 {
            sp(el, pstate_sp, ctx, block)
        } else {
            x64(n, ctx, block)
        };
        let operand2 = extend_reg_i64(m, extend_type, shift, ctx, block);
        let (result, _) = block.uadd_i64(operand1, operand2);
        if d == 31 {
            block.write_reg64(
                if !pstate_sp {
                    ctx.sp(EL::EL0)
                } else {
                    ctx.sp(el)
                },
                result,
            );
        } else {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn adds64<'a>(
        ctx: &ArmIRCtx,
        option: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm3: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, imm3, option, 17 => d, n, m, shift, extend_type, graph, block);

        let operand1 = if n == 31 {
            sp(el, pstate_sp, ctx, block)
        } else {
            x64(n, ctx, block)
        };
        let operand2 = extend_reg_i64(m, extend_type, shift, ctx, block);
        let (result, new_c) = block.uadd_i64(operand1, operand2);
        let (_, new_v) = block.sadd_i64(operand1, operand2);

        let n = block.slt_i64(result, Bv64::zero());
        let z = block.eq_i64(result, Bv64::zero());
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), new_c);
        block.write_reg1(ctx.reg1::<pstate::V>(), new_v);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn sub64<'a>(
        ctx: &ArmIRCtx,
        option: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm3: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, imm3, option, 10 => d, n, m, shift, extend_type, graph, block);

        let operand1 = if n == 31 {
            sp(el, pstate_sp, ctx, block)
        } else {
            x64(n, ctx, block)
        };
        let operand2 = extend_reg_i64(m, extend_type, shift, ctx, block);
        let (result, _) = block.usub_i64(operand1, operand2);
        if d == 31 {
            block.write_reg64(
                if !pstate_sp {
                    ctx.sp(EL::EL0)
                } else {
                    ctx.sp(el)
                },
                result,
            );
        } else {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn subs64<'a>(
        ctx: &ArmIRCtx,
        option: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        imm3: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, imm3, option, 17 => d, n, m, shift, extend_type, graph, block);

        let operand1 = if n == 31 {
            sp(el, pstate_sp, ctx, block)
        } else {
            x64(n, ctx, block)
        };
        let operand2 = extend_reg_i64(m, extend_type, shift, ctx, block);
        let (result, new_c) = block.usub_i64(operand1, operand2);
        let (_, new_v) = block.ssub_i64(operand1, operand2);

        let n = block.slt_i64(result, Bv64::zero());
        let z = block.eq_i64(result, Bv64::zero());
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), new_c);
        block.write_reg1(ctx.reg1::<pstate::V>(), new_v);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }
}

pub mod add_sub_carry {
    use crate::core::model::ir::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($rd:ident, $rn:ident, $rm:ident, $ops:literal => $d:ident, $n:ident, $m:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();
            let $m: u32 = $rm.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn adc32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // decode!(rd, rn, rm, 7 => d, n, m, graph, block);
        panic!("wrong");
        // let operand1 = x32(n, ctx, block);
        // let operand2 = x32(m, ctx, block);
        // let (result, _) = block.uadd_i32(operand1, operand2);
        // let c = block.read_reg1(ctx.reg1::<pstate::C>());
        // let c = block.b_to_i32(c);
        // let (result, _) = block.uadd_i32(result, c);
        // block.write_reg32(regs.w(d), result);

        // inc_pc(ctx, block);
        // graph
    }

    pub fn adcs32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // decode!(rd, rn, rm, 15 => d, n, m, graph, block);
        panic!("wrong");
        // let operand1 = x32(n, ctx, block);
        // let operand2 = x32(m, ctx, block);
        // let c = block.read_reg1(ctx.reg1::<pstate::C>());
        // let c = block.b_to_i32(c);

        // let (result, _) = block.uadd_i32(operand1, operand2);
        // let (result, new_c) = block.uadd_i32(result, c);
        // block.write_reg32(regs.w(d), result);

        // let n = block.slt_i32(result, Bv32::zero());
        // let z = block.eq_i32(result, Bv32::zero());

        // let (result, _) = block.sadd_i32(operand1, operand2);
        // let (_, new_v) = block.uadd_i32(result, c);

        // block.write_reg1(ctx.reg1::<pstate::N>(), n);
        // block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        // block.write_reg1(ctx.reg1::<pstate::C>(), new_c);
        // block.write_reg1(ctx.reg1::<pstate::V>(), new_v);

        // inc_pc(ctx, block);
        // graph
    }

    pub fn sbc32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // decode!(rd, rn, rm, 8 => d, n, m, graph, block);
        panic!("wrong");
        // let operand1 = x32(n, ctx, block);
        // let operand2 = x32(m, ctx, block);
        // let operand2 = block.not_i32(operand2);

        // let (result, _) = block.uadd_i32(operand1, operand2);
        // let c = block.read_reg1(ctx.reg1::<pstate::C>());
        // let c = block.b_to_i32(c);
        // let (result, _) = block.uadd_i32(result, c);
        // block.write_reg32(regs.w(d), result);

        // inc_pc(ctx, block);
        // graph
    }

    pub fn sbcs32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // decode!(rd, rn, rm, 16 => d, n, m, graph, block);
        panic!("wrong");
        // let operand1 = x32(n, ctx, block);
        // let operand2 = x32(m, ctx, block);
        // let operand2 = block.not_i32(operand2);
        // let c = block.read_reg1(ctx.reg1::<pstate::C>());
        // let c = block.b_to_i32(c);

        // let (result, _) = block.uadd_i32(operand1, operand2);
        // let (result, new_c) = block.uadd_i32(result, c);
        // block.write_reg32(regs.w(d), result);

        // let n = block.slt_i32(result, Bv32::zero());
        // let z = block.eq_i32(result, Bv32::zero());

        // let (result, _) = block.sadd_i32(operand1, operand2);
        // let (_, new_v) = block.uadd_i32(result, c);

        // block.write_reg1(ctx.reg1::<pstate::N>(), n);
        // block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        // block.write_reg1(ctx.reg1::<pstate::C>(), new_c);
        // block.write_reg1(ctx.reg1::<pstate::V>(), new_v);

        // inc_pc(ctx, block);
        // graph
    }

    pub fn adc64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // decode!(rd, rn, rm, 7 => d, n, m, graph, block);
        panic!("wrong");
        // let operand1 = x64(n, ctx, block);
        // let operand2 = x64(m, ctx, block);
        // let (result, _) = block.uadd_i64(operand1, operand2);
        // let c = block.read_reg1(ctx.reg1::<pstate::C>());
        // let c = block.b_to_i64(c);
        // let (result, _) = block.uadd_i64(result, c);
        // if d != 31 {
        //     block.write_reg64(ctx.arr64::<X>(d), result);
        // }

        // inc_pc(ctx, block);
        // graph
    }

    pub fn adcs64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // decode!(rd, rn, rm, 15 => d, n, m, graph, block);
        panic!("wrong");
        // let operand1 = x64(n, ctx, block);
        // let operand2 = x64(m, ctx, block);
        // let c = block.read_reg1(ctx.reg1::<pstate::C>());
        // let c = block.b_to_i64(c);

        // let (result, _) = block.uadd_i64(operand1, operand2);
        // let (result, new_c) = block.uadd_i64(result, c);
        // if d != 31 {
        //     block.write_reg64(ctx.arr64::<X>(d), result);
        // }

        // let n = block.slt_i64(result, Bv64::zero());
        // let z = block.eq_i64(result, Bv64::zero());

        // let (result, _) = block.sadd_i64(operand1, operand2);
        // let (_, new_v) = block.uadd_i64(result, c);

        // block.write_reg1(ctx.reg1::<pstate::N>(), n);
        // block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        // block.write_reg1(ctx.reg1::<pstate::C>(), new_c);
        // block.write_reg1(ctx.reg1::<pstate::V>(), new_v);

        // inc_pc(ctx, block);
        // graph
    }

    pub fn sbc64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // decode!(rd, rn, rm, 8 => d, n, m, graph, block);
        panic!("wrong");
        // let operand1 = x64(n, ctx, block);
        // let operand2 = x64(m, ctx, block);
        // let operand2 = block.not_i64(operand2);

        // let (result, _) = block.uadd_i64(operand1, operand2);
        // let c = block.read_reg1(ctx.reg1::<pstate::C>());
        // let c = block.b_to_i64(c);
        // let (result, _) = block.uadd_i64(result, c);
        // if d != 31 {
        //     block.write_reg64(ctx.arr64::<X>(d), result);
        // }

        // inc_pc(ctx, block);
        // graph
    }

    pub fn sbcs64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // decode!(rd, rn, rm, 16 => d, n, m, graph, block);
        panic!("wrong");
        // let operand1 = x64(n, ctx, block);
        // let operand2 = x64(m, ctx, block);
        // let operand2 = block.not_i64(operand2);
        // let c = block.read_reg1(ctx.reg1::<pstate::C>());
        // let c = block.b_to_i64(c);

        // let (result, _) = block.uadd_i64(operand1, operand2);
        // let (result, new_c) = block.uadd_i64(result, c);
        // if d != 31 {
        //     block.write_reg64(ctx.arr64::<X>(d), result);
        // }

        // let n = block.slt_i64(result, Bv64::zero());
        // let z = block.eq_i64(result, Bv64::zero());

        // let (result, _) = block.sadd_i64(operand1, operand2);
        // let (_, new_v) = block.uadd_i64(result, c);

        // block.write_reg1(ctx.reg1::<pstate::N>(), n);
        // block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        // block.write_reg1(ctx.reg1::<pstate::C>(), new_c);
        // block.write_reg1(ctx.reg1::<pstate::V>(), new_v);

        // inc_pc(ctx, block);
        // graph
    }
}

pub mod add_sub_checkedptr {
    use crate::core::model::ir::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn addpt<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _imm3: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn subpt<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _imm3: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }
}

pub mod rotate_right_into_flags {
    use crate::core::model::ir::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn rmif<'a>(
        _ctx: &ArmIRCtx,
        _mask: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _imm6: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }
}

pub mod evaluate_into_flags {}

// Note: these conditional cmp instructions are very slow in the way they are implemented
// As of now, full fledged cfg isn't well supported in the IRGraph. They can probably be
// a lot faster if they just use that.

pub mod cond_cmp_reg {
    use crate::core::model::ir::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn ccmn32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        nzcv: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let n = rn.load();
        let m = rm.load();
        let mut graph = IRGraph::with_capacity(1, 0, 21);
        let block = graph.block_mut(graph.root());

        let cond_holds = condition_holds(cond, ctx, block);
        // if cond_holds
        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let (result, t_c) = block.uadd_i32(operand1, operand2);
        let (_, t_v) = block.sadd_i32(operand1, operand2);
        let t_n = block.slt_i32(result, Bv32::zero());
        let t_z = block.eq_i32(result, Bv32::zero());

        let n = block.select_b(cond_holds, t_n, Bit::Lit(nzcv[3]));
        let z = block.select_b(cond_holds, t_z, Bit::Lit(nzcv[2]));
        let c = block.select_b(cond_holds, t_c, Bit::Lit(nzcv[1]));
        let v = block.select_b(cond_holds, t_v, Bit::Lit(nzcv[0]));

        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        inc_pc(ctx, block);
        graph
    }

    pub fn ccmp32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        nzcv: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let n = rn.load();
        let m = rm.load();
        let mut graph = IRGraph::with_capacity(1, 0, 21);
        let block = graph.block_mut(graph.root());

        let cond_holds = condition_holds(cond, ctx, block);
        // if cond_holds
        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let (result, overflow) = block.usub_i32(operand1, operand2);

        let t_n = block.slt_i32(result, Bv32::zero());
        let t_z = block.eq_i32(result, Bv32::zero());
        let t_c = overflow;
        let (_, t_v) = block.ssub_i32(operand1, operand2);

        let n = block.select_b(cond_holds, t_n, Bit::Lit(nzcv[3]));
        let z = block.select_b(cond_holds, t_z, Bit::Lit(nzcv[2]));
        let c = block.select_b(cond_holds, t_c, Bit::Lit(nzcv[1]));
        let v = block.select_b(cond_holds, t_v, Bit::Lit(nzcv[0]));

        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        inc_pc(ctx, block);
        graph
    }

    pub fn ccmn64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        nzcv: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let n = rn.load();
        let m = rm.load();
        let mut graph = IRGraph::with_capacity(1, 0, 21);
        let block = graph.block_mut(graph.root());

        let cond_holds = condition_holds(cond, ctx, block);
        // if cond_holds
        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let (result, overflow) = block.uadd_i64(operand1, operand2);

        let t_n = block.slt_i64(result, Bv64::zero());
        let t_z = block.eq_i64(result, Bv64::zero());
        let t_c = overflow;
        let (_, t_v) = block.sadd_i64(operand1, operand2);

        let n = block.select_b(cond_holds, t_n, Bit::Lit(nzcv[3]));
        let z = block.select_b(cond_holds, t_z, Bit::Lit(nzcv[2]));
        let c = block.select_b(cond_holds, t_c, Bit::Lit(nzcv[1]));
        let v = block.select_b(cond_holds, t_v, Bit::Lit(nzcv[0]));

        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        inc_pc(ctx, block);
        graph
    }

    pub fn ccmp64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        nzcv: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let n = rn.load();
        let m = rm.load();
        let mut graph = IRGraph::with_capacity(1, 0, 21);
        let block = graph.block_mut(graph.root());

        let cond_holds = condition_holds(cond, ctx, block);
        // if cond_holds
        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let (result, overflow) = block.usub_i64(operand1, operand2);

        let t_n = block.slt_i64(result, Bv64::zero());
        let t_z = block.eq_i64(result, Bv64::zero());
        let t_c = overflow;
        let (_, t_v) = block.ssub_i64(operand1, operand2);

        let n = block.select_b(cond_holds, t_n, Bit::Lit(nzcv[3]));
        let z = block.select_b(cond_holds, t_z, Bit::Lit(nzcv[2]));
        let c = block.select_b(cond_holds, t_c, Bit::Lit(nzcv[1]));
        let v = block.select_b(cond_holds, t_v, Bit::Lit(nzcv[0]));

        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        inc_pc(ctx, block);
        graph
    }
}

pub mod cond_cmp_imm {
    use crate::core::model::ir::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn ccmn32<'a>(
        ctx: &ArmIRCtx,
        imm5: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        nzcv: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let n = rn.load();
        let imm = imm5.load();
        let mut graph = IRGraph::with_capacity(1, 0, 20);
        let block = graph.block_mut(graph.root());

        let cond_holds = condition_holds(cond, ctx, block);
        // if cond_holds
        let operand1 = x32(n, ctx, block);
        let operand2 = Bv32::Lit(imm);
        let (result, t_c) = block.uadd_i32(operand1, operand2);
        let (_, t_v) = block.sadd_i32(operand1, operand2);
        let t_n = block.slt_i32(result, Bv32::zero());
        let t_z = block.eq_i32(result, Bv32::zero());

        let n = block.select_b(cond_holds, t_n, Bit::Lit(nzcv[3]));
        let z = block.select_b(cond_holds, t_z, Bit::Lit(nzcv[2]));
        let c = block.select_b(cond_holds, t_c, Bit::Lit(nzcv[1]));
        let v = block.select_b(cond_holds, t_v, Bit::Lit(nzcv[0]));

        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        inc_pc(ctx, block);
        graph
    }

    pub fn ccmp32<'a>(
        ctx: &ArmIRCtx,
        imm5: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        nzcv: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let n = rn.load();
        let imm = imm5.load();
        let mut graph = IRGraph::with_capacity(1, 0, 20);
        let block = graph.block_mut(graph.root());

        let cond_holds = condition_holds(cond, ctx, block);
        // if cond_holds
        let operand1 = x32(n, ctx, block);
        let operand2 = Bv32::Lit(imm);
        let (result, overflow) = block.usub_i32(operand1, operand2);

        let t_n = block.slt_i32(result, Bv32::zero());
        let t_z = block.eq_i32(result, Bv32::zero());
        let t_c = overflow;
        let (_, t_v) = block.ssub_i32(operand1, operand2);

        let n = block.select_b(cond_holds, t_n, Bit::Lit(nzcv[3]));
        let z = block.select_b(cond_holds, t_z, Bit::Lit(nzcv[2]));
        let c = block.select_b(cond_holds, t_c, Bit::Lit(nzcv[1]));
        let v = block.select_b(cond_holds, t_v, Bit::Lit(nzcv[0]));

        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        inc_pc(ctx, block);
        graph
    }

    pub fn ccmn64<'a>(
        ctx: &ArmIRCtx,
        imm5: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        nzcv: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let n = rn.load();
        let imm = imm5.load();
        let mut graph = IRGraph::with_capacity(1, 0, 20);
        let block = graph.block_mut(graph.root());

        let cond_holds = condition_holds(cond, ctx, block);
        // if cond_holds
        let operand1 = x64(n, ctx, block);
        let operand2 = Bv64::Lit(imm);
        let (result, overflow) = block.uadd_i64(operand1, operand2);

        let t_n = block.slt_i64(result, Bv64::zero());
        let t_z = block.eq_i64(result, Bv64::zero());
        let t_c = overflow;
        let (_, t_v) = block.sadd_i64(operand1, operand2);

        let n = block.select_b(cond_holds, t_n, Bit::Lit(nzcv[3]));
        let z = block.select_b(cond_holds, t_z, Bit::Lit(nzcv[2]));
        let c = block.select_b(cond_holds, t_c, Bit::Lit(nzcv[1]));
        let v = block.select_b(cond_holds, t_v, Bit::Lit(nzcv[0]));

        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        inc_pc(ctx, block);
        graph
    }

    pub fn ccmp64<'a>(
        ctx: &ArmIRCtx,
        imm5: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        nzcv: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let n = rn.load();
        let imm = imm5.load();
        let mut graph = IRGraph::with_capacity(1, 0, 20);
        let block = graph.block_mut(graph.root());

        let cond_holds = condition_holds(cond, ctx, block);
        // if cond_holds
        let operand1 = x64(n, ctx, block);
        let operand2 = Bv64::Lit(imm);
        let (result, overflow) = block.usub_i64(operand1, operand2);
        let t_n = block.slt_i64(result, Bv64::zero());
        let t_z = block.eq_i64(result, Bv64::zero());
        let t_c = overflow;
        let (_, t_v) = block.ssub_i64(operand1, operand2);

        let n = block.select_b(cond_holds, t_n, Bit::Lit(nzcv[3]));
        let z = block.select_b(cond_holds, t_z, Bit::Lit(nzcv[2]));
        let c = block.select_b(cond_holds, t_c, Bit::Lit(nzcv[1]));
        let v = block.select_b(cond_holds, t_v, Bit::Lit(nzcv[0]));

        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        inc_pc(ctx, block);
        graph
    }
}

pub mod cond_select {
    use crate::core::model::ir::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($rd:ident, $rn:ident, $rm:ident, $ops:literal => $d:ident, $n:ident, $m:ident, $graph:ident, $block:ident) => {
            let $d = $rd.load();
            let $n = $rn.load();
            let $m = $rm.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn csel32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 12 => d, n, m, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let t_case = x32(n, ctx, block);
        let f_case = x32(m, ctx, block);
        let result = block.select_i32(cond_holds, t_case, f_case);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn csinc32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 13 => d, n, m, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let t_case = x32(n, ctx, block);
        let f_case = x32(m, ctx, block);
        let (f_case, _) = block.uadd_i32(f_case, Bv32::one());
        let result = block.select_i32(cond_holds, t_case, f_case);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn csinv32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 13 => d, n, m, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let t_case = x32(n, ctx, block);
        let f_case = x32(m, ctx, block);
        let f_case = block.not_i32(f_case);
        let result = block.select_i32(cond_holds, t_case, f_case);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn csneg32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 14 => d, n, m, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let t_case = x32(n, ctx, block);
        let f_case = x32(m, ctx, block);
        let f_case = block.not_i32(f_case);
        let (f_case, _) = block.uadd_i32(f_case, Bv32::one());
        let result = block.select_i32(cond_holds, t_case, f_case);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn csel64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 11 => d, n, m, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let t_case = x64(n, ctx, block);
        let f_case = x64(m, ctx, block);
        let result = block.select_i64(cond_holds, t_case, f_case);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn csinc64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 12 => d, n, m, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let t_case = x64(n, ctx, block);
        let f_case = x64(m, ctx, block);
        let (f_case, _) = block.uadd_i64(f_case, Bv64::one());
        let result = block.select_i64(cond_holds, t_case, f_case);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn csinv64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 12 => d, n, m, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let t_case = x64(n, ctx, block);
        let f_case = x64(m, ctx, block);
        let f_case = block.not_i64(f_case);
        let result = block.select_i64(cond_holds, t_case, f_case);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn csneg64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        cond: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 13 => d, n, m, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let t_case = x64(n, ctx, block);
        let f_case = x64(m, ctx, block);
        let f_case = block.not_i64(f_case);
        let (f_case, _) = block.uadd_i64(f_case, Bv64::one());
        let result = block.select_i64(cond_holds, t_case, f_case);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }
}

pub mod three_source {
    use crate::core::model::ir::*;

    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($rd:ident, $rn:ident, $rm:ident, $ra:ident, $ops:literal => $d:ident, $n:ident, $m:ident, $a:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();
            let $m: u32 = $rm.load();
            let $a: u32 = $ra.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn madd32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 7 => d, n, m, a, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let operand3 = x32(a, ctx, block);

        let (product, _) = block.umul_i32(operand1, operand2);
        let (result, _) = block.uadd_i32(product, operand3);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn msub32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 7 => d, n, m, a, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let operand3 = x32(a, ctx, block);

        let (product, _) = block.umul_i32(operand1, operand2);
        let (result, _) = block.usub_i32(operand3, product);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn madd64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 6 => d, n, m, a, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let operand3 = x64(a, ctx, block);

        let (product, _) = block.umul_i64(operand1, operand2);
        let (result, _) = block.uadd_i64(product, operand3);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn msub64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 6 => d, n, m, a, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let operand3 = x64(a, ctx, block);

        let (product, _) = block.umul_i64(operand1, operand2);
        let (result, _) = block.usub_i64(operand3, product);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn smaddl<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 8 => d, n, m, a, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand1 = block.sext_i32_to_i64(operand1);
        let operand2 = x32(m, ctx, block);
        let operand2 = block.sext_i32_to_i64(operand2);
        let operand3 = x64(a, ctx, block);

        let (product, _) = block.smul_i64(operand1, operand2);
        let (result, _) = block.sadd_i64(operand3, product);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn smsubl<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 8 => d, n, m, a, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand1 = block.sext_i32_to_i64(operand1);
        let operand2 = x32(m, ctx, block);
        let operand2 = block.sext_i32_to_i64(operand2);
        let operand3 = x64(a, ctx, block);

        let (product, _) = block.smul_i64(operand1, operand2);
        let (result, _) = block.ssub_i64(operand3, product);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn smulh<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 8 => d, n, m, _a, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);

        let operand1 = block.sext_i64_to_i128(operand1);
        let operand2 = block.sext_i64_to_i128(operand2);

        let (result, _) = block.smul_i128(operand1, operand2);
        let result = block.lsr_i128(result, Bv128::Lit(64));
        let result = block.trunc_i128_to_i64(result);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn maddpt<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _ra: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn msubpt<'a>(
        _ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _ra: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn umaddl<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 8 => d, n, m, a, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand1 = block.zext_i32_to_i64(operand1);
        let operand2 = x32(m, ctx, block);
        let operand2 = block.zext_i32_to_i64(operand2);
        let operand3 = x64(a, ctx, block);

        let (product, _) = block.umul_i64(operand1, operand2);
        let (result, _) = block.uadd_i64(operand3, product);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn umsubl<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 8 => d, n, m, a, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand1 = block.zext_i32_to_i64(operand1);
        let operand2 = x32(m, ctx, block);
        let operand2 = block.zext_i32_to_i64(operand2);
        let operand3 = x64(a, ctx, block);

        let (product, _) = block.umul_i64(operand1, operand2);
        let (result, _) = block.usub_i64(operand3, product);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn umulh<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        ra: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, ra, 8 => d, n, m, _a, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);

        let operand1 = block.zext_i64_to_i128(operand1);
        let operand2 = block.zext_i64_to_i128(operand2);

        let (result, _) = block.umul_i128(operand1, operand2);
        let result = block.lsr_i128(result, Bv128::Lit(64));
        let result = block.trunc_i128_to_i64(result);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }
}
