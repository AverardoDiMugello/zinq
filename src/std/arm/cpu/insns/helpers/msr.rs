use crate::core::model::{ir::*, *};
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate};
use crate::std::arm::cpu::ArmCpu;

use bitvec::prelude::*;

pub fn emulate_msr_reg(cpu: &ArmCpu, ctx: &mut ProcCtx<ArmCpu>, args: Vec<CallArg>) {
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

    let data = args
        .get(1)
        .and_then(|arg| {
            if let CallArg::Bv64(val) = arg {
                Some(val.clone())
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding");

    let op0: u8 = 2 + if this_instr.view_bits::<Lsb0>()[19] {
        1
    } else {
        0
    };
    let op1: u8 = (&this_instr.view_bits::<Lsb0>()[16..19]).load();
    let crn: u8 = (&this_instr.view_bits::<Lsb0>()[12..16]).load();
    let crm: u8 = (&this_instr.view_bits::<Lsb0>()[8..12]).load();
    let op2: u8 = (&this_instr.view_bits::<Lsb0>()[5..8]).load();

    match (op0, op1, crn, crm, op2) {
        (0b11, 0b111, 0b1110, 0b0010, 0b001) => {
            ctx.write::<CNTPS_CTL_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b001) => {
        // 	ctx.write::<ERXCTLR>(data);
        // }
        (0b11, 0b000, 0b0101, 0b0101, 0b010) => {
            ctx.write::<ERXMISC2_EL1>(data);
        }
        (0b11, 0b100, 0b1100, 0b0000, 0b000) => {
            ctx.write::<VBAR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1010, 0b0010, 0b001) => {
        // 	ctx.write::<MAIR1>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b001) => {
            ctx.write::<ICC_EOIR1_EL1>(data);
        }
        // (0b10, 0b000, 0b1110, 0b10:m[4:3], m[2:0]) => {
        // 	ctx.write::<PMEVCNTSVRn_EL1>(data);
        // }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b011) => {
        // 	ctx.write::<ERXMISC5>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1011, 0b001) => {
            ctx.write::<ICC_DIR_EL1>(data);
        }
        (0b11, 0b100, 0b1110, 0b0101, 0b000) => {
            ctx.write::<CNTHPS_TVAL_EL2>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b011) => {
            ctx.write::<ICC_BPR1_EL1>(data);
        }
        (0b11, 0b000, 0b1001, 0b1011, 0b100) => {
            ctx.write::<TRBMAR_EL1>(data);
        }
        (0b11, 0b100, 0b0001, 0b0010, 0b011) => {
            ctx.write::<TRCITECR_EL2>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b1011, 0b111) => {
        // 	ctx.write::<ICH_VMCR>(data);
        // }
        (0b10, 0b001, 0b0000, 0b1100, 0b111) => {
            ctx.write::<TRCIDR4>(data);
        }
        (0b11, 0b100, 0b0001, 0b0000, 0b001) => {
            ctx.write::<ACTLR_EL2>(data);
        }
        (0b11, 0b011, 0b1101, 0b0010, 0b010) => {
            ctx.write::<AMCGCR_EL0>(data);
        }
        (0b11, 0b011, 0b1101, 0b0011, 0b000) => {
            ctx.write::<AMCNTENCLR1_EL0>(data);
        }
        (0b11, 0b000, 0b1101, 0b0000, 0b101) => {
            ctx.write::<ACCDATA_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0010, 0b001) => {
            ctx.write::<ID_ISAR1_EL1>(data);
        }
        (0b10, 0b000, 0b1001, 0b1110, 0b001) => {
            ctx.write::<SPMINTENSET_EL1>(data);
        }
        (0b10, 0b001, 0b0011, 0b0011, 0b010) => {
            ctx.write::<TRCVMIDCCTLR1>(data);
        }
        // (0b10, 0b000, 0b0000, m[3:0], 0b110) => {
        // 	ctx.write::<DBGWVRn_EL1>(data);
        // }
        // (0b1111, 0b000, 0b0110, 0b0000, 0b000) => {
        // 	ctx.write::<DFAR>(data);
        // }
        (0b11, 0b000, 0b0001, 0b0000, 0b010) => {
            ctx.write::<CPACR_EL1>(data);
        }
        // (0b1110, 0b000, 0b0111, 0b1000, 0b110) => {
        // 	ctx.write::<DBGCLAIMSET>(data);
        // }
        (0b11, 0b000, 0b1110, 0b0001, 0b000) => {
            ctx.write::<CNTKCTL_EL1>(data);
        }
        (0b10, 0b001, 0b0011, 0b0010, 0b010) => {
            ctx.write::<TRCVMIDCCTLR0>(data);
        }
        (0b11, 0b000, 0b0101, 0b0011, 0b010) => {
            ctx.write::<ERXGSR_EL1>(data);
        }
        (0b11, 0b000, 0b0010, 0b0010, 0b011) => {
            ctx.write::<APDBKeyHi_EL1>(data);
        }
        // (0b1110, 0b000, 0b0000, 0b0110, 0b010) => {
        // 	ctx.write::<DBGOSECCR>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0000, 0b001) => {
            ctx.write::<TRCTRACEIDR>(data);
        }
        // (0b1111, 0b001, 0b0000, 0b0000, 0b010) => {
        // 	ctx.write::<CCSIDR2>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0011, 0b001) => {
            ctx.write::<MVFR1_EL1>(data);
        }
        // (0b1111, 0b100, 0b0010, 0b0001, 0b010) => {
        // 	ctx.write::<VTCR>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1011, 0b111) => {
            ctx.write::<ICC_SGI0R_EL1>(data);
        }
        (0b10, 0b001, 0b1001, 0b0001, 0b010) => {
            ctx.write::<BRBTGTINJ_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1011, 0b101) => {
            ctx.write::<ICC_SGI1R_EL1>(data);
        }
        (0b11, 0b110, 0b0001, 0b0000, 0b001) => {
            ctx.write::<ACTLR_EL3>(data);
        }
        (0b10, 0b001, 0b0000, 0b1101, 0b111) => {
            ctx.write::<TRCIDR5>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b101) => {
            ctx.write::<ICC_SRE_EL1>(data);
        }
        (0b11, 0b100, 0b1010, 0b0110, 0b001) => {
            ctx.write::<MPAMVPM1_EL2>(data);
        }
        (0b11, 0b011, 0b1001, 0b1110, 0b011) => {
            ctx.write::<PMOVSSET_EL0>(data);
        }
        // (0b1111, 0b000, 0b0001, 0b0000, 0b000) => {
        // 	ctx.write::<SCTLR>(data);
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b111) => {
            ctx.write::<PMCEID1_EL0>(data);
        }
        // (0b10, 0b000, 0b0000, m[3:0], 0b111) => {
        // 	ctx.write::<DBGWCRn_EL1>(data);
        // }
        (0b11, 0b100, 0b1010, 0b1000, 0b010) => {
            ctx.write::<MECID_P1_EL2>(data);
        }
        (0b11, 0b000, 0b0001, 0b0010, 0b110) => {
            ctx.write::<SMCR_EL1>(data);
        }
        (0b11, 0b011, 0b0100, 0b0100, 0b010) => {
            ctx.write::<FPMR>(data);
        }
        (0b10, 0b011, 0b1001, 0b1100, 0b000) => {
            ctx.write::<SPMCR_EL0>(data);
        }
        (0b11, 0b000, 0b1010, 0b0100, 0b100) => {
            ctx.write::<MPAMIDR_EL1>(data);
        }
        (0b11, 0b000, 0b0100, 0b0000, 0b000) => {
            ctx.write::<SPSR_EL1>(data);
        }
        (0b11, 0b000, 0b0100, 0b0000, 0b001) => {
            ctx.write::<ELR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b010) => {
        // 	ctx.write::<ERXMISC4>(data);
        // }
        (0b11, 0b011, 0b1101, 0b0000, 0b101) => {
            ctx.write::<TPIDR2_EL0>(data);
        }
        // (0b1111, 0b000, 0b1010, 0b0010, 0b000) => {
        // 	ctx.write::<MAIR0>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b000) => {
            ctx.write::<ICV_IAR1_EL1>(data);
        }
        (0b11, 0b110, 0b1100, 0b0000, 0b000) => {
            ctx.write::<VBAR_EL3>(data);
        }
        (0b11, 0b100, 0b0011, 0b0001, 0b100) => {
            ctx.write::<HDFGRTR_EL2>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0001, 0b001) => {
        // 	ctx.write::<AIFSR>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0001, 0b001) => {
            ctx.write::<ID_PFR1_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b0000, 0b000) => {
            ctx.write::<VBAR_EL1>(data);
        }
        (0b11, 0b000, 0b1001, 0b1101, 0b011) => {
            ctx.write::<PMSSCR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b1100, 0b000) => {
            ctx.write::<TRCTSCTLR>(data);
        }
        (0b11, 0b000, 0b1001, 0b1010, 0b001) => {
            ctx.write::<PMBPTR_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0010, 0b110) => {
            ctx.write::<ID_MMFR4_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b110) => {
        // 	ctx.write::<ERXMISC6>(data);
        // }
        (0b11, 0b011, 0b1101, 0b0010, 0b101) => {
            ctx.write::<AMCNTENSET0_EL0>(data);
        }
        // (0b1110, 0b000, 0b0001, 0b0011, 0b100) => {
        // 	ctx.write::<DBGOSDLR>(data);
        // }
        (0b11, 0b110, 0b0100, 0b0000, 0b001) => {
            ctx.write::<ELR_EL3>(data);
        }
        (0b11, 0b000, 0b1001, 0b1011, 0b110) => {
            ctx.write::<TRBTRG_EL1>(data);
        }
        (0b11, 0b110, 0b0100, 0b0000, 0b000) => {
            ctx.write::<SPSR_EL3>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1011, 0b001) => {
        // 	ctx.write::<ICV_DIR>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0111, 0b000) => {
            ctx.write::<ID_AA64MMFR0_EL1>(data);
        }
        (0b11, 0b110, 0b0001, 0b0001, 0b000) => {
            ctx.write::<SCR_EL3>(data);
        }
        (0b11, 0b100, 0b0011, 0b0001, 0b111) => {
            ctx.write::<HFGITR2_EL2>(data);
        }
        (0b10, 0b000, 0b0001, 0b0000, 0b000) => {
            ctx.write::<MDRAR_EL1>(data);
        }
        (0b11, 0b110, 0b0001, 0b0010, 0b110) => {
            ctx.write::<SMCR_EL3>(data);
        }
        (0b11, 0b011, 0b1110, 0b0000, 0b010) => {
            ctx.write::<CNTVCT_EL0>(data);
        }
        (0b11, 0b000, 0b0000, 0b0101, 0b101) => {
            ctx.write::<ID_AA64AFR1_EL1>(data);
        }
        (0b11, 0b110, 0b1100, 0b0001, 0b001) => {
            ctx.write::<VDISR_EL3>(data);
        }
        (0b11, 0b000, 0b0001, 0b0010, 0b011) => {
            ctx.write::<TRCITECR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b1111, 0b111) => {
            ctx.write::<TRCIDR7>(data);
        }
        (0b11, 0b000, 0b0000, 0b0101, 0b000) => {
            ctx.write::<ID_AA64DFR0_EL1>(data);
        }
        (0b11, 0b110, 0b1100, 0b1100, 0b101) => {
            ctx.write::<ICC_SRE_EL3>(data);
        }
        (0b11, 0b011, 0b1001, 0b0110, 0b000) => {
            ctx.write::<PMICFILTR_EL0>(data);
        }
        (0b11, 0b000, 0b0001, 0b0000, 0b001) => {
            ctx.write::<ACTLR_EL1>(data);
        }
        (0b11, 0b011, 0b1110, 0b0011, 0b000) => {
            ctx.write::<CNTV_TVAL_EL0>(data);
        }
        (0b11, 0b000, 0b0000, 0b0100, 0b010) => {
            ctx.write::<ID_AA64PFR2_EL1>(data);
        }
        // (0b1111, 0b110, 0b1100, 0b1100, 0b100) => {
        // 	ctx.write::<ICC_MCTLR>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0011, 0b010) => {
            ctx.write::<TRCVIPCSSCTLR>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1011, 0b001) => {
        // 	ctx.write::<ICC_DIR>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b001) => {
            ctx.write::<ICV_EOIR1_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0100, 0b100) => {
            ctx.write::<ID_AA64ZFR0_EL1>(data);
        }
        // (0b1110, 0b000, 0b0111, 0b1001, 0b110) => {
        // 	ctx.write::<DBGCLAIMCLR>(data);
        // }
        // (0b1111, 0b000, 0b0001, 0b0010, 0b001) => {
        // 	ctx.write::<TRFCR>(data);
        // }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b000) => {
        // 	ctx.write::<ERXFR>(data);
        // }
        // (0b1111, 0b100, 0b1110, 0b0010, 0b001) => {
        // 	ctx.write::<CNTHP_CTL>(data);
        // }
        // (0b1111, 0b100, 0b0101, 0b0010, 0b000) => {
        // 	ctx.write::<HSR>(data);
        // }
        (0b10, 0b011, 0b0000, 0b0001, 0b000) => {
            ctx.write::<MDCCSR_EL0>(data);
        }
        (0b11, 0b100, 0b1100, 0b1011, 0b101) => {
            ctx.write::<ICH_ELRSR_EL2>(data);
        }
        // (0b1110, 0b000, 0b0000, 0b0101, 0b000) => {
        // 	ctx.write::<DBGDTRRXint>(data);
        // }
        (0b11, 0b100, 0b1100, 0b1001, 0b101) => {
            ctx.write::<ICC_SRE_EL2>(data);
        }
        (0b11, 0b100, 0b0100, 0b0011, 0b001) => {
            ctx.write::<SPSR_abt>(data);
        }
        (0b10, 0b001, 0b0000, 0b1110, 0b111) => {
            ctx.write::<TRCIDR6>(data);
        }
        // (0b1110, 0b000, 0b0000, 0b0000, 0b010) => {
        // 	ctx.write::<DBGDTRRXext>(data);
        // }
        (0b11, 0b000, 0b1010, 0b0101, 0b011) => {
            ctx.write::<MPAMSM_EL1>(data);
        }
        (0b11, 0b000, 0b1001, 0b1001, 0b100) => {
            ctx.write::<PMSFCR_EL1>(data);
        }
        (0b11, 0b100, 0b1100, 0b0001, 0b001) => {
            ctx.write::<VDISR_EL2>(data);
        }
        (0b11, 0b100, 0b0001, 0b0010, 0b110) => {
            ctx.write::<SMCR_EL2>(data);
        }
        (0b11, 0b100, 0b0110, 0b0000, 0b100) => {
            ctx.write::<HPFAR_EL2>(data);
        }
        (0b11, 0b100, 0b0100, 0b0000, 0b000) => {
            ctx.write::<SPSR_EL2>(data);
        }
        (0b11, 0b110, 0b0010, 0b0001, 0b110) => {
            ctx.write::<GPCCR_EL3>(data);
        }
        (0b11, 0b011, 0b1001, 0b1100, 0b101) => {
            ctx.write::<PMSELR_EL0>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b111) => {
        // 	ctx.write::<ERXMISC7>(data);
        // }
        (0b10, 0b111, 0b1001, 0b1110, 0b111) => {
            ctx.write::<SPMSCR_EL1>(data);
        }
        (0b11, 0b100, 0b0100, 0b0000, 0b001) => {
            ctx.write::<ELR_EL2>(data);
        }
        (0b11, 0b000, 0b1001, 0b1011, 0b000) => {
            ctx.write::<TRBLIMITR_EL1>(data);
        }
        (0b11, 0b000, 0b1010, 0b0100, 0b011) => {
            ctx.write::<LORC_EL1>(data);
        }
        (0b11, 0b100, 0b0011, 0b0001, 0b011) => {
            ctx.write::<HFGWTR2_EL2>(data);
        }
        (0b11, 0b000, 0b1010, 0b0100, 0b001) => {
            ctx.write::<LOREA_EL1>(data);
        }
        (0b11, 0b000, 0b0010, 0b0101, 0b010) => {
            ctx.write::<GCSCRE0_EL1>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0000, 0b101) => {
        // 	ctx.write::<MPIDR>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0000, 0b010) => {
            ctx.write::<TRCVICTLR>(data);
        }
        (0b11, 0b000, 0b0000, 0b0111, 0b001) => {
            ctx.write::<ID_AA64MMFR1_EL1>(data);
        }
        // (0b1110, 0b111, 0b0000, 0b0000, 0b000) => {
        // 	ctx.write::<JIDR>(data);
        // }
        // (0b10, 0b001, 0b0001, 0b0:m[2:0], 0b010) => {
        // 	ctx.write::<TRCSSCCRn>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0101, 0b100) => {
            ctx.write::<ID_AA64AFR0_EL1>(data);
        }
        (0b11, 0b000, 0b0101, 0b0011, 0b000) => {
            ctx.write::<ERRIDR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b101) => {
        // 	ctx.write::<ERXMISC3>(data);
        // }
        (0b11, 0b000, 0b1101, 0b0000, 0b111) => {
            ctx.write::<SCXTNUM_EL1>(data);
        }
        (0b11, 0b100, 0b1101, 0b0000, 0b010) => {
            ctx.write::<TPIDR_EL2>(data);
        }
        (0b11, 0b000, 0b0000, 0b0001, 0b000) => {
            ctx.write::<ID_PFR0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1101, 0b010) => {
        // 	ctx.write::<PMXEVCNTR>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0111, 0b100) => {
            ctx.write::<TRCSEQSTR>(data);
        }
        (0b10, 0b000, 0b0001, 0b0001, 0b100) => {
            ctx.write::<OSLSR_EL1>(data);
        }
        (0b11, 0b011, 0b1101, 0b0011, 0b001) => {
            ctx.write::<AMCNTENSET1_EL0>(data);
        }
        (0b11, 0b000, 0b0000, 0b0011, 0b110) => {
            ctx.write::<ID_MMFR5_EL1>(data);
        }
        (0b11, 0b001, 0b0000, 0b0000, 0b110) => {
            ctx.write::<SMIDR_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1011, 0b110) => {
            ctx.write::<ICC_ASGI1R_EL1>(data);
        }
        // (0b1111, 0b000, 0b0001, 0b0001, 0b001) => {
        // 	ctx.write::<SDER>(data);
        // }
        // (0b11, 0b000, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.write::<ICV_AP1Rn_EL1>(data);
        // }
        (0b10, 0b001, 0b0000, 0b1010, 0b111) => {
            ctx.write::<TRCIDR2>(data);
        }
        // (0b10, 0b001, 0b0000, 0b00:m[1:0], 0b100) => {
        // 	ctx.write::<TRCSEQEVRn>(data);
        // }
        (0b11, 0b100, 0b0100, 0b0011, 0b011) => {
            ctx.write::<SPSR_fiq>(data);
        }
        (0b10, 0b011, 0b0000, 0b0101, 0b000) => {
            ctx.write::<DBGDTRRX_EL0>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b011) => {
        // 	ctx.write::<PMOVSR>(data);
        // }
        (0b11, 0b100, 0b1010, 0b0011, 0b000) => {
            ctx.write::<AMAIR_EL2>(data);
        }
        (0b11, 0b000, 0b0000, 0b0101, 0b001) => {
            ctx.write::<ID_AA64DFR1_EL1>(data);
        }
        // (0b1111, 0b100, 0b1010, 0b0011, 0b001) => {
        // 	ctx.write::<HAMAIR1>(data);
        // }
        (0b11, 0b011, 0b1110, 0b0011, 0b001) => {
            ctx.write::<CNTV_CTL_EL0>(data);
        }
        (0b11, 0b100, 0b0101, 0b0010, 0b000) => {
            ctx.write::<ESR_EL2>(data);
        }
        (0b11, 0b110, 0b0001, 0b0000, 0b011) => {
            ctx.write::<SCTLR2_EL3>(data);
        }
        (0b11, 0b000, 0b0000, 0b0100, 0b111) => {
            ctx.write::<ID_AA64FPFR0_EL1>(data);
        }
        // (0b11, 0b011, 0b1110, 0b11:m[4:3], m[2:0]) => {
        // 	ctx.write::<PMEVTYPERn_EL0>(data);
        // }
        (0b11, 0b100, 0b0001, 0b0000, 0b011) => {
            ctx.write::<SCTLR2_EL2>(data);
        }
        (0b11, 0b110, 0b0101, 0b0010, 0b000) => {
            ctx.write::<ESR_EL3>(data);
        }
        (0b10, 0b001, 0b1001, 0b0000, 0b000) => {
            ctx.write::<BRBCR_EL2>(data);
        }
        // (0b1111, 0b100, 0b1010, 0b0011, 0b000) => {
        // 	ctx.write::<HAMAIR0>(data);
        // }
        (0b11, 0b110, 0b1010, 0b0011, 0b000) => {
            ctx.write::<AMAIR_EL3>(data);
        }
        (0b10, 0b011, 0b1001, 0b1100, 0b011) => {
            ctx.write::<SPMOVSCLR_EL0>(data);
        }
        (0b11, 0b000, 0b0001, 0b0000, 0b101) => {
            ctx.write::<RGSR_EL1>(data);
        }
        (0b11, 0b100, 0b0011, 0b0001, 0b010) => {
            ctx.write::<HFGRTR2_EL2>(data);
        }
        (0b11, 0b000, 0b1100, 0b1000, 0b001) => {
            ctx.write::<ICV_EOIR0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b1111, 0b111) => {
        // 	ctx.write::<PMCCFILTR>(data);
        // }
        (0b11, 0b100, 0b1100, 0b1011, 0b001) => {
            ctx.write::<ICH_VTR_EL2>(data);
        }
        (0b10, 0b001, 0b0000, 0b1011, 0b111) => {
            ctx.write::<TRCIDR3>(data);
        }
        (0b11, 0b011, 0b1101, 0b0000, 0b111) => {
            ctx.write::<SCXTNUM_EL0>(data);
        }
        // (0b1110, 0b000, 0b0111, 0b0001, 0b111) => {
        // 	ctx.write::<DBGDEVID1>(data);
        // }
        // (0b1111, 0b000, 0b1001, 0b1101, 0b001) => {
        // 	ctx.write::<PMXEVTYPER>(data);
        // }
        (0b11, 0b110, 0b1101, 0b0000, 0b010) => {
            ctx.write::<TPIDR_EL3>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b0000, 0b001) => {
        // 	ctx.write::<RVBAR>(data);
        // }
        (0b11, 0b001, 0b0000, 0b0000, 0b100) => {
            ctx.write::<GMID_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b100) => {
        // 	ctx.write::<ERXMISC2>(data);
        // }
        (0b10, 0b001, 0b1001, 0b0000, 0b001) => {
            ctx.write::<BRBFCR_EL1>(data);
        }
        (0b11, 0b000, 0b1001, 0b1001, 0b110) => {
            ctx.write::<PMSLATFR_EL1>(data);
        }
        (0b11, 0b011, 0b1001, 0b1100, 0b010) => {
            ctx.write::<PMCNTENCLR_EL0>(data);
        }
        // (0b1111, 0b000, 0b0111, 0b0100, 0b000) => {
        // 	ctx.write::<PAR>(data);
        // }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b000) => {
        // 	ctx.write::<ERXMISC0>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1001, 0b101) => {
            ctx.write::<ICC_NMIAR1_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b111) => {
        // 	ctx.write::<ERXADDR2>(data);
        // }
        (0b11, 0b000, 0b1101, 0b0000, 0b100) => {
            ctx.write::<TPIDR_EL1>(data);
        }
        (0b11, 0b000, 0b0101, 0b0101, 0b011) => {
            ctx.write::<ERXMISC3_EL1>(data);
        }
        (0b11, 0b100, 0b1101, 0b0000, 0b111) => {
            ctx.write::<SCXTNUM_EL2>(data);
        }
        (0b10, 0b000, 0b0000, 0b0011, 0b010) => {
            ctx.write::<OSDTRTX_EL1>(data);
        }
        // (0b10, 0b001, 0b0001, 0b0:m[2:0], 0b011) => {
        // 	ctx.write::<TRCSSPCICRn>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1001, 0b101) => {
            ctx.write::<PMSEVFR_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1000, 0b001) => {
            ctx.write::<ICC_EOIR0_EL1>(data);
        }
        (0b11, 0b100, 0b1110, 0b0011, 0b001) => {
            ctx.write::<CNTHV_CTL_EL2>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b1011, 0b010) => {
        // 	ctx.write::<ICH_MISR>(data);
        // }
        (0b11, 0b000, 0b0100, 0b0011, 0b000) => {
            let allint = (data >> 13) & 1;
            ctx.write::<pstate::ALLINT>(allint);
        }
        (0b10, 0b000, 0b1001, 0b1101, 0b100) => {
            ctx.write::<SPMIIDR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b1001, 0b111) => {
            ctx.write::<TRCIDR1>(data);
        }
        (0b11, 0b100, 0b0000, 0b0000, 0b101) => {
            ctx.write::<VMPIDR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0000, 0b001) => {
        // 	ctx.write::<CONTEXTIDR>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0010, 0b000) => {
            ctx.write::<ID_ISAR0_EL1>(data);
        }
        (0b10, 0b000, 0b0001, 0b0100, 0b100) => {
            ctx.write::<DBGPRCR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0011, 0b000) => {
        // 	ctx.write::<CNTV_TVAL>(data);
        // }
        (0b11, 0b000, 0b1010, 0b0011, 0b000) => {
            ctx.write::<AMAIR_EL1>(data);
        }
        // (0b1110, 0b000, 0b0000, m[3:0], 0b101) => {
        // 	ctx.write::<DBGBCRn>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1000, 0b011) => {
            ctx.write::<ICC_BPR0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0000, 0b010) => {
        // 	ctx.write::<TPIDRURW>(data);
        // }
        (0b11, 0b000, 0b0101, 0b0010, 0b000) => {
            ctx.write::<ESR_EL1>(data);
        }
        // (0b1110, 0b000, 0b0001, 0b0000, 0b000) => {
        // 	ctx.write::<DBGDRAR>(data);
        // }
        // (0b1110, 0b000, 0b0001, m[3:0], 0b001) => {
        // 	ctx.write::<DBGBXVRn>(data);
        // }
        (0b11, 0b011, 0b1101, 0b0010, 0b100) => {
            ctx.write::<AMCNTENCLR0_EL0>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b011) => {
        // 	ctx.write::<ERXADDR>(data);
        // }
        (0b11, 0b000, 0b0010, 0b0001, 0b010) => {
            ctx.write::<APIBKeyLo_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0011, 0b000) => {
            ctx.write::<MVFR0_EL1>(data);
        }
        (0b11, 0b000, 0b0001, 0b0000, 0b011) => {
            ctx.write::<SCTLR2_EL1>(data);
        }
        (0b11, 0b011, 0b0100, 0b0010, 0b110) => {
            let ssbs = (data >> 12) & 1;
            ctx.write::<pstate::SSBS>(ssbs);
        }
        (0b10, 0b001, 0b1001, 0b0000, 0b000) => {
            ctx.write::<BRBCR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0100, 0b0110, 0b000) => {
        // 	ctx.write::<ICV_PMR>(data);
        // }
        (0b11, 0b100, 0b1010, 0b0110, 0b000) => {
            ctx.write::<MPAMVPM0_EL2>(data);
        }
        (0b11, 0b011, 0b1110, 0b0000, 0b001) => {
            ctx.write::<CNTPCT_EL0>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0001, 0b000) => {
        // 	ctx.write::<CNTKCTL>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b101) => {
        // 	ctx.write::<ICC_SRE>(data);
        // }
        (0b11, 0b100, 0b1010, 0b0100, 0b000) => {
            ctx.write::<MPAMHCR_EL2>(data);
        }
        (0b10, 0b001, 0b0000, 0b1000, 0b111) => {
            ctx.write::<TRCIDR0>(data);
        }
        // (0b1111, 0b000, 0b0100, 0b0110, 0b000) => {
        // 	ctx.write::<ICC_PMR>(data);
        // }
        (0b11, 0b011, 0b1110, 0b0010, 0b000) => {
            ctx.write::<CNTP_TVAL_EL0>(data);
        }
        // (0b11, 0b000, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.write::<ICC_AP1Rn_EL1>(data);
        // }
        (0b11, 0b011, 0b1101, 0b0000, 0b010) => {
            ctx.write::<TPIDR_EL0>(data);
        }
        (0b11, 0b110, 0b1101, 0b0000, 0b111) => {
            ctx.write::<SCXTNUM_EL3>(data);
        }
        // (0b1110, 0b000, 0b0111, 0b0000, 0b111) => {
        // 	ctx.write::<DBGDEVID2>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1000, 0b000) => {
            ctx.write::<ICV_IAR0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b0000, 0b010) => {
        // 	ctx.write::<RMR>(data);
        // }
        (0b11, 0b100, 0b1010, 0b1000, 0b000) => {
            ctx.write::<MECID_P0_EL2>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0000, 0b000) => {
        // 	ctx.write::<CNTFRQ>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1010, 0b000) => {
            ctx.write::<PMBLIMITR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0101, 0b001) => {
        // 	ctx.write::<ERXMISC1>(data);
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b110) => {
            ctx.write::<PMCEID0_EL0>(data);
        }
        // (0b1111, 0b000, 0b0001, 0b0000, 0b010) => {
        // 	ctx.write::<CPACR>(data);
        // }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b100) => {
        // 	ctx.write::<HCR2>(data);
        // }
        (0b11, 0b100, 0b0010, 0b0011, 0b010) => {
            ctx.write::<HDBSSBR_EL2>(data);
        }
        (0b11, 0b100, 0b0100, 0b0011, 0b000) => {
            ctx.write::<SPSR_irq>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b010) => {
        // 	ctx.write::<ICC_HPPIR1>(data);
        // }
        (0b10, 0b001, 0b0000, 0b1111, 0b000) => {
            ctx.write::<TRCBBCTLR>(data);
        }
        (0b11, 0b000, 0b0000, 0b0100, 0b001) => {
            ctx.write::<ID_AA64PFR1_EL1>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b100) => {
        // 	ctx.write::<PMCEID2>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0011, 0b110) => {
        // 	ctx.write::<ID_MMFR5>(data);
        // }
        (0b11, 0b011, 0b0100, 0b0101, 0b000) => {
            ctx.write::<DSPSR_EL0>(data);
        }
        (0b11, 0b100, 0b0010, 0b0011, 0b100) => {
            ctx.write::<HACDBSBR_EL2>(data);
        }
        (0b11, 0b001, 0b0000, 0b0000, 0b010) => {
            ctx.write::<CCSIDR2_EL1>(data);
        }
        (0b11, 0b000, 0b1010, 0b0010, 0b000) => {
            ctx.write::<MAIR_EL1>(data);
        }
        (0b11, 0b000, 0b0001, 0b0000, 0b110) => {
            ctx.write::<GCR_EL1>(data);
        }
        // (0b1111, 0b100, 0b0001, 0b0000, 0b011) => {
        // 	ctx.write::<HACTLR2>(data);
        // }
        // (0b1111, 0b110, 0b1100, 0b1100, 0b111) => {
        // 	ctx.write::<ICC_MGRPEN1>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0110, 0b000) => {
            ctx.write::<TRCAUXCTLR>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b010) => {
        // 	ctx.write::<ERXSTATUS>(data);
        // }
        // (0b1110, 0b000, 0b0000, m[3:0], 0b110) => {
        // 	ctx.write::<DBGWVRn>(data);
        // }
        (0b11, 0b011, 0b0100, 0b0101, 0b001) => {
            ctx.write::<DLR_EL0>(data);
        }
        (0b11, 0b000, 0b0000, 0b0001, 0b010) => {
            ctx.write::<ID_DFR0_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0111, 0b011) => {
            ctx.write::<ID_AA64MMFR3_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b110) => {
            ctx.write::<ICV_IGRPEN0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0011, 0b000) => {
        // 	ctx.write::<AMCNTENCLR1>(data);
        // }
        (0b11, 0b100, 0b0001, 0b0010, 0b010) => {
            ctx.write::<HCRX_EL2>(data);
        }
        (0b11, 0b001, 0b0000, 0b0000, 0b000) => {
            ctx.write::<CCSIDR_EL1>(data);
        }
        // (0b1111, 0b100, 0b0101, 0b0001, 0b001) => {
        // 	ctx.write::<HAIFSR>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0011, 0b100) => {
            ctx.write::<ID_PFR2_EL1>(data);
        }
        (0b10, 0b011, 0b0000, 0b0101, 0b000) => {
            ctx.write::<DBGDTRTX_EL0>(data);
        }
        (0b10, 0b001, 0b0011, 0b0000, 0b010) => {
            ctx.write::<TRCCIDCCTLR0>(data);
        }
        (0b10, 0b001, 0b0011, 0b0001, 0b010) => {
            ctx.write::<TRCCIDCCTLR1>(data);
        }
        // (0b1110, 0b000, 0b0000, 0b0110, 0b000) => {
        // 	ctx.write::<DBGWFAR>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b110) => {
            ctx.write::<ICC_IGRPEN0_EL1>(data);
        }
        // (0b10, 0b001, 0b1000, m[3:0], m[4]:0b10) => {
        // 	ctx.write::<BRBTGTn_EL1>(data);
        // }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b100) => {
        // 	ctx.write::<AMCNTENCLR0>(data);
        // }
        (0b11, 0b000, 0b1100, 0b0000, 0b001) => {
            ctx.write::<RVBAR_EL1>(data);
        }
        (0b11, 0b000, 0b0010, 0b0001, 0b011) => {
            ctx.write::<APIBKeyHi_EL1>(data);
        }
        (0b11, 0b011, 0b1101, 0b0010, 0b011) => {
            ctx.write::<AMUSERENR_EL0>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b0001, 0b000) => {
        // 	ctx.write::<ISR>(data);
        // }
        // (0b1111, 0b000, 0b0101, 0b0011, 0b000) => {
        // 	ctx.write::<ERRIDR>(data);
        // }
        // (0b1111, 0b000, 0b0001, 0b0001, 0b010) => {
        // 	ctx.write::<NSACR>(data);
        // }
        // (0b1111, 0b100, 0b0001, 0b0010, 0b001) => {
        // 	ctx.write::<HTRFCR>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b110) => {
        // 	ctx.write::<ID_MMFR4>(data);
        // }
        // (0b10, 0b011, 0b1110, 0b010:m[3], m[2:0]) => {
        // 	ctx.write::<SPMEVFILTRn_EL0>(data);
        // }
        (0b11, 0b000, 0b1010, 0b0010, 0b001) => {
            ctx.write::<MAIR2_EL1>(data);
        }
        // (0b10, 0b001, 0b0011, m[2:0]:0b0, 0b001) => {
        // 	ctx.write::<TRCVMIDCVRn>(data);
        // }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b101) => {
        // 	ctx.write::<PMCEID3>(data);
        // }
        // (0b1111, 0b110, 0b1100, 0b1100, 0b101) => {
        // 	ctx.write::<ICC_MSRE>(data);
        // }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b000) => {
        // 	ctx.write::<PMCR>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b010) => {
        // 	ctx.write::<ICC_HPPIR0>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1000, 0b000) => {
            ctx.write::<ICC_IAR0_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b010) => {
            ctx.write::<ICC_HPPIR1_EL1>(data);
        }
        (0b10, 0b000, 0b0000, 0b0000, 0b010) => {
            ctx.write::<OSDTRRX_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0010, 0b010) => {
            ctx.write::<ID_ISAR2_EL1>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b111) => {
        // 	ctx.write::<PMCEID1>(data);
        // }
        (0b11, 0b110, 0b1010, 0b0001, 0b001) => {
            ctx.write::<MAIR2_EL3>(data);
        }
        // (0b1111, 0b000, 0b0010, 0b0000, 0b011) => {
        // 	ctx.write::<TTBCR2>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b100) => {
        // 	ctx.write::<ICV_CTLR>(data);
        // }
        (0b11, 0b100, 0b1010, 0b0010, 0b000) => {
            ctx.write::<MAIR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0010, 0b000) => {
        // 	ctx.write::<CNTHPS_TVAL>(data);
        // }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b100) => {
        // 	ctx.write::<ERXFR2>(data);
        // }
        (0b11, 0b100, 0b1010, 0b1001, 0b000) => {
            ctx.write::<VMECID_P_EL2>(data);
        }
        (0b10, 0b011, 0b1001, 0b1100, 0b101) => {
            ctx.write::<SPMSELR_EL0>(data);
        }
        (0b10, 0b000, 0b0111, 0b1001, 0b110) => {
            ctx.write::<DBGCLAIMCLR_EL1>(data);
        }
        // (0b10, 0b001, 0b1000, m[3:0], m[4]:0b00) => {
        // 	ctx.write::<BRBINFn_EL1>(data);
        // }
        (0b11, 0b011, 0b1001, 0b1101, 0b001) => {
            ctx.write::<PMXEVTYPER_EL0>(data);
        }
        (0b10, 0b011, 0b1001, 0b1100, 0b100) => {
            ctx.write::<SPMZR_EL0>(data);
        }
        (0b11, 0b110, 0b1100, 0b0000, 0b001) => {
            ctx.write::<RVBAR_EL3>(data);
        }
        (0b11, 0b011, 0b0100, 0b0010, 0b101) => {
            let dit = (data >> 24) & 1;
            ctx.write::<pstate::DIT>(dit);
        }
        // (0b1111, 0b100, 0b1010, 0b0010, 0b001) => {
        // 	ctx.write::<HMAIR1>(data);
        // }
        (0b11, 0b100, 0b1110, 0b0010, 0b010) => {
            ctx.write::<CNTHP_CVAL_EL2>(data);
        }
        (0b10, 0b001, 0b1001, 0b0001, 0b000) => {
            ctx.write::<BRBINFINJ_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0010, 0b000) => {
        // 	ctx.write::<CNTP_TVAL>(data);
        // }
        (0b11, 0b000, 0b0101, 0b0101, 0b001) => {
            ctx.write::<ERXMISC1_EL1>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b1011, 0b011) => {
        // 	ctx.write::<ICH_EISR>(data);
        // }
        // (0b10, 0b001, 0b0010, m[2:0]:0b0, 0b01:m[3]) => {
        // 	ctx.write::<TRCACATRn>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1110, 0b010) => {
            ctx.write::<PMINTENCLR_EL1>(data);
        }
        // (0b1111, 0b100, 0b1010, 0b0010, 0b000) => {
        // 	ctx.write::<HMAIR0>(data);
        // }
        (0b11, 0b100, 0b1100, 0b0000, 0b001) => {
            ctx.write::<RVBAR_EL2>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b0001, 0b001) => {
        // 	ctx.write::<VDISR>(data);
        // }
        (0b11, 0b011, 0b1101, 0b0010, 0b110) => {
            ctx.write::<AMCG1IDR_EL0>(data);
        }
        (0b11, 0b000, 0b1100, 0b1000, 0b011) => {
            ctx.write::<ICV_BPR0_EL1>(data);
        }
        (0b11, 0b111, 0b1110, 0b0010, 0b000) => {
            ctx.write::<CNTPS_TVAL_EL1>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0000, 0b100) => {
        // 	ctx.write::<TPIDRPRW>(data);
        // }
        // (0b11, 0b011, 0b1101, 0b011:m[3], m[2:0]) => {
        // 	ctx.write::<AMEVTYPER0n_EL0>(data);
        // }
        // (0b1111, 0b100, 0b0001, 0b0000, 0b000) => {
        // 	ctx.write::<HSCTLR>(data);
        // }
        // (0b11, 0b011, 0b1101, 0b111:m[3], m[2:0]) => {
        // 	ctx.write::<AMEVTYPER1n_EL0>(data);
        // }
        (0b11, 0b100, 0b1010, 0b0110, 0b010) => {
            ctx.write::<MPAMVPM2_EL2>(data);
        }
        // (0b1111, 0b100, 0b0010, 0b0000, 0b010) => {
        // 	ctx.write::<HTCR>(data);
        // }
        (0b11, 0b011, 0b0100, 0b0010, 0b000) => {
            let n = (data >> 31) & 1;
            ctx.write::<pstate::N>(n);
            let z = (data >> 30) & 1;
            ctx.write::<pstate::Z>(z);
            let c = (data >> 29) & 1;
            ctx.write::<pstate::C>(c);
            let v = (data >> 28) & 1;
            ctx.write::<pstate::V>(v);
        }
        (0b11, 0b110, 0b1010, 0b0010, 0b000) => {
            ctx.write::<MAIR_EL3>(data);
        }
        (0b11, 0b000, 0b0000, 0b0011, 0b010) => {
            ctx.write::<MVFR2_EL1>(data);
        }
        (0b11, 0b100, 0b1010, 0b0001, 0b001) => {
            ctx.write::<MAIR2_EL2>(data);
        }
        (0b11, 0b100, 0b1010, 0b1000, 0b111) => {
            ctx.write::<MECIDR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b110) => {
        // 	ctx.write::<PMCEID0>(data);
        // }
        (0b11, 0b100, 0b1110, 0b0100, 0b010) => {
            ctx.write::<CNTHVS_CVAL_EL2>(data);
        }
        // (0b10, 0b001, 0b0000, 0b10:m[1:0], 0b100) => {
        // 	ctx.write::<TRCEXTINSELRn>(data);
        // }
        // (0b1110, 0b000, 0b0111, 0b0010, 0b111) => {
        // 	ctx.write::<DBGDEVID>(data);
        // }
        // (0b11, 0b000, 0b1100, 0b1000, 0b1:m[1:0]) => {
        // 	ctx.write::<ICV_AP0Rn_EL1>(data);
        // }
        (0b11, 0b100, 0b0010, 0b0011, 0b011) => {
            ctx.write::<HDBSSPROD_EL2>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b111) => {
        // 	ctx.write::<ID_MMFR3>(data);
        // }
        // (0b1110, 0b000, 0b0010, 0b0000, 0b000) => {
        // 	ctx.write::<DBGDSAR>(data);
        // }
        // (0b11, 0b100, 0b1101, 0b100:m[3], m[2:0]) => {
        // 	ctx.write::<AMEVCNTVOFF0n_EL2>(data);
        // }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b001) => {
        // 	ctx.write::<AMCFGR>(data);
        // }
        // (0b10, 0b011, 0b1110, 0b000:m[3], m[2:0]) => {
        // 	ctx.write::<SPMEVCNTRn_EL0>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b000) => {
            ctx.write::<ICC_IAR1_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b0001, 0b000) => {
            ctx.write::<ISR_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1000, 0b010) => {
            ctx.write::<ICC_HPPIR0_EL1>(data);
        }
        // (0b11, 0b100, 0b1101, 0b101:m[3], m[2:0]) => {
        // 	ctx.write::<AMEVCNTVOFF1n_EL2>(data);
        // }
        (0b11, 0b000, 0b0010, 0b0000, 0b010) => {
            ctx.write::<TCR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b1010, 0b000) => {
            ctx.write::<TRCRSR>(data);
        }
        (0b11, 0b000, 0b0000, 0b0010, 0b011) => {
            ctx.write::<ID_ISAR3_EL1>(data);
        }
        (0b10, 0b001, 0b0111, 0b0010, 0b111) => {
            ctx.write::<TRCDEVID>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0000, 0b000) => {
        // 	ctx.write::<DFSR>(data);
        // }
        (0b11, 0b011, 0b0100, 0b0010, 0b010) => {
            let sm = (data >> 0) & 1;
            ctx.write::<pstate::SM>(sm);
            let za = (data >> 1) & 1;
            ctx.write::<pstate::ZA>(za);
        }
        (0b11, 0b000, 0b0101, 0b0101, 0b000) => {
            ctx.write::<ERXMISC0_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1011, 0b001) => {
            ctx.write::<ICV_DIR_EL1>(data);
        }
        (0b11, 0b110, 0b1100, 0b1100, 0b111) => {
            ctx.write::<ICC_IGRPEN1_EL3>(data);
        }
        (0b11, 0b100, 0b1010, 0b0100, 0b001) => {
            ctx.write::<MPAMVPMV_EL2>(data);
        }
        // (0b1111, 0b100, 0b0110, 0b0000, 0b010) => {
        // 	ctx.write::<HIFAR>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b0000, 0b001) => {
        // 	ctx.write::<MVBAR>(data);
        // }
        (0b11, 0b011, 0b0100, 0b0010, 0b111) => {
            let tco = (data >> 25) & 1;
            ctx.write::<pstate::TCO>(tco);
        }
        (0b11, 0b000, 0b1001, 0b1110, 0b101) => {
            ctx.write::<PMECR_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b011) => {
            ctx.write::<ICV_BPR1_EL1>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b011) => {
        // 	ctx.write::<ID_AFR0>(data);
        // }
        (0b11, 0b000, 0b0110, 0b0000, 0b101) => {
            ctx.write::<PFAR_EL1>(data);
        }
        // (0b10, 0b001, 0b0000, 0b00:m[1:0], 0b101) => {
        // 	ctx.write::<TRCCNTRLDVRn>(data);
        // }
        (0b11, 0b011, 0b1110, 0b0010, 0b001) => {
            ctx.write::<CNTP_CTL_EL0>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b0000, 0b000) => {
        // 	ctx.write::<VBAR>(data);
        // }
        (0b11, 0b011, 0b0100, 0b0010, 0b001) => {
            let d = (data >> 9) & 1;
            ctx.write::<pstate::D>(d);
            let a = (data >> 8) & 1;
            ctx.write::<pstate::A>(a);
            let i = (data >> 7) & 1;
            ctx.write::<pstate::I>(i);
            let f = (data >> 6) & 1;
            ctx.write::<pstate::F>(f);
        }
        // (0b1110, 0b000, 0b0000, 0b0010, 0b000) => {
        // 	ctx.write::<DBGDCCINT>(data);
        // }
        (0b11, 0b000, 0b0001, 0b0010, 0b100) => {
            ctx.write::<SMPRI_EL1>(data);
        }
        (0b11, 0b011, 0b1110, 0b1111, 0b111) => {
            ctx.write::<PMCCFILTR_EL0>(data);
        }
        (0b11, 0b100, 0b1010, 0b0110, 0b011) => {
            ctx.write::<MPAMVPM3_EL2>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b110) => {
        // 	ctx.write::<ID_MMFR2>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0001, 0b010) => {
            ctx.write::<TRCVIIECTLR>(data);
        }
        (0b10, 0b000, 0b0001, 0b0000, 0b100) => {
            ctx.write::<OSLAR_EL1>(data);
        }
        (0b11, 0b011, 0b0000, 0b0000, 0b111) => {
            ctx.write::<DCZID_EL0>(data);
        }
        (0b11, 0b100, 0b0001, 0b0001, 0b111) => {
            ctx.write::<HACR_EL2>(data);
        }
        (0b11, 0b011, 0b0000, 0b0000, 0b001) => {
            ctx.write::<CTR_EL0>(data);
        }
        (0b11, 0b000, 0b0000, 0b0001, 0b011) => {
            ctx.write::<ID_AFR0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b011) => {
        // 	ctx.write::<AMUSERENR>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b100) => {
        // 	ctx.write::<ID_MMFR0>(data);
        // }
        (0b10, 0b011, 0b1001, 0b1100, 0b001) => {
            ctx.write::<SPMCNTENSET_EL0>(data);
        }
        // (0b1111, 0b100, 0b0000, 0b0000, 0b101) => {
        // 	ctx.write::<VMPIDR>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0101, 0b010) => {
            ctx.write::<ID_AA64DFR2_EL1>(data);
        }
        (0b11, 0b000, 0b0111, 0b0100, 0b000) => {
            ctx.write::<PAR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b001) => {
        // 	ctx.write::<ICC_EOIR0>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1010, 0b100) => {
            ctx.write::<PMSDSFR_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0000, 0b101) => {
            ctx.write::<MPIDR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b1110, 0b000) => {
            ctx.write::<TRCCCCTLR>(data);
        }
        (0b11, 0b100, 0b0010, 0b0000, 0b010) => {
            ctx.write::<TCR_EL2>(data);
        }
        (0b11, 0b001, 0b0000, 0b0000, 0b111) => {
            ctx.write::<AIDR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0001, 0b0011, 0b001) => {
        // 	ctx.write::<SDCR>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0100, 0b000) => {
            ctx.write::<ID_AA64PFR0_EL1>(data);
        }
        (0b11, 0b000, 0b0010, 0b0010, 0b010) => {
            ctx.write::<APDBKeyLo_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b111) => {
            ctx.write::<ICV_IGRPEN1_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0011, 0b001) => {
        // 	ctx.write::<CNTV_CTL>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b001) => {
        // 	ctx.write::<ICV_EOIR1>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1011, 0b111) => {
            ctx.write::<TRBIDR_EL1>(data);
        }
        // (0b1110, 0b111, 0b0001, 0b0000, 0b000) => {
        // 	ctx.write::<JOSCR>(data);
        // }
        // (0b1110, 0b111, 0b0010, 0b0000, 0b000) => {
        // 	ctx.write::<JMCR>(data);
        // }
        // (0b1110, 0b000, 0b0000, 0b0111, 0b000) => {
        // 	ctx.write::<DBGVCR>(data);
        // }
        (0b11, 0b100, 0b0001, 0b0010, 0b101) => {
            ctx.write::<SMPRIMAP_EL2>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b1011, 0b001) => {
        // 	ctx.write::<ICH_VTR>(data);
        // }
        (0b10, 0b000, 0b1001, 0b1101, 0b111) => {
            ctx.write::<SPMCFGR_EL1>(data);
        }
        (0b11, 0b100, 0b0010, 0b0001, 0b010) => {
            ctx.write::<VTCR_EL2>(data);
        }
        (0b10, 0b000, 0b0000, 0b0110, 0b010) => {
            ctx.write::<OSECCR_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0111, 0b010) => {
            ctx.write::<ID_AA64MMFR2_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0011, 0b101) => {
            ctx.write::<ID_DFR1_EL1>(data);
        }
        (0b11, 0b100, 0b1110, 0b0010, 0b001) => {
            ctx.write::<CNTHP_CTL_EL2>(data);
        }
        // (0b1111, 0b000, 0b0010, 0b0000, 0b010) => {
        // 	ctx.write::<TTBCR>(data);
        // }
        // (0b11, 0b000, 0b1100, 0b1000, 0b1:m[1:0]) => {
        // 	ctx.write::<ICC_AP0Rn_EL1>(data);
        // }
        (0b11, 0b100, 0b0110, 0b0000, 0b101) => {
            ctx.write::<PFAR_EL2>(data);
        }
        // (0b1111, 0b001, 0b0000, 0b0000, 0b000) => {
        // 	ctx.write::<CCSIDR>(data);
        // }
        (0b10, 0b011, 0b0000, 0b0100, 0b000) => {
            ctx.write::<DBGDTR_EL0>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b111) => {
            ctx.write::<ICC_IGRPEN1_EL1>(data);
        }
        // (0b1111, 0b011, 0b0100, 0b0101, 0b000) => {
        // 	ctx.write::<DSPSR>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b001) => {
        // 	ctx.write::<ICV_EOIR0>(data);
        // }
        (0b11, 0b100, 0b1110, 0b0000, 0b110) => {
            ctx.write::<CNTPOFF_EL2>(data);
        }
        (0b10, 0b001, 0b0000, 0b0000, 0b111) => {
            ctx.write::<TRCIMSPEC0>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b100) => {
        // 	ctx.write::<ICC_CTLR>(data);
        // }
        (0b11, 0b100, 0b0001, 0b0001, 0b100) => {
            ctx.write::<HFGRTR_EL2>(data);
        }
        (0b11, 0b110, 0b0010, 0b0000, 0b010) => {
            ctx.write::<TCR_EL3>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b001) => {
        // 	ctx.write::<ICC_EOIR1>(data);
        // }
        (0b11, 0b011, 0b0100, 0b0100, 0b001) => {
            ctx.write::<FPSR>(data);
        }
        (0b11, 0b000, 0b0101, 0b0100, 0b110) => {
            ctx.write::<ERXPFGCDN_EL1>(data);
        }
        (0b11, 0b000, 0b1001, 0b1001, 0b011) => {
            ctx.write::<PMSIRR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0011, 0b000) => {
        // 	ctx.write::<CNTHVS_TVAL>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b101) => {
        // 	ctx.write::<ID_MMFR1>(data);
        // }
        (0b11, 0b100, 0b0010, 0b0011, 0b101) => {
            ctx.write::<HACDBSCONS_EL2>(data);
        }
        (0b11, 0b100, 0b1110, 0b0011, 0b010) => {
            ctx.write::<CNTHV_CVAL_EL2>(data);
        }
        (0b11, 0b100, 0b0010, 0b0000, 0b000) => {
            ctx.write::<TTBR0_EL2>(data);
        }
        (0b11, 0b000, 0b1010, 0b0100, 0b000) => {
            ctx.write::<LORSA_EL1>(data);
        }
        (0b11, 0b110, 0b1100, 0b1100, 0b100) => {
            ctx.write::<ICC_CTLR_EL3>(data);
        }
        (0b11, 0b000, 0b0001, 0b0010, 0b001) => {
            ctx.write::<TRFCR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b000) => {
        // 	ctx.write::<ICV_IAR1>(data);
        // }
        (0b11, 0b100, 0b1010, 0b0011, 0b001) => {
            ctx.write::<AMAIR2_EL2>(data);
        }
        (0b10, 0b001, 0b1001, 0b0010, 0b000) => {
            ctx.write::<BRBIDR0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b010) => {
        // 	ctx.write::<ICV_HPPIR1>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1001, 0b010) => {
            ctx.write::<PMSICR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b010) => {
        // 	ctx.write::<PMCNTENCLR>(data);
        // }
        (0b11, 0b000, 0b0001, 0b0000, 0b000) => {
            ctx.write::<SCTLR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b010) => {
        // 	ctx.write::<ID_ISAR2>(data);
        // }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b100) => {
        // 	ctx.write::<PMSWINC>(data);
        // }
        (0b11, 0b000, 0b1010, 0b0101, 0b000) => {
            ctx.write::<MPAM1_EL1>(data);
        }
        (0b11, 0b100, 0b1100, 0b1011, 0b000) => {
            ctx.write::<ICH_HCR_EL2>(data);
        }
        (0b11, 0b100, 0b0101, 0b0000, 0b001) => {
            ctx.write::<IFSR32_EL2>(data);
        }
        (0b11, 0b000, 0b0010, 0b0001, 0b001) => {
            ctx.write::<APIAKeyHi_EL1>(data);
        }
        (0b11, 0b110, 0b0001, 0b0001, 0b010) => {
            ctx.write::<CPTR_EL3>(data);
        }
        (0b10, 0b000, 0b1001, 0b1101, 0b101) => {
            ctx.write::<SPMDEVARCH_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b111) => {
        // 	ctx.write::<ICV_IGRPEN1>(data);
        // }
        (0b11, 0b100, 0b1100, 0b1011, 0b111) => {
            ctx.write::<ICH_VMCR_EL2>(data);
        }
        (0b10, 0b001, 0b0000, 0b0001, 0b001) => {
            ctx.write::<TRCQCTLR>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b110) => {
        // 	ctx.write::<ICV_IGRPEN0>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1110, 0b110) => {
            ctx.write::<PMMIR_EL1>(data);
        }
        (0b11, 0b000, 0b0101, 0b0100, 0b000) => {
            ctx.write::<ERXFR_EL1>(data);
        }
        (0b11, 0b100, 0b0001, 0b0001, 0b010) => {
            ctx.write::<CPTR_EL2>(data);
        }
        (0b11, 0b100, 0b1110, 0b0101, 0b010) => {
            ctx.write::<CNTHPS_CVAL_EL2>(data);
        }
        (0b11, 0b100, 0b0010, 0b0110, 0b000) => {
            ctx.write::<VSTTBR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1010, 0b0010, 0b001) => {
        // 	ctx.write::<NMRR>(data);
        // }
        (0b10, 0b110, 0b1001, 0b1110, 0b111) => {
            ctx.write::<SPMROOTCR_EL3>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0011, 0b100) => {
        // 	ctx.write::<ID_PFR2>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0011, 0b000) => {
            ctx.write::<TRCSTATR>(data);
        }
        (0b11, 0b100, 0b0001, 0b0001, 0b011) => {
            ctx.write::<HSTR_EL2>(data);
        }
        (0b11, 0b000, 0b0010, 0b0011, 0b000) => {
            ctx.write::<APGAKeyLo_EL1>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b011) => {
        // 	ctx.write::<ID_ISAR3>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b010) => {
        // 	ctx.write::<ICV_HPPIR0>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1011, 0b010) => {
            ctx.write::<TRBBASER_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b11:m[4:3], m[2:0]) => {
        // 	ctx.write::<PMEVTYPERn>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0001, 0b111) => {
            ctx.write::<ID_MMFR3_EL1>(data);
        }
        (0b11, 0b011, 0b1001, 0b0100, 0b000) => {
            ctx.write::<PMICNTR_EL0>(data);
        }
        (0b11, 0b000, 0b1001, 0b1110, 0b111) => {
            ctx.write::<PMIAR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0001, 0b000) => {
        // 	ctx.write::<ADFSR>(data);
        // }
        (0b11, 0b000, 0b1100, 0b0000, 0b010) => {
            ctx.write::<RMR_EL1>(data);
        }
        (0b11, 0b110, 0b1010, 0b0011, 0b001) => {
            ctx.write::<AMAIR2_EL3>(data);
        }
        (0b11, 0b000, 0b1010, 0b0010, 0b010) => {
            ctx.write::<PIRE0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b000) => {
        // 	ctx.write::<ICV_IAR0>(data);
        // }
        (0b11, 0b110, 0b0010, 0b0000, 0b000) => {
            ctx.write::<TTBR0_EL3>(data);
        }
        (0b11, 0b100, 0b0001, 0b0010, 0b001) => {
            ctx.write::<TRFCR_EL2>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0011, 0b101) => {
        // 	ctx.write::<ID_DFR1>(data);
        // }
        (0b11, 0b000, 0b0010, 0b0000, 0b000) => {
            ctx.write::<TTBR0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b011) => {
        // 	ctx.write::<ICV_BPR0>(data);
        // }
        (0b11, 0b000, 0b1010, 0b0011, 0b001) => {
            ctx.write::<AMAIR2_EL1>(data);
        }
        (0b11, 0b000, 0b1101, 0b0000, 0b011) => {
            ctx.write::<RCWSMASK_EL1>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b0000, 0b000) => {
        // 	ctx.write::<HVBAR>(data);
        // }
        (0b11, 0b110, 0b1100, 0b0000, 0b010) => {
            ctx.write::<RMR_EL3>(data);
        }
        (0b11, 0b011, 0b1101, 0b0010, 0b001) => {
            ctx.write::<AMCFGR_EL0>(data);
        }
        // (0b1111, 0b100, 0b0000, 0b0000, 0b000) => {
        // 	ctx.write::<VPIDR>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b1:m[1:0]) => {
        // 	ctx.write::<ICV_AP0Rn>(data);
        // }
        // (0b1111, 0b000, 0b0001, 0b0000, 0b001) => {
        // 	ctx.write::<ACTLR>(data);
        // }
        (0b11, 0b000, 0b1010, 0b0100, 0b111) => {
            ctx.write::<LORID_EL1>(data);
        }
        (0b11, 0b100, 0b0001, 0b0000, 0b000) => {
            ctx.write::<SCTLR_EL2>(data);
        }
        (0b11, 0b000, 0b0100, 0b0010, 0b011) => {
            let pan = (data >> 22) & 1;
            ctx.write::<pstate::PAN>(pan);
        }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b001) => {
        // 	ctx.write::<ID_ISAR1>(data);
        // }
        (0b11, 0b110, 0b0001, 0b0001, 0b001) => {
            ctx.write::<SDER32_EL3>(data);
        }
        // (0b1110, 0b000, 0b0000, 0b0000, 0b000) => {
        // 	ctx.write::<DBGDIDR>(data);
        // }
        // (0b1111, 0b100, 0b1110, 0b0001, 0b000) => {
        // 	ctx.write::<CNTHCTL>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b000) => {
        // 	ctx.write::<ID_PFR0>(data);
        // }
        (0b10, 0b011, 0b1001, 0b1100, 0b010) => {
            ctx.write::<SPMCNTENCLR_EL0>(data);
        }
        (0b10, 0b001, 0b1001, 0b0000, 0b010) => {
            ctx.write::<BRBTS_EL1>(data);
        }
        // (0b1111, 0b000, 0b0001, 0b0000, 0b011) => {
        // 	ctx.write::<ACTLR2>(data);
        // }
        (0b11, 0b110, 0b0110, 0b0000, 0b101) => {
            ctx.write::<MFAR_EL3>(data);
        }
        (0b10, 0b001, 0b0000, 0b1000, 0b000) => {
            ctx.write::<TRCEVENTCTL0R>(data);
        }
        (0b10, 0b000, 0b1110, 0b1100, 0b000) => {
            ctx.write::<PMICNTSVR_EL1>(data);
        }
        (0b11, 0b100, 0b0010, 0b0110, 0b010) => {
            ctx.write::<VSTCR_EL2>(data);
        }
        (0b11, 0b100, 0b1010, 0b0110, 0b110) => {
            ctx.write::<MPAMVPM6_EL2>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b1:m[1:0]) => {
        // 	ctx.write::<ICC_AP0Rn>(data);
        // }
        (0b10, 0b000, 0b0111, 0b1110, 0b110) => {
            ctx.write::<DBGAUTHSTATUS_EL1>(data);
        }
        (0b11, 0b011, 0b1110, 0b0000, 0b110) => {
            ctx.write::<CNTVCTSS_EL0>(data);
        }
        (0b11, 0b000, 0b0000, 0b0110, 0b010) => {
            ctx.write::<ID_AA64ISAR2_EL1>(data);
        }
        // (0b1110, 0b000, 0b0111, 0b1110, 0b110) => {
        // 	ctx.write::<DBGAUTHSTATUS>(data);
        // }
        (0b11, 0b100, 0b1010, 0b1001, 0b001) => {
            ctx.write::<VMECID_A_EL2>(data);
        }
        (0b11, 0b000, 0b0000, 0b0010, 0b111) => {
            ctx.write::<ID_ISAR6_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b010) => {
            ctx.write::<ICV_HPPIR1_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0011, 0b001) => {
        // 	ctx.write::<CNTHVS_CTL>(data);
        // }
        // (0b1111, 0b000, 0b0110, 0b0000, 0b010) => {
        // 	ctx.write::<IFAR>(data);
        // }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b000) => {
        // 	ctx.write::<PMUSERENR>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b001) => {
        // 	ctx.write::<ID_PFR1>(data);
        // }
        // (0b1110, 0b000, 0b0001, 0b0001, 0b100) => {
        // 	ctx.write::<DBGOSLSR>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0000, 0b110) => {
            ctx.write::<REVIDR_EL1>(data);
        }
        (0b11, 0b100, 0b0001, 0b0001, 0b101) => {
            ctx.write::<HFGWTR_EL2>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b000) => {
        // 	ctx.write::<ID_ISAR0>(data);
        // }
        (0b11, 0b100, 0b0001, 0b0011, 0b001) => {
            ctx.write::<SDER32_EL2>(data);
        }
        (0b11, 0b110, 0b0001, 0b0001, 0b101) => {
            ctx.write::<FGWTE3_EL3>(data);
        }
        (0b11, 0b011, 0b1001, 0b1101, 0b010) => {
            ctx.write::<PMXEVCNTR_EL0>(data);
        }
        // (0b10, 0b011, 0b1110, 0b001:m[3], m[2:0]) => {
        // 	ctx.write::<SPMEVTYPERn_EL0>(data);
        // }
        (0b11, 0b110, 0b0001, 0b0000, 0b000) => {
            ctx.write::<SCTLR_EL3>(data);
        }
        (0b11, 0b100, 0b0011, 0b0001, 0b001) => {
            ctx.write::<HDFGWTR2_EL2>(data);
        }
        (0b11, 0b100, 0b1100, 0b0000, 0b010) => {
            ctx.write::<RMR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b001) => {
        // 	ctx.write::<PMCNTENSET>(data);
        // }
        (0b11, 0b011, 0b1110, 0b0011, 0b010) => {
            ctx.write::<CNTV_CVAL_EL0>(data);
        }
        (0b11, 0b000, 0b0101, 0b0011, 0b001) => {
            ctx.write::<ERRSELR_EL1>(data);
        }
        (0b11, 0b110, 0b0010, 0b0001, 0b100) => {
            ctx.write::<GPTBR_EL3>(data);
        }
        (0b11, 0b011, 0b1001, 0b1100, 0b100) => {
            ctx.write::<PMSWINC_EL0>(data);
        }
        (0b11, 0b100, 0b1010, 0b0010, 0b010) => {
            ctx.write::<PIRE0_EL2>(data);
        }
        (0b11, 0b000, 0b1100, 0b1011, 0b011) => {
            ctx.write::<ICC_RPR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b011) => {
        // 	ctx.write::<ICV_BPR1>(data);
        // }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b010) => {
        // 	ctx.write::<HCPTR>(data);
        // }
        (0b10, 0b001, 0b0001, 0b0001, 0b100) => {
            ctx.write::<TRCOSLSR>(data);
        }
        (0b11, 0b000, 0b0100, 0b0110, 0b000) => {
            ctx.write::<ICC_PMR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0001, 0b010) => {
        // 	ctx.write::<ID_DFR0>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1100, 0b100) => {
            ctx.write::<ICC_CTLR_EL1>(data);
        }
        // (0b10, 0b001, 0b1000, m[3:0], m[4]:0b01) => {
        // 	ctx.write::<BRBSRCn_EL1>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0100, 0b101) => {
            ctx.write::<ID_AA64SMFR0_EL1>(data);
        }
        // (0b1111, 0b100, 0b1101, 0b0000, 0b010) => {
        // 	ctx.write::<HTPIDR>(data);
        // }
        (0b11, 0b110, 0b0001, 0b0010, 0b000) => {
            ctx.write::<ZCR_EL3>(data);
        }
        (0b11, 0b000, 0b0101, 0b0100, 0b010) => {
            ctx.write::<ERXSTATUS_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b000) => {
        // 	ctx.write::<ICC_IAR0>(data);
        // }
        (0b11, 0b000, 0b0010, 0b0000, 0b001) => {
            ctx.write::<TTBR1_EL1>(data);
        }
        (0b11, 0b100, 0b1110, 0b0000, 0b011) => {
            ctx.write::<CNTVOFF_EL2>(data);
        }
        (0b11, 0b000, 0b0010, 0b0000, 0b011) => {
            ctx.write::<TCR2_EL1>(data);
        }
        (0b11, 0b100, 0b0001, 0b0001, 0b001) => {
            ctx.write::<MDCR_EL2>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b1000, 0b0:m[1:0]) => {
        // 	ctx.write::<ICH_AP0Rn>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1011, 0b011) => {
        // 	ctx.write::<ICC_RPR>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b100) => {
        // 	ctx.write::<ID_ISAR4>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0010, 0b001) => {
            ctx.write::<TRCITEEDCR>(data);
        }
        (0b10, 0b110, 0b1001, 0b1101, 0b011) => {
            ctx.write::<SPMACCESSR_EL3>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b0000, 0b010) => {
        // 	ctx.write::<HRMR>(data);
        // }
        (0b11, 0b100, 0b1010, 0b0110, 0b111) => {
            ctx.write::<MPAMVPM7_EL2>(data);
        }
        (0b11, 0b100, 0b1010, 0b0010, 0b100) => {
            ctx.write::<POR_EL2>(data);
        }
        (0b11, 0b110, 0b0010, 0b0101, 0b001) => {
            ctx.write::<GCSPR_EL3>(data);
        }
        // (0b1110, 0b000, 0b0000, m[3:0], 0b111) => {
        // 	ctx.write::<DBGWCRn>(data);
        // }
        (0b11, 0b100, 0b0110, 0b0000, 0b000) => {
            ctx.write::<FAR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1011, 0b011) => {
        // 	ctx.write::<ICV_RPR>(data);
        // }
        (0b11, 0b100, 0b0101, 0b0110, 0b000) => {
            ctx.write::<TFSR_EL2>(data);
        }
        // (0b10, 0b001, 0b0000, 0b01:m[1:0], 0b101) => {
        // 	ctx.write::<TRCCNTCTLRn>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1000, 0b010) => {
            ctx.write::<ICV_HPPIR0_EL1>(data);
        }
        (0b11, 0b100, 0b0011, 0b0001, 0b000) => {
            ctx.write::<HDFGRTR2_EL2>(data);
        }
        (0b11, 0b110, 0b0101, 0b0110, 0b000) => {
            ctx.write::<TFSR_EL3>(data);
        }
        (0b10, 0b001, 0b0000, 0b1001, 0b000) => {
            ctx.write::<TRCEVENTCTL1R>(data);
        }
        (0b11, 0b110, 0b0110, 0b0000, 0b000) => {
            ctx.write::<FAR_EL3>(data);
        }
        (0b11, 0b100, 0b0010, 0b0101, 0b001) => {
            ctx.write::<GCSPR_EL2>(data);
        }
        (0b11, 0b110, 0b1010, 0b0010, 0b100) => {
            ctx.write::<POR_EL3>(data);
        }
        (0b11, 0b100, 0b0001, 0b0001, 0b110) => {
            ctx.write::<HFGITR_EL2>(data);
        }
        (0b11, 0b000, 0b0000, 0b0110, 0b011) => {
            ctx.write::<ID_AA64ISAR3_EL1>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0000, 0b010) => {
        // 	ctx.write::<TCMTR>(data);
        // }
        (0b10, 0b100, 0b1001, 0b1101, 0b011) => {
            ctx.write::<SPMACCESSR_EL2>(data);
        }
        // (0b1111, 0b011, 0b0100, 0b0101, 0b001) => {
        // 	ctx.write::<DLR>(data);
        // }
        (0b10, 0b000, 0b0000, 0b0101, 0b010) => {
            ctx.write::<MDSTEPOP_EL1>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b010) => {
        // 	ctx.write::<AMCGCR>(data);
        // }
        // (0b1111, 0b100, 0b1100, 0b110:m[3], m[2:0]) => {
        // 	ctx.write::<ICH_LRn>(data);
        // }
        (0b11, 0b000, 0b0101, 0b0100, 0b011) => {
            ctx.write::<ERXADDR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b101) => {
        // 	ctx.write::<ID_ISAR5>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1110, 0b100) => {
            ctx.write::<PMUACR_EL1>(data);
        }
        (0b11, 0b110, 0b0001, 0b0011, 0b001) => {
            ctx.write::<MDCR_EL3>(data);
        }
        (0b11, 0b000, 0b1001, 0b1001, 0b000) => {
            ctx.write::<PMSCR_EL1>(data);
        }
        // (0b10, 0b001, 0b0001, 0b1:m[2:0], 0b010) => {
        // 	ctx.write::<TRCSSCSRn>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b000) => {
        // 	ctx.write::<ICC_IAR1>(data);
        // }
        (0b11, 0b100, 0b0001, 0b0010, 0b000) => {
            ctx.write::<ZCR_EL2>(data);
        }
        (0b11, 0b000, 0b0101, 0b0100, 0b001) => {
            ctx.write::<ERXCTLR_EL1>(data);
        }
        (0b10, 0b000, 0b0111, 0b1000, 0b110) => {
            ctx.write::<DBGCLAIMSET_EL1>(data);
        }
        (0b10, 0b100, 0b0000, 0b0111, 0b000) => {
            ctx.write::<DBGVCR32_EL2>(data);
        }
        (0b11, 0b000, 0b1101, 0b0000, 0b110) => {
            ctx.write::<RCWMASK_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b0110, 0b100) => {
            ctx.write::<TRCSEQRSTEVR>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0000, 0b000) => {
        // 	ctx.write::<FCSEIDR>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b011) => {
        // 	ctx.write::<ICC_BPR1>(data);
        // }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b000) => {
        // 	ctx.write::<AMCR>(data);
        // }
        (0b11, 0b100, 0b0010, 0b0000, 0b011) => {
            ctx.write::<TCR2_EL2>(data);
        }
        (0b11, 0b100, 0b0010, 0b0000, 0b001) => {
            ctx.write::<TTBR1_EL2>(data);
        }
        // (0b10, 0b001, 0b0000, 0b0:m[2:0], 0b111) => {
        // 	ctx.write::<TRCIMSPECn>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1010, 0b111) => {
            ctx.write::<PMBIDR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0011, 0b001) => {
        // 	ctx.write::<CNTHV_CTL>(data);
        // }
        (0b11, 0b011, 0b1001, 0b1101, 0b100) => {
            ctx.write::<PMZR_EL0>(data);
        }
        // (0b1111, 0b000, 0b0001, 0b0001, 0b000) => {
        // 	ctx.write::<SCR>(data);
        // }
        // (0b1110, 0b000, 0b0000, 0b0101, 0b000) => {
        // 	ctx.write::<DBGDTRTXint>(data);
        // }
        (0b11, 0b000, 0b1010, 0b0010, 0b100) => {
            ctx.write::<POR_EL1>(data);
        }
        // (0b1110, 0b000, 0b0000, 0b0011, 0b010) => {
        // 	ctx.write::<DBGDTRTXext>(data);
        // }
        (0b11, 0b011, 0b0010, 0b0101, 0b001) => {
            ctx.write::<GCSPR_EL0>(data);
        }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b000) => {
        // 	ctx.write::<HCR>(data);
        // }
        (0b11, 0b000, 0b0110, 0b0000, 0b000) => {
            ctx.write::<FAR_EL1>(data);
        }
        (0b11, 0b000, 0b1010, 0b0101, 0b001) => {
            ctx.write::<MPAM0_EL1>(data);
        }
        (0b11, 0b011, 0b1110, 0b0010, 0b010) => {
            ctx.write::<CNTP_CVAL_EL0>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0000, 0b011) => {
        // 	ctx.write::<TLBTR>(data);
        // }
        (0b11, 0b000, 0b0101, 0b0110, 0b000) => {
            ctx.write::<TFSR_EL1>(data);
        }
        // (0b11, 0b100, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.write::<ICH_AP1Rn_EL2>(data);
        // }
        // (0b1111, 0b000, 0b0011, 0b0000, 0b000) => {
        // 	ctx.write::<DACR>(data);
        // }
        (0b11, 0b000, 0b0010, 0b0010, 0b000) => {
            ctx.write::<APDAKeyLo_EL1>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b1001, 0b101) => {
        // 	ctx.write::<ICC_HSRE>(data);
        // }
        (0b11, 0b000, 0b0100, 0b0010, 0b010) => {
            let el = (data >> 2) & 0b11;
            ctx.write::<pstate::EL>(el);
        }
        (0b11, 0b100, 0b1010, 0b0010, 0b101) => {
            ctx.write::<S2PIR_EL2>(data);
        }
        (0b11, 0b000, 0b0010, 0b0101, 0b001) => {
            ctx.write::<GCSPR_EL1>(data);
        }
        // (0b1111, 0b100, 0b1110, 0b0010, 0b000) => {
        // 	ctx.write::<CNTHP_TVAL>(data);
        // }
        (0b11, 0b011, 0b1010, 0b0010, 0b100) => {
            ctx.write::<POR_EL0>(data);
        }
        // (0b10, 0b001, 0b0000, 0b10:m[1:0], 0b101) => {
        // 	ctx.write::<TRCCNTVRn>(data);
        // }
        (0b10, 0b000, 0b1001, 0b1101, 0b011) => {
            ctx.write::<SPMACCESSR_EL1>(data);
        }
        (0b11, 0b000, 0b0101, 0b0110, 0b001) => {
            ctx.write::<TFSRE0_EL1>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0010, 0b111) => {
        // 	ctx.write::<ID_ISAR6>(data);
        // }
        (0b11, 0b011, 0b1101, 0b0010, 0b000) => {
            ctx.write::<AMCR_EL0>(data);
        }
        (0b11, 0b000, 0b1001, 0b1110, 0b001) => {
            ctx.write::<PMINTENSET_EL1>(data);
        }
        (0b11, 0b100, 0b1001, 0b1001, 0b000) => {
            ctx.write::<PMSCR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1000, 0b011) => {
        // 	ctx.write::<ICC_BPR0>(data);
        // }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b111) => {
        // 	ctx.write::<HACR>(data);
        // }
        (0b11, 0b000, 0b0001, 0b0010, 0b000) => {
            ctx.write::<ZCR_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0001, 0b110) => {
            ctx.write::<ID_MMFR2_EL1>(data);
        }
        (0b11, 0b000, 0b1101, 0b0000, 0b001) => {
            ctx.write::<CONTEXTIDR_EL1>(data);
        }
        (0b11, 0b100, 0b1010, 0b0110, 0b101) => {
            ctx.write::<MPAMVPM5_EL2>(data);
        }
        (0b11, 0b000, 0b0000, 0b0000, 0b000) => {
            ctx.write::<MIDR_EL1>(data);
        }
        (0b11, 0b000, 0b1010, 0b0100, 0b010) => {
            ctx.write::<LORN_EL1>(data);
        }
        (0b11, 0b100, 0b0101, 0b0010, 0b011) => {
            ctx.write::<VSESR_EL2>(data);
        }
        (0b11, 0b100, 0b0101, 0b0001, 0b001) => {
            ctx.write::<AFSR1_EL2>(data);
        }
        (0b11, 0b100, 0b1010, 0b1000, 0b001) => {
            ctx.write::<MECID_A0_EL2>(data);
        }
        (0b11, 0b100, 0b1010, 0b0101, 0b000) => {
            ctx.write::<MPAM2_EL2>(data);
        }
        (0b11, 0b100, 0b1100, 0b1011, 0b011) => {
            ctx.write::<ICH_EISR_EL2>(data);
        }
        (0b10, 0b000, 0b1001, 0b1101, 0b110) => {
            ctx.write::<SPMDEVAFF_EL1>(data);
        }
        // (0b10, 0b001, 0b0010, m[2:0]:0b0, 0b00:m[3]) => {
        // 	ctx.write::<TRCACVRn>(data);
        // }
        (0b11, 0b110, 0b0100, 0b0001, 0b000) => {
            ctx.write::<SP_EL2>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.write::<ICV_AP1Rn>(data);
        // }
        (0b11, 0b000, 0b0010, 0b0010, 0b001) => {
            ctx.write::<APDAKeyHi_EL1>(data);
        }
        // (0b1111, 0b100, 0b0001, 0b0000, 0b001) => {
        // 	ctx.write::<HACTLR>(data);
        // }
        // (0b10, 0b000, 0b0000, m[3:0], 0b100) => {
        // 	ctx.write::<DBGBVRn_EL1>(data);
        // }
        (0b11, 0b100, 0b1110, 0b0100, 0b001) => {
            ctx.write::<CNTHVS_CTL_EL2>(data);
        }
        (0b11, 0b100, 0b1010, 0b0010, 0b011) => {
            ctx.write::<PIR_EL2>(data);
        }
        (0b11, 0b000, 0b0100, 0b0011, 0b001) => {
            let pm = (data >> 32) & 1;
            ctx.write::<pstate::PM>(pm);
        }
        (0b11, 0b110, 0b1010, 0b0010, 0b011) => {
            ctx.write::<PIR_EL3>(data);
        }
        (0b11, 0b110, 0b1010, 0b1010, 0b001) => {
            ctx.write::<MECID_RL_A_EL3>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1101, 0b000) => {
        // 	ctx.write::<PMCCNTR>(data);
        // }
        // (0b11, 0b100, 0b1100, 0b110:m[3], m[2:0]) => {
        // 	ctx.write::<ICH_LRn_EL2>(data);
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b001) => {
            ctx.write::<PMCNTENSET_EL0>(data);
        }
        (0b11, 0b000, 0b1010, 0b0010, 0b101) => {
            ctx.write::<S2POR_EL1>(data);
        }
        // (0b10, 0b011, 0b1110, 0b011:m[3], m[2:0]) => {
        // 	ctx.write::<SPMEVFILT2Rn_EL0>(data);
        // }
        (0b11, 0b100, 0b0001, 0b0001, 0b000) => {
            ctx.write::<HCR_EL2>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b1011, 0b101) => {
        // 	ctx.write::<ICH_ELRSR>(data);
        // }
        // (0b1111, 0b000, 0b1101, 0b011:m[3], m[2:0]) => {
        // 	ctx.write::<AMEVTYPER0n>(data);
        // }
        // (0b1111, 0b000, 0b1001, 0b1100, 0b101) => {
        // 	ctx.write::<PMSELR>(data);
        // }
        (0b11, 0b110, 0b0101, 0b0010, 0b011) => {
            ctx.write::<VSESR_EL3>(data);
        }
        (0b11, 0b110, 0b0101, 0b0001, 0b001) => {
            ctx.write::<AFSR1_EL3>(data);
        }
        (0b11, 0b000, 0b0000, 0b0010, 0b101) => {
            ctx.write::<ID_ISAR5_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b0001, 0b000) => {
            ctx.write::<TRCPRGCTLR>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.write::<ICC_AP1Rn>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0110, 0b001) => {
            ctx.write::<ID_AA64ISAR1_EL1>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b011) => {
        // 	ctx.write::<PMOVSSET>(data);
        // }
        (0b10, 0b000, 0b0000, 0b0010, 0b000) => {
            ctx.write::<MDCCINT_EL1>(data);
        }
        // (0b10, 0b000, 0b0000, m[3:0], 0b101) => {
        // 	ctx.write::<DBGBCRn_EL1>(data);
        // }
        (0b10, 0b001, 0b0111, 0b1001, 0b110) => {
            ctx.write::<TRCCLAIMCLR>(data);
        }
        // (0b10, 0b001, 0b0001, m[3:0], 0b00:m[4]) => {
        // 	ctx.write::<TRCRSCTLRn>(data);
        // }
        (0b10, 0b011, 0b1001, 0b1110, 0b011) => {
            ctx.write::<SPMOVSSET_EL0>(data);
        }
        (0b11, 0b011, 0b1001, 0b1110, 0b000) => {
            ctx.write::<PMUSERENR_EL0>(data);
        }
        (0b11, 0b100, 0b1101, 0b0000, 0b001) => {
            ctx.write::<CONTEXTIDR_EL2>(data);
        }
        (0b11, 0b111, 0b1110, 0b0010, 0b010) => {
            ctx.write::<CNTPS_CVAL_EL1>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b10:m[4:3], m[2:0]) => {
        // 	ctx.write::<PMEVCNTRn>(data);
        // }
        (0b10, 0b001, 0b0111, 0b1000, 0b110) => {
            ctx.write::<TRCCLAIMSET>(data);
        }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b110) => {
        // 	ctx.write::<PMMIR>(data);
        // }
        (0b11, 0b011, 0b0100, 0b0100, 0b000) => {
            ctx.write::<FPCR>(data);
        }
        (0b11, 0b000, 0b0101, 0b0001, 0b001) => {
            ctx.write::<AFSR1_EL1>(data);
        }
        (0b11, 0b011, 0b1110, 0b0000, 0b101) => {
            ctx.write::<CNTPCTSS_EL0>(data);
        }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b001) => {
        // 	ctx.write::<HDCR>(data);
        // }
        // (0b1111, 0b000, 0b1100, 0b0001, 0b001) => {
        // 	ctx.write::<DISR>(data);
        // }
        (0b11, 0b100, 0b1110, 0b0100, 0b000) => {
            ctx.write::<CNTHVS_TVAL_EL2>(data);
        }
        (0b11, 0b100, 0b0100, 0b0001, 0b000) => {
            ctx.write::<SP_EL1>(data);
        }
        // (0b1110, 0b000, 0b0000, m[3:0], 0b100) => {
        // 	ctx.write::<DBGBVRn>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1011, 0b001) => {
            ctx.write::<TRBPTR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0000, 0b011) => {
        // 	ctx.write::<TPIDRURO>(data);
        // }
        (0b11, 0b001, 0b0000, 0b0000, 0b001) => {
            ctx.write::<CLIDR_EL1>(data);
        }
        (0b11, 0b000, 0b1010, 0b0010, 0b011) => {
            ctx.write::<PIR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b0010, 0b010) => {
            ctx.write::<TRCVISSCTLR>(data);
        }
        (0b11, 0b011, 0b0010, 0b0100, 0b000) => {
            panic!("RNDR is read-only!")
        }
        // (0b1111, 0b100, 0b1100, 0b111:m[3], m[2:0]) => {
        // 	ctx.write::<ICH_LRCn>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0111, 0b100) => {
            ctx.write::<ID_AA64MMFR4_EL1>(data);
        }
        (0b11, 0b000, 0b1100, 0b0001, 0b001) => {
            ctx.write::<DISR_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0001, 0b100) => {
            ctx.write::<ID_MMFR0_EL1>(data);
        }
        // (0b1111, 0b100, 0b0101, 0b0001, 0b000) => {
        // 	ctx.write::<HADFSR>(data);
        // }
        // (0b1111, 0b000, 0b1110, 0b0010, 0b001) => {
        // 	ctx.write::<CNTP_CTL>(data);
        // }
        (0b11, 0b000, 0b0100, 0b0001, 0b000) => {
            ctx.write::<SP_EL0>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0011, 0b000) => {
        // 	ctx.write::<CNTHV_TVAL>(data);
        // }
        // (0b1111, 0b100, 0b1100, 0b1011, 0b000) => {
        // 	ctx.write::<ICH_HCR>(data);
        // }
        (0b11, 0b100, 0b1110, 0b0010, 0b000) => {
            ctx.write::<CNTHP_TVAL_EL2>(data);
        }
        // (0b10, 0b000, 0b1001, 0b1101, 0b00:m[0]) => {
        // 	ctx.write::<SPMCGCRn_EL1>(data);
        // }
        (0b11, 0b000, 0b0100, 0b0010, 0b000) => {
            let sp = (data >> 0) & 1;
            ctx.write::<pstate::SP>(sp);
        }
        (0b11, 0b000, 0b1001, 0b1011, 0b011) => {
            ctx.write::<TRBSR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b0100, 0b000) => {
            ctx.write::<TRCCONFIGR>(data);
        }
        // (0b1111, 0b000, 0b0000, 0b0000, 0b110) => {
        // 	ctx.write::<REVIDR>(data);
        // }
        (0b10, 0b001, 0b0111, 0b1111, 0b110) => {
            ctx.write::<TRCDEVARCH>(data);
        }
        (0b11, 0b000, 0b0101, 0b0001, 0b000) => {
            ctx.write::<AFSR0_EL1>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b111) => {
        // 	ctx.write::<ICC_IGRPEN1>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0000, 0b000) => {
        // 	ctx.write::<MIDR>(data);
        // }
        (0b11, 0b011, 0b0010, 0b0100, 0b001) => {
            ctx.write::<RNDRRS>(data);
        }
        (0b11, 0b100, 0b1100, 0b1011, 0b010) => {
            ctx.write::<ICH_MISR_EL2>(data);
        }
        (0b11, 0b000, 0b1001, 0b1001, 0b111) => {
            ctx.write::<PMSIDR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0011, 0b001) => {
        // 	ctx.write::<AMCNTENSET1>(data);
        // }
        (0b10, 0b001, 0b0000, 0b1011, 0b000) => {
            ctx.write::<TRCSTALLCTLR>(data);
        }
        (0b11, 0b011, 0b1001, 0b1101, 0b000) => {
            ctx.write::<PMCCNTR_EL0>(data);
        }
        // (0b10, 0b001, 0b0011, m[2:0]:0b0, 0b000) => {
        // 	ctx.write::<TRCCIDCVRn>(data);
        // }
        (0b10, 0b000, 0b1001, 0b1110, 0b010) => {
            ctx.write::<SPMINTENCLR_EL1>(data);
        }
        (0b10, 0b001, 0b1001, 0b0001, 0b001) => {
            ctx.write::<BRBSRCINJ_EL1>(data);
        }
        (0b11, 0b100, 0b0000, 0b0000, 0b000) => {
            ctx.write::<VPIDR_EL2>(data);
        }
        // (0b1111, 0b000, 0b1010, 0b0010, 0b000) => {
        // 	ctx.write::<PRRR>(data);
        // }
        (0b11, 0b100, 0b0010, 0b0010, 0b000) => {
            ctx.write::<VNCR_EL2>(data);
        }
        (0b11, 0b010, 0b0000, 0b0000, 0b000) => {
            ctx.write::<CSSELR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b0101, 0b110) => {
            ctx.write::<TRCIDR13>(data);
        }
        (0b11, 0b100, 0b0010, 0b0101, 0b000) => {
            ctx.write::<GCSCR_EL2>(data);
        }
        (0b11, 0b000, 0b0101, 0b0100, 0b101) => {
            ctx.write::<ERXPFGCTL_EL1>(data);
        }
        (0b10, 0b000, 0b0000, 0b0010, 0b010) => {
            ctx.write::<MDSCR_EL1>(data);
        }
        // (0b1111, 0b001, 0b0000, 0b0000, 0b111) => {
        // 	ctx.write::<AIDR>(data);
        // }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b010) => {
        // 	ctx.write::<PMINTENCLR>(data);
        // }
        (0b11, 0b100, 0b0011, 0b0001, 0b101) => {
            ctx.write::<HDFGWTR_EL2>(data);
        }
        (0b11, 0b000, 0b0000, 0b0001, 0b101) => {
            ctx.write::<ID_MMFR1_EL1>(data);
        }
        (0b11, 0b000, 0b1001, 0b1011, 0b101) => {
            ctx.write::<TRBMPAM_EL1>(data);
        }
        (0b11, 0b110, 0b0010, 0b0101, 0b000) => {
            ctx.write::<GCSCR_EL3>(data);
        }
        (0b10, 0b001, 0b0000, 0b0100, 0b110) => {
            ctx.write::<TRCIDR12>(data);
        }
        (0b11, 0b100, 0b0011, 0b0001, 0b110) => {
            ctx.write::<HAFGRTR_EL2>(data);
        }
        // (0b1110, 0b000, 0b0000, 0b0010, 0b010) => {
        // 	ctx.write::<DBGDSCRext>(data);
        // }
        // (0b1110, 0b000, 0b0000, 0b0001, 0b000) => {
        // 	ctx.write::<DBGDSCRint>(data);
        // }
        (0b11, 0b100, 0b0010, 0b0001, 0b000) => {
            ctx.write::<VTTBR_EL2>(data);
        }
        (0b10, 0b000, 0b0001, 0b0011, 0b100) => {
            ctx.write::<OSDLR_EL1>(data);
        }
        (0b10, 0b000, 0b1110, 0b1011, 0b111) => {
            ctx.write::<PMCCNTSVR_EL1>(data);
        }
        // (0b1111, 0b100, 0b0101, 0b0010, 0b011) => {
        // 	ctx.write::<VDFSR>(data);
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b011) => {
            ctx.write::<PMOVSCLR_EL0>(data);
        }
        (0b11, 0b100, 0b1110, 0b0101, 0b001) => {
            ctx.write::<CNTHPS_CTL_EL2>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b0010, 0b101) => {
        // 	ctx.write::<AMCNTENSET0>(data);
        // }
        // (0b1111, 0b011, 0b0100, 0b0101, 0b010) => {
        // 	ctx.write::<DSPSR2>(data);
        // }
        (0b11, 0b011, 0b1001, 0b1100, 0b000) => {
            ctx.write::<PMCR_EL0>(data);
        }
        // (0b1111, 0b100, 0b0001, 0b0001, 0b011) => {
        // 	ctx.write::<HSTR>(data);
        // }
        (0b10, 0b001, 0b0000, 0b1101, 0b000) => {
            ctx.write::<TRCSYNCPR>(data);
        }
        // (0b1111, 0b000, 0b1100, 0b1100, 0b110) => {
        // 	ctx.write::<ICC_IGRPEN0>(data);
        // }
        (0b11, 0b100, 0b0101, 0b0001, 0b000) => {
            ctx.write::<AFSR0_EL2>(data);
        }
        (0b10, 0b000, 0b0000, 0b0100, 0b010) => {
            ctx.write::<MDSELR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1010, 0b0011, 0b001) => {
        // 	ctx.write::<AMAIR1>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1001, 0b001) => {
            ctx.write::<PMSNEVFR_EL1>(data);
        }
        (0b11, 0b000, 0b0100, 0b0010, 0b100) => {
            let uao = (data >> 23) & 1;
            ctx.write::<pstate::UAO>(uao);
        }
        (0b11, 0b000, 0b0101, 0b0100, 0b100) => {
            ctx.write::<ERXPFGF_EL1>(data);
        }
        // (0b1111, 0b100, 0b0110, 0b0000, 0b100) => {
        // 	ctx.write::<HPFAR>(data);
        // }
        (0b11, 0b100, 0b1010, 0b1000, 0b011) => {
            ctx.write::<MECID_A1_EL2>(data);
        }
        (0b11, 0b100, 0b1010, 0b0110, 0b100) => {
            ctx.write::<MPAMVPM4_EL2>(data);
        }
        (0b11, 0b000, 0b1100, 0b1100, 0b100) => {
            ctx.write::<ICV_CTLR_EL1>(data);
        }
        // (0b1111, 0b000, 0b1101, 0b111:m[3], m[2:0]) => {
        // 	ctx.write::<AMEVTYPER1n>(data);
        // }
        (0b11, 0b100, 0b0101, 0b0011, 0b000) => {
            ctx.write::<FPEXC32_EL2>(data);
        }
        (0b11, 0b011, 0b1101, 0b0000, 0b011) => {
            ctx.write::<TPIDRRO_EL0>(data);
        }
        // (0b1110, 0b000, 0b0001, 0b0100, 0b100) => {
        // 	ctx.write::<DBGPRCR>(data);
        // }
        (0b11, 0b100, 0b1110, 0b0011, 0b000) => {
            ctx.write::<CNTHV_TVAL_EL2>(data);
        }
        // (0b1111, 0b010, 0b0000, 0b0000, 0b000) => {
        // 	ctx.write::<CSSELR>(data);
        // }
        (0b10, 0b001, 0b0000, 0b0000, 0b110) => {
            ctx.write::<TRCIDR8>(data);
        }
        (0b11, 0b100, 0b1110, 0b0001, 0b000) => {
            ctx.write::<CNTHCTL_EL2>(data);
        }
        // (0b1111, 0b100, 0b1100, 0b1001, 0b0:m[1:0]) => {
        // 	ctx.write::<ICH_AP1Rn>(data);
        // }
        // (0b1111, 0b000, 0b0010, 0b0000, 0b001) => {
        // 	ctx.write::<TTBR1>(data);
        // }
        (0b11, 0b000, 0b0010, 0b0101, 0b000) => {
            ctx.write::<GCSCR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b0010, 0b110) => {
            ctx.write::<TRCIDR10>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0100, 0b101) => {
        // 	ctx.write::<ERXCTLR2>(data);
        // }
        // (0b1111, 0b000, 0b1001, 0b1110, 0b001) => {
        // 	ctx.write::<PMINTENSET>(data);
        // }
        (0b11, 0b000, 0b0010, 0b0011, 0b001) => {
            ctx.write::<APGAKeyHi_EL1>(data);
        }
        // (0b1111, 0b001, 0b0000, 0b0000, 0b001) => {
        // 	ctx.write::<CLIDR>(data);
        // }
        // (0b11, 0b011, 0b1110, 0b10:m[4:3], m[2:0]) => {
        // 	ctx.write::<PMEVCNTRn_EL0>(data);
        // }
        // (0b1110, 0b000, 0b0001, 0b0000, 0b100) => {
        // 	ctx.write::<DBGOSLAR>(data);
        // }
        (0b11, 0b100, 0b0011, 0b0000, 0b000) => {
            ctx.write::<DACR32_EL2>(data);
        }
        // (0b1111, 0b000, 0b1110, 0b0010, 0b001) => {
        // 	ctx.write::<CNTHPS_CTL>(data);
        // }
        (0b11, 0b000, 0b0100, 0b0110, 0b000) => {
            ctx.write::<ICV_PMR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0101, 0b0000, 0b001) => {
        // 	ctx.write::<IFSR>(data);
        // }
        // (0b11, 0b011, 0b1101, 0b010:m[3], m[2:0]) => {
        // 	ctx.write::<AMEVCNTR0n_EL0>(data);
        // }
        // (0b11, 0b011, 0b1101, 0b110:m[3], m[2:0]) => {
        // 	ctx.write::<AMEVCNTR1n_EL0>(data);
        // }
        // (0b1111, 0b000, 0b0000, 0b0000, 0b001) => {
        // 	ctx.write::<CTR>(data);
        // }
        (0b11, 0b000, 0b1100, 0b1011, 0b011) => {
            ctx.write::<ICV_RPR_EL1>(data);
        }
        (0b10, 0b001, 0b0000, 0b0011, 0b110) => {
            ctx.write::<TRCIDR11>(data);
        }
        // (0b1111, 0b100, 0b0110, 0b0000, 0b000) => {
        // 	ctx.write::<HDFAR>(data);
        // }
        (0b11, 0b000, 0b1001, 0b1010, 0b011) => {
            ctx.write::<PMBSR_EL1>(data);
        }
        // (0b1111, 0b000, 0b0010, 0b0000, 0b000) => {
        // 	ctx.write::<TTBR0>(data);
        // }
        (0b11, 0b011, 0b1110, 0b0000, 0b000) => {
            ctx.write::<CNTFRQ_EL0>(data);
        }
        (0b10, 0b001, 0b0000, 0b0001, 0b110) => {
            ctx.write::<TRCIDR9>(data);
        }
        (0b10, 0b001, 0b0111, 0b1110, 0b110) => {
            ctx.write::<TRCAUTHSTATUS>(data);
        }
        (0b11, 0b000, 0b1100, 0b1001, 0b101) => {
            ctx.write::<ICV_NMIAR1_EL1>(data);
        }
        (0b11, 0b000, 0b0010, 0b0001, 0b000) => {
            ctx.write::<APIAKeyLo_EL1>(data);
        }
        (0b11, 0b000, 0b0000, 0b0110, 0b000) => {
            ctx.write::<ID_AA64ISAR0_EL1>(data);
        }
        // (0b11, 0b100, 0b1100, 0b1000, 0b0:m[1:0]) => {
        // 	ctx.write::<ICH_AP0Rn_EL2>(data);
        // }
        // (0b1111, 0b000, 0b0101, 0b0011, 0b001) => {
        // 	ctx.write::<ERRSELR>(data);
        // }
        (0b11, 0b110, 0b1010, 0b0101, 0b000) => {
            ctx.write::<MPAM3_EL3>(data);
        }
        // (0b1111, 0b000, 0b1010, 0b0011, 0b000) => {
        // 	ctx.write::<AMAIR0>(data);
        // }
        (0b11, 0b000, 0b0000, 0b0010, 0b100) => {
            ctx.write::<ID_ISAR4_EL1>(data);
        }
        (0b11, 0b110, 0b0101, 0b0001, 0b000) => {
            ctx.write::<AFSR0_EL3>(data);
        }
        _ => panic!(
            "Unknown MSR (register) {0} {1} {2} {3} {4}",
            op0, op1, crn, crm, op2
        ),
    }
}
