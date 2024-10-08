use crate::core::model::ir::*;
use crate::std::arm::cpu::registers::*;
use bitvec::prelude::*;

/// Helper function for decoding bitfield and logical instruction operands.
/// Consult ARM docs for more info
fn decode_bit_masks(
    n: bool,
    imms: BitVec<u32>,
    immr: BitVec<u32>,
    immediate: bool,
    m: usize,
) -> (BitVec<u32>, BitVec<u32>) {
    assert!(m == 32 || m == 64);
    // constant integer len = HighestSetBit(immN:NOT(imms));
    let len = if n {
        imms.len()
    } else {
        imms.last_zero().unwrap_or(0)
    };
    assert_ne!(len, 0);
    assert!(m >= (1 << len));

    let mut levels: BitVec<usize, Lsb0> = BitVec::with_capacity(6);
    levels.resize(len, true);
    levels.resize(6, false);

    let imms: usize = imms.load();
    let immr: usize = immr.load();
    let levels: usize = levels.load();
    assert!(!(immediate && ((imms & levels) == levels)));

    let s = imms & levels;
    let r = immr & levels;
    let diff = (s.overflowing_sub(r).0) & 0b111111; // 6-bit subtract

    let esize = 1 << len;
    let d = diff
        .view_bits::<Lsb0>()
        .get(0..len)
        .expect("Decoder guarantees valid instructions")
        .load::<usize>();
    let mut welem: BitVec<usize, Lsb0> = BitVec::with_capacity(esize);
    welem.resize(s + 1, true);
    welem.resize(esize, false);
    let mut telem: BitVec<usize, Lsb0> = BitVec::with_capacity(esize);
    telem.resize(d + 1, true);
    telem.resize(esize, false);
    let mut wmask = BitVec::with_capacity(m);
    // Not 100% sure why this is a ROR left instead of ROR right like in the ARM docs,
    // but I think it has something to do with BitVec's Lsb0 vs Msb0.
    // If someone thinks this through and wants to right a comment explaining or re-write
    // the code to closer match the ARM pseudocode, please do!
    welem.rotate_left(r);
    wmask.resize_with(m, |idx| welem[idx % welem.len()]);
    let mut tmask = BitVec::with_capacity(m);
    tmask.resize_with(m, |idx| telem[idx % telem.len()]);

    (wmask, tmask)
}

pub mod one_source {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};

    pub fn autiasppc<'a>(_ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn autibsppc<'a>(_ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        todo!()
    }
}

pub mod pc_rel {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn adr<'a>(
        ctx: &ArmIRCtx,
        immlo: &BitSlice<u32>,
        immhi: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let d: u32 = rd.load();
        let mut imm = bitarr!(u32, Lsb0; 0; 64);
        (&mut imm[0..2]).copy_from_bitslice(immlo);
        (&mut imm[2..21]).copy_from_bitslice(immhi);
        (&mut imm[21..]).fill(*immhi.last().unwrap());
        let imm = imm.load();

        let mut graph = IRGraph::with_capacity(1, 0, 3);
        let block = graph.block_mut(graph.root());

        let pc = block.read_reg64(ctx.reg64::<PC>());
        let (result, _) = block.uadd_i64(pc, Bv64::Lit(imm));
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn adrp<'a>(
        ctx: &ArmIRCtx,
        immlo: &BitSlice<u32>,
        immhi: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let d: u32 = rd.load();
        let mut imm = bitarr!(u32, Lsb0; 0; 64);
        (&mut imm[12..14]).copy_from_bitslice(immlo);
        (&mut imm[14..33]).copy_from_bitslice(immhi);
        (&mut imm[33..]).fill(*immhi.last().unwrap());
        let imm = imm.load();

        let mut graph = IRGraph::with_capacity(1, 0, 4);
        let block = graph.block_mut(graph.root());

        let pc = block.read_reg64(ctx.reg64::<PC>());
        let base = block.and_i64(pc, Bv64::Lit(0xffffffff_fffff000));
        let (result, _) = block.uadd_i64(base, Bv64::Lit(imm));
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }
}

pub mod add_sub {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($sh:ident, $imm12:ident, $rn:ident, $rd:ident, $sz:ty, $ops:literal => $n:ident, $d:ident, $imm:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();
            let $imm: $sz = match $sh {
                false => $imm12.load(),
                true => $imm12.load::<$sz>() << 12,
            };

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn add32<'a>(
        ctx: &ArmIRCtx,
        sh: bool,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(sh, imm12, rn, rd, u32, 4 => n, d, imm, graph, block);

        let operand1 = if n == 31 {
            wsp(el, pstate_sp, ctx, block)
        } else {
            x32(n, ctx, block)
        };
        let (result, _) = block.uadd_i32(operand1, Bv32::Lit(imm));
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
        sh: bool,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(sh, imm12, rn, rd, u32, 10 => n, d, imm, graph, block);

        let operand1 = if n == 31 {
            wsp(el, pstate_sp, ctx, block)
        } else {
            x32(n, ctx, block)
        };
        let (result, overflow) = block.uadd_i32(operand1, Bv32::Lit(imm));

        let n = block.slt_i32(result, Bv32::zero());
        let z = block.eq_i32(result, Bv32::zero());
        let c = overflow;
        let (_, v) = block.sadd_i32(operand1, Bv32::Lit(imm));
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
        sh: bool,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(sh, imm12, rn, rd, u32, 3 => n, d, imm, graph, block);

        let operand1 = if n == 31 {
            wsp(el, pstate_sp, ctx, block)
        } else {
            x32(n, ctx, block)
        };
        let operand2 = Bv32::Lit(imm);
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
        sh: bool,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(sh, imm12, rn, rd, u32, 10 => n, d, imm, graph, block);

        let operand1 = if n == 31 {
            wsp(el, pstate_sp, ctx, block)
        } else {
            x32(n, ctx, block)
        };
        let operand2 = Bv32::Lit(imm);
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
        sh: bool,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(sh, imm12, rn, rd, u64, 3 => n, d, imm, graph, block);

        let operand1 = if n == 31 {
            sp(el, pstate_sp, ctx, block)
        } else {
            x64(n, ctx, block)
        };
        let (result, _) = block.uadd_i64(operand1, Bv64::Lit(imm));
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
        sh: bool,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(sh, imm12, rn, rd, u64, 10 => n, d, imm, graph, block);

        let operand1 = if n == 31 {
            sp(el, pstate_sp, ctx, block)
        } else {
            x64(n, ctx, block)
        };
        let (result, overflow) = block.uadd_i64(operand1, Bv64::Lit(imm));

        let n = block.slt_i64(result, Bv64::zero());
        let z = block.eq_i64(result, Bv64::zero());
        let c = overflow;
        let (_, v) = block.sadd_i64(operand1, Bv64::Lit(imm));
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn sub64<'a>(
        ctx: &ArmIRCtx,
        sh: bool,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(sh, imm12, rn, rd, u64, 3 => n, d, imm, graph, block);

        let operand1 = if n == 31 {
            sp(el, pstate_sp, ctx, block)
        } else {
            x64(n, ctx, block)
        };
        let (result, _) = block.usub_i64(operand1, Bv64::Lit(imm));
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
        sh: bool,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(sh, imm12, rn, rd, u64, 10 => n, d, imm, graph, block);

        let operand1 = if n == 31 {
            sp(el, pstate_sp, ctx, block)
        } else {
            x64(n, ctx, block)
        };
        let operand2 = Bv64::Lit(imm);
        let (result, overflow) = block.usub_i64(operand1, operand2);

        let n = block.slt_i64(result, Bv64::zero());
        let z = block.eq_i64(result, Bv64::zero());
        let c = overflow;
        let (_, v) = block.ssub_i64(operand1, operand2);
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);

        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }
}

pub mod add_sub_tags {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn addg<'a>(
        _ctx: &ArmIRCtx,
        _uimm6: &BitSlice<u32>,
        _uimm4: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn subg<'a>(
        _ctx: &ArmIRCtx,
        _uimm6: &BitSlice<u32>,
        _uimm4: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }
}

pub mod min_max {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($imm8:ident, $rn:ident, $rd:ident, $sz:ty, $ops:literal => $n:ident, $d:ident, $imm:ident, $graph:ident, $block:ident) => {
            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();
            let $imm: $sz = $imm8.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn smax32<'a>(
        ctx: &ArmIRCtx,
        imm8: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm8, rn, rd, i8, 4 => n, d, imm, graph, block);

        let mut cmpval = 0u32;
        cmpval.view_bits_mut::<Lsb0>().store(imm as i32);
        let operand1 = x32(n, ctx, block);
        let result = block.smax_i32(operand1, Bv32::Lit(cmpval));
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn umax32<'a>(
        ctx: &ArmIRCtx,
        imm8: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm8, rn, rd, u32, 4 => n, d, imm, graph, block);

        let operand1 = x32(n, ctx, block);
        let result = block.umax_i32(operand1, Bv32::Lit(imm));
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn smin32<'a>(
        ctx: &ArmIRCtx,
        imm8: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm8, rn, rd, i8, 4 => n, d, imm, graph, block);

        let mut cmpval = 0u32;
        cmpval.view_bits_mut::<Lsb0>().store(imm as i32);
        let operand1 = x32(n, ctx, block);
        let result = block.smin_i32(operand1, Bv32::Lit(cmpval));
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn umin32<'a>(
        ctx: &ArmIRCtx,
        imm8: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm8, rn, rd, u32, 4 => n, d, imm, graph, block);

        let operand1 = x32(n, ctx, block);
        let result = block.umin_i32(operand1, Bv32::Lit(imm));
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn smax64<'a>(
        ctx: &ArmIRCtx,
        imm8: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm8, rn, rd, i8, 3 => n, d, imm, graph, block);

        let mut cmpval = 0u64;
        cmpval.view_bits_mut::<Lsb0>().store(imm as i64);
        let operand1 = x64(n, ctx, block);
        let result = block.smax_i64(operand1, Bv64::Lit(cmpval));
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn umax64<'a>(
        ctx: &ArmIRCtx,
        imm8: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm8, rn, rd, u64, 3 => n, d, imm, graph, block);

        let operand1 = x64(n, ctx, block);
        let result = block.umax_i64(operand1, Bv64::Lit(imm));
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn smin64<'a>(
        ctx: &ArmIRCtx,
        imm8: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm8, rn, rd, i8, 3 => n, d, imm, graph, block);

        let mut cmpval = 0u64;
        cmpval.view_bits_mut::<Lsb0>().store(imm as i64);
        let operand1 = x64(n, ctx, block);
        let result = block.smin_i64(operand1, Bv64::Lit(cmpval));
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn umin64<'a>(
        ctx: &ArmIRCtx,
        imm8: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm8, rn, rd, u64, 3 => n, d, imm, graph, block);

        let operand1 = x64(n, ctx, block);
        let result = block.umin_i64(operand1, Bv64::Lit(imm));
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }
}

pub mod logical {
    use super::decode_bit_masks;
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($big_n:ident, $imms:ident, $immr:ident, $rn:ident, $rd:ident, $sz:literal, $ops:literal => $n:ident, $d:ident, $imm:ident, $graph:ident, $block:ident) => {
            let ($imm, _) =
                decode_bit_masks($big_n, $imms.to_bitvec(), $immr.to_bitvec(), true, $sz);

            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn and32<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, 32, 4 => n, d, imm, graph, block);

        let operand1 = x32(n, ctx, block);
        let result = block.and_i32(operand1, Bv32::Lit(imm.load()));
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

    pub fn orr32<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, 32, 4 => n, d, imm, graph, block);

        let operand1 = x32(n, ctx, block);
        let result = block.or_i32(operand1, Bv32::Lit(imm.load()));
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

    pub fn eor32<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, 32, 4 => n, d, imm, graph, block);

        let operand1 = x32(n, ctx, block);
        let result = block.xor_i32(operand1, Bv32::Lit(imm.load()));
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

    pub fn ands32<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, 32, 10 => n, d, imm, graph, block);

        let operand1 = x32(n, ctx, block);
        let result = block.and_i32(operand1, Bv32::Lit(imm.load()));

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
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, 64, 3 => n, d, imm, graph, block);

        let operand1 = x64(n, ctx, block);
        let result = block.and_i64(operand1, Bv64::Lit(imm.load()));
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

    pub fn orr64<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, 64, 3 => n, d, imm, graph, block);

        let operand1 = x64(n, ctx, block);
        let result = block.or_i64(operand1, Bv64::Lit(imm.load()));
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

    pub fn eor64<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, 64, 3 => n, d, imm, graph, block);

        let operand1 = x64(n, ctx, block);
        let result = block.xor_i64(operand1, Bv64::Lit(imm.load()));
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

    pub fn ands64<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, 64, 9 => n, d, imm, graph, block);

        let operand1 = x64(n, ctx, block);
        let result = block.and_i64(operand1, Bv64::Lit(imm.load()));
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

pub mod move_wide {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($hw:ident, $imm16:ident, $rd:ident, $sz:ty, $ops:literal => $d:ident, $imm:ident, $pos:ident, $graph:ident, $block:ident) => {
            let $d = $rd.load();
            let $imm: $sz = $imm16.load();
            let $pos: $sz = $hw.load::<$sz>() << 4;

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn movn32<'a>(
        ctx: &ArmIRCtx,
        hw: &BitSlice<u32>,
        imm16: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(hw, imm16, rd, u32, 1 => d, imm, pos, graph, block);

        block.write_reg64(ctx.arr64::<X>(d), Bv64::Lit((!(imm << pos as u32)) as u64));
        inc_pc(ctx, block);
        graph
    }

    pub fn movz32<'a>(
        ctx: &ArmIRCtx,
        hw: &BitSlice<u32>,
        imm16: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(hw, imm16, rd, u32, 1 => d, imm, pos, graph, block);

        block.write_reg64(ctx.arr64::<X>(d), Bv64::Lit((imm as u64) << (pos as u32)));
        inc_pc(ctx, block);
        graph
    }

    pub fn movk32<'a>(
        ctx: &ArmIRCtx,
        hw: &BitSlice<u32>,
        imm16: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(hw, imm16, rd, u32, 5 => d, imm, pos, graph, block);

        let result = x32(d, ctx, block);
        // Zero bits (pos,pos+16)
        let result = block.and_i32(result, Bv32::Lit(!(0xffff << pos)));
        let result = block.or_i32(result, Bv32::Lit(imm << pos));
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn movn64<'a>(
        ctx: &ArmIRCtx,
        hw: &BitSlice<u32>,
        imm16: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(hw, imm16, rd, u64, 1 => d, imm, pos, graph, block);

        block.write_reg64(ctx.arr64::<X>(d), Bv64::Lit(!(imm << pos)));
        inc_pc(ctx, block);
        graph
    }

    pub fn movz64<'a>(
        ctx: &ArmIRCtx,
        hw: &BitSlice<u32>,
        imm16: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(hw, imm16, rd, u64, 1 => d, imm, pos, graph, block);

        block.write_reg64(ctx.arr64::<X>(d), Bv64::Lit(imm << pos));
        inc_pc(ctx, block);
        graph
    }

    pub fn movk64<'a>(
        ctx: &ArmIRCtx,
        hw: &BitSlice<u32>,
        imm16: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(hw, imm16, rd, u64, 3 => d, imm, pos, graph, block);

        let result = x64(d, ctx, block);
        // Zero bits (pos,pos+16)
        let result = block.and_i64(result, Bv64::Lit(!(0xffff << pos)));
        let result = block.or_i64(result, Bv64::Lit(imm << pos));
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }
}

pub mod bitfield {
    use super::decode_bit_masks;
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($big_n:ident, $imms:ident, $immr:ident, $rn:ident, $rd:ident, $sz:ty, $ops:literal => $s:ident, $r:ident, $n:ident, $d:ident, $wmask:ident, $tmask:ident, $graph:ident, $block:ident) => {
            let $s: $sz = $imms.load();
            let $r: $sz = $immr.load();

            let ($wmask, $tmask) = decode_bit_masks(
                $big_n,
                $imms.to_bitvec(),
                $immr.to_bitvec(),
                false,
                <$sz>::BITS as usize,
            );

            let $d: u32 = $rd.load();
            let $n: u32 = $rn.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn sbfm32<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, u32, 11 => s, r, n, d, wmask, tmask, graph, block);

        let src = x32(n, ctx, block);
        let bot = block.ror_i32(src, Bv32::Lit(r));
        let bot = block.and_i32(bot, Bv32::Lit(wmask.load()));

        let src_s = block.nth_bit_i32(src, Bv32::Lit(s));
        let top = block.all_b_in_i32(src_s);

        let not_tmask = block.not_i32(Bv32::Lit(tmask.load()));
        let lhs = block.and_i32(top, not_tmask);
        let rhs = block.and_i32(bot, Bv32::Lit(tmask.load()));
        let result = block.or_i32(lhs, rhs);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn bfm32<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, u32, 13 => _s, r, n, d, wmask, tmask, graph, block);

        let dst = x32(d, ctx, block);
        let src = x32(n, ctx, block);

        let not_wmask = block.not_i32(Bv32::Lit(wmask.load()));
        let lhs = block.and_i32(dst, not_wmask);
        let src_ror_r = block.ror_i32(src, Bv32::Lit(r));
        let rhs = block.and_i32(src_ror_r, Bv32::Lit(wmask.load()));
        let bot = block.or_i32(lhs, rhs);

        let not_tmask = block.not_i32(Bv32::Lit(tmask.load()));
        let lhs = block.and_i32(dst, not_tmask);
        let rhs = block.and_i32(bot, Bv32::Lit(tmask.load()));
        let result = block.or_i32(lhs, rhs);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn ubfm32<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, u32, 6 => _s, r, n, d, wmask, tmask, graph, block);

        let src = x32(n, ctx, block);
        let bot = block.ror_i32(src, Bv32::Lit(r));
        let bot = block.and_i32(bot, Bv32::Lit(wmask.load()));

        let result = block.and_i32(bot, Bv32::Lit(tmask.load()));
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn sbfm64<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, u64, 10 => s, r, n, d, wmask, tmask, graph, block);

        let src = x64(n, ctx, block);
        let bot = block.ror_i64(src, Bv64::Lit(r));
        let bot = block.and_i64(bot, Bv64::Lit(wmask.load()));

        let src_s = block.nth_bit_i64(src, Bv64::Lit(s));
        let top = block.all_b_in_i64(src_s);

        let not_tmask = block.not_i64(Bv64::Lit(tmask.load()));
        let lhs = block.and_i64(top, not_tmask);
        let rhs = block.and_i64(bot, Bv64::Lit(tmask.load()));
        let result = block.or_i64(lhs, rhs);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn bfm64<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, u64, 12 => _s, r, n, d, wmask, tmask, graph, block);

        let dst = x64(d, ctx, block);
        let src = x64(n, ctx, block);

        let not_wmask = block.not_i64(Bv64::Lit(wmask.load()));
        let lhs = block.and_i64(dst, not_wmask);
        let src_ror_r = block.ror_i64(src, Bv64::Lit(r));
        let rhs = block.and_i64(src_ror_r, Bv64::Lit(wmask.load()));
        let bot = block.or_i64(lhs, rhs);

        let not_tmask = block.not_i64(Bv64::Lit(tmask.load()));
        let lhs = block.and_i64(dst, not_tmask);
        let rhs = block.and_i64(bot, Bv64::Lit(tmask.load()));
        let result = block.or_i64(lhs, rhs);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }

    pub fn ubfm64<'a>(
        ctx: &ArmIRCtx,
        n: bool,
        immr: &BitSlice<u32>,
        imms: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(n, imms, immr, rn, rd, u64, 5 => _s, r, n, d, wmask, tmask, graph, block);

        let src = x64(n, ctx, block);
        let bot = block.ror_i64(src, Bv64::Lit(r));
        let bot = block.and_i64(bot, Bv64::Lit(wmask.load()));

        let result = block.and_i64(bot, Bv64::Lit(tmask.load()));
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }

        inc_pc(ctx, block);
        graph
    }
}

pub mod extract {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($imms:ident, $rm:ident, $rn:ident, $rd:ident, $sz:ty, $ops:literal => $m:ident, $n:ident, $d:ident, $lsb:ident, $graph:ident, $block:ident) => {
            let $d = $rd.load();
            let $n = $rn.load();
            let $m = $rm.load();
            let $lsb = $imms.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn extr32<'a>(
        ctx: &ArmIRCtx,
        imms: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imms, rm, rn, rd, u32, 5 => m, n, d, lsb, graph, block);

        let operand1 = x32(n, ctx, block);
        let operand2 = x32(m, ctx, block);
        let result = block.fshr_i32(operand1, operand2, lsb);
        let result = block.zext_i32_to_i64(result);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }

    pub fn extr64<'a>(
        ctx: &ArmIRCtx,
        imms: &BitSlice<u32>,
        rm: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rd: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imms, rm, rn, rd, u64, 4 => m, n, d, lsb, graph, block);

        let operand1 = x64(n, ctx, block);
        let operand2 = x64(m, ctx, block);
        let result = block.fshr_i64(operand1, operand2, lsb);
        if d != 31 {
            block.write_reg64(ctx.arr64::<X>(d), result);
        }
        inc_pc(ctx, block);
        graph
    }
}
