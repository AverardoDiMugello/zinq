pub mod one_source {}

pub mod two_source {
    // TODO: all these are wrong. Need to incorporate fp configuration
    use crate::core::model::ir::*;
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

    fn fmul16<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv16_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg16(ctx.arr16::<H>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let prod = block.mul_f16(operand1, operand2);
        let result = block.insert_bv16_in_elem0_bv128(result, prod);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fmul32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv32_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg32(ctx.arr32::<S>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let prod = block.mul_f32(operand1, operand2);
        let result = block.insert_bv32_in_elem0_bv128(result, prod);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fmul64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv64_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg64(ctx.arr64::<D>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let prod = block.mul_f64(operand1, operand2);
        let result = block.insert_bv64_in_elem0_bv128(result, prod);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fdiv16<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv16_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg16(ctx.arr16::<H>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let quot = block.div_f16(operand1, operand2);
        let result = block.insert_bv16_in_elem0_bv128(result, quot);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fdiv32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv32_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg32(ctx.arr32::<S>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let quot = block.div_f32(operand1, operand2);
        let result = block.insert_bv32_in_elem0_bv128(result, quot);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fdiv64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv64_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg64(ctx.arr64::<D>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let quot = block.div_f64(operand1, operand2);
        let result = block.insert_bv64_in_elem0_bv128(result, quot);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fadd16<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv16_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg16(ctx.arr16::<H>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let sum = block.add_f16(operand1, operand2);
        let result = block.insert_bv16_in_elem0_bv128(result, sum);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fadd32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv32_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg32(ctx.arr32::<S>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let sum = block.add_f32(operand1, operand2);
        let result = block.insert_bv32_in_elem0_bv128(result, sum);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fadd64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv64_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg64(ctx.arr64::<D>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let sum = block.add_f64(operand1, operand2);
        let result = block.insert_bv64_in_elem0_bv128(result, sum);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fsub16<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv16_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg16(ctx.arr16::<H>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let diff = block.sub_f16(operand1, operand2);
        let result = block.insert_bv16_in_elem0_bv128(result, diff);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fsub32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv32_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg32(ctx.arr32::<S>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let diff = block.sub_f32(operand1, operand2);
        let result = block.insert_bv32_in_elem0_bv128(result, diff);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fsub64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv64_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg64(ctx.arr64::<D>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let diff = block.sub_f64(operand1, operand2);
        let result = block.insert_bv64_in_elem0_bv128(result, diff);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fmax16<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 1 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv16_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg16(ctx.arr16::<H>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let max = block.max_f16(operand1, operand2);
        let result = block.insert_bv16_in_elem0_bv128(result, max);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fmax32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv32_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg32(ctx.arr32::<S>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let max = block.max_f32(operand1, operand2);
        let result = block.insert_bv32_in_elem0_bv128(result, max);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fmax64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv64_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg64(ctx.arr64::<D>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let max = block.max_f64(operand1, operand2);
        let result = block.insert_bv64_in_elem0_bv128(result, max);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fmin16<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 1 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv16_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg16(ctx.arr16::<H>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let min = block.min_f16(operand1, operand2);
        let result = block.insert_bv16_in_elem0_bv128(result, min);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fmin32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv32_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg32(ctx.arr32::<S>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let min = block.min_f32(operand1, operand2);
        let result = block.insert_bv32_in_elem0_bv128(result, min);
        block.write_reg128(regs.q(d), result);
        graph
    }

    fn fmin64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(rd, rn, rm, 8 => d, n, m, graph, block);

        let op1_full = block.read_reg128(regs.q(n));
        let operand1 = block.extract_bv64_from_elem0_bv128(op1_full);
        let operand2 = block.read_reg64(ctx.arr64::<D>(m));
        let merge = block.read_reg1(regs.fpcr_is_merging());
        let result = block.select_i128(merge, op1_full, Bv128::zero());
        let min = block.min_f64(operand1, operand2);
        let result = block.insert_bv64_in_elem0_bv128(result, min);
        block.write_reg128(regs.q(d), result);
        graph
    }
}
