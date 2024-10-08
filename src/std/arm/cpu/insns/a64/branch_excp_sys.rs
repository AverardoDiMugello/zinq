pub mod cond_br_imm {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmBrAttrs, ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($imm19:ident, $ops:literal => $offset:ident, $graph:ident, $block:ident) => {
            let mut $offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut $offset[2..21]).copy_from_bitslice($imm19);
            (&mut $offset[21..]).fill(*$imm19.last().unwrap());
            let $offset = $offset.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn b_cond<'a>(
        ctx: &ArmIRCtx,
        imm19: &BitSlice<u32>,
        cond: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm19, 11 => offset, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let pc = block.read_reg64(ctx.reg64::<PC>());

        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond_holds,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }

    pub fn bc_cond<'a>(
        ctx: &ArmIRCtx,
        imm19: &BitSlice<u32>,
        cond: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm19, 11 => offset, graph, block);

        let cond_holds = condition_holds(cond, ctx, block);
        let pc = block.read_reg64(ctx.reg64::<PC>());

        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond_holds,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }
}

pub mod misc_br_imm {
    // TODO: RETAASPPC, RETABSPPC
}

pub mod excp_gen {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::{insns::helpers::*, ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn svc<'a>(_ctx: &ArmIRCtx, imm16: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 1);
        graph
            .block_mut(graph.root())
            .call0(emulate_svc, vec![CallParam::Bv16(Bv16::Lit(imm16.load()))]);
        graph
    }

    pub fn hvc<'a>(_ctx: &ArmIRCtx, imm16: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 1);
        graph
            .block_mut(graph.root())
            .call0(emulate_hvc, vec![CallParam::Bv16(Bv16::Lit(imm16.load()))]);
        graph
    }

    pub fn smc<'a>(_ctx: &ArmIRCtx, imm16: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 1);
        graph
            .block_mut(graph.root())
            .call0(emulate_smc, vec![CallParam::Bv16(Bv16::Lit(imm16.load()))]);
        graph
    }

    // TODO: BRK, HLT, TCANCEL, DCPS1, DCPS2, DCPS3
}

pub mod sys_insn_reg {
    // TODO: WFET, WFIT
}

pub mod hints {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};

    pub fn nop<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        graph.block_mut(graph.root()).nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    // TODO: all...
}

pub mod barriers {
    // TODO: barriers are no-ops, emulators must be strongly-ordered
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};

    pub fn dsb<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        graph.block_mut(graph.root()).nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn dmb<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        graph.block_mut(graph.root()).nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn isb<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        graph.block_mut(graph.root()).nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn sb<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        graph.block_mut(graph.root()).nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn dsb_nxs<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        graph.block_mut(graph.root()).nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod pstate {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn msr<'a>(
        ctx: &ArmIRCtx,
        operand: &BitSlice<u32>,
        field: PSTATEField,
        this_instr: u32,
    ) -> IRGraph<ArmCpu> {
        // TODO: call AArch64.CheckSystemAccess?
        match field {
            PSTATEField::SSBS => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph
                    .block_mut(graph.root())
                    .write_reg1(ctx.reg1::<pstate::SSBS>(), Bit::Lit(operand[0]));
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::SP => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph
                    .block_mut(graph.root())
                    .write_reg1(ctx.reg1::<pstate::SP>(), Bit::Lit(operand[0]));
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::DAIFSet => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph.block_mut(graph.root()).call0(
                    emulate_msr_imm_daifset,
                    vec![CallParam::Bv32(Bv32::Lit(this_instr))],
                );
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::DAIFClr => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph.block_mut(graph.root()).call0(
                    emulate_msr_imm_daifclr,
                    vec![CallParam::Bv32(Bv32::Lit(this_instr))],
                );
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::PAN => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph
                    .block_mut(graph.root())
                    .write_reg1(ctx.reg1::<pstate::PAN>(), Bit::Lit(operand[0]));
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::UAO => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph
                    .block_mut(graph.root())
                    .write_reg1(ctx.reg1::<pstate::UAO>(), Bit::Lit(operand[0]));
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::DIT => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph
                    .block_mut(graph.root())
                    .write_reg1(ctx.reg1::<pstate::DIT>(), Bit::Lit(operand[0]));
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::TCO => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph
                    .block_mut(graph.root())
                    .write_reg1(ctx.reg1::<pstate::TCO>(), Bit::Lit(operand[0]));
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::ALLINT => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph.block_mut(graph.root()).call0(
                    emulate_msr_imm_allint,
                    vec![CallParam::Bv32(Bv32::Lit(this_instr))],
                );
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
            PSTATEField::SVCRSM => todo!("SVE/SME PState"),
            PSTATEField::SVCRZA => todo!("SVE/SME PState"),
            PSTATEField::SVCRSMZA => todo!("SVE/SME PState"),
            PSTATEField::PM => {
                let mut graph = IRGraph::with_capacity(1, 0, 1);
                graph
                    .block_mut(graph.root())
                    .write_reg1(ctx.reg1::<pstate::PM>(), Bit::Lit(operand[0]));
                inc_pc(ctx, graph.block_mut(graph.root()));
                graph
            }
        }
    }

    pub fn cfinv<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 12);
        let block = graph.block_mut(graph.root());

        let pstate_c = block.read_reg1(ctx.reg1::<pstate::C>());
        let not_c = block.not_b(pstate_c);
        block.write_reg1(ctx.reg1::<pstate::C>(), not_c);
        inc_pc(ctx, block);
        graph
    }

    pub fn xaflag<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 12);
        let block = graph.block_mut(graph.root());

        let pstate_c = block.read_reg1(ctx.reg1::<pstate::C>());
        let pstate_z = block.read_reg1(ctx.reg1::<pstate::Z>());
        // bit n = NOT(PSTATE.C) AND NOT(PSTATE.Z);
        let not_c = block.not_b(pstate_c);
        let not_z = block.not_b(pstate_z);
        let n = block.and_b(not_c, not_z);
        // bit z = PSTATE.Z AND PSTATE.C;
        let z = block.and_b(pstate_z, pstate_c);
        // bit c = PSTATE.C OR PSTATE.Z;
        let c = block.or_b(pstate_c, pstate_z);
        // bit v = NOT(PSTATE.C) AND PSTATE.Z;
        let v = block.and_b(not_c, pstate_z);

        // PSTATE.N = n;
        // PSTATE.Z = z;
        // PSTATE.C = c;
        // PSTATE.V = v;
        block.write_reg1(ctx.reg1::<pstate::N>(), n);
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), v);
        inc_pc(ctx, block);
        graph
    }

    pub fn axflag<'a>(ctx: &ArmIRCtx) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 10);
        let block = graph.block_mut(graph.root());
        // bit z = PSTATE.Z OR PSTATE.V;
        let z = block.read_reg1(ctx.reg1::<pstate::Z>());
        let v = block.read_reg1(ctx.reg1::<pstate::V>());
        let z = block.or_b(z, v);
        // bit c = PSTATE.C AND NOT(PSTATE.V);
        let c = block.read_reg1(ctx.reg1::<pstate::C>());
        let not_v = block.not_b(v);
        let c = block.and_b(c, not_v);
        // PSTATE.<N,Z,C,V> = '0' : z : c : '0';
        block.write_reg1(ctx.reg1::<pstate::N>(), Bit::zero());
        block.write_reg1(ctx.reg1::<pstate::Z>(), z);
        block.write_reg1(ctx.reg1::<pstate::C>(), c);
        block.write_reg1(ctx.reg1::<pstate::V>(), Bit::zero());
        inc_pc(ctx, block);
        graph
    }
}

pub mod sys_with_result {
    // TODO: TSTART, TTEST
}

pub mod sys_insns {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn sys<'a>(
        ctx: &ArmIRCtx,
        op1: &BitSlice<u32>,
        crn: &BitSlice<u32>,
        crm: &BitSlice<u32>,
        op2: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        let sys_op0: u8 = 1;
        let sys_op1: u8 = op1.load();
        let sys_crn: u8 = crn.load();
        let sys_crm: u8 = crm.load();
        let sys_op2: u8 = op2.load();
        let t: u32 = rt.load();
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        let block = graph.block_mut(graph.root());
        let data = x64(t, ctx, block);
        block.call0(
            emulate_sys,
            vec![
                CallParam::Bv8(Bv8::Lit(sys_op0)),
                CallParam::Bv8(Bv8::Lit(sys_op1)),
                CallParam::Bv8(Bv8::Lit(sys_crn)),
                CallParam::Bv8(Bv8::Lit(sys_crm)),
                CallParam::Bv8(Bv8::Lit(sys_op2)),
                CallParam::Bv64(data),
            ],
        );
        inc_pc(ctx, block);
        graph
    }

    // TODO: SYSL panics
}

pub mod sys_reg_move {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn msr<'a>(ctx: &ArmIRCtx, this_instr: u32, rt: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        let t: u32 = rt.load();
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        let block = graph.block_mut(graph.root());
        let data = x64(t, ctx, block);
        block.call0(
            emulate_msr_reg,
            vec![
                CallParam::Bv32(Bv32::Lit(this_instr)),
                CallParam::Bv64(data),
            ],
        );
        inc_pc(ctx, block);
        graph
    }

    pub fn mrs<'a>(ctx: &ArmIRCtx, this_instr: u32, rt: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        let t: u32 = rt.load();
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        let block = graph.block_mut(graph.root());
        let data = block.call_i64(emulate_mrs, vec![CallParam::Bv32(Bv32::Lit(this_instr))]);
        if t != 31 {
            block.write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, block);
        graph
    }
}

pub mod sys_pair_insns {
    // TODO: SYSP
}

pub mod uncond_br_reg {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmBrAttrs, ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    pub fn br<'a>(ctx: &ArmIRCtx, rn: &BitSlice<u32>, _in_guarded_page: bool) -> IRGraph<ArmCpu> {
        let n: u32 = rn.load();
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        let block = graph.block_mut(graph.root());
        // TODO: set BTypeNext?
        let target = x64(n, ctx, block);
        block.br(
            target,
            ArmBrAttrs::General {
                branch_type: BranchType::INDIR,
                branch_conditional: false,
            },
        );
        graph
    }

    // TODO: BRAAZ, BRABZ

    pub fn blr<'a>(ctx: &ArmIRCtx, rn: &BitSlice<u32>, _gcs: bool) -> IRGraph<ArmCpu> {
        let n: u32 = rn.load();
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        let block = graph.block_mut(graph.root());
        // TODO: GCS feature
        // TODO: set BTypeNext?

        let target = x64(n, ctx, block);
        let pc = block.read_reg64(ctx.reg64::<PC>());
        let (ret, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.write_reg64(ctx.arr64::<X>(30), ret);
        block.br(
            target,
            ArmBrAttrs::General {
                branch_type: BranchType::INDCALL,
                branch_conditional: false,
            },
        );
        graph
    }

    // TODO: BLRAAZ, BLRABZ

    pub fn ret<'a>(ctx: &ArmIRCtx, rn: &BitSlice<u32>, _gcs: bool) -> IRGraph<ArmCpu> {
        let n: u32 = rn.load();
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        let block = graph.block_mut(graph.root());
        // TODO: GCS feature
        // TODO: set BTypeNext?
        let target = x64(n, ctx, block);
        block.br(
            target,
            ArmBrAttrs::General {
                branch_type: BranchType::RET,
                branch_conditional: false,
            },
        );
        graph
    }

    // TODO: RETAA, RETAASPPC, RETABSPPC

    pub fn eret<'a>(ctx: &ArmIRCtx, el: EL) -> IRGraph<ArmCpu> {
        let mut graph = IRGraph::with_capacity(1, 0, 2);
        let block = graph.block_mut(graph.root());
        let target = block.read_reg64(ctx.elr(el));
        block.br(
            target,
            ArmBrAttrs::Eret {
                pac: false,
                use_key_a: true,
                auth_then_branch: false,
            },
        );
        graph
    }

    // TODO: ERETAA, ERETAB

    // TODO: DRPS

    // TODO: BRAA, BRAB, BLRAA, BLRAB
}

pub mod uncond_br_imm {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmBrAttrs, ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($imm26:ident, $ops:literal => $offset:ident, $graph:ident, $block:ident) => {
            let mut $offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut $offset[2..28]).copy_from_bitslice($imm26);
            (&mut $offset[28..]).fill(*$imm26.last().unwrap());
            let $offset = $offset.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn b<'a>(ctx: &ArmIRCtx, imm26: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        decode!(imm26, 3 => offset, graph, block);

        let pc = block.read_reg64(ctx.reg64::<PC>());
        let (addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        block.br(
            addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: false,
            },
        );
        graph
    }

    pub fn bl<'a>(ctx: &ArmIRCtx, imm26: &BitSlice<u32>, _gcs: bool) -> IRGraph<ArmCpu> {
        decode!(imm26, 5 => offset, graph, block);

        // TODO: GCS feature

        let pc = block.read_reg64(ctx.reg64::<PC>());
        let (target, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (ret, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.write_reg64(ctx.arr64::<X>(30), ret);
        block.br(
            target,
            ArmBrAttrs::General {
                branch_type: BranchType::DIRCALL,
                branch_conditional: false,
            },
        );
        graph
    }
}

pub mod cmp_br_imm {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmBrAttrs, ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($imm19:ident, $rt:ident, $ops:literal => $offset:ident, $t:ident, $graph:ident, $block:ident) => {
            let $t: u32 = $rt.load();
            let mut $offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut $offset[2..21]).copy_from_bitslice($imm19);
            (&mut $offset[21..]).fill(*$imm19.last().unwrap());
            let $offset = $offset.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn cbz32<'a>(ctx: &ArmIRCtx, imm19: &BitSlice<u32>, rt: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        decode!(imm19, rt, 6 => offset, t, graph, block);

        let operand1 = x32(t, ctx, block);
        let cond = block.eq_i32(operand1, Bv32::zero());

        let pc = block.read_reg64(ctx.reg64::<PC>());
        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }

    pub fn cbz64<'a>(ctx: &ArmIRCtx, imm19: &BitSlice<u32>, rt: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        decode!(imm19, rt, 6 => offset, t, graph, block);

        let operand1 = x64(t, ctx, block);
        let cond = block.eq_i64(operand1, Bv64::zero());

        let pc = block.read_reg64(ctx.reg64::<PC>());
        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }

    pub fn cbnz32<'a>(
        ctx: &ArmIRCtx,
        imm19: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm19, rt, 6 => offset, t, graph, block);

        let operand1 = x32(t, ctx, block);
        let cond = block.ne_i32(operand1, Bv32::zero());

        let pc = block.read_reg64(ctx.reg64::<PC>());
        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }

    pub fn cbnz64<'a>(
        ctx: &ArmIRCtx,
        imm19: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(imm19, rt, 6 => offset, t, graph, block);

        let operand1 = x64(t, ctx, block);
        let cond = block.ne_i64(operand1, Bv64::zero());

        let pc = block.read_reg64(ctx.reg64::<PC>());
        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }
}

pub mod tst_br_imm {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmBrAttrs, ArmCpu, ArmIRCtx};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($b5:ident, $b40:ident, $imm14:ident, $rt:ident, $ops:literal => $t:ident, $offset:ident, $bit_pos:ident, $graph:ident, $block:ident) => {
            let $t: u32 = $rt.load();
            let mut $bit_pos = bitarr!(u32, Lsb0; 0; 64);
            (&mut $bit_pos[0..5]).copy_from_bitslice($b40);
            $bit_pos.set(5, $b5);
            let $bit_pos = $bit_pos.load();

            let mut $offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut $offset[2..16]).copy_from_bitslice($imm14);
            (&mut $offset[16..]).fill(*$imm14.last().unwrap());
            let $offset = $offset.load();

            let mut $graph = IRGraph::with_capacity(1, 0, $ops);
            let $block = $graph.block_mut($graph.root());
        };
    }

    pub fn tbz32<'a>(
        ctx: &ArmIRCtx,
        b5: bool,
        b40: &BitSlice<u32>,
        imm14: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(b5, b40, imm14, rt, 7 => t, offset, bit_pos, graph, block);

        let operand = x32(t, ctx, block);
        let bit = block.nth_bit_i32(operand, Bv32::Lit(bit_pos));

        let cond = block.eq_b(bit, Bit::zero());
        let pc = block.read_reg64(ctx.reg64::<PC>());

        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }

    pub fn tbz64<'a>(
        ctx: &ArmIRCtx,
        b5: bool,
        b40: &BitSlice<u32>,
        imm14: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(b5, b40, imm14, rt, 7 => t, offset, bit_pos, graph, block);

        let operand = x64(t, ctx, block);
        let bit = block.nth_bit_i64(operand, Bv64::Lit(bit_pos));

        let cond = block.eq_b(bit, Bit::zero());
        let pc = block.read_reg64(ctx.reg64::<PC>());

        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }

    pub fn tbnz32<'a>(
        ctx: &ArmIRCtx,
        b5: bool,
        b40: &BitSlice<u32>,
        imm14: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(b5, b40, imm14, rt, 7 => t, offset, bit_pos, graph, block);

        let operand = x32(t, ctx, block);
        let bit = block.nth_bit_i32(operand, Bv32::Lit(bit_pos));

        let cond = block.eq_b(bit, Bit::one());
        let pc = block.read_reg64(ctx.reg64::<PC>());

        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }

    pub fn tbnz64<'a>(
        ctx: &ArmIRCtx,
        b5: bool,
        b40: &BitSlice<u32>,
        imm14: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        decode!(b5, b40, imm14, rt, 7 => t, offset, bit_pos, graph, block);

        let operand = x64(t, ctx, block);
        let bit = block.nth_bit_i64(operand, Bv64::Lit(bit_pos));

        let cond = block.eq_b(bit, Bit::one());
        let pc = block.read_reg64(ctx.reg64::<PC>());

        let (t_addr, _) = block.uadd_i64(pc, Bv64::Lit(offset));
        let (f_addr, _) = block.uadd_i64(pc, Bv64::Lit(4));
        block.cbr(
            cond,
            t_addr,
            f_addr,
            ArmBrAttrs::General {
                branch_type: BranchType::DIR,
                branch_conditional: true,
            },
        );
        graph
    }
}
