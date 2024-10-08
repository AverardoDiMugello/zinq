use crate::std::arm::cpu::registers::*;
use bitvec::prelude::*;

use super::helpers::PSTATEField;
use crate::model::ir;
use crate::std::arm::cpu::{pseudoc::*, registers::pstate, ArmCpu, ArmCtx, ArmIRCtx, Feat};

mod branch_excp_sys;
mod data_proc_imm;
mod data_proc_reg;
// mod data_proc_simdfp;
mod load_store;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn decode_a64(&mut self, insn: u32, ir_ctx: &ArmIRCtx) -> Option<ir::IRGraph<ArmCpu>> {
        let bits = insn.view_bits::<Lsb0>();
        let op0 = bits[31];
        let op1 = &bits[25..29];
        match (op0, (op1[3], op1[2], op1[1], op1[0])) {
            // Reserved
            (false, (false, false, false, false)) => None,
            // SME
            (true, (false, false, false, false)) => todo!("SME"),
            // Unallocated
            (_, (false, false, false, true)) => None,
            // SVE
            (_, (false, false, true, false)) => todo!("SVE"),
            // Unallocated
            (_, (false, false, true, true)) => None,
            // Data Proc - Immediate
            (_, (true, false, false, _)) => {
                let op0 = &bits[29..31];
                let op1 = &bits[22..26];
                match ((op0[1], op0[0]), (op1[3], op1[2], op1[1], op1[0])) {
                    // Data-processing (1 source immediate)
                    ((true, true), (true, true, true, _)) => {
                        if self.is_feat_impl(Feat::PAuthLR) {
                            return None;
                        }
                        match (
                            bits[31],
                            (bits[22], bits[21]),
                            (bits[4], bits[3], bits[2], bits[1], bits[0]),
                        ) {
                            (true, (false, false), (true, true, true, true, true)) => {
                                Some(data_proc_imm::one_source::autiasppc(ir_ctx))
                            }
                            (true, (false, true), (true, true, true, true, true)) => {
                                Some(data_proc_imm::one_source::autibsppc(ir_ctx))
                            }
                            _ => return None,
                        }
                    }
                    // PC-rel. addressing
                    ((_, _), (false, false, _, _)) => {
                        let op = bits[31];
                        let immlo = &bits[29..31];
                        let immhi = &bits[5..24];
                        let rd = &bits[0..5];
                        match op {
                            false => Some(data_proc_imm::pc_rel::adr(ir_ctx, immlo, immhi, rd)),
                            true => Some(data_proc_imm::pc_rel::adrp(ir_ctx, immlo, immhi, rd)),
                        }
                    }
                    // Add/subtract (immediate)
                    ((_, _), (false, true, false, _)) => {
                        let sf = bits[31];
                        let op = bits[30];
                        let s = bits[29];
                        let sh = bits[22];
                        let imm12 = &bits[10..22];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, op, s) {
                            (false, false, false) => Some(data_proc_imm::add_sub::add32(
                                ir_ctx,
                                sh,
                                imm12,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (false, false, true) => Some(data_proc_imm::add_sub::adds32(
                                ir_ctx,
                                sh,
                                imm12,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (false, true, false) => Some(data_proc_imm::add_sub::sub32(
                                ir_ctx,
                                sh,
                                imm12,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (false, true, true) => Some(data_proc_imm::add_sub::subs32(
                                ir_ctx,
                                sh,
                                imm12,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (true, false, false) => Some(data_proc_imm::add_sub::add64(
                                ir_ctx,
                                sh,
                                imm12,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (true, false, true) => Some(data_proc_imm::add_sub::adds64(
                                ir_ctx,
                                sh,
                                imm12,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (true, true, false) => Some(data_proc_imm::add_sub::sub64(
                                ir_ctx,
                                sh,
                                imm12,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (true, true, true) => Some(data_proc_imm::add_sub::subs64(
                                ir_ctx,
                                sh,
                                imm12,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                        }
                    }
                    // Add/subtract (immediate, with tags)
                    ((_, _), (false, true, true, false)) => {
                        if !self.is_feat_impl(Feat::MTE) {
                            return None;
                        }
                        let sf = bits[31];
                        let op = bits[30];
                        let s = bits[29];
                        let uimm6 = &bits[16..22];
                        let uimm4 = &bits[10..14];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, op, s) {
                            (false, _, _) => None,   // Unallocated
                            (true, _, true) => None, // Unallocated
                            (true, false, false) => Some(data_proc_imm::add_sub_tags::addg(
                                ir_ctx, uimm6, uimm4, rn, rd,
                            )),
                            (true, true, false) => Some(data_proc_imm::add_sub_tags::subg(
                                ir_ctx, uimm6, uimm4, rn, rd,
                            )),
                        }
                    }
                    // Min/max (immediate)
                    ((_, _), (false, true, true, true)) => {
                        if !self.is_feat_impl(Feat::CSSC) {
                            return None;
                        }
                        let sf = bits[31];
                        let op = bits[30];
                        let s = bits[29];
                        let opc = &bits[18..22];
                        let imm8 = &bits[10..18];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, op, s, (opc[3], opc[2], opc[1], opc[0])) {
                            (_, false, false, (false, true, _, _)) => None, // Unallocated
                            (_, false, false, (true, _, _, _)) => None,     // Unallocated
                            (_, false, true, _) => None,                    // Unallocated
                            (_, true, _, _) => None,                        // Unallocated
                            (false, false, false, (false, false, false, false)) => {
                                Some(data_proc_imm::min_max::smax32(ir_ctx, imm8, rn, rd))
                            }
                            (false, false, false, (false, false, false, true)) => {
                                Some(data_proc_imm::min_max::umax32(ir_ctx, imm8, rn, rd))
                            }
                            (false, false, false, (false, false, true, false)) => {
                                Some(data_proc_imm::min_max::smin32(ir_ctx, imm8, rn, rd))
                            }
                            (false, false, false, (false, false, true, true)) => {
                                Some(data_proc_imm::min_max::umin32(ir_ctx, imm8, rn, rd))
                            }
                            (true, false, false, (false, false, false, false)) => {
                                Some(data_proc_imm::min_max::smax64(ir_ctx, imm8, rn, rd))
                            }
                            (true, false, false, (false, false, false, true)) => {
                                Some(data_proc_imm::min_max::umax64(ir_ctx, imm8, rn, rd))
                            }
                            (true, false, false, (false, false, true, false)) => {
                                Some(data_proc_imm::min_max::smin64(ir_ctx, imm8, rn, rd))
                            }
                            (true, false, false, (false, false, true, true)) => {
                                Some(data_proc_imm::min_max::umin64(ir_ctx, imm8, rn, rd))
                            }
                        }
                    }
                    // Logical (immediate)
                    ((_, _), (true, false, false, _)) => {
                        let sf = bits[31];
                        let opc = &bits[29..31];
                        let n = bits[22];
                        let immr = &bits[16..22];
                        let imms = &bits[10..16];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, (opc[1], opc[0]), n) {
                            (false, _, true) => None, // Unallocated
                            (false, (false, false), false) => Some(data_proc_imm::logical::and32(
                                ir_ctx,
                                n,
                                immr,
                                imms,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (false, (false, true), false) => Some(data_proc_imm::logical::orr32(
                                ir_ctx,
                                n,
                                immr,
                                imms,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (false, (true, false), false) => Some(data_proc_imm::logical::eor32(
                                ir_ctx,
                                n,
                                immr,
                                imms,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (false, (true, true), false) => Some(data_proc_imm::logical::ands32(
                                ir_ctx, n, immr, imms, rn, rd,
                            )),
                            (true, (false, false), _) => Some(data_proc_imm::logical::and64(
                                ir_ctx,
                                n,
                                immr,
                                imms,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (true, (false, true), _) => Some(data_proc_imm::logical::orr64(
                                ir_ctx,
                                n,
                                immr,
                                imms,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (true, (true, false), _) => Some(data_proc_imm::logical::eor64(
                                ir_ctx,
                                n,
                                immr,
                                imms,
                                rn,
                                rd,
                                self.curr_el(),
                                self.read::<pstate::SP>() != 0,
                            )),
                            (true, (true, true), _) => Some(data_proc_imm::logical::ands64(
                                ir_ctx, n, immr, imms, rn, rd,
                            )),
                        }
                    }
                    // Move wide (immediate)
                    ((_, _), (true, false, true, _)) => {
                        let sf = bits[31];
                        let opc = &bits[29..31];
                        let hw = &bits[21..23];
                        let imm16 = &bits[5..21];
                        let rd = &bits[0..5];
                        match (sf, (opc[1], opc[0]), (hw[1], hw[0])) {
                            (_, (false, true), (false, _)) => None, // Unallocated
                            (false, _, (true, _)) => None,          // Unallocated
                            (false, (false, false), (false, _)) => {
                                Some(data_proc_imm::move_wide::movn32(ir_ctx, hw, imm16, rd))
                            }
                            (false, (true, false), (false, _)) => {
                                Some(data_proc_imm::move_wide::movz32(ir_ctx, hw, imm16, rd))
                            }
                            (false, (true, true), (false, _)) => {
                                Some(data_proc_imm::move_wide::movk32(ir_ctx, hw, imm16, rd))
                            }
                            (true, (false, false), _) => {
                                Some(data_proc_imm::move_wide::movn64(ir_ctx, hw, imm16, rd))
                            }
                            (true, (false, true), (true, _)) => None, // Unallocated
                            (true, (true, false), _) => {
                                Some(data_proc_imm::move_wide::movz64(ir_ctx, hw, imm16, rd))
                            }
                            (true, (true, true), _) => {
                                Some(data_proc_imm::move_wide::movk64(ir_ctx, hw, imm16, rd))
                            }
                        }
                    }
                    // Bitfield
                    ((_, _), (true, true, false, _)) => {
                        let sf = bits[31];
                        let opc = &bits[29..31];
                        let n = bits[22];
                        let immr = &bits[16..22];
                        let imms = &bits[10..16];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, (opc[1], opc[0]), n) {
                            (false, _, true) => None, // Unallocated
                            (false, (false, false), false) => Some(
                                data_proc_imm::bitfield::sbfm32(ir_ctx, n, immr, imms, rn, rd),
                            ),
                            (false, (false, true), false) => Some(data_proc_imm::bitfield::bfm32(
                                ir_ctx, n, immr, imms, rn, rd,
                            )),
                            (false, (true, false), false) => Some(data_proc_imm::bitfield::ubfm32(
                                ir_ctx, n, immr, imms, rn, rd,
                            )),
                            (false, (true, true), false) => None, // Unallocated
                            (true, _, false) => None,             // Unallocated
                            (true, (false, false), true) => Some(data_proc_imm::bitfield::sbfm64(
                                ir_ctx, n, immr, imms, rn, rd,
                            )),
                            (true, (false, true), true) => Some(data_proc_imm::bitfield::bfm64(
                                ir_ctx, n, immr, imms, rn, rd,
                            )),
                            (true, (true, false), true) => Some(data_proc_imm::bitfield::ubfm64(
                                ir_ctx, n, immr, imms, rn, rd,
                            )),
                            (true, (true, true), true) => None, // Unallocated
                        }
                    }
                    // Extract
                    (op0, (true, true, true, _)) if op0 != (true, true) => {
                        let sf = bits[31];
                        let op2 = bits[30];
                        let op1 = bits[29];
                        let n = bits[22];
                        let o0 = bits[21];
                        let rm = &bits[16..21];
                        let imms = &bits[10..16];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, (op2, op1), n, o0, imms[5]) {
                            (_, (false, false), _, true, _) => None, // Unallocated
                            (_, (false, true), _, _, _) => None,     // Unallocated
                            (_, (true, _), _, _, _) => None,         // Unallocated
                            (false, (false, false), false, false, false) => {
                                Some(data_proc_imm::extract::extr32(ir_ctx, imms, rm, rn, rd))
                            }
                            (false, (false, false), false, false, true) => None, // Unallocated
                            (false, (false, false), true, false, _) => None,     // Unallocated
                            (true, (false, false), false, false, _) => None,     // Unallocated
                            (true, (false, false), true, false, _) => {
                                Some(data_proc_imm::extract::extr64(ir_ctx, imms, rm, rn, rd))
                            }
                        }
                    }
                    // Unspeficied by the architecture
                    _ => return None,
                }
            }
            // Branches, Exception-generating, and System
            (_, (true, false, true, _)) => {
                let op0 = &bits[29..32];
                let op1 = &bits[12..26];
                let op2 = &bits[0..5];
                match (
                    (op0[2], op0[1], op0[0]),
                    (
                        op1[13], op1[12], op1[11], op1[10], op1[9], op1[8], op1[7], op1[6], op1[5],
                        op1[4], op1[3], op1[2], op1[1], op1[0],
                    ),
                ) {
                    // Conditional branch (immediate)
                    ((false, true, false), (false, false, ..)) => {
                        let imm19 = &bits[5..24];
                        let o0 = bits[4];
                        let cond = &bits[0..4];
                        match o0 {
                            false => {
                                Some(branch_excp_sys::cond_br_imm::b_cond(ir_ctx, imm19, cond))
                            }
                            true => {
                                if !self.is_feat_impl(Feat::HBC) {
                                    None
                                } else {
                                    Some(branch_excp_sys::cond_br_imm::bc_cond(ir_ctx, imm19, cond))
                                }
                            }
                        }
                    }
                    // Miscellaneous branch (immediate)
                    ((false, true, false), (false, true, ..)) => {
                        let opc = &bits[21..24];
                        let imm16 = &bits[5..21];
                        let op2 = &bits[0..5];
                        match (opc.load::<u8>(), op2.load::<u8>()) {
                            (0b000, 0b11111) => {
                                // RETAASPPC
                                if !self.is_feat_impl(Feat::PAuthLR) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            (0b001, 0b11111) => {
                                // RETABSPPC
                                if !self.is_feat_impl(Feat::PAuthLR) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            _ => None, // Unallocated
                        }
                    }
                    // Exception generation
                    ((true, true, false), (false, false, ..)) => {
                        let opc = &bits[21..24];
                        let imm16 = &bits[5..21];
                        let op2 = &bits[2..5];
                        let ll = &bits[0..2];
                        match (
                            (opc[2], opc[1], opc[0]),
                            (op2[2], op2[1], op2[0]),
                            (ll[1], ll[0]),
                        ) {
                            (_, (false, false, true), _) => None, // Unallocated
                            (_, (false, true, _), _) => None,     // Unallocated
                            (_, (true, _, _), _) => None,         // Unallocated
                            ((_, true, true), (false, false, false), (false, true)) => None, // Unallocated
                            ((_, true, true), (false, false, false), (true, _)) => None, // Unallocated
                            ((false, false, false), (false, false, false), (false, false)) => None, // Unallocated
                            // SVC
                            ((false, false, false), (false, false, false), (false, true)) => {
                                Some(branch_excp_sys::excp_gen::svc(ir_ctx, imm16))
                            }
                            // HVC
                            ((false, false, false), (false, false, false), (true, false)) => {
                                Some(branch_excp_sys::excp_gen::hvc(ir_ctx, imm16))
                            }
                            // SMC
                            ((false, false, false), (false, false, false), (true, true)) => {
                                Some(branch_excp_sys::excp_gen::smc(ir_ctx, imm16))
                            }
                            // BRK
                            ((false, false, true), (false, false, false), (false, false)) => {
                                todo!()
                            }
                            ((false, false, true), (false, false, false), (false, true)) => None, // Unallocated
                            ((false, false, true), (false, false, false), (true, _)) => None, // Unallocated
                            // HLT
                            ((false, true, false), (false, false, false), (false, false)) => {
                                todo!()
                            }
                            ((false, true, false), (false, false, false), (false, true)) => None, // Unallocated
                            ((false, true, false), (false, false, false), (true, _)) => None, // Unallocated
                            // TCANCEL
                            ((false, true, true), (false, false, false), (false, false)) => {
                                if !self.is_feat_impl(Feat::TME) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            ((true, _, false), (false, false, false), _) => None, // Unallocated
                            ((true, _, true), (false, false, false), (false, false)) => None, // Unallocated
                            // DCPS1
                            ((true, false, true), (false, false, false), (false, true)) => todo!(),
                            // DCPS2
                            ((true, false, true), (false, false, false), (true, false)) => todo!(),
                            // DCPS3
                            ((true, false, true), (false, false, false), (true, true)) => todo!(),
                        }
                    }
                    // System instructions with register argument
                    (
                        (true, true, false),
                        (
                            false,
                            true,
                            false,
                            false,
                            false,
                            false,
                            false,
                            false,
                            true,
                            true,
                            false,
                            false,
                            false,
                            true,
                        ),
                    ) => {
                        let crm = &bits[8..12];
                        let op2 = &bits[5..8];
                        let rt = &bits[0..5];
                        if crm.load::<u8>() == 0 {
                            match (op2[2], op2[1], op2[0]) {
                                (false, false, false) => {
                                    if !self.is_feat_impl(Feat::WFxT) {
                                        None
                                    } else {
                                        // WFET
                                        todo!()
                                    }
                                }
                                (false, false, true) => {
                                    if !self.is_feat_impl(Feat::WFxT) {
                                        None
                                    } else {
                                        // WFIT
                                        todo!()
                                    }
                                }
                                (false, true, _) => None, // Unallocated
                                (true, ..) => None,       // Unallocated
                            }
                        } else {
                            // Unallocated
                            None
                        }
                    }
                    // Hints
                    (
                        (true, true, false),
                        (
                            false,
                            true,
                            false,
                            false,
                            false,
                            false,
                            false,
                            false,
                            true,
                            true,
                            false,
                            false,
                            true,
                            false,
                        ),
                    ) => {
                        if op2.load::<u8>() == 0b11111 {
                            let crm = &bits[8..12];
                            let op2 = &bits[5..8];
                            match ((crm[3], crm[2], crm[1], crm[0]), (op2[2], op2[1], op2[0])) {
                                ((false, false, false, false), (false, false, false)) => {
                                    Some(branch_excp_sys::hints::nop(ir_ctx))
                                }
                                _ => Some(branch_excp_sys::hints::nop(ir_ctx)),
                            }
                        } else {
                            None // Unallocated
                        }
                    }
                    // Barriers
                    (
                        (true, true, false),
                        (
                            false,
                            true,
                            false,
                            false,
                            false,
                            false,
                            false,
                            false,
                            true,
                            true,
                            false,
                            false,
                            true,
                            true,
                        ),
                    ) => {
                        let crm = &bits[8..12];
                        let op2 = &bits[5..8];
                        let rt = &bits[0..5];
                        match (
                            (crm[3], crm[2], crm[1], crm[0]),
                            (op2[2], op2[1], op2[0]),
                            rt.load::<u8>(),
                        ) {
                            // CLREX
                            (_, (false, true, false), 0b11111) => todo!(),
                            // DSB -- Memory barrier
                            (_, (true, false, false), 0b11111) => {
                                Some(branch_excp_sys::barriers::dsb(ir_ctx))
                            }
                            // DMB
                            (_, (true, false, true), 0b11111) => {
                                Some(branch_excp_sys::barriers::dmb(ir_ctx))
                            }
                            // ISB
                            (_, (true, true, false), 0b11111) => {
                                Some(branch_excp_sys::barriers::isb(ir_ctx))
                            }
                            // SB
                            (_, (true, true, true), 0b11111) => {
                                if !self.is_feat_impl(Feat::SB) {
                                    None
                                } else {
                                    Some(branch_excp_sys::barriers::sb(ir_ctx))
                                }
                            }
                            ((_, _, _, true), (false, true, true), 0b11111) => None, // Unallocated
                            ((_, _, false, _), (false, false, _), 0b11111) => None,  // Unallocated
                            ((_, _, true, _), (false, false, false), 0b11111) => None, // Unallocated
                            // DSB -- Memory nXS barrier
                            ((_, _, true, false), (false, false, true), 0b11111) => {
                                if !self.is_feat_impl(Feat::XS) {
                                    None
                                } else {
                                    Some(branch_excp_sys::barriers::dsb_nxs(ir_ctx))
                                }
                            }
                            ((_, _, true, false), (false, true, true), 0b11111) => None, // Unallocated
                            ((_, _, true, true), (false, false, true), 0b11111) => None, // Unallocated
                            ((false, false, false, false), (false, true, true), 0b11111) => {
                                if !self.is_feat_impl(Feat::TME) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            ((false, true, false, false), (false, true, true), 0b11111) => None, // Unallocated
                            ((true, _, false, false), (false, true, true), 0b11111) => None, // Unallocated
                            _ => None, // Unallocated
                        }
                    }
                    // PSTATE
                    (
                        (true, true, false),
                        (
                            false,
                            true,
                            false,
                            false,
                            false,
                            false,
                            false,
                            _,
                            _,
                            _,
                            false,
                            true,
                            false,
                            false,
                        ),
                    ) => {
                        let op1 = &bits[16..19];
                        let crm = &bits[8..12];
                        let op2 = &bits[5..8];
                        let rt = &bits[0..5];
                        match (op1.load::<u8>(), op2.load::<u8>(), rt.load::<u8>()) {
                            (0b000, 0b000, 0b11111) => {
                                // CFINV
                                if !self.is_feat_impl(Feat::FlagM) {
                                    None
                                } else {
                                    Some(branch_excp_sys::pstate::cfinv(ir_ctx))
                                }
                            }
                            (0b000, 0b001, 0b11111) => {
                                // XAFLAG
                                if !self.is_feat_impl(Feat::FlagM2) {
                                    None
                                } else {
                                    Some(branch_excp_sys::pstate::xaflag(ir_ctx))
                                }
                            }
                            (0b000, 0b010, 0b11111) => {
                                // AXFLAG
                                if !self.is_feat_impl(Feat::FlagM2) {
                                    None
                                } else {
                                    Some(branch_excp_sys::pstate::axflag(ir_ctx))
                                }
                            }
                            (_, _, 0b11111) => {
                                // MSR (immediate)
                                let mut need_secure = false;
                                let min_el;
                                match (op1[2], op1[1], op1[0]) {
                                    (false, false, _) => min_el = EL::EL1,
                                    (false, true, false) => min_el = EL::EL1,
                                    (false, true, true) => min_el = EL::EL0,
                                    (true, false, false) => min_el = EL::EL2,
                                    (true, false, true) => {
                                        if !self.is_feat_impl(Feat::VHE) {
                                            return None;
                                        }
                                        min_el = EL::EL2;
                                    }
                                    (true, true, false) => {
                                        min_el = EL::EL3;
                                    }
                                    (true, true, true) => {
                                        min_el = EL::EL1;
                                        need_secure = true;
                                    }
                                }
                                if self.curr_el().as_num() < min_el.as_num()
                                    || (need_secure
                                        && self.current_security_state() != SecurityState::Secure)
                                {
                                    return None;
                                }
                                let operand = crm;
                                let field;
                                match (op1[2], op1[1], op1[0], op2[2], op2[1], op2[0]) {
                                    (false, false, false, false, true, true) => {
                                        if !self.is_feat_impl(Feat::UAO) {
                                            return None;
                                        }
                                        field = PSTATEField::UAO;
                                    }
                                    (false, false, false, true, false, false) => {
                                        if !self.is_feat_impl(Feat::PAN) {
                                            return None;
                                        }
                                        field = PSTATEField::PAN;
                                    }
                                    (false, false, false, true, false, true) => {
                                        field = PSTATEField::SP;
                                    }
                                    (false, false, true, false, false, false) => {
                                        if (crm[3], crm[2], crm[1]) == (false, false, false) {
                                            if !self.is_feat_impl(Feat::NMI) {
                                                return None;
                                            }
                                            field = PSTATEField::ALLINT;
                                        } else if (crm[3], crm[2], crm[1]) == (false, false, true) {
                                            if !self.is_feat_impl(Feat::EBEP) {
                                                return None;
                                            }
                                            field = PSTATEField::PM;
                                        } else {
                                            return None;
                                        }
                                    }
                                    (false, true, true, false, true, false) => {
                                        if !self.is_feat_impl(Feat::DIT) {
                                            return None;
                                        }
                                        field = PSTATEField::DIT;
                                    }
                                    (false, true, true, false, true, true) => {
                                        if (crm[3], crm[2], crm[1]) == (false, false, true) {
                                            if !self.is_feat_impl(Feat::SME) {
                                                return None;
                                            }
                                            field = PSTATEField::SVCRSM;
                                        } else if (crm[3], crm[2], crm[1]) == (false, true, false) {
                                            if !self.is_feat_impl(Feat::SME) {
                                                return None;
                                            }
                                            field = PSTATEField::SVCRZA;
                                        } else if (crm[3], crm[2], crm[1]) == (false, true, true) {
                                            if !self.is_feat_impl(Feat::SME) {
                                                return None;
                                            }
                                            field = PSTATEField::SVCRSMZA;
                                        } else {
                                            return None;
                                        }
                                    }
                                    (false, true, true, true, false, false) => {
                                        if !self.is_feat_impl(Feat::MTE) {
                                            return None;
                                        }
                                        field = PSTATEField::TCO;
                                    }
                                    (false, true, true, true, true, false) => {
                                        field = PSTATEField::DAIFSet;
                                    }
                                    (false, true, true, true, true, true) => {
                                        field = PSTATEField::DAIFClr;
                                    }
                                    (false, true, true, false, false, true) => {
                                        if !self.is_feat_impl(Feat::SSBS) {
                                            return None;
                                        }
                                        field = PSTATEField::SSBS;
                                    }
                                    _ => return None,
                                };
                                Some(branch_excp_sys::pstate::msr(ir_ctx, operand, field, insn))
                            }
                            _ => None, // Unallocated
                        }
                    }
                    // System with result
                    ((true, true, false), (false, true, false, false, true, false, false, ..)) => {
                        let op1 = &bits[16..19];
                        let crn = &bits[12..16];
                        let crm = &bits[8..12];
                        let op2 = &bits[5..8];
                        let rt = &bits[0..5];
                        match (
                            op1.load::<u8>(),
                            crn.load::<u8>(),
                            crm.load::<u8>(),
                            op2.load::<u8>(),
                        ) {
                            (0b011, 0b0011, 0b0000, 0b011) => {
                                // TSTART
                                if !self.is_feat_impl(Feat::TME) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            (0b011, 0b0011, 0b0001, 0b011) => {
                                // TTEST
                                if !self.is_feat_impl(Feat::TME) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            _ => None, // Unallocated
                        }
                    }
                    // System instructions
                    ((true, true, false), (false, true, false, false, _, false, true, ..)) => {
                        let l = bits[21];
                        let op1 = &bits[16..19];
                        let crn = &bits[12..16];
                        let crm = &bits[8..12];
                        let op2 = &bits[5..8];
                        let rt = &bits[0..5];
                        match l {
                            false => Some(branch_excp_sys::sys_insns::sys(
                                ir_ctx, op1, crn, crm, op2, rt,
                            )),
                            true => todo!(), // SYSL
                        }
                    }
                    // System register move
                    ((true, true, false), (false, true, false, false, _, true, ..)) => {
                        let l = bits[21];
                        let o0 = bits[19];
                        let op1 = &bits[16..19];
                        let crn = &bits[12..16];
                        let crm = &bits[8..12];
                        let op2 = &bits[5..8];
                        let rt = &bits[0..5];
                        match l {
                            false => Some(branch_excp_sys::sys_reg_move::msr(ir_ctx, insn, rt)),
                            true => Some(branch_excp_sys::sys_reg_move::mrs(ir_ctx, insn, rt)),
                        }
                    }
                    // System pair instructions
                    ((true, true, false), (false, true, false, true, _, false, true, ..)) => {
                        let l = bits[21];
                        let op1 = &bits[16..19];
                        let crn = &bits[12..16];
                        let crm = &bits[8..12];
                        let op2 = &bits[5..8];
                        let rt = &bits[0..5];
                        match l {
                            false => {
                                if !self.is_feat_impl(Feat::SYSINSTR128) {
                                    None
                                } else {
                                    // SYSP
                                    todo!()
                                }
                            }
                            true => None, // Unallocated
                        }
                    }
                    // System register pair move
                    ((true, true, false), (false, true, false, true, _, true, ..)) => {
                        let l = bits[21];
                        let o0 = bits[19];
                        let op1 = &bits[16..19];
                        let crn = &bits[12..16];
                        let crm = &bits[8..12];
                        let op2 = &bits[5..8];
                        let rt = &bits[0..5];
                        match l {
                            false => {
                                if !self.is_feat_impl(Feat::SYSINSTR128) {
                                    None
                                } else {
                                    // MSRR
                                    todo!()
                                }
                            }
                            true => {
                                if !self.is_feat_impl(Feat::SYSINSTR128) {
                                    None
                                } else {
                                    // MRRS
                                    todo!()
                                }
                            }
                        }
                    }
                    // Unconditional branch (register)
                    ((true, true, false), (true, ..)) => {
                        let opc = &bits[21..25];
                        let op2 = &bits[16..21];
                        let op3 = &bits[10..16];
                        let rn = &bits[5..10];
                        let op4 = &bits[0..5];
                        match (
                            opc.load::<u8>(),
                            op2.load::<u8>(),
                            op3.load::<u8>(),
                            rn.load::<u8>(),
                            op4.load::<u8>(),
                        ) {
                            // BR
                            (0b0000, 0b11111, 0b000000, _, 0b00000) => {
                                Some(branch_excp_sys::uncond_br_reg::br(ir_ctx, rn, false))
                            }
                            // BRAA, BRAAZ, BRAB, BRABZ — Key A, zero modifier
                            (0b0000, 0b11111, 0b000010, _, 0b11111) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // BRAA, BRAAZ, BRAB, BRABZ — Key B, zero modifier
                            (0b0000, 0b11111, 0b000011, _, 0b11111) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // BLR
                            (0b0001, 0b11111, 0b000000, _, 0b00000) => {
                                Some(branch_excp_sys::uncond_br_reg::blr(
                                    ir_ctx,
                                    rn,
                                    self.is_feat_impl(Feat::GCS),
                                ))
                            }
                            // BLRAA, BLRAAZ, BLRAB, BLRABZ — Key A, zero modifier
                            (0b0001, 0b11111, 0b000010, _, 0b11111) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // BLRAA, BLRAAZ, BLRAB, BLRABZ — Key B, zero modifier
                            (0b0001, 0b11111, 0b000011, _, 0b11111) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // RET
                            (0b0010, 0b11111, 0b000000, _, 0b00000) => {
                                Some(branch_excp_sys::uncond_br_reg::ret(
                                    ir_ctx,
                                    rn,
                                    self.is_feat_impl(Feat::GCS),
                                ))
                            }
                            // RETAA, RETAB — RETAA
                            (0b0010, 0b11111, 0b000010, 0b11111, 0b11111) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // RETAASPPC, RETABSPPC (register) — RETAASPPC
                            (0b0010, 0b11111, 0b000010, 0b11111, _) => {
                                if !self.is_feat_impl(Feat::PAuthLR) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // RETAA, RETAB — RETAB
                            (0b0010, 0b11111, 0b000011, 0b11111, 0b11111) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // RETAASPPC, RETABSPPC (register) — RETABSPPC
                            (0b0010, 0b11111, 0b000011, 0b11111, _) => {
                                if !self.is_feat_impl(Feat::PAuthLR) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // ERET
                            (0b0100, 0b11111, 0b000000, 0b11111, 0b00000) => {
                                Some(branch_excp_sys::uncond_br_reg::eret(ir_ctx, self.curr_el()))
                            }
                            // ERETAA, ERETAB — ERETAA
                            (0b0100, 0b11111, 0b000010, 0b11111, 0b11111) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // ERETAA, ERETAB — ERETAB
                            (0b0100, 0b11111, 0b000011, 0b11111, 0b11111) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // DRPS
                            (0b0101, 0b11111, 0b000000, 0b11111, 0b00000) => todo!(),
                            // BRAA, BRAAZ, BRAB, BRABZ — Key A, register modifier
                            (0b1000, 0b11111, 0b000010, _, _) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // BRAA, BRAAZ, BRAB, BRABZ — Key B, register modifier
                            (0b1000, 0b11111, 0b000011, _, _) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // BLRAA, BLRAAZ, BLRAB, BLRABZ — Key A, register modifier
                            (0b1001, 0b11111, 0b000010, _, _) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            // BLRAA, BLRAAZ, BLRAB, BLRABZ — Key B, register modifier
                            (0b1001, 0b11111, 0b000011, _, _) => {
                                if !self.is_feat_impl(Feat::PAuth) {
                                    None
                                } else {
                                    todo!()
                                }
                            }
                            _ => None, // Unallocated
                        }
                    }
                    // Unconditional branch (immediate)
                    ((_, false, false), _) => {
                        let op = bits[31];
                        let imm26 = &bits[0..26];
                        match op {
                            false => Some(branch_excp_sys::uncond_br_imm::b(ir_ctx, imm26)),
                            true => Some(branch_excp_sys::uncond_br_imm::bl(
                                ir_ctx,
                                imm26,
                                self.is_feat_impl(Feat::GCS),
                            )),
                        }
                    }
                    // Compare and branch (immediate)
                    ((_, false, true), (false, ..)) => {
                        let sf = bits[31];
                        let op = bits[24];
                        let imm19 = &bits[5..24];
                        let rt = &bits[0..5];
                        match (sf, op) {
                            (false, false) => {
                                Some(branch_excp_sys::cmp_br_imm::cbz32(ir_ctx, imm19, rt))
                            }
                            (false, true) => {
                                Some(branch_excp_sys::cmp_br_imm::cbnz32(ir_ctx, imm19, rt))
                            }
                            (true, false) => {
                                Some(branch_excp_sys::cmp_br_imm::cbz64(ir_ctx, imm19, rt))
                            }
                            (true, true) => {
                                Some(branch_excp_sys::cmp_br_imm::cbnz64(ir_ctx, imm19, rt))
                            }
                        }
                    }
                    // Test and branch (immediate)
                    ((_, false, true), (true, ..)) => {
                        let b5 = bits[31];
                        let op = bits[24];
                        let b40 = &bits[19..24];
                        let imm14 = &bits[5..19];
                        let rt = &bits[0..5];
                        match (b5, op) {
                            (false, false) => Some(branch_excp_sys::tst_br_imm::tbz32(
                                ir_ctx, b5, b40, imm14, rt,
                            )),
                            (false, true) => Some(branch_excp_sys::tst_br_imm::tbnz32(
                                ir_ctx, b5, b40, imm14, rt,
                            )),
                            (true, false) => Some(branch_excp_sys::tst_br_imm::tbz64(
                                ir_ctx, b5, b40, imm14, rt,
                            )),
                            (true, true) => Some(branch_excp_sys::tst_br_imm::tbnz64(
                                ir_ctx, b5, b40, imm14, rt,
                            )),
                        }
                    }
                    // Unallocated
                    ((_, true, true), _) => None,
                    // Unspecified by the architecture
                    _ => None,
                }
            }
            // Data Proc - Register
            (_, (_, true, false, true)) => {
                let op0 = bits[30];
                let op1 = bits[28];
                let op2 = &bits[21..25];
                let op3 = &bits[10..16];
                match (
                    op0,
                    op1,
                    (op2[3], op2[2], op2[1], op2[0]),
                    (op3[5], op3[4], op3[3], op3[2], op3[1], op3[0]),
                ) {
                    // Data-processing (2 source)
                    (false, true, (false, true, true, false), _) => {
                        let sf = bits[31];
                        let s = bits[29];
                        let rm = &bits[16..21];
                        let opcode = &bits[10..16];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (
                            sf,
                            s,
                            (
                                opcode[5], opcode[4], opcode[3], opcode[2], opcode[1], opcode[0],
                            ),
                        ) {
                            (_, _, (true, ..)) => None,                                 // Unallocated
                            (_, false, (false, false, false, true, true, _)) => None, // Unallocated
                            (false, false, (false, _, true, true, _, _)) => None,     // Unallocated
                            (false, false, (false, false, false, _, false, _)) => None, // Unallocated
                            (false, false, (false, false, false, false, true, false)) => {
                                Some(data_proc_reg::two_source::udiv32(ir_ctx, rm, rn, rd))
                            }
                            (false, false, (false, false, false, false, true, true)) => {
                                Some(data_proc_reg::two_source::sdiv32(ir_ctx, rm, rn, rd))
                            }
                            (false, false, (false, false, true, false, false, false)) => {
                                Some(data_proc_reg::two_source::lslv32(ir_ctx, rm, rn, rd))
                            }
                            (false, false, (false, false, true, false, false, true)) => {
                                Some(data_proc_reg::two_source::lsrv32(ir_ctx, rm, rn, rd))
                            }
                            (false, false, (false, false, true, false, true, false)) => {
                                Some(data_proc_reg::two_source::asrv32(ir_ctx, rm, rn, rd))
                            }
                            (false, false, (false, false, true, false, true, true)) => {
                                Some(data_proc_reg::two_source::rorv32(ir_ctx, rm, rn, rd))
                            }
                            (false, false, (false, true, false, _, true, true)) => None, // Unallocated
                            (false, false, (false, true, false, false, false, false)) => {
                                if !self.is_feat_impl(Feat::CRC32) {
                                    None
                                } else {
                                    todo!("CRC32B")
                                }
                            }
                            (false, false, (false, true, false, false, false, true)) => {
                                if !self.is_feat_impl(Feat::CRC32) {
                                    None
                                } else {
                                    todo!("CRC32H")
                                }
                            }
                            (false, false, (false, true, false, false, true, false)) => {
                                if !self.is_feat_impl(Feat::CRC32) {
                                    None
                                } else {
                                    todo!("CRC32W")
                                }
                            }
                            (false, false, (false, true, false, true, false, false)) => {
                                if !self.is_feat_impl(Feat::CRC32) {
                                    None
                                } else {
                                    todo!("CRC32CB")
                                }
                            }
                            (false, false, (false, true, false, true, false, true)) => {
                                if !self.is_feat_impl(Feat::CRC32) {
                                    None
                                } else {
                                    todo!("CRC32CH")
                                }
                            }
                            (false, false, (false, true, false, true, true, false)) => {
                                if !self.is_feat_impl(Feat::CRC32) {
                                    None
                                } else {
                                    todo!("CRC32CW")
                                }
                            }
                            (false, false, (false, true, true, false, hi, lo)) => {
                                if !self.is_feat_impl(Feat::CSSC) {
                                    None
                                } else {
                                    match (hi, lo) {
                                        (false, false) => Some(data_proc_reg::two_source::smax32(
                                            ir_ctx, rm, rn, rd,
                                        )),
                                        (false, true) => Some(data_proc_reg::two_source::umax32(
                                            ir_ctx, rm, rn, rd,
                                        )),
                                        (true, false) => Some(data_proc_reg::two_source::smin32(
                                            ir_ctx, rm, rn, rd,
                                        )),
                                        (true, true) => Some(data_proc_reg::two_source::umin32(
                                            ir_ctx, rm, rn, rd,
                                        )),
                                    }
                                }
                            }
                            (false, true, (false, ..)) => None, // Unallocated
                            (true, _, (false, false, false, false, false, true)) => None, // Unallocated
                            (true, _, (false, true, false, false, false, _)) => None, // Unallocated
                            (true, false, (false, false, false, false, false, false)) => {
                                if !self.is_feat_impl(Feat::MTE) {
                                    None
                                } else {
                                    Some(data_proc_reg::two_source::subp(ir_ctx, rm, rn, rd))
                                }
                            }
                            (true, false, (false, false, false, false, true, false)) => {
                                Some(data_proc_reg::two_source::udiv64(ir_ctx, rm, rn, rd))
                            }
                            (true, false, (false, false, false, false, true, true)) => {
                                Some(data_proc_reg::two_source::sdiv64(ir_ctx, rm, rn, rd))
                            }
                            (true, false, (false, false, false, true, false, false)) => {
                                if !self.is_feat_impl(Feat::MTE) {
                                    None
                                } else {
                                    Some(data_proc_reg::two_source::irg(ir_ctx, rm, rn, rd))
                                }
                            }
                            (true, false, (false, false, false, true, false, true)) => {
                                if !self.is_feat_impl(Feat::MTE) {
                                    None
                                } else {
                                    Some(data_proc_reg::two_source::gmi(ir_ctx, rm, rn, rd))
                                }
                            }
                            (true, false, (false, false, true, false, false, false)) => {
                                Some(data_proc_reg::two_source::lslv64(ir_ctx, rm, rn, rd))
                            }
                            (true, false, (false, false, true, false, false, true)) => {
                                Some(data_proc_reg::two_source::lsrv64(ir_ctx, rm, rn, rd))
                            }

                            (true, false, (false, false, true, false, true, false)) => {
                                Some(data_proc_reg::two_source::asrv64(ir_ctx, rm, rn, rd))
                            }

                            (true, false, (false, false, true, false, true, true)) => {
                                Some(data_proc_reg::two_source::rorv64(ir_ctx, rm, rn, rd))
                            }

                            (true, false, (false, false, true, true, false, false)) => {
                                if !self.is_feat_impl(Feat::PAuthLR) {
                                    None
                                } else {
                                    Some(data_proc_reg::two_source::pacga(ir_ctx, rm, rn, rd))
                                }
                            }
                            (true, false, (false, false, true, true, false, true)) => None, // Unallocated
                            (true, false, (false, false, true, true, true, _)) => None, // Unallocated
                            (true, false, (false, true, false, _, true, false)) => None, // Unallocated
                            (true, false, (false, true, false, false, true, true)) => {
                                if !self.is_feat_impl(Feat::CRC32) {
                                    None
                                } else {
                                    todo!("CRC32X")
                                }
                            }
                            (true, false, (false, true, false, true, false, _)) => None, // Unallocated
                            (true, false, (false, true, false, true, true, true)) => {
                                if !self.is_feat_impl(Feat::CRC32) {
                                    None
                                } else {
                                    todo!("CRC32CX")
                                }
                            }
                            (true, false, (false, true, true, false, hi, lo)) => {
                                if !self.is_feat_impl(Feat::CSSC) {
                                    None
                                } else {
                                    match (hi, lo) {
                                        (false, false) => Some(data_proc_reg::two_source::smax64(
                                            ir_ctx, rm, rn, rd,
                                        )),
                                        (false, true) => Some(data_proc_reg::two_source::umax64(
                                            ir_ctx, rm, rn, rd,
                                        )),
                                        (true, false) => Some(data_proc_reg::two_source::smin64(
                                            ir_ctx, rm, rn, rd,
                                        )),
                                        (true, true) => Some(data_proc_reg::two_source::umin64(
                                            ir_ctx, rm, rn, rd,
                                        )),
                                    }
                                }
                            }
                            (true, false, (false, true, true, true, _, _)) => None, // Unallocated
                            (true, true, (false, _, false, false, true, _)) => None, // Unallocated
                            (true, true, (false, _, false, true, _, _)) => None,    // Unallocated
                            (true, true, (false, _, true, _, _, _)) => None,        // Unallocated
                            (true, true, (false, false, false, false, false, false)) => {
                                if !self.is_feat_impl(Feat::MTE) {
                                    None
                                } else {
                                    Some(data_proc_reg::two_source::subps(ir_ctx, rm, rn, rd))
                                }
                            }
                        }
                    }
                    // Data-processing (1 source)
                    (true, true, (false, true, true, false), _) => {
                        let sf = bits[31];
                        let s = bits[29];
                        let opcode2 = &bits[16..21];
                        let opcode = &bits[10..16];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (
                            sf,
                            s,
                            (opcode2[4], opcode2[3], opcode2[2], opcode2[1], opcode2[0]),
                            (
                                opcode[5], opcode[4], opcode[3], opcode[2], opcode[1], opcode[0],
                            ),
                        ) {
                            (
                                _,
                                false,
                                (false, false, false, false, false),
                                (false, false, true, false, false, true),
                            ) => None, // Unallocated
                            (
                                _,
                                false,
                                (false, false, false, false, false),
                                (false, false, true, false, true, _),
                            ) => None, // Unallocated
                            (
                                _,
                                false,
                                (false, false, false, false, false),
                                (false, false, true, true, ..),
                            ) => None, // Unallocated
                            (_, false, (false, false, false, false, false), (false, true, ..)) => {
                                None
                            } // Unallocated
                            (_, false, (false, false, false, false, false), (true, ..)) => None, // Unallocated
                            (_, false, (false, false, false, true, _), ..) => None, // Unallocated
                            (_, false, (false, false, true, ..), ..) => None,       // Unallocated
                            (_, false, (false, true, ..), ..) => None,              // Unallocated
                            (_, false, (true, ..), ..) => None,                     // Unallocated
                            (_, true, ..) => None,                                  // Unallocated
                            (
                                false,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, false, false, false),
                            ) => Some(data_proc_reg::one_source::rbit32(ir_ctx, rn, rd)),
                            (
                                false,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, false, false, true),
                            ) => Some(data_proc_reg::one_source::rev1632(ir_ctx, rn, rd)),
                            (
                                false,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, false, true, false),
                            ) => Some(data_proc_reg::one_source::rev32(ir_ctx, rn, rd)),
                            (
                                false,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, false, true, true),
                            ) => None, // Unallocated
                            (
                                false,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, true, false, false),
                            ) => Some(data_proc_reg::one_source::clz32(ir_ctx, rn, rd)),
                            (
                                false,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, true, false, true),
                            ) => Some(data_proc_reg::one_source::cls32(ir_ctx, rn, rd)),
                            (
                                false,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, true, true, b),
                            ) => {
                                if !self.is_feat_impl(Feat::CSSC) {
                                    None
                                } else {
                                    match b {
                                        false => {
                                            Some(data_proc_reg::one_source::ctz32(ir_ctx, rn, rd))
                                        }
                                        true => {
                                            Some(data_proc_reg::one_source::cnt32(ir_ctx, rn, rd))
                                        }
                                    }
                                }
                            }
                            (
                                false,
                                false,
                                (false, false, false, false, false),
                                (false, false, true, false, false, false),
                            ) => {
                                if !self.is_feat_impl(Feat::CSSC) {
                                    None
                                } else {
                                    Some(data_proc_reg::one_source::abs32(ir_ctx, rn, rd))
                                }
                            }
                            (false, false, (false, false, false, false, true), ..) => None, // Unallocated
                            (
                                true,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, false, false, false),
                            ) => Some(data_proc_reg::one_source::rbit64(ir_ctx, rn, rd)),
                            (
                                true,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, false, false, true),
                            ) => Some(data_proc_reg::one_source::rev1664(ir_ctx, rn, rd)),
                            (
                                true,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, false, true, false),
                            ) => Some(data_proc_reg::one_source::rev3264(ir_ctx, rn, rd)),
                            (
                                true,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, false, true, true),
                            ) => Some(data_proc_reg::one_source::rev64(ir_ctx, rn, rd)),
                            (
                                true,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, true, false, false),
                            ) => Some(data_proc_reg::one_source::clz64(ir_ctx, rn, rd)),
                            (
                                true,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, true, false, true),
                            ) => Some(data_proc_reg::one_source::cls64(ir_ctx, rn, rd)),
                            (
                                true,
                                false,
                                (false, false, false, false, false),
                                (false, false, false, true, true, b),
                            ) => {
                                if !self.is_feat_impl(Feat::CSSC) {
                                    None
                                } else {
                                    match b {
                                        false => {
                                            Some(data_proc_reg::one_source::ctz64(ir_ctx, rn, rd))
                                        }
                                        true => {
                                            Some(data_proc_reg::one_source::cnt64(ir_ctx, rn, rd))
                                        }
                                    }
                                }
                            }
                            (
                                true,
                                false,
                                (false, false, false, false, false),
                                (false, false, true, false, false, false),
                            ) => {
                                if !self.is_feat_impl(Feat::CSSC) {
                                    None
                                } else {
                                    Some(data_proc_reg::one_source::abs64(ir_ctx, rn, rd))
                                }
                            }
                            (true, false, (false, false, false, false, true), ..) => {
                                todo!("PAC support")
                            }
                        }
                    }
                    // Logical (shifted register)
                    (_, false, (false, _, _, _), _) => {
                        let sf = bits[31];
                        let opc = &bits[29..31];
                        let shift = &bits[22..24];
                        let n = bits[21];
                        let rm = &bits[16..21];
                        let imm6 = &bits[10..16];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, (opc[1], opc[0]), n) {
                            (false, (false, false), false) => Some(data_proc_reg::logical::and32(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (false, (false, false), true) => Some(data_proc_reg::logical::bic32(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (false, (false, true), false) => Some(data_proc_reg::logical::orr32(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (false, (false, true), true) => Some(data_proc_reg::logical::orn32(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (false, (true, false), false) => Some(data_proc_reg::logical::eor32(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (false, (true, false), true) => Some(data_proc_reg::logical::eon32(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (false, (true, true), false) => Some(data_proc_reg::logical::ands32(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (false, (true, true), true) => Some(data_proc_reg::logical::bics32(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (true, (false, false), false) => Some(data_proc_reg::logical::and64(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (true, (false, false), true) => Some(data_proc_reg::logical::bic64(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (true, (false, true), false) => Some(data_proc_reg::logical::orr64(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (true, (false, true), true) => Some(data_proc_reg::logical::orn64(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (true, (true, false), false) => Some(data_proc_reg::logical::eor64(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (true, (true, false), true) => Some(data_proc_reg::logical::eon64(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (true, (true, true), false) => Some(data_proc_reg::logical::ands64(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                            (true, (true, true), true) => Some(data_proc_reg::logical::bics64(
                                ir_ctx, shift, &rm, imm6, rn, rd,
                            )),
                        }
                    }
                    // Add/subtract (shifted register)
                    (_, false, (true, _, _, false), _) => {
                        let sf = bits[31];
                        let opc = bits[30];
                        let s = bits[29];
                        let shift = &bits[22..24];
                        let rm = &bits[16..21];
                        let imm6 = &bits[10..16];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, opc, s) {
                            (false, false, false) => Some(data_proc_reg::add_sub_shifted::add32(
                                ir_ctx, shift, rm, imm6, rn, rd,
                            )),
                            (false, false, true) => Some(data_proc_reg::add_sub_shifted::adds32(
                                ir_ctx, shift, rm, imm6, rn, rd,
                            )),
                            (false, true, false) => Some(data_proc_reg::add_sub_shifted::sub32(
                                ir_ctx, shift, rm, imm6, rn, rd,
                            )),
                            (false, true, true) => Some(data_proc_reg::add_sub_shifted::subs32(
                                ir_ctx, shift, rm, imm6, rn, rd,
                            )),
                            (true, false, false) => Some(data_proc_reg::add_sub_shifted::add64(
                                ir_ctx, shift, rm, imm6, rn, rd,
                            )),
                            (true, false, true) => Some(data_proc_reg::add_sub_shifted::adds64(
                                ir_ctx, shift, rm, imm6, rn, rd,
                            )),
                            (true, true, false) => Some(data_proc_reg::add_sub_shifted::sub64(
                                ir_ctx, shift, rm, imm6, rn, rd,
                            )),
                            (true, true, true) => Some(data_proc_reg::add_sub_shifted::subs64(
                                ir_ctx, shift, rm, imm6, rn, rd,
                            )),
                        }
                    }
                    // Add/subtract (extended register)
                    (_, false, (true, _, _, true), _) => {
                        let sf = bits[31];
                        let opc = bits[30];
                        let s = bits[29];
                        let opt = &bits[22..24];
                        let rm = &bits[16..21];
                        let option = &bits[13..16];
                        let imm3 = &bits[10..13];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, opc, s, (opt[1], opt[0])) {
                            (_, _, _, (false, true)) => None, // Unallocated
                            (_, _, _, (true, _)) => None,     // Unallocated
                            (false, false, false, (false, false)) => {
                                Some(data_proc_reg::add_sub_extended::add32(
                                    ir_ctx,
                                    option,
                                    rm,
                                    imm3,
                                    rn,
                                    rd,
                                    self.curr_el(),
                                    self.read::<pstate::SP>() != 0,
                                ))
                            }
                            (false, false, true, (false, false)) => {
                                Some(data_proc_reg::add_sub_extended::adds32(
                                    ir_ctx,
                                    option,
                                    rm,
                                    imm3,
                                    rn,
                                    rd,
                                    self.curr_el(),
                                    self.read::<pstate::SP>() != 0,
                                ))
                            }
                            (false, true, false, (false, false)) => {
                                Some(data_proc_reg::add_sub_extended::sub32(
                                    ir_ctx,
                                    option,
                                    rm,
                                    imm3,
                                    rn,
                                    rd,
                                    self.curr_el(),
                                    self.read::<pstate::SP>() != 0,
                                ))
                            }
                            (false, true, true, (false, false)) => {
                                Some(data_proc_reg::add_sub_extended::subs32(
                                    ir_ctx,
                                    option,
                                    rm,
                                    imm3,
                                    rn,
                                    rd,
                                    self.curr_el(),
                                    self.read::<pstate::SP>() != 0,
                                ))
                            }
                            (true, false, false, (false, false)) => {
                                Some(data_proc_reg::add_sub_extended::add64(
                                    ir_ctx,
                                    option,
                                    rm,
                                    imm3,
                                    rn,
                                    rd,
                                    self.curr_el(),
                                    self.read::<pstate::SP>() != 0,
                                ))
                            }
                            (true, false, true, (false, false)) => {
                                Some(data_proc_reg::add_sub_extended::adds64(
                                    ir_ctx,
                                    option,
                                    rm,
                                    imm3,
                                    rn,
                                    rd,
                                    self.curr_el(),
                                    self.read::<pstate::SP>() != 0,
                                ))
                            }
                            (true, true, false, (false, false)) => {
                                Some(data_proc_reg::add_sub_extended::sub64(
                                    ir_ctx,
                                    option,
                                    rm,
                                    imm3,
                                    rn,
                                    rd,
                                    self.curr_el(),
                                    self.read::<pstate::SP>() != 0,
                                ))
                            }
                            (true, true, true, (false, false)) => {
                                Some(data_proc_reg::add_sub_extended::subs64(
                                    ir_ctx,
                                    option,
                                    rm,
                                    imm3,
                                    rn,
                                    rd,
                                    self.curr_el(),
                                    self.read::<pstate::SP>() != 0,
                                ))
                            }
                        }
                    }
                    // Add/subtract (with carry)
                    (
                        _,
                        true,
                        (false, false, false, false),
                        (false, false, false, false, false, false),
                    ) => {
                        let sf = bits[31];
                        let opc = bits[30];
                        let s = bits[29];
                        let rm = &bits[16..21];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, opc, s) {
                            (false, false, false) => {
                                Some(data_proc_reg::add_sub_carry::adc32(ir_ctx, rm, rn, rd))
                            }
                            (false, false, true) => {
                                Some(data_proc_reg::add_sub_carry::adcs32(ir_ctx, rm, rn, rd))
                            }
                            (false, true, false) => {
                                Some(data_proc_reg::add_sub_carry::sbc32(ir_ctx, rm, rn, rd))
                            }
                            (false, true, true) => {
                                Some(data_proc_reg::add_sub_carry::sbcs32(ir_ctx, rm, rn, rd))
                            }
                            (true, false, false) => {
                                Some(data_proc_reg::add_sub_carry::adc64(ir_ctx, rm, rn, rd))
                            }
                            (true, false, true) => {
                                Some(data_proc_reg::add_sub_carry::adcs64(ir_ctx, rm, rn, rd))
                            }
                            (true, true, false) => {
                                Some(data_proc_reg::add_sub_carry::sbc64(ir_ctx, rm, rn, rd))
                            }
                            (true, true, true) => {
                                Some(data_proc_reg::add_sub_carry::sbcs64(ir_ctx, rm, rn, rd))
                            }
                        }
                    }
                    // Add/subtract (checked pointer)
                    (_, true, (false, false, false, false), (false, false, true, _, _, _)) => {
                        if !self.is_feat_impl(Feat::CPA) {
                            return None;
                        }
                        let sf = bits[31];
                        let op = bits[30];
                        let s = bits[29];
                        let rm = &bits[16..21];
                        let imm3 = &bits[10..13];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, op, s) {
                            (false, _, _) => None,   // Unallocated
                            (true, _, true) => None, // Unallocated
                            (true, false, false) => Some(data_proc_reg::add_sub_checkedptr::addpt(
                                ir_ctx, rm, imm3, rn, rd,
                            )),
                            (true, true, false) => Some(data_proc_reg::add_sub_checkedptr::subpt(
                                ir_ctx, rm, imm3, rn, rd,
                            )),
                        }
                    }
                    // Unallocated
                    (_, true, (false, false, false, false), (false, true, true, _, _, _)) => None,
                    // Unallocated
                    (
                        _,
                        true,
                        (false, false, false, false),
                        (true, false, false, false, false, false),
                    ) => None,
                    // Unallocated
                    (_, true, (false, false, false, false), (true, _, true, _, _, _)) => None,
                    // Rotate right into flags
                    (
                        _,
                        true,
                        (false, false, false, false),
                        (_, false, false, false, false, true),
                    ) => {
                        if !self.is_feat_impl(Feat::FlagM) {
                            return None;
                        }
                        let sf = bits[31];
                        let op = bits[30];
                        let s = bits[29];
                        let imm6 = &bits[15..21];
                        let rn = &bits[5..10];
                        let o2 = bits[4];
                        let mask = &bits[0..4];
                        match (sf, op, s, o2) {
                            (false, _, _, _) => None,        // Unallocated
                            (true, false, false, _) => None, // Unallocated
                            (true, false, true, false) => {
                                Some(data_proc_reg::rotate_right_into_flags::rmif(
                                    ir_ctx, mask, rn, imm6,
                                ))
                            }
                            (true, false, true, true) => None, // Unallocated
                            (true, true, _, _) => None,        // Unallocated
                        }
                    }
                    // Unallocated
                    (_, true, (false, false, false, false), (_, false, false, true, false, _)) => {
                        None
                    }
                    // Unallocated
                    (_, true, (false, false, false, false), (_, true, false, _, false, _)) => None,
                    // Evaluate into flags
                    (_, true, (false, false, false, false), (_, _, false, false, true, false)) => {
                        todo!()
                    }
                    // Unallocated
                    (_, true, (false, false, false, false), (_, _, false, false, true, true)) => {
                        None
                    }
                    // Unallocated
                    (_, true, (false, false, false, false), (_, _, false, true, true, _)) => None,
                    // Conditional compare (register)
                    (_, true, (false, false, true, false), (_, _, _, _, false, _)) => {
                        let sf = bits[31];
                        let op = bits[30];
                        let s = bits[29];
                        let rm = &bits[16..21];
                        let cond = &bits[12..16];
                        let o2 = bits[10];
                        let rn = &bits[5..10];
                        let o3 = bits[4];
                        let nzcv = &bits[0..4];
                        match (sf, op, s, o2, o3) {
                            (_, _, false, _, _) => None,       // Unallocated
                            (_, _, true, false, true) => None, // Unallocated
                            (_, _, true, true, _) => None,     // Unallocated
                            (false, false, true, false, false) => Some(
                                data_proc_reg::cond_cmp_reg::ccmn32(ir_ctx, rm, cond, rn, nzcv),
                            ),
                            (false, true, true, false, false) => Some(
                                data_proc_reg::cond_cmp_reg::ccmp32(ir_ctx, rm, cond, rn, nzcv),
                            ),
                            (true, false, true, false, false) => Some(
                                data_proc_reg::cond_cmp_reg::ccmn64(ir_ctx, rm, cond, rn, nzcv),
                            ),
                            (true, true, true, false, false) => Some(
                                data_proc_reg::cond_cmp_reg::ccmp64(ir_ctx, rm, cond, rn, nzcv),
                            ),
                        }
                    }
                    // Conditional compare (immediate)
                    (_, true, (false, false, true, false), (_, _, _, _, true, _)) => {
                        let sf = bits[31];
                        let op = bits[30];
                        let s = bits[29];
                        let imm5 = &bits[16..21];
                        let cond = &bits[12..16];
                        let o2 = bits[10];
                        let rn = &bits[5..10];
                        let o3 = bits[4];
                        let nzcv = &bits[0..4];
                        match (sf, op, s, o2, o3) {
                            (_, _, false, _, _) => None,       // Unallocated
                            (_, _, true, false, true) => None, // Unallocated
                            (_, _, true, true, _) => None,     // Unallocated
                            (false, false, true, false, false) => Some(
                                data_proc_reg::cond_cmp_imm::ccmn32(ir_ctx, imm5, cond, rn, nzcv),
                            ),
                            (false, true, true, false, false) => Some(
                                data_proc_reg::cond_cmp_imm::ccmp32(ir_ctx, imm5, cond, rn, nzcv),
                            ),
                            (true, false, true, false, false) => Some(
                                data_proc_reg::cond_cmp_imm::ccmn64(ir_ctx, imm5, cond, rn, nzcv),
                            ),
                            (true, true, true, false, false) => Some(
                                data_proc_reg::cond_cmp_imm::ccmp64(ir_ctx, imm5, cond, rn, nzcv),
                            ),
                        }
                    }
                    // Conditional select
                    (_, true, (false, true, false, false), _) => {
                        let sf = bits[31];
                        let op = bits[30];
                        let s = bits[29];
                        let rm = &bits[16..21];
                        let cond = &bits[12..16];
                        let op2 = &bits[10..12];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, op, s, (op2[1], op2[0])) {
                            (_, _, false, (true, _)) => None, // Unallocated
                            (_, _, true, _) => None,          // Unallocated
                            (false, false, false, (false, false)) => {
                                Some(data_proc_reg::cond_select::csel32(ir_ctx, rm, cond, rn, rd))
                            }
                            (false, false, false, (false, true)) => Some(
                                data_proc_reg::cond_select::csinc32(ir_ctx, rm, cond, rn, rd),
                            ),
                            (false, true, false, (false, false)) => Some(
                                data_proc_reg::cond_select::csinv32(ir_ctx, rm, cond, rn, rd),
                            ),
                            (false, true, false, (false, true)) => Some(
                                data_proc_reg::cond_select::csneg32(ir_ctx, rm, cond, rn, rd),
                            ),
                            (true, false, false, (false, false)) => {
                                Some(data_proc_reg::cond_select::csel64(ir_ctx, rm, cond, rn, rd))
                            }
                            (true, false, false, (false, true)) => Some(
                                data_proc_reg::cond_select::csinc64(ir_ctx, rm, cond, rn, rd),
                            ),
                            (true, true, false, (false, false)) => Some(
                                data_proc_reg::cond_select::csinv64(ir_ctx, rm, cond, rn, rd),
                            ),
                            (true, true, false, (false, true)) => Some(
                                data_proc_reg::cond_select::csneg64(ir_ctx, rm, cond, rn, rd),
                            ),
                        }
                    }
                    // Unallocated
                    (_, true, (false, _, _, true), _) => None,
                    // Data-processing (3 source)
                    (_, true, (true, _, _, _), _) => {
                        let sf = bits[31];
                        let op54 = &bits[29..31];
                        let op31 = &bits[21..24];
                        let rm = &bits[16..21];
                        let o0 = bits[15];
                        let ra = &bits[10..15];
                        let rn = &bits[5..10];
                        let rd = &bits[0..5];
                        match (sf, (op54[1], op54[0]), (op31[2], op31[1], op31[0]), o0) {
                            (_, (false, false), (true, false, false), _) => None, // Unallocated
                            (_, (false, true), _, _) => None,                     // Unallocated
                            (_, (true, _), _, _) => None,                         // Unallocated
                            (false, (false, false), (_, false, true), _) => None, // Unallocated
                            (false, (false, false), (_, true, _), _) => None,     // Unallocated
                            (false, (false, false), (false, false, false), false) => {
                                Some(data_proc_reg::three_source::madd32(ir_ctx, rm, ra, rn, rd))
                            }
                            (false, (false, false), (false, false, false), true) => {
                                Some(data_proc_reg::three_source::msub32(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (_, true, false), true) => None, // Unallocated
                            (true, (false, false), (false, false, false), false) => {
                                Some(data_proc_reg::three_source::madd64(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (false, false, false), true) => {
                                Some(data_proc_reg::three_source::msub64(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (false, false, true), false) => {
                                Some(data_proc_reg::three_source::smaddl(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (false, false, true), true) => {
                                Some(data_proc_reg::three_source::smsubl(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (false, true, false), false) => {
                                Some(data_proc_reg::three_source::smulh(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (false, true, true), false) => {
                                if !self.is_feat_impl(Feat::CPA) {
                                    None
                                } else {
                                    Some(data_proc_reg::three_source::maddpt(
                                        ir_ctx, rm, ra, rn, rd,
                                    ))
                                }
                            }
                            (true, (false, false), (false, true, true), true) => {
                                if !self.is_feat_impl(Feat::CPA) {
                                    None
                                } else {
                                    Some(data_proc_reg::three_source::msubpt(
                                        ir_ctx, rm, ra, rn, rd,
                                    ))
                                }
                            }
                            (true, (false, false), (true, false, true), false) => {
                                Some(data_proc_reg::three_source::umaddl(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (true, false, true), true) => {
                                Some(data_proc_reg::three_source::umsubl(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (true, true, false), false) => {
                                Some(data_proc_reg::three_source::umulh(ir_ctx, rm, ra, rn, rd))
                            }
                            (true, (false, false), (true, true, true), _) => None, // Unallocated
                        }
                    }
                }
            }
            // Data Proc - Scalar Floating-point and Adv. SIMD
            (_, (_, true, true, true)) => {
                let op0 = &bits[28..32];
                let op1 = &bits[23..25];
                let op2 = &bits[19..23];
                let op3 = &bits[10..19];
                match (
                    (op0[3], op0[2], op0[1], op0[0]),
                    (op1[1], op1[0]),
                    (op2[3], op2[2], op2[1], op2[0]),
                    (
                        op3[8], op3[7], op3[6], op3[5], op3[4], op3[3], op3[2], op3[1], op3[0],
                    ),
                ) {
                    (
                        (false, false, false, false),
                        (false, _),
                        (_, true, false, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => None, // Unallocated
                    (
                        (false, false, true, false),
                        (false, _),
                        (_, true, false, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => None, // Unallocated
                    (
                        (false, true, false, false),
                        (false, _),
                        (_, true, false, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => todo!(), // Cryptographic AES
                    (
                        (false, true, false, true),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, false, _, _, _, false, false),
                    ) => todo!(), // Cryptographic three-register SHA
                    (
                        (false, true, false, true),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, false, _, _, _, true, false),
                    ) => None, // Unallocated
                    (
                        (false, true, false, true),
                        (false, _),
                        (_, true, false, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => todo!(), // Cryptographic two-register SHA
                    (
                        (false, true, true, false),
                        (false, _),
                        (_, true, false, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => None, // Unallocated
                    (
                        (false, true, true, true),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, false, _, _, _, _, false),
                    ) => None, // Unallocated
                    (
                        (false, true, true, true),
                        (false, _),
                        (_, true, false, true),
                        (false, false, _, false, _, _, _, true, false),
                    ) => None, // Unallocated
                    (
                        (false, true, _, true),
                        (false, false),
                        (false, false, _, _),
                        (_, _, _, false, _, _, _, _, true),
                    ) => todo!(), // Advanced SIMD scalar copy
                    (
                        (false, true, _, true),
                        (false, true),
                        (false, false, _, _),
                        (_, _, _, false, _, _, _, _, true),
                    ) => None, // Unallocated
                    (
                        (false, true, _, true),
                        (false, _),
                        (false, true, true, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => None, // Unallocated
                    (
                        (false, true, _, true),
                        (false, _),
                        (true, false, _, _),
                        (_, _, _, false, false, _, _, _, true),
                    ) => todo!(), // Advanced SIMD scalar three same FP16
                    (
                        (false, true, _, true),
                        (false, _),
                        (true, false, _, _),
                        (_, _, _, false, true, _, _, _, true),
                    ) => None, // Unallocated
                    (
                        (false, true, _, true),
                        (false, _),
                        (true, true, true, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => todo!(), // Advanced SIMD scalar two-register miscellaneous FP16
                    (
                        (false, true, _, true),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, true, _, _, _, _, false),
                    ) => None, // Unallocated
                    (
                        (false, true, _, true),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, true, _, _, _, _, true),
                    ) => todo!(), // Advanced SIMD scalar three same extra
                    (
                        (false, true, _, true),
                        (false, _),
                        (_, true, false, false),
                        (false, false, _, _, _, _, _, true, false),
                    ) => todo!(), // Advanced SIMD scalar two-register miscellaneous
                    (
                        (false, true, _, true),
                        (false, _),
                        (_, true, true, false),
                        (false, false, _, _, _, _, _, true, false),
                    ) => todo!(), // Advanced SIMD scalar pairwise
                    (
                        (false, true, _, true),
                        (false, _),
                        (_, true, _, _),
                        (false, true, _, _, _, _, _, true, false),
                    ) => None, // Unallocated
                    (
                        (false, true, _, true),
                        (false, _),
                        (_, true, _, _),
                        (true, _, _, _, _, _, _, true, false),
                    ) => None, // Unallocated
                    (
                        (false, true, _, true),
                        (false, _),
                        (_, true, _, _),
                        (_, _, _, _, _, _, _, false, false),
                    ) => todo!(), // Advanced SIMD scalar three different
                    (
                        (false, true, _, true),
                        (false, _),
                        (_, true, _, _),
                        (_, _, _, _, _, _, _, _, true),
                    ) => todo!(), // Advanced SIMD scalar three same
                    ((false, true, _, true), (true, false), _, (_, _, _, _, _, _, _, _, true)) => {
                        // Advanced SIMD scalar shift by immediate
                        todo!()
                    }
                    ((false, true, _, true), (true, true), _, (_, _, _, _, _, _, _, _, true)) => {
                        // Unallocated
                        None
                    }
                    ((false, true, _, true), (true, _), _, (_, _, _, _, _, _, _, _, false)) => {
                        // Advanced SIMD scalar x indexed element
                        todo!()
                    }
                    (
                        (false, _, false, false),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, false, _, _, _, false, false),
                    ) => {
                        // Advanced SIMD table lookup
                        todo!()
                    }
                    (
                        (false, _, false, false),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, false, _, _, _, true, false),
                    ) => {
                        // Advanced SIMD permute
                        todo!()
                    }
                    (
                        (false, _, true, false),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, false, _, _, _, _, false),
                    ) => {
                        // Advanced SIMD extract
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (false, false),
                        (false, false, _, _),
                        (_, _, _, false, _, _, _, _, true),
                    ) => {
                        // Advanced SIMD copy
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (false, true),
                        (false, false, _, _),
                        (_, _, _, false, _, _, _, _, true),
                    ) => {
                        // Unallocated
                        None
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (false, true, true, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => {
                        // Unallocated
                        None
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (true, false, _, _),
                        (_, _, _, false, false, _, _, _, true),
                    ) => {
                        // Advanced SIMD three same (FP16)
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (true, false, _, _),
                        (_, _, _, false, true, _, _, _, true),
                    ) => {
                        // Unallocated
                        None
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (true, true, true, true),
                        (false, false, _, _, _, _, _, true, false),
                    ) => {
                        // Advanced SIMD two-register miscellaneous (FP16)
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, true, _, _, _, _, false),
                    ) => {
                        // Unallocated
                        None
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (_, false, _, _),
                        (_, _, _, true, _, _, _, _, true),
                    ) => {
                        // Advanced SIMD three-register extension
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (_, true, false, false),
                        (false, false, _, _, _, _, _, true, false),
                    ) => {
                        // Advanced SIMD two-register miscellaneous
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (_, true, true, false),
                        (false, false, _, _, _, _, _, true, false),
                    ) => {
                        // Advanced SIMD across lanes
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (_, true, _, _),
                        (false, true, _, _, _, _, _, true, false),
                    ) => {
                        // Unallocated
                        None
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (_, true, _, _),
                        (true, _, _, _, _, _, _, true, false),
                    ) => {
                        // Unallocated
                        None
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (_, true, _, _),
                        (_, _, _, _, _, _, _, false, false),
                    ) => {
                        // Advanced SIMD three different
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (false, _),
                        (_, true, _, _),
                        (_, _, _, _, _, _, _, _, true),
                    ) => {
                        // Advanced SIMD three same
                        todo!()
                    }
                    (
                        (false, _, _, false),
                        (true, false),
                        (false, false, false, false),
                        (_, _, _, _, _, _, _, _, true),
                    ) => {
                        // Advanced SIMD modified immediate
                        todo!()
                    }
                    ((false, _, _, false), (true, false), _, (_, _, _, _, _, _, _, _, true)) => {
                        // Advanced SIMD shift by immediate
                        todo!()
                    }
                    ((false, _, _, false), (true, true), _, (_, _, _, _, _, _, _, _, true)) => {
                        // Unallocated
                        None
                    }
                    ((false, _, _, false), (true, _), _, (_, _, _, _, _, _, _, _, false)) => {
                        // Advanced SIMD vector x indexed element
                        todo!()
                    }
                    (
                        (true, true, false, false),
                        (false, false),
                        (true, false, _, _),
                        (_, _, _, true, false, _, _, _, _),
                    ) => {
                        // Cryptographic three-register, imm2
                        todo!()
                    }
                    (
                        (true, true, false, false),
                        (false, false),
                        (true, true, _, _),
                        (_, _, _, true, _, false, false, _, _),
                    ) => {
                        // Cryptographic three-register SHA 512
                        todo!()
                    }
                    (
                        (true, true, false, false),
                        (false, false),
                        _,
                        (_, _, _, false, _, _, _, _, _),
                    ) => {
                        // Cryptographic four-register
                        todo!()
                    }
                    ((true, true, false, false), (false, true), (false, false, _, _), _) => {
                        if !self.is_feat_impl(Feat::SHA3) {
                            None
                        } else {
                            // XAR
                            todo!()
                        }
                    }
                    (
                        (true, true, false, false),
                        (false, true),
                        (true, false, false, false),
                        (false, false, false, true, false, false, false, _, _),
                    ) => {
                        // Cryptographic two-register SHA 512
                        todo!()
                    }
                    ((true, _, _, false), (true, _), ..) => {
                        // Unallocated
                        None
                    }
                    ((_, false, _, true), (false, _), (_, false, ..), _) => {
                        // Conversion between floating-point and fixed-point
                        todo!()
                    }
                    (
                        (_, false, _, true),
                        (false, _),
                        (_, true, ..),
                        (_, _, _, false, false, false, false, false, false),
                    ) => {
                        // Conversion between floating-point and integer
                        todo!()
                    }
                    (
                        (_, false, _, true),
                        (false, _),
                        (_, true, ..),
                        (_, _, _, _, true, false, false, false, false),
                    ) => {
                        // Floating-point data-processing (1 source)
                        todo!()
                    }
                    (
                        (_, false, _, true),
                        (false, _),
                        (_, true, ..),
                        (_, _, _, _, _, true, false, false, false),
                    ) => {
                        // Floating-point compare
                        todo!()
                    }
                    (
                        (_, false, _, true),
                        (false, _),
                        (_, true, ..),
                        (_, _, _, _, _, _, true, false, false),
                    ) => {
                        // Floating-point immediate
                        todo!()
                    }
                    (
                        (_, false, _, true),
                        (false, _),
                        (_, true, ..),
                        (_, _, _, _, _, _, _, false, true),
                    ) => {
                        // Floating-point conditional compare
                        todo!()
                    }
                    (
                        (_, false, _, true),
                        (false, _),
                        (_, true, ..),
                        (_, _, _, _, _, _, _, true, false),
                    ) => {
                        // Floating-point data-processing (2 source)
                        todo!()
                    }
                    (
                        (_, false, _, true),
                        (false, _),
                        (_, true, ..),
                        (_, _, _, _, _, _, _, true, true),
                    ) => {
                        // Floating-point conditional select
                        todo!()
                    }
                    ((_, false, _, true), (true, _), ..) => {
                        // Floating-point data-processing (3 source)
                        todo!()
                    }
                    _ => None, // Unspecified by the architecture
                }
            }
            // Loads and Stores
            (_, (_, true, _, false)) => {
                let op0 = &bits[28..32];
                let op1 = bits[26];
                let op2 = &bits[10..25];
                match ((op0[3], op0[2], op0[1], op0[0]), op1, op2) {
                    // Compare and swap pair
                    // op2 = 00x1xxxxxxxxxxx
                    ((false, _, false, false), false, op2)
                        if (op2[14], op2[13], op2[11]) == (false, false, true) =>
                    {
                        todo!()
                    }
                    // Advanced SIMD load/store multiple structures
                    // op2 = 00x000000xxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[13], *(&op2[6..12].load::<u8>())) == (false, false, 0) =>
                    {
                        todo!()
                    }
                    // Unallocated
                    // op2 = 00x000001xxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[13], *(&op2[6..12].load::<u8>())) == (false, false, 1) =>
                    {
                        None
                    }
                    // Advanced SIMD load/store multiple structures (post-indexed)
                    // op2 = 01x0xxxxxxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[13], op2[11]) == (false, true, false) =>
                    {
                        todo!()
                    }
                    // Unallocated
                    // op2 = 0xx1xxxxxxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[11]) == (false, true) =>
                    {
                        None
                    }
                    // Unallocated
                    // op2 = 10x10001xxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[13], op2[11], *(&op2[7..11].load::<u8>()))
                            == (true, false, true, 1) =>
                    {
                        None
                    }
                    // Unallocated
                    // op2 = 10x1001xxxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[13], op2[11], *(&op2[8..11].load::<u8>()))
                            == (true, false, true, 1) =>
                    {
                        None
                    }
                    // Unallocated
                    // op2 = 10x101xxxxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[13], op2[11], *(&op2[9..11].load::<u8>()))
                            == (true, false, true, 1) =>
                    {
                        None
                    }
                    // Unallocated
                    // op2 = 10x11xxxxxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[13], op2[11], op2[10]) == (true, false, true, true) =>
                    {
                        None
                    }
                    // Advanced SIMD load/store single structure
                    // op2 = 10xx0000xxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[13], *(&op2[7..10].load::<u8>())) == (true, false, 0) =>
                    {
                        todo!()
                    }
                    // Advanced SIMD load/store single structure (post-indexed)
                    // op2 = 11xxxxxxxxxxxxx
                    ((false, _, false, false), true, op2) if (op2[14], op2[13]) == (true, true) => {
                        todo!()
                    }
                    // Unallocated
                    // op2 = x0x00001xxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[13], *(&op2[7..12].load::<u8>())) == (false, 1) =>
                    {
                        None
                    }
                    // Unallocated
                    // op2 = x0x0001xxxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[13], *(&op2[8..12].load::<u8>())) == (false, 1) =>
                    {
                        None
                    }
                    // Unallocated
                    // op2 = x0x001xxxxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[13], *(&op2[9..12].load::<u8>())) == (false, 1) =>
                    {
                        None
                    }
                    // Unallocated
                    // op2 = x0x01xxxxxxxxxx
                    ((false, _, false, false), true, op2)
                        if (op2[13], *(&op2[10..12].load::<u8>())) == (false, 1) =>
                    {
                        None
                    }
                    // RCW compare and swap
                    // op2 = 1xx1xxxxx000010
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[11], *(&op2[0..6].load::<u8>())) == (true, true, 0b10) =>
                    {
                        todo!()
                    }
                    // RCW compare and swap pair
                    // op2 = 1xx1xxxxx000011
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[11], *(&op2[0..6].load::<u8>())) == (true, true, 0b11) =>
                    {
                        todo!()
                    }
                    // 128-bit atomic memory operations
                    // op2 = 1xx1xxxxxxxxx00
                    ((false, _, false, false), true, op2)
                        if (op2[14], op2[11], *(&op2[0..2].load::<u8>())) == (true, true, 0) =>
                    {
                        todo!()
                    }
                    // Load/store memory tags
                    ((true, true, false, true), false, op2)
                        if (op2[14], op2[11]) == (true, true) =>
                    {
                        todo!()
                    }
                    // Load/store exclusive pair
                    ((true, _, false, false), false, op2)
                        if (op2[14], op2[13], op2[11]) == (false, false, true) =>
                    {
                        todo!()
                    }
                    ((true, _, false, false), true, _) => None, // Unallocated
                    ((_, _, false, false), false, op2) => match (op2[14], op2[13], op2[11]) {
                        // Load/store exclusive register
                        // 00x0xxxxxxxxxxx
                        (false, false, false) => {
                            let size = &bits[30..32];
                            let l = bits[22];
                            let rs = &bits[16..21];
                            let o0 = bits[15];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((size[1], size[0]), l, o0) {
                                ((false, false), false, false) => {
                                    Some(load_store::ldst_exclusive_reg::stxrb(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, false), false, true) => {
                                    Some(load_store::ldst_exclusive_reg::stlxrb(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, false), true, false) => {
                                    Some(load_store::ldst_exclusive_reg::ldxrb(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, false), true, true) => {
                                    Some(load_store::ldst_exclusive_reg::ldaxrb(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, true), false, false) => {
                                    Some(load_store::ldst_exclusive_reg::stxrh(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, true), false, true) => {
                                    Some(load_store::ldst_exclusive_reg::stlxrh(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, true), true, false) => {
                                    Some(load_store::ldst_exclusive_reg::ldxrh(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, true), true, true) => {
                                    Some(load_store::ldst_exclusive_reg::ldaxrh(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, false), false, false) => {
                                    Some(load_store::ldst_exclusive_reg::stxr32(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, false), false, true) => {
                                    Some(load_store::ldst_exclusive_reg::stlxr32(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, false), true, false) => {
                                    Some(load_store::ldst_exclusive_reg::ldxr32(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, false), true, true) => {
                                    Some(load_store::ldst_exclusive_reg::ldaxr32(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, true), false, false) => {
                                    Some(load_store::ldst_exclusive_reg::stxr64(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, true), false, true) => {
                                    Some(load_store::ldst_exclusive_reg::stlxr64(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, true), true, false) => {
                                    Some(load_store::ldst_exclusive_reg::ldxr64(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, true), true, true) => {
                                    Some(load_store::ldst_exclusive_reg::ldaxr64(
                                        ir_ctx,
                                        rs,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                            }
                        }
                        // Load/store ordered
                        // 01x0xxxxxxxxxxx
                        (false, true, false) => {
                            let size = &bits[30..32];
                            let l = bits[22];
                            let o0 = bits[15];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((size[1], size[0]), l, o0) {
                                ((false, false), false, false) => {
                                    if !self.is_feat_impl(Feat::LOR) {
                                        None
                                    } else {
                                        Some(load_store::ldst_ordered::stllrb(
                                            ir_ctx,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                        ))
                                    }
                                }
                                ((false, false), false, true) => {
                                    Some(load_store::ldst_ordered::stlrb(
                                        ir_ctx,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, false), true, false) => {
                                    if !self.is_feat_impl(Feat::LOR) {
                                        None
                                    } else {
                                        Some(load_store::ldst_ordered::ldlarb(
                                            ir_ctx,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                        ))
                                    }
                                }
                                ((false, false), true, true) => {
                                    Some(load_store::ldst_ordered::ldarb(
                                        ir_ctx,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, true), false, false) => {
                                    if !self.is_feat_impl(Feat::LOR) {
                                        None
                                    } else {
                                        Some(load_store::ldst_ordered::stllrh(
                                            ir_ctx,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                        ))
                                    }
                                }
                                ((false, true), false, true) => {
                                    Some(load_store::ldst_ordered::stlrh(
                                        ir_ctx,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((false, true), true, false) => {
                                    if !self.is_feat_impl(Feat::LOR) {
                                        None
                                    } else {
                                        Some(load_store::ldst_ordered::ldlarh(
                                            ir_ctx,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                        ))
                                    }
                                }
                                ((false, true), true, true) => {
                                    Some(load_store::ldst_ordered::ldarh(
                                        ir_ctx,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, false), false, false) => {
                                    if !self.is_feat_impl(Feat::LOR) {
                                        None
                                    } else {
                                        Some(load_store::ldst_ordered::stllr32(
                                            ir_ctx,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                        ))
                                    }
                                }
                                ((true, false), false, true) => {
                                    Some(load_store::ldst_ordered::stlr32(
                                        ir_ctx,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, false), true, false) => {
                                    if !self.is_feat_impl(Feat::LOR) {
                                        None
                                    } else {
                                        Some(load_store::ldst_ordered::ldlar32(
                                            ir_ctx,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                        ))
                                    }
                                }
                                ((true, false), true, true) => {
                                    Some(load_store::ldst_ordered::ldar32(
                                        ir_ctx,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, true), false, false) => {
                                    if !self.is_feat_impl(Feat::LOR) {
                                        None
                                    } else {
                                        Some(load_store::ldst_ordered::stllr64(
                                            ir_ctx,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                        ))
                                    }
                                }
                                ((true, true), false, true) => {
                                    Some(load_store::ldst_ordered::stlr64(
                                        ir_ctx,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                                ((true, true), true, false) => {
                                    if !self.is_feat_impl(Feat::LOR) {
                                        None
                                    } else {
                                        Some(load_store::ldst_ordered::ldlar64(
                                            ir_ctx,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                        ))
                                    }
                                }
                                ((true, true), true, true) => {
                                    Some(load_store::ldst_ordered::ldar64(
                                        ir_ctx,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                    ))
                                }
                            }
                        }
                        // Compare and swap
                        // 01x1xxxxxxxxxxx
                        (false, true, true) => todo!(),
                        _ => None, // Unspecified by the architecture
                    },
                    ((_, _, false, true), op1, op2) => match (
                        op1,
                        (op2[14], op2[13]),
                        op2[11],
                        &op2[2..11].load::<u16>(),
                        (op2[1], op2[0]),
                    ) {
                        // LDIAPP/STILP
                        // 10x0xxxxxxxxx10
                        (false, (true, false), false, _, (true, false)) => todo!(),
                        // LDAPR/STLR (writeback)
                        // 11x000000000010
                        (false, (true, true), false, 0, (true, false)) => todo!(),
                        // LDAPR/STLR (unscaled immediate)
                        // 1xx0xxxxxxxxx00
                        (false, (true, _), false, _, (false, false)) => todo!(),
                        // LDAPR/STLR (SIMD&FP)
                        // 1xx0xxxxxxxxx10
                        (true, (true, _), false, _, (true, false)) => todo!(),
                        // Load register (literal)
                        // 0xxxxxxxxxxxxxx
                        (_, (false, _), ..) => {
                            let opc = &bits[30..32];
                            let vr = bits[26];
                            let imm19 = &bits[5..24];
                            let rt = &bits[0..5];
                            match ((opc[1], opc[0]), vr) {
                                ((false, false), false) => {
                                    Some(load_store::ld_reg_lit::ldr32(ir_ctx, imm19, rt))
                                }
                                ((false, false), true) => {
                                    Some(load_store::ld_reg_lit::ldr32_simdfp(ir_ctx, imm19, rt))
                                }
                                ((false, true), false) => {
                                    Some(load_store::ld_reg_lit::ldr64(ir_ctx, imm19, rt))
                                }
                                ((false, true), true) => {
                                    Some(load_store::ld_reg_lit::ldr64_simdfp(ir_ctx, imm19, rt))
                                }
                                ((true, false), false) => {
                                    Some(load_store::ld_reg_lit::ldrsw(ir_ctx, imm19, rt))
                                }
                                ((true, false), true) => {
                                    Some(load_store::ld_reg_lit::ldr128_simdfp(ir_ctx, imm19, rt))
                                }
                                ((true, true), false) => {
                                    Some(load_store::ld_reg_lit::prfm(ir_ctx, imm19, rt))
                                }
                                ((true, true), true) => None, // Unallocated
                            }
                        }
                        // Memory Copy and Memory Set
                        // 1xx0xxxxxxxxx01
                        (_, (true, _), false, _, (false, true)) => todo!(),
                        _ => None, // Unspecified by the architecture
                    },
                    ((_, _, true, false), _, op2) => match (op2[14], op2[13]) {
                        // Load/store no-allocate pair (offset)
                        // 00xxxxxxxxxxxxx
                        (false, false) => {
                            let opc = &bits[30..32];
                            let vr = bits[26];
                            let l = bits[22];
                            let imm7 = &bits[15..22];
                            let rt2 = &bits[10..15];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((opc[1], opc[0]), vr, l) {
                                ((false, false), false, false) => {
                                    Some(load_store::ldst_noalloc_pair_offset::stnp32(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((false, false), false, true) => {
                                    Some(load_store::ldst_noalloc_pair_offset::ldnp32(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((false, false), true, false) => {
                                    Some(load_store::ldst_noalloc_pair_offset::stnp32_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, true) => {
                                    Some(load_store::ldst_noalloc_pair_offset::ldnp32_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, _) => None, // Unallocated
                                ((false, true), true, false) => {
                                    Some(load_store::ldst_noalloc_pair_offset::stnp64_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, true) => {
                                    Some(load_store::ldst_noalloc_pair_offset::ldnp64_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, false) => {
                                    Some(load_store::ldst_noalloc_pair_offset::stnp64(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((true, false), false, true) => {
                                    Some(load_store::ldst_noalloc_pair_offset::ldnp64(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((true, false), true, false) => {
                                    Some(load_store::ldst_noalloc_pair_offset::stnp128_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, true) => {
                                    Some(load_store::ldst_noalloc_pair_offset::ldnp128_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), ..) => None, // Unallocated
                            }
                        }
                        // Load/store register pair (post-indexed)
                        // 01xxxxxxxxxxxxx
                        (false, true) => {
                            let opc = &bits[30..32];
                            let vr = bits[26];
                            let l = bits[22];
                            let imm7 = &bits[15..22];
                            let rt2 = &bits[10..15];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((opc[1], opc[0]), vr, l) {
                                ((false, false), false, false) => {
                                    Some(load_store::ldst_reg_pair_post::stp32(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((false, false), false, true) => {
                                    Some(load_store::ldst_reg_pair_post::ldp32(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((false, false), true, false) => {
                                    Some(load_store::ldst_reg_pair_post::stp32_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, true) => {
                                    Some(load_store::ldst_reg_pair_post::ldp32_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, false) => {
                                    if !self.is_feat_impl(Feat::MTE) {
                                        None
                                    } else {
                                        Some(load_store::ldst_reg_pair_post::stgp(
                                            ir_ctx,
                                            imm7,
                                            rt2,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                            self.is_feat_impl(Feat::CPA2),
                                            self.is_feat_impl(Feat::LSE2),
                                        ))
                                    }
                                }
                                ((false, true), false, true) => {
                                    Some(load_store::ldst_reg_pair_post::ldpsw(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, false) => {
                                    Some(load_store::ldst_reg_pair_post::stp64_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, true) => {
                                    Some(load_store::ldst_reg_pair_post::ldp64_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, false) => {
                                    Some(load_store::ldst_reg_pair_post::stp64(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((true, false), false, true) => {
                                    Some(load_store::ldst_reg_pair_post::ldp64(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((true, false), true, false) => {
                                    Some(load_store::ldst_reg_pair_post::stp128_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, true) => {
                                    Some(load_store::ldst_reg_pair_post::ldp128_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), ..) => None, // Unallocated
                            }
                        }
                        // Load/store register pair (offset)
                        // 10xxxxxxxxxxxxx
                        (true, false) => {
                            let opc = &bits[30..32];
                            let vr = bits[26];
                            let l = bits[22];
                            let imm7 = &bits[15..22];
                            let rt2 = &bits[10..15];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((opc[1], opc[0]), vr, l) {
                                ((false, false), false, false) => {
                                    Some(load_store::ldst_reg_pair_offset::stp32(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((false, false), false, true) => {
                                    Some(load_store::ldst_reg_pair_offset::ldp32(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((false, false), true, false) => {
                                    Some(load_store::ldst_reg_pair_offset::stp32_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, true) => {
                                    Some(load_store::ldst_reg_pair_offset::ldp32_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, false) => {
                                    if !self.is_feat_impl(Feat::MTE) {
                                        None
                                    } else {
                                        Some(load_store::ldst_reg_pair_offset::stgp(
                                            ir_ctx,
                                            imm7,
                                            rt2,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                            self.is_feat_impl(Feat::CPA2),
                                            self.is_feat_impl(Feat::LSE2),
                                        ))
                                    }
                                }
                                ((false, true), false, true) => {
                                    Some(load_store::ldst_reg_pair_offset::ldpsw(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, false) => {
                                    Some(load_store::ldst_reg_pair_offset::stp64_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, true) => {
                                    Some(load_store::ldst_reg_pair_offset::ldp64_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, false) => {
                                    Some(load_store::ldst_reg_pair_offset::stp64(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((true, false), false, true) => {
                                    Some(load_store::ldst_reg_pair_offset::ldp64(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((true, false), true, false) => {
                                    Some(load_store::ldst_reg_pair_offset::stp128_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, true) => {
                                    Some(load_store::ldst_reg_pair_offset::ldp128_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), ..) => None, // Unallocated
                            }
                        }
                        // Load/store register pair (pre-indexed)
                        // 11xxxxxxxxxxxxx
                        (true, true) => {
                            let opc = &bits[30..32];
                            let vr = bits[26];
                            let l = bits[22];
                            let imm7 = &bits[15..22];
                            let rt2 = &bits[10..15];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((opc[1], opc[0]), vr, l) {
                                ((false, false), false, false) => {
                                    Some(load_store::ldst_reg_pair_pre::stp32(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((false, false), false, true) => {
                                    Some(load_store::ldst_reg_pair_pre::ldp32(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((false, false), true, false) => {
                                    Some(load_store::ldst_reg_pair_pre::stp32_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, true) => {
                                    Some(load_store::ldst_reg_pair_pre::ldp32_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, false) => {
                                    if !self.is_feat_impl(Feat::MTE) {
                                        None
                                    } else {
                                        Some(load_store::ldst_reg_pair_pre::stgp(
                                            ir_ctx,
                                            imm7,
                                            rt2,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                            self.is_feat_impl(Feat::CPA2),
                                            self.is_feat_impl(Feat::LSE2),
                                        ))
                                    }
                                }
                                ((false, true), false, true) => {
                                    Some(load_store::ldst_reg_pair_pre::ldpsw(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, false) => {
                                    Some(load_store::ldst_reg_pair_pre::stp64_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, true) => {
                                    Some(load_store::ldst_reg_pair_pre::ldp64_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, false) => {
                                    Some(load_store::ldst_reg_pair_pre::stp64(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((true, false), false, true) => {
                                    Some(load_store::ldst_reg_pair_pre::ldp64(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                        self.is_feat_impl(Feat::LSE2),
                                    ))
                                }
                                ((true, false), true, false) => {
                                    Some(load_store::ldst_reg_pair_pre::stp128_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, true) => {
                                    Some(load_store::ldst_reg_pair_pre::ldp128_simdfp(
                                        ir_ctx,
                                        imm7,
                                        rt2,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), ..) => None, // Unallocated
                            }
                        }
                    },
                    ((_, _, true, true), _, op2) => match (op2[14], op2[11], op2[1], op2[0]) {
                        // Load/store register (unscaled immediate)
                        // 0xx0xxxxxxxxx00
                        (false, false, false, false) => {
                            let size = &bits[30..32];
                            let vr = bits[26];
                            let opc = &bits[22..24];
                            let imm12 = &bits[12..21];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((size[1], size[0]), vr, (opc[1], opc[0])) {
                                ((false, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::sturb(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldurb(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldursb64(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldursb32(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::stur8_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldur8_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::stur128_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldur128_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::sturh(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldurh(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldursh64(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldursh32(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::stur16_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldur16_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (true, _)) => None, // Unallocated
                                ((true, _), false, (true, true)) => None, // Unallocated
                                ((true, _), true, (true, _)) => None,     // Unallocated
                                ((true, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::stur32(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldur32(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldursw(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (false, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::stur32_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (false, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldur32_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::stur64(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldur64(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (true, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::prfum(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, false)) => {
                                    Some(load_store::ldst_reg_usc_imm::stur64_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, true)) => {
                                    Some(load_store::ldst_reg_usc_imm::ldur64_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                            }
                        }
                        // Load/store register (immediate post-indexed)
                        // 0xx0xxxxxxxxx01
                        (false, false, false, true) => {
                            let size = &bits[30..32];
                            let vr = bits[26];
                            let opc = &bits[22..24];
                            let imm9 = &bits[12..21];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((size[1], size[0]), vr, (opc[1], opc[0])) {
                                ((false, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_post_imm::strb(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldrb(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_post_imm::ldrsb64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldrsb32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, false)) => {
                                    Some(load_store::ldst_reg_post_imm::str8_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldr8_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, false)) => {
                                    Some(load_store::ldst_reg_post_imm::str128_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldr128_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_post_imm::strh(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldrh(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, false)) => {
                                    Some(load_store::ldst_reg_post_imm::ldrsh64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldrsh32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, false)) => {
                                    Some(load_store::ldst_reg_post_imm::str16_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldr16_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (true, _)) => None, // Unallocated
                                ((true, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_post_imm::str32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldr32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_post_imm::ldrsw(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, true)) => None, // Unallocated
                                ((true, false), true, (false, false)) => {
                                    Some(load_store::ldst_reg_post_imm::str32_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (false, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldr32_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (true, _)) => None, // Unallocated
                                ((true, true), _, (true, _)) => None,     // Unallocated
                                ((true, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_post_imm::str64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldr64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, false)) => {
                                    Some(load_store::ldst_reg_post_imm::str64_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, true)) => {
                                    Some(load_store::ldst_reg_post_imm::ldr64_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                            }
                        }
                        // Load/store register (unprivileged)
                        // 0xx0xxxxxxxxx10
                        (false, false, true, false) => {
                            let size = &bits[30..32];
                            let vr = bits[26];
                            let opc = &bits[22..24];
                            let imm9 = &bits[12..21];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((size[1], size[0]), vr, (opc[1], opc[0])) {
                                (_, true, _) => None, // Unallocated
                                ((false, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_unpriv::sttrb(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtrb(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtrsb64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, true)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtrsb32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_unpriv::sttrh(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtrh(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, false)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtrsh64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, true)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtrsh32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_unpriv::sttr32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtr32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtrsw(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, true)) => None, // Unallocated
                                ((true, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_unpriv::sttr64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_unpriv::ldtr64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (true, _)) => None, // Unallocated
                            }
                        }
                        // Load/store register (immediate pre-indexed)
                        // 0xx0xxxxxxxxx11
                        (false, false, true, true) => {
                            let size = &bits[30..32];
                            let vr = bits[26];
                            let opc = &bits[22..24];
                            let imm9 = &bits[12..21];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((size[1], size[0]), vr, (opc[1], opc[0])) {
                                ((false, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::strb(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldrb(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldrsb64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldrsb32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::str8_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldr8_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::str128_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldr128_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::strh(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldrh(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldrsh64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldrsh32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::str16_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldr16_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (true, _)) => None, // Unallocated
                                ((true, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::str32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldr32(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldrsw(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, true)) => None, // Unallocated
                                ((true, false), true, (false, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::str32_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (false, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldr32_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (true, _)) => None, // Unallocated
                                ((true, true), _, (true, _)) => None,     // Unallocated
                                ((true, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::str64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldr64(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, false)) => {
                                    Some(load_store::ldst_reg_pre_imm::str64_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, true)) => {
                                    Some(load_store::ldst_reg_pre_imm::ldr64_simdfp(
                                        ir_ctx,
                                        imm9,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                            }
                        }
                        // Atomic memory operations
                        // 0xx1xxxxxxxxx00
                        (false, true, false, false) => todo!(),
                        // Load/store register (register offset)
                        // 0xx1xxxxxxxxx10
                        (false, true, true, false) => {
                            let size = &bits[30..32];
                            let vr = bits[26];
                            let opc = &bits[22..24];
                            let rm = &bits[16..21];
                            let option = &bits[13..16];
                            let s = bits[12];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((size[1], size[0]), vr, (opc[1], opc[0]), option[1], rt) {
                                ((false, false), false, (false, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::strb(
                                        ir_ctx,
                                        rm,
                                        option,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (false, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldrb(
                                        ir_ctx,
                                        rm,
                                        option,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldrsb64(
                                        ir_ctx,
                                        rm,
                                        option,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldrsb32(
                                        ir_ctx,
                                        rm,
                                        option,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::str8_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldr8_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::str128_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldr128_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::strh(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldrh(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldrsh64(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldrsh32(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::str16_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldr16_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (true, _), _, _) => None, // Unallocated
                                ((true, _), false, (true, true), _, _) => None, // Unallocated
                                ((true, _), true, (true, _), _, _) => None,     // Unallocated
                                ((true, false), false, (false, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::str32(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (false, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldr32(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldrsw(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (false, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::str32_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (false, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldr32_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::str64(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldr64(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (true, false), true, rt)
                                    if (rt[4] && rt[3]) =>
                                {
                                    if !self.is_feat_impl(Feat::RPRFM) {
                                        None
                                    } else {
                                        Some(load_store::ldst_reg_regoff::rprfm(
                                            ir_ctx,
                                            rm,
                                            option,
                                            s,
                                            rn,
                                            rt,
                                            self.curr_el(),
                                            self.read::<pstate::SP>() != 0,
                                            self.is_feat_impl(Feat::CPA2),
                                        ))
                                    }
                                }
                                ((true, true), false, (true, false), true, _) => {
                                    Some(load_store::ldst_reg_regoff::pfrm(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (true, false), false, _) => {
                                    // Unallocated
                                    None
                                }
                                ((true, true), true, (false, false), _, _) => {
                                    Some(load_store::ldst_reg_regoff::str64_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, true), _, _) => {
                                    Some(load_store::ldst_reg_regoff::ldr64_simdfp(
                                        ir_ctx,
                                        rm,
                                        option,
                                        s,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                            }
                        }
                        // Load/store register (pac)
                        // 0xx1xxxxxxxxxx1
                        (false, true, _, true) => todo!(),
                        // Load/store register (unsigned immediate)
                        // 1xxxxxxxxxxxxxx
                        (true, ..) => {
                            let size = &bits[30..32];
                            let vr = bits[26];
                            let opc = &bits[22..24];
                            let imm12 = &bits[10..22];
                            let rn = &bits[5..10];
                            let rt = &bits[0..5];
                            match ((size[1], size[0]), vr, (opc[1], opc[0])) {
                                ((false, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::strb(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldrb(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldrsb64(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), false, (true, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldrsb32(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::str8_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (false, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldr8_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::str128_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, false), true, (true, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldr128_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::strh(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldrh(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldrsh64(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), false, (true, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldrsh32(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::str16_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (false, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldr16_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((false, true), true, (true, _)) => None, // Unallocated
                                ((true, _), false, (true, true)) => None, // Unallocated
                                ((true, _), true, (true, _)) => None,     // Unallocated
                                ((true, false), false, (false, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::str32(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (false, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldr32(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), false, (true, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldrsw(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (false, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::str32_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, false), true, (false, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldr32_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::str64(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (false, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldr64(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), false, (true, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::prfm(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, false)) => {
                                    Some(load_store::ldst_reg_usn_imm::str64_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                                ((true, true), true, (false, true)) => {
                                    Some(load_store::ldst_reg_usn_imm::ldr64_simdfp(
                                        ir_ctx,
                                        imm12,
                                        rn,
                                        rt,
                                        self.curr_el(),
                                        self.read::<pstate::SP>() != 0,
                                        self.is_feat_impl(Feat::CPA2),
                                    ))
                                }
                            }
                        }
                    },
                    _ => None, // Unspecified by the architecture
                }
            }
        }
    }
}
