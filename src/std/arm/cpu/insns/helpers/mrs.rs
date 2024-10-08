use crate::core::model::{ir::*, proc::*};
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate};
use crate::std::arm::cpu::ArmCpu;
use bitvec::prelude::*;

pub fn emulate_mrs<'a>(cpu: &ArmCpu, ctx: &mut ProcCtx<'a, ArmCpu>, args: Vec<CallArg>) -> u64 {
    let this_instr = args
        .first()
        .and_then(|arg| {
            if let CallArg::Bv32(this_instr) = arg {
                Some(this_instr)
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding")
        .clone();

    let mut ctx = cpu.ctx(ctx);

    let op0: u8 = 2 + if this_instr.view_bits::<Lsb0>()[19] {
        1
    } else {
        0
    };
    let op1: u8 = (&this_instr.view_bits::<Lsb0>()[16..19]).load();
    let crn: u8 = (&this_instr.view_bits::<Lsb0>()[12..16]).load();
    let crm: u8 = (&this_instr.view_bits::<Lsb0>()[8..12]).load();
    let op2: u8 = (&this_instr.view_bits::<Lsb0>()[5..8]).load();

    let data = match (op0, op1, crn, crm, op2) {
        (0b11, 0b111, 0b1110, 0b0010, 0b001) => ctx.read::<CNTPS_CTL_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0100, 0b001) => {
        // 	ctx.read::<ERXCTLR>()
        // }
        (0b11, 0b000, 0b0101, 0b0101, 0b010) => ctx.read::<ERXMISC2_EL1>(),
        (0b11, 0b100, 0b1100, 0b0000, 0b000) => ctx.read::<VBAR_EL2>(),
        // (0b1111, 0b000, 0b1010, 0b0010, 0b001) => {
        // 	ctx.read::<MAIR1>()
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b001) => ctx.read::<ICC_EOIR1_EL1>(),
        // (0b10, 0b000, 0b1110, 0b10:m[4:3], m[2:0]) => {
        // 	ctx.read::<PMEVCNTSVRn_EL1>()
        // }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b011) => {
        // 	ctx.read::<ERXMISC5>()
        // }
        (0b11, 0b000, 0b1100, 0b1011, 0b001) => ctx.read::<ICC_DIR_EL1>(),
        (0b11, 0b100, 0b1110, 0b0101, 0b000) => ctx.read::<CNTHPS_TVAL_EL2>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b011) => ctx.read::<ICC_BPR1_EL1>(),
        (0b11, 0b000, 0b1001, 0b1011, 0b100) => ctx.read::<TRBMAR_EL1>(),
        (0b11, 0b100, 0b0001, 0b0010, 0b011) => ctx.read::<TRCITECR_EL2>(),
        // (0b1111, 0b100, 0b1100, 0b1011, 0b111) => {
        // 	ctx.read::<ICH_VMCR>()
        // }
        (0b10, 0b001, 0b0000, 0b1100, 0b111) => ctx.read::<TRCIDR4>(),
        (0b11, 0b100, 0b0001, 0b0000, 0b001) => ctx.read::<ACTLR_EL2>(),
        (0b11, 0b011, 0b1101, 0b0010, 0b010) => ctx.read::<AMCGCR_EL0>(),
        (0b11, 0b011, 0b1101, 0b0011, 0b000) => ctx.read::<AMCNTENCLR1_EL0>(),
        (0b11, 0b000, 0b1101, 0b0000, 0b101) => ctx.read::<ACCDATA_EL1>(),
        (0b11, 0b000, 0b0000, 0b0010, 0b001) => ctx.read::<ID_ISAR1_EL1>(),
        (0b10, 0b000, 0b1001, 0b1110, 0b001) => ctx.read::<SPMINTENSET_EL1>(),
        (0b10, 0b001, 0b0011, 0b0011, 0b010) => ctx.read::<TRCVMIDCCTLR1>(),
        // (0b10, 0b000, 0b0000, m[3:0], 0b110) => {
        // 	ctx.read::<DBGWVRn_EL1>()
        // }
        // (0b1111, 0b000, 0b0110, 0b0000, 0b000) => {
        // 	ctx.read::<DFAR>()
        // }
        (0b11, 0b000, 0b0001, 0b0000, 0b010) => ctx.read::<CPACR_EL1>(),
        // (0b1110, 0b000, 0b0111, 0b1000, 0b110) => {
        // 	ctx.read::<DBGCLAIMSET>()
        // }
        (0b11, 0b000, 0b1110, 0b0001, 0b000) => ctx.read::<CNTKCTL_EL1>(),
        (0b10, 0b001, 0b0011, 0b0010, 0b010) => ctx.read::<TRCVMIDCCTLR0>(),
        (0b11, 0b000, 0b0101, 0b0011, 0b010) => ctx.read::<ERXGSR_EL1>(),
        (0b11, 0b000, 0b0010, 0b0010, 0b011) => ctx.read::<APDBKeyHi_EL1>(),
        // (0b1110, 0b000, 0b0000, 0b0110, 0b010) => {
        // 	ctx.read::<DBGOSECCR>()
        // }
        (0b10, 0b001, 0b0000, 0b0000, 0b001) => ctx.read::<TRCTRACEIDR>(),
        // (0b1111, 0b001, 0b0000, 0b0000, 0b010) => {
        // 	ctx.read::<CCSIDR2>()
        // }
        (0b11, 0b000, 0b0000, 0b0011, 0b001) => ctx.read::<MVFR1_EL1>(),
        // (0b1111, 0b100, 0b0010, 0b0001, 0b010) => {
        // 	ctx.read::<VTCR>()
        // }
        (0b11, 0b000, 0b1100, 0b1011, 0b111) => ctx.read::<ICC_SGI0R_EL1>(),
        (0b10, 0b001, 0b1001, 0b0001, 0b010) => ctx.read::<BRBTGTINJ_EL1>(),
        (0b11, 0b000, 0b1100, 0b1011, 0b101) => ctx.read::<ICC_SGI1R_EL1>(),
        (0b11, 0b110, 0b0001, 0b0000, 0b001) => ctx.read::<ACTLR_EL3>(),
        (0b10, 0b001, 0b0000, 0b1101, 0b111) => ctx.read::<TRCIDR5>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b101) => ctx.read::<ICC_SRE_EL1>(),
        (0b11, 0b100, 0b1010, 0b0110, 0b001) => ctx.read::<MPAMVPM1_EL2>(),
        (0b11, 0b011, 0b1001, 0b1110, 0b011) => ctx.read::<PMOVSSET_EL0>(),
        // (0b1111, 0b000, 0b0001, 0b0000, 0b000) => {
        // 	ctx.read::<SCTLR>()
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b111) => ctx.read::<PMCEID1_EL0>(),
        // (0b10, 0b000, 0b0000, m[3:0], 0b111) => {
        // 	ctx.read::<DBGWCRn_EL1>()
        // }
        (0b11, 0b100, 0b1010, 0b1000, 0b010) => ctx.read::<MECID_P1_EL2>(),
        (0b11, 0b000, 0b0001, 0b0010, 0b110) => ctx.read::<SMCR_EL1>(),
        (0b11, 0b011, 0b0100, 0b0100, 0b010) => ctx.read::<FPMR>(),
        (0b10, 0b011, 0b1001, 0b1100, 0b000) => ctx.read::<SPMCR_EL0>(),
        (0b11, 0b000, 0b1010, 0b0100, 0b100) => ctx.read::<MPAMIDR_EL1>(),
        (0b11, 0b000, 0b0100, 0b0000, 0b000) => ctx.read::<SPSR_EL1>(),
        (0b11, 0b000, 0b0100, 0b0000, 0b001) => ctx.read::<ELR_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0101, 0b010) => {
        // 	ctx.read::<ERXMISC4>()
        // }
        (0b11, 0b011, 0b1101, 0b0000, 0b101) => ctx.read::<TPIDR2_EL0>(),
        // (0b1111, 0b000, 0b1010, 0b0010, 0b000) => {
        // 	ctx.read::<MAIR0>()
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b000) => ctx.read::<ICV_IAR1_EL1>(),
        (0b11, 0b110, 0b1100, 0b0000, 0b000) => ctx.read::<VBAR_EL3>(),
        (0b11, 0b100, 0b0011, 0b0001, 0b100) => ctx.read::<HDFGRTR_EL2>(),
        // (0b1111, 0b000, 0b0101, 0b0001, 0b001) => {
        // 	ctx.read::<AIFSR>()
        // }
        (0b11, 0b000, 0b0000, 0b0001, 0b001) => ctx.read::<ID_PFR1_EL1>(),
        (0b11, 0b000, 0b1100, 0b0000, 0b000) => ctx.read::<VBAR_EL1>(),
        (0b11, 0b000, 0b1001, 0b1101, 0b011) => ctx.read::<PMSSCR_EL1>(),
        (0b10, 0b001, 0b0000, 0b1100, 0b000) => ctx.read::<TRCTSCTLR>(),
        (0b11, 0b000, 0b1001, 0b1010, 0b001) => ctx.read::<PMBPTR_EL1>(),
        (0b11, 0b000, 0b0000, 0b0010, 0b110) => ctx.read::<ID_MMFR4_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0101, 0b110) => {
        // 	ctx.read::<ERXMISC6>()
        // }
        (0b11, 0b011, 0b1101, 0b0010, 0b101) => ctx.read::<AMCNTENSET0_EL0>(),
        // (0b1110, 0b000, 0b0001, 0b0011, 0b100) => {
        // 	ctx.read::<DBGOSDLR>()
        // }
        (0b11, 0b110, 0b0100, 0b0000, 0b001) => ctx.read::<ELR_EL3>(),
        (0b11, 0b000, 0b1001, 0b1011, 0b110) => ctx.read::<TRBTRG_EL1>(),
        (0b11, 0b110, 0b0100, 0b0000, 0b000) => ctx.read::<SPSR_EL3>(),
        // (0b1111, 0b000, 0b1100, 0b1011, 0b001) => {
        // 	ctx.read::<ICV_DIR>()
        // }
        (0b11, 0b000, 0b0000, 0b0111, 0b000) => ctx.read::<ID_AA64MMFR0_EL1>(),
        (0b11, 0b110, 0b0001, 0b0001, 0b000) => ctx.read::<SCR_EL3>(),
        (0b11, 0b100, 0b0011, 0b0001, 0b111) => ctx.read::<HFGITR2_EL2>(),
        (0b10, 0b000, 0b0001, 0b0000, 0b000) => ctx.read::<MDRAR_EL1>(),
        (0b11, 0b110, 0b0001, 0b0010, 0b110) => ctx.read::<SMCR_EL3>(),
        (0b11, 0b011, 0b1110, 0b0000, 0b010) => {
            // ctx.read::<CNTVCT_EL0>()
            ctx.physical_count()
        }
        (0b11, 0b000, 0b0000, 0b0101, 0b101) => ctx.read::<ID_AA64AFR1_EL1>(),
        (0b11, 0b110, 0b1100, 0b0001, 0b001) => ctx.read::<VDISR_EL3>(),
        (0b11, 0b000, 0b0001, 0b0010, 0b011) => ctx.read::<TRCITECR_EL1>(),
        (0b10, 0b001, 0b0000, 0b1111, 0b111) => ctx.read::<TRCIDR7>(),
        (0b11, 0b000, 0b0000, 0b0101, 0b000) => ctx.read::<ID_AA64DFR0_EL1>(),
        (0b11, 0b110, 0b1100, 0b1100, 0b101) => ctx.read::<ICC_SRE_EL3>(),
        (0b11, 0b011, 0b1001, 0b0110, 0b000) => ctx.read::<PMICFILTR_EL0>(),
        (0b11, 0b000, 0b0001, 0b0000, 0b001) => ctx.read::<ACTLR_EL1>(),
        (0b11, 0b011, 0b1110, 0b0011, 0b000) => ctx.read::<CNTV_TVAL_EL0>(),
        (0b11, 0b000, 0b0000, 0b0100, 0b010) => ctx.read::<ID_AA64PFR2_EL1>(),
        // (0b1111, 0b110, 0b1100, 0b1100, 0b100) => {
        // 	ctx.read::<ICC_MCTLR>()
        // }
        (0b10, 0b001, 0b0000, 0b0011, 0b010) => ctx.read::<TRCVIPCSSCTLR>(),
        // (0b1111, 0b000, 0b1100, 0b1011, 0b001) => {
        // 	ctx.read::<ICC_DIR>()
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b001) => ctx.read::<ICV_EOIR1_EL1>(),
        (0b11, 0b000, 0b0000, 0b0100, 0b100) => ctx.read::<ID_AA64ZFR0_EL1>(),
        // (0b1110, 0b000, 0b0111, 0b1001, 0b110) => {
        // 	ctx.read::<DBGCLAIMCLR>()
        // }
        // (0b1111, 0b000, 0b0001, 0b0010, 0b001) => {
        // 	ctx.read::<TRFCR>()
        // }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b000) => {
        // 	ctx.read::<ERXFR>()
        // }
        // (0b1111, 0b100, 0b1110, 0b0010, 0b001) => {
        // 	ctx.read::<CNTHP_CTL>()
        // }
        // (0b1111, 0b100, 0b0101, 0b0010, 0b000) => {
        // 	ctx.read::<HSR>()
        // }
        (0b10, 0b011, 0b0000, 0b0001, 0b000) => ctx.read::<MDCCSR_EL0>(),
        (0b11, 0b100, 0b1100, 0b1011, 0b101) => ctx.read::<ICH_ELRSR_EL2>(),
        // (0b1110, 0b000, 0b0000, 0b0101, 0b000) => {
        // 	ctx.read::<DBGDTRRXint>()
        // }
        (0b11, 0b100, 0b1100, 0b1001, 0b101) => ctx.read::<ICC_SRE_EL2>(),
        (0b11, 0b100, 0b0100, 0b0011, 0b001) => ctx.read::<SPSR_abt>(),
        (0b10, 0b001, 0b0000, 0b1110, 0b111) => ctx.read::<TRCIDR6>(),
        // (0b1110, 0b000, 0b0000, 0b0000, 0b010) => {
        // 	ctx.read::<DBGDTRRXext>()
        // }
        (0b11, 0b000, 0b1010, 0b0101, 0b011) => ctx.read::<MPAMSM_EL1>(),
        (0b11, 0b000, 0b1001, 0b1001, 0b100) => ctx.read::<PMSFCR_EL1>(),
        (0b11, 0b100, 0b1100, 0b0001, 0b001) => ctx.read::<VDISR_EL2>(),
        (0b11, 0b100, 0b0001, 0b0010, 0b110) => ctx.read::<SMCR_EL2>(),
        (0b11, 0b100, 0b0110, 0b0000, 0b100) => ctx.read::<HPFAR_EL2>(),
        (0b11, 0b100, 0b0100, 0b0000, 0b000) => ctx.read::<SPSR_EL2>(),
        (0b11, 0b110, 0b0010, 0b0001, 0b110) => ctx.read::<GPCCR_EL3>(),
        (0b11, 0b011, 0b1001, 0b1100, 0b101) => ctx.read::<PMSELR_EL0>(),
        // (0b1111, 0b000, 0b0101, 0b0101, 0b111) => {
        // 	ctx.read::<ERXMISC7>()
        // }
        (0b10, 0b111, 0b1001, 0b1110, 0b111) => ctx.read::<SPMSCR_EL1>(),
        (0b11, 0b100, 0b0100, 0b0000, 0b001) => ctx.read::<ELR_EL2>(),
        (0b11, 0b000, 0b1001, 0b1011, 0b000) => ctx.read::<TRBLIMITR_EL1>(),
        (0b11, 0b000, 0b1010, 0b0100, 0b011) => ctx.read::<LORC_EL1>(),
        (0b11, 0b100, 0b0011, 0b0001, 0b011) => ctx.read::<HFGWTR2_EL2>(),
        (0b11, 0b000, 0b1010, 0b0100, 0b001) => ctx.read::<LOREA_EL1>(),
        (0b11, 0b000, 0b0010, 0b0101, 0b010) => ctx.read::<GCSCRE0_EL1>(),
        // (0b1111, 0b000, 0b0000, 0b0000, 0b101) => {
        // 	ctx.read::<MPIDR>()
        // }
        (0b10, 0b001, 0b0000, 0b0000, 0b010) => ctx.read::<TRCVICTLR>(),
        (0b11, 0b000, 0b0000, 0b0111, 0b001) => ctx.read::<ID_AA64MMFR1_EL1>(),
        // (0b1110, 0b111, 0b0000, 0b0000, 0b000) => {
        // 	ctx.read::<JIDR>()
        // }
        // (0b10, 0b001, 0b0001, 0b0:m[2:0], 0b010) => {
        // 	ctx.read::<TRCSSCCRn>()
        // }
        (0b11, 0b000, 0b0000, 0b0101, 0b100) => ctx.read::<ID_AA64AFR0_EL1>(),
        (0b11, 0b000, 0b0101, 0b0011, 0b000) => ctx.read::<ERRIDR_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0101, 0b101) => {
        // 	ctx.read::<ERXMISC3>()
        // }
        (0b11, 0b000, 0b1101, 0b0000, 0b111) => ctx.read::<SCXTNUM_EL1>(),
        (0b11, 0b100, 0b1101, 0b0000, 0b010) => ctx.read::<TPIDR_EL2>(),
        (0b11, 0b000, 0b0000, 0b0001, 0b000) => ctx.read::<ID_PFR0_EL1>(),
        // (0b1111, 0b000, 0b1001, 0b1101, 0b010) => {
        // 	ctx.read::<PMXEVCNTR>()
        // }
        (0b10, 0b001, 0b0000, 0b0111, 0b100) => ctx.read::<TRCSEQSTR>(),
        (0b10, 0b000, 0b0001, 0b0001, 0b100) => ctx.read::<OSLSR_EL1>(),
        (0b11, 0b011, 0b1101, 0b0011, 0b001) => ctx.read::<AMCNTENSET1_EL0>(),
        (0b11, 0b000, 0b0000, 0b0011, 0b110) => ctx.read::<ID_MMFR5_EL1>(),
        (0b11, 0b001, 0b0000, 0b0000, 0b110) => ctx.read::<SMIDR_EL1>(),
        (0b11, 0b000, 0b1100, 0b1011, 0b110) => ctx.read::<ICC_ASGI1R_EL1>(),
        // (0b1111, 0b000, 0b0001, 0b0001, 0b001) => {
        // 	ctx.read::<SDER>()
        // }
        // (0b11, 0b000, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.read::<ICV_AP1Rn_EL1>()
        // }
        (0b10, 0b001, 0b0000, 0b1010, 0b111) => ctx.read::<TRCIDR2>(),
        // (0b10, 0b001, 0b0000, 0b00:m[1:0], 0b100) => {
        // 	ctx.read::<TRCSEQEVRn>()
        // }
        (0b11, 0b100, 0b0100, 0b0011, 0b011) => ctx.read::<SPSR_fiq>(),
        (0b10, 0b011, 0b0000, 0b0101, 0b000) => ctx.read::<DBGDTRRX_EL0>(),
        // (0b1111, 0b000, 0b1001, 0b1100, 0b011) => {
        // 	ctx.read::<PMOVSR>()
        // }
        (0b11, 0b100, 0b1010, 0b0011, 0b000) => ctx.read::<AMAIR_EL2>(),
        (0b11, 0b000, 0b0000, 0b0101, 0b001) => ctx.read::<ID_AA64DFR1_EL1>(),
        // (0b1111, 0b100, 0b1010, 0b0011, 0b001) => {
        // 	ctx.read::<HAMAIR1>()
        // }
        (0b11, 0b011, 0b1110, 0b0011, 0b001) => ctx.read::<CNTV_CTL_EL0>(),
        (0b11, 0b100, 0b0101, 0b0010, 0b000) => ctx.read::<ESR_EL2>(),
        (0b11, 0b110, 0b0001, 0b0000, 0b011) => ctx.read::<SCTLR2_EL3>(),
        (0b11, 0b000, 0b0000, 0b0100, 0b111) => ctx.read::<ID_AA64FPFR0_EL1>(),
        // (0b11, 0b011, 0b1110, 0b11:m[4:3], m[2:0]) => {
        // 	ctx.read::<PMEVTYPERn_EL0>()
        // }
        (0b11, 0b100, 0b0001, 0b0000, 0b011) => ctx.read::<SCTLR2_EL2>(),
        (0b11, 0b110, 0b0101, 0b0010, 0b000) => ctx.read::<ESR_EL3>(),
        (0b10, 0b001, 0b1001, 0b0000, 0b000) => ctx.read::<BRBCR_EL2>(),
        // (0b1111, 0b100, 0b1010, 0b0011, 0b000) => {
        // 	ctx.read::<HAMAIR0>()
        // }
        (0b11, 0b110, 0b1010, 0b0011, 0b000) => ctx.read::<AMAIR_EL3>(),
        (0b10, 0b011, 0b1001, 0b1100, 0b011) => ctx.read::<SPMOVSCLR_EL0>(),
        (0b11, 0b000, 0b0001, 0b0000, 0b101) => ctx.read::<RGSR_EL1>(),
        (0b11, 0b100, 0b0011, 0b0001, 0b010) => ctx.read::<HFGRTR2_EL2>(),
        (0b11, 0b000, 0b1100, 0b1000, 0b001) => ctx.read::<ICV_EOIR0_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b1111, 0b111) => {
        // 	ctx.read::<PMCCFILTR>()
        // }
        (0b11, 0b100, 0b1100, 0b1011, 0b001) => ctx.read::<ICH_VTR_EL2>(),
        (0b10, 0b001, 0b0000, 0b1011, 0b111) => ctx.read::<TRCIDR3>(),
        (0b11, 0b011, 0b1101, 0b0000, 0b111) => ctx.read::<SCXTNUM_EL0>(),
        // (0b1110, 0b000, 0b0111, 0b0001, 0b111) => {
        // 	ctx.read::<DBGDEVID1>()
        // }
        // (0b1111, 0b000, 0b1001, 0b1101, 0b001) => {
        // 	ctx.read::<PMXEVTYPER>()
        // }
        (0b11, 0b110, 0b1101, 0b0000, 0b010) => ctx.read::<TPIDR_EL3>(),
        // (0b1111, 0b000, 0b1100, 0b0000, 0b001) => {
        // 	ctx.read::<RVBAR>()
        // }
        (0b11, 0b001, 0b0000, 0b0000, 0b100) => ctx.read::<GMID_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0101, 0b100) => {
        // 	ctx.read::<ERXMISC2>()
        // }
        (0b10, 0b001, 0b1001, 0b0000, 0b001) => ctx.read::<BRBFCR_EL1>(),
        (0b11, 0b000, 0b1001, 0b1001, 0b110) => ctx.read::<PMSLATFR_EL1>(),
        (0b11, 0b011, 0b1001, 0b1100, 0b010) => ctx.read::<PMCNTENCLR_EL0>(),
        // (0b1111, 0b000, 0b0111, 0b0100, 0b000) => {
        // 	ctx.read::<PAR>()
        // }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b000) => {
        // 	ctx.read::<ERXMISC0>()
        // }
        (0b11, 0b000, 0b1100, 0b1001, 0b101) => ctx.read::<ICC_NMIAR1_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0100, 0b111) => {
        // 	ctx.read::<ERXADDR2>()
        // }
        (0b11, 0b000, 0b1101, 0b0000, 0b100) => ctx.read::<TPIDR_EL1>(),
        (0b11, 0b000, 0b0101, 0b0101, 0b011) => ctx.read::<ERXMISC3_EL1>(),
        (0b11, 0b100, 0b1101, 0b0000, 0b111) => ctx.read::<SCXTNUM_EL2>(),
        (0b10, 0b000, 0b0000, 0b0011, 0b010) => ctx.read::<OSDTRTX_EL1>(),
        // (0b10, 0b001, 0b0001, 0b0:m[2:0], 0b011) => {
        // 	ctx.read::<TRCSSPCICRn>()
        // }
        (0b11, 0b000, 0b1001, 0b1001, 0b101) => ctx.read::<PMSEVFR_EL1>(),
        (0b11, 0b000, 0b1100, 0b1000, 0b001) => ctx.read::<ICC_EOIR0_EL1>(),
        (0b11, 0b100, 0b1110, 0b0011, 0b001) => ctx.read::<CNTHV_CTL_EL2>(),
        // (0b1111, 0b100, 0b1100, 0b1011, 0b010) => {
        // 	ctx.read::<ICH_MISR>()
        // }
        (0b11, 0b000, 0b0100, 0b0011, 0b000) => {
            let allint = ctx.read::<pstate::ALLINT>() != 0;
            let allint = if allint { 1 } else { 0 };
            let allint = allint << 13;
            allint
        }
        (0b10, 0b000, 0b1001, 0b1101, 0b100) => ctx.read::<SPMIIDR_EL1>(),
        (0b10, 0b001, 0b0000, 0b1001, 0b111) => ctx.read::<TRCIDR1>(),
        (0b11, 0b100, 0b0000, 0b0000, 0b101) => ctx.read::<VMPIDR_EL2>(),
        // (0b1111, 0b000, 0b1101, 0b0000, 0b001) => {
        // 	ctx.read::<CONTEXTIDR>()
        // }
        (0b11, 0b000, 0b0000, 0b0010, 0b000) => ctx.read::<ID_ISAR0_EL1>(),
        (0b10, 0b000, 0b0001, 0b0100, 0b100) => ctx.read::<DBGPRCR_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b0011, 0b000) => {
        // 	ctx.read::<CNTV_TVAL>()
        // }
        (0b11, 0b000, 0b1010, 0b0011, 0b000) => ctx.read::<AMAIR_EL1>(),
        // (0b1110, 0b000, 0b0000, m[3:0], 0b101) => {
        // 	ctx.read::<DBGBCRn>()
        // }
        (0b11, 0b000, 0b1100, 0b1000, 0b011) => ctx.read::<ICC_BPR0_EL1>(),
        // (0b1111, 0b000, 0b1101, 0b0000, 0b010) => {
        // 	ctx.read::<TPIDRURW>()
        // }
        (0b11, 0b000, 0b0101, 0b0010, 0b000) => ctx.read::<ESR_EL1>(),
        // (0b1110, 0b000, 0b0001, 0b0000, 0b000) => {
        // 	ctx.read::<DBGDRAR>()
        // }
        // (0b1110, 0b000, 0b0001, m[3:0], 0b001) => {
        // 	ctx.read::<DBGBXVRn>()
        // }
        (0b11, 0b011, 0b1101, 0b0010, 0b100) => ctx.read::<AMCNTENCLR0_EL0>(),
        // (0b1111, 0b000, 0b0101, 0b0100, 0b011) => {
        // 	ctx.read::<ERXADDR>()
        // }
        (0b11, 0b000, 0b0010, 0b0001, 0b010) => ctx.read::<APIBKeyLo_EL1>(),
        (0b11, 0b000, 0b0000, 0b0011, 0b000) => ctx.read::<MVFR0_EL1>(),
        (0b11, 0b000, 0b0001, 0b0000, 0b011) => ctx.read::<SCTLR2_EL1>(),
        (0b11, 0b011, 0b0100, 0b0010, 0b110) => {
            let ssbs = ctx.read::<pstate::SSBS>() != 0;
            let ssbs = if ssbs { 1 } else { 0 };
            let ssbs = ssbs << 12;
            ssbs
        }
        (0b10, 0b001, 0b1001, 0b0000, 0b000) => ctx.read::<BRBCR_EL1>(),
        // (0b1111, 0b000, 0b0100, 0b0110, 0b000) => {
        // 	ctx.read::<ICV_PMR>()
        // }
        (0b11, 0b100, 0b1010, 0b0110, 0b000) => ctx.read::<MPAMVPM0_EL2>(),
        (0b11, 0b011, 0b1110, 0b0000, 0b001) => ctx.read::<CNTPCT_EL0>(),
        // (0b1111, 0b000, 0b1110, 0b0001, 0b000) => {
        // 	ctx.read::<CNTKCTL>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b101) => {
        // 	ctx.read::<ICC_SRE>()
        // }
        (0b11, 0b100, 0b1010, 0b0100, 0b000) => ctx.read::<MPAMHCR_EL2>(),
        (0b10, 0b001, 0b0000, 0b1000, 0b111) => ctx.read::<TRCIDR0>(),
        // (0b1111, 0b000, 0b0100, 0b0110, 0b000) => {
        // 	ctx.read::<ICC_PMR>()
        // }
        (0b11, 0b011, 0b1110, 0b0010, 0b000) => ctx.read::<CNTP_TVAL_EL0>(),
        // (0b11, 0b000, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.read::<ICC_AP1Rn_EL1>()
        // }
        (0b11, 0b011, 0b1101, 0b0000, 0b010) => ctx.read::<TPIDR_EL0>(),
        (0b11, 0b110, 0b1101, 0b0000, 0b111) => ctx.read::<SCXTNUM_EL3>(),
        // (0b1110, 0b000, 0b0111, 0b0000, 0b111) => {
        // 	ctx.read::<DBGDEVID2>()
        // }
        (0b11, 0b000, 0b1100, 0b1000, 0b000) => ctx.read::<ICV_IAR0_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b0000, 0b010) => {
        // 	ctx.read::<RMR>()
        // }
        (0b11, 0b100, 0b1010, 0b1000, 0b000) => ctx.read::<MECID_P0_EL2>(),
        // (0b1111, 0b000, 0b1110, 0b0000, 0b000) => {
        // 	ctx.read::<CNTFRQ>()
        // }
        (0b11, 0b000, 0b1001, 0b1010, 0b000) => ctx.read::<PMBLIMITR_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0101, 0b001) => {
        // 	ctx.read::<ERXMISC1>()
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b110) => ctx.read::<PMCEID0_EL0>(),
        // (0b1111, 0b000, 0b0001, 0b0000, 0b010) => {
        // 	ctx.read::<CPACR>()
        // }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b100) => {
        // 	ctx.read::<HCR2>()
        // }
        (0b11, 0b100, 0b0010, 0b0011, 0b010) => ctx.read::<HDBSSBR_EL2>(),
        (0b11, 0b100, 0b0100, 0b0011, 0b000) => ctx.read::<SPSR_irq>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b010) => {
        // 	ctx.read::<ICC_HPPIR1>()
        // }
        (0b10, 0b001, 0b0000, 0b1111, 0b000) => ctx.read::<TRCBBCTLR>(),
        (0b11, 0b000, 0b0000, 0b0100, 0b001) => ctx.read::<ID_AA64PFR1_EL1>(),
        // (0b1111, 0b000, 0b1001, 0b1110, 0b100) => {
        // 	ctx.read::<PMCEID2>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0011, 0b110) => {
        // 	ctx.read::<ID_MMFR5>()
        // }
        (0b11, 0b011, 0b0100, 0b0101, 0b000) => ctx.read::<DSPSR_EL0>(),
        (0b11, 0b100, 0b0010, 0b0011, 0b100) => ctx.read::<HACDBSBR_EL2>(),
        (0b11, 0b001, 0b0000, 0b0000, 0b010) => ctx.read::<CCSIDR2_EL1>(),
        (0b11, 0b000, 0b1010, 0b0010, 0b000) => ctx.read::<MAIR_EL1>(),
        (0b11, 0b000, 0b0001, 0b0000, 0b110) => ctx.read::<GCR_EL1>(),
        // (0b1111, 0b100, 0b0001, 0b0000, 0b011) => {
        // 	ctx.read::<HACTLR2>()
        // }
        // (0b1111, 0b110, 0b1100, 0b1100, 0b111) => {
        // 	ctx.read::<ICC_MGRPEN1>()
        // }
        (0b10, 0b001, 0b0000, 0b0110, 0b000) => ctx.read::<TRCAUXCTLR>(),
        // (0b1111, 0b000, 0b0101, 0b0100, 0b010) => {
        // 	ctx.read::<ERXSTATUS>()
        // }
        // (0b1110, 0b000, 0b0000, m[3:0], 0b110) => {
        // 	ctx.read::<DBGWVRn>()
        // }
        (0b11, 0b011, 0b0100, 0b0101, 0b001) => ctx.read::<DLR_EL0>(),
        (0b11, 0b000, 0b0000, 0b0001, 0b010) => ctx.read::<ID_DFR0_EL1>(),
        (0b11, 0b000, 0b0000, 0b0111, 0b011) => ctx.read::<ID_AA64MMFR3_EL1>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b110) => ctx.read::<ICV_IGRPEN0_EL1>(),
        // (0b1111, 0b000, 0b1101, 0b0011, 0b000) => {
        // 	ctx.read::<AMCNTENCLR1>()
        // }
        (0b11, 0b100, 0b0001, 0b0010, 0b010) => ctx.read::<HCRX_EL2>(),
        (0b11, 0b001, 0b0000, 0b0000, 0b000) => ctx.read::<CCSIDR_EL1>(),
        // (0b1111, 0b100, 0b0101, 0b0001, 0b001) => {
        // 	ctx.read::<HAIFSR>()
        // }
        (0b11, 0b000, 0b0000, 0b0011, 0b100) => ctx.read::<ID_PFR2_EL1>(),
        (0b10, 0b011, 0b0000, 0b0101, 0b000) => ctx.read::<DBGDTRTX_EL0>(),
        (0b10, 0b001, 0b0011, 0b0000, 0b010) => ctx.read::<TRCCIDCCTLR0>(),
        (0b10, 0b001, 0b0011, 0b0001, 0b010) => ctx.read::<TRCCIDCCTLR1>(),
        // (0b1110, 0b000, 0b0000, 0b0110, 0b000) => {
        // 	ctx.read::<DBGWFAR>()
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b110) => ctx.read::<ICC_IGRPEN0_EL1>(),
        // (0b10, 0b001, 0b1000, m[3:0], m[4]:0b10) => {
        // 	ctx.read::<BRBTGTn_EL1>()
        // }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b100) => {
        // 	ctx.read::<AMCNTENCLR0>()
        // }
        (0b11, 0b000, 0b1100, 0b0000, 0b001) => ctx.read::<RVBAR_EL1>(),
        (0b11, 0b000, 0b0010, 0b0001, 0b011) => ctx.read::<APIBKeyHi_EL1>(),
        (0b11, 0b011, 0b1101, 0b0010, 0b011) => ctx.read::<AMUSERENR_EL0>(),
        // (0b1111, 0b000, 0b1100, 0b0001, 0b000) => {
        // 	ctx.read::<ISR>()
        // }
        // (0b1111, 0b000, 0b0101, 0b0011, 0b000) => {
        // 	ctx.read::<ERRIDR>()
        // }
        // (0b1111, 0b000, 0b0001, 0b0001, 0b010) => {
        // 	ctx.read::<NSACR>()
        // }
        // (0b1111, 0b100, 0b0001, 0b0010, 0b001) => {
        // 	ctx.read::<HTRFCR>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b110) => {
        // 	ctx.read::<ID_MMFR4>()
        // }
        // (0b10, 0b011, 0b1110, 0b010:m[3], m[2:0]) => {
        // 	ctx.read::<SPMEVFILTRn_EL0>()
        // }
        (0b11, 0b000, 0b1010, 0b0010, 0b001) => ctx.read::<MAIR2_EL1>(),
        // (0b10, 0b001, 0b0011, m[2:0]:0b0, 0b001) => {
        // 	ctx.read::<TRCVMIDCVRn>()
        // }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b101) => {
        // 	ctx.read::<PMCEID3>()
        // }
        // (0b1111, 0b110, 0b1100, 0b1100, 0b101) => {
        // 	ctx.read::<ICC_MSRE>()
        // }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b000) => {
        // 	ctx.read::<PMCR>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b010) => {
        // 	ctx.read::<ICC_HPPIR0>()
        // }
        (0b11, 0b000, 0b1100, 0b1000, 0b000) => ctx.read::<ICC_IAR0_EL1>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b010) => ctx.read::<ICC_HPPIR1_EL1>(),
        (0b10, 0b000, 0b0000, 0b0000, 0b010) => ctx.read::<OSDTRRX_EL1>(),
        (0b11, 0b000, 0b0000, 0b0010, 0b010) => ctx.read::<ID_ISAR2_EL1>(),
        // (0b1111, 0b000, 0b1001, 0b1100, 0b111) => {
        // 	ctx.read::<PMCEID1>()
        // }
        (0b11, 0b110, 0b1010, 0b0001, 0b001) => ctx.read::<MAIR2_EL3>(),
        // (0b1111, 0b000, 0b0010, 0b0000, 0b011) => {
        // 	ctx.read::<TTBCR2>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b100) => {
        // 	ctx.read::<ICV_CTLR>()
        // }
        (0b11, 0b100, 0b1010, 0b0010, 0b000) => ctx.read::<MAIR_EL2>(),
        // (0b1111, 0b000, 0b1110, 0b0010, 0b000) => {
        // 	ctx.read::<CNTHPS_TVAL>()
        // }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b100) => {
        // 	ctx.read::<ERXFR2>()
        // }
        (0b11, 0b100, 0b1010, 0b1001, 0b000) => ctx.read::<VMECID_P_EL2>(),
        (0b10, 0b011, 0b1001, 0b1100, 0b101) => ctx.read::<SPMSELR_EL0>(),
        (0b10, 0b000, 0b0111, 0b1001, 0b110) => ctx.read::<DBGCLAIMCLR_EL1>(),
        // (0b10, 0b001, 0b1000, m[3:0], m[4]:0b00) => {
        // 	ctx.read::<BRBINFn_EL1>()
        // }
        (0b11, 0b011, 0b1001, 0b1101, 0b001) => ctx.read::<PMXEVTYPER_EL0>(),
        (0b10, 0b011, 0b1001, 0b1100, 0b100) => ctx.read::<SPMZR_EL0>(),
        (0b11, 0b110, 0b1100, 0b0000, 0b001) => ctx.read::<RVBAR_EL3>(),
        (0b11, 0b011, 0b0100, 0b0010, 0b101) => {
            let dit = ctx.read::<pstate::DIT>() != 0;
            let dit = if dit { 1 } else { 0 };
            let dit = dit << 24;
            dit
        }
        // (0b1111, 0b100, 0b1010, 0b0010, 0b001) => {
        // 	ctx.read::<HMAIR1>()
        // }
        (0b11, 0b100, 0b1110, 0b0010, 0b010) => ctx.read::<CNTHP_CVAL_EL2>(),
        (0b10, 0b001, 0b1001, 0b0001, 0b000) => ctx.read::<BRBINFINJ_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b0010, 0b000) => {
        // 	ctx.read::<CNTP_TVAL>()
        // }
        (0b11, 0b000, 0b0101, 0b0101, 0b001) => ctx.read::<ERXMISC1_EL1>(),
        // (0b1111, 0b100, 0b1100, 0b1011, 0b011) => {
        // 	ctx.read::<ICH_EISR>()
        // }
        // (0b10, 0b001, 0b0010, m[2:0]:0b0, 0b01:m[3]) => {
        // 	ctx.read::<TRCACATRn>()
        // }
        (0b11, 0b000, 0b1001, 0b1110, 0b010) => ctx.read::<PMINTENCLR_EL1>(),
        // (0b1111, 0b100, 0b1010, 0b0010, 0b000) => {
        // 	ctx.read::<HMAIR0>()
        // }
        (0b11, 0b100, 0b1100, 0b0000, 0b001) => ctx.read::<RVBAR_EL2>(),
        // (0b1111, 0b100, 0b1100, 0b0001, 0b001) => {
        // 	ctx.read::<VDISR>()
        // }
        (0b11, 0b011, 0b1101, 0b0010, 0b110) => ctx.read::<AMCG1IDR_EL0>(),
        (0b11, 0b000, 0b1100, 0b1000, 0b011) => ctx.read::<ICV_BPR0_EL1>(),
        (0b11, 0b111, 0b1110, 0b0010, 0b000) => ctx.read::<CNTPS_TVAL_EL1>(),
        // (0b1111, 0b000, 0b1101, 0b0000, 0b100) => {
        // 	ctx.read::<TPIDRPRW>()
        // }
        // (0b11, 0b011, 0b1101, 0b011:m[3], m[2:0]) => {
        // 	ctx.read::<AMEVTYPER0n_EL0>()
        // }
        // (0b1111, 0b100, 0b0001, 0b0000, 0b000) => {
        // 	ctx.read::<HSCTLR>()
        // }
        // (0b11, 0b011, 0b1101, 0b111:m[3], m[2:0]) => {
        // 	ctx.read::<AMEVTYPER1n_EL0>()
        // }
        (0b11, 0b100, 0b1010, 0b0110, 0b010) => ctx.read::<MPAMVPM2_EL2>(),
        // (0b1111, 0b100, 0b0010, 0b0000, 0b010) => {
        // 	ctx.read::<HTCR>()
        // }
        (0b11, 0b011, 0b0100, 0b0010, 0b000) => {
            let n = ctx.read::<pstate::N>() != 0;
            let n = if n { 1 } else { 0 };
            let n = n << 31;

            let z = ctx.read::<pstate::Z>() != 0;
            let z = if z { 1 } else { 0 };
            let z = z << 30;

            let c = ctx.read::<pstate::C>() != 0;
            let c = if c { 1 } else { 0 };
            let c = c << 29;

            let v = ctx.read::<pstate::V>() != 0;
            let v = if v { 1 } else { 0 };
            let v = v << 28;

            let nz = n | z;
            let nzc = nz | c;
            let nzcv = nzc | v;
            nzcv
        }
        (0b11, 0b110, 0b1010, 0b0010, 0b000) => ctx.read::<MAIR_EL3>(),
        (0b11, 0b000, 0b0000, 0b0011, 0b010) => ctx.read::<MVFR2_EL1>(),
        (0b11, 0b100, 0b1010, 0b0001, 0b001) => ctx.read::<MAIR2_EL2>(),
        (0b11, 0b100, 0b1010, 0b1000, 0b111) => ctx.read::<MECIDR_EL2>(),
        // (0b1111, 0b000, 0b1001, 0b1100, 0b110) => {
        // 	ctx.read::<PMCEID0>()
        // }
        (0b11, 0b100, 0b1110, 0b0100, 0b010) => ctx.read::<CNTHVS_CVAL_EL2>(),
        // (0b10, 0b001, 0b0000, 0b10:m[1:0], 0b100) => {
        // 	ctx.read::<TRCEXTINSELRn>()
        // }
        // (0b1110, 0b000, 0b0111, 0b0010, 0b111) => {
        // 	ctx.read::<DBGDEVID>()
        // }
        // (0b11, 0b000, 0b1100, 0b1000, 0b1:m[1:0]) => {
        // 	ctx.read::<ICV_AP0Rn_EL1>()
        // }
        (0b11, 0b100, 0b0010, 0b0011, 0b011) => ctx.read::<HDBSSPROD_EL2>(),
        // (0b1111, 0b000, 0b0000, 0b0001, 0b111) => {
        // 	ctx.read::<ID_MMFR3>()
        // }
        // (0b1110, 0b000, 0b0010, 0b0000, 0b000) => {
        // 	ctx.read::<DBGDSAR>()
        // }
        // (0b11, 0b100, 0b1101, 0b100:m[3], m[2:0]) => {
        // 	ctx.read::<AMEVCNTVOFF0n_EL2>()
        // }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b001) => {
        // 	ctx.read::<AMCFGR>()
        // }
        // (0b10, 0b011, 0b1110, 0b000:m[3], m[2:0]) => {
        // 	ctx.read::<SPMEVCNTRn_EL0>()
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b000) => ctx.read::<ICC_IAR1_EL1>(),
        (0b11, 0b000, 0b1100, 0b0001, 0b000) => ctx.read::<ISR_EL1>(),
        (0b11, 0b000, 0b1100, 0b1000, 0b010) => ctx.read::<ICC_HPPIR0_EL1>(),
        // (0b11, 0b100, 0b1101, 0b101:m[3], m[2:0]) => {
        // 	ctx.read::<AMEVCNTVOFF1n_EL2>()
        // }
        (0b11, 0b000, 0b0010, 0b0000, 0b010) => ctx.read::<TCR_EL1>(),
        (0b10, 0b001, 0b0000, 0b1010, 0b000) => ctx.read::<TRCRSR>(),
        (0b11, 0b000, 0b0000, 0b0010, 0b011) => ctx.read::<ID_ISAR3_EL1>(),
        (0b10, 0b001, 0b0111, 0b0010, 0b111) => ctx.read::<TRCDEVID>(),
        // (0b1111, 0b000, 0b0101, 0b0000, 0b000) => {
        // 	ctx.read::<DFSR>()
        // }
        (0b11, 0b011, 0b0100, 0b0010, 0b010) => {
            let sm = ctx.read::<pstate::SM>() != 0;
            let sm = if sm { 1 } else { 0 };
            let sm = sm << 0;

            let za = ctx.read::<pstate::ZA>() != 0;
            let za = if za { 1 } else { 0 };
            let za = za << 1;

            let svcr = sm | za;
            svcr
        }
        (0b11, 0b000, 0b0101, 0b0101, 0b000) => ctx.read::<ERXMISC0_EL1>(),
        (0b11, 0b000, 0b1100, 0b1011, 0b001) => ctx.read::<ICV_DIR_EL1>(),
        (0b11, 0b110, 0b1100, 0b1100, 0b111) => ctx.read::<ICC_IGRPEN1_EL3>(),
        (0b11, 0b100, 0b1010, 0b0100, 0b001) => ctx.read::<MPAMVPMV_EL2>(),
        // (0b1111, 0b100, 0b0110, 0b0000, 0b010) => {
        // 	ctx.read::<HIFAR>()
        // }
        // (0b1111, 0b000, 0b1100, 0b0000, 0b001) => {
        // 	ctx.read::<MVBAR>()
        // }
        (0b11, 0b011, 0b0100, 0b0010, 0b111) => {
            let tco = ctx.read::<pstate::TCO>() != 0;
            let tco = if tco { 1 } else { 0 };
            let tco = tco << 25;
            tco
        }
        (0b11, 0b000, 0b1001, 0b1110, 0b101) => ctx.read::<PMECR_EL1>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b011) => ctx.read::<ICV_BPR1_EL1>(),
        // (0b1111, 0b000, 0b0000, 0b0001, 0b011) => {
        // 	ctx.read::<ID_AFR0>()
        // }
        (0b11, 0b000, 0b0110, 0b0000, 0b101) => ctx.read::<PFAR_EL1>(),
        // (0b10, 0b001, 0b0000, 0b00:m[1:0], 0b101) => {
        // 	ctx.read::<TRCCNTRLDVRn>()
        // }
        (0b11, 0b011, 0b1110, 0b0010, 0b001) => ctx.read::<CNTP_CTL_EL0>(),
        // (0b1111, 0b000, 0b1100, 0b0000, 0b000) => {
        // 	ctx.read::<VBAR>()
        // }
        (0b11, 0b011, 0b0100, 0b0010, 0b001) => {
            let d = ctx.read::<pstate::D>() != 0;
            let d = if d { 1 } else { 0 };
            let d = d << 9;

            let a = ctx.read::<pstate::A>() != 0;
            let a = if a { 1 } else { 0 };
            let a = a << 8;

            let i = ctx.read::<pstate::I>() != 0;
            let i = if i { 1 } else { 0 };
            let i = i << 7;

            let f = ctx.read::<pstate::F>() != 0;
            let f = if f { 1 } else { 0 };
            let f = f << 6;

            let da = d | a;
            let dai = da | i;
            let daif = dai | f;
            daif
        }
        // (0b1110, 0b000, 0b0000, 0b0010, 0b000) => {
        // 	ctx.read::<DBGDCCINT>()
        // }
        (0b11, 0b000, 0b0001, 0b0010, 0b100) => ctx.read::<SMPRI_EL1>(),
        (0b11, 0b011, 0b1110, 0b1111, 0b111) => ctx.read::<PMCCFILTR_EL0>(),
        (0b11, 0b100, 0b1010, 0b0110, 0b011) => ctx.read::<MPAMVPM3_EL2>(),
        // (0b1111, 0b000, 0b0000, 0b0001, 0b110) => {
        // 	ctx.read::<ID_MMFR2>()
        // }
        (0b10, 0b001, 0b0000, 0b0001, 0b010) => ctx.read::<TRCVIIECTLR>(),
        (0b10, 0b000, 0b0001, 0b0000, 0b100) => ctx.read::<OSLAR_EL1>(),
        (0b11, 0b011, 0b0000, 0b0000, 0b111) => ctx.read::<DCZID_EL0>(),
        (0b11, 0b100, 0b0001, 0b0001, 0b111) => ctx.read::<HACR_EL2>(),
        (0b11, 0b011, 0b0000, 0b0000, 0b001) => ctx.read::<CTR_EL0>(),
        (0b11, 0b000, 0b0000, 0b0001, 0b011) => ctx.read::<ID_AFR0_EL1>(),
        // (0b1111, 0b000, 0b1101, 0b0010, 0b011) => {
        // 	ctx.read::<AMUSERENR>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b100) => {
        // 	ctx.read::<ID_MMFR0>()
        // }
        (0b10, 0b011, 0b1001, 0b1100, 0b001) => ctx.read::<SPMCNTENSET_EL0>(),
        // (0b1111, 0b100, 0b0000, 0b0000, 0b101) => {
        // 	ctx.read::<VMPIDR>()
        // }
        (0b11, 0b000, 0b0000, 0b0101, 0b010) => ctx.read::<ID_AA64DFR2_EL1>(),
        (0b11, 0b000, 0b0111, 0b0100, 0b000) => ctx.read::<PAR_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1000, 0b001) => {
        // 	ctx.read::<ICC_EOIR0>()
        // }
        (0b11, 0b000, 0b1001, 0b1010, 0b100) => ctx.read::<PMSDSFR_EL1>(),
        (0b11, 0b000, 0b0000, 0b0000, 0b101) => ctx.read::<MPIDR_EL1>(),
        (0b10, 0b001, 0b0000, 0b1110, 0b000) => ctx.read::<TRCCCCTLR>(),
        (0b11, 0b100, 0b0010, 0b0000, 0b010) => ctx.read::<TCR_EL2>(),
        (0b11, 0b001, 0b0000, 0b0000, 0b111) => ctx.read::<AIDR_EL1>(),
        // (0b1111, 0b000, 0b0001, 0b0011, 0b001) => {
        // 	ctx.read::<SDCR>()
        // }
        (0b11, 0b000, 0b0000, 0b0100, 0b000) => ctx.read::<ID_AA64PFR0_EL1>(),
        (0b11, 0b000, 0b0010, 0b0010, 0b010) => ctx.read::<APDBKeyLo_EL1>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b111) => ctx.read::<ICV_IGRPEN1_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b0011, 0b001) => {
        // 	ctx.read::<CNTV_CTL>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b001) => {
        // 	ctx.read::<ICV_EOIR1>()
        // }
        (0b11, 0b000, 0b1001, 0b1011, 0b111) => ctx.read::<TRBIDR_EL1>(),
        // (0b1110, 0b111, 0b0001, 0b0000, 0b000) => {
        // 	ctx.read::<JOSCR>()
        // }
        // (0b1110, 0b111, 0b0010, 0b0000, 0b000) => {
        // 	ctx.read::<JMCR>()
        // }
        // (0b1110, 0b000, 0b0000, 0b0111, 0b000) => {
        // 	ctx.read::<DBGVCR>()
        // }
        (0b11, 0b100, 0b0001, 0b0010, 0b101) => ctx.read::<SMPRIMAP_EL2>(),
        // (0b1111, 0b100, 0b1100, 0b1011, 0b001) => {
        // 	ctx.read::<ICH_VTR>()
        // }
        (0b10, 0b000, 0b1001, 0b1101, 0b111) => ctx.read::<SPMCFGR_EL1>(),
        (0b11, 0b100, 0b0010, 0b0001, 0b010) => ctx.read::<VTCR_EL2>(),
        (0b10, 0b000, 0b0000, 0b0110, 0b010) => ctx.read::<OSECCR_EL1>(),
        (0b11, 0b000, 0b0000, 0b0111, 0b010) => ctx.read::<ID_AA64MMFR2_EL1>(),
        (0b11, 0b000, 0b0000, 0b0011, 0b101) => ctx.read::<ID_DFR1_EL1>(),
        (0b11, 0b100, 0b1110, 0b0010, 0b001) => ctx.read::<CNTHP_CTL_EL2>(),
        // (0b1111, 0b000, 0b0010, 0b0000, 0b010) => {
        // 	ctx.read::<TTBCR>()
        // }
        // (0b11, 0b000, 0b1100, 0b1000, 0b1:m[1:0]) => {
        // 	ctx.read::<ICC_AP0Rn_EL1>()
        // }
        (0b11, 0b100, 0b0110, 0b0000, 0b101) => ctx.read::<PFAR_EL2>(),
        // (0b1111, 0b001, 0b0000, 0b0000, 0b000) => {
        // 	ctx.read::<CCSIDR>()
        // }
        (0b10, 0b011, 0b0000, 0b0100, 0b000) => ctx.read::<DBGDTR_EL0>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b111) => ctx.read::<ICC_IGRPEN1_EL1>(),
        // (0b1111, 0b011, 0b0100, 0b0101, 0b000) => {
        // 	ctx.read::<DSPSR>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b001) => {
        // 	ctx.read::<ICV_EOIR0>()
        // }
        (0b11, 0b100, 0b1110, 0b0000, 0b110) => ctx.read::<CNTPOFF_EL2>(),
        (0b10, 0b001, 0b0000, 0b0000, 0b111) => ctx.read::<TRCIMSPEC0>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b100) => {
        // 	ctx.read::<ICC_CTLR>()
        // }
        (0b11, 0b100, 0b0001, 0b0001, 0b100) => ctx.read::<HFGRTR_EL2>(),
        (0b11, 0b110, 0b0010, 0b0000, 0b010) => ctx.read::<TCR_EL3>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b001) => {
        // 	ctx.read::<ICC_EOIR1>()
        // }
        (0b11, 0b011, 0b0100, 0b0100, 0b001) => ctx.read::<FPSR>(),
        (0b11, 0b000, 0b0101, 0b0100, 0b110) => ctx.read::<ERXPFGCDN_EL1>(),
        (0b11, 0b000, 0b1001, 0b1001, 0b011) => ctx.read::<PMSIRR_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b0011, 0b000) => {
        // 	ctx.read::<CNTHVS_TVAL>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b101) => {
        // 	ctx.read::<ID_MMFR1>()
        // }
        (0b11, 0b100, 0b0010, 0b0011, 0b101) => ctx.read::<HACDBSCONS_EL2>(),
        (0b11, 0b100, 0b1110, 0b0011, 0b010) => ctx.read::<CNTHV_CVAL_EL2>(),
        (0b11, 0b100, 0b0010, 0b0000, 0b000) => ctx.read::<TTBR0_EL2>(),
        (0b11, 0b000, 0b1010, 0b0100, 0b000) => ctx.read::<LORSA_EL1>(),
        (0b11, 0b110, 0b1100, 0b1100, 0b100) => ctx.read::<ICC_CTLR_EL3>(),
        (0b11, 0b000, 0b0001, 0b0010, 0b001) => ctx.read::<TRFCR_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b000) => {
        // 	ctx.read::<ICV_IAR1>()
        // }
        (0b11, 0b100, 0b1010, 0b0011, 0b001) => ctx.read::<AMAIR2_EL2>(),
        (0b10, 0b001, 0b1001, 0b0010, 0b000) => ctx.read::<BRBIDR0_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b010) => {
        // 	ctx.read::<ICV_HPPIR1>()
        // }
        (0b11, 0b000, 0b1001, 0b1001, 0b010) => ctx.read::<PMSICR_EL1>(),
        // (0b1111, 0b000, 0b1001, 0b1100, 0b010) => {
        // 	ctx.read::<PMCNTENCLR>()
        // }
        (0b11, 0b000, 0b0001, 0b0000, 0b000) => ctx.read::<SCTLR_EL1>() & !(1 << 17),
        // (0b1111, 0b000, 0b0000, 0b0010, 0b010) => {
        // 	ctx.read::<ID_ISAR2>()
        // }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b100) => {
        // 	ctx.read::<PMSWINC>()
        // }
        (0b11, 0b000, 0b1010, 0b0101, 0b000) => ctx.read::<MPAM1_EL1>(),
        (0b11, 0b100, 0b1100, 0b1011, 0b000) => ctx.read::<ICH_HCR_EL2>(),
        (0b11, 0b100, 0b0101, 0b0000, 0b001) => ctx.read::<IFSR32_EL2>(),
        (0b11, 0b000, 0b0010, 0b0001, 0b001) => ctx.read::<APIAKeyHi_EL1>(),
        (0b11, 0b110, 0b0001, 0b0001, 0b010) => ctx.read::<CPTR_EL3>(),
        (0b10, 0b000, 0b1001, 0b1101, 0b101) => ctx.read::<SPMDEVARCH_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b111) => {
        // 	ctx.read::<ICV_IGRPEN1>()
        // }
        (0b11, 0b100, 0b1100, 0b1011, 0b111) => ctx.read::<ICH_VMCR_EL2>(),
        (0b10, 0b001, 0b0000, 0b0001, 0b001) => ctx.read::<TRCQCTLR>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b110) => {
        // 	ctx.read::<ICV_IGRPEN0>()
        // }
        (0b11, 0b000, 0b1001, 0b1110, 0b110) => ctx.read::<PMMIR_EL1>(),
        (0b11, 0b000, 0b0101, 0b0100, 0b000) => ctx.read::<ERXFR_EL1>(),
        (0b11, 0b100, 0b0001, 0b0001, 0b010) => ctx.read::<CPTR_EL2>(),
        (0b11, 0b100, 0b1110, 0b0101, 0b010) => ctx.read::<CNTHPS_CVAL_EL2>(),
        (0b11, 0b100, 0b0010, 0b0110, 0b000) => ctx.read::<VSTTBR_EL2>(),
        // (0b1111, 0b000, 0b1010, 0b0010, 0b001) => {
        // 	ctx.read::<NMRR>()
        // }
        (0b10, 0b110, 0b1001, 0b1110, 0b111) => ctx.read::<SPMROOTCR_EL3>(),
        // (0b1111, 0b000, 0b0000, 0b0011, 0b100) => {
        // 	ctx.read::<ID_PFR2>()
        // }
        (0b10, 0b001, 0b0000, 0b0011, 0b000) => ctx.read::<TRCSTATR>(),
        (0b11, 0b100, 0b0001, 0b0001, 0b011) => ctx.read::<HSTR_EL2>(),
        (0b11, 0b000, 0b0010, 0b0011, 0b000) => ctx.read::<APGAKeyLo_EL1>(),
        // (0b1111, 0b000, 0b0000, 0b0010, 0b011) => {
        // 	ctx.read::<ID_ISAR3>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b010) => {
        // 	ctx.read::<ICV_HPPIR0>()
        // }
        (0b11, 0b000, 0b1001, 0b1011, 0b010) => ctx.read::<TRBBASER_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b11:m[4:3], m[2:0]) => {
        // 	ctx.read::<PMEVTYPERn>()
        // }
        (0b11, 0b000, 0b0000, 0b0001, 0b111) => ctx.read::<ID_MMFR3_EL1>(),
        (0b11, 0b011, 0b1001, 0b0100, 0b000) => ctx.read::<PMICNTR_EL0>(),
        (0b11, 0b000, 0b1001, 0b1110, 0b111) => ctx.read::<PMIAR_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0001, 0b000) => {
        // 	ctx.read::<ADFSR>()
        // }
        (0b11, 0b000, 0b1100, 0b0000, 0b010) => ctx.read::<RMR_EL1>(),
        (0b11, 0b110, 0b1010, 0b0011, 0b001) => ctx.read::<AMAIR2_EL3>(),
        (0b11, 0b000, 0b1010, 0b0010, 0b010) => ctx.read::<PIRE0_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1000, 0b000) => {
        // 	ctx.read::<ICV_IAR0>()
        // }
        (0b11, 0b110, 0b0010, 0b0000, 0b000) => ctx.read::<TTBR0_EL3>(),
        (0b11, 0b100, 0b0001, 0b0010, 0b001) => ctx.read::<TRFCR_EL2>(),
        // (0b1111, 0b000, 0b0000, 0b0011, 0b101) => {
        // 	ctx.read::<ID_DFR1>()
        // }
        (0b11, 0b000, 0b0010, 0b0000, 0b000) => ctx.read::<TTBR0_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1000, 0b011) => {
        // 	ctx.read::<ICV_BPR0>()
        // }
        (0b11, 0b000, 0b1010, 0b0011, 0b001) => ctx.read::<AMAIR2_EL1>(),
        (0b11, 0b000, 0b1101, 0b0000, 0b011) => ctx.read::<RCWSMASK_EL1>(),
        // (0b1111, 0b100, 0b1100, 0b0000, 0b000) => {
        // 	ctx.read::<HVBAR>()
        // }
        (0b11, 0b110, 0b1100, 0b0000, 0b010) => ctx.read::<RMR_EL3>(),
        (0b11, 0b011, 0b1101, 0b0010, 0b001) => ctx.read::<AMCFGR_EL0>(),
        // (0b1111, 0b100, 0b0000, 0b0000, 0b000) => {
        // 	ctx.read::<VPIDR>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b1:m[1:0]) => {
        // 	ctx.read::<ICV_AP0Rn>()
        // }
        // (0b1111, 0b000, 0b0001, 0b0000, 0b001) => {
        // 	ctx.read::<ACTLR>()
        // }
        (0b11, 0b000, 0b1010, 0b0100, 0b111) => ctx.read::<LORID_EL1>(),
        (0b11, 0b100, 0b0001, 0b0000, 0b000) => ctx.read::<SCTLR_EL2>() & !(1 << 17),
        (0b11, 0b000, 0b0100, 0b0010, 0b011) => {
            let pan = ctx.read::<pstate::PAN>() != 0;
            let pan = if pan { 1 } else { 0 };
            let pan = pan << 22;
            pan
        }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b001) => {
        // 	ctx.read::<ID_ISAR1>()
        // }
        (0b11, 0b110, 0b0001, 0b0001, 0b001) => ctx.read::<SDER32_EL3>(),
        // (0b1110, 0b000, 0b0000, 0b0000, 0b000) => {
        // 	ctx.read::<DBGDIDR>()
        // }
        // (0b1111, 0b100, 0b1110, 0b0001, 0b000) => {
        // 	ctx.read::<CNTHCTL>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b000) => {
        // 	ctx.read::<ID_PFR0>()
        // }
        (0b10, 0b011, 0b1001, 0b1100, 0b010) => ctx.read::<SPMCNTENCLR_EL0>(),
        (0b10, 0b001, 0b1001, 0b0000, 0b010) => ctx.read::<BRBTS_EL1>(),
        // (0b1111, 0b000, 0b0001, 0b0000, 0b011) => {
        // 	ctx.read::<ACTLR2>()
        // }
        (0b11, 0b110, 0b0110, 0b0000, 0b101) => ctx.read::<MFAR_EL3>(),
        (0b10, 0b001, 0b0000, 0b1000, 0b000) => ctx.read::<TRCEVENTCTL0R>(),
        (0b10, 0b000, 0b1110, 0b1100, 0b000) => ctx.read::<PMICNTSVR_EL1>(),
        (0b11, 0b100, 0b0010, 0b0110, 0b010) => ctx.read::<VSTCR_EL2>(),
        (0b11, 0b100, 0b1010, 0b0110, 0b110) => ctx.read::<MPAMVPM6_EL2>(),
        // (0b1111, 0b000, 0b1100, 0b1000, 0b1:m[1:0]) => {
        // 	ctx.read::<ICC_AP0Rn>()
        // }
        (0b10, 0b000, 0b0111, 0b1110, 0b110) => ctx.read::<DBGAUTHSTATUS_EL1>(),
        (0b11, 0b011, 0b1110, 0b0000, 0b110) => {
            // ctx.read::<CNTVCTSS_EL0>()
            ctx.physical_count()
        }
        (0b11, 0b000, 0b0000, 0b0110, 0b010) => ctx.read::<ID_AA64ISAR2_EL1>(),
        // (0b1110, 0b000, 0b0111, 0b1110, 0b110) => {
        // 	ctx.read::<DBGAUTHSTATUS>()
        // }
        (0b11, 0b100, 0b1010, 0b1001, 0b001) => ctx.read::<VMECID_A_EL2>(),
        (0b11, 0b000, 0b0000, 0b0010, 0b111) => ctx.read::<ID_ISAR6_EL1>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b010) => ctx.read::<ICV_HPPIR1_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b0011, 0b001) => {
        // 	ctx.read::<CNTHVS_CTL>()
        // }
        // (0b1111, 0b000, 0b0110, 0b0000, 0b010) => {
        // 	ctx.read::<IFAR>()
        // }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b000) => {
        // 	ctx.read::<PMUSERENR>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b001) => {
        // 	ctx.read::<ID_PFR1>()
        // }
        // (0b1110, 0b000, 0b0001, 0b0001, 0b100) => {
        // 	ctx.read::<DBGOSLSR>()
        // }
        (0b11, 0b000, 0b0000, 0b0000, 0b110) => ctx.read::<REVIDR_EL1>(),
        (0b11, 0b100, 0b0001, 0b0001, 0b101) => ctx.read::<HFGWTR_EL2>(),
        // (0b1111, 0b000, 0b0000, 0b0010, 0b000) => {
        // 	ctx.read::<ID_ISAR0>()
        // }
        (0b11, 0b100, 0b0001, 0b0011, 0b001) => ctx.read::<SDER32_EL2>(),
        (0b11, 0b110, 0b0001, 0b0001, 0b101) => ctx.read::<FGWTE3_EL3>(),
        (0b11, 0b011, 0b1001, 0b1101, 0b010) => ctx.read::<PMXEVCNTR_EL0>(),
        // (0b10, 0b011, 0b1110, 0b001:m[3], m[2:0]) => {
        // 	ctx.read::<SPMEVTYPERn_EL0>()
        // }
        (0b11, 0b110, 0b0001, 0b0000, 0b000) => {
            ctx.read::<SCTLR_EL3>() & !(1 << 17) & !(0b11 << 38)
        }
        (0b11, 0b100, 0b0011, 0b0001, 0b001) => ctx.read::<HDFGWTR2_EL2>(),
        (0b11, 0b100, 0b1100, 0b0000, 0b010) => ctx.read::<RMR_EL2>(),
        // (0b1111, 0b000, 0b1001, 0b1100, 0b001) => {
        // 	ctx.read::<PMCNTENSET>()
        // }
        (0b11, 0b011, 0b1110, 0b0011, 0b010) => ctx.read::<CNTV_CVAL_EL0>(),
        (0b11, 0b000, 0b0101, 0b0011, 0b001) => ctx.read::<ERRSELR_EL1>(),
        (0b11, 0b110, 0b0010, 0b0001, 0b100) => ctx.read::<GPTBR_EL3>(),
        (0b11, 0b011, 0b1001, 0b1100, 0b100) => ctx.read::<PMSWINC_EL0>(),
        (0b11, 0b100, 0b1010, 0b0010, 0b010) => ctx.read::<PIRE0_EL2>(),
        (0b11, 0b000, 0b1100, 0b1011, 0b011) => ctx.read::<ICC_RPR_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b011) => {
        // 	ctx.read::<ICV_BPR1>()
        // }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b010) => {
        // 	ctx.read::<HCPTR>()
        // }
        (0b10, 0b001, 0b0001, 0b0001, 0b100) => ctx.read::<TRCOSLSR>(),
        (0b11, 0b000, 0b0100, 0b0110, 0b000) => ctx.read::<ICC_PMR_EL1>(),
        // (0b1111, 0b000, 0b0000, 0b0001, 0b010) => {
        // 	ctx.read::<ID_DFR0>()
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b100) => ctx.read::<ICC_CTLR_EL1>(),
        // (0b10, 0b001, 0b1000, m[3:0], m[4]:0b01) => {
        // 	ctx.read::<BRBSRCn_EL1>()
        // }
        (0b11, 0b000, 0b0000, 0b0100, 0b101) => ctx.read::<ID_AA64SMFR0_EL1>(),
        // (0b1111, 0b100, 0b1101, 0b0000, 0b010) => {
        // 	ctx.read::<HTPIDR>()
        // }
        (0b11, 0b110, 0b0001, 0b0010, 0b000) => ctx.read::<ZCR_EL3>(),
        (0b11, 0b000, 0b0101, 0b0100, 0b010) => ctx.read::<ERXSTATUS_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1000, 0b000) => {
        // 	ctx.read::<ICC_IAR0>()
        // }
        (0b11, 0b000, 0b0010, 0b0000, 0b001) => ctx.read::<TTBR1_EL1>(),
        (0b11, 0b100, 0b1110, 0b0000, 0b011) => ctx.read::<CNTVOFF_EL2>(),
        (0b11, 0b000, 0b0010, 0b0000, 0b011) => ctx.read::<TCR2_EL1>(),
        (0b11, 0b100, 0b0001, 0b0001, 0b001) => ctx.read::<MDCR_EL2>(),
        // (0b1111, 0b100, 0b1100, 0b1000, 0b0:m[1:0]) => {
        // 	ctx.read::<ICH_AP0Rn>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1011, 0b011) => {
        // 	ctx.read::<ICC_RPR>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b100) => {
        // 	ctx.read::<ID_ISAR4>()
        // }
        (0b10, 0b001, 0b0000, 0b0010, 0b001) => ctx.read::<TRCITEEDCR>(),
        (0b10, 0b110, 0b1001, 0b1101, 0b011) => ctx.read::<SPMACCESSR_EL3>(),
        // (0b1111, 0b100, 0b1100, 0b0000, 0b010) => {
        // 	ctx.read::<HRMR>()
        // }
        (0b11, 0b100, 0b1010, 0b0110, 0b111) => ctx.read::<MPAMVPM7_EL2>(),
        (0b11, 0b100, 0b1010, 0b0010, 0b100) => ctx.read::<POR_EL2>(),
        (0b11, 0b110, 0b0010, 0b0101, 0b001) => ctx.read::<GCSPR_EL3>(),
        // (0b1110, 0b000, 0b0000, m[3:0], 0b111) => {
        // 	ctx.read::<DBGWCRn>()
        // }
        (0b11, 0b100, 0b0110, 0b0000, 0b000) => ctx.read::<FAR_EL2>(),
        // (0b1111, 0b000, 0b1100, 0b1011, 0b011) => {
        // 	ctx.read::<ICV_RPR>()
        // }
        (0b11, 0b100, 0b0101, 0b0110, 0b000) => ctx.read::<TFSR_EL2>(),
        // (0b10, 0b001, 0b0000, 0b01:m[1:0], 0b101) => {
        // 	ctx.read::<TRCCNTCTLRn>()
        // }
        (0b11, 0b000, 0b1100, 0b1000, 0b010) => ctx.read::<ICV_HPPIR0_EL1>(),
        (0b11, 0b100, 0b0011, 0b0001, 0b000) => ctx.read::<HDFGRTR2_EL2>(),
        (0b11, 0b110, 0b0101, 0b0110, 0b000) => ctx.read::<TFSR_EL3>(),
        (0b10, 0b001, 0b0000, 0b1001, 0b000) => ctx.read::<TRCEVENTCTL1R>(),
        (0b11, 0b110, 0b0110, 0b0000, 0b000) => ctx.read::<FAR_EL3>(),
        (0b11, 0b100, 0b0010, 0b0101, 0b001) => ctx.read::<GCSPR_EL2>(),
        (0b11, 0b110, 0b1010, 0b0010, 0b100) => ctx.read::<POR_EL3>(),
        (0b11, 0b100, 0b0001, 0b0001, 0b110) => ctx.read::<HFGITR_EL2>(),
        (0b11, 0b000, 0b0000, 0b0110, 0b011) => ctx.read::<ID_AA64ISAR3_EL1>(),
        // (0b1111, 0b000, 0b0000, 0b0000, 0b010) => {
        // 	ctx.read::<TCMTR>()
        // }
        (0b10, 0b100, 0b1001, 0b1101, 0b011) => ctx.read::<SPMACCESSR_EL2>(),
        // (0b1111, 0b011, 0b0100, 0b0101, 0b001) => {
        // 	ctx.read::<DLR>()
        // }
        (0b10, 0b000, 0b0000, 0b0101, 0b010) => ctx.read::<MDSTEPOP_EL1>(),
        // (0b1111, 0b000, 0b1101, 0b0010, 0b010) => {
        // 	ctx.read::<AMCGCR>()
        // }
        // (0b1111, 0b100, 0b1100, 0b110:m[3], m[2:0]) => {
        // 	ctx.read::<ICH_LRn>()
        // }
        (0b11, 0b000, 0b0101, 0b0100, 0b011) => ctx.read::<ERXADDR_EL1>(),
        // (0b1111, 0b000, 0b0000, 0b0010, 0b101) => {
        // 	ctx.read::<ID_ISAR5>()
        // }
        (0b11, 0b000, 0b1001, 0b1110, 0b100) => ctx.read::<PMUACR_EL1>(),
        (0b11, 0b110, 0b0001, 0b0011, 0b001) => ctx.read::<MDCR_EL3>(),
        (0b11, 0b000, 0b1001, 0b1001, 0b000) => ctx.read::<PMSCR_EL1>(),
        // (0b10, 0b001, 0b0001, 0b1:m[2:0], 0b010) => {
        // 	ctx.read::<TRCSSCSRn>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b000) => {
        // 	ctx.read::<ICC_IAR1>()
        // }
        (0b11, 0b100, 0b0001, 0b0010, 0b000) => ctx.read::<ZCR_EL2>(),
        (0b11, 0b000, 0b0101, 0b0100, 0b001) => ctx.read::<ERXCTLR_EL1>(),
        (0b10, 0b000, 0b0111, 0b1000, 0b110) => ctx.read::<DBGCLAIMSET_EL1>(),
        (0b10, 0b100, 0b0000, 0b0111, 0b000) => ctx.read::<DBGVCR32_EL2>(),
        (0b11, 0b000, 0b1101, 0b0000, 0b110) => ctx.read::<RCWMASK_EL1>(),
        (0b10, 0b001, 0b0000, 0b0110, 0b100) => ctx.read::<TRCSEQRSTEVR>(),
        // (0b1111, 0b000, 0b1101, 0b0000, 0b000) => {
        // 	ctx.read::<FCSEIDR>()
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b011) => {
        // 	ctx.read::<ICC_BPR1>()
        // }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b000) => {
        // 	ctx.read::<AMCR>()
        // }
        (0b11, 0b100, 0b0010, 0b0000, 0b011) => ctx.read::<TCR2_EL2>(),
        (0b11, 0b100, 0b0010, 0b0000, 0b001) => ctx.read::<TTBR1_EL2>(),
        // (0b10, 0b001, 0b0000, 0b0:m[2:0], 0b111) => {
        // 	ctx.read::<TRCIMSPECn>()
        // }
        (0b11, 0b000, 0b1001, 0b1010, 0b111) => ctx.read::<PMBIDR_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b0011, 0b001) => {
        // 	ctx.read::<CNTHV_CTL>()
        // }
        (0b11, 0b011, 0b1001, 0b1101, 0b100) => ctx.read::<PMZR_EL0>(),
        // (0b1111, 0b000, 0b0001, 0b0001, 0b000) => {
        // 	ctx.read::<SCR>()
        // }
        // (0b1110, 0b000, 0b0000, 0b0101, 0b000) => {
        // 	ctx.read::<DBGDTRTXint>()
        // }
        (0b11, 0b000, 0b1010, 0b0010, 0b100) => ctx.read::<POR_EL1>(),
        // (0b1110, 0b000, 0b0000, 0b0011, 0b010) => {
        // 	ctx.read::<DBGDTRTXext>()
        // }
        (0b11, 0b011, 0b0010, 0b0101, 0b001) => ctx.read::<GCSPR_EL0>(),
        // (0b1111, 0b100, 0b0001, 0b0001, 0b000) => {
        // 	ctx.read::<HCR>()
        // }
        (0b11, 0b000, 0b0110, 0b0000, 0b000) => ctx.read::<FAR_EL1>(),
        (0b11, 0b000, 0b1010, 0b0101, 0b001) => ctx.read::<MPAM0_EL1>(),
        (0b11, 0b011, 0b1110, 0b0010, 0b010) => ctx.read::<CNTP_CVAL_EL0>(),
        // (0b1111, 0b000, 0b0000, 0b0000, 0b011) => {
        // 	ctx.read::<TLBTR>()
        // }
        (0b11, 0b000, 0b0101, 0b0110, 0b000) => ctx.read::<TFSR_EL1>(),
        // (0b11, 0b100, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.read::<ICH_AP1Rn_EL2>()
        // }
        // (0b1111, 0b000, 0b0011, 0b0000, 0b000) => {
        // 	ctx.read::<DACR>()
        // }
        (0b11, 0b000, 0b0010, 0b0010, 0b000) => ctx.read::<APDAKeyLo_EL1>(),
        // (0b1111, 0b100, 0b1100, 0b1001, 0b101) => {
        // 	ctx.read::<ICC_HSRE>()
        // }
        (0b11, 0b000, 0b0100, 0b0010, 0b010) => {
            let el = ctx.curr_el();
            el.as_num() << 2
        }
        (0b11, 0b100, 0b1010, 0b0010, 0b101) => ctx.read::<S2PIR_EL2>(),
        (0b11, 0b000, 0b0010, 0b0101, 0b001) => ctx.read::<GCSPR_EL1>(),
        // (0b1111, 0b100, 0b1110, 0b0010, 0b000) => {
        // 	ctx.read::<CNTHP_TVAL>()
        // }
        (0b11, 0b011, 0b1010, 0b0010, 0b100) => ctx.read::<POR_EL0>(),
        // (0b10, 0b001, 0b0000, 0b10:m[1:0], 0b101) => {
        // 	ctx.read::<TRCCNTVRn>()
        // }
        (0b10, 0b000, 0b1001, 0b1101, 0b011) => ctx.read::<SPMACCESSR_EL1>(),
        (0b11, 0b000, 0b0101, 0b0110, 0b001) => ctx.read::<TFSRE0_EL1>(),
        // (0b1111, 0b000, 0b0000, 0b0010, 0b111) => {
        // 	ctx.read::<ID_ISAR6>()
        // }
        (0b11, 0b011, 0b1101, 0b0010, 0b000) => ctx.read::<AMCR_EL0>(),
        (0b11, 0b000, 0b1001, 0b1110, 0b001) => ctx.read::<PMINTENSET_EL1>(),
        (0b11, 0b100, 0b1001, 0b1001, 0b000) => ctx.read::<PMSCR_EL2>(),
        // (0b1111, 0b000, 0b1100, 0b1000, 0b011) => {
        // 	ctx.read::<ICC_BPR0>()
        // }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b111) => {
        // 	ctx.read::<HACR>()
        // }
        (0b11, 0b000, 0b0001, 0b0010, 0b000) => ctx.read::<ZCR_EL1>(),
        (0b11, 0b000, 0b0000, 0b0001, 0b110) => ctx.read::<ID_MMFR2_EL1>(),
        (0b11, 0b000, 0b1101, 0b0000, 0b001) => ctx.read::<CONTEXTIDR_EL1>(),
        (0b11, 0b100, 0b1010, 0b0110, 0b101) => ctx.read::<MPAMVPM5_EL2>(),
        (0b11, 0b000, 0b0000, 0b0000, 0b000) => ctx.read::<MIDR_EL1>(),
        (0b11, 0b000, 0b1010, 0b0100, 0b010) => ctx.read::<LORN_EL1>(),
        (0b11, 0b100, 0b0101, 0b0010, 0b011) => ctx.read::<VSESR_EL2>(),
        (0b11, 0b100, 0b0101, 0b0001, 0b001) => ctx.read::<AFSR1_EL2>(),
        (0b11, 0b100, 0b1010, 0b1000, 0b001) => ctx.read::<MECID_A0_EL2>(),
        (0b11, 0b100, 0b1010, 0b0101, 0b000) => ctx.read::<MPAM2_EL2>(),
        (0b11, 0b100, 0b1100, 0b1011, 0b011) => ctx.read::<ICH_EISR_EL2>(),
        (0b10, 0b000, 0b1001, 0b1101, 0b110) => ctx.read::<SPMDEVAFF_EL1>(),
        // (0b10, 0b001, 0b0010, m[2:0]:0b0, 0b00:m[3]) => {
        // 	ctx.read::<TRCACVRn>()
        // }
        (0b11, 0b110, 0b0100, 0b0001, 0b000) => ctx.read::<SP_EL2>(),
        // (0b1111, 0b000, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.read::<ICV_AP1Rn>()
        // }
        (0b11, 0b000, 0b0010, 0b0010, 0b001) => ctx.read::<APDAKeyHi_EL1>(),
        // (0b1111, 0b100, 0b0001, 0b0000, 0b001) => {
        // 	ctx.read::<HACTLR>()
        // }
        // (0b10, 0b000, 0b0000, m[3:0], 0b100) => {
        // 	ctx.read::<DBGBVRn_EL1>()
        // }
        (0b11, 0b100, 0b1110, 0b0100, 0b001) => ctx.read::<CNTHVS_CTL_EL2>(),
        (0b11, 0b100, 0b1010, 0b0010, 0b011) => ctx.read::<PIR_EL2>(),
        (0b11, 0b000, 0b0100, 0b0011, 0b001) => {
            let pm = ctx.read::<pstate::PM>() != 0;
            let pm = if pm { 1 } else { 0 };
            let pm = pm << 32;
            pm
        }
        (0b11, 0b110, 0b1010, 0b0010, 0b011) => ctx.read::<PIR_EL3>(),
        (0b11, 0b110, 0b1010, 0b1010, 0b001) => ctx.read::<MECID_RL_A_EL3>(),
        // (0b1111, 0b000, 0b1001, 0b1101, 0b000) => {
        // 	ctx.read::<PMCCNTR>()
        // }
        // (0b11, 0b100, 0b1100, 0b110:m[3], m[2:0]) => {
        // 	ctx.read::<ICH_LRn_EL2>()
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b001) => ctx.read::<PMCNTENSET_EL0>(),
        (0b11, 0b000, 0b1010, 0b0010, 0b101) => ctx.read::<S2POR_EL1>(),
        // (0b10, 0b011, 0b1110, 0b011:m[3], m[2:0]) => {
        // 	ctx.read::<SPMEVFILT2Rn_EL0>()
        // }
        (0b11, 0b100, 0b0001, 0b0001, 0b000) => ctx.read::<HCR_EL2>(),
        // (0b1111, 0b100, 0b1100, 0b1011, 0b101) => {
        // 	ctx.read::<ICH_ELRSR>()
        // }
        // (0b1111, 0b000, 0b1101, 0b011:m[3], m[2:0]) => {
        // 	ctx.read::<AMEVTYPER0n>()
        // }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b101) => {
        // 	ctx.read::<PMSELR>()
        // }
        (0b11, 0b110, 0b0101, 0b0010, 0b011) => ctx.read::<VSESR_EL3>(),
        (0b11, 0b110, 0b0101, 0b0001, 0b001) => ctx.read::<AFSR1_EL3>(),
        (0b11, 0b000, 0b0000, 0b0010, 0b101) => ctx.read::<ID_ISAR5_EL1>(),
        (0b10, 0b001, 0b0000, 0b0001, 0b000) => ctx.read::<TRCPRGCTLR>(),
        // (0b1111, 0b000, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.read::<ICC_AP1Rn>()
        // }
        (0b11, 0b000, 0b0000, 0b0110, 0b001) => ctx.read::<ID_AA64ISAR1_EL1>(),
        // (0b1111, 0b000, 0b1001, 0b1110, 0b011) => {
        // 	ctx.read::<PMOVSSET>()
        // }
        (0b10, 0b000, 0b0000, 0b0010, 0b000) => ctx.read::<MDCCINT_EL1>(),
        // (0b10, 0b000, 0b0000, m[3:0], 0b101) => {
        // 	ctx.read::<DBGBCRn_EL1>()
        // }
        (0b10, 0b001, 0b0111, 0b1001, 0b110) => ctx.read::<TRCCLAIMCLR>(),
        // (0b10, 0b001, 0b0001, m[3:0], 0b00:m[4]) => {
        // 	ctx.read::<TRCRSCTLRn>()
        // }
        (0b10, 0b011, 0b1001, 0b1110, 0b011) => ctx.read::<SPMOVSSET_EL0>(),
        (0b11, 0b011, 0b1001, 0b1110, 0b000) => ctx.read::<PMUSERENR_EL0>(),
        (0b11, 0b100, 0b1101, 0b0000, 0b001) => ctx.read::<CONTEXTIDR_EL2>(),
        (0b11, 0b111, 0b1110, 0b0010, 0b010) => ctx.read::<CNTPS_CVAL_EL1>(),
        // (0b1111, 0b000, 0b1110, 0b10:m[4:3], m[2:0]) => {
        // 	ctx.read::<PMEVCNTRn>()
        // }
        (0b10, 0b001, 0b0111, 0b1000, 0b110) => ctx.read::<TRCCLAIMSET>(),
        // (0b1111, 0b000, 0b1001, 0b1110, 0b110) => {
        // 	ctx.read::<PMMIR>()
        // }
        (0b11, 0b011, 0b0100, 0b0100, 0b000) => ctx.read::<FPCR>(),
        (0b11, 0b000, 0b0101, 0b0001, 0b001) => ctx.read::<AFSR1_EL1>(),
        (0b11, 0b011, 0b1110, 0b0000, 0b101) => ctx.read::<CNTPCTSS_EL0>(),
        // (0b1111, 0b100, 0b0001, 0b0001, 0b001) => {
        // 	ctx.read::<HDCR>()
        // }
        // (0b1111, 0b000, 0b1100, 0b0001, 0b001) => {
        // 	ctx.read::<DISR>()
        // }
        (0b11, 0b100, 0b1110, 0b0100, 0b000) => ctx.read::<CNTHVS_TVAL_EL2>(),
        (0b11, 0b100, 0b0100, 0b0001, 0b000) => ctx.read::<SP_EL1>(),
        // (0b1110, 0b000, 0b0000, m[3:0], 0b100) => {
        // 	ctx.read::<DBGBVRn>()
        // }
        (0b11, 0b000, 0b1001, 0b1011, 0b001) => ctx.read::<TRBPTR_EL1>(),
        // (0b1111, 0b000, 0b1101, 0b0000, 0b011) => {
        // 	ctx.read::<TPIDRURO>()
        // }
        (0b11, 0b001, 0b0000, 0b0000, 0b001) => ctx.read::<CLIDR_EL1>(),
        (0b11, 0b000, 0b1010, 0b0010, 0b011) => ctx.read::<PIR_EL1>(),
        (0b10, 0b001, 0b0000, 0b0010, 0b010) => ctx.read::<TRCVISSCTLR>(),
        (0b11, 0b011, 0b0010, 0b0100, 0b000) => {
            ctx.write::<pstate::N>(0);
            ctx.write::<pstate::Z>(0);
            ctx.write::<pstate::C>(0);
            ctx.write::<pstate::V>(0);
            ctx.ctx.rng()
        }
        // (0b1111, 0b100, 0b1100, 0b111:m[3], m[2:0]) => {
        // 	ctx.read::<ICH_LRCn>()
        // }
        (0b11, 0b000, 0b0000, 0b0111, 0b100) => ctx.read::<ID_AA64MMFR4_EL1>(),
        (0b11, 0b000, 0b1100, 0b0001, 0b001) => ctx.read::<DISR_EL1>(),
        (0b11, 0b000, 0b0000, 0b0001, 0b100) => ctx.read::<ID_MMFR0_EL1>(),
        // (0b1111, 0b100, 0b0101, 0b0001, 0b000) => {
        // 	ctx.read::<HADFSR>()
        // }
        // (0b1111, 0b000, 0b1110, 0b0010, 0b001) => {
        // 	ctx.read::<CNTP_CTL>()
        // }
        (0b11, 0b000, 0b0100, 0b0001, 0b000) => ctx.read::<SP_EL0>(),
        // (0b1111, 0b000, 0b1110, 0b0011, 0b000) => {
        // 	ctx.read::<CNTHV_TVAL>()
        // }
        // (0b1111, 0b100, 0b1100, 0b1011, 0b000) => {
        // 	ctx.read::<ICH_HCR>()
        // }
        (0b11, 0b100, 0b1110, 0b0010, 0b000) => ctx.read::<CNTHP_TVAL_EL2>(),
        // (0b10, 0b000, 0b1001, 0b1101, 0b00:m[0]) => {
        // 	ctx.read::<SPMCGCRn_EL1>()
        // }
        (0b11, 0b000, 0b0100, 0b0010, 0b000) => {
            let sp = ctx.read::<pstate::SP>() != 0;
            let sp = if sp { 1 } else { 0 };
            let sp = sp << 0;
            sp
        }
        (0b11, 0b000, 0b1001, 0b1011, 0b011) => ctx.read::<TRBSR_EL1>(),
        (0b10, 0b001, 0b0000, 0b0100, 0b000) => ctx.read::<TRCCONFIGR>(),
        // (0b1111, 0b000, 0b0000, 0b0000, 0b110) => {
        // 	ctx.read::<REVIDR>()
        // }
        (0b10, 0b001, 0b0111, 0b1111, 0b110) => ctx.read::<TRCDEVARCH>(),
        (0b11, 0b000, 0b0101, 0b0001, 0b000) => ctx.read::<AFSR0_EL1>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b111) => {
        // 	ctx.read::<ICC_IGRPEN1>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0000, 0b000) => {
        // 	ctx.read::<MIDR>()
        // }
        (0b11, 0b011, 0b0010, 0b0100, 0b001) => {
            ctx.write::<pstate::N>(0);
            ctx.write::<pstate::Z>(0);
            ctx.write::<pstate::C>(0);
            ctx.write::<pstate::V>(0);
            ctx.ctx.rng()
        }
        (0b11, 0b100, 0b1100, 0b1011, 0b010) => ctx.read::<ICH_MISR_EL2>(),
        (0b11, 0b000, 0b1001, 0b1001, 0b111) => ctx.read::<PMSIDR_EL1>(),
        // (0b1111, 0b000, 0b1101, 0b0011, 0b001) => {
        // 	ctx.read::<AMCNTENSET1>()
        // }
        (0b10, 0b001, 0b0000, 0b1011, 0b000) => ctx.read::<TRCSTALLCTLR>(),
        (0b11, 0b011, 0b1001, 0b1101, 0b000) => ctx.read::<PMCCNTR_EL0>(),
        // (0b10, 0b001, 0b0011, m[2:0]:0b0, 0b000) => {
        // 	ctx.read::<TRCCIDCVRn>()
        // }
        (0b10, 0b000, 0b1001, 0b1110, 0b010) => ctx.read::<SPMINTENCLR_EL1>(),
        (0b10, 0b001, 0b1001, 0b0001, 0b001) => ctx.read::<BRBSRCINJ_EL1>(),
        (0b11, 0b100, 0b0000, 0b0000, 0b000) => ctx.read::<VPIDR_EL2>(),
        // (0b1111, 0b000, 0b1010, 0b0010, 0b000) => {
        // 	ctx.read::<PRRR>()
        // }
        (0b11, 0b100, 0b0010, 0b0010, 0b000) => ctx.read::<VNCR_EL2>(),
        (0b11, 0b010, 0b0000, 0b0000, 0b000) => ctx.read::<CSSELR_EL1>(),
        (0b10, 0b001, 0b0000, 0b0101, 0b110) => ctx.read::<TRCIDR13>(),
        (0b11, 0b100, 0b0010, 0b0101, 0b000) => ctx.read::<GCSCR_EL2>(),
        (0b11, 0b000, 0b0101, 0b0100, 0b101) => ctx.read::<ERXPFGCTL_EL1>(),
        (0b10, 0b000, 0b0000, 0b0010, 0b010) => ctx.read::<MDSCR_EL1>(),
        // (0b1111, 0b001, 0b0000, 0b0000, 0b111) => {
        // 	ctx.read::<AIDR>()
        // }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b010) => {
        // 	ctx.read::<PMINTENCLR>()
        // }
        (0b11, 0b100, 0b0011, 0b0001, 0b101) => ctx.read::<HDFGWTR_EL2>(),
        (0b11, 0b000, 0b0000, 0b0001, 0b101) => ctx.read::<ID_MMFR1_EL1>(),
        (0b11, 0b000, 0b1001, 0b1011, 0b101) => ctx.read::<TRBMPAM_EL1>(),
        (0b11, 0b110, 0b0010, 0b0101, 0b000) => ctx.read::<GCSCR_EL3>(),
        (0b10, 0b001, 0b0000, 0b0100, 0b110) => ctx.read::<TRCIDR12>(),
        (0b11, 0b100, 0b0011, 0b0001, 0b110) => ctx.read::<HAFGRTR_EL2>(),
        // (0b1110, 0b000, 0b0000, 0b0010, 0b010) => {
        // 	ctx.read::<DBGDSCRext>()
        // }
        // (0b1110, 0b000, 0b0000, 0b0001, 0b000) => {
        // 	ctx.read::<DBGDSCRint>()
        // }
        (0b11, 0b100, 0b0010, 0b0001, 0b000) => ctx.read::<VTTBR_EL2>(),
        (0b10, 0b000, 0b0001, 0b0011, 0b100) => ctx.read::<OSDLR_EL1>(),
        (0b10, 0b000, 0b1110, 0b1011, 0b111) => ctx.read::<PMCCNTSVR_EL1>(),
        // (0b1111, 0b100, 0b0101, 0b0010, 0b011) => {
        // 	ctx.read::<VDFSR>()
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b011) => ctx.read::<PMOVSCLR_EL0>(),
        (0b11, 0b100, 0b1110, 0b0101, 0b001) => ctx.read::<CNTHPS_CTL_EL2>(),
        // (0b1111, 0b000, 0b1101, 0b0010, 0b101) => {
        // 	ctx.read::<AMCNTENSET0>()
        // }
        // (0b1111, 0b011, 0b0100, 0b0101, 0b010) => {
        // 	ctx.read::<DSPSR2>()
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b000) => ctx.read::<PMCR_EL0>(),
        // (0b1111, 0b100, 0b0001, 0b0001, 0b011) => {
        // 	ctx.read::<HSTR>()
        // }
        (0b10, 0b001, 0b0000, 0b1101, 0b000) => ctx.read::<TRCSYNCPR>(),
        // (0b1111, 0b000, 0b1100, 0b1100, 0b110) => {
        // 	ctx.read::<ICC_IGRPEN0>()
        // }
        (0b11, 0b100, 0b0101, 0b0001, 0b000) => ctx.read::<AFSR0_EL2>(),
        (0b10, 0b000, 0b0000, 0b0100, 0b010) => ctx.read::<MDSELR_EL1>(),
        // (0b1111, 0b000, 0b1010, 0b0011, 0b001) => {
        // 	ctx.read::<AMAIR1>()
        // }
        (0b11, 0b000, 0b1001, 0b1001, 0b001) => ctx.read::<PMSNEVFR_EL1>(),
        (0b11, 0b000, 0b0100, 0b0010, 0b100) => {
            let uao = ctx.read::<pstate::UAO>() != 0;
            let uao = if uao { 1 } else { 0 };
            let uao = uao << 23;
            uao
        }
        (0b11, 0b000, 0b0101, 0b0100, 0b100) => ctx.read::<ERXPFGF_EL1>(),
        // (0b1111, 0b100, 0b0110, 0b0000, 0b100) => {
        // 	ctx.read::<HPFAR>()
        // }
        (0b11, 0b100, 0b1010, 0b1000, 0b011) => ctx.read::<MECID_A1_EL2>(),
        (0b11, 0b100, 0b1010, 0b0110, 0b100) => ctx.read::<MPAMVPM4_EL2>(),
        (0b11, 0b000, 0b1100, 0b1100, 0b100) => ctx.read::<ICV_CTLR_EL1>(),
        // (0b1111, 0b000, 0b1101, 0b111:m[3], m[2:0]) => {
        // 	ctx.read::<AMEVTYPER1n>()
        // }
        (0b11, 0b100, 0b0101, 0b0011, 0b000) => ctx.read::<FPEXC32_EL2>(),
        (0b11, 0b011, 0b1101, 0b0000, 0b011) => ctx.read::<TPIDRRO_EL0>(),
        // (0b1110, 0b000, 0b0001, 0b0100, 0b100) => {
        // 	ctx.read::<DBGPRCR>()
        // }
        (0b11, 0b100, 0b1110, 0b0011, 0b000) => ctx.read::<CNTHV_TVAL_EL2>(),
        // (0b1111, 0b010, 0b0000, 0b0000, 0b000) => {
        // 	ctx.read::<CSSELR>()
        // }
        (0b10, 0b001, 0b0000, 0b0000, 0b110) => ctx.read::<TRCIDR8>(),
        (0b11, 0b100, 0b1110, 0b0001, 0b000) => ctx.read::<CNTHCTL_EL2>(),
        // (0b1111, 0b100, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.read::<ICH_AP1Rn>()
        // }
        // (0b1111, 0b000, 0b0010, 0b0000, 0b001) => {
        // 	ctx.read::<TTBR1>()
        // }
        (0b11, 0b000, 0b0010, 0b0101, 0b000) => ctx.read::<GCSCR_EL1>(),
        (0b10, 0b001, 0b0000, 0b0010, 0b110) => ctx.read::<TRCIDR10>(),
        // (0b1111, 0b000, 0b0101, 0b0100, 0b101) => {
        // 	ctx.read::<ERXCTLR2>()
        // }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b001) => {
        // 	ctx.read::<PMINTENSET>()
        // }
        (0b11, 0b000, 0b0010, 0b0011, 0b001) => ctx.read::<APGAKeyHi_EL1>(),
        // (0b1111, 0b001, 0b0000, 0b0000, 0b001) => {
        // 	ctx.read::<CLIDR>()
        // }
        // (0b11, 0b011, 0b1110, 0b10:m[4:3], m[2:0]) => {
        // 	ctx.read::<PMEVCNTRn_EL0>()
        // }
        // (0b1110, 0b000, 0b0001, 0b0000, 0b100) => {
        // 	ctx.read::<DBGOSLAR>()
        // }
        (0b11, 0b100, 0b0011, 0b0000, 0b000) => ctx.read::<DACR32_EL2>(),
        // (0b1111, 0b000, 0b1110, 0b0010, 0b001) => {
        // 	ctx.read::<CNTHPS_CTL>()
        // }
        (0b11, 0b000, 0b0100, 0b0110, 0b000) => ctx.read::<ICV_PMR_EL1>(),
        // (0b1111, 0b000, 0b0101, 0b0000, 0b001) => {
        // 	ctx.read::<IFSR>()
        // }
        // (0b11, 0b011, 0b1101, 0b010:m[3], m[2:0]) => {
        // 	ctx.read::<AMEVCNTR0n_EL0>()
        // }
        // (0b11, 0b011, 0b1101, 0b110:m[3], m[2:0]) => {
        // 	ctx.read::<AMEVCNTR1n_EL0>()
        // }
        // (0b1111, 0b000, 0b0000, 0b0000, 0b001) => {
        // 	ctx.read::<CTR>()
        // }
        (0b11, 0b000, 0b1100, 0b1011, 0b011) => ctx.read::<ICV_RPR_EL1>(),
        (0b10, 0b001, 0b0000, 0b0011, 0b110) => ctx.read::<TRCIDR11>(),
        // (0b1111, 0b100, 0b0110, 0b0000, 0b000) => {
        // 	ctx.read::<HDFAR>()
        // }
        (0b11, 0b000, 0b1001, 0b1010, 0b011) => ctx.read::<PMBSR_EL1>(),
        // (0b1111, 0b000, 0b0010, 0b0000, 0b000) => {
        // 	ctx.read::<TTBR0>()
        // }
        (0b11, 0b011, 0b1110, 0b0000, 0b000) => ctx.read::<CNTFRQ_EL0>() & (u32::MAX as u64),
        (0b10, 0b001, 0b0000, 0b0001, 0b110) => ctx.read::<TRCIDR9>(),
        (0b10, 0b001, 0b0111, 0b1110, 0b110) => ctx.read::<TRCAUTHSTATUS>(),
        (0b11, 0b000, 0b1100, 0b1001, 0b101) => ctx.read::<ICV_NMIAR1_EL1>(),
        (0b11, 0b000, 0b0010, 0b0001, 0b000) => ctx.read::<APIAKeyLo_EL1>(),
        (0b11, 0b000, 0b0000, 0b0110, 0b000) => ctx.read::<ID_AA64ISAR0_EL1>(),
        // (0b11, 0b100, 0b1100, 0b1000, 0b0:m[1:0]) => {
        // 	ctx.read::<ICH_AP0Rn_EL2>()
        // }
        // (0b1111, 0b000, 0b0101, 0b0011, 0b001) => {
        // 	ctx.read::<ERRSELR>()
        // }
        (0b11, 0b110, 0b1010, 0b0101, 0b000) => ctx.read::<MPAM3_EL3>(),
        // (0b1111, 0b000, 0b1010, 0b0011, 0b000) => {
        // 	ctx.read::<AMAIR0>()
        // }
        (0b11, 0b000, 0b0000, 0b0010, 0b100) => ctx.read::<ID_ISAR4_EL1>(),
        (0b11, 0b110, 0b0101, 0b0001, 0b000) => ctx.read::<AFSR0_EL3>(),
        _ => panic!("Unknown MRS {0} {1} {2} {3} {4}", op0, op1, crn, crm, op2),
    };
    data
}
