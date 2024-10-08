use crate::core::model::proc::{ProcDecl, Register, RegisterArray};
use crate::std::arm::cpu::ArmCpu;

/// Selected Error Record Control Register
pub struct ERXCTLR;
impl Register for ERXCTLR {
    const NAME: &'static str = "ERXCTLR";
    const LEN: usize = 64;
}

/// ERXCTLR register fields
pub mod erxctlr {
    use crate::core::model::proc::RegisterView;

    /// ERXCTLR\[31:0\]
    pub struct ERRnCTLRlo;
    impl RegisterView for ERRnCTLRlo {
        type Register = super::ERXCTLR;
        const NAME: &'static str = "ERRnCTLRlo";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Media and VFP Feature Register 2
pub struct MVFR2;
impl Register for MVFR2 {
    const NAME: &'static str = "MVFR2";
    const LEN: usize = 64;
}

/// MVFR2 register fields
pub mod mvfr2 {
    use crate::core::model::proc::RegisterView;

    /// MVFR2\[7:4\]
    pub struct FPMisc;
    impl RegisterView for FPMisc {
        type Register = super::MVFR2;
        const NAME: &'static str = "FPMisc";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// MVFR2\[3:0\]
    pub struct SIMDMisc;
    impl RegisterView for SIMDMisc {
        type Register = super::MVFR2;
        const NAME: &'static str = "SIMDMisc";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Memory Attribute Indirection Register 1
pub struct MAIR1;
impl Register for MAIR1 {
    const NAME: &'static str = "MAIR1";
    const LEN: usize = 64;
}

/// Selected Error Record Miscellaneous Register 5
pub struct ERXMISC5;
impl Register for ERXMISC5 {
    const NAME: &'static str = "ERXMISC5";
    const LEN: usize = 64;
}

/// ERXMISC5 register fields
pub mod erxmisc5 {
    use crate::core::model::proc::RegisterView;

    /// ERXMISC5\[31:0\]
    pub struct ERRnMISC2hi;
    impl RegisterView for ERRnMISC2hi {
        type Register = super::ERXMISC5;
        const NAME: &'static str = "ERRnMISC2hi";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Interrupt Controller Virtual Machine Control Register
#[allow(non_camel_case_types)]
pub struct ICH_VMCR;
impl Register for ICH_VMCR {
    const NAME: &'static str = "ICH_VMCR";
    const LEN: usize = 64;
}

/// ICH_VMCR register fields
pub mod ich_vmcr {
    use crate::core::model::proc::RegisterView;

    /// ICH_VMCR\[31:24\]
    pub struct VPMR;
    impl RegisterView for VPMR {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VPMR";
        const OFFSET: usize = 24;
        const LEN: usize = 8;
    }

    /// ICH_VMCR\[23:21\]
    pub struct VBPR0;
    impl RegisterView for VBPR0 {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VBPR0";
        const OFFSET: usize = 21;
        const LEN: usize = 3;
    }

    /// ICH_VMCR\[20:18\]
    pub struct VBPR1;
    impl RegisterView for VBPR1 {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VBPR1";
        const OFFSET: usize = 18;
        const LEN: usize = 3;
    }

    /// ICH_VMCR\[9\]
    pub struct VEOIM;
    impl RegisterView for VEOIM {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VEOIM";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// ICH_VMCR\[4\]
    pub struct VCBPR;
    impl RegisterView for VCBPR {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VCBPR";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// ICH_VMCR\[3\]
    pub struct VFIQEn;
    impl RegisterView for VFIQEn {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VFIQEn";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// ICH_VMCR\[2\]
    pub struct VAckCtl;
    impl RegisterView for VAckCtl {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VAckCtl";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// ICH_VMCR\[1\]
    pub struct VENG1;
    impl RegisterView for VENG1 {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VENG1";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICH_VMCR\[0\]
    pub struct VENG0;
    impl RegisterView for VENG0 {
        type Register = super::ICH_VMCR;
        const NAME: &'static str = "VENG0";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Data Fault Address Register
pub struct DFAR;
impl Register for DFAR {
    const NAME: &'static str = "DFAR";
    const LEN: usize = 64;
}

/// Exception Link Register (Hyp mode)
#[allow(non_camel_case_types)]
pub struct ELR_hyp;
impl Register for ELR_hyp {
    const NAME: &'static str = "ELR_hyp";
    const LEN: usize = 64;
}

/// Debug CLAIM Tag Set register
pub struct DBGCLAIMSET;
impl Register for DBGCLAIMSET {
    const NAME: &'static str = "DBGCLAIMSET";
    const LEN: usize = 64;
}

/// DBGCLAIMSET register fields
pub mod dbgclaimset {
    use crate::core::model::proc::RegisterView;

    /// DBGCLAIMSET\[7:0\]
    pub struct CLAIM;
    impl RegisterView for CLAIM {
        type Register = super::DBGCLAIMSET;
        const NAME: &'static str = "CLAIM";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Debug OS Lock Exception Catch Control Register
pub struct DBGOSECCR;
impl Register for DBGOSECCR {
    const NAME: &'static str = "DBGOSECCR";
    const LEN: usize = 64;
}

/// DBGOSECCR register fields
pub mod dbgoseccr {
    use crate::core::model::proc::RegisterView;

    /// DBGOSECCR\[31:0\]
    pub struct EDECCR;
    impl RegisterView for EDECCR {
        type Register = super::DBGOSECCR;
        const NAME: &'static str = "EDECCR";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Current Cache Size ID Register 2
pub struct CCSIDR2;
impl Register for CCSIDR2 {
    const NAME: &'static str = "CCSIDR2";
    const LEN: usize = 64;
}

/// CCSIDR2 register fields
pub mod ccsidr2 {
    use crate::core::model::proc::RegisterView;

    /// CCSIDR2\[23:0\]
    pub struct NumSets;
    impl RegisterView for NumSets {
        type Register = super::CCSIDR2;
        const NAME: &'static str = "NumSets";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Virtualization Translation Control Register
pub struct VTCR;
impl Register for VTCR {
    const NAME: &'static str = "VTCR";
    const LEN: usize = 64;
}

/// VTCR register fields
pub mod vtcr {
    use crate::core::model::proc::RegisterView;

    /// VTCR\[28\]
    pub struct HWU62;
    impl RegisterView for HWU62 {
        type Register = super::VTCR;
        const NAME: &'static str = "HWU62";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// VTCR\[27\]
    pub struct HWU61;
    impl RegisterView for HWU61 {
        type Register = super::VTCR;
        const NAME: &'static str = "HWU61";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// VTCR\[26\]
    pub struct HWU60;
    impl RegisterView for HWU60 {
        type Register = super::VTCR;
        const NAME: &'static str = "HWU60";
        const OFFSET: usize = 26;
        const LEN: usize = 1;
    }

    /// VTCR\[25\]
    pub struct HWU59;
    impl RegisterView for HWU59 {
        type Register = super::VTCR;
        const NAME: &'static str = "HWU59";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// VTCR\[13:12\]
    pub struct SH0;
    impl RegisterView for SH0 {
        type Register = super::VTCR;
        const NAME: &'static str = "SH0";
        const OFFSET: usize = 12;
        const LEN: usize = 2;
    }

    /// VTCR\[11:10\]
    pub struct ORGN0;
    impl RegisterView for ORGN0 {
        type Register = super::VTCR;
        const NAME: &'static str = "ORGN0";
        const OFFSET: usize = 10;
        const LEN: usize = 2;
    }

    /// VTCR\[9:8\]
    pub struct IRGN0;
    impl RegisterView for IRGN0 {
        type Register = super::VTCR;
        const NAME: &'static str = "IRGN0";
        const OFFSET: usize = 8;
        const LEN: usize = 2;
    }

    /// VTCR\[7:6\]
    pub struct SL0;
    impl RegisterView for SL0 {
        type Register = super::VTCR;
        const NAME: &'static str = "SL0";
        const OFFSET: usize = 6;
        const LEN: usize = 2;
    }

    /// VTCR\[4\]
    pub struct S;
    impl RegisterView for S {
        type Register = super::VTCR;
        const NAME: &'static str = "S";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// VTCR\[3:0\]
    pub struct T0SZ;
    impl RegisterView for T0SZ {
        type Register = super::VTCR;
        const NAME: &'static str = "T0SZ";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// System Control Register
pub struct SCTLR;
impl Register for SCTLR {
    const NAME: &'static str = "SCTLR";
    const LEN: usize = 64;
}

/// SCTLR register fields
pub mod sctlr {
    use crate::core::model::proc::RegisterView;

    /// SCTLR\[31\]
    pub struct DSSBS;
    impl RegisterView for DSSBS {
        type Register = super::SCTLR;
        const NAME: &'static str = "DSSBS";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// SCTLR\[30\]
    pub struct TE;
    impl RegisterView for TE {
        type Register = super::SCTLR;
        const NAME: &'static str = "TE";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// SCTLR\[29\]
    pub struct AFE;
    impl RegisterView for AFE {
        type Register = super::SCTLR;
        const NAME: &'static str = "AFE";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// SCTLR\[28\]
    pub struct TRE;
    impl RegisterView for TRE {
        type Register = super::SCTLR;
        const NAME: &'static str = "TRE";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// SCTLR\[25\]
    pub struct EE;
    impl RegisterView for EE {
        type Register = super::SCTLR;
        const NAME: &'static str = "EE";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// SCTLR\[23\]
    pub struct SPAN;
    impl RegisterView for SPAN {
        type Register = super::SCTLR;
        const NAME: &'static str = "SPAN";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// SCTLR\[20\]
    pub struct UWXN;
    impl RegisterView for UWXN {
        type Register = super::SCTLR;
        const NAME: &'static str = "UWXN";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// SCTLR\[19\]
    pub struct WXN;
    impl RegisterView for WXN {
        type Register = super::SCTLR;
        const NAME: &'static str = "WXN";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// SCTLR\[18\]
    #[allow(non_camel_case_types)]
    pub struct nTWE;
    impl RegisterView for nTWE {
        type Register = super::SCTLR;
        const NAME: &'static str = "nTWE";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// SCTLR\[16\]
    #[allow(non_camel_case_types)]
    pub struct nTWI;
    impl RegisterView for nTWI {
        type Register = super::SCTLR;
        const NAME: &'static str = "nTWI";
        const OFFSET: usize = 16;
        const LEN: usize = 1;
    }

    /// SCTLR\[13\]
    pub struct V;
    impl RegisterView for V {
        type Register = super::SCTLR;
        const NAME: &'static str = "V";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// SCTLR\[12\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::SCTLR;
        const NAME: &'static str = "I";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// SCTLR\[10\]
    pub struct EnRCTX;
    impl RegisterView for EnRCTX {
        type Register = super::SCTLR;
        const NAME: &'static str = "EnRCTX";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }

    /// SCTLR\[8\]
    pub struct SED;
    impl RegisterView for SED {
        type Register = super::SCTLR;
        const NAME: &'static str = "SED";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// SCTLR\[7\]
    pub struct ITD;
    impl RegisterView for ITD {
        type Register = super::SCTLR;
        const NAME: &'static str = "ITD";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// SCTLR\[6\]
    pub struct UNK;
    impl RegisterView for UNK {
        type Register = super::SCTLR;
        const NAME: &'static str = "UNK";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// SCTLR\[5\]
    pub struct CP15BEN;
    impl RegisterView for CP15BEN {
        type Register = super::SCTLR;
        const NAME: &'static str = "CP15BEN";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// SCTLR\[4\]
    pub struct LSMAOE;
    impl RegisterView for LSMAOE {
        type Register = super::SCTLR;
        const NAME: &'static str = "LSMAOE";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// SCTLR\[3\]
    #[allow(non_camel_case_types)]
    pub struct nTLSMD;
    impl RegisterView for nTLSMD {
        type Register = super::SCTLR;
        const NAME: &'static str = "nTLSMD";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// SCTLR\[2\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::SCTLR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// SCTLR\[1\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::SCTLR;
        const NAME: &'static str = "A";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// SCTLR\[0\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::SCTLR;
        const NAME: &'static str = "M";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Selected Error Record Miscellaneous Register 4
pub struct ERXMISC4;
impl Register for ERXMISC4 {
    const NAME: &'static str = "ERXMISC4";
    const LEN: usize = 64;
}

/// ERXMISC4 register fields
pub mod erxmisc4 {
    use crate::core::model::proc::RegisterView;

    /// ERXMISC4\[31:0\]
    pub struct ERRnMISC2lo;
    impl RegisterView for ERRnMISC2lo {
        type Register = super::ERXMISC4;
        const NAME: &'static str = "ERRnMISC2lo";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Memory Attribute Indirection Register 0
pub struct MAIR0;
impl Register for MAIR0 {
    const NAME: &'static str = "MAIR0";
    const LEN: usize = 64;
}

/// Floating-Point Exception Control register
pub struct FPEXC;
impl Register for FPEXC {
    const NAME: &'static str = "FPEXC";
    const LEN: usize = 64;
}

/// FPEXC register fields
pub mod fpexc {
    use crate::core::model::proc::RegisterView;

    /// FPEXC\[31\]
    pub struct EX;
    impl RegisterView for EX {
        type Register = super::FPEXC;
        const NAME: &'static str = "EX";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// FPEXC\[30\]
    pub struct EN;
    impl RegisterView for EN {
        type Register = super::FPEXC;
        const NAME: &'static str = "EN";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// FPEXC\[29\]
    pub struct DEX;
    impl RegisterView for DEX {
        type Register = super::FPEXC;
        const NAME: &'static str = "DEX";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// FPEXC\[28\]
    pub struct FP2V;
    impl RegisterView for FP2V {
        type Register = super::FPEXC;
        const NAME: &'static str = "FP2V";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// FPEXC\[27\]
    pub struct VV;
    impl RegisterView for VV {
        type Register = super::FPEXC;
        const NAME: &'static str = "VV";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// FPEXC\[26\]
    pub struct TFV;
    impl RegisterView for TFV {
        type Register = super::FPEXC;
        const NAME: &'static str = "TFV";
        const OFFSET: usize = 26;
        const LEN: usize = 1;
    }

    /// FPEXC\[10:8\]
    pub struct VECITR;
    impl RegisterView for VECITR {
        type Register = super::FPEXC;
        const NAME: &'static str = "VECITR";
        const OFFSET: usize = 8;
        const LEN: usize = 3;
    }

    /// FPEXC\[7\]
    pub struct IDF;
    impl RegisterView for IDF {
        type Register = super::FPEXC;
        const NAME: &'static str = "IDF";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// FPEXC\[4\]
    pub struct IXF;
    impl RegisterView for IXF {
        type Register = super::FPEXC;
        const NAME: &'static str = "IXF";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// FPEXC\[3\]
    pub struct UFF;
    impl RegisterView for UFF {
        type Register = super::FPEXC;
        const NAME: &'static str = "UFF";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// FPEXC\[2\]
    pub struct OFF;
    impl RegisterView for OFF {
        type Register = super::FPEXC;
        const NAME: &'static str = "OFF";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// FPEXC\[1\]
    pub struct DZF;
    impl RegisterView for DZF {
        type Register = super::FPEXC;
        const NAME: &'static str = "DZF";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// FPEXC\[0\]
    pub struct IOF;
    impl RegisterView for IOF {
        type Register = super::FPEXC;
        const NAME: &'static str = "IOF";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Auxiliary Instruction Fault Status Register
pub struct AIFSR;
impl Register for AIFSR {
    const NAME: &'static str = "AIFSR";
    const LEN: usize = 64;
}

/// Media and VFP Feature Register 1
pub struct MVFR1;
impl Register for MVFR1 {
    const NAME: &'static str = "MVFR1";
    const LEN: usize = 64;
}

/// MVFR1 register fields
pub mod mvfr1 {
    use crate::core::model::proc::RegisterView;

    /// MVFR1\[31:28\]
    pub struct SIMDFMAC;
    impl RegisterView for SIMDFMAC {
        type Register = super::MVFR1;
        const NAME: &'static str = "SIMDFMAC";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// MVFR1\[27:24\]
    pub struct FPHP;
    impl RegisterView for FPHP {
        type Register = super::MVFR1;
        const NAME: &'static str = "FPHP";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// MVFR1\[23:20\]
    pub struct SIMDHP;
    impl RegisterView for SIMDHP {
        type Register = super::MVFR1;
        const NAME: &'static str = "SIMDHP";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// MVFR1\[19:16\]
    pub struct SIMDSP;
    impl RegisterView for SIMDSP {
        type Register = super::MVFR1;
        const NAME: &'static str = "SIMDSP";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// MVFR1\[15:12\]
    pub struct SIMDInt;
    impl RegisterView for SIMDInt {
        type Register = super::MVFR1;
        const NAME: &'static str = "SIMDInt";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// MVFR1\[11:8\]
    pub struct SIMDLS;
    impl RegisterView for SIMDLS {
        type Register = super::MVFR1;
        const NAME: &'static str = "SIMDLS";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// MVFR1\[7:4\]
    pub struct FPDNaN;
    impl RegisterView for FPDNaN {
        type Register = super::MVFR1;
        const NAME: &'static str = "FPDNaN";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// MVFR1\[3:0\]
    pub struct FPFtZ;
    impl RegisterView for FPFtZ {
        type Register = super::MVFR1;
        const NAME: &'static str = "FPFtZ";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Selected Error Record Miscellaneous Register 6
pub struct ERXMISC6;
impl Register for ERXMISC6 {
    const NAME: &'static str = "ERXMISC6";
    const LEN: usize = 64;
}

/// ERXMISC6 register fields
pub mod erxmisc6 {
    use crate::core::model::proc::RegisterView;

    /// ERXMISC6\[31:0\]
    pub struct ERRnMISC3lo;
    impl RegisterView for ERRnMISC3lo {
        type Register = super::ERXMISC6;
        const NAME: &'static str = "ERRnMISC3lo";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Debug OS Double Lock Register
pub struct DBGOSDLR;
impl Register for DBGOSDLR {
    const NAME: &'static str = "DBGOSDLR";
    const LEN: usize = 64;
}

/// DBGOSDLR register fields
pub mod dbgosdlr {
    use crate::core::model::proc::RegisterView;

    /// DBGOSDLR\[0\]
    pub struct DLK;
    impl RegisterView for DLK {
        type Register = super::DBGOSDLR;
        const NAME: &'static str = "DLK";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Counter-timer Virtual Count register
pub struct CNTVCT;
impl Register for CNTVCT {
    const NAME: &'static str = "CNTVCT";
    const LEN: usize = 64;
}

/// Interrupt Controller Deactivate Virtual Interrupt Register
#[allow(non_camel_case_types)]
pub struct ICV_DIR;
impl Register for ICV_DIR {
    const NAME: &'static str = "ICV_DIR";
    const LEN: usize = 64;
}

/// ICV_DIR register fields
pub mod icv_dir {
    use crate::core::model::proc::RegisterView;

    /// ICV_DIR\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICV_DIR;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Interrupt Controller Monitor Control Register
#[allow(non_camel_case_types)]
pub struct ICC_MCTLR;
impl Register for ICC_MCTLR {
    const NAME: &'static str = "ICC_MCTLR";
    const LEN: usize = 64;
}

/// ICC_MCTLR register fields
pub mod icc_mctlr {
    use crate::core::model::proc::RegisterView;

    /// ICC_MCTLR\[19\]
    pub struct ExtRange;
    impl RegisterView for ExtRange {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "ExtRange";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[18\]
    pub struct RSS;
    impl RegisterView for RSS {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "RSS";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[17\]
    #[allow(non_camel_case_types)]
    pub struct nDS;
    impl RegisterView for nDS {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "nDS";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[15\]
    pub struct A3V;
    impl RegisterView for A3V {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "A3V";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[14\]
    pub struct SEIS;
    impl RegisterView for SEIS {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "SEIS";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[13:11\]
    pub struct IDbits;
    impl RegisterView for IDbits {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "IDbits";
        const OFFSET: usize = 11;
        const LEN: usize = 3;
    }

    /// ICC_MCTLR\[10:8\]
    pub struct PRIbits;
    impl RegisterView for PRIbits {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "PRIbits";
        const OFFSET: usize = 8;
        const LEN: usize = 3;
    }

    /// ICC_MCTLR\[6\]
    pub struct PMHE;
    impl RegisterView for PMHE {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "PMHE";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[5\]
    pub struct RM;
    impl RegisterView for RM {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "RM";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[4\]
    #[allow(non_camel_case_types)]
    pub struct EOImode_EL1NS;
    impl RegisterView for EOImode_EL1NS {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "EOImode_EL1NS";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[3\]
    #[allow(non_camel_case_types)]
    pub struct EOImode_EL1S;
    impl RegisterView for EOImode_EL1S {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "EOImode_EL1S";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[2\]
    #[allow(non_camel_case_types)]
    pub struct EOImode_EL3;
    impl RegisterView for EOImode_EL3 {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "EOImode_EL3";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[1\]
    #[allow(non_camel_case_types)]
    pub struct CBPR_EL1NS;
    impl RegisterView for CBPR_EL1NS {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "CBPR_EL1NS";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICC_MCTLR\[0\]
    #[allow(non_camel_case_types)]
    pub struct CBPR_EL1S;
    impl RegisterView for CBPR_EL1S {
        type Register = super::ICC_MCTLR;
        const NAME: &'static str = "CBPR_EL1S";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Deactivate Interrupt Register
#[allow(non_camel_case_types)]
pub struct ICC_DIR;
impl Register for ICC_DIR {
    const NAME: &'static str = "ICC_DIR";
    const LEN: usize = 64;
}

/// ICC_DIR register fields
pub mod icc_dir {
    use crate::core::model::proc::RegisterView;

    /// ICC_DIR\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_DIR;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Debug CLAIM Tag Clear register
pub struct DBGCLAIMCLR;
impl Register for DBGCLAIMCLR {
    const NAME: &'static str = "DBGCLAIMCLR";
    const LEN: usize = 64;
}

/// DBGCLAIMCLR register fields
pub mod dbgclaimclr {
    use crate::core::model::proc::RegisterView;

    /// DBGCLAIMCLR\[7:0\]
    pub struct CLAIM;
    impl RegisterView for CLAIM {
        type Register = super::DBGCLAIMCLR;
        const NAME: &'static str = "CLAIM";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Trace Filter Control Register
pub struct TRFCR;
impl Register for TRFCR {
    const NAME: &'static str = "TRFCR";
    const LEN: usize = 64;
}

/// TRFCR register fields
pub mod trfcr {
    use crate::core::model::proc::RegisterView;

    /// TRFCR\[6:5\]
    pub struct TS;
    impl RegisterView for TS {
        type Register = super::TRFCR;
        const NAME: &'static str = "TS";
        const OFFSET: usize = 5;
        const LEN: usize = 2;
    }

    /// TRFCR\[1\]
    pub struct E1TRE;
    impl RegisterView for E1TRE {
        type Register = super::TRFCR;
        const NAME: &'static str = "E1TRE";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// TRFCR\[0\]
    pub struct E0TRE;
    impl RegisterView for E0TRE {
        type Register = super::TRFCR;
        const NAME: &'static str = "E0TRE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Selected Error Record Feature Register
pub struct ERXFR;
impl Register for ERXFR {
    const NAME: &'static str = "ERXFR";
    const LEN: usize = 64;
}

/// ERXFR register fields
pub mod erxfr {
    use crate::core::model::proc::RegisterView;

    /// ERXFR\[31:0\]
    pub struct ERRnFRlo;
    impl RegisterView for ERRnFRlo {
        type Register = super::ERXFR;
        const NAME: &'static str = "ERRnFRlo";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Counter-timer Hyp Physical Timer Control register
#[allow(non_camel_case_types)]
pub struct CNTHP_CTL;
impl Register for CNTHP_CTL {
    const NAME: &'static str = "CNTHP_CTL";
    const LEN: usize = 64;
}

/// CNTHP_CTL register fields
pub mod cnthp_ctl {
    use crate::core::model::proc::RegisterView;

    /// CNTHP_CTL\[2\]
    pub struct ISTATUS;
    impl RegisterView for ISTATUS {
        type Register = super::CNTHP_CTL;
        const NAME: &'static str = "ISTATUS";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// CNTHP_CTL\[1\]
    pub struct IMASK;
    impl RegisterView for IMASK {
        type Register = super::CNTHP_CTL;
        const NAME: &'static str = "IMASK";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// CNTHP_CTL\[0\]
    pub struct ENABLE;
    impl RegisterView for ENABLE {
        type Register = super::CNTHP_CTL;
        const NAME: &'static str = "ENABLE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Floating-Point System ID register
pub struct FPSID;
impl Register for FPSID {
    const NAME: &'static str = "FPSID";
    const LEN: usize = 64;
}

/// FPSID register fields
pub mod fpsid {
    use crate::core::model::proc::RegisterView;

    /// FPSID\[31:24\]
    pub struct Implementer;
    impl RegisterView for Implementer {
        type Register = super::FPSID;
        const NAME: &'static str = "Implementer";
        const OFFSET: usize = 24;
        const LEN: usize = 8;
    }

    /// FPSID\[23\]
    pub struct SW;
    impl RegisterView for SW {
        type Register = super::FPSID;
        const NAME: &'static str = "SW";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// FPSID\[22:16\]
    pub struct Subarchitecture;
    impl RegisterView for Subarchitecture {
        type Register = super::FPSID;
        const NAME: &'static str = "Subarchitecture";
        const OFFSET: usize = 16;
        const LEN: usize = 7;
    }

    /// FPSID\[15:8\]
    pub struct PartNum;
    impl RegisterView for PartNum {
        type Register = super::FPSID;
        const NAME: &'static str = "PartNum";
        const OFFSET: usize = 8;
        const LEN: usize = 8;
    }

    /// FPSID\[7:4\]
    pub struct Variant;
    impl RegisterView for Variant {
        type Register = super::FPSID;
        const NAME: &'static str = "Variant";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// FPSID\[3:0\]
    pub struct Revision;
    impl RegisterView for Revision {
        type Register = super::FPSID;
        const NAME: &'static str = "Revision";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Hyp Syndrome Register
pub struct HSR;
impl Register for HSR {
    const NAME: &'static str = "HSR";
    const LEN: usize = 64;
}

/// HSR register fields
pub mod hsr {
    use crate::core::model::proc::RegisterView;

    /// HSR\[31:26\]
    pub struct EC;
    impl RegisterView for EC {
        type Register = super::HSR;
        const NAME: &'static str = "EC";
        const OFFSET: usize = 26;
        const LEN: usize = 6;
    }

    /// HSR\[25\]
    pub struct IL;
    impl RegisterView for IL {
        type Register = super::HSR;
        const NAME: &'static str = "IL";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// HSR\[24:0\]
    pub struct ISS;
    impl RegisterView for ISS {
        type Register = super::HSR;
        const NAME: &'static str = "ISS";
        const OFFSET: usize = 0;
        const LEN: usize = 25;
    }
}

/// Debug Data Transfer Register, Receive
pub struct DBGDTRRXint;
impl Register for DBGDTRRXint {
    const NAME: &'static str = "DBGDTRRXint";
    const LEN: usize = 64;
}

/// DBGDTRRXint register fields
pub mod dbgdtrrxint {
    use crate::core::model::proc::RegisterView;

    /// DBGDTRRXint\[31:0\]
    pub struct DTRRX;
    impl RegisterView for DTRRX {
        type Register = super::DBGDTRRXint;
        const NAME: &'static str = "DTRRX";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Debug OS Lock Data Transfer Register, Receive, External View
pub struct DBGDTRRXext;
impl Register for DBGDTRRXext {
    const NAME: &'static str = "DBGDTRRXext";
    const LEN: usize = 64;
}

/// DBGDTRRXext register fields
pub mod dbgdtrrxext {
    use crate::core::model::proc::RegisterView;

    /// DBGDTRRXext\[31:0\]
    pub struct DTRRX;
    impl RegisterView for DTRRX {
        type Register = super::DBGDTRRXext;
        const NAME: &'static str = "DTRRX";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Selected Error Record Miscellaneous Register 7
pub struct ERXMISC7;
impl Register for ERXMISC7 {
    const NAME: &'static str = "ERXMISC7";
    const LEN: usize = 64;
}

/// ERXMISC7 register fields
pub mod erxmisc7 {
    use crate::core::model::proc::RegisterView;

    /// ERXMISC7\[31:0\]
    pub struct ERRnMISC3hi;
    impl RegisterView for ERRnMISC3hi {
        type Register = super::ERXMISC7;
        const NAME: &'static str = "ERRnMISC3hi";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Media and VFP Feature Register 0
pub struct MVFR0;
impl Register for MVFR0 {
    const NAME: &'static str = "MVFR0";
    const LEN: usize = 64;
}

/// MVFR0 register fields
pub mod mvfr0 {
    use crate::core::model::proc::RegisterView;

    /// MVFR0\[31:28\]
    pub struct FPRound;
    impl RegisterView for FPRound {
        type Register = super::MVFR0;
        const NAME: &'static str = "FPRound";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// MVFR0\[27:24\]
    pub struct FPShVec;
    impl RegisterView for FPShVec {
        type Register = super::MVFR0;
        const NAME: &'static str = "FPShVec";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// MVFR0\[23:20\]
    pub struct FPSqrt;
    impl RegisterView for FPSqrt {
        type Register = super::MVFR0;
        const NAME: &'static str = "FPSqrt";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// MVFR0\[19:16\]
    pub struct FPDivide;
    impl RegisterView for FPDivide {
        type Register = super::MVFR0;
        const NAME: &'static str = "FPDivide";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// MVFR0\[15:12\]
    pub struct FPTrap;
    impl RegisterView for FPTrap {
        type Register = super::MVFR0;
        const NAME: &'static str = "FPTrap";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// MVFR0\[11:8\]
    pub struct FPDP;
    impl RegisterView for FPDP {
        type Register = super::MVFR0;
        const NAME: &'static str = "FPDP";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// MVFR0\[7:4\]
    pub struct FPSP;
    impl RegisterView for FPSP {
        type Register = super::MVFR0;
        const NAME: &'static str = "FPSP";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// MVFR0\[3:0\]
    pub struct SIMDReg;
    impl RegisterView for SIMDReg {
        type Register = super::MVFR0;
        const NAME: &'static str = "SIMDReg";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Counter-timer Hyp Physical CompareValue register
#[allow(non_camel_case_types)]
pub struct CNTHP_CVAL;
impl Register for CNTHP_CVAL {
    const NAME: &'static str = "CNTHP_CVAL";
    const LEN: usize = 64;
}

/// CNTHP_CVAL register fields
pub mod cnthp_cval {
    use crate::core::model::proc::RegisterView;

    /// CNTHP_CVAL\[63:0\]
    pub struct CompareValue;
    impl RegisterView for CompareValue {
        type Register = super::CNTHP_CVAL;
        const NAME: &'static str = "CompareValue";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Multiprocessor Affinity Register
pub struct MPIDR;
impl Register for MPIDR {
    const NAME: &'static str = "MPIDR";
    const LEN: usize = 64;
}

/// MPIDR register fields
pub mod mpidr {
    use crate::core::model::proc::RegisterView;

    /// MPIDR\[31\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::MPIDR;
        const NAME: &'static str = "M";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// MPIDR\[30\]
    pub struct U;
    impl RegisterView for U {
        type Register = super::MPIDR;
        const NAME: &'static str = "U";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// MPIDR\[24\]
    pub struct MT;
    impl RegisterView for MT {
        type Register = super::MPIDR;
        const NAME: &'static str = "MT";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// MPIDR\[23:16\]
    pub struct Aff2;
    impl RegisterView for Aff2 {
        type Register = super::MPIDR;
        const NAME: &'static str = "Aff2";
        const OFFSET: usize = 16;
        const LEN: usize = 8;
    }

    /// MPIDR\[15:8\]
    pub struct Aff1;
    impl RegisterView for Aff1 {
        type Register = super::MPIDR;
        const NAME: &'static str = "Aff1";
        const OFFSET: usize = 8;
        const LEN: usize = 8;
    }

    /// MPIDR\[7:0\]
    pub struct Aff0;
    impl RegisterView for Aff0 {
        type Register = super::MPIDR;
        const NAME: &'static str = "Aff0";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Jazelle ID Register
pub struct JIDR;
impl Register for JIDR {
    const NAME: &'static str = "JIDR";
    const LEN: usize = 64;
}

/// Selected Error Record Miscellaneous Register 3
pub struct ERXMISC3;
impl Register for ERXMISC3 {
    const NAME: &'static str = "ERXMISC3";
    const LEN: usize = 64;
}

/// ERXMISC3 register fields
pub mod erxmisc3 {
    use crate::core::model::proc::RegisterView;

    /// ERXMISC3\[31:0\]
    pub struct ERRnMISC1hi;
    impl RegisterView for ERRnMISC1hi {
        type Register = super::ERXMISC3;
        const NAME: &'static str = "ERRnMISC1hi";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Performance Monitors Selected Event Count Register
pub struct PMXEVCNTR;
impl Register for PMXEVCNTR {
    const NAME: &'static str = "PMXEVCNTR";
    const LEN: usize = 64;
}

/// Secure Debug Enable Register
pub struct SDER;
impl Register for SDER {
    const NAME: &'static str = "SDER";
    const LEN: usize = 64;
}

/// SDER register fields
pub mod sder {
    use crate::core::model::proc::RegisterView;

    /// SDER\[1\]
    pub struct SUNIDEN;
    impl RegisterView for SUNIDEN {
        type Register = super::SDER;
        const NAME: &'static str = "SUNIDEN";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// SDER\[0\]
    pub struct SUIDEN;
    impl RegisterView for SUIDEN {
        type Register = super::SDER;
        const NAME: &'static str = "SUIDEN";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Performance Monitors Overflow Flag Status Register
pub struct PMOVSR;
impl Register for PMOVSR {
    const NAME: &'static str = "PMOVSR";
    const LEN: usize = 64;
}

/// PMOVSR register fields
pub mod pmovsr {
    use crate::core::model::proc::RegisterView;

    /// PMOVSR\[31\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::PMOVSR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }
}

/// Hyp Auxiliary Memory Attribute Indirection Register 1
pub struct HAMAIR1;
impl Register for HAMAIR1 {
    const NAME: &'static str = "HAMAIR1";
    const LEN: usize = 64;
}

/// Hyp Auxiliary Memory Attribute Indirection Register 0
pub struct HAMAIR0;
impl Register for HAMAIR0 {
    const NAME: &'static str = "HAMAIR0";
    const LEN: usize = 64;
}

/// Performance Monitors Cycle Count Filter Register
pub struct PMCCFILTR;
impl Register for PMCCFILTR {
    const NAME: &'static str = "PMCCFILTR";
    const LEN: usize = 64;
}

/// PMCCFILTR register fields
pub mod pmccfiltr {
    use crate::core::model::proc::RegisterView;

    /// PMCCFILTR\[31\]
    pub struct P;
    impl RegisterView for P {
        type Register = super::PMCCFILTR;
        const NAME: &'static str = "P";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// PMCCFILTR\[30\]
    pub struct U;
    impl RegisterView for U {
        type Register = super::PMCCFILTR;
        const NAME: &'static str = "U";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// PMCCFILTR\[29\]
    pub struct NSK;
    impl RegisterView for NSK {
        type Register = super::PMCCFILTR;
        const NAME: &'static str = "NSK";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// PMCCFILTR\[28\]
    pub struct NSU;
    impl RegisterView for NSU {
        type Register = super::PMCCFILTR;
        const NAME: &'static str = "NSU";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// PMCCFILTR\[27\]
    pub struct NSH;
    impl RegisterView for NSH {
        type Register = super::PMCCFILTR;
        const NAME: &'static str = "NSH";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// PMCCFILTR\[21\]
    pub struct RLU;
    impl RegisterView for RLU {
        type Register = super::PMCCFILTR;
        const NAME: &'static str = "RLU";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Alias Software Generated Interrupt Group 1 Register
#[allow(non_camel_case_types)]
pub struct ICC_ASGI1R;
impl Register for ICC_ASGI1R {
    const NAME: &'static str = "ICC_ASGI1R";
    const LEN: usize = 64;
}

/// ICC_ASGI1R register fields
pub mod icc_asgi1r {
    use crate::core::model::proc::RegisterView;

    /// ICC_ASGI1R\[55:48\]
    pub struct Aff3;
    impl RegisterView for Aff3 {
        type Register = super::ICC_ASGI1R;
        const NAME: &'static str = "Aff3";
        const OFFSET: usize = 48;
        const LEN: usize = 8;
    }

    /// ICC_ASGI1R\[47:44\]
    pub struct RS;
    impl RegisterView for RS {
        type Register = super::ICC_ASGI1R;
        const NAME: &'static str = "RS";
        const OFFSET: usize = 44;
        const LEN: usize = 4;
    }

    /// ICC_ASGI1R\[40\]
    pub struct IRM;
    impl RegisterView for IRM {
        type Register = super::ICC_ASGI1R;
        const NAME: &'static str = "IRM";
        const OFFSET: usize = 40;
        const LEN: usize = 1;
    }

    /// ICC_ASGI1R\[39:32\]
    pub struct Aff2;
    impl RegisterView for Aff2 {
        type Register = super::ICC_ASGI1R;
        const NAME: &'static str = "Aff2";
        const OFFSET: usize = 32;
        const LEN: usize = 8;
    }

    /// ICC_ASGI1R\[27:24\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_ASGI1R;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ICC_ASGI1R\[23:16\]
    pub struct Aff1;
    impl RegisterView for Aff1 {
        type Register = super::ICC_ASGI1R;
        const NAME: &'static str = "Aff1";
        const OFFSET: usize = 16;
        const LEN: usize = 8;
    }

    /// ICC_ASGI1R\[15:0\]
    pub struct TargetList;
    impl RegisterView for TargetList {
        type Register = super::ICC_ASGI1R;
        const NAME: &'static str = "TargetList";
        const OFFSET: usize = 0;
        const LEN: usize = 16;
    }
}

/// Debug Device ID register 1
pub struct DBGDEVID1;
impl Register for DBGDEVID1 {
    const NAME: &'static str = "DBGDEVID1";
    const LEN: usize = 64;
}

/// DBGDEVID1 register fields
pub mod dbgdevid1 {
    use crate::core::model::proc::RegisterView;

    /// DBGDEVID1\[3:0\]
    pub struct PCSROffset;
    impl RegisterView for PCSROffset {
        type Register = super::DBGDEVID1;
        const NAME: &'static str = "PCSROffset";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Performance Monitors Selected Event Type Register
pub struct PMXEVTYPER;
impl Register for PMXEVTYPER {
    const NAME: &'static str = "PMXEVTYPER";
    const LEN: usize = 64;
}

/// PMXEVTYPER register fields
pub mod pmxevtyper {
    use crate::core::model::proc::RegisterView;

    /// PMXEVTYPER\[31:0\]
    pub struct ETR;
    impl RegisterView for ETR {
        type Register = super::PMXEVTYPER;
        const NAME: &'static str = "ETR";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Reset Vector Base Address Register
pub struct RVBAR;
impl Register for RVBAR {
    const NAME: &'static str = "RVBAR";
    const LEN: usize = 64;
}

/// RVBAR register fields
pub mod rvbar {
    use crate::core::model::proc::RegisterView;

    /// RVBAR\[31:1\]
    pub struct ResetAddress;
    impl RegisterView for ResetAddress {
        type Register = super::RVBAR;
        const NAME: &'static str = "ResetAddress";
        const OFFSET: usize = 1;
        const LEN: usize = 31;
    }
}

/// Selected Error Record Miscellaneous Register 2
pub struct ERXMISC2;
impl Register for ERXMISC2 {
    const NAME: &'static str = "ERXMISC2";
    const LEN: usize = 64;
}

/// ERXMISC2 register fields
pub mod erxmisc2 {
    use crate::core::model::proc::RegisterView;

    /// ERXMISC2\[31:0\]
    pub struct ERRnMISC1lo;
    impl RegisterView for ERRnMISC1lo {
        type Register = super::ERXMISC2;
        const NAME: &'static str = "ERRnMISC1lo";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Physical Address Register
pub struct PAR;
impl Register for PAR {
    const NAME: &'static str = "PAR";
    const LEN: usize = 64;
}

/// PAR register fields
pub mod par {
    use crate::core::model::proc::RegisterView;

    /// PAR\[39:12\]
    pub struct PA;
    impl RegisterView for PA {
        type Register = super::PAR;
        const NAME: &'static str = "PA";
        const OFFSET: usize = 12;
        const LEN: usize = 28;
    }

    /// PAR\[11\]
    pub struct LPAE;
    impl RegisterView for LPAE {
        type Register = super::PAR;
        const NAME: &'static str = "LPAE";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// PAR\[10\]
    pub struct NOS;
    impl RegisterView for NOS {
        type Register = super::PAR;
        const NAME: &'static str = "NOS";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }

    /// PAR\[9\]
    pub struct NS;
    impl RegisterView for NS {
        type Register = super::PAR;
        const NAME: &'static str = "NS";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// PAR\[8:7\]
    pub struct SH;
    impl RegisterView for SH {
        type Register = super::PAR;
        const NAME: &'static str = "SH";
        const OFFSET: usize = 7;
        const LEN: usize = 2;
    }

    /// PAR\[1\]
    pub struct SS;
    impl RegisterView for SS {
        type Register = super::PAR;
        const NAME: &'static str = "SS";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// PAR\[0\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::PAR;
        const NAME: &'static str = "F";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }

    /// PAR\[63:56\]
    pub struct ATTR;
    impl RegisterView for ATTR {
        type Register = super::PAR;
        const NAME: &'static str = "ATTR";
        const OFFSET: usize = 56;
        const LEN: usize = 8;
    }

    /// PAR\[9\]
    pub struct FSTAGE;
    impl RegisterView for FSTAGE {
        type Register = super::PAR;
        const NAME: &'static str = "FSTAGE";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// PAR\[8\]
    pub struct S2WLK;
    impl RegisterView for S2WLK {
        type Register = super::PAR;
        const NAME: &'static str = "S2WLK";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// PAR\[6:1\]
    pub struct FST;
    impl RegisterView for FST {
        type Register = super::PAR;
        const NAME: &'static str = "FST";
        const OFFSET: usize = 1;
        const LEN: usize = 6;
    }

    /// PAR\[6:4\]
    pub struct Inner;
    impl RegisterView for Inner {
        type Register = super::PAR;
        const NAME: &'static str = "Inner";
        const OFFSET: usize = 4;
        const LEN: usize = 3;
    }

    /// PAR\[3:2\]
    pub struct Outer;
    impl RegisterView for Outer {
        type Register = super::PAR;
        const NAME: &'static str = "Outer";
        const OFFSET: usize = 2;
        const LEN: usize = 2;
    }

    /// PAR\[6\]
    #[allow(non_camel_case_types)]
    pub struct FS_6;
    impl RegisterView for FS_6 {
        type Register = super::PAR;
        const NAME: &'static str = "FS_6";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }
    /// PAR\[5:1\]
    #[allow(non_camel_case_types)]
    pub struct FS_6_5_1;
    impl RegisterView for FS_6_5_1 {
        type Register = super::PAR;
        const NAME: &'static str = "FS_6_5_1";
        const OFFSET: usize = 1;
        const LEN: usize = 5;
    }
}

/// Selected Error Record Miscellaneous Register 0
pub struct ERXMISC0;
impl Register for ERXMISC0 {
    const NAME: &'static str = "ERXMISC0";
    const LEN: usize = 64;
}

/// ERXMISC0 register fields
pub mod erxmisc0 {
    use crate::core::model::proc::RegisterView;

    /// ERXMISC0\[31:0\]
    pub struct ERRnMISC0lo;
    impl RegisterView for ERRnMISC0lo {
        type Register = super::ERXMISC0;
        const NAME: &'static str = "ERRnMISC0lo";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Selected Error Record Address Register 2
pub struct ERXADDR2;
impl Register for ERXADDR2 {
    const NAME: &'static str = "ERXADDR2";
    const LEN: usize = 64;
}

/// ERXADDR2 register fields
pub mod erxaddr2 {
    use crate::core::model::proc::RegisterView;

    /// ERXADDR2\[31:0\]
    pub struct ERRnADDRhi;
    impl RegisterView for ERRnADDRhi {
        type Register = super::ERXADDR2;
        const NAME: &'static str = "ERRnADDRhi";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Interrupt Controller Maintenance Interrupt State Register
#[allow(non_camel_case_types)]
pub struct ICH_MISR;
impl Register for ICH_MISR {
    const NAME: &'static str = "ICH_MISR";
    const LEN: usize = 64;
}

/// ICH_MISR register fields
pub mod ich_misr {
    use crate::core::model::proc::RegisterView;

    /// ICH_MISR\[7\]
    pub struct VGrp1D;
    impl RegisterView for VGrp1D {
        type Register = super::ICH_MISR;
        const NAME: &'static str = "VGrp1D";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// ICH_MISR\[6\]
    pub struct VGrp1E;
    impl RegisterView for VGrp1E {
        type Register = super::ICH_MISR;
        const NAME: &'static str = "VGrp1E";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// ICH_MISR\[5\]
    pub struct VGrp0D;
    impl RegisterView for VGrp0D {
        type Register = super::ICH_MISR;
        const NAME: &'static str = "VGrp0D";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// ICH_MISR\[4\]
    pub struct VGrp0E;
    impl RegisterView for VGrp0E {
        type Register = super::ICH_MISR;
        const NAME: &'static str = "VGrp0E";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// ICH_MISR\[3\]
    pub struct NP;
    impl RegisterView for NP {
        type Register = super::ICH_MISR;
        const NAME: &'static str = "NP";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// ICH_MISR\[2\]
    pub struct LRENP;
    impl RegisterView for LRENP {
        type Register = super::ICH_MISR;
        const NAME: &'static str = "LRENP";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// ICH_MISR\[1\]
    pub struct U;
    impl RegisterView for U {
        type Register = super::ICH_MISR;
        const NAME: &'static str = "U";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICH_MISR\[0\]
    pub struct EOI;
    impl RegisterView for EOI {
        type Register = super::ICH_MISR;
        const NAME: &'static str = "EOI";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Context ID Register
pub struct CONTEXTIDR;
impl Register for CONTEXTIDR {
    const NAME: &'static str = "CONTEXTIDR";
    const LEN: usize = 64;
}

/// CONTEXTIDR register fields
pub mod contextidr {
    use crate::core::model::proc::RegisterView;

    /// CONTEXTIDR\[31:0\]
    pub struct PROCID;
    impl RegisterView for PROCID {
        type Register = super::CONTEXTIDR;
        const NAME: &'static str = "PROCID";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }

    /// CONTEXTIDR\[7:0\]
    pub struct ASID;
    impl RegisterView for ASID {
        type Register = super::CONTEXTIDR;
        const NAME: &'static str = "ASID";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Counter-timer Physical Count register
pub struct CNTPCT;
impl Register for CNTPCT {
    const NAME: &'static str = "CNTPCT";
    const LEN: usize = 64;
}

/// Counter-timer Virtual Timer TimerValue register
#[allow(non_camel_case_types)]
pub struct CNTV_TVAL;
impl Register for CNTV_TVAL {
    const NAME: &'static str = "CNTV_TVAL";
    const LEN: usize = 64;
}

/// CNTV_TVAL register fields
pub mod cntv_tval {
    use crate::core::model::proc::RegisterView;

    /// CNTV_TVAL\[31:0\]
    pub struct TimerValue;
    impl RegisterView for TimerValue {
        type Register = super::CNTV_TVAL;
        const NAME: &'static str = "TimerValue";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Debug Breakpoint Control Registers, n = 15 - 0
pub struct DBGBCRn;
impl RegisterArray for DBGBCRn {
    const NAME: &'static str = "DBGBCRn";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// DBGBCRn register fields
pub mod dbgbcrn {
    use crate::core::model::proc::RegisterArrayView;

    /// DBGBCRn\[23:20\]
    pub struct BT;
    impl RegisterArrayView for BT {
        type RegisterArray = super::DBGBCRn;
        const NAME: &'static str = "BT";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// DBGBCRn\[19:16\]
    pub struct LBN;
    impl RegisterArrayView for LBN {
        type RegisterArray = super::DBGBCRn;
        const NAME: &'static str = "LBN";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// DBGBCRn\[15:14\]
    pub struct SSC;
    impl RegisterArrayView for SSC {
        type RegisterArray = super::DBGBCRn;
        const NAME: &'static str = "SSC";
        const OFFSET: usize = 14;
        const LEN: usize = 2;
    }

    /// DBGBCRn\[13\]
    pub struct HMC;
    impl RegisterArrayView for HMC {
        type RegisterArray = super::DBGBCRn;
        const NAME: &'static str = "HMC";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// DBGBCRn\[8:5\]
    pub struct BAS;
    impl RegisterArrayView for BAS {
        type RegisterArray = super::DBGBCRn;
        const NAME: &'static str = "BAS";
        const OFFSET: usize = 5;
        const LEN: usize = 4;
    }

    /// DBGBCRn\[2:1\]
    pub struct PMC;
    impl RegisterArrayView for PMC {
        type RegisterArray = super::DBGBCRn;
        const NAME: &'static str = "PMC";
        const OFFSET: usize = 1;
        const LEN: usize = 2;
    }

    /// DBGBCRn\[0\]
    pub struct E;
    impl RegisterArrayView for E {
        type RegisterArray = super::DBGBCRn;
        const NAME: &'static str = "E";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Counter-timer Self-Synchronized Virtual Count register
pub struct CNTVCTSS;
impl Register for CNTVCTSS {
    const NAME: &'static str = "CNTVCTSS";
    const LEN: usize = 64;
}

/// Saved Program Status Register (Monitor mode)
#[allow(non_camel_case_types)]
pub struct SPSR_mon;
impl Register for SPSR_mon {
    const NAME: &'static str = "SPSR_mon";
    const LEN: usize = 64;
}

/// SPSR_mon register fields
pub mod spsr_mon {
    use crate::core::model::proc::RegisterView;

    /// SPSR_mon\[31\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "N";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[30\]
    pub struct Z;
    impl RegisterView for Z {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "Z";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[29\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "C";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[28\]
    pub struct V;
    impl RegisterView for V {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "V";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[27\]
    pub struct Q;
    impl RegisterView for Q {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "Q";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[15:10\]
    pub struct IT;
    impl RegisterView for IT {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "IT";
        const OFFSET: usize = 10;
        const LEN: usize = 6;
    }

    /// SPSR_mon\[24\]
    pub struct J;
    impl RegisterView for J {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "J";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[23\]
    pub struct SSBS;
    impl RegisterView for SSBS {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "SSBS";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[22\]
    pub struct PAN;
    impl RegisterView for PAN {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "PAN";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[21\]
    pub struct DIT;
    impl RegisterView for DIT {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "DIT";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[20\]
    pub struct IL;
    impl RegisterView for IL {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "IL";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[19:16\]
    pub struct GE;
    impl RegisterView for GE {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "GE";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// SPSR_mon\[9\]
    pub struct E;
    impl RegisterView for E {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "E";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[8\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "A";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[7\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "I";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[6\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "F";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[5\]
    pub struct T;
    impl RegisterView for T {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "T";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// SPSR_mon\[4:0\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::SPSR_mon;
        const NAME: &'static str = "M";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// PL0 Read/Write Software Thread ID Register
pub struct TPIDRURW;
impl Register for TPIDRURW {
    const NAME: &'static str = "TPIDRURW";
    const LEN: usize = 64;
}

/// TPIDRURW register fields
pub mod tpidrurw {
    use crate::core::model::proc::RegisterView;

    /// TPIDRURW\[31:0\]
    pub struct TID;
    impl RegisterView for TID {
        type Register = super::TPIDRURW;
        const NAME: &'static str = "TID";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Debug ROM Address Register
pub struct DBGDRAR;
impl Register for DBGDRAR {
    const NAME: &'static str = "DBGDRAR";
    const LEN: usize = 64;
}

/// DBGDRAR register fields
pub mod dbgdrar {
    use crate::core::model::proc::RegisterView;

    /// DBGDRAR\[1:0\]
    pub struct Valid;
    impl RegisterView for Valid {
        type Register = super::DBGDRAR;
        const NAME: &'static str = "Valid";
        const OFFSET: usize = 0;
        const LEN: usize = 2;
    }

    /// DBGDRAR\[47:12\]
    pub struct ROMADDR;
    impl RegisterView for ROMADDR {
        type Register = super::DBGDRAR;
        const NAME: &'static str = "ROMADDR";
        const OFFSET: usize = 12;
        const LEN: usize = 36;
    }
}

/// Debug Breakpoint Extended Value Registers, n = 15 - 0
pub struct DBGBXVRn;
impl RegisterArray for DBGBXVRn {
    const NAME: &'static str = "DBGBXVRn";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// DBGBXVRn register fields
pub mod dbgbxvrn {
    use crate::core::model::proc::RegisterArrayView;

    /// DBGBXVRn\[31:0\]
    pub struct ContextID2;
    impl RegisterArrayView for ContextID2 {
        type RegisterArray = super::DBGBXVRn;
        const NAME: &'static str = "ContextID2";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }

    /// DBGBXVRn\[15:8\]
    #[allow(non_camel_case_types)]
    pub struct VMID_15_8;
    impl RegisterArrayView for VMID_15_8 {
        type RegisterArray = super::DBGBXVRn;
        const NAME: &'static str = "VMID_15_8";
        const OFFSET: usize = 8;
        const LEN: usize = 8;
    }
    /// DBGBXVRn\[7:0\]
    #[allow(non_camel_case_types)]
    pub struct VMID_15_8_7_0;
    impl RegisterArrayView for VMID_15_8_7_0 {
        type RegisterArray = super::DBGBXVRn;
        const NAME: &'static str = "VMID_15_8_7_0";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Selected Error Record Address Register
pub struct ERXADDR;
impl Register for ERXADDR {
    const NAME: &'static str = "ERXADDR";
    const LEN: usize = 64;
}

/// ERXADDR register fields
pub mod erxaddr {
    use crate::core::model::proc::RegisterView;

    /// ERXADDR\[31:0\]
    pub struct ERRnADDRlo;
    impl RegisterView for ERRnADDRlo {
        type Register = super::ERXADDR;
        const NAME: &'static str = "ERRnADDRlo";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Interrupt Controller Virtual Interrupt Priority Mask Register
#[allow(non_camel_case_types)]
pub struct ICV_PMR;
impl Register for ICV_PMR {
    const NAME: &'static str = "ICV_PMR";
    const LEN: usize = 64;
}

/// ICV_PMR register fields
pub mod icv_pmr {
    use crate::core::model::proc::RegisterView;

    /// ICV_PMR\[7:0\]
    pub struct Priority;
    impl RegisterView for Priority {
        type Register = super::ICV_PMR;
        const NAME: &'static str = "Priority";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Counter-timer Kernel Control register
pub struct CNTKCTL;
impl Register for CNTKCTL {
    const NAME: &'static str = "CNTKCTL";
    const LEN: usize = 64;
}

/// CNTKCTL register fields
pub mod cntkctl {
    use crate::core::model::proc::RegisterView;

    /// CNTKCTL\[17\]
    pub struct EVNTIS;
    impl RegisterView for EVNTIS {
        type Register = super::CNTKCTL;
        const NAME: &'static str = "EVNTIS";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// CNTKCTL\[9\]
    pub struct PL0PTEN;
    impl RegisterView for PL0PTEN {
        type Register = super::CNTKCTL;
        const NAME: &'static str = "PL0PTEN";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// CNTKCTL\[8\]
    pub struct PL0VTEN;
    impl RegisterView for PL0VTEN {
        type Register = super::CNTKCTL;
        const NAME: &'static str = "PL0VTEN";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// CNTKCTL\[7:4\]
    pub struct EVNTI;
    impl RegisterView for EVNTI {
        type Register = super::CNTKCTL;
        const NAME: &'static str = "EVNTI";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// CNTKCTL\[3\]
    pub struct EVNTDIR;
    impl RegisterView for EVNTDIR {
        type Register = super::CNTKCTL;
        const NAME: &'static str = "EVNTDIR";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// CNTKCTL\[2\]
    pub struct EVNTEN;
    impl RegisterView for EVNTEN {
        type Register = super::CNTKCTL;
        const NAME: &'static str = "EVNTEN";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// CNTKCTL\[1\]
    pub struct PL0VCTEN;
    impl RegisterView for PL0VCTEN {
        type Register = super::CNTKCTL;
        const NAME: &'static str = "PL0VCTEN";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// CNTKCTL\[0\]
    pub struct PL0PCTEN;
    impl RegisterView for PL0PCTEN {
        type Register = super::CNTKCTL;
        const NAME: &'static str = "PL0PCTEN";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller System Register Enable register
#[allow(non_camel_case_types)]
pub struct ICC_SRE;
impl Register for ICC_SRE {
    const NAME: &'static str = "ICC_SRE";
    const LEN: usize = 64;
}

/// ICC_SRE register fields
pub mod icc_sre {
    use crate::core::model::proc::RegisterView;

    /// ICC_SRE\[2\]
    pub struct DIB;
    impl RegisterView for DIB {
        type Register = super::ICC_SRE;
        const NAME: &'static str = "DIB";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// ICC_SRE\[1\]
    pub struct DFB;
    impl RegisterView for DFB {
        type Register = super::ICC_SRE;
        const NAME: &'static str = "DFB";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICC_SRE\[0\]
    pub struct SRE;
    impl RegisterView for SRE {
        type Register = super::ICC_SRE;
        const NAME: &'static str = "SRE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Interrupt Priority Mask Register
#[allow(non_camel_case_types)]
pub struct ICC_PMR;
impl Register for ICC_PMR {
    const NAME: &'static str = "ICC_PMR";
    const LEN: usize = 64;
}

/// ICC_PMR register fields
pub mod icc_pmr {
    use crate::core::model::proc::RegisterView;

    /// ICC_PMR\[7:0\]
    pub struct Priority;
    impl RegisterView for Priority {
        type Register = super::ICC_PMR;
        const NAME: &'static str = "Priority";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Debug Device ID register 2
pub struct DBGDEVID2;
impl Register for DBGDEVID2 {
    const NAME: &'static str = "DBGDEVID2";
    const LEN: usize = 64;
}

/// Reset Management Register
pub struct RMR;
impl Register for RMR {
    const NAME: &'static str = "RMR";
    const LEN: usize = 64;
}

/// RMR register fields
pub mod rmr {
    use crate::core::model::proc::RegisterView;

    /// RMR\[1\]
    pub struct RR;
    impl RegisterView for RR {
        type Register = super::RMR;
        const NAME: &'static str = "RR";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// RMR\[0\]
    pub struct AA64;
    impl RegisterView for AA64 {
        type Register = super::RMR;
        const NAME: &'static str = "AA64";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Counter-timer Frequency register
pub struct CNTFRQ;
impl Register for CNTFRQ {
    const NAME: &'static str = "CNTFRQ";
    const LEN: usize = 64;
}

/// Selected Error Record Miscellaneous Register 1
pub struct ERXMISC1;
impl Register for ERXMISC1 {
    const NAME: &'static str = "ERXMISC1";
    const LEN: usize = 64;
}

/// ERXMISC1 register fields
pub mod erxmisc1 {
    use crate::core::model::proc::RegisterView;

    /// ERXMISC1\[31:0\]
    pub struct ERRnMISC0hi;
    impl RegisterView for ERRnMISC0hi {
        type Register = super::ERXMISC1;
        const NAME: &'static str = "ERRnMISC0hi";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Architectural Feature Access Control Register
pub struct CPACR;
impl Register for CPACR {
    const NAME: &'static str = "CPACR";
    const LEN: usize = 64;
}

/// CPACR register fields
pub mod cpacr {
    use crate::core::model::proc::RegisterView;

    /// CPACR\[31\]
    pub struct ASEDIS;
    impl RegisterView for ASEDIS {
        type Register = super::CPACR;
        const NAME: &'static str = "ASEDIS";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// CPACR\[28\]
    pub struct TRCDIS;
    impl RegisterView for TRCDIS {
        type Register = super::CPACR;
        const NAME: &'static str = "TRCDIS";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// CPACR\[23:22\]
    #[allow(non_camel_case_types)]
    pub struct cp11;
    impl RegisterView for cp11 {
        type Register = super::CPACR;
        const NAME: &'static str = "cp11";
        const OFFSET: usize = 22;
        const LEN: usize = 2;
    }

    /// CPACR\[21:20\]
    #[allow(non_camel_case_types)]
    pub struct cp10;
    impl RegisterView for cp10 {
        type Register = super::CPACR;
        const NAME: &'static str = "cp10";
        const OFFSET: usize = 20;
        const LEN: usize = 2;
    }
}

/// Hyp Configuration Register 2
pub struct HCR2;
impl Register for HCR2 {
    const NAME: &'static str = "HCR2";
    const LEN: usize = 64;
}

/// HCR2 register fields
pub mod hcr2 {
    use crate::core::model::proc::RegisterView;

    /// HCR2\[22\]
    pub struct TTLBIS;
    impl RegisterView for TTLBIS {
        type Register = super::HCR2;
        const NAME: &'static str = "TTLBIS";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// HCR2\[20\]
    pub struct TOCU;
    impl RegisterView for TOCU {
        type Register = super::HCR2;
        const NAME: &'static str = "TOCU";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// HCR2\[18\]
    pub struct TICAB;
    impl RegisterView for TICAB {
        type Register = super::HCR2;
        const NAME: &'static str = "TICAB";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// HCR2\[17\]
    pub struct TID4;
    impl RegisterView for TID4 {
        type Register = super::HCR2;
        const NAME: &'static str = "TID4";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// HCR2\[6\]
    pub struct MIOCNCE;
    impl RegisterView for MIOCNCE {
        type Register = super::HCR2;
        const NAME: &'static str = "MIOCNCE";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// HCR2\[5\]
    pub struct TEA;
    impl RegisterView for TEA {
        type Register = super::HCR2;
        const NAME: &'static str = "TEA";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// HCR2\[4\]
    pub struct TERR;
    impl RegisterView for TERR {
        type Register = super::HCR2;
        const NAME: &'static str = "TERR";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// HCR2\[1\]
    pub struct ID;
    impl RegisterView for ID {
        type Register = super::HCR2;
        const NAME: &'static str = "ID";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// HCR2\[0\]
    pub struct CD;
    impl RegisterView for CD {
        type Register = super::HCR2;
        const NAME: &'static str = "CD";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Highest Priority Pending Interrupt Register 1
#[allow(non_camel_case_types)]
pub struct ICC_HPPIR1;
impl Register for ICC_HPPIR1 {
    const NAME: &'static str = "ICC_HPPIR1";
    const LEN: usize = 64;
}

/// ICC_HPPIR1 register fields
pub mod icc_hppir1 {
    use crate::core::model::proc::RegisterView;

    /// ICC_HPPIR1\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_HPPIR1;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Performance Monitors Common Event Identification register 2
pub struct PMCEID2;
impl Register for PMCEID2 {
    const NAME: &'static str = "PMCEID2";
    const LEN: usize = 64;
}

/// Memory Model Feature Register 5
#[allow(non_camel_case_types)]
pub struct ID_MMFR5;
impl Register for ID_MMFR5 {
    const NAME: &'static str = "ID_MMFR5";
    const LEN: usize = 64;
}

/// ID_MMFR5 register fields
pub mod id_mmfr5 {
    use crate::core::model::proc::RegisterView;

    /// ID_MMFR5\[7:4\]
    #[allow(non_camel_case_types)]
    pub struct nTLBPA;
    impl RegisterView for nTLBPA {
        type Register = super::ID_MMFR5;
        const NAME: &'static str = "nTLBPA";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_MMFR5\[3:0\]
    pub struct ETS;
    impl RegisterView for ETS {
        type Register = super::ID_MMFR5;
        const NAME: &'static str = "ETS";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Hyp Auxiliary Control Register 2
pub struct HACTLR2;
impl Register for HACTLR2 {
    const NAME: &'static str = "HACTLR2";
    const LEN: usize = 64;
}

/// Interrupt Controller Monitor Interrupt Group 1 Enable register
#[allow(non_camel_case_types)]
pub struct ICC_MGRPEN1;
impl Register for ICC_MGRPEN1 {
    const NAME: &'static str = "ICC_MGRPEN1";
    const LEN: usize = 64;
}

/// ICC_MGRPEN1 register fields
pub mod icc_mgrpen1 {
    use crate::core::model::proc::RegisterView;

    /// ICC_MGRPEN1\[1\]
    pub struct EnableGrp1S;
    impl RegisterView for EnableGrp1S {
        type Register = super::ICC_MGRPEN1;
        const NAME: &'static str = "EnableGrp1S";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICC_MGRPEN1\[0\]
    pub struct EnableGrp1NS;
    impl RegisterView for EnableGrp1NS {
        type Register = super::ICC_MGRPEN1;
        const NAME: &'static str = "EnableGrp1NS";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Selected Error Record Primary Status Register
pub struct ERXSTATUS;
impl Register for ERXSTATUS {
    const NAME: &'static str = "ERXSTATUS";
    const LEN: usize = 64;
}

/// ERXSTATUS register fields
pub mod erxstatus {
    use crate::core::model::proc::RegisterView;

    /// ERXSTATUS\[31:0\]
    pub struct ERRnSTATUSlo;
    impl RegisterView for ERRnSTATUSlo {
        type Register = super::ERXSTATUS;
        const NAME: &'static str = "ERRnSTATUSlo";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Debug Watchpoint Value Registers, n = 15 - 0
pub struct DBGWVRn;
impl RegisterArray for DBGWVRn {
    const NAME: &'static str = "DBGWVRn";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// DBGWVRn register fields
pub mod dbgwvrn {
    use crate::core::model::proc::RegisterArrayView;

    /// DBGWVRn\[31:2\]
    pub struct VA;
    impl RegisterArrayView for VA {
        type RegisterArray = super::DBGWVRn;
        const NAME: &'static str = "VA";
        const OFFSET: usize = 2;
        const LEN: usize = 30;
    }
}

/// Interrupt Controller Software Generated Interrupt Group 0 Register
#[allow(non_camel_case_types)]
pub struct ICC_SGI0R;
impl Register for ICC_SGI0R {
    const NAME: &'static str = "ICC_SGI0R";
    const LEN: usize = 64;
}

/// ICC_SGI0R register fields
pub mod icc_sgi0r {
    use crate::core::model::proc::RegisterView;

    /// ICC_SGI0R\[55:48\]
    pub struct Aff3;
    impl RegisterView for Aff3 {
        type Register = super::ICC_SGI0R;
        const NAME: &'static str = "Aff3";
        const OFFSET: usize = 48;
        const LEN: usize = 8;
    }

    /// ICC_SGI0R\[47:44\]
    pub struct RS;
    impl RegisterView for RS {
        type Register = super::ICC_SGI0R;
        const NAME: &'static str = "RS";
        const OFFSET: usize = 44;
        const LEN: usize = 4;
    }

    /// ICC_SGI0R\[40\]
    pub struct IRM;
    impl RegisterView for IRM {
        type Register = super::ICC_SGI0R;
        const NAME: &'static str = "IRM";
        const OFFSET: usize = 40;
        const LEN: usize = 1;
    }

    /// ICC_SGI0R\[39:32\]
    pub struct Aff2;
    impl RegisterView for Aff2 {
        type Register = super::ICC_SGI0R;
        const NAME: &'static str = "Aff2";
        const OFFSET: usize = 32;
        const LEN: usize = 8;
    }

    /// ICC_SGI0R\[27:24\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_SGI0R;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ICC_SGI0R\[23:16\]
    pub struct Aff1;
    impl RegisterView for Aff1 {
        type Register = super::ICC_SGI0R;
        const NAME: &'static str = "Aff1";
        const OFFSET: usize = 16;
        const LEN: usize = 8;
    }

    /// ICC_SGI0R\[15:0\]
    pub struct TargetList;
    impl RegisterView for TargetList {
        type Register = super::ICC_SGI0R;
        const NAME: &'static str = "TargetList";
        const OFFSET: usize = 0;
        const LEN: usize = 16;
    }
}

/// Activity Monitors Count Enable Clear Register 1
pub struct AMCNTENCLR1;
impl Register for AMCNTENCLR1 {
    const NAME: &'static str = "AMCNTENCLR1";
    const LEN: usize = 64;
}

/// Hyp Auxiliary Instruction Fault Status Register
pub struct HAIFSR;
impl Register for HAIFSR {
    const NAME: &'static str = "HAIFSR";
    const LEN: usize = 64;
}

/// Debug Watchpoint Fault Address Register
pub struct DBGWFAR;
impl Register for DBGWFAR {
    const NAME: &'static str = "DBGWFAR";
    const LEN: usize = 64;
}

/// Activity Monitors Count Enable Clear Register 0
pub struct AMCNTENCLR0;
impl Register for AMCNTENCLR0 {
    const NAME: &'static str = "AMCNTENCLR0";
    const LEN: usize = 64;
}

/// Interrupt Status Register
pub struct ISR;
impl Register for ISR {
    const NAME: &'static str = "ISR";
    const LEN: usize = 64;
}

/// ISR register fields
pub mod isr {
    use crate::core::model::proc::RegisterView;

    /// ISR\[8\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::ISR;
        const NAME: &'static str = "A";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// ISR\[7\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::ISR;
        const NAME: &'static str = "I";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// ISR\[6\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::ISR;
        const NAME: &'static str = "F";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }
}

/// Error Record ID Register
pub struct ERRIDR;
impl Register for ERRIDR {
    const NAME: &'static str = "ERRIDR";
    const LEN: usize = 64;
}

/// ERRIDR register fields
pub mod erridr {
    use crate::core::model::proc::RegisterView;

    /// ERRIDR\[15:0\]
    pub struct NUM;
    impl RegisterView for NUM {
        type Register = super::ERRIDR;
        const NAME: &'static str = "NUM";
        const OFFSET: usize = 0;
        const LEN: usize = 16;
    }
}

/// Non-Secure Access Control Register
pub struct NSACR;
impl Register for NSACR {
    const NAME: &'static str = "NSACR";
    const LEN: usize = 64;
}

/// NSACR register fields
pub mod nsacr {
    use crate::core::model::proc::RegisterView;

    /// NSACR\[20\]
    pub struct NSTRCDIS;
    impl RegisterView for NSTRCDIS {
        type Register = super::NSACR;
        const NAME: &'static str = "NSTRCDIS";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// NSACR\[15\]
    pub struct NSASEDIS;
    impl RegisterView for NSASEDIS {
        type Register = super::NSACR;
        const NAME: &'static str = "NSASEDIS";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// NSACR\[11\]
    #[allow(non_camel_case_types)]
    pub struct cp11;
    impl RegisterView for cp11 {
        type Register = super::NSACR;
        const NAME: &'static str = "cp11";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// NSACR\[10\]
    #[allow(non_camel_case_types)]
    pub struct cp10;
    impl RegisterView for cp10 {
        type Register = super::NSACR;
        const NAME: &'static str = "cp10";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }
}

/// Hyp Trace Filter Control Register
pub struct HTRFCR;
impl Register for HTRFCR {
    const NAME: &'static str = "HTRFCR";
    const LEN: usize = 64;
}

/// HTRFCR register fields
pub mod htrfcr {
    use crate::core::model::proc::RegisterView;

    /// HTRFCR\[6:5\]
    pub struct TS;
    impl RegisterView for TS {
        type Register = super::HTRFCR;
        const NAME: &'static str = "TS";
        const OFFSET: usize = 5;
        const LEN: usize = 2;
    }

    /// HTRFCR\[3\]
    pub struct CX;
    impl RegisterView for CX {
        type Register = super::HTRFCR;
        const NAME: &'static str = "CX";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// HTRFCR\[1\]
    pub struct E2TRE;
    impl RegisterView for E2TRE {
        type Register = super::HTRFCR;
        const NAME: &'static str = "E2TRE";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// HTRFCR\[0\]
    pub struct E0HTRE;
    impl RegisterView for E0HTRE {
        type Register = super::HTRFCR;
        const NAME: &'static str = "E0HTRE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Memory Model Feature Register 4
#[allow(non_camel_case_types)]
pub struct ID_MMFR4;
impl Register for ID_MMFR4 {
    const NAME: &'static str = "ID_MMFR4";
    const LEN: usize = 64;
}

/// ID_MMFR4 register fields
pub mod id_mmfr4 {
    use crate::core::model::proc::RegisterView;

    /// ID_MMFR4\[31:28\]
    pub struct EVT;
    impl RegisterView for EVT {
        type Register = super::ID_MMFR4;
        const NAME: &'static str = "EVT";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_MMFR4\[27:24\]
    pub struct CCIDX;
    impl RegisterView for CCIDX {
        type Register = super::ID_MMFR4;
        const NAME: &'static str = "CCIDX";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_MMFR4\[23:20\]
    pub struct LSM;
    impl RegisterView for LSM {
        type Register = super::ID_MMFR4;
        const NAME: &'static str = "LSM";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_MMFR4\[19:16\]
    pub struct HPDS;
    impl RegisterView for HPDS {
        type Register = super::ID_MMFR4;
        const NAME: &'static str = "HPDS";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_MMFR4\[15:12\]
    pub struct CnP;
    impl RegisterView for CnP {
        type Register = super::ID_MMFR4;
        const NAME: &'static str = "CnP";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_MMFR4\[11:8\]
    pub struct XNX;
    impl RegisterView for XNX {
        type Register = super::ID_MMFR4;
        const NAME: &'static str = "XNX";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_MMFR4\[7:4\]
    pub struct AC2;
    impl RegisterView for AC2 {
        type Register = super::ID_MMFR4;
        const NAME: &'static str = "AC2";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_MMFR4\[3:0\]
    pub struct SpecSEI;
    impl RegisterView for SpecSEI {
        type Register = super::ID_MMFR4;
        const NAME: &'static str = "SpecSEI";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Performance Monitors Common Event Identification register 3
pub struct PMCEID3;
impl Register for PMCEID3 {
    const NAME: &'static str = "PMCEID3";
    const LEN: usize = 64;
}

/// Interrupt Controller Monitor System Register Enable register
#[allow(non_camel_case_types)]
pub struct ICC_MSRE;
impl Register for ICC_MSRE {
    const NAME: &'static str = "ICC_MSRE";
    const LEN: usize = 64;
}

/// ICC_MSRE register fields
pub mod icc_msre {
    use crate::core::model::proc::RegisterView;

    /// ICC_MSRE\[3\]
    pub struct Enable;
    impl RegisterView for Enable {
        type Register = super::ICC_MSRE;
        const NAME: &'static str = "Enable";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// ICC_MSRE\[2\]
    pub struct DIB;
    impl RegisterView for DIB {
        type Register = super::ICC_MSRE;
        const NAME: &'static str = "DIB";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// ICC_MSRE\[1\]
    pub struct DFB;
    impl RegisterView for DFB {
        type Register = super::ICC_MSRE;
        const NAME: &'static str = "DFB";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICC_MSRE\[0\]
    pub struct SRE;
    impl RegisterView for SRE {
        type Register = super::ICC_MSRE;
        const NAME: &'static str = "SRE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Performance Monitors Control Register
pub struct PMCR;
impl Register for PMCR {
    const NAME: &'static str = "PMCR";
    const LEN: usize = 64;
}

/// PMCR register fields
pub mod pmcr {
    use crate::core::model::proc::RegisterView;

    /// PMCR\[31:24\]
    pub struct IMP;
    impl RegisterView for IMP {
        type Register = super::PMCR;
        const NAME: &'static str = "IMP";
        const OFFSET: usize = 24;
        const LEN: usize = 8;
    }

    /// PMCR\[23:16\]
    pub struct IDCODE;
    impl RegisterView for IDCODE {
        type Register = super::PMCR;
        const NAME: &'static str = "IDCODE";
        const OFFSET: usize = 16;
        const LEN: usize = 8;
    }

    /// PMCR\[15:11\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::PMCR;
        const NAME: &'static str = "N";
        const OFFSET: usize = 11;
        const LEN: usize = 5;
    }

    /// PMCR\[9\]
    pub struct FZO;
    impl RegisterView for FZO {
        type Register = super::PMCR;
        const NAME: &'static str = "FZO";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// PMCR\[7\]
    pub struct LP;
    impl RegisterView for LP {
        type Register = super::PMCR;
        const NAME: &'static str = "LP";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// PMCR\[6\]
    pub struct LC;
    impl RegisterView for LC {
        type Register = super::PMCR;
        const NAME: &'static str = "LC";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// PMCR\[5\]
    pub struct DP;
    impl RegisterView for DP {
        type Register = super::PMCR;
        const NAME: &'static str = "DP";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// PMCR\[4\]
    pub struct X;
    impl RegisterView for X {
        type Register = super::PMCR;
        const NAME: &'static str = "X";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// PMCR\[3\]
    pub struct D;
    impl RegisterView for D {
        type Register = super::PMCR;
        const NAME: &'static str = "D";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// PMCR\[2\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::PMCR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// PMCR\[1\]
    pub struct P;
    impl RegisterView for P {
        type Register = super::PMCR;
        const NAME: &'static str = "P";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// PMCR\[0\]
    pub struct E;
    impl RegisterView for E {
        type Register = super::PMCR;
        const NAME: &'static str = "E";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Highest Priority Pending Interrupt Register 0
#[allow(non_camel_case_types)]
pub struct ICC_HPPIR0;
impl Register for ICC_HPPIR0 {
    const NAME: &'static str = "ICC_HPPIR0";
    const LEN: usize = 64;
}

/// ICC_HPPIR0 register fields
pub mod icc_hppir0 {
    use crate::core::model::proc::RegisterView;

    /// ICC_HPPIR0\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_HPPIR0;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Performance Monitors Common Event Identification register 1
pub struct PMCEID1;
impl Register for PMCEID1 {
    const NAME: &'static str = "PMCEID1";
    const LEN: usize = 64;
}

/// Translation Table Base Control Register 2
pub struct TTBCR2;
impl Register for TTBCR2 {
    const NAME: &'static str = "TTBCR2";
    const LEN: usize = 64;
}

/// TTBCR2 register fields
pub mod ttbcr2 {
    use crate::core::model::proc::RegisterView;

    /// TTBCR2\[18\]
    pub struct HWU162;
    impl RegisterView for HWU162 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HWU162";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// TTBCR2\[17\]
    pub struct HWU161;
    impl RegisterView for HWU161 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HWU161";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// TTBCR2\[16\]
    pub struct HWU160;
    impl RegisterView for HWU160 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HWU160";
        const OFFSET: usize = 16;
        const LEN: usize = 1;
    }

    /// TTBCR2\[15\]
    pub struct HWU159;
    impl RegisterView for HWU159 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HWU159";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// TTBCR2\[14\]
    pub struct HWU062;
    impl RegisterView for HWU062 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HWU062";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// TTBCR2\[13\]
    pub struct HWU061;
    impl RegisterView for HWU061 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HWU061";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// TTBCR2\[12\]
    pub struct HWU060;
    impl RegisterView for HWU060 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HWU060";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// TTBCR2\[11\]
    pub struct HWU059;
    impl RegisterView for HWU059 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HWU059";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// TTBCR2\[10\]
    pub struct HPD1;
    impl RegisterView for HPD1 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HPD1";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }

    /// TTBCR2\[9\]
    pub struct HPD0;
    impl RegisterView for HPD0 {
        type Register = super::TTBCR2;
        const NAME: &'static str = "HPD0";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Virtual Control Register
#[allow(non_camel_case_types)]
pub struct ICV_CTLR;
impl Register for ICV_CTLR {
    const NAME: &'static str = "ICV_CTLR";
    const LEN: usize = 64;
}

/// ICV_CTLR register fields
pub mod icv_ctlr {
    use crate::core::model::proc::RegisterView;

    /// ICV_CTLR\[19\]
    pub struct ExtRange;
    impl RegisterView for ExtRange {
        type Register = super::ICV_CTLR;
        const NAME: &'static str = "ExtRange";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// ICV_CTLR\[18\]
    pub struct RSS;
    impl RegisterView for RSS {
        type Register = super::ICV_CTLR;
        const NAME: &'static str = "RSS";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// ICV_CTLR\[15\]
    pub struct A3V;
    impl RegisterView for A3V {
        type Register = super::ICV_CTLR;
        const NAME: &'static str = "A3V";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// ICV_CTLR\[14\]
    pub struct SEIS;
    impl RegisterView for SEIS {
        type Register = super::ICV_CTLR;
        const NAME: &'static str = "SEIS";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// ICV_CTLR\[13:11\]
    pub struct IDbits;
    impl RegisterView for IDbits {
        type Register = super::ICV_CTLR;
        const NAME: &'static str = "IDbits";
        const OFFSET: usize = 11;
        const LEN: usize = 3;
    }

    /// ICV_CTLR\[10:8\]
    pub struct PRIbits;
    impl RegisterView for PRIbits {
        type Register = super::ICV_CTLR;
        const NAME: &'static str = "PRIbits";
        const OFFSET: usize = 8;
        const LEN: usize = 3;
    }

    /// ICV_CTLR\[1\]
    pub struct EOImode;
    impl RegisterView for EOImode {
        type Register = super::ICV_CTLR;
        const NAME: &'static str = "EOImode";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICV_CTLR\[0\]
    pub struct CBPR;
    impl RegisterView for CBPR {
        type Register = super::ICV_CTLR;
        const NAME: &'static str = "CBPR";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Counter-timer Secure Physical Timer TimerValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHPS_TVAL;
impl Register for CNTHPS_TVAL {
    const NAME: &'static str = "CNTHPS_TVAL";
    const LEN: usize = 64;
}

/// CNTHPS_TVAL register fields
pub mod cnthps_tval {
    use crate::core::model::proc::RegisterView;

    /// CNTHPS_TVAL\[31:0\]
    pub struct TimerValue;
    impl RegisterView for TimerValue {
        type Register = super::CNTHPS_TVAL;
        const NAME: &'static str = "TimerValue";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Selected Error Record Feature Register 2
pub struct ERXFR2;
impl Register for ERXFR2 {
    const NAME: &'static str = "ERXFR2";
    const LEN: usize = 64;
}

/// ERXFR2 register fields
pub mod erxfr2 {
    use crate::core::model::proc::RegisterView;

    /// ERXFR2\[31:0\]
    pub struct ERRnFRhi;
    impl RegisterView for ERRnFRhi {
        type Register = super::ERXFR2;
        const NAME: &'static str = "ERRnFRhi";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Floating-Point Status and Control Register
pub struct FPSCR;
impl Register for FPSCR {
    const NAME: &'static str = "FPSCR";
    const LEN: usize = 64;
}

/// FPSCR register fields
pub mod fpscr {
    use crate::core::model::proc::RegisterView;

    /// FPSCR\[31\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::FPSCR;
        const NAME: &'static str = "N";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// FPSCR\[30\]
    pub struct Z;
    impl RegisterView for Z {
        type Register = super::FPSCR;
        const NAME: &'static str = "Z";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// FPSCR\[29\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::FPSCR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// FPSCR\[28\]
    pub struct V;
    impl RegisterView for V {
        type Register = super::FPSCR;
        const NAME: &'static str = "V";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// FPSCR\[27\]
    pub struct QC;
    impl RegisterView for QC {
        type Register = super::FPSCR;
        const NAME: &'static str = "QC";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// FPSCR\[26\]
    pub struct AHP;
    impl RegisterView for AHP {
        type Register = super::FPSCR;
        const NAME: &'static str = "AHP";
        const OFFSET: usize = 26;
        const LEN: usize = 1;
    }

    /// FPSCR\[25\]
    pub struct DN;
    impl RegisterView for DN {
        type Register = super::FPSCR;
        const NAME: &'static str = "DN";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// FPSCR\[24\]
    pub struct FZ;
    impl RegisterView for FZ {
        type Register = super::FPSCR;
        const NAME: &'static str = "FZ";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// FPSCR\[23:22\]
    pub struct RMode;
    impl RegisterView for RMode {
        type Register = super::FPSCR;
        const NAME: &'static str = "RMode";
        const OFFSET: usize = 22;
        const LEN: usize = 2;
    }

    /// FPSCR\[21:20\]
    pub struct Stride;
    impl RegisterView for Stride {
        type Register = super::FPSCR;
        const NAME: &'static str = "Stride";
        const OFFSET: usize = 20;
        const LEN: usize = 2;
    }

    /// FPSCR\[19\]
    pub struct FZ16;
    impl RegisterView for FZ16 {
        type Register = super::FPSCR;
        const NAME: &'static str = "FZ16";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// FPSCR\[18:16\]
    pub struct Len;
    impl RegisterView for Len {
        type Register = super::FPSCR;
        const NAME: &'static str = "Len";
        const OFFSET: usize = 16;
        const LEN: usize = 3;
    }

    /// FPSCR\[15\]
    pub struct IDE;
    impl RegisterView for IDE {
        type Register = super::FPSCR;
        const NAME: &'static str = "IDE";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// FPSCR\[12\]
    pub struct IXE;
    impl RegisterView for IXE {
        type Register = super::FPSCR;
        const NAME: &'static str = "IXE";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// FPSCR\[11\]
    pub struct UFE;
    impl RegisterView for UFE {
        type Register = super::FPSCR;
        const NAME: &'static str = "UFE";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// FPSCR\[10\]
    pub struct OFE;
    impl RegisterView for OFE {
        type Register = super::FPSCR;
        const NAME: &'static str = "OFE";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }

    /// FPSCR\[9\]
    pub struct DZE;
    impl RegisterView for DZE {
        type Register = super::FPSCR;
        const NAME: &'static str = "DZE";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// FPSCR\[8\]
    pub struct IOE;
    impl RegisterView for IOE {
        type Register = super::FPSCR;
        const NAME: &'static str = "IOE";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// FPSCR\[7\]
    pub struct IDC;
    impl RegisterView for IDC {
        type Register = super::FPSCR;
        const NAME: &'static str = "IDC";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// FPSCR\[4\]
    pub struct IXC;
    impl RegisterView for IXC {
        type Register = super::FPSCR;
        const NAME: &'static str = "IXC";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// FPSCR\[3\]
    pub struct UFC;
    impl RegisterView for UFC {
        type Register = super::FPSCR;
        const NAME: &'static str = "UFC";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// FPSCR\[2\]
    pub struct OFC;
    impl RegisterView for OFC {
        type Register = super::FPSCR;
        const NAME: &'static str = "OFC";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// FPSCR\[1\]
    pub struct DZC;
    impl RegisterView for DZC {
        type Register = super::FPSCR;
        const NAME: &'static str = "DZC";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// FPSCR\[0\]
    pub struct IOC;
    impl RegisterView for IOC {
        type Register = super::FPSCR;
        const NAME: &'static str = "IOC";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Hyp Memory Attribute Indirection Register 1
pub struct HMAIR1;
impl Register for HMAIR1 {
    const NAME: &'static str = "HMAIR1";
    const LEN: usize = 64;
}

/// Counter-timer Physical Timer TimerValue register
#[allow(non_camel_case_types)]
pub struct CNTP_TVAL;
impl Register for CNTP_TVAL {
    const NAME: &'static str = "CNTP_TVAL";
    const LEN: usize = 64;
}

/// CNTP_TVAL register fields
pub mod cntp_tval {
    use crate::core::model::proc::RegisterView;

    /// CNTP_TVAL\[31:0\]
    pub struct TimerValue;
    impl RegisterView for TimerValue {
        type Register = super::CNTP_TVAL;
        const NAME: &'static str = "TimerValue";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Interrupt Controller End of Interrupt Status Register
#[allow(non_camel_case_types)]
pub struct ICH_EISR;
impl Register for ICH_EISR {
    const NAME: &'static str = "ICH_EISR";
    const LEN: usize = 64;
}

/// Hyp Memory Attribute Indirection Register 0
pub struct HMAIR0;
impl Register for HMAIR0 {
    const NAME: &'static str = "HMAIR0";
    const LEN: usize = 64;
}

/// Virtual Deferred Interrupt Status Register
pub struct VDISR;
impl Register for VDISR {
    const NAME: &'static str = "VDISR";
    const LEN: usize = 64;
}

/// VDISR register fields
pub mod vdisr {
    use crate::core::model::proc::RegisterView;

    /// VDISR\[31\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::VDISR;
        const NAME: &'static str = "A";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// VDISR\[15:14\]
    pub struct AET;
    impl RegisterView for AET {
        type Register = super::VDISR;
        const NAME: &'static str = "AET";
        const OFFSET: usize = 14;
        const LEN: usize = 2;
    }

    /// VDISR\[12\]
    pub struct ExT;
    impl RegisterView for ExT {
        type Register = super::VDISR;
        const NAME: &'static str = "ExT";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// VDISR\[3:0\]
    pub struct FS;
    impl RegisterView for FS {
        type Register = super::VDISR;
        const NAME: &'static str = "FS";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }

    /// VDISR\[9\]
    pub struct LPAE;
    impl RegisterView for LPAE {
        type Register = super::VDISR;
        const NAME: &'static str = "LPAE";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// VDISR\[5:0\]
    pub struct STATUS;
    impl RegisterView for STATUS {
        type Register = super::VDISR;
        const NAME: &'static str = "STATUS";
        const OFFSET: usize = 0;
        const LEN: usize = 6;
    }
}

/// PL1 Software Thread ID Register
pub struct TPIDRPRW;
impl Register for TPIDRPRW {
    const NAME: &'static str = "TPIDRPRW";
    const LEN: usize = 64;
}

/// TPIDRPRW register fields
pub mod tpidrprw {
    use crate::core::model::proc::RegisterView;

    /// TPIDRPRW\[31:0\]
    pub struct TID;
    impl RegisterView for TID {
        type Register = super::TPIDRPRW;
        const NAME: &'static str = "TID";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Hyp System Control Register
pub struct HSCTLR;
impl Register for HSCTLR {
    const NAME: &'static str = "HSCTLR";
    const LEN: usize = 64;
}

/// HSCTLR register fields
pub mod hsctlr {
    use crate::core::model::proc::RegisterView;

    /// HSCTLR\[31\]
    pub struct DSSBS;
    impl RegisterView for DSSBS {
        type Register = super::HSCTLR;
        const NAME: &'static str = "DSSBS";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// HSCTLR\[30\]
    pub struct TE;
    impl RegisterView for TE {
        type Register = super::HSCTLR;
        const NAME: &'static str = "TE";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// HSCTLR\[25\]
    pub struct EE;
    impl RegisterView for EE {
        type Register = super::HSCTLR;
        const NAME: &'static str = "EE";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// HSCTLR\[19\]
    pub struct WXN;
    impl RegisterView for WXN {
        type Register = super::HSCTLR;
        const NAME: &'static str = "WXN";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// HSCTLR\[12\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::HSCTLR;
        const NAME: &'static str = "I";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// HSCTLR\[8\]
    pub struct SED;
    impl RegisterView for SED {
        type Register = super::HSCTLR;
        const NAME: &'static str = "SED";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// HSCTLR\[7\]
    pub struct ITD;
    impl RegisterView for ITD {
        type Register = super::HSCTLR;
        const NAME: &'static str = "ITD";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// HSCTLR\[5\]
    pub struct CP15BEN;
    impl RegisterView for CP15BEN {
        type Register = super::HSCTLR;
        const NAME: &'static str = "CP15BEN";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// HSCTLR\[4\]
    pub struct LSMAOE;
    impl RegisterView for LSMAOE {
        type Register = super::HSCTLR;
        const NAME: &'static str = "LSMAOE";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// HSCTLR\[3\]
    #[allow(non_camel_case_types)]
    pub struct nTLSMD;
    impl RegisterView for nTLSMD {
        type Register = super::HSCTLR;
        const NAME: &'static str = "nTLSMD";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// HSCTLR\[2\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::HSCTLR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// HSCTLR\[1\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::HSCTLR;
        const NAME: &'static str = "A";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// HSCTLR\[0\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::HSCTLR;
        const NAME: &'static str = "M";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Hyp Translation Control Register
pub struct HTCR;
impl Register for HTCR {
    const NAME: &'static str = "HTCR";
    const LEN: usize = 64;
}

/// HTCR register fields
pub mod htcr {
    use crate::core::model::proc::RegisterView;

    /// HTCR\[28\]
    pub struct HWU62;
    impl RegisterView for HWU62 {
        type Register = super::HTCR;
        const NAME: &'static str = "HWU62";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// HTCR\[27\]
    pub struct HWU61;
    impl RegisterView for HWU61 {
        type Register = super::HTCR;
        const NAME: &'static str = "HWU61";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// HTCR\[26\]
    pub struct HWU60;
    impl RegisterView for HWU60 {
        type Register = super::HTCR;
        const NAME: &'static str = "HWU60";
        const OFFSET: usize = 26;
        const LEN: usize = 1;
    }

    /// HTCR\[25\]
    pub struct HWU59;
    impl RegisterView for HWU59 {
        type Register = super::HTCR;
        const NAME: &'static str = "HWU59";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// HTCR\[24\]
    pub struct HPD;
    impl RegisterView for HPD {
        type Register = super::HTCR;
        const NAME: &'static str = "HPD";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// HTCR\[13:12\]
    pub struct SH0;
    impl RegisterView for SH0 {
        type Register = super::HTCR;
        const NAME: &'static str = "SH0";
        const OFFSET: usize = 12;
        const LEN: usize = 2;
    }

    /// HTCR\[11:10\]
    pub struct ORGN0;
    impl RegisterView for ORGN0 {
        type Register = super::HTCR;
        const NAME: &'static str = "ORGN0";
        const OFFSET: usize = 10;
        const LEN: usize = 2;
    }

    /// HTCR\[9:8\]
    pub struct IRGN0;
    impl RegisterView for IRGN0 {
        type Register = super::HTCR;
        const NAME: &'static str = "IRGN0";
        const OFFSET: usize = 8;
        const LEN: usize = 2;
    }

    /// HTCR\[2:0\]
    pub struct T0SZ;
    impl RegisterView for T0SZ {
        type Register = super::HTCR;
        const NAME: &'static str = "T0SZ";
        const OFFSET: usize = 0;
        const LEN: usize = 3;
    }
}

/// Saved Program Status Register (Undefined mode)
#[allow(non_camel_case_types)]
pub struct SPSR_und;
impl Register for SPSR_und {
    const NAME: &'static str = "SPSR_und";
    const LEN: usize = 64;
}

/// SPSR_und register fields
pub mod spsr_und {
    use crate::core::model::proc::RegisterView;

    /// SPSR_und\[31\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::SPSR_und;
        const NAME: &'static str = "N";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// SPSR_und\[30\]
    pub struct Z;
    impl RegisterView for Z {
        type Register = super::SPSR_und;
        const NAME: &'static str = "Z";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// SPSR_und\[29\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::SPSR_und;
        const NAME: &'static str = "C";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// SPSR_und\[28\]
    pub struct V;
    impl RegisterView for V {
        type Register = super::SPSR_und;
        const NAME: &'static str = "V";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// SPSR_und\[27\]
    pub struct Q;
    impl RegisterView for Q {
        type Register = super::SPSR_und;
        const NAME: &'static str = "Q";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// SPSR_und\[15:10\]
    pub struct IT;
    impl RegisterView for IT {
        type Register = super::SPSR_und;
        const NAME: &'static str = "IT";
        const OFFSET: usize = 10;
        const LEN: usize = 6;
    }

    /// SPSR_und\[24\]
    pub struct J;
    impl RegisterView for J {
        type Register = super::SPSR_und;
        const NAME: &'static str = "J";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// SPSR_und\[23\]
    pub struct SSBS;
    impl RegisterView for SSBS {
        type Register = super::SPSR_und;
        const NAME: &'static str = "SSBS";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// SPSR_und\[22\]
    pub struct PAN;
    impl RegisterView for PAN {
        type Register = super::SPSR_und;
        const NAME: &'static str = "PAN";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// SPSR_und\[21\]
    pub struct DIT;
    impl RegisterView for DIT {
        type Register = super::SPSR_und;
        const NAME: &'static str = "DIT";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// SPSR_und\[20\]
    pub struct IL;
    impl RegisterView for IL {
        type Register = super::SPSR_und;
        const NAME: &'static str = "IL";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// SPSR_und\[19:16\]
    pub struct GE;
    impl RegisterView for GE {
        type Register = super::SPSR_und;
        const NAME: &'static str = "GE";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// SPSR_und\[9\]
    pub struct E;
    impl RegisterView for E {
        type Register = super::SPSR_und;
        const NAME: &'static str = "E";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// SPSR_und\[8\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::SPSR_und;
        const NAME: &'static str = "A";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// SPSR_und\[7\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::SPSR_und;
        const NAME: &'static str = "I";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// SPSR_und\[6\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::SPSR_und;
        const NAME: &'static str = "F";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// SPSR_und\[5\]
    pub struct T;
    impl RegisterView for T {
        type Register = super::SPSR_und;
        const NAME: &'static str = "T";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// SPSR_und\[4:0\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::SPSR_und;
        const NAME: &'static str = "M";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Performance Monitors Common Event Identification register 0
pub struct PMCEID0;
impl Register for PMCEID0 {
    const NAME: &'static str = "PMCEID0";
    const LEN: usize = 64;
}

/// Activity Monitors Event Counter Registers 0, n = 3 - 0
pub struct AMEVCNTR0n;
impl RegisterArray for AMEVCNTR0n {
    const NAME: &'static str = "AMEVCNTR0n";
    const ELEMS: usize = 3;
    const ELEM_LEN: usize = 64;
}

/// AMEVCNTR0n register fields
pub mod amevcntr0n {
    use crate::core::model::proc::RegisterArrayView;

    /// AMEVCNTR0n\[63:0\]
    pub struct ACNT;
    impl RegisterArrayView for ACNT {
        type RegisterArray = super::AMEVCNTR0n;
        const NAME: &'static str = "ACNT";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Activity Monitors Event Counter Registers 1, n = 15 - 0
pub struct AMEVCNTR1n;
impl RegisterArray for AMEVCNTR1n {
    const NAME: &'static str = "AMEVCNTR1n";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// AMEVCNTR1n register fields
pub mod amevcntr1n {
    use crate::core::model::proc::RegisterArrayView;

    /// AMEVCNTR1n\[63:0\]
    pub struct ACNT;
    impl RegisterArrayView for ACNT {
        type RegisterArray = super::AMEVCNTR1n;
        const NAME: &'static str = "ACNT";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Debug Device ID register 0
pub struct DBGDEVID;
impl Register for DBGDEVID {
    const NAME: &'static str = "DBGDEVID";
    const LEN: usize = 64;
}

/// DBGDEVID register fields
pub mod dbgdevid {
    use crate::core::model::proc::RegisterView;

    /// DBGDEVID\[31:28\]
    pub struct CIDMask;
    impl RegisterView for CIDMask {
        type Register = super::DBGDEVID;
        const NAME: &'static str = "CIDMask";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// DBGDEVID\[27:24\]
    pub struct AuxRegs;
    impl RegisterView for AuxRegs {
        type Register = super::DBGDEVID;
        const NAME: &'static str = "AuxRegs";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// DBGDEVID\[23:20\]
    pub struct DoubleLock;
    impl RegisterView for DoubleLock {
        type Register = super::DBGDEVID;
        const NAME: &'static str = "DoubleLock";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// DBGDEVID\[19:16\]
    pub struct VirtExtns;
    impl RegisterView for VirtExtns {
        type Register = super::DBGDEVID;
        const NAME: &'static str = "VirtExtns";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// DBGDEVID\[15:12\]
    pub struct VectorCatch;
    impl RegisterView for VectorCatch {
        type Register = super::DBGDEVID;
        const NAME: &'static str = "VectorCatch";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// DBGDEVID\[11:8\]
    pub struct BPAddrMask;
    impl RegisterView for BPAddrMask {
        type Register = super::DBGDEVID;
        const NAME: &'static str = "BPAddrMask";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// DBGDEVID\[7:4\]
    pub struct WPAddrMask;
    impl RegisterView for WPAddrMask {
        type Register = super::DBGDEVID;
        const NAME: &'static str = "WPAddrMask";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// DBGDEVID\[3:0\]
    pub struct PCSample;
    impl RegisterView for PCSample {
        type Register = super::DBGDEVID;
        const NAME: &'static str = "PCSample";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Memory Model Feature Register 3
#[allow(non_camel_case_types)]
pub struct ID_MMFR3;
impl Register for ID_MMFR3 {
    const NAME: &'static str = "ID_MMFR3";
    const LEN: usize = 64;
}

/// ID_MMFR3 register fields
pub mod id_mmfr3 {
    use crate::core::model::proc::RegisterView;

    /// ID_MMFR3\[31:28\]
    pub struct Supersec;
    impl RegisterView for Supersec {
        type Register = super::ID_MMFR3;
        const NAME: &'static str = "Supersec";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_MMFR3\[27:24\]
    pub struct CMemSz;
    impl RegisterView for CMemSz {
        type Register = super::ID_MMFR3;
        const NAME: &'static str = "CMemSz";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_MMFR3\[23:20\]
    pub struct CohWalk;
    impl RegisterView for CohWalk {
        type Register = super::ID_MMFR3;
        const NAME: &'static str = "CohWalk";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_MMFR3\[19:16\]
    pub struct PAN;
    impl RegisterView for PAN {
        type Register = super::ID_MMFR3;
        const NAME: &'static str = "PAN";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_MMFR3\[15:12\]
    pub struct MaintBcst;
    impl RegisterView for MaintBcst {
        type Register = super::ID_MMFR3;
        const NAME: &'static str = "MaintBcst";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_MMFR3\[11:8\]
    pub struct BPMaint;
    impl RegisterView for BPMaint {
        type Register = super::ID_MMFR3;
        const NAME: &'static str = "BPMaint";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_MMFR3\[7:4\]
    pub struct CMaintSW;
    impl RegisterView for CMaintSW {
        type Register = super::ID_MMFR3;
        const NAME: &'static str = "CMaintSW";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_MMFR3\[3:0\]
    pub struct CMaintVA;
    impl RegisterView for CMaintVA {
        type Register = super::ID_MMFR3;
        const NAME: &'static str = "CMaintVA";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Debug Self Address Register
pub struct DBGDSAR;
impl Register for DBGDSAR {
    const NAME: &'static str = "DBGDSAR";
    const LEN: usize = 64;
}

/// Activity Monitors Configuration Register
pub struct AMCFGR;
impl Register for AMCFGR {
    const NAME: &'static str = "AMCFGR";
    const LEN: usize = 64;
}

/// AMCFGR register fields
pub mod amcfgr {
    use crate::core::model::proc::RegisterView;

    /// AMCFGR\[31:28\]
    pub struct NCG;
    impl RegisterView for NCG {
        type Register = super::AMCFGR;
        const NAME: &'static str = "NCG";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// AMCFGR\[24\]
    pub struct HDBG;
    impl RegisterView for HDBG {
        type Register = super::AMCFGR;
        const NAME: &'static str = "HDBG";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// AMCFGR\[13:8\]
    pub struct SIZE;
    impl RegisterView for SIZE {
        type Register = super::AMCFGR;
        const NAME: &'static str = "SIZE";
        const OFFSET: usize = 8;
        const LEN: usize = 6;
    }

    /// AMCFGR\[7:0\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::AMCFGR;
        const NAME: &'static str = "N";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Data Fault Status Register
pub struct DFSR;
impl Register for DFSR {
    const NAME: &'static str = "DFSR";
    const LEN: usize = 64;
}

/// DFSR register fields
pub mod dfsr {
    use crate::core::model::proc::RegisterView;

    /// DFSR\[16\]
    pub struct FnV;
    impl RegisterView for FnV {
        type Register = super::DFSR;
        const NAME: &'static str = "FnV";
        const OFFSET: usize = 16;
        const LEN: usize = 1;
    }

    /// DFSR\[15:14\]
    pub struct AET;
    impl RegisterView for AET {
        type Register = super::DFSR;
        const NAME: &'static str = "AET";
        const OFFSET: usize = 14;
        const LEN: usize = 2;
    }

    /// DFSR\[13\]
    pub struct CM;
    impl RegisterView for CM {
        type Register = super::DFSR;
        const NAME: &'static str = "CM";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// DFSR\[12\]
    pub struct ExT;
    impl RegisterView for ExT {
        type Register = super::DFSR;
        const NAME: &'static str = "ExT";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// DFSR\[11\]
    pub struct WnR;
    impl RegisterView for WnR {
        type Register = super::DFSR;
        const NAME: &'static str = "WnR";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// DFSR\[3:0\]
    pub struct FS;
    impl RegisterView for FS {
        type Register = super::DFSR;
        const NAME: &'static str = "FS";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }

    /// DFSR\[9\]
    pub struct LPAE;
    impl RegisterView for LPAE {
        type Register = super::DFSR;
        const NAME: &'static str = "LPAE";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// DFSR\[7:4\]
    pub struct Domain;
    impl RegisterView for Domain {
        type Register = super::DFSR;
        const NAME: &'static str = "Domain";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// DFSR\[5:0\]
    pub struct STATUS;
    impl RegisterView for STATUS {
        type Register = super::DFSR;
        const NAME: &'static str = "STATUS";
        const OFFSET: usize = 0;
        const LEN: usize = 6;
    }
}

/// Hyp Instruction Fault Address Register
pub struct HIFAR;
impl Register for HIFAR {
    const NAME: &'static str = "HIFAR";
    const LEN: usize = 64;
}

/// HIFAR register fields
pub mod hifar {
    use crate::core::model::proc::RegisterView;

    /// HIFAR\[31:0\]
    pub struct VA;
    impl RegisterView for VA {
        type Register = super::HIFAR;
        const NAME: &'static str = "VA";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Saved Program Status Register (Hyp mode)
#[allow(non_camel_case_types)]
pub struct SPSR_hyp;
impl Register for SPSR_hyp {
    const NAME: &'static str = "SPSR_hyp";
    const LEN: usize = 64;
}

/// SPSR_hyp register fields
pub mod spsr_hyp {
    use crate::core::model::proc::RegisterView;

    /// SPSR_hyp\[31\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "N";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[30\]
    pub struct Z;
    impl RegisterView for Z {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "Z";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[29\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "C";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[28\]
    pub struct V;
    impl RegisterView for V {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "V";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[27\]
    pub struct Q;
    impl RegisterView for Q {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "Q";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[15:10\]
    pub struct IT;
    impl RegisterView for IT {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "IT";
        const OFFSET: usize = 10;
        const LEN: usize = 6;
    }

    /// SPSR_hyp\[24\]
    pub struct J;
    impl RegisterView for J {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "J";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[23\]
    pub struct SSBS;
    impl RegisterView for SSBS {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "SSBS";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[22\]
    pub struct PAN;
    impl RegisterView for PAN {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "PAN";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[21\]
    pub struct DIT;
    impl RegisterView for DIT {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "DIT";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[20\]
    pub struct IL;
    impl RegisterView for IL {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "IL";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[19:16\]
    pub struct GE;
    impl RegisterView for GE {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "GE";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// SPSR_hyp\[9\]
    pub struct E;
    impl RegisterView for E {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "E";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[8\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "A";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[7\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "I";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[6\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "F";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[5\]
    pub struct T;
    impl RegisterView for T {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "T";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// SPSR_hyp\[4:0\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::SPSR_hyp;
        const NAME: &'static str = "M";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Monitor Vector Base Address Register
pub struct MVBAR;
impl Register for MVBAR {
    const NAME: &'static str = "MVBAR";
    const LEN: usize = 64;
}

/// MVBAR register fields
pub mod mvbar {
    use crate::core::model::proc::RegisterView;

    /// MVBAR\[31:5\]
    pub struct VBA;
    impl RegisterView for VBA {
        type Register = super::MVBAR;
        const NAME: &'static str = "VBA";
        const OFFSET: usize = 5;
        const LEN: usize = 27;
    }

    /// MVBAR\[4:0\]
    pub struct Reserved;
    impl RegisterView for Reserved {
        type Register = super::MVBAR;
        const NAME: &'static str = "Reserved";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Auxiliary Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_AFR0;
impl Register for ID_AFR0 {
    const NAME: &'static str = "ID_AFR0";
    const LEN: usize = 64;
}

/// Saved Program Status Register
pub struct SPSR;
impl Register for SPSR {
    const NAME: &'static str = "SPSR";
    const LEN: usize = 64;
}

/// SPSR register fields
pub mod spsr {
    use crate::core::model::proc::RegisterView;

    /// SPSR\[31\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::SPSR;
        const NAME: &'static str = "N";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// SPSR\[30\]
    pub struct Z;
    impl RegisterView for Z {
        type Register = super::SPSR;
        const NAME: &'static str = "Z";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// SPSR\[29\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::SPSR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// SPSR\[28\]
    pub struct V;
    impl RegisterView for V {
        type Register = super::SPSR;
        const NAME: &'static str = "V";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// SPSR\[27\]
    pub struct Q;
    impl RegisterView for Q {
        type Register = super::SPSR;
        const NAME: &'static str = "Q";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// SPSR\[15:10\]
    pub struct IT;
    impl RegisterView for IT {
        type Register = super::SPSR;
        const NAME: &'static str = "IT";
        const OFFSET: usize = 10;
        const LEN: usize = 6;
    }

    /// SPSR\[24\]
    pub struct J;
    impl RegisterView for J {
        type Register = super::SPSR;
        const NAME: &'static str = "J";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// SPSR\[23\]
    pub struct SSBS;
    impl RegisterView for SSBS {
        type Register = super::SPSR;
        const NAME: &'static str = "SSBS";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// SPSR\[22\]
    pub struct PAN;
    impl RegisterView for PAN {
        type Register = super::SPSR;
        const NAME: &'static str = "PAN";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// SPSR\[21\]
    pub struct DIT;
    impl RegisterView for DIT {
        type Register = super::SPSR;
        const NAME: &'static str = "DIT";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// SPSR\[20\]
    pub struct IL;
    impl RegisterView for IL {
        type Register = super::SPSR;
        const NAME: &'static str = "IL";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// SPSR\[19:16\]
    pub struct GE;
    impl RegisterView for GE {
        type Register = super::SPSR;
        const NAME: &'static str = "GE";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// SPSR\[9\]
    pub struct E;
    impl RegisterView for E {
        type Register = super::SPSR;
        const NAME: &'static str = "E";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// SPSR\[8\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::SPSR;
        const NAME: &'static str = "A";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// SPSR\[7\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::SPSR;
        const NAME: &'static str = "I";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// SPSR\[6\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::SPSR;
        const NAME: &'static str = "F";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// SPSR\[5\]
    pub struct T;
    impl RegisterView for T {
        type Register = super::SPSR;
        const NAME: &'static str = "T";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// SPSR\[4:0\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::SPSR;
        const NAME: &'static str = "M";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Vector Base Address Register
pub struct VBAR;
impl Register for VBAR {
    const NAME: &'static str = "VBAR";
    const LEN: usize = 64;
}

/// VBAR register fields
pub mod vbar {
    use crate::core::model::proc::RegisterView;

    /// VBAR\[31:5\]
    pub struct VBA;
    impl RegisterView for VBA {
        type Register = super::VBAR;
        const NAME: &'static str = "VBA";
        const OFFSET: usize = 5;
        const LEN: usize = 27;
    }
}

/// DCC Interrupt Enable Register
pub struct DBGDCCINT;
impl Register for DBGDCCINT {
    const NAME: &'static str = "DBGDCCINT";
    const LEN: usize = 64;
}

/// DBGDCCINT register fields
pub mod dbgdccint {
    use crate::core::model::proc::RegisterView;

    /// DBGDCCINT\[30\]
    pub struct RX;
    impl RegisterView for RX {
        type Register = super::DBGDCCINT;
        const NAME: &'static str = "RX";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// DBGDCCINT\[29\]
    pub struct TX;
    impl RegisterView for TX {
        type Register = super::DBGDCCINT;
        const NAME: &'static str = "TX";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }
}

/// Memory Model Feature Register 2
#[allow(non_camel_case_types)]
pub struct ID_MMFR2;
impl Register for ID_MMFR2 {
    const NAME: &'static str = "ID_MMFR2";
    const LEN: usize = 64;
}

/// ID_MMFR2 register fields
pub mod id_mmfr2 {
    use crate::core::model::proc::RegisterView;

    /// ID_MMFR2\[31:28\]
    pub struct HWAccFlg;
    impl RegisterView for HWAccFlg {
        type Register = super::ID_MMFR2;
        const NAME: &'static str = "HWAccFlg";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_MMFR2\[27:24\]
    pub struct WFIStall;
    impl RegisterView for WFIStall {
        type Register = super::ID_MMFR2;
        const NAME: &'static str = "WFIStall";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_MMFR2\[23:20\]
    pub struct MemBarr;
    impl RegisterView for MemBarr {
        type Register = super::ID_MMFR2;
        const NAME: &'static str = "MemBarr";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_MMFR2\[19:16\]
    pub struct UniTLB;
    impl RegisterView for UniTLB {
        type Register = super::ID_MMFR2;
        const NAME: &'static str = "UniTLB";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_MMFR2\[15:12\]
    pub struct HvdTLB;
    impl RegisterView for HvdTLB {
        type Register = super::ID_MMFR2;
        const NAME: &'static str = "HvdTLB";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_MMFR2\[11:8\]
    pub struct L1HvdRng;
    impl RegisterView for L1HvdRng {
        type Register = super::ID_MMFR2;
        const NAME: &'static str = "L1HvdRng";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_MMFR2\[7:4\]
    pub struct L1HvdBG;
    impl RegisterView for L1HvdBG {
        type Register = super::ID_MMFR2;
        const NAME: &'static str = "L1HvdBG";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_MMFR2\[3:0\]
    pub struct L1HvdFG;
    impl RegisterView for L1HvdFG {
        type Register = super::ID_MMFR2;
        const NAME: &'static str = "L1HvdFG";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Activity Monitors User Enable Register
pub struct AMUSERENR;
impl Register for AMUSERENR {
    const NAME: &'static str = "AMUSERENR";
    const LEN: usize = 64;
}

/// AMUSERENR register fields
pub mod amuserenr {
    use crate::core::model::proc::RegisterView;

    /// AMUSERENR\[0\]
    pub struct EN;
    impl RegisterView for EN {
        type Register = super::AMUSERENR;
        const NAME: &'static str = "EN";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Memory Model Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_MMFR0;
impl Register for ID_MMFR0 {
    const NAME: &'static str = "ID_MMFR0";
    const LEN: usize = 64;
}

/// ID_MMFR0 register fields
pub mod id_mmfr0 {
    use crate::core::model::proc::RegisterView;

    /// ID_MMFR0\[31:28\]
    pub struct InnerShr;
    impl RegisterView for InnerShr {
        type Register = super::ID_MMFR0;
        const NAME: &'static str = "InnerShr";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_MMFR0\[27:24\]
    pub struct FCSE;
    impl RegisterView for FCSE {
        type Register = super::ID_MMFR0;
        const NAME: &'static str = "FCSE";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_MMFR0\[23:20\]
    pub struct AuxReg;
    impl RegisterView for AuxReg {
        type Register = super::ID_MMFR0;
        const NAME: &'static str = "AuxReg";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_MMFR0\[19:16\]
    pub struct TCM;
    impl RegisterView for TCM {
        type Register = super::ID_MMFR0;
        const NAME: &'static str = "TCM";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_MMFR0\[15:12\]
    pub struct ShareLvl;
    impl RegisterView for ShareLvl {
        type Register = super::ID_MMFR0;
        const NAME: &'static str = "ShareLvl";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_MMFR0\[11:8\]
    pub struct OuterShr;
    impl RegisterView for OuterShr {
        type Register = super::ID_MMFR0;
        const NAME: &'static str = "OuterShr";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_MMFR0\[7:4\]
    pub struct PMSA;
    impl RegisterView for PMSA {
        type Register = super::ID_MMFR0;
        const NAME: &'static str = "PMSA";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_MMFR0\[3:0\]
    pub struct VMSA;
    impl RegisterView for VMSA {
        type Register = super::ID_MMFR0;
        const NAME: &'static str = "VMSA";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Virtualization Multiprocessor ID Register
pub struct VMPIDR;
impl Register for VMPIDR {
    const NAME: &'static str = "VMPIDR";
    const LEN: usize = 64;
}

/// VMPIDR register fields
pub mod vmpidr {
    use crate::core::model::proc::RegisterView;

    /// VMPIDR\[31\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::VMPIDR;
        const NAME: &'static str = "M";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// VMPIDR\[30\]
    pub struct U;
    impl RegisterView for U {
        type Register = super::VMPIDR;
        const NAME: &'static str = "U";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// VMPIDR\[24\]
    pub struct MT;
    impl RegisterView for MT {
        type Register = super::VMPIDR;
        const NAME: &'static str = "MT";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// VMPIDR\[23:16\]
    pub struct Aff2;
    impl RegisterView for Aff2 {
        type Register = super::VMPIDR;
        const NAME: &'static str = "Aff2";
        const OFFSET: usize = 16;
        const LEN: usize = 8;
    }

    /// VMPIDR\[15:8\]
    pub struct Aff1;
    impl RegisterView for Aff1 {
        type Register = super::VMPIDR;
        const NAME: &'static str = "Aff1";
        const OFFSET: usize = 8;
        const LEN: usize = 8;
    }

    /// VMPIDR\[7:0\]
    pub struct Aff0;
    impl RegisterView for Aff0 {
        type Register = super::VMPIDR;
        const NAME: &'static str = "Aff0";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Interrupt Controller End Of Interrupt Register 0
#[allow(non_camel_case_types)]
pub struct ICC_EOIR0;
impl Register for ICC_EOIR0 {
    const NAME: &'static str = "ICC_EOIR0";
    const LEN: usize = 64;
}

/// ICC_EOIR0 register fields
pub mod icc_eoir0 {
    use crate::core::model::proc::RegisterView;

    /// ICC_EOIR0\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_EOIR0;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Secure Debug Control Register
pub struct SDCR;
impl Register for SDCR {
    const NAME: &'static str = "SDCR";
    const LEN: usize = 64;
}

/// SDCR register fields
pub mod sdcr {
    use crate::core::model::proc::RegisterView;

    /// SDCR\[28\]
    pub struct MTPME;
    impl RegisterView for MTPME {
        type Register = super::SDCR;
        const NAME: &'static str = "MTPME";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// SDCR\[27\]
    pub struct TDCC;
    impl RegisterView for TDCC {
        type Register = super::SDCR;
        const NAME: &'static str = "TDCC";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// SDCR\[23\]
    pub struct SCCD;
    impl RegisterView for SCCD {
        type Register = super::SDCR;
        const NAME: &'static str = "SCCD";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// SDCR\[21\]
    pub struct EPMAD;
    impl RegisterView for EPMAD {
        type Register = super::SDCR;
        const NAME: &'static str = "EPMAD";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// SDCR\[20\]
    pub struct EDAD;
    impl RegisterView for EDAD {
        type Register = super::SDCR;
        const NAME: &'static str = "EDAD";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// SDCR\[19\]
    pub struct TTRF;
    impl RegisterView for TTRF {
        type Register = super::SDCR;
        const NAME: &'static str = "TTRF";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// SDCR\[18\]
    pub struct STE;
    impl RegisterView for STE {
        type Register = super::SDCR;
        const NAME: &'static str = "STE";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// SDCR\[17\]
    pub struct SPME;
    impl RegisterView for SPME {
        type Register = super::SDCR;
        const NAME: &'static str = "SPME";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// SDCR\[15:14\]
    pub struct SPD;
    impl RegisterView for SPD {
        type Register = super::SDCR;
        const NAME: &'static str = "SPD";
        const OFFSET: usize = 14;
        const LEN: usize = 2;
    }
}

/// Counter-timer Virtual Timer Control register
#[allow(non_camel_case_types)]
pub struct CNTV_CTL;
impl Register for CNTV_CTL {
    const NAME: &'static str = "CNTV_CTL";
    const LEN: usize = 64;
}

/// CNTV_CTL register fields
pub mod cntv_ctl {
    use crate::core::model::proc::RegisterView;

    /// CNTV_CTL\[2\]
    pub struct ISTATUS;
    impl RegisterView for ISTATUS {
        type Register = super::CNTV_CTL;
        const NAME: &'static str = "ISTATUS";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// CNTV_CTL\[1\]
    pub struct IMASK;
    impl RegisterView for IMASK {
        type Register = super::CNTV_CTL;
        const NAME: &'static str = "IMASK";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// CNTV_CTL\[0\]
    pub struct ENABLE;
    impl RegisterView for ENABLE {
        type Register = super::CNTV_CTL;
        const NAME: &'static str = "ENABLE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Virtual End Of Interrupt Register 1
#[allow(non_camel_case_types)]
pub struct ICV_EOIR1;
impl Register for ICV_EOIR1 {
    const NAME: &'static str = "ICV_EOIR1";
    const LEN: usize = 64;
}

/// ICV_EOIR1 register fields
pub mod icv_eoir1 {
    use crate::core::model::proc::RegisterView;

    /// ICV_EOIR1\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICV_EOIR1;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Jazelle OS Control Register
pub struct JOSCR;
impl Register for JOSCR {
    const NAME: &'static str = "JOSCR";
    const LEN: usize = 64;
}

/// Jazelle Main Configuration Register
pub struct JMCR;
impl Register for JMCR {
    const NAME: &'static str = "JMCR";
    const LEN: usize = 64;
}

/// Debug Vector Catch Register
pub struct DBGVCR;
impl Register for DBGVCR {
    const NAME: &'static str = "DBGVCR";
    const LEN: usize = 64;
}

/// DBGVCR register fields
pub mod dbgvcr {
    use crate::core::model::proc::RegisterView;

    /// DBGVCR\[31\]
    pub struct NSF;
    impl RegisterView for NSF {
        type Register = super::DBGVCR;
        const NAME: &'static str = "NSF";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// DBGVCR\[30\]
    pub struct NSI;
    impl RegisterView for NSI {
        type Register = super::DBGVCR;
        const NAME: &'static str = "NSI";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// DBGVCR\[28\]
    pub struct NSD;
    impl RegisterView for NSD {
        type Register = super::DBGVCR;
        const NAME: &'static str = "NSD";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// DBGVCR\[27\]
    pub struct NSP;
    impl RegisterView for NSP {
        type Register = super::DBGVCR;
        const NAME: &'static str = "NSP";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// DBGVCR\[26\]
    pub struct NSS;
    impl RegisterView for NSS {
        type Register = super::DBGVCR;
        const NAME: &'static str = "NSS";
        const OFFSET: usize = 26;
        const LEN: usize = 1;
    }

    /// DBGVCR\[25\]
    pub struct NSU;
    impl RegisterView for NSU {
        type Register = super::DBGVCR;
        const NAME: &'static str = "NSU";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// DBGVCR\[15\]
    pub struct MF;
    impl RegisterView for MF {
        type Register = super::DBGVCR;
        const NAME: &'static str = "MF";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// DBGVCR\[14\]
    pub struct MI;
    impl RegisterView for MI {
        type Register = super::DBGVCR;
        const NAME: &'static str = "MI";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// DBGVCR\[12\]
    pub struct MD;
    impl RegisterView for MD {
        type Register = super::DBGVCR;
        const NAME: &'static str = "MD";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// DBGVCR\[11\]
    pub struct MP;
    impl RegisterView for MP {
        type Register = super::DBGVCR;
        const NAME: &'static str = "MP";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// DBGVCR\[10\]
    pub struct MS;
    impl RegisterView for MS {
        type Register = super::DBGVCR;
        const NAME: &'static str = "MS";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }

    /// DBGVCR\[7\]
    pub struct SF;
    impl RegisterView for SF {
        type Register = super::DBGVCR;
        const NAME: &'static str = "SF";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// DBGVCR\[6\]
    pub struct SI;
    impl RegisterView for SI {
        type Register = super::DBGVCR;
        const NAME: &'static str = "SI";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// DBGVCR\[4\]
    pub struct SD;
    impl RegisterView for SD {
        type Register = super::DBGVCR;
        const NAME: &'static str = "SD";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// DBGVCR\[3\]
    pub struct SP;
    impl RegisterView for SP {
        type Register = super::DBGVCR;
        const NAME: &'static str = "SP";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// DBGVCR\[2\]
    pub struct SS;
    impl RegisterView for SS {
        type Register = super::DBGVCR;
        const NAME: &'static str = "SS";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// DBGVCR\[1\]
    pub struct SU;
    impl RegisterView for SU {
        type Register = super::DBGVCR;
        const NAME: &'static str = "SU";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// DBGVCR\[7\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::DBGVCR;
        const NAME: &'static str = "F";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// DBGVCR\[6\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::DBGVCR;
        const NAME: &'static str = "I";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// DBGVCR\[4\]
    pub struct D;
    impl RegisterView for D {
        type Register = super::DBGVCR;
        const NAME: &'static str = "D";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// DBGVCR\[3\]
    pub struct P;
    impl RegisterView for P {
        type Register = super::DBGVCR;
        const NAME: &'static str = "P";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// DBGVCR\[2\]
    pub struct S;
    impl RegisterView for S {
        type Register = super::DBGVCR;
        const NAME: &'static str = "S";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// DBGVCR\[1\]
    pub struct U;
    impl RegisterView for U {
        type Register = super::DBGVCR;
        const NAME: &'static str = "U";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller VGIC Type Register
#[allow(non_camel_case_types)]
pub struct ICH_VTR;
impl Register for ICH_VTR {
    const NAME: &'static str = "ICH_VTR";
    const LEN: usize = 64;
}

/// ICH_VTR register fields
pub mod ich_vtr {
    use crate::core::model::proc::RegisterView;

    /// ICH_VTR\[31:29\]
    pub struct PRIbits;
    impl RegisterView for PRIbits {
        type Register = super::ICH_VTR;
        const NAME: &'static str = "PRIbits";
        const OFFSET: usize = 29;
        const LEN: usize = 3;
    }

    /// ICH_VTR\[28:26\]
    pub struct PREbits;
    impl RegisterView for PREbits {
        type Register = super::ICH_VTR;
        const NAME: &'static str = "PREbits";
        const OFFSET: usize = 26;
        const LEN: usize = 3;
    }

    /// ICH_VTR\[25:23\]
    pub struct IDbits;
    impl RegisterView for IDbits {
        type Register = super::ICH_VTR;
        const NAME: &'static str = "IDbits";
        const OFFSET: usize = 23;
        const LEN: usize = 3;
    }

    /// ICH_VTR\[22\]
    pub struct SEIS;
    impl RegisterView for SEIS {
        type Register = super::ICH_VTR;
        const NAME: &'static str = "SEIS";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// ICH_VTR\[21\]
    pub struct A3V;
    impl RegisterView for A3V {
        type Register = super::ICH_VTR;
        const NAME: &'static str = "A3V";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// ICH_VTR\[20\]
    #[allow(non_camel_case_types)]
    pub struct nV4;
    impl RegisterView for nV4 {
        type Register = super::ICH_VTR;
        const NAME: &'static str = "nV4";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// ICH_VTR\[19\]
    pub struct TDS;
    impl RegisterView for TDS {
        type Register = super::ICH_VTR;
        const NAME: &'static str = "TDS";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// ICH_VTR\[4:0\]
    pub struct ListRegs;
    impl RegisterView for ListRegs {
        type Register = super::ICH_VTR;
        const NAME: &'static str = "ListRegs";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Translation Table Base Control Register
pub struct TTBCR;
impl Register for TTBCR {
    const NAME: &'static str = "TTBCR";
    const LEN: usize = 64;
}

/// TTBCR register fields
pub mod ttbcr {
    use crate::core::model::proc::RegisterView;

    /// TTBCR\[31\]
    pub struct EAE;
    impl RegisterView for EAE {
        type Register = super::TTBCR;
        const NAME: &'static str = "EAE";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// TTBCR\[5\]
    pub struct PD1;
    impl RegisterView for PD1 {
        type Register = super::TTBCR;
        const NAME: &'static str = "PD1";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// TTBCR\[4\]
    pub struct PD0;
    impl RegisterView for PD0 {
        type Register = super::TTBCR;
        const NAME: &'static str = "PD0";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// TTBCR\[2:0\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::TTBCR;
        const NAME: &'static str = "N";
        const OFFSET: usize = 0;
        const LEN: usize = 3;
    }

    /// TTBCR\[29:28\]
    pub struct SH1;
    impl RegisterView for SH1 {
        type Register = super::TTBCR;
        const NAME: &'static str = "SH1";
        const OFFSET: usize = 28;
        const LEN: usize = 2;
    }

    /// TTBCR\[27:26\]
    pub struct ORGN1;
    impl RegisterView for ORGN1 {
        type Register = super::TTBCR;
        const NAME: &'static str = "ORGN1";
        const OFFSET: usize = 26;
        const LEN: usize = 2;
    }

    /// TTBCR\[25:24\]
    pub struct IRGN1;
    impl RegisterView for IRGN1 {
        type Register = super::TTBCR;
        const NAME: &'static str = "IRGN1";
        const OFFSET: usize = 24;
        const LEN: usize = 2;
    }

    /// TTBCR\[23\]
    pub struct EPD1;
    impl RegisterView for EPD1 {
        type Register = super::TTBCR;
        const NAME: &'static str = "EPD1";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// TTBCR\[22\]
    pub struct A1;
    impl RegisterView for A1 {
        type Register = super::TTBCR;
        const NAME: &'static str = "A1";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// TTBCR\[18:16\]
    pub struct T1SZ;
    impl RegisterView for T1SZ {
        type Register = super::TTBCR;
        const NAME: &'static str = "T1SZ";
        const OFFSET: usize = 16;
        const LEN: usize = 3;
    }

    /// TTBCR\[13:12\]
    pub struct SH0;
    impl RegisterView for SH0 {
        type Register = super::TTBCR;
        const NAME: &'static str = "SH0";
        const OFFSET: usize = 12;
        const LEN: usize = 2;
    }

    /// TTBCR\[11:10\]
    pub struct ORGN0;
    impl RegisterView for ORGN0 {
        type Register = super::TTBCR;
        const NAME: &'static str = "ORGN0";
        const OFFSET: usize = 10;
        const LEN: usize = 2;
    }

    /// TTBCR\[9:8\]
    pub struct IRGN0;
    impl RegisterView for IRGN0 {
        type Register = super::TTBCR;
        const NAME: &'static str = "IRGN0";
        const OFFSET: usize = 8;
        const LEN: usize = 2;
    }

    /// TTBCR\[7\]
    pub struct EPD0;
    impl RegisterView for EPD0 {
        type Register = super::TTBCR;
        const NAME: &'static str = "EPD0";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// TTBCR\[6\]
    pub struct T2E;
    impl RegisterView for T2E {
        type Register = super::TTBCR;
        const NAME: &'static str = "T2E";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// TTBCR\[2:0\]
    pub struct T0SZ;
    impl RegisterView for T0SZ {
        type Register = super::TTBCR;
        const NAME: &'static str = "T0SZ";
        const OFFSET: usize = 0;
        const LEN: usize = 3;
    }
}

/// Current Cache Size ID Register
pub struct CCSIDR;
impl Register for CCSIDR {
    const NAME: &'static str = "CCSIDR";
    const LEN: usize = 64;
}

/// CCSIDR register fields
pub mod ccsidr {
    use crate::core::model::proc::RegisterView;

    /// CCSIDR\[12:3\]
    pub struct Associativity;
    impl RegisterView for Associativity {
        type Register = super::CCSIDR;
        const NAME: &'static str = "Associativity";
        const OFFSET: usize = 3;
        const LEN: usize = 10;
    }

    /// CCSIDR\[2:0\]
    pub struct LineSize;
    impl RegisterView for LineSize {
        type Register = super::CCSIDR;
        const NAME: &'static str = "LineSize";
        const OFFSET: usize = 0;
        const LEN: usize = 3;
    }

    /// CCSIDR\[27:13\]
    pub struct NumSets;
    impl RegisterView for NumSets {
        type Register = super::CCSIDR;
        const NAME: &'static str = "NumSets";
        const OFFSET: usize = 13;
        const LEN: usize = 15;
    }
}

/// Debug Saved Program Status Register
pub struct DSPSR;
impl Register for DSPSR {
    const NAME: &'static str = "DSPSR";
    const LEN: usize = 64;
}

/// DSPSR register fields
pub mod dspsr {
    use crate::core::model::proc::RegisterView;

    /// DSPSR\[31\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::DSPSR;
        const NAME: &'static str = "N";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// DSPSR\[30\]
    pub struct Z;
    impl RegisterView for Z {
        type Register = super::DSPSR;
        const NAME: &'static str = "Z";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// DSPSR\[29\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::DSPSR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// DSPSR\[28\]
    pub struct V;
    impl RegisterView for V {
        type Register = super::DSPSR;
        const NAME: &'static str = "V";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// DSPSR\[27\]
    pub struct Q;
    impl RegisterView for Q {
        type Register = super::DSPSR;
        const NAME: &'static str = "Q";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// DSPSR\[15:10\]
    pub struct IT;
    impl RegisterView for IT {
        type Register = super::DSPSR;
        const NAME: &'static str = "IT";
        const OFFSET: usize = 10;
        const LEN: usize = 6;
    }

    /// DSPSR\[24\]
    pub struct DIT;
    impl RegisterView for DIT {
        type Register = super::DSPSR;
        const NAME: &'static str = "DIT";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// DSPSR\[23\]
    pub struct SSBS;
    impl RegisterView for SSBS {
        type Register = super::DSPSR;
        const NAME: &'static str = "SSBS";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// DSPSR\[22\]
    pub struct PAN;
    impl RegisterView for PAN {
        type Register = super::DSPSR;
        const NAME: &'static str = "PAN";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// DSPSR\[21\]
    pub struct SS;
    impl RegisterView for SS {
        type Register = super::DSPSR;
        const NAME: &'static str = "SS";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// DSPSR\[20\]
    pub struct IL;
    impl RegisterView for IL {
        type Register = super::DSPSR;
        const NAME: &'static str = "IL";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// DSPSR\[19:16\]
    pub struct GE;
    impl RegisterView for GE {
        type Register = super::DSPSR;
        const NAME: &'static str = "GE";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// DSPSR\[9\]
    pub struct E;
    impl RegisterView for E {
        type Register = super::DSPSR;
        const NAME: &'static str = "E";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// DSPSR\[8\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::DSPSR;
        const NAME: &'static str = "A";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// DSPSR\[7\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::DSPSR;
        const NAME: &'static str = "I";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// DSPSR\[6\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::DSPSR;
        const NAME: &'static str = "F";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// DSPSR\[5\]
    pub struct T;
    impl RegisterView for T {
        type Register = super::DSPSR;
        const NAME: &'static str = "T";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// DSPSR\[4:0\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::DSPSR;
        const NAME: &'static str = "M";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Interrupt Controller Virtual End Of Interrupt Register 0
#[allow(non_camel_case_types)]
pub struct ICV_EOIR0;
impl Register for ICV_EOIR0 {
    const NAME: &'static str = "ICV_EOIR0";
    const LEN: usize = 64;
}

/// ICV_EOIR0 register fields
pub mod icv_eoir0 {
    use crate::core::model::proc::RegisterView;

    /// ICV_EOIR0\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICV_EOIR0;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Interrupt Controller Software Generated Interrupt Group 1 Register
#[allow(non_camel_case_types)]
pub struct ICC_SGI1R;
impl Register for ICC_SGI1R {
    const NAME: &'static str = "ICC_SGI1R";
    const LEN: usize = 64;
}

/// ICC_SGI1R register fields
pub mod icc_sgi1r {
    use crate::core::model::proc::RegisterView;

    /// ICC_SGI1R\[55:48\]
    pub struct Aff3;
    impl RegisterView for Aff3 {
        type Register = super::ICC_SGI1R;
        const NAME: &'static str = "Aff3";
        const OFFSET: usize = 48;
        const LEN: usize = 8;
    }

    /// ICC_SGI1R\[47:44\]
    pub struct RS;
    impl RegisterView for RS {
        type Register = super::ICC_SGI1R;
        const NAME: &'static str = "RS";
        const OFFSET: usize = 44;
        const LEN: usize = 4;
    }

    /// ICC_SGI1R\[40\]
    pub struct IRM;
    impl RegisterView for IRM {
        type Register = super::ICC_SGI1R;
        const NAME: &'static str = "IRM";
        const OFFSET: usize = 40;
        const LEN: usize = 1;
    }

    /// ICC_SGI1R\[39:32\]
    pub struct Aff2;
    impl RegisterView for Aff2 {
        type Register = super::ICC_SGI1R;
        const NAME: &'static str = "Aff2";
        const OFFSET: usize = 32;
        const LEN: usize = 8;
    }

    /// ICC_SGI1R\[27:24\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_SGI1R;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ICC_SGI1R\[23:16\]
    pub struct Aff1;
    impl RegisterView for Aff1 {
        type Register = super::ICC_SGI1R;
        const NAME: &'static str = "Aff1";
        const OFFSET: usize = 16;
        const LEN: usize = 8;
    }

    /// ICC_SGI1R\[15:0\]
    pub struct TargetList;
    impl RegisterView for TargetList {
        type Register = super::ICC_SGI1R;
        const NAME: &'static str = "TargetList";
        const OFFSET: usize = 0;
        const LEN: usize = 16;
    }
}

/// Interrupt Controller Control Register
#[allow(non_camel_case_types)]
pub struct ICC_CTLR;
impl Register for ICC_CTLR {
    const NAME: &'static str = "ICC_CTLR";
    const LEN: usize = 64;
}

/// ICC_CTLR register fields
pub mod icc_ctlr {
    use crate::core::model::proc::RegisterView;

    /// ICC_CTLR\[19\]
    pub struct ExtRange;
    impl RegisterView for ExtRange {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "ExtRange";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// ICC_CTLR\[18\]
    pub struct RSS;
    impl RegisterView for RSS {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "RSS";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// ICC_CTLR\[15\]
    pub struct A3V;
    impl RegisterView for A3V {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "A3V";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// ICC_CTLR\[14\]
    pub struct SEIS;
    impl RegisterView for SEIS {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "SEIS";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// ICC_CTLR\[13:11\]
    pub struct IDbits;
    impl RegisterView for IDbits {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "IDbits";
        const OFFSET: usize = 11;
        const LEN: usize = 3;
    }

    /// ICC_CTLR\[10:8\]
    pub struct PRIbits;
    impl RegisterView for PRIbits {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "PRIbits";
        const OFFSET: usize = 8;
        const LEN: usize = 3;
    }

    /// ICC_CTLR\[6\]
    pub struct PMHE;
    impl RegisterView for PMHE {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "PMHE";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// ICC_CTLR\[1\]
    pub struct EOImode;
    impl RegisterView for EOImode {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "EOImode";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICC_CTLR\[0\]
    pub struct CBPR;
    impl RegisterView for CBPR {
        type Register = super::ICC_CTLR;
        const NAME: &'static str = "CBPR";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Counter-timer Virtual Timer CompareValue register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHV_CVAL;
impl Register for CNTHV_CVAL {
    const NAME: &'static str = "CNTHV_CVAL";
    const LEN: usize = 64;
}

/// CNTHV_CVAL register fields
pub mod cnthv_cval {
    use crate::core::model::proc::RegisterView;

    /// CNTHV_CVAL\[63:0\]
    pub struct CompareValue;
    impl RegisterView for CompareValue {
        type Register = super::CNTHV_CVAL;
        const NAME: &'static str = "CompareValue";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Interrupt Controller End Of Interrupt Register 1
#[allow(non_camel_case_types)]
pub struct ICC_EOIR1;
impl Register for ICC_EOIR1 {
    const NAME: &'static str = "ICC_EOIR1";
    const LEN: usize = 64;
}

/// ICC_EOIR1 register fields
pub mod icc_eoir1 {
    use crate::core::model::proc::RegisterView;

    /// ICC_EOIR1\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_EOIR1;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Counter-timer Secure Virtual Timer TimerValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHVS_TVAL;
impl Register for CNTHVS_TVAL {
    const NAME: &'static str = "CNTHVS_TVAL";
    const LEN: usize = 64;
}

/// CNTHVS_TVAL register fields
pub mod cnthvs_tval {
    use crate::core::model::proc::RegisterView;

    /// CNTHVS_TVAL\[31:0\]
    pub struct TimerValue;
    impl RegisterView for TimerValue {
        type Register = super::CNTHVS_TVAL;
        const NAME: &'static str = "TimerValue";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Memory Model Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_MMFR1;
impl Register for ID_MMFR1 {
    const NAME: &'static str = "ID_MMFR1";
    const LEN: usize = 64;
}

/// ID_MMFR1 register fields
pub mod id_mmfr1 {
    use crate::core::model::proc::RegisterView;

    /// ID_MMFR1\[31:28\]
    pub struct BPred;
    impl RegisterView for BPred {
        type Register = super::ID_MMFR1;
        const NAME: &'static str = "BPred";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_MMFR1\[27:24\]
    pub struct L1TstCln;
    impl RegisterView for L1TstCln {
        type Register = super::ID_MMFR1;
        const NAME: &'static str = "L1TstCln";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_MMFR1\[23:20\]
    pub struct L1Uni;
    impl RegisterView for L1Uni {
        type Register = super::ID_MMFR1;
        const NAME: &'static str = "L1Uni";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_MMFR1\[19:16\]
    pub struct L1Hvd;
    impl RegisterView for L1Hvd {
        type Register = super::ID_MMFR1;
        const NAME: &'static str = "L1Hvd";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_MMFR1\[15:12\]
    pub struct L1UniSW;
    impl RegisterView for L1UniSW {
        type Register = super::ID_MMFR1;
        const NAME: &'static str = "L1UniSW";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_MMFR1\[11:8\]
    pub struct L1HvdSW;
    impl RegisterView for L1HvdSW {
        type Register = super::ID_MMFR1;
        const NAME: &'static str = "L1HvdSW";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_MMFR1\[7:4\]
    pub struct L1UniVA;
    impl RegisterView for L1UniVA {
        type Register = super::ID_MMFR1;
        const NAME: &'static str = "L1UniVA";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_MMFR1\[3:0\]
    pub struct L1HvdVA;
    impl RegisterView for L1HvdVA {
        type Register = super::ID_MMFR1;
        const NAME: &'static str = "L1HvdVA";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Interrupt Controller Virtual Interrupt Acknowledge Register 1
#[allow(non_camel_case_types)]
pub struct ICV_IAR1;
impl Register for ICV_IAR1 {
    const NAME: &'static str = "ICV_IAR1";
    const LEN: usize = 64;
}

/// ICV_IAR1 register fields
pub mod icv_iar1 {
    use crate::core::model::proc::RegisterView;

    /// ICV_IAR1\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICV_IAR1;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 1
#[allow(non_camel_case_types)]
pub struct ICV_HPPIR1;
impl Register for ICV_HPPIR1 {
    const NAME: &'static str = "ICV_HPPIR1";
    const LEN: usize = 64;
}

/// ICV_HPPIR1 register fields
pub mod icv_hppir1 {
    use crate::core::model::proc::RegisterView;

    /// ICV_HPPIR1\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICV_HPPIR1;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Performance Monitors Count Enable Clear register
pub struct PMCNTENCLR;
impl Register for PMCNTENCLR {
    const NAME: &'static str = "PMCNTENCLR";
    const LEN: usize = 64;
}

/// PMCNTENCLR register fields
pub mod pmcntenclr {
    use crate::core::model::proc::RegisterView;

    /// PMCNTENCLR\[31\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::PMCNTENCLR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }
}

/// Instruction Set Attribute Register 2
#[allow(non_camel_case_types)]
pub struct ID_ISAR2;
impl Register for ID_ISAR2 {
    const NAME: &'static str = "ID_ISAR2";
    const LEN: usize = 64;
}

/// ID_ISAR2 register fields
pub mod id_isar2 {
    use crate::core::model::proc::RegisterView;

    /// ID_ISAR2\[31:28\]
    pub struct Reversal;
    impl RegisterView for Reversal {
        type Register = super::ID_ISAR2;
        const NAME: &'static str = "Reversal";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_ISAR2\[27:24\]
    #[allow(non_camel_case_types)]
    pub struct PSR_AR;
    impl RegisterView for PSR_AR {
        type Register = super::ID_ISAR2;
        const NAME: &'static str = "PSR_AR";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_ISAR2\[23:20\]
    pub struct MultU;
    impl RegisterView for MultU {
        type Register = super::ID_ISAR2;
        const NAME: &'static str = "MultU";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_ISAR2\[19:16\]
    pub struct MultS;
    impl RegisterView for MultS {
        type Register = super::ID_ISAR2;
        const NAME: &'static str = "MultS";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_ISAR2\[15:12\]
    pub struct Mult;
    impl RegisterView for Mult {
        type Register = super::ID_ISAR2;
        const NAME: &'static str = "Mult";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_ISAR2\[11:8\]
    pub struct MultiAccessInt;
    impl RegisterView for MultiAccessInt {
        type Register = super::ID_ISAR2;
        const NAME: &'static str = "MultiAccessInt";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_ISAR2\[7:4\]
    pub struct MemHint;
    impl RegisterView for MemHint {
        type Register = super::ID_ISAR2;
        const NAME: &'static str = "MemHint";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_ISAR2\[3:0\]
    pub struct LoadStore;
    impl RegisterView for LoadStore {
        type Register = super::ID_ISAR2;
        const NAME: &'static str = "LoadStore";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Performance Monitors Software Increment register
pub struct PMSWINC;
impl Register for PMSWINC {
    const NAME: &'static str = "PMSWINC";
    const LEN: usize = 64;
}

/// Interrupt Controller Virtual Interrupt Group 1 Enable register
#[allow(non_camel_case_types)]
pub struct ICV_IGRPEN1;
impl Register for ICV_IGRPEN1 {
    const NAME: &'static str = "ICV_IGRPEN1";
    const LEN: usize = 64;
}

/// ICV_IGRPEN1 register fields
pub mod icv_igrpen1 {
    use crate::core::model::proc::RegisterView;

    /// ICV_IGRPEN1\[0\]
    pub struct Enable;
    impl RegisterView for Enable {
        type Register = super::ICV_IGRPEN1;
        const NAME: &'static str = "Enable";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Virtual Interrupt Group 0 Enable register
#[allow(non_camel_case_types)]
pub struct ICV_IGRPEN0;
impl Register for ICV_IGRPEN0 {
    const NAME: &'static str = "ICV_IGRPEN0";
    const LEN: usize = 64;
}

/// ICV_IGRPEN0 register fields
pub mod icv_igrpen0 {
    use crate::core::model::proc::RegisterView;

    /// ICV_IGRPEN0\[0\]
    pub struct Enable;
    impl RegisterView for Enable {
        type Register = super::ICV_IGRPEN0;
        const NAME: &'static str = "Enable";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Normal Memory Remap Register
pub struct NMRR;
impl Register for NMRR {
    const NAME: &'static str = "NMRR";
    const LEN: usize = 64;
}

/// Virtualization Translation Table Base Register
pub struct VTTBR;
impl Register for VTTBR {
    const NAME: &'static str = "VTTBR";
    const LEN: usize = 64;
}

/// VTTBR register fields
pub mod vttbr {
    use crate::core::model::proc::RegisterView;

    /// VTTBR\[55:48\]
    pub struct VMID;
    impl RegisterView for VMID {
        type Register = super::VTTBR;
        const NAME: &'static str = "VMID";
        const OFFSET: usize = 48;
        const LEN: usize = 8;
    }

    /// VTTBR\[47:1\]
    pub struct BADDR;
    impl RegisterView for BADDR {
        type Register = super::VTTBR;
        const NAME: &'static str = "BADDR";
        const OFFSET: usize = 1;
        const LEN: usize = 47;
    }

    /// VTTBR\[0\]
    pub struct CnP;
    impl RegisterView for CnP {
        type Register = super::VTTBR;
        const NAME: &'static str = "CnP";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Processor Feature Register 2
#[allow(non_camel_case_types)]
pub struct ID_PFR2;
impl Register for ID_PFR2 {
    const NAME: &'static str = "ID_PFR2";
    const LEN: usize = 64;
}

/// ID_PFR2 register fields
pub mod id_pfr2 {
    use crate::core::model::proc::RegisterView;

    /// ID_PFR2\[11:8\]
    #[allow(non_camel_case_types)]
    pub struct RAS_frac;
    impl RegisterView for RAS_frac {
        type Register = super::ID_PFR2;
        const NAME: &'static str = "RAS_frac";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_PFR2\[7:4\]
    pub struct SSBS;
    impl RegisterView for SSBS {
        type Register = super::ID_PFR2;
        const NAME: &'static str = "SSBS";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_PFR2\[3:0\]
    pub struct CSV3;
    impl RegisterView for CSV3 {
        type Register = super::ID_PFR2;
        const NAME: &'static str = "CSV3";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Instruction Set Attribute Register 3
#[allow(non_camel_case_types)]
pub struct ID_ISAR3;
impl Register for ID_ISAR3 {
    const NAME: &'static str = "ID_ISAR3";
    const LEN: usize = 64;
}

/// ID_ISAR3 register fields
pub mod id_isar3 {
    use crate::core::model::proc::RegisterView;

    /// ID_ISAR3\[31:28\]
    pub struct T32EE;
    impl RegisterView for T32EE {
        type Register = super::ID_ISAR3;
        const NAME: &'static str = "T32EE";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_ISAR3\[27:24\]
    pub struct TrueNOP;
    impl RegisterView for TrueNOP {
        type Register = super::ID_ISAR3;
        const NAME: &'static str = "TrueNOP";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_ISAR3\[23:20\]
    pub struct T32Copy;
    impl RegisterView for T32Copy {
        type Register = super::ID_ISAR3;
        const NAME: &'static str = "T32Copy";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_ISAR3\[19:16\]
    pub struct TabBranch;
    impl RegisterView for TabBranch {
        type Register = super::ID_ISAR3;
        const NAME: &'static str = "TabBranch";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_ISAR3\[15:12\]
    pub struct SynchPrim;
    impl RegisterView for SynchPrim {
        type Register = super::ID_ISAR3;
        const NAME: &'static str = "SynchPrim";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_ISAR3\[11:8\]
    pub struct SVC;
    impl RegisterView for SVC {
        type Register = super::ID_ISAR3;
        const NAME: &'static str = "SVC";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_ISAR3\[7:4\]
    pub struct SIMD;
    impl RegisterView for SIMD {
        type Register = super::ID_ISAR3;
        const NAME: &'static str = "SIMD";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_ISAR3\[3:0\]
    pub struct Saturate;
    impl RegisterView for Saturate {
        type Register = super::ID_ISAR3;
        const NAME: &'static str = "Saturate";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0
#[allow(non_camel_case_types)]
pub struct ICV_HPPIR0;
impl Register for ICV_HPPIR0 {
    const NAME: &'static str = "ICV_HPPIR0";
    const LEN: usize = 64;
}

/// ICV_HPPIR0 register fields
pub mod icv_hppir0 {
    use crate::core::model::proc::RegisterView;

    /// ICV_HPPIR0\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICV_HPPIR0;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Performance Monitors Event Type Registers, n = 30 - 0
pub struct PMEVTYPERn;
impl RegisterArray for PMEVTYPERn {
    const NAME: &'static str = "PMEVTYPERn";
    const ELEMS: usize = 30;
    const ELEM_LEN: usize = 64;
}

/// PMEVTYPERn register fields
pub mod pmevtypern {
    use crate::core::model::proc::RegisterArrayView;

    /// PMEVTYPERn\[31\]
    pub struct P;
    impl RegisterArrayView for P {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "P";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// PMEVTYPERn\[30\]
    pub struct U;
    impl RegisterArrayView for U {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "U";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// PMEVTYPERn\[29\]
    pub struct NSK;
    impl RegisterArrayView for NSK {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "NSK";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// PMEVTYPERn\[28\]
    pub struct NSU;
    impl RegisterArrayView for NSU {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "NSU";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// PMEVTYPERn\[27\]
    pub struct NSH;
    impl RegisterArrayView for NSH {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "NSH";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// PMEVTYPERn\[25\]
    pub struct MT;
    impl RegisterArrayView for MT {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "MT";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// PMEVTYPERn\[21\]
    pub struct RLU;
    impl RegisterArrayView for RLU {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "RLU";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// PMEVTYPERn\[15:10\]
    #[allow(non_camel_case_types)]
    pub struct evtCount_15_10;
    impl RegisterArrayView for evtCount_15_10 {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "evtCount_15_10";
        const OFFSET: usize = 10;
        const LEN: usize = 6;
    }
    /// PMEVTYPERn\[9:0\]
    #[allow(non_camel_case_types)]
    pub struct evtCount_15_10_9_0;
    impl RegisterArrayView for evtCount_15_10_9_0 {
        type RegisterArray = super::PMEVTYPERn;
        const NAME: &'static str = "evtCount_15_10_9_0";
        const OFFSET: usize = 0;
        const LEN: usize = 10;
    }
}

/// Counter-timer Virtual Offset register
pub struct CNTVOFF;
impl Register for CNTVOFF {
    const NAME: &'static str = "CNTVOFF";
    const LEN: usize = 64;
}

/// Auxiliary Data Fault Status Register
pub struct ADFSR;
impl Register for ADFSR {
    const NAME: &'static str = "ADFSR";
    const LEN: usize = 64;
}

/// Interrupt Controller Virtual Interrupt Acknowledge Register 0
#[allow(non_camel_case_types)]
pub struct ICV_IAR0;
impl Register for ICV_IAR0 {
    const NAME: &'static str = "ICV_IAR0";
    const LEN: usize = 64;
}

/// ICV_IAR0 register fields
pub mod icv_iar0 {
    use crate::core::model::proc::RegisterView;

    /// ICV_IAR0\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICV_IAR0;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Debug Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_DFR1;
impl Register for ID_DFR1 {
    const NAME: &'static str = "ID_DFR1";
    const LEN: usize = 64;
}

/// ID_DFR1 register fields
pub mod id_dfr1 {
    use crate::core::model::proc::RegisterView;

    /// ID_DFR1\[7:4\]
    pub struct HPMN0;
    impl RegisterView for HPMN0 {
        type Register = super::ID_DFR1;
        const NAME: &'static str = "HPMN0";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_DFR1\[3:0\]
    pub struct MTPMU;
    impl RegisterView for MTPMU {
        type Register = super::ID_DFR1;
        const NAME: &'static str = "MTPMU";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Interrupt Controller Virtual Binary Point Register 0
#[allow(non_camel_case_types)]
pub struct ICV_BPR0;
impl Register for ICV_BPR0 {
    const NAME: &'static str = "ICV_BPR0";
    const LEN: usize = 64;
}

/// ICV_BPR0 register fields
pub mod icv_bpr0 {
    use crate::core::model::proc::RegisterView;

    /// ICV_BPR0\[2:0\]
    pub struct BinaryPoint;
    impl RegisterView for BinaryPoint {
        type Register = super::ICV_BPR0;
        const NAME: &'static str = "BinaryPoint";
        const OFFSET: usize = 0;
        const LEN: usize = 3;
    }
}

/// Hyp Vector Base Address Register
pub struct HVBAR;
impl Register for HVBAR {
    const NAME: &'static str = "HVBAR";
    const LEN: usize = 64;
}

/// HVBAR register fields
pub mod hvbar {
    use crate::core::model::proc::RegisterView;

    /// HVBAR\[31:5\]
    pub struct VBA;
    impl RegisterView for VBA {
        type Register = super::HVBAR;
        const NAME: &'static str = "VBA";
        const OFFSET: usize = 5;
        const LEN: usize = 27;
    }
}

/// Virtualization Processor ID Register
pub struct VPIDR;
impl Register for VPIDR {
    const NAME: &'static str = "VPIDR";
    const LEN: usize = 64;
}

/// VPIDR register fields
pub mod vpidr {
    use crate::core::model::proc::RegisterView;

    /// VPIDR\[31:24\]
    pub struct Implementer;
    impl RegisterView for Implementer {
        type Register = super::VPIDR;
        const NAME: &'static str = "Implementer";
        const OFFSET: usize = 24;
        const LEN: usize = 8;
    }

    /// VPIDR\[23:20\]
    pub struct Variant;
    impl RegisterView for Variant {
        type Register = super::VPIDR;
        const NAME: &'static str = "Variant";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// VPIDR\[19:16\]
    pub struct Architecture;
    impl RegisterView for Architecture {
        type Register = super::VPIDR;
        const NAME: &'static str = "Architecture";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// VPIDR\[15:4\]
    pub struct PartNum;
    impl RegisterView for PartNum {
        type Register = super::VPIDR;
        const NAME: &'static str = "PartNum";
        const OFFSET: usize = 4;
        const LEN: usize = 12;
    }

    /// VPIDR\[3:0\]
    pub struct Revision;
    impl RegisterView for Revision {
        type Register = super::VPIDR;
        const NAME: &'static str = "Revision";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Interrupt Controller Virtual Active Priorities Group 0 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICV_AP0Rn;
impl RegisterArray for ICV_AP0Rn {
    const NAME: &'static str = "ICV_AP0Rn";
    const ELEMS: usize = 3;
    const ELEM_LEN: usize = 64;
}

/// Auxiliary Control Register
pub struct ACTLR;
impl Register for ACTLR {
    const NAME: &'static str = "ACTLR";
    const LEN: usize = 64;
}

/// Counter-timer Virtual Timer CompareValue register
#[allow(non_camel_case_types)]
pub struct CNTV_CVAL;
impl Register for CNTV_CVAL {
    const NAME: &'static str = "CNTV_CVAL";
    const LEN: usize = 64;
}

/// CNTV_CVAL register fields
pub mod cntv_cval {
    use crate::core::model::proc::RegisterView;

    /// CNTV_CVAL\[63:0\]
    pub struct CompareValue;
    impl RegisterView for CompareValue {
        type Register = super::CNTV_CVAL;
        const NAME: &'static str = "CompareValue";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Instruction Set Attribute Register 1
#[allow(non_camel_case_types)]
pub struct ID_ISAR1;
impl Register for ID_ISAR1 {
    const NAME: &'static str = "ID_ISAR1";
    const LEN: usize = 64;
}

/// ID_ISAR1 register fields
pub mod id_isar1 {
    use crate::core::model::proc::RegisterView;

    /// ID_ISAR1\[31:28\]
    pub struct Jazelle;
    impl RegisterView for Jazelle {
        type Register = super::ID_ISAR1;
        const NAME: &'static str = "Jazelle";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_ISAR1\[27:24\]
    pub struct Interwork;
    impl RegisterView for Interwork {
        type Register = super::ID_ISAR1;
        const NAME: &'static str = "Interwork";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_ISAR1\[23:20\]
    pub struct Immediate;
    impl RegisterView for Immediate {
        type Register = super::ID_ISAR1;
        const NAME: &'static str = "Immediate";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_ISAR1\[19:16\]
    pub struct IfThen;
    impl RegisterView for IfThen {
        type Register = super::ID_ISAR1;
        const NAME: &'static str = "IfThen";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_ISAR1\[15:12\]
    pub struct Extend;
    impl RegisterView for Extend {
        type Register = super::ID_ISAR1;
        const NAME: &'static str = "Extend";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_ISAR1\[11:8\]
    #[allow(non_camel_case_types)]
    pub struct Except_AR;
    impl RegisterView for Except_AR {
        type Register = super::ID_ISAR1;
        const NAME: &'static str = "Except_AR";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_ISAR1\[7:4\]
    pub struct Except;
    impl RegisterView for Except {
        type Register = super::ID_ISAR1;
        const NAME: &'static str = "Except";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_ISAR1\[3:0\]
    pub struct Endian;
    impl RegisterView for Endian {
        type Register = super::ID_ISAR1;
        const NAME: &'static str = "Endian";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Debug ID Register
pub struct DBGDIDR;
impl Register for DBGDIDR {
    const NAME: &'static str = "DBGDIDR";
    const LEN: usize = 64;
}

/// DBGDIDR register fields
pub mod dbgdidr {
    use crate::core::model::proc::RegisterView;

    /// DBGDIDR\[31:28\]
    pub struct WRPs;
    impl RegisterView for WRPs {
        type Register = super::DBGDIDR;
        const NAME: &'static str = "WRPs";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// DBGDIDR\[27:24\]
    pub struct BRPs;
    impl RegisterView for BRPs {
        type Register = super::DBGDIDR;
        const NAME: &'static str = "BRPs";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// DBGDIDR\[23:20\]
    #[allow(non_camel_case_types)]
    pub struct CTX_CMPs;
    impl RegisterView for CTX_CMPs {
        type Register = super::DBGDIDR;
        const NAME: &'static str = "CTX_CMPs";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// DBGDIDR\[19:16\]
    pub struct Version;
    impl RegisterView for Version {
        type Register = super::DBGDIDR;
        const NAME: &'static str = "Version";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// DBGDIDR\[14\]
    #[allow(non_camel_case_types)]
    pub struct nSUHD_imp;
    impl RegisterView for nSUHD_imp {
        type Register = super::DBGDIDR;
        const NAME: &'static str = "nSUHD_imp";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// DBGDIDR\[12\]
    #[allow(non_camel_case_types)]
    pub struct SE_imp;
    impl RegisterView for SE_imp {
        type Register = super::DBGDIDR;
        const NAME: &'static str = "SE_imp";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }
}

/// Counter-timer Hyp Control register
pub struct CNTHCTL;
impl Register for CNTHCTL {
    const NAME: &'static str = "CNTHCTL";
    const LEN: usize = 64;
}

/// CNTHCTL register fields
pub mod cnthctl {
    use crate::core::model::proc::RegisterView;

    /// CNTHCTL\[17\]
    pub struct EVNTIS;
    impl RegisterView for EVNTIS {
        type Register = super::CNTHCTL;
        const NAME: &'static str = "EVNTIS";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// CNTHCTL\[7:4\]
    pub struct EVNTI;
    impl RegisterView for EVNTI {
        type Register = super::CNTHCTL;
        const NAME: &'static str = "EVNTI";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// CNTHCTL\[3\]
    pub struct EVNTDIR;
    impl RegisterView for EVNTDIR {
        type Register = super::CNTHCTL;
        const NAME: &'static str = "EVNTDIR";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// CNTHCTL\[2\]
    pub struct EVNTEN;
    impl RegisterView for EVNTEN {
        type Register = super::CNTHCTL;
        const NAME: &'static str = "EVNTEN";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// CNTHCTL\[1\]
    pub struct PL1PCEN;
    impl RegisterView for PL1PCEN {
        type Register = super::CNTHCTL;
        const NAME: &'static str = "PL1PCEN";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// CNTHCTL\[0\]
    pub struct PL1PCTEN;
    impl RegisterView for PL1PCTEN {
        type Register = super::CNTHCTL;
        const NAME: &'static str = "PL1PCTEN";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Processor Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_PFR0;
impl Register for ID_PFR0 {
    const NAME: &'static str = "ID_PFR0";
    const LEN: usize = 64;
}

/// ID_PFR0 register fields
pub mod id_pfr0 {
    use crate::core::model::proc::RegisterView;

    /// ID_PFR0\[31:28\]
    pub struct RAS;
    impl RegisterView for RAS {
        type Register = super::ID_PFR0;
        const NAME: &'static str = "RAS";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_PFR0\[27:24\]
    pub struct DIT;
    impl RegisterView for DIT {
        type Register = super::ID_PFR0;
        const NAME: &'static str = "DIT";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_PFR0\[23:20\]
    pub struct AMU;
    impl RegisterView for AMU {
        type Register = super::ID_PFR0;
        const NAME: &'static str = "AMU";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_PFR0\[19:16\]
    pub struct CSV2;
    impl RegisterView for CSV2 {
        type Register = super::ID_PFR0;
        const NAME: &'static str = "CSV2";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_PFR0\[15:12\]
    pub struct State3;
    impl RegisterView for State3 {
        type Register = super::ID_PFR0;
        const NAME: &'static str = "State3";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_PFR0\[11:8\]
    pub struct State2;
    impl RegisterView for State2 {
        type Register = super::ID_PFR0;
        const NAME: &'static str = "State2";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_PFR0\[7:4\]
    pub struct State1;
    impl RegisterView for State1 {
        type Register = super::ID_PFR0;
        const NAME: &'static str = "State1";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_PFR0\[3:0\]
    pub struct State0;
    impl RegisterView for State0 {
        type Register = super::ID_PFR0;
        const NAME: &'static str = "State0";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Auxiliary Control Register 2
pub struct ACTLR2;
impl Register for ACTLR2 {
    const NAME: &'static str = "ACTLR2";
    const LEN: usize = 64;
}

/// Interrupt Controller Active Priorities Group 0 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICC_AP0Rn;
impl RegisterArray for ICC_AP0Rn {
    const NAME: &'static str = "ICC_AP0Rn";
    const ELEMS: usize = 3;
    const ELEM_LEN: usize = 64;
}

/// Counter-timer Self-Synchronized Physical Count register
pub struct CNTPCTSS;
impl Register for CNTPCTSS {
    const NAME: &'static str = "CNTPCTSS";
    const LEN: usize = 64;
}

/// Debug Authentication Status register
pub struct DBGAUTHSTATUS;
impl Register for DBGAUTHSTATUS {
    const NAME: &'static str = "DBGAUTHSTATUS";
    const LEN: usize = 64;
}

/// DBGAUTHSTATUS register fields
pub mod dbgauthstatus {
    use crate::core::model::proc::RegisterView;

    /// DBGAUTHSTATUS\[7:6\]
    pub struct SNID;
    impl RegisterView for SNID {
        type Register = super::DBGAUTHSTATUS;
        const NAME: &'static str = "SNID";
        const OFFSET: usize = 6;
        const LEN: usize = 2;
    }

    /// DBGAUTHSTATUS\[5:4\]
    pub struct SID;
    impl RegisterView for SID {
        type Register = super::DBGAUTHSTATUS;
        const NAME: &'static str = "SID";
        const OFFSET: usize = 4;
        const LEN: usize = 2;
    }

    /// DBGAUTHSTATUS\[3:2\]
    pub struct NSNID;
    impl RegisterView for NSNID {
        type Register = super::DBGAUTHSTATUS;
        const NAME: &'static str = "NSNID";
        const OFFSET: usize = 2;
        const LEN: usize = 2;
    }

    /// DBGAUTHSTATUS\[1:0\]
    pub struct NSID;
    impl RegisterView for NSID {
        type Register = super::DBGAUTHSTATUS;
        const NAME: &'static str = "NSID";
        const OFFSET: usize = 0;
        const LEN: usize = 2;
    }
}

/// Counter-timer Secure Virtual Timer Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHVS_CTL;
impl Register for CNTHVS_CTL {
    const NAME: &'static str = "CNTHVS_CTL";
    const LEN: usize = 64;
}

/// CNTHVS_CTL register fields
pub mod cnthvs_ctl {
    use crate::core::model::proc::RegisterView;

    /// CNTHVS_CTL\[2\]
    pub struct ISTATUS;
    impl RegisterView for ISTATUS {
        type Register = super::CNTHVS_CTL;
        const NAME: &'static str = "ISTATUS";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// CNTHVS_CTL\[1\]
    pub struct IMASK;
    impl RegisterView for IMASK {
        type Register = super::CNTHVS_CTL;
        const NAME: &'static str = "IMASK";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// CNTHVS_CTL\[0\]
    pub struct ENABLE;
    impl RegisterView for ENABLE {
        type Register = super::CNTHVS_CTL;
        const NAME: &'static str = "ENABLE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Instruction Fault Address Register
pub struct IFAR;
impl Register for IFAR {
    const NAME: &'static str = "IFAR";
    const LEN: usize = 64;
}

/// IFAR register fields
pub mod ifar {
    use crate::core::model::proc::RegisterView;

    /// IFAR\[31:0\]
    pub struct VA;
    impl RegisterView for VA {
        type Register = super::IFAR;
        const NAME: &'static str = "VA";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Performance Monitors User Enable Register
pub struct PMUSERENR;
impl Register for PMUSERENR {
    const NAME: &'static str = "PMUSERENR";
    const LEN: usize = 64;
}

/// PMUSERENR register fields
pub mod pmuserenr {
    use crate::core::model::proc::RegisterView;

    /// PMUSERENR\[6\]
    pub struct TID;
    impl RegisterView for TID {
        type Register = super::PMUSERENR;
        const NAME: &'static str = "TID";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// PMUSERENR\[3\]
    pub struct ER;
    impl RegisterView for ER {
        type Register = super::PMUSERENR;
        const NAME: &'static str = "ER";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// PMUSERENR\[2\]
    pub struct CR;
    impl RegisterView for CR {
        type Register = super::PMUSERENR;
        const NAME: &'static str = "CR";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// PMUSERENR\[1\]
    pub struct SW;
    impl RegisterView for SW {
        type Register = super::PMUSERENR;
        const NAME: &'static str = "SW";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// PMUSERENR\[0\]
    pub struct EN;
    impl RegisterView for EN {
        type Register = super::PMUSERENR;
        const NAME: &'static str = "EN";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Processor Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_PFR1;
impl Register for ID_PFR1 {
    const NAME: &'static str = "ID_PFR1";
    const LEN: usize = 64;
}

/// ID_PFR1 register fields
pub mod id_pfr1 {
    use crate::core::model::proc::RegisterView;

    /// ID_PFR1\[31:28\]
    pub struct GIC;
    impl RegisterView for GIC {
        type Register = super::ID_PFR1;
        const NAME: &'static str = "GIC";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_PFR1\[27:24\]
    #[allow(non_camel_case_types)]
    pub struct Virt_frac;
    impl RegisterView for Virt_frac {
        type Register = super::ID_PFR1;
        const NAME: &'static str = "Virt_frac";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_PFR1\[23:20\]
    #[allow(non_camel_case_types)]
    pub struct Sec_frac;
    impl RegisterView for Sec_frac {
        type Register = super::ID_PFR1;
        const NAME: &'static str = "Sec_frac";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_PFR1\[19:16\]
    pub struct GenTimer;
    impl RegisterView for GenTimer {
        type Register = super::ID_PFR1;
        const NAME: &'static str = "GenTimer";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_PFR1\[15:12\]
    pub struct Virtualization;
    impl RegisterView for Virtualization {
        type Register = super::ID_PFR1;
        const NAME: &'static str = "Virtualization";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_PFR1\[11:8\]
    pub struct MProgMod;
    impl RegisterView for MProgMod {
        type Register = super::ID_PFR1;
        const NAME: &'static str = "MProgMod";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_PFR1\[7:4\]
    pub struct Security;
    impl RegisterView for Security {
        type Register = super::ID_PFR1;
        const NAME: &'static str = "Security";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_PFR1\[3:0\]
    pub struct ProgMod;
    impl RegisterView for ProgMod {
        type Register = super::ID_PFR1;
        const NAME: &'static str = "ProgMod";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Debug OS Lock Status Register
pub struct DBGOSLSR;
impl Register for DBGOSLSR {
    const NAME: &'static str = "DBGOSLSR";
    const LEN: usize = 64;
}

/// DBGOSLSR register fields
pub mod dbgoslsr {
    use crate::core::model::proc::RegisterView;

    /// DBGOSLSR\[0\]
    pub struct OSLM;
    impl RegisterView for OSLM {
        type Register = super::DBGOSLSR;
        const NAME: &'static str = "OSLM";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }

    /// DBGOSLSR\[2\]
    #[allow(non_camel_case_types)]
    pub struct nTT;
    impl RegisterView for nTT {
        type Register = super::DBGOSLSR;
        const NAME: &'static str = "nTT";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// DBGOSLSR\[1\]
    pub struct OSLK;
    impl RegisterView for OSLK {
        type Register = super::DBGOSLSR;
        const NAME: &'static str = "OSLK";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }
}

/// Instruction Set Attribute Register 0
#[allow(non_camel_case_types)]
pub struct ID_ISAR0;
impl Register for ID_ISAR0 {
    const NAME: &'static str = "ID_ISAR0";
    const LEN: usize = 64;
}

/// ID_ISAR0 register fields
pub mod id_isar0 {
    use crate::core::model::proc::RegisterView;

    /// ID_ISAR0\[27:24\]
    pub struct Divide;
    impl RegisterView for Divide {
        type Register = super::ID_ISAR0;
        const NAME: &'static str = "Divide";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_ISAR0\[23:20\]
    pub struct Debug;
    impl RegisterView for Debug {
        type Register = super::ID_ISAR0;
        const NAME: &'static str = "Debug";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_ISAR0\[19:16\]
    pub struct Coproc;
    impl RegisterView for Coproc {
        type Register = super::ID_ISAR0;
        const NAME: &'static str = "Coproc";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_ISAR0\[15:12\]
    pub struct CmpBranch;
    impl RegisterView for CmpBranch {
        type Register = super::ID_ISAR0;
        const NAME: &'static str = "CmpBranch";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_ISAR0\[11:8\]
    pub struct BitField;
    impl RegisterView for BitField {
        type Register = super::ID_ISAR0;
        const NAME: &'static str = "BitField";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_ISAR0\[7:4\]
    pub struct BitCount;
    impl RegisterView for BitCount {
        type Register = super::ID_ISAR0;
        const NAME: &'static str = "BitCount";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_ISAR0\[3:0\]
    pub struct Swap;
    impl RegisterView for Swap {
        type Register = super::ID_ISAR0;
        const NAME: &'static str = "Swap";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Performance Monitors Count Enable Set register
pub struct PMCNTENSET;
impl Register for PMCNTENSET {
    const NAME: &'static str = "PMCNTENSET";
    const LEN: usize = 64;
}

/// PMCNTENSET register fields
pub mod pmcntenset {
    use crate::core::model::proc::RegisterView;

    /// PMCNTENSET\[31\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::PMCNTENSET;
        const NAME: &'static str = "C";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Virtual Binary Point Register 1
#[allow(non_camel_case_types)]
pub struct ICV_BPR1;
impl Register for ICV_BPR1 {
    const NAME: &'static str = "ICV_BPR1";
    const LEN: usize = 64;
}

/// ICV_BPR1 register fields
pub mod icv_bpr1 {
    use crate::core::model::proc::RegisterView;

    /// ICV_BPR1\[2:0\]
    pub struct BinaryPoint;
    impl RegisterView for BinaryPoint {
        type Register = super::ICV_BPR1;
        const NAME: &'static str = "BinaryPoint";
        const OFFSET: usize = 0;
        const LEN: usize = 3;
    }
}

/// Hyp Architectural Feature Trap Register
pub struct HCPTR;
impl Register for HCPTR {
    const NAME: &'static str = "HCPTR";
    const LEN: usize = 64;
}

/// HCPTR register fields
pub mod hcptr {
    use crate::core::model::proc::RegisterView;

    /// HCPTR\[31\]
    pub struct TCPAC;
    impl RegisterView for TCPAC {
        type Register = super::HCPTR;
        const NAME: &'static str = "TCPAC";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// HCPTR\[30\]
    pub struct TAM;
    impl RegisterView for TAM {
        type Register = super::HCPTR;
        const NAME: &'static str = "TAM";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// HCPTR\[20\]
    pub struct TTA;
    impl RegisterView for TTA {
        type Register = super::HCPTR;
        const NAME: &'static str = "TTA";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// HCPTR\[15\]
    pub struct TASE;
    impl RegisterView for TASE {
        type Register = super::HCPTR;
        const NAME: &'static str = "TASE";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// HCPTR\[11\]
    pub struct TCP11;
    impl RegisterView for TCP11 {
        type Register = super::HCPTR;
        const NAME: &'static str = "TCP11";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// HCPTR\[10\]
    pub struct TCP10;
    impl RegisterView for TCP10 {
        type Register = super::HCPTR;
        const NAME: &'static str = "TCP10";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }
}

/// Debug Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_DFR0;
impl Register for ID_DFR0 {
    const NAME: &'static str = "ID_DFR0";
    const LEN: usize = 64;
}

/// ID_DFR0 register fields
pub mod id_dfr0 {
    use crate::core::model::proc::RegisterView;

    /// ID_DFR0\[31:28\]
    pub struct TraceFilt;
    impl RegisterView for TraceFilt {
        type Register = super::ID_DFR0;
        const NAME: &'static str = "TraceFilt";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_DFR0\[27:24\]
    pub struct PerfMon;
    impl RegisterView for PerfMon {
        type Register = super::ID_DFR0;
        const NAME: &'static str = "PerfMon";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_DFR0\[23:20\]
    pub struct MProfDbg;
    impl RegisterView for MProfDbg {
        type Register = super::ID_DFR0;
        const NAME: &'static str = "MProfDbg";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_DFR0\[19:16\]
    pub struct MMapTrc;
    impl RegisterView for MMapTrc {
        type Register = super::ID_DFR0;
        const NAME: &'static str = "MMapTrc";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_DFR0\[15:12\]
    pub struct CopTrc;
    impl RegisterView for CopTrc {
        type Register = super::ID_DFR0;
        const NAME: &'static str = "CopTrc";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_DFR0\[11:8\]
    pub struct MMapDbg;
    impl RegisterView for MMapDbg {
        type Register = super::ID_DFR0;
        const NAME: &'static str = "MMapDbg";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_DFR0\[7:4\]
    pub struct CopSDbg;
    impl RegisterView for CopSDbg {
        type Register = super::ID_DFR0;
        const NAME: &'static str = "CopSDbg";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_DFR0\[3:0\]
    pub struct CopDbg;
    impl RegisterView for CopDbg {
        type Register = super::ID_DFR0;
        const NAME: &'static str = "CopDbg";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Hyp Software Thread ID Register
pub struct HTPIDR;
impl Register for HTPIDR {
    const NAME: &'static str = "HTPIDR";
    const LEN: usize = 64;
}

/// HTPIDR register fields
pub mod htpidr {
    use crate::core::model::proc::RegisterView;

    /// HTPIDR\[31:0\]
    pub struct TID;
    impl RegisterView for TID {
        type Register = super::HTPIDR;
        const NAME: &'static str = "TID";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Interrupt Controller Interrupt Acknowledge Register 0
#[allow(non_camel_case_types)]
pub struct ICC_IAR0;
impl Register for ICC_IAR0 {
    const NAME: &'static str = "ICC_IAR0";
    const LEN: usize = 64;
}

/// ICC_IAR0 register fields
pub mod icc_iar0 {
    use crate::core::model::proc::RegisterView;

    /// ICC_IAR0\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_IAR0;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// Interrupt Controller Hyp Active Priorities Group 0 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICH_AP0Rn;
impl RegisterArray for ICH_AP0Rn {
    const NAME: &'static str = "ICH_AP0Rn";
    const ELEMS: usize = 3;
    const ELEM_LEN: usize = 64;
}

/// Interrupt Controller Running Priority Register
#[allow(non_camel_case_types)]
pub struct ICC_RPR;
impl Register for ICC_RPR {
    const NAME: &'static str = "ICC_RPR";
    const LEN: usize = 64;
}

/// ICC_RPR register fields
pub mod icc_rpr {
    use crate::core::model::proc::RegisterView;

    /// ICC_RPR\[7:0\]
    pub struct Priority;
    impl RegisterView for Priority {
        type Register = super::ICC_RPR;
        const NAME: &'static str = "Priority";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Instruction Set Attribute Register 4
#[allow(non_camel_case_types)]
pub struct ID_ISAR4;
impl Register for ID_ISAR4 {
    const NAME: &'static str = "ID_ISAR4";
    const LEN: usize = 64;
}

/// ID_ISAR4 register fields
pub mod id_isar4 {
    use crate::core::model::proc::RegisterView;

    /// ID_ISAR4\[31:28\]
    #[allow(non_camel_case_types)]
    pub struct SWP_frac;
    impl RegisterView for SWP_frac {
        type Register = super::ID_ISAR4;
        const NAME: &'static str = "SWP_frac";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_ISAR4\[27:24\]
    #[allow(non_camel_case_types)]
    pub struct PSR_M;
    impl RegisterView for PSR_M {
        type Register = super::ID_ISAR4;
        const NAME: &'static str = "PSR_M";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_ISAR4\[23:20\]
    #[allow(non_camel_case_types)]
    pub struct SynchPrim_frac;
    impl RegisterView for SynchPrim_frac {
        type Register = super::ID_ISAR4;
        const NAME: &'static str = "SynchPrim_frac";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_ISAR4\[19:16\]
    pub struct Barrier;
    impl RegisterView for Barrier {
        type Register = super::ID_ISAR4;
        const NAME: &'static str = "Barrier";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_ISAR4\[15:12\]
    pub struct SMC;
    impl RegisterView for SMC {
        type Register = super::ID_ISAR4;
        const NAME: &'static str = "SMC";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_ISAR4\[11:8\]
    pub struct Writeback;
    impl RegisterView for Writeback {
        type Register = super::ID_ISAR4;
        const NAME: &'static str = "Writeback";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_ISAR4\[7:4\]
    pub struct WithShifts;
    impl RegisterView for WithShifts {
        type Register = super::ID_ISAR4;
        const NAME: &'static str = "WithShifts";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_ISAR4\[3:0\]
    pub struct Unpriv;
    impl RegisterView for Unpriv {
        type Register = super::ID_ISAR4;
        const NAME: &'static str = "Unpriv";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Hyp Reset Management Register
pub struct HRMR;
impl Register for HRMR {
    const NAME: &'static str = "HRMR";
    const LEN: usize = 64;
}

/// HRMR register fields
pub mod hrmr {
    use crate::core::model::proc::RegisterView;

    /// HRMR\[1\]
    pub struct RR;
    impl RegisterView for RR {
        type Register = super::HRMR;
        const NAME: &'static str = "RR";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// HRMR\[0\]
    pub struct AA64;
    impl RegisterView for AA64 {
        type Register = super::HRMR;
        const NAME: &'static str = "AA64";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Debug Watchpoint Control Registers, n = 15 - 0
pub struct DBGWCRn;
impl RegisterArray for DBGWCRn {
    const NAME: &'static str = "DBGWCRn";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// DBGWCRn register fields
pub mod dbgwcrn {
    use crate::core::model::proc::RegisterArrayView;

    /// DBGWCRn\[28:24\]
    pub struct MASK;
    impl RegisterArrayView for MASK {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "MASK";
        const OFFSET: usize = 24;
        const LEN: usize = 5;
    }

    /// DBGWCRn\[20\]
    pub struct WT;
    impl RegisterArrayView for WT {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "WT";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// DBGWCRn\[19:16\]
    pub struct LBN;
    impl RegisterArrayView for LBN {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "LBN";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// DBGWCRn\[15:14\]
    pub struct SSC;
    impl RegisterArrayView for SSC {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "SSC";
        const OFFSET: usize = 14;
        const LEN: usize = 2;
    }

    /// DBGWCRn\[13\]
    pub struct HMC;
    impl RegisterArrayView for HMC {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "HMC";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// DBGWCRn\[12:5\]
    pub struct BAS;
    impl RegisterArrayView for BAS {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "BAS";
        const OFFSET: usize = 5;
        const LEN: usize = 8;
    }

    /// DBGWCRn\[4:3\]
    pub struct LSC;
    impl RegisterArrayView for LSC {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "LSC";
        const OFFSET: usize = 3;
        const LEN: usize = 2;
    }

    /// DBGWCRn\[2:1\]
    pub struct PAC;
    impl RegisterArrayView for PAC {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "PAC";
        const OFFSET: usize = 1;
        const LEN: usize = 2;
    }

    /// DBGWCRn\[0\]
    pub struct E;
    impl RegisterArrayView for E {
        type RegisterArray = super::DBGWCRn;
        const NAME: &'static str = "E";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Virtual Running Priority Register
#[allow(non_camel_case_types)]
pub struct ICV_RPR;
impl Register for ICV_RPR {
    const NAME: &'static str = "ICV_RPR";
    const LEN: usize = 64;
}

/// ICV_RPR register fields
pub mod icv_rpr {
    use crate::core::model::proc::RegisterView;

    /// ICV_RPR\[7:0\]
    pub struct Priority;
    impl RegisterView for Priority {
        type Register = super::ICV_RPR;
        const NAME: &'static str = "Priority";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// TCM Type Register
pub struct TCMTR;
impl Register for TCMTR {
    const NAME: &'static str = "TCMTR";
    const LEN: usize = 64;
}

/// Debug Link Register
pub struct DLR;
impl Register for DLR {
    const NAME: &'static str = "DLR";
    const LEN: usize = 64;
}

/// Activity Monitors Counter Group Configuration Register
pub struct AMCGCR;
impl Register for AMCGCR {
    const NAME: &'static str = "AMCGCR";
    const LEN: usize = 64;
}

/// AMCGCR register fields
pub mod amcgcr {
    use crate::core::model::proc::RegisterView;

    /// AMCGCR\[15:8\]
    pub struct CG1NC;
    impl RegisterView for CG1NC {
        type Register = super::AMCGCR;
        const NAME: &'static str = "CG1NC";
        const OFFSET: usize = 8;
        const LEN: usize = 8;
    }

    /// AMCGCR\[7:0\]
    pub struct CG0NC;
    impl RegisterView for CG0NC {
        type Register = super::AMCGCR;
        const NAME: &'static str = "CG0NC";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Interrupt Controller List Registers, n = 15 - 0
#[allow(non_camel_case_types)]
pub struct ICH_LRn;
impl RegisterArray for ICH_LRn {
    const NAME: &'static str = "ICH_LRn";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// ICH_LRn register fields
pub mod ich_lrn {
    use crate::core::model::proc::RegisterArrayView;

    /// ICH_LRn\[31:0\]
    #[allow(non_camel_case_types)]
    pub struct vINTID;
    impl RegisterArrayView for vINTID {
        type RegisterArray = super::ICH_LRn;
        const NAME: &'static str = "vINTID";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Instruction Set Attribute Register 5
#[allow(non_camel_case_types)]
pub struct ID_ISAR5;
impl Register for ID_ISAR5 {
    const NAME: &'static str = "ID_ISAR5";
    const LEN: usize = 64;
}

/// ID_ISAR5 register fields
pub mod id_isar5 {
    use crate::core::model::proc::RegisterView;

    /// ID_ISAR5\[31:28\]
    pub struct VCMA;
    impl RegisterView for VCMA {
        type Register = super::ID_ISAR5;
        const NAME: &'static str = "VCMA";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_ISAR5\[27:24\]
    pub struct RDM;
    impl RegisterView for RDM {
        type Register = super::ID_ISAR5;
        const NAME: &'static str = "RDM";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_ISAR5\[19:16\]
    pub struct CRC32;
    impl RegisterView for CRC32 {
        type Register = super::ID_ISAR5;
        const NAME: &'static str = "CRC32";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_ISAR5\[15:12\]
    pub struct SHA2;
    impl RegisterView for SHA2 {
        type Register = super::ID_ISAR5;
        const NAME: &'static str = "SHA2";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_ISAR5\[11:8\]
    pub struct SHA1;
    impl RegisterView for SHA1 {
        type Register = super::ID_ISAR5;
        const NAME: &'static str = "SHA1";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_ISAR5\[7:4\]
    pub struct AES;
    impl RegisterView for AES {
        type Register = super::ID_ISAR5;
        const NAME: &'static str = "AES";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_ISAR5\[3:0\]
    pub struct SEVL;
    impl RegisterView for SEVL {
        type Register = super::ID_ISAR5;
        const NAME: &'static str = "SEVL";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Interrupt Controller Interrupt Acknowledge Register 1
#[allow(non_camel_case_types)]
pub struct ICC_IAR1;
impl Register for ICC_IAR1 {
    const NAME: &'static str = "ICC_IAR1";
    const LEN: usize = 64;
}

/// ICC_IAR1 register fields
pub mod icc_iar1 {
    use crate::core::model::proc::RegisterView;

    /// ICC_IAR1\[23:0\]
    pub struct INTID;
    impl RegisterView for INTID {
        type Register = super::ICC_IAR1;
        const NAME: &'static str = "INTID";
        const OFFSET: usize = 0;
        const LEN: usize = 24;
    }
}

/// FCSE Process ID register
pub struct FCSEIDR;
impl Register for FCSEIDR {
    const NAME: &'static str = "FCSEIDR";
    const LEN: usize = 64;
}

/// Interrupt Controller Binary Point Register 1
#[allow(non_camel_case_types)]
pub struct ICC_BPR1;
impl Register for ICC_BPR1 {
    const NAME: &'static str = "ICC_BPR1";
    const LEN: usize = 64;
}

/// ICC_BPR1 register fields
pub mod icc_bpr1 {
    use crate::core::model::proc::RegisterView;

    /// ICC_BPR1\[2:0\]
    pub struct BinaryPoint;
    impl RegisterView for BinaryPoint {
        type Register = super::ICC_BPR1;
        const NAME: &'static str = "BinaryPoint";
        const OFFSET: usize = 0;
        const LEN: usize = 3;
    }
}

/// Activity Monitors Control Register
pub struct AMCR;
impl Register for AMCR {
    const NAME: &'static str = "AMCR";
    const LEN: usize = 64;
}

/// AMCR register fields
pub mod amcr {
    use crate::core::model::proc::RegisterView;

    /// AMCR\[17\]
    pub struct CG1RZ;
    impl RegisterView for CG1RZ {
        type Register = super::AMCR;
        const NAME: &'static str = "CG1RZ";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// AMCR\[10\]
    pub struct HDBG;
    impl RegisterView for HDBG {
        type Register = super::AMCR;
        const NAME: &'static str = "HDBG";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }
}

/// Counter-timer Virtual Timer Control register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHV_CTL;
impl Register for CNTHV_CTL {
    const NAME: &'static str = "CNTHV_CTL";
    const LEN: usize = 64;
}

/// CNTHV_CTL register fields
pub mod cnthv_ctl {
    use crate::core::model::proc::RegisterView;

    /// CNTHV_CTL\[2\]
    pub struct ISTATUS;
    impl RegisterView for ISTATUS {
        type Register = super::CNTHV_CTL;
        const NAME: &'static str = "ISTATUS";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// CNTHV_CTL\[1\]
    pub struct IMASK;
    impl RegisterView for IMASK {
        type Register = super::CNTHV_CTL;
        const NAME: &'static str = "IMASK";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// CNTHV_CTL\[0\]
    pub struct ENABLE;
    impl RegisterView for ENABLE {
        type Register = super::CNTHV_CTL;
        const NAME: &'static str = "ENABLE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Secure Configuration Register
pub struct SCR;
impl Register for SCR {
    const NAME: &'static str = "SCR";
    const LEN: usize = 64;
}

/// SCR register fields
pub mod scr {
    use crate::core::model::proc::RegisterView;

    /// SCR\[15\]
    pub struct TERR;
    impl RegisterView for TERR {
        type Register = super::SCR;
        const NAME: &'static str = "TERR";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// SCR\[13\]
    pub struct TWE;
    impl RegisterView for TWE {
        type Register = super::SCR;
        const NAME: &'static str = "TWE";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// SCR\[12\]
    pub struct TWI;
    impl RegisterView for TWI {
        type Register = super::SCR;
        const NAME: &'static str = "TWI";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// SCR\[9\]
    pub struct SIF;
    impl RegisterView for SIF {
        type Register = super::SCR;
        const NAME: &'static str = "SIF";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// SCR\[8\]
    pub struct HCE;
    impl RegisterView for HCE {
        type Register = super::SCR;
        const NAME: &'static str = "HCE";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// SCR\[7\]
    pub struct SCD;
    impl RegisterView for SCD {
        type Register = super::SCR;
        const NAME: &'static str = "SCD";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// SCR\[6\]
    #[allow(non_camel_case_types)]
    pub struct nET;
    impl RegisterView for nET {
        type Register = super::SCR;
        const NAME: &'static str = "nET";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// SCR\[5\]
    pub struct AW;
    impl RegisterView for AW {
        type Register = super::SCR;
        const NAME: &'static str = "AW";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// SCR\[4\]
    pub struct FW;
    impl RegisterView for FW {
        type Register = super::SCR;
        const NAME: &'static str = "FW";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// SCR\[3\]
    pub struct EA;
    impl RegisterView for EA {
        type Register = super::SCR;
        const NAME: &'static str = "EA";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// SCR\[2\]
    pub struct FIQ;
    impl RegisterView for FIQ {
        type Register = super::SCR;
        const NAME: &'static str = "FIQ";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// SCR\[1\]
    pub struct IRQ;
    impl RegisterView for IRQ {
        type Register = super::SCR;
        const NAME: &'static str = "IRQ";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// SCR\[0\]
    pub struct NS;
    impl RegisterView for NS {
        type Register = super::SCR;
        const NAME: &'static str = "NS";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Debug Data Transfer Register, Transmit
pub struct DBGDTRTXint;
impl Register for DBGDTRTXint {
    const NAME: &'static str = "DBGDTRTXint";
    const LEN: usize = 64;
}

/// DBGDTRTXint register fields
pub mod dbgdtrtxint {
    use crate::core::model::proc::RegisterView;

    /// DBGDTRTXint\[31:0\]
    pub struct DTRTX;
    impl RegisterView for DTRTX {
        type Register = super::DBGDTRTXint;
        const NAME: &'static str = "DTRTX";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Debug OS Lock Data Transfer Register, Transmit
pub struct DBGDTRTXext;
impl Register for DBGDTRTXext {
    const NAME: &'static str = "DBGDTRTXext";
    const LEN: usize = 64;
}

/// DBGDTRTXext register fields
pub mod dbgdtrtxext {
    use crate::core::model::proc::RegisterView;

    /// DBGDTRTXext\[31:0\]
    pub struct DTRTX;
    impl RegisterView for DTRTX {
        type Register = super::DBGDTRTXext;
        const NAME: &'static str = "DTRTX";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Hyp Configuration Register
pub struct HCR;
impl Register for HCR {
    const NAME: &'static str = "HCR";
    const LEN: usize = 64;
}

/// HCR register fields
pub mod hcr {
    use crate::core::model::proc::RegisterView;

    /// HCR\[30\]
    pub struct TRVM;
    impl RegisterView for TRVM {
        type Register = super::HCR;
        const NAME: &'static str = "TRVM";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// HCR\[29\]
    pub struct HCD;
    impl RegisterView for HCD {
        type Register = super::HCR;
        const NAME: &'static str = "HCD";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// HCR\[27\]
    pub struct TGE;
    impl RegisterView for TGE {
        type Register = super::HCR;
        const NAME: &'static str = "TGE";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// HCR\[26\]
    pub struct TVM;
    impl RegisterView for TVM {
        type Register = super::HCR;
        const NAME: &'static str = "TVM";
        const OFFSET: usize = 26;
        const LEN: usize = 1;
    }

    /// HCR\[25\]
    pub struct TTLB;
    impl RegisterView for TTLB {
        type Register = super::HCR;
        const NAME: &'static str = "TTLB";
        const OFFSET: usize = 25;
        const LEN: usize = 1;
    }

    /// HCR\[24\]
    pub struct TPU;
    impl RegisterView for TPU {
        type Register = super::HCR;
        const NAME: &'static str = "TPU";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// HCR\[23\]
    pub struct TPC;
    impl RegisterView for TPC {
        type Register = super::HCR;
        const NAME: &'static str = "TPC";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// HCR\[22\]
    pub struct TSW;
    impl RegisterView for TSW {
        type Register = super::HCR;
        const NAME: &'static str = "TSW";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// HCR\[21\]
    pub struct TAC;
    impl RegisterView for TAC {
        type Register = super::HCR;
        const NAME: &'static str = "TAC";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// HCR\[20\]
    pub struct TIDCP;
    impl RegisterView for TIDCP {
        type Register = super::HCR;
        const NAME: &'static str = "TIDCP";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// HCR\[19\]
    pub struct TSC;
    impl RegisterView for TSC {
        type Register = super::HCR;
        const NAME: &'static str = "TSC";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// HCR\[18\]
    pub struct TID3;
    impl RegisterView for TID3 {
        type Register = super::HCR;
        const NAME: &'static str = "TID3";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// HCR\[17\]
    pub struct TID2;
    impl RegisterView for TID2 {
        type Register = super::HCR;
        const NAME: &'static str = "TID2";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// HCR\[16\]
    pub struct TID1;
    impl RegisterView for TID1 {
        type Register = super::HCR;
        const NAME: &'static str = "TID1";
        const OFFSET: usize = 16;
        const LEN: usize = 1;
    }

    /// HCR\[15\]
    pub struct TID0;
    impl RegisterView for TID0 {
        type Register = super::HCR;
        const NAME: &'static str = "TID0";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// HCR\[14\]
    pub struct TWE;
    impl RegisterView for TWE {
        type Register = super::HCR;
        const NAME: &'static str = "TWE";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// HCR\[13\]
    pub struct TWI;
    impl RegisterView for TWI {
        type Register = super::HCR;
        const NAME: &'static str = "TWI";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// HCR\[12\]
    pub struct DC;
    impl RegisterView for DC {
        type Register = super::HCR;
        const NAME: &'static str = "DC";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// HCR\[11:10\]
    pub struct BSU;
    impl RegisterView for BSU {
        type Register = super::HCR;
        const NAME: &'static str = "BSU";
        const OFFSET: usize = 10;
        const LEN: usize = 2;
    }

    /// HCR\[9\]
    pub struct FB;
    impl RegisterView for FB {
        type Register = super::HCR;
        const NAME: &'static str = "FB";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// HCR\[8\]
    pub struct VA;
    impl RegisterView for VA {
        type Register = super::HCR;
        const NAME: &'static str = "VA";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// HCR\[7\]
    pub struct VI;
    impl RegisterView for VI {
        type Register = super::HCR;
        const NAME: &'static str = "VI";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// HCR\[6\]
    pub struct VF;
    impl RegisterView for VF {
        type Register = super::HCR;
        const NAME: &'static str = "VF";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// HCR\[5\]
    pub struct AMO;
    impl RegisterView for AMO {
        type Register = super::HCR;
        const NAME: &'static str = "AMO";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// HCR\[4\]
    pub struct IMO;
    impl RegisterView for IMO {
        type Register = super::HCR;
        const NAME: &'static str = "IMO";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// HCR\[3\]
    pub struct FMO;
    impl RegisterView for FMO {
        type Register = super::HCR;
        const NAME: &'static str = "FMO";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// HCR\[2\]
    pub struct PTW;
    impl RegisterView for PTW {
        type Register = super::HCR;
        const NAME: &'static str = "PTW";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// HCR\[1\]
    pub struct SWIO;
    impl RegisterView for SWIO {
        type Register = super::HCR;
        const NAME: &'static str = "SWIO";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// HCR\[0\]
    pub struct VM;
    impl RegisterView for VM {
        type Register = super::HCR;
        const NAME: &'static str = "VM";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// TLB Type Register
pub struct TLBTR;
impl Register for TLBTR {
    const NAME: &'static str = "TLBTR";
    const LEN: usize = 64;
}

/// TLBTR register fields
pub mod tlbtr {
    use crate::core::model::proc::RegisterView;

    /// TLBTR\[0\]
    #[allow(non_camel_case_types)]
    pub struct nU;
    impl RegisterView for nU {
        type Register = super::TLBTR;
        const NAME: &'static str = "nU";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Domain Access Control Register
pub struct DACR;
impl Register for DACR {
    const NAME: &'static str = "DACR";
    const LEN: usize = 64;
}

/// Interrupt Controller Hyp System Register Enable register
#[allow(non_camel_case_types)]
pub struct ICC_HSRE;
impl Register for ICC_HSRE {
    const NAME: &'static str = "ICC_HSRE";
    const LEN: usize = 64;
}

/// ICC_HSRE register fields
pub mod icc_hsre {
    use crate::core::model::proc::RegisterView;

    /// ICC_HSRE\[3\]
    pub struct Enable;
    impl RegisterView for Enable {
        type Register = super::ICC_HSRE;
        const NAME: &'static str = "Enable";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// ICC_HSRE\[2\]
    pub struct DIB;
    impl RegisterView for DIB {
        type Register = super::ICC_HSRE;
        const NAME: &'static str = "DIB";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// ICC_HSRE\[1\]
    pub struct DFB;
    impl RegisterView for DFB {
        type Register = super::ICC_HSRE;
        const NAME: &'static str = "DFB";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICC_HSRE\[0\]
    pub struct SRE;
    impl RegisterView for SRE {
        type Register = super::ICC_HSRE;
        const NAME: &'static str = "SRE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Counter-timer Hyp Physical Timer TimerValue register
#[allow(non_camel_case_types)]
pub struct CNTHP_TVAL;
impl Register for CNTHP_TVAL {
    const NAME: &'static str = "CNTHP_TVAL";
    const LEN: usize = 64;
}

/// CNTHP_TVAL register fields
pub mod cnthp_tval {
    use crate::core::model::proc::RegisterView;

    /// CNTHP_TVAL\[31:0\]
    pub struct TimerValue;
    impl RegisterView for TimerValue {
        type Register = super::CNTHP_TVAL;
        const NAME: &'static str = "TimerValue";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Saved Program Status Register (Supervisor mode)
#[allow(non_camel_case_types)]
pub struct SPSR_svc;
impl Register for SPSR_svc {
    const NAME: &'static str = "SPSR_svc";
    const LEN: usize = 64;
}

/// SPSR_svc register fields
pub mod spsr_svc {
    use crate::core::model::proc::RegisterView;

    /// SPSR_svc\[31\]
    pub struct N;
    impl RegisterView for N {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "N";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[30\]
    pub struct Z;
    impl RegisterView for Z {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "Z";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[29\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "C";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[28\]
    pub struct V;
    impl RegisterView for V {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "V";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[27\]
    pub struct Q;
    impl RegisterView for Q {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "Q";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[15:10\]
    pub struct IT;
    impl RegisterView for IT {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "IT";
        const OFFSET: usize = 10;
        const LEN: usize = 6;
    }

    /// SPSR_svc\[24\]
    pub struct J;
    impl RegisterView for J {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "J";
        const OFFSET: usize = 24;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[23\]
    pub struct SSBS;
    impl RegisterView for SSBS {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "SSBS";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[22\]
    pub struct PAN;
    impl RegisterView for PAN {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "PAN";
        const OFFSET: usize = 22;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[21\]
    pub struct DIT;
    impl RegisterView for DIT {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "DIT";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[20\]
    pub struct IL;
    impl RegisterView for IL {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "IL";
        const OFFSET: usize = 20;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[19:16\]
    pub struct GE;
    impl RegisterView for GE {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "GE";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// SPSR_svc\[9\]
    pub struct E;
    impl RegisterView for E {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "E";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[8\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "A";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[7\]
    pub struct I;
    impl RegisterView for I {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "I";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[6\]
    pub struct F;
    impl RegisterView for F {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "F";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[5\]
    pub struct T;
    impl RegisterView for T {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "T";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// SPSR_svc\[4:0\]
    pub struct M;
    impl RegisterView for M {
        type Register = super::SPSR_svc;
        const NAME: &'static str = "M";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Instruction Set Attribute Register 6
#[allow(non_camel_case_types)]
pub struct ID_ISAR6;
impl Register for ID_ISAR6 {
    const NAME: &'static str = "ID_ISAR6";
    const LEN: usize = 64;
}

/// ID_ISAR6 register fields
pub mod id_isar6 {
    use crate::core::model::proc::RegisterView;

    /// ID_ISAR6\[31:28\]
    pub struct CLRBHB;
    impl RegisterView for CLRBHB {
        type Register = super::ID_ISAR6;
        const NAME: &'static str = "CLRBHB";
        const OFFSET: usize = 28;
        const LEN: usize = 4;
    }

    /// ID_ISAR6\[27:24\]
    pub struct I8MM;
    impl RegisterView for I8MM {
        type Register = super::ID_ISAR6;
        const NAME: &'static str = "I8MM";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// ID_ISAR6\[23:20\]
    pub struct BF16;
    impl RegisterView for BF16 {
        type Register = super::ID_ISAR6;
        const NAME: &'static str = "BF16";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// ID_ISAR6\[19:16\]
    pub struct SPECRES;
    impl RegisterView for SPECRES {
        type Register = super::ID_ISAR6;
        const NAME: &'static str = "SPECRES";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// ID_ISAR6\[15:12\]
    pub struct SB;
    impl RegisterView for SB {
        type Register = super::ID_ISAR6;
        const NAME: &'static str = "SB";
        const OFFSET: usize = 12;
        const LEN: usize = 4;
    }

    /// ID_ISAR6\[11:8\]
    pub struct FHM;
    impl RegisterView for FHM {
        type Register = super::ID_ISAR6;
        const NAME: &'static str = "FHM";
        const OFFSET: usize = 8;
        const LEN: usize = 4;
    }

    /// ID_ISAR6\[7:4\]
    pub struct DP;
    impl RegisterView for DP {
        type Register = super::ID_ISAR6;
        const NAME: &'static str = "DP";
        const OFFSET: usize = 4;
        const LEN: usize = 4;
    }

    /// ID_ISAR6\[3:0\]
    pub struct JSCVT;
    impl RegisterView for JSCVT {
        type Register = super::ID_ISAR6;
        const NAME: &'static str = "JSCVT";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Interrupt Controller Binary Point Register 0
#[allow(non_camel_case_types)]
pub struct ICC_BPR0;
impl Register for ICC_BPR0 {
    const NAME: &'static str = "ICC_BPR0";
    const LEN: usize = 64;
}

/// ICC_BPR0 register fields
pub mod icc_bpr0 {
    use crate::core::model::proc::RegisterView;

    /// ICC_BPR0\[2:0\]
    pub struct BinaryPoint;
    impl RegisterView for BinaryPoint {
        type Register = super::ICC_BPR0;
        const NAME: &'static str = "BinaryPoint";
        const OFFSET: usize = 0;
        const LEN: usize = 3;
    }
}

/// Hyp Auxiliary Configuration Register
pub struct HACR;
impl Register for HACR {
    const NAME: &'static str = "HACR";
    const LEN: usize = 64;
}

/// Interrupt Controller Virtual Active Priorities Group 1 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICV_AP1Rn;
impl RegisterArray for ICV_AP1Rn {
    const NAME: &'static str = "ICV_AP1Rn";
    const ELEMS: usize = 3;
    const ELEM_LEN: usize = 64;
}

/// Hyp Auxiliary Control Register
pub struct HACTLR;
impl Register for HACTLR {
    const NAME: &'static str = "HACTLR";
    const LEN: usize = 64;
}

/// Performance Monitors Cycle Count Register
pub struct PMCCNTR;
impl Register for PMCCNTR {
    const NAME: &'static str = "PMCCNTR";
    const LEN: usize = 64;
}

/// PMCCNTR register fields
pub mod pmccntr {
    use crate::core::model::proc::RegisterView;

    /// PMCCNTR\[63:0\]
    pub struct CCNT;
    impl RegisterView for CCNT {
        type Register = super::PMCCNTR;
        const NAME: &'static str = "CCNT";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Interrupt Controller Empty List Register Status Register
#[allow(non_camel_case_types)]
pub struct ICH_ELRSR;
impl Register for ICH_ELRSR {
    const NAME: &'static str = "ICH_ELRSR";
    const LEN: usize = 64;
}

/// Activity Monitors Event Type Registers 0, n = 3 - 0
pub struct AMEVTYPER0n;
impl RegisterArray for AMEVTYPER0n {
    const NAME: &'static str = "AMEVTYPER0n";
    const ELEMS: usize = 3;
    const ELEM_LEN: usize = 64;
}

/// AMEVTYPER0n register fields
pub mod amevtyper0n {
    use crate::core::model::proc::RegisterArrayView;

    /// AMEVTYPER0n\[15:0\]
    #[allow(non_camel_case_types)]
    pub struct evtCount;
    impl RegisterArrayView for evtCount {
        type RegisterArray = super::AMEVTYPER0n;
        const NAME: &'static str = "evtCount";
        const OFFSET: usize = 0;
        const LEN: usize = 16;
    }
}

/// Performance Monitors Event Counter Selection Register
pub struct PMSELR;
impl Register for PMSELR {
    const NAME: &'static str = "PMSELR";
    const LEN: usize = 64;
}

/// PMSELR register fields
pub mod pmselr {
    use crate::core::model::proc::RegisterView;

    /// PMSELR\[4:0\]
    pub struct SEL;
    impl RegisterView for SEL {
        type Register = super::PMSELR;
        const NAME: &'static str = "SEL";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Interrupt Controller Active Priorities Group 1 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICC_AP1Rn;
impl RegisterArray for ICC_AP1Rn {
    const NAME: &'static str = "ICC_AP1Rn";
    const ELEMS: usize = 3;
    const ELEM_LEN: usize = 64;
}

/// Performance Monitors Overflow Flag Status Set register
pub struct PMOVSSET;
impl Register for PMOVSSET {
    const NAME: &'static str = "PMOVSSET";
    const LEN: usize = 64;
}

/// PMOVSSET register fields
pub mod pmovsset {
    use crate::core::model::proc::RegisterView;

    /// PMOVSSET\[31\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::PMOVSSET;
        const NAME: &'static str = "C";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }
}

/// Performance Monitors Event Count Registers, n = 30 - 0
pub struct PMEVCNTRn;
impl RegisterArray for PMEVCNTRn {
    const NAME: &'static str = "PMEVCNTRn";
    const ELEMS: usize = 30;
    const ELEM_LEN: usize = 64;
}

/// PMEVCNTRn register fields
pub mod pmevcntrn {
    use crate::core::model::proc::RegisterArrayView;

    /// PMEVCNTRn\[31:0\]
    pub struct EVCNT;
    impl RegisterArrayView for EVCNT {
        type RegisterArray = super::PMEVCNTRn;
        const NAME: &'static str = "EVCNT";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Performance Monitors Machine Identification Register
pub struct PMMIR;
impl Register for PMMIR {
    const NAME: &'static str = "PMMIR";
    const LEN: usize = 64;
}

/// PMMIR register fields
pub mod pmmir {
    use crate::core::model::proc::RegisterView;

    /// PMMIR\[27:24\]
    pub struct EDGE;
    impl RegisterView for EDGE {
        type Register = super::PMMIR;
        const NAME: &'static str = "EDGE";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// PMMIR\[23:20\]
    pub struct THWIDTH;
    impl RegisterView for THWIDTH {
        type Register = super::PMMIR;
        const NAME: &'static str = "THWIDTH";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// PMMIR\[19:16\]
    #[allow(non_camel_case_types)]
    pub struct BUS_WIDTH;
    impl RegisterView for BUS_WIDTH {
        type Register = super::PMMIR;
        const NAME: &'static str = "BUS_WIDTH";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// PMMIR\[15:8\]
    #[allow(non_camel_case_types)]
    pub struct BUS_SLOTS;
    impl RegisterView for BUS_SLOTS {
        type Register = super::PMMIR;
        const NAME: &'static str = "BUS_SLOTS";
        const OFFSET: usize = 8;
        const LEN: usize = 8;
    }

    /// PMMIR\[7:0\]
    pub struct SLOTS;
    impl RegisterView for SLOTS {
        type Register = super::PMMIR;
        const NAME: &'static str = "SLOTS";
        const OFFSET: usize = 0;
        const LEN: usize = 8;
    }
}

/// Hyp Debug Control Register
pub struct HDCR;
impl Register for HDCR {
    const NAME: &'static str = "HDCR";
    const LEN: usize = 64;
}

/// HDCR register fields
pub mod hdcr {
    use crate::core::model::proc::RegisterView;

    /// HDCR\[29\]
    pub struct HPMFZO;
    impl RegisterView for HPMFZO {
        type Register = super::HDCR;
        const NAME: &'static str = "HPMFZO";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// HDCR\[28\]
    pub struct MTPME;
    impl RegisterView for MTPME {
        type Register = super::HDCR;
        const NAME: &'static str = "MTPME";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// HDCR\[27\]
    pub struct TDCC;
    impl RegisterView for TDCC {
        type Register = super::HDCR;
        const NAME: &'static str = "TDCC";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// HDCR\[26\]
    pub struct HLP;
    impl RegisterView for HLP {
        type Register = super::HDCR;
        const NAME: &'static str = "HLP";
        const OFFSET: usize = 26;
        const LEN: usize = 1;
    }

    /// HDCR\[23\]
    pub struct HCCD;
    impl RegisterView for HCCD {
        type Register = super::HDCR;
        const NAME: &'static str = "HCCD";
        const OFFSET: usize = 23;
        const LEN: usize = 1;
    }

    /// HDCR\[19\]
    pub struct TTRF;
    impl RegisterView for TTRF {
        type Register = super::HDCR;
        const NAME: &'static str = "TTRF";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// HDCR\[17\]
    pub struct HPMD;
    impl RegisterView for HPMD {
        type Register = super::HDCR;
        const NAME: &'static str = "HPMD";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// HDCR\[11\]
    pub struct TDRA;
    impl RegisterView for TDRA {
        type Register = super::HDCR;
        const NAME: &'static str = "TDRA";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// HDCR\[10\]
    pub struct TDOSA;
    impl RegisterView for TDOSA {
        type Register = super::HDCR;
        const NAME: &'static str = "TDOSA";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }

    /// HDCR\[9\]
    pub struct TDA;
    impl RegisterView for TDA {
        type Register = super::HDCR;
        const NAME: &'static str = "TDA";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// HDCR\[8\]
    pub struct TDE;
    impl RegisterView for TDE {
        type Register = super::HDCR;
        const NAME: &'static str = "TDE";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// HDCR\[7\]
    pub struct HPME;
    impl RegisterView for HPME {
        type Register = super::HDCR;
        const NAME: &'static str = "HPME";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// HDCR\[6\]
    pub struct TPM;
    impl RegisterView for TPM {
        type Register = super::HDCR;
        const NAME: &'static str = "TPM";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// HDCR\[5\]
    pub struct TPMCR;
    impl RegisterView for TPMCR {
        type Register = super::HDCR;
        const NAME: &'static str = "TPMCR";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// HDCR\[4:0\]
    pub struct HPMN;
    impl RegisterView for HPMN {
        type Register = super::HDCR;
        const NAME: &'static str = "HPMN";
        const OFFSET: usize = 0;
        const LEN: usize = 5;
    }
}

/// Deferred Interrupt Status Register
pub struct DISR;
impl Register for DISR {
    const NAME: &'static str = "DISR";
    const LEN: usize = 64;
}

/// DISR register fields
pub mod disr {
    use crate::core::model::proc::RegisterView;

    /// DISR\[31\]
    pub struct A;
    impl RegisterView for A {
        type Register = super::DISR;
        const NAME: &'static str = "A";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// DISR\[15:14\]
    pub struct AET;
    impl RegisterView for AET {
        type Register = super::DISR;
        const NAME: &'static str = "AET";
        const OFFSET: usize = 14;
        const LEN: usize = 2;
    }

    /// DISR\[9\]
    pub struct EA;
    impl RegisterView for EA {
        type Register = super::DISR;
        const NAME: &'static str = "EA";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// DISR\[5:0\]
    pub struct DFSC;
    impl RegisterView for DFSC {
        type Register = super::DISR;
        const NAME: &'static str = "DFSC";
        const OFFSET: usize = 0;
        const LEN: usize = 6;
    }

    /// DISR\[12\]
    pub struct ExT;
    impl RegisterView for ExT {
        type Register = super::DISR;
        const NAME: &'static str = "ExT";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// DISR\[3:0\]
    pub struct FS;
    impl RegisterView for FS {
        type Register = super::DISR;
        const NAME: &'static str = "FS";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }

    /// DISR\[9\]
    pub struct LPAE;
    impl RegisterView for LPAE {
        type Register = super::DISR;
        const NAME: &'static str = "LPAE";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// DISR\[5:0\]
    pub struct STATUS;
    impl RegisterView for STATUS {
        type Register = super::DISR;
        const NAME: &'static str = "STATUS";
        const OFFSET: usize = 0;
        const LEN: usize = 6;
    }
}

/// Debug Breakpoint Value Registers, n = 15 - 0
pub struct DBGBVRn;
impl RegisterArray for DBGBVRn {
    const NAME: &'static str = "DBGBVRn";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// DBGBVRn register fields
pub mod dbgbvrn {
    use crate::core::model::proc::RegisterArrayView;

    /// DBGBVRn\[31:0\]
    pub struct ContextID;
    impl RegisterArrayView for ContextID {
        type RegisterArray = super::DBGBVRn;
        const NAME: &'static str = "ContextID";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }

    /// DBGBVRn\[31:2\]
    pub struct VA;
    impl RegisterArrayView for VA {
        type RegisterArray = super::DBGBVRn;
        const NAME: &'static str = "VA";
        const OFFSET: usize = 2;
        const LEN: usize = 30;
    }
}

/// PL0 Read-Only Software Thread ID Register
pub struct TPIDRURO;
impl Register for TPIDRURO {
    const NAME: &'static str = "TPIDRURO";
    const LEN: usize = 64;
}

/// TPIDRURO register fields
pub mod tpidruro {
    use crate::core::model::proc::RegisterView;

    /// TPIDRURO\[31:0\]
    pub struct TID;
    impl RegisterView for TID {
        type Register = super::TPIDRURO;
        const NAME: &'static str = "TID";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Counter-timer Secure Virtual Timer CompareValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHVS_CVAL;
impl Register for CNTHVS_CVAL {
    const NAME: &'static str = "CNTHVS_CVAL";
    const LEN: usize = 64;
}

/// CNTHVS_CVAL register fields
pub mod cnthvs_cval {
    use crate::core::model::proc::RegisterView;

    /// CNTHVS_CVAL\[63:0\]
    pub struct CompareValue;
    impl RegisterView for CompareValue {
        type Register = super::CNTHVS_CVAL;
        const NAME: &'static str = "CompareValue";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Interrupt Controller List Registers, n = 15 - 0
#[allow(non_camel_case_types)]
pub struct ICH_LRCn;
impl RegisterArray for ICH_LRCn {
    const NAME: &'static str = "ICH_LRCn";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// ICH_LRCn register fields
pub mod ich_lrcn {
    use crate::core::model::proc::RegisterArrayView;

    /// ICH_LRCn\[31:30\]
    pub struct State;
    impl RegisterArrayView for State {
        type RegisterArray = super::ICH_LRCn;
        const NAME: &'static str = "State";
        const OFFSET: usize = 30;
        const LEN: usize = 2;
    }

    /// ICH_LRCn\[29\]
    pub struct HW;
    impl RegisterArrayView for HW {
        type RegisterArray = super::ICH_LRCn;
        const NAME: &'static str = "HW";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// ICH_LRCn\[28\]
    pub struct Group;
    impl RegisterArrayView for Group {
        type RegisterArray = super::ICH_LRCn;
        const NAME: &'static str = "Group";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// ICH_LRCn\[23:16\]
    pub struct Priority;
    impl RegisterArrayView for Priority {
        type RegisterArray = super::ICH_LRCn;
        const NAME: &'static str = "Priority";
        const OFFSET: usize = 16;
        const LEN: usize = 8;
    }

    /// ICH_LRCn\[12:0\]
    #[allow(non_camel_case_types)]
    pub struct pINTID;
    impl RegisterArrayView for pINTID {
        type RegisterArray = super::ICH_LRCn;
        const NAME: &'static str = "pINTID";
        const OFFSET: usize = 0;
        const LEN: usize = 13;
    }
}

/// Hyp Auxiliary Data Fault Status Register
pub struct HADFSR;
impl Register for HADFSR {
    const NAME: &'static str = "HADFSR";
    const LEN: usize = 64;
}

/// Counter-timer Physical Timer Control register
#[allow(non_camel_case_types)]
pub struct CNTP_CTL;
impl Register for CNTP_CTL {
    const NAME: &'static str = "CNTP_CTL";
    const LEN: usize = 64;
}

/// CNTP_CTL register fields
pub mod cntp_ctl {
    use crate::core::model::proc::RegisterView;

    /// CNTP_CTL\[2\]
    pub struct ISTATUS;
    impl RegisterView for ISTATUS {
        type Register = super::CNTP_CTL;
        const NAME: &'static str = "ISTATUS";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// CNTP_CTL\[1\]
    pub struct IMASK;
    impl RegisterView for IMASK {
        type Register = super::CNTP_CTL;
        const NAME: &'static str = "IMASK";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// CNTP_CTL\[0\]
    pub struct ENABLE;
    impl RegisterView for ENABLE {
        type Register = super::CNTP_CTL;
        const NAME: &'static str = "ENABLE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Counter-timer Virtual Timer TimerValue register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHV_TVAL;
impl Register for CNTHV_TVAL {
    const NAME: &'static str = "CNTHV_TVAL";
    const LEN: usize = 64;
}

/// CNTHV_TVAL register fields
pub mod cnthv_tval {
    use crate::core::model::proc::RegisterView;

    /// CNTHV_TVAL\[31:0\]
    pub struct TimerValue;
    impl RegisterView for TimerValue {
        type Register = super::CNTHV_TVAL;
        const NAME: &'static str = "TimerValue";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Interrupt Controller Hyp Control Register
#[allow(non_camel_case_types)]
pub struct ICH_HCR;
impl Register for ICH_HCR {
    const NAME: &'static str = "ICH_HCR";
    const LEN: usize = 64;
}

/// ICH_HCR register fields
pub mod ich_hcr {
    use crate::core::model::proc::RegisterView;

    /// ICH_HCR\[31:27\]
    pub struct EOIcount;
    impl RegisterView for EOIcount {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "EOIcount";
        const OFFSET: usize = 27;
        const LEN: usize = 5;
    }

    /// ICH_HCR\[14\]
    pub struct TDIR;
    impl RegisterView for TDIR {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "TDIR";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[13\]
    pub struct TSEI;
    impl RegisterView for TSEI {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "TSEI";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[12\]
    pub struct TALL1;
    impl RegisterView for TALL1 {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "TALL1";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[11\]
    pub struct TALL0;
    impl RegisterView for TALL0 {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "TALL0";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[10\]
    pub struct TC;
    impl RegisterView for TC {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "TC";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[8\]
    #[allow(non_camel_case_types)]
    pub struct vSGIEOICount;
    impl RegisterView for vSGIEOICount {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "vSGIEOICount";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[7\]
    pub struct VGrp1DIE;
    impl RegisterView for VGrp1DIE {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "VGrp1DIE";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[6\]
    pub struct VGrp1EIE;
    impl RegisterView for VGrp1EIE {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "VGrp1EIE";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[5\]
    pub struct VGrp0DIE;
    impl RegisterView for VGrp0DIE {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "VGrp0DIE";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[4\]
    pub struct VGrp0EIE;
    impl RegisterView for VGrp0EIE {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "VGrp0EIE";
        const OFFSET: usize = 4;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[3\]
    pub struct NPIE;
    impl RegisterView for NPIE {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "NPIE";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[2\]
    pub struct LRENPIE;
    impl RegisterView for LRENPIE {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "LRENPIE";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[1\]
    pub struct UIE;
    impl RegisterView for UIE {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "UIE";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// ICH_HCR\[0\]
    pub struct En;
    impl RegisterView for En {
        type Register = super::ICH_HCR;
        const NAME: &'static str = "En";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Revision ID Register
pub struct REVIDR;
impl Register for REVIDR {
    const NAME: &'static str = "REVIDR";
    const LEN: usize = 64;
}

/// Interrupt Controller Interrupt Group 1 Enable register
#[allow(non_camel_case_types)]
pub struct ICC_IGRPEN1;
impl Register for ICC_IGRPEN1 {
    const NAME: &'static str = "ICC_IGRPEN1";
    const LEN: usize = 64;
}

/// ICC_IGRPEN1 register fields
pub mod icc_igrpen1 {
    use crate::core::model::proc::RegisterView;

    /// ICC_IGRPEN1\[0\]
    pub struct Enable;
    impl RegisterView for Enable {
        type Register = super::ICC_IGRPEN1;
        const NAME: &'static str = "Enable";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Main ID Register
pub struct MIDR;
impl Register for MIDR {
    const NAME: &'static str = "MIDR";
    const LEN: usize = 64;
}

/// MIDR register fields
pub mod midr {
    use crate::core::model::proc::RegisterView;

    /// MIDR\[31:24\]
    pub struct Implementer;
    impl RegisterView for Implementer {
        type Register = super::MIDR;
        const NAME: &'static str = "Implementer";
        const OFFSET: usize = 24;
        const LEN: usize = 8;
    }

    /// MIDR\[23:20\]
    pub struct Variant;
    impl RegisterView for Variant {
        type Register = super::MIDR;
        const NAME: &'static str = "Variant";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// MIDR\[19:16\]
    pub struct Architecture;
    impl RegisterView for Architecture {
        type Register = super::MIDR;
        const NAME: &'static str = "Architecture";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// MIDR\[15:4\]
    pub struct PartNum;
    impl RegisterView for PartNum {
        type Register = super::MIDR;
        const NAME: &'static str = "PartNum";
        const OFFSET: usize = 4;
        const LEN: usize = 12;
    }

    /// MIDR\[3:0\]
    pub struct Revision;
    impl RegisterView for Revision {
        type Register = super::MIDR;
        const NAME: &'static str = "Revision";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Activity Monitors Count Enable Set Register 1
pub struct AMCNTENSET1;
impl Register for AMCNTENSET1 {
    const NAME: &'static str = "AMCNTENSET1";
    const LEN: usize = 64;
}

/// Primary Region Remap Register
pub struct PRRR;
impl Register for PRRR {
    const NAME: &'static str = "PRRR";
    const LEN: usize = 64;
}

/// PRRR register fields
pub mod prrr {
    use crate::core::model::proc::RegisterView;

    /// PRRR\[19\]
    pub struct NS1;
    impl RegisterView for NS1 {
        type Register = super::PRRR;
        const NAME: &'static str = "NS1";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// PRRR\[18\]
    pub struct NS0;
    impl RegisterView for NS0 {
        type Register = super::PRRR;
        const NAME: &'static str = "NS0";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// PRRR\[17\]
    pub struct DS1;
    impl RegisterView for DS1 {
        type Register = super::PRRR;
        const NAME: &'static str = "DS1";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// PRRR\[16\]
    pub struct DS0;
    impl RegisterView for DS0 {
        type Register = super::PRRR;
        const NAME: &'static str = "DS0";
        const OFFSET: usize = 16;
        const LEN: usize = 1;
    }
}

/// Auxiliary ID Register
pub struct AIDR;
impl Register for AIDR {
    const NAME: &'static str = "AIDR";
    const LEN: usize = 64;
}

/// Performance Monitors Interrupt Enable Clear register
pub struct PMINTENCLR;
impl Register for PMINTENCLR {
    const NAME: &'static str = "PMINTENCLR";
    const LEN: usize = 64;
}

/// PMINTENCLR register fields
pub mod pmintenclr {
    use crate::core::model::proc::RegisterView;

    /// PMINTENCLR\[31\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::PMINTENCLR;
        const NAME: &'static str = "C";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }
}

/// Debug Status and Control Register, External View
pub struct DBGDSCRext;
impl Register for DBGDSCRext {
    const NAME: &'static str = "DBGDSCRext";
    const LEN: usize = 64;
}

/// DBGDSCRext register fields
pub mod dbgdscrext {
    use crate::core::model::proc::RegisterView;

    /// DBGDSCRext\[31\]
    pub struct TFO;
    impl RegisterView for TFO {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "TFO";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[30\]
    pub struct RXfull;
    impl RegisterView for RXfull {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "RXfull";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[29\]
    pub struct TXfull;
    impl RegisterView for TXfull {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "TXfull";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[27\]
    pub struct RXO;
    impl RegisterView for RXO {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "RXO";
        const OFFSET: usize = 27;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[26\]
    pub struct TXU;
    impl RegisterView for TXU {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "TXU";
        const OFFSET: usize = 26;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[23:22\]
    pub struct INTdis;
    impl RegisterView for INTdis {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "INTdis";
        const OFFSET: usize = 22;
        const LEN: usize = 2;
    }

    /// DBGDSCRext\[21\]
    pub struct TDA;
    impl RegisterView for TDA {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "TDA";
        const OFFSET: usize = 21;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[19\]
    pub struct SC2;
    impl RegisterView for SC2 {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "SC2";
        const OFFSET: usize = 19;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[18\]
    pub struct NS;
    impl RegisterView for NS {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "NS";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[17\]
    pub struct SPNIDdis;
    impl RegisterView for SPNIDdis {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "SPNIDdis";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[16\]
    pub struct SPIDdis;
    impl RegisterView for SPIDdis {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "SPIDdis";
        const OFFSET: usize = 16;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[15\]
    pub struct MDBGen;
    impl RegisterView for MDBGen {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "MDBGen";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[14\]
    pub struct HDE;
    impl RegisterView for HDE {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "HDE";
        const OFFSET: usize = 14;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[12\]
    pub struct UDCCdis;
    impl RegisterView for UDCCdis {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "UDCCdis";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[6\]
    pub struct ERR;
    impl RegisterView for ERR {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "ERR";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// DBGDSCRext\[5:2\]
    pub struct MOE;
    impl RegisterView for MOE {
        type Register = super::DBGDSCRext;
        const NAME: &'static str = "MOE";
        const OFFSET: usize = 2;
        const LEN: usize = 4;
    }
}

/// Debug Status and Control Register, Internal View
pub struct DBGDSCRint;
impl Register for DBGDSCRint {
    const NAME: &'static str = "DBGDSCRint";
    const LEN: usize = 64;
}

/// DBGDSCRint register fields
pub mod dbgdscrint {
    use crate::core::model::proc::RegisterView;

    /// DBGDSCRint\[30\]
    pub struct RXfull;
    impl RegisterView for RXfull {
        type Register = super::DBGDSCRint;
        const NAME: &'static str = "RXfull";
        const OFFSET: usize = 30;
        const LEN: usize = 1;
    }

    /// DBGDSCRint\[29\]
    pub struct TXfull;
    impl RegisterView for TXfull {
        type Register = super::DBGDSCRint;
        const NAME: &'static str = "TXfull";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// DBGDSCRint\[18\]
    pub struct NS;
    impl RegisterView for NS {
        type Register = super::DBGDSCRint;
        const NAME: &'static str = "NS";
        const OFFSET: usize = 18;
        const LEN: usize = 1;
    }

    /// DBGDSCRint\[17\]
    pub struct SPNIDdis;
    impl RegisterView for SPNIDdis {
        type Register = super::DBGDSCRint;
        const NAME: &'static str = "SPNIDdis";
        const OFFSET: usize = 17;
        const LEN: usize = 1;
    }

    /// DBGDSCRint\[16\]
    pub struct SPIDdis;
    impl RegisterView for SPIDdis {
        type Register = super::DBGDSCRint;
        const NAME: &'static str = "SPIDdis";
        const OFFSET: usize = 16;
        const LEN: usize = 1;
    }

    /// DBGDSCRint\[15\]
    pub struct MDBGen;
    impl RegisterView for MDBGen {
        type Register = super::DBGDSCRint;
        const NAME: &'static str = "MDBGen";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// DBGDSCRint\[12\]
    pub struct UDCCdis;
    impl RegisterView for UDCCdis {
        type Register = super::DBGDSCRint;
        const NAME: &'static str = "UDCCdis";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// DBGDSCRint\[5:2\]
    pub struct MOE;
    impl RegisterView for MOE {
        type Register = super::DBGDSCRint;
        const NAME: &'static str = "MOE";
        const OFFSET: usize = 2;
        const LEN: usize = 4;
    }
}

/// Virtual SError Exception Syndrome Register
pub struct VDFSR;
impl Register for VDFSR {
    const NAME: &'static str = "VDFSR";
    const LEN: usize = 64;
}

/// VDFSR register fields
pub mod vdfsr {
    use crate::core::model::proc::RegisterView;

    /// VDFSR\[15:14\]
    pub struct AET;
    impl RegisterView for AET {
        type Register = super::VDFSR;
        const NAME: &'static str = "AET";
        const OFFSET: usize = 14;
        const LEN: usize = 2;
    }

    /// VDFSR\[12\]
    pub struct ExT;
    impl RegisterView for ExT {
        type Register = super::VDFSR;
        const NAME: &'static str = "ExT";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }
}

/// Activity Monitors Count Enable Set Register 0
pub struct AMCNTENSET0;
impl Register for AMCNTENSET0 {
    const NAME: &'static str = "AMCNTENSET0";
    const LEN: usize = 64;
}

/// Debug Saved Process State Register 2
pub struct DSPSR2;
impl Register for DSPSR2 {
    const NAME: &'static str = "DSPSR2";
    const LEN: usize = 64;
}

/// DSPSR2 register fields
pub mod dspsr2 {
    use crate::core::model::proc::RegisterView;

    /// DSPSR2\[1\]
    pub struct PPEND;
    impl RegisterView for PPEND {
        type Register = super::DSPSR2;
        const NAME: &'static str = "PPEND";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }
}

/// Hyp System Trap Register
pub struct HSTR;
impl Register for HSTR {
    const NAME: &'static str = "HSTR";
    const LEN: usize = 64;
}

/// HSTR register fields
pub mod hstr {
    use crate::core::model::proc::RegisterView;

    /// HSTR\[15\]
    pub struct T15;
    impl RegisterView for T15 {
        type Register = super::HSTR;
        const NAME: &'static str = "T15";
        const OFFSET: usize = 15;
        const LEN: usize = 1;
    }

    /// HSTR\[13\]
    pub struct T13;
    impl RegisterView for T13 {
        type Register = super::HSTR;
        const NAME: &'static str = "T13";
        const OFFSET: usize = 13;
        const LEN: usize = 1;
    }

    /// HSTR\[12\]
    pub struct T12;
    impl RegisterView for T12 {
        type Register = super::HSTR;
        const NAME: &'static str = "T12";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// HSTR\[11\]
    pub struct T11;
    impl RegisterView for T11 {
        type Register = super::HSTR;
        const NAME: &'static str = "T11";
        const OFFSET: usize = 11;
        const LEN: usize = 1;
    }

    /// HSTR\[10\]
    pub struct T10;
    impl RegisterView for T10 {
        type Register = super::HSTR;
        const NAME: &'static str = "T10";
        const OFFSET: usize = 10;
        const LEN: usize = 1;
    }

    /// HSTR\[9\]
    pub struct T9;
    impl RegisterView for T9 {
        type Register = super::HSTR;
        const NAME: &'static str = "T9";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// HSTR\[8\]
    pub struct T8;
    impl RegisterView for T8 {
        type Register = super::HSTR;
        const NAME: &'static str = "T8";
        const OFFSET: usize = 8;
        const LEN: usize = 1;
    }

    /// HSTR\[7\]
    pub struct T7;
    impl RegisterView for T7 {
        type Register = super::HSTR;
        const NAME: &'static str = "T7";
        const OFFSET: usize = 7;
        const LEN: usize = 1;
    }

    /// HSTR\[6\]
    pub struct T6;
    impl RegisterView for T6 {
        type Register = super::HSTR;
        const NAME: &'static str = "T6";
        const OFFSET: usize = 6;
        const LEN: usize = 1;
    }

    /// HSTR\[5\]
    pub struct T5;
    impl RegisterView for T5 {
        type Register = super::HSTR;
        const NAME: &'static str = "T5";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// HSTR\[3\]
    pub struct T3;
    impl RegisterView for T3 {
        type Register = super::HSTR;
        const NAME: &'static str = "T3";
        const OFFSET: usize = 3;
        const LEN: usize = 1;
    }

    /// HSTR\[2\]
    pub struct T2;
    impl RegisterView for T2 {
        type Register = super::HSTR;
        const NAME: &'static str = "T2";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// HSTR\[1\]
    pub struct T1;
    impl RegisterView for T1 {
        type Register = super::HSTR;
        const NAME: &'static str = "T1";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// HSTR\[0\]
    pub struct T0;
    impl RegisterView for T0 {
        type Register = super::HSTR;
        const NAME: &'static str = "T0";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Interrupt Group 0 Enable register
#[allow(non_camel_case_types)]
pub struct ICC_IGRPEN0;
impl Register for ICC_IGRPEN0 {
    const NAME: &'static str = "ICC_IGRPEN0";
    const LEN: usize = 64;
}

/// ICC_IGRPEN0 register fields
pub mod icc_igrpen0 {
    use crate::core::model::proc::RegisterView;

    /// ICC_IGRPEN0\[0\]
    pub struct Enable;
    impl RegisterView for Enable {
        type Register = super::ICC_IGRPEN0;
        const NAME: &'static str = "Enable";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Auxiliary Memory Attribute Indirection Register 1
pub struct AMAIR1;
impl Register for AMAIR1 {
    const NAME: &'static str = "AMAIR1";
    const LEN: usize = 64;
}

/// Hyp IPA Fault Address Register
pub struct HPFAR;
impl Register for HPFAR {
    const NAME: &'static str = "HPFAR";
    const LEN: usize = 64;
}

/// HPFAR register fields
pub mod hpfar {
    use crate::core::model::proc::RegisterView;

    /// HPFAR\[31:4\]
    pub struct FIPA;
    impl RegisterView for FIPA {
        type Register = super::HPFAR;
        const NAME: &'static str = "FIPA";
        const OFFSET: usize = 4;
        const LEN: usize = 28;
    }
}

/// Counter-timer Physical Timer CompareValue register
#[allow(non_camel_case_types)]
pub struct CNTP_CVAL;
impl Register for CNTP_CVAL {
    const NAME: &'static str = "CNTP_CVAL";
    const LEN: usize = 64;
}

/// CNTP_CVAL register fields
pub mod cntp_cval {
    use crate::core::model::proc::RegisterView;

    /// CNTP_CVAL\[63:0\]
    pub struct CompareValue;
    impl RegisterView for CompareValue {
        type Register = super::CNTP_CVAL;
        const NAME: &'static str = "CompareValue";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Counter-timer Secure Physical Timer CompareValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHPS_CVAL;
impl Register for CNTHPS_CVAL {
    const NAME: &'static str = "CNTHPS_CVAL";
    const LEN: usize = 64;
}

/// CNTHPS_CVAL register fields
pub mod cnthps_cval {
    use crate::core::model::proc::RegisterView;

    /// CNTHPS_CVAL\[63:0\]
    pub struct CompareValue;
    impl RegisterView for CompareValue {
        type Register = super::CNTHPS_CVAL;
        const NAME: &'static str = "CompareValue";
        const OFFSET: usize = 0;
        const LEN: usize = 64;
    }
}

/// Activity Monitors Event Type Registers 1, n = 15 - 0
pub struct AMEVTYPER1n;
impl RegisterArray for AMEVTYPER1n {
    const NAME: &'static str = "AMEVTYPER1n";
    const ELEMS: usize = 15;
    const ELEM_LEN: usize = 64;
}

/// AMEVTYPER1n register fields
pub mod amevtyper1n {
    use crate::core::model::proc::RegisterArrayView;

    /// AMEVTYPER1n\[15:0\]
    #[allow(non_camel_case_types)]
    pub struct evtCount;
    impl RegisterArrayView for evtCount {
        type RegisterArray = super::AMEVTYPER1n;
        const NAME: &'static str = "evtCount";
        const OFFSET: usize = 0;
        const LEN: usize = 16;
    }
}

/// Debug Power Control Register
pub struct DBGPRCR;
impl Register for DBGPRCR {
    const NAME: &'static str = "DBGPRCR";
    const LEN: usize = 64;
}

/// DBGPRCR register fields
pub mod dbgprcr {
    use crate::core::model::proc::RegisterView;

    /// DBGPRCR\[0\]
    pub struct CORENPDRQ;
    impl RegisterView for CORENPDRQ {
        type Register = super::DBGPRCR;
        const NAME: &'static str = "CORENPDRQ";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Cache Size Selection Register
pub struct CSSELR;
impl Register for CSSELR {
    const NAME: &'static str = "CSSELR";
    const LEN: usize = 64;
}

/// CSSELR register fields
pub mod csselr {
    use crate::core::model::proc::RegisterView;

    /// CSSELR\[3:1\]
    pub struct Level;
    impl RegisterView for Level {
        type Register = super::CSSELR;
        const NAME: &'static str = "Level";
        const OFFSET: usize = 1;
        const LEN: usize = 3;
    }

    /// CSSELR\[0\]
    pub struct InD;
    impl RegisterView for InD {
        type Register = super::CSSELR;
        const NAME: &'static str = "InD";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Interrupt Controller Hyp Active Priorities Group 1 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICH_AP1Rn;
impl RegisterArray for ICH_AP1Rn {
    const NAME: &'static str = "ICH_AP1Rn";
    const ELEMS: usize = 3;
    const ELEM_LEN: usize = 64;
}

/// Translation Table Base Register 1
pub struct TTBR1;
impl Register for TTBR1 {
    const NAME: &'static str = "TTBR1";
    const LEN: usize = 64;
}

/// TTBR1 register fields
pub mod ttbr1 {
    use crate::core::model::proc::RegisterView;

    /// TTBR1\[31:7\]
    pub struct TTB1;
    impl RegisterView for TTB1 {
        type Register = super::TTBR1;
        const NAME: &'static str = "TTB1";
        const OFFSET: usize = 7;
        const LEN: usize = 25;
    }

    /// TTBR1\[0\]
    pub struct IRGN;
    impl RegisterView for IRGN {
        type Register = super::TTBR1;
        const NAME: &'static str = "IRGN";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }

    /// TTBR1\[5\]
    pub struct NOS;
    impl RegisterView for NOS {
        type Register = super::TTBR1;
        const NAME: &'static str = "NOS";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// TTBR1\[4:3\]
    pub struct RGN;
    impl RegisterView for RGN {
        type Register = super::TTBR1;
        const NAME: &'static str = "RGN";
        const OFFSET: usize = 3;
        const LEN: usize = 2;
    }

    /// TTBR1\[2\]
    pub struct IMP;
    impl RegisterView for IMP {
        type Register = super::TTBR1;
        const NAME: &'static str = "IMP";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// TTBR1\[1\]
    pub struct S;
    impl RegisterView for S {
        type Register = super::TTBR1;
        const NAME: &'static str = "S";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// TTBR1\[55:48\]
    pub struct ASID;
    impl RegisterView for ASID {
        type Register = super::TTBR1;
        const NAME: &'static str = "ASID";
        const OFFSET: usize = 48;
        const LEN: usize = 8;
    }

    /// TTBR1\[47:1\]
    pub struct BADDR;
    impl RegisterView for BADDR {
        type Register = super::TTBR1;
        const NAME: &'static str = "BADDR";
        const OFFSET: usize = 1;
        const LEN: usize = 47;
    }

    /// TTBR1\[0\]
    pub struct CnP;
    impl RegisterView for CnP {
        type Register = super::TTBR1;
        const NAME: &'static str = "CnP";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Selected Error Record Control Register 2
pub struct ERXCTLR2;
impl Register for ERXCTLR2 {
    const NAME: &'static str = "ERXCTLR2";
    const LEN: usize = 64;
}

/// ERXCTLR2 register fields
pub mod erxctlr2 {
    use crate::core::model::proc::RegisterView;

    /// ERXCTLR2\[31:0\]
    pub struct ERRnCTLRhi;
    impl RegisterView for ERRnCTLRhi {
        type Register = super::ERXCTLR2;
        const NAME: &'static str = "ERRnCTLRhi";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Performance Monitors Interrupt Enable Set register
pub struct PMINTENSET;
impl Register for PMINTENSET {
    const NAME: &'static str = "PMINTENSET";
    const LEN: usize = 64;
}

/// PMINTENSET register fields
pub mod pmintenset {
    use crate::core::model::proc::RegisterView;

    /// PMINTENSET\[31\]
    pub struct C;
    impl RegisterView for C {
        type Register = super::PMINTENSET;
        const NAME: &'static str = "C";
        const OFFSET: usize = 31;
        const LEN: usize = 1;
    }
}

/// Cache Level ID Register
pub struct CLIDR;
impl Register for CLIDR {
    const NAME: &'static str = "CLIDR";
    const LEN: usize = 64;
}

/// CLIDR register fields
pub mod clidr {
    use crate::core::model::proc::RegisterView;

    /// CLIDR\[31:30\]
    pub struct ICB;
    impl RegisterView for ICB {
        type Register = super::CLIDR;
        const NAME: &'static str = "ICB";
        const OFFSET: usize = 30;
        const LEN: usize = 2;
    }

    /// CLIDR\[29:27\]
    pub struct LoUU;
    impl RegisterView for LoUU {
        type Register = super::CLIDR;
        const NAME: &'static str = "LoUU";
        const OFFSET: usize = 27;
        const LEN: usize = 3;
    }

    /// CLIDR\[26:24\]
    pub struct LoC;
    impl RegisterView for LoC {
        type Register = super::CLIDR;
        const NAME: &'static str = "LoC";
        const OFFSET: usize = 24;
        const LEN: usize = 3;
    }

    /// CLIDR\[23:21\]
    pub struct LoUIS;
    impl RegisterView for LoUIS {
        type Register = super::CLIDR;
        const NAME: &'static str = "LoUIS";
        const OFFSET: usize = 21;
        const LEN: usize = 3;
    }
}

/// Debug OS Lock Access Register
pub struct DBGOSLAR;
impl Register for DBGOSLAR {
    const NAME: &'static str = "DBGOSLAR";
    const LEN: usize = 64;
}

/// DBGOSLAR register fields
pub mod dbgoslar {
    use crate::core::model::proc::RegisterView;

    /// DBGOSLAR\[31:0\]
    pub struct OSLA;
    impl RegisterView for OSLA {
        type Register = super::DBGOSLAR;
        const NAME: &'static str = "OSLA";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Counter-timer Secure Physical Timer Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHPS_CTL;
impl Register for CNTHPS_CTL {
    const NAME: &'static str = "CNTHPS_CTL";
    const LEN: usize = 64;
}

/// CNTHPS_CTL register fields
pub mod cnthps_ctl {
    use crate::core::model::proc::RegisterView;

    /// CNTHPS_CTL\[2\]
    pub struct ISTATUS;
    impl RegisterView for ISTATUS {
        type Register = super::CNTHPS_CTL;
        const NAME: &'static str = "ISTATUS";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// CNTHPS_CTL\[1\]
    pub struct IMASK;
    impl RegisterView for IMASK {
        type Register = super::CNTHPS_CTL;
        const NAME: &'static str = "IMASK";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// CNTHPS_CTL\[0\]
    pub struct ENABLE;
    impl RegisterView for ENABLE {
        type Register = super::CNTHPS_CTL;
        const NAME: &'static str = "ENABLE";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Instruction Fault Status Register
pub struct IFSR;
impl Register for IFSR {
    const NAME: &'static str = "IFSR";
    const LEN: usize = 64;
}

/// IFSR register fields
pub mod ifsr {
    use crate::core::model::proc::RegisterView;

    /// IFSR\[16\]
    pub struct FnV;
    impl RegisterView for FnV {
        type Register = super::IFSR;
        const NAME: &'static str = "FnV";
        const OFFSET: usize = 16;
        const LEN: usize = 1;
    }

    /// IFSR\[12\]
    pub struct ExT;
    impl RegisterView for ExT {
        type Register = super::IFSR;
        const NAME: &'static str = "ExT";
        const OFFSET: usize = 12;
        const LEN: usize = 1;
    }

    /// IFSR\[3:0\]
    pub struct FS;
    impl RegisterView for FS {
        type Register = super::IFSR;
        const NAME: &'static str = "FS";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }

    /// IFSR\[9\]
    pub struct LPAE;
    impl RegisterView for LPAE {
        type Register = super::IFSR;
        const NAME: &'static str = "LPAE";
        const OFFSET: usize = 9;
        const LEN: usize = 1;
    }

    /// IFSR\[5:0\]
    pub struct STATUS;
    impl RegisterView for STATUS {
        type Register = super::IFSR;
        const NAME: &'static str = "STATUS";
        const OFFSET: usize = 0;
        const LEN: usize = 6;
    }
}

/// Cache Type Register
pub struct CTR;
impl Register for CTR {
    const NAME: &'static str = "CTR";
    const LEN: usize = 64;
}

/// CTR register fields
pub mod ctr {
    use crate::core::model::proc::RegisterView;

    /// CTR\[29\]
    pub struct DIC;
    impl RegisterView for DIC {
        type Register = super::CTR;
        const NAME: &'static str = "DIC";
        const OFFSET: usize = 29;
        const LEN: usize = 1;
    }

    /// CTR\[28\]
    pub struct IDC;
    impl RegisterView for IDC {
        type Register = super::CTR;
        const NAME: &'static str = "IDC";
        const OFFSET: usize = 28;
        const LEN: usize = 1;
    }

    /// CTR\[27:24\]
    pub struct CWG;
    impl RegisterView for CWG {
        type Register = super::CTR;
        const NAME: &'static str = "CWG";
        const OFFSET: usize = 24;
        const LEN: usize = 4;
    }

    /// CTR\[23:20\]
    pub struct ERG;
    impl RegisterView for ERG {
        type Register = super::CTR;
        const NAME: &'static str = "ERG";
        const OFFSET: usize = 20;
        const LEN: usize = 4;
    }

    /// CTR\[19:16\]
    pub struct DminLine;
    impl RegisterView for DminLine {
        type Register = super::CTR;
        const NAME: &'static str = "DminLine";
        const OFFSET: usize = 16;
        const LEN: usize = 4;
    }

    /// CTR\[15:14\]
    pub struct L1Ip;
    impl RegisterView for L1Ip {
        type Register = super::CTR;
        const NAME: &'static str = "L1Ip";
        const OFFSET: usize = 14;
        const LEN: usize = 2;
    }

    /// CTR\[3:0\]
    pub struct IminLine;
    impl RegisterView for IminLine {
        type Register = super::CTR;
        const NAME: &'static str = "IminLine";
        const OFFSET: usize = 0;
        const LEN: usize = 4;
    }
}

/// Hyp Data Fault Address Register
pub struct HDFAR;
impl Register for HDFAR {
    const NAME: &'static str = "HDFAR";
    const LEN: usize = 64;
}

/// HDFAR register fields
pub mod hdfar {
    use crate::core::model::proc::RegisterView;

    /// HDFAR\[31:0\]
    pub struct VA;
    impl RegisterView for VA {
        type Register = super::HDFAR;
        const NAME: &'static str = "VA";
        const OFFSET: usize = 0;
        const LEN: usize = 32;
    }
}

/// Translation Table Base Register 0
pub struct TTBR0;
impl Register for TTBR0 {
    const NAME: &'static str = "TTBR0";
    const LEN: usize = 64;
}

/// TTBR0 register fields
pub mod ttbr0 {
    use crate::core::model::proc::RegisterView;

    /// TTBR0\[31:7\]
    pub struct TTB0;
    impl RegisterView for TTB0 {
        type Register = super::TTBR0;
        const NAME: &'static str = "TTB0";
        const OFFSET: usize = 7;
        const LEN: usize = 25;
    }

    /// TTBR0\[0\]
    pub struct IRGN;
    impl RegisterView for IRGN {
        type Register = super::TTBR0;
        const NAME: &'static str = "IRGN";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }

    /// TTBR0\[5\]
    pub struct NOS;
    impl RegisterView for NOS {
        type Register = super::TTBR0;
        const NAME: &'static str = "NOS";
        const OFFSET: usize = 5;
        const LEN: usize = 1;
    }

    /// TTBR0\[4:3\]
    pub struct RGN;
    impl RegisterView for RGN {
        type Register = super::TTBR0;
        const NAME: &'static str = "RGN";
        const OFFSET: usize = 3;
        const LEN: usize = 2;
    }

    /// TTBR0\[2\]
    pub struct IMP;
    impl RegisterView for IMP {
        type Register = super::TTBR0;
        const NAME: &'static str = "IMP";
        const OFFSET: usize = 2;
        const LEN: usize = 1;
    }

    /// TTBR0\[1\]
    pub struct S;
    impl RegisterView for S {
        type Register = super::TTBR0;
        const NAME: &'static str = "S";
        const OFFSET: usize = 1;
        const LEN: usize = 1;
    }

    /// TTBR0\[55:48\]
    pub struct ASID;
    impl RegisterView for ASID {
        type Register = super::TTBR0;
        const NAME: &'static str = "ASID";
        const OFFSET: usize = 48;
        const LEN: usize = 8;
    }

    /// TTBR0\[47:1\]
    pub struct BADDR;
    impl RegisterView for BADDR {
        type Register = super::TTBR0;
        const NAME: &'static str = "BADDR";
        const OFFSET: usize = 1;
        const LEN: usize = 47;
    }

    /// TTBR0\[0\]
    pub struct CnP;
    impl RegisterView for CnP {
        type Register = super::TTBR0;
        const NAME: &'static str = "CnP";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Error Record Select Register
pub struct ERRSELR;
impl Register for ERRSELR {
    const NAME: &'static str = "ERRSELR";
    const LEN: usize = 64;
}

/// ERRSELR register fields
pub mod errselr {
    use crate::core::model::proc::RegisterView;

    /// ERRSELR\[15:0\]
    pub struct SEL;
    impl RegisterView for SEL {
        type Register = super::ERRSELR;
        const NAME: &'static str = "SEL";
        const OFFSET: usize = 0;
        const LEN: usize = 16;
    }
}

/// Hyp Translation Table Base Register
pub struct HTTBR;
impl Register for HTTBR {
    const NAME: &'static str = "HTTBR";
    const LEN: usize = 64;
}

/// HTTBR register fields
pub mod httbr {
    use crate::core::model::proc::RegisterView;

    /// HTTBR\[47:1\]
    pub struct BADDR;
    impl RegisterView for BADDR {
        type Register = super::HTTBR;
        const NAME: &'static str = "BADDR";
        const OFFSET: usize = 1;
        const LEN: usize = 47;
    }

    /// HTTBR\[0\]
    pub struct CnP;
    impl RegisterView for CnP {
        type Register = super::HTTBR;
        const NAME: &'static str = "CnP";
        const OFFSET: usize = 0;
        const LEN: usize = 1;
    }
}

/// Auxiliary Memory Attribute Indirection Register 0
pub struct AMAIR0;
impl Register for AMAIR0 {
    const NAME: &'static str = "AMAIR0";
    const LEN: usize = 64;
}
impl ArmCpu {
    pub fn decl_aarch32_sysregs(&self, decl: &mut ProcDecl<ArmCpu>) {
        decl.reg::<ERXCTLR>()
            .reg::<MVFR2>()
            .reg::<MAIR1>()
            .reg::<ERXMISC5>()
            .reg::<ICH_VMCR>()
            .reg::<DFAR>()
            .reg::<ELR_hyp>()
            .reg::<DBGCLAIMSET>()
            .reg::<DBGOSECCR>()
            .reg::<CCSIDR2>()
            .reg::<VTCR>()
            .reg::<SCTLR>()
            .reg::<ERXMISC4>()
            .reg::<MAIR0>()
            .reg::<FPEXC>()
            .reg::<AIFSR>()
            .reg::<MVFR1>()
            .reg::<ERXMISC6>()
            .reg::<DBGOSDLR>()
            .reg::<CNTVCT>()
            .reg::<ICV_DIR>()
            .reg::<ICC_MCTLR>()
            .reg::<ICC_DIR>()
            .reg::<DBGCLAIMCLR>()
            .reg::<TRFCR>()
            .reg::<ERXFR>()
            .reg::<CNTHP_CTL>()
            .reg::<FPSID>()
            .reg::<HSR>()
            .reg::<DBGDTRRXint>()
            .reg::<DBGDTRRXext>()
            .reg::<ERXMISC7>()
            .reg::<MVFR0>()
            .reg::<CNTHP_CVAL>()
            .reg::<MPIDR>()
            .reg::<JIDR>()
            .reg::<ERXMISC3>()
            .reg::<PMXEVCNTR>()
            .reg::<SDER>()
            .reg::<PMOVSR>()
            .reg::<HAMAIR1>()
            .reg::<HAMAIR0>()
            .reg::<PMCCFILTR>()
            .reg::<ICC_ASGI1R>()
            .reg::<DBGDEVID1>()
            .reg::<PMXEVTYPER>()
            .reg::<RVBAR>()
            .reg::<ERXMISC2>()
            .reg::<PAR>()
            .reg::<ERXMISC0>()
            .reg::<ERXADDR2>()
            .reg::<ICH_MISR>()
            .reg::<CONTEXTIDR>()
            .reg::<CNTPCT>()
            .reg::<CNTV_TVAL>()
            .reg_array::<DBGBCRn>()
            .reg::<CNTVCTSS>()
            .reg::<SPSR_mon>()
            .reg::<TPIDRURW>()
            .reg::<DBGDRAR>()
            .reg_array::<DBGBXVRn>()
            .reg::<ERXADDR>()
            .reg::<ICV_PMR>()
            .reg::<CNTKCTL>()
            .reg::<ICC_SRE>()
            .reg::<ICC_PMR>()
            .reg::<DBGDEVID2>()
            .reg::<RMR>()
            .reg::<CNTFRQ>()
            .reg::<ERXMISC1>()
            .reg::<CPACR>()
            .reg::<HCR2>()
            .reg::<ICC_HPPIR1>()
            .reg::<PMCEID2>()
            .reg::<ID_MMFR5>()
            .reg::<HACTLR2>()
            .reg::<ICC_MGRPEN1>()
            .reg::<ERXSTATUS>()
            .reg_array::<DBGWVRn>()
            .reg::<ICC_SGI0R>()
            .reg::<AMCNTENCLR1>()
            .reg::<HAIFSR>()
            .reg::<DBGWFAR>()
            .reg::<AMCNTENCLR0>()
            .reg::<ISR>()
            .reg::<ERRIDR>()
            .reg::<NSACR>()
            .reg::<HTRFCR>()
            .reg::<ID_MMFR4>()
            .reg::<PMCEID3>()
            .reg::<ICC_MSRE>()
            .reg::<PMCR>()
            .reg::<ICC_HPPIR0>()
            .reg::<PMCEID1>()
            .reg::<TTBCR2>()
            .reg::<ICV_CTLR>()
            .reg::<CNTHPS_TVAL>()
            .reg::<ERXFR2>()
            .reg::<FPSCR>()
            .reg::<HMAIR1>()
            .reg::<CNTP_TVAL>()
            .reg::<ICH_EISR>()
            .reg::<HMAIR0>()
            .reg::<VDISR>()
            .reg::<TPIDRPRW>()
            .reg::<HSCTLR>()
            .reg::<HTCR>()
            .reg::<SPSR_und>()
            .reg::<PMCEID0>()
            .reg_array::<AMEVCNTR0n>()
            .reg_array::<AMEVCNTR1n>()
            .reg::<DBGDEVID>()
            .reg::<ID_MMFR3>()
            .reg::<DBGDSAR>()
            .reg::<AMCFGR>()
            .reg::<DFSR>()
            .reg::<HIFAR>()
            .reg::<SPSR_hyp>()
            .reg::<MVBAR>()
            .reg::<ID_AFR0>()
            .reg::<SPSR>()
            .reg::<VBAR>()
            .reg::<DBGDCCINT>()
            .reg::<ID_MMFR2>()
            .reg::<AMUSERENR>()
            .reg::<ID_MMFR0>()
            .reg::<VMPIDR>()
            .reg::<ICC_EOIR0>()
            .reg::<SDCR>()
            .reg::<CNTV_CTL>()
            .reg::<ICV_EOIR1>()
            .reg::<JOSCR>()
            .reg::<JMCR>()
            .reg::<DBGVCR>()
            .reg::<ICH_VTR>()
            .reg::<TTBCR>()
            .reg::<CCSIDR>()
            .reg::<DSPSR>()
            .reg::<ICV_EOIR0>()
            .reg::<ICC_SGI1R>()
            .reg::<ICC_CTLR>()
            .reg::<CNTHV_CVAL>()
            .reg::<ICC_EOIR1>()
            .reg::<CNTHVS_TVAL>()
            .reg::<ID_MMFR1>()
            .reg::<ICV_IAR1>()
            .reg::<ICV_HPPIR1>()
            .reg::<PMCNTENCLR>()
            .reg::<ID_ISAR2>()
            .reg::<PMSWINC>()
            .reg::<ICV_IGRPEN1>()
            .reg::<ICV_IGRPEN0>()
            .reg::<NMRR>()
            .reg::<VTTBR>()
            .reg::<ID_PFR2>()
            .reg::<ID_ISAR3>()
            .reg::<ICV_HPPIR0>()
            .reg_array::<PMEVTYPERn>()
            .reg::<CNTVOFF>()
            .reg::<ADFSR>()
            .reg::<ICV_IAR0>()
            .reg::<ID_DFR1>()
            .reg::<ICV_BPR0>()
            .reg::<HVBAR>()
            .reg::<VPIDR>()
            .reg_array::<ICV_AP0Rn>()
            .reg::<ACTLR>()
            .reg::<CNTV_CVAL>()
            .reg::<ID_ISAR1>()
            .reg::<DBGDIDR>()
            .reg::<CNTHCTL>()
            .reg::<ID_PFR0>()
            .reg::<ACTLR2>()
            .reg_array::<ICC_AP0Rn>()
            .reg::<CNTPCTSS>()
            .reg::<DBGAUTHSTATUS>()
            .reg::<CNTHVS_CTL>()
            .reg::<IFAR>()
            .reg::<PMUSERENR>()
            .reg::<ID_PFR1>()
            .reg::<DBGOSLSR>()
            .reg::<ID_ISAR0>()
            .reg::<PMCNTENSET>()
            .reg::<ICV_BPR1>()
            .reg::<HCPTR>()
            .reg::<ID_DFR0>()
            .reg::<HTPIDR>()
            .reg::<ICC_IAR0>()
            .reg_array::<ICH_AP0Rn>()
            .reg::<ICC_RPR>()
            .reg::<ID_ISAR4>()
            .reg::<HRMR>()
            .reg_array::<DBGWCRn>()
            .reg::<ICV_RPR>()
            .reg::<TCMTR>()
            .reg::<DLR>()
            .reg::<AMCGCR>()
            .reg_array::<ICH_LRn>()
            .reg::<ID_ISAR5>()
            .reg::<ICC_IAR1>()
            .reg::<FCSEIDR>()
            .reg::<ICC_BPR1>()
            .reg::<AMCR>()
            .reg::<CNTHV_CTL>()
            .reg::<SCR>()
            .reg::<DBGDTRTXint>()
            .reg::<DBGDTRTXext>()
            .reg::<HCR>()
            .reg::<TLBTR>()
            .reg::<DACR>()
            .reg::<ICC_HSRE>()
            .reg::<CNTHP_TVAL>()
            .reg::<SPSR_svc>()
            .reg::<ID_ISAR6>()
            .reg::<ICC_BPR0>()
            .reg::<HACR>()
            .reg_array::<ICV_AP1Rn>()
            .reg::<HACTLR>()
            .reg::<PMCCNTR>()
            .reg::<ICH_ELRSR>()
            .reg_array::<AMEVTYPER0n>()
            .reg::<PMSELR>()
            .reg_array::<ICC_AP1Rn>()
            .reg::<PMOVSSET>()
            .reg_array::<PMEVCNTRn>()
            .reg::<PMMIR>()
            .reg::<HDCR>()
            .reg::<DISR>()
            .reg_array::<DBGBVRn>()
            .reg::<TPIDRURO>()
            .reg::<CNTHVS_CVAL>()
            .reg_array::<ICH_LRCn>()
            .reg::<HADFSR>()
            .reg::<CNTP_CTL>()
            .reg::<CNTHV_TVAL>()
            .reg::<ICH_HCR>()
            .reg::<REVIDR>()
            .reg::<ICC_IGRPEN1>()
            .reg::<MIDR>()
            .reg::<AMCNTENSET1>()
            .reg::<PRRR>()
            .reg::<AIDR>()
            .reg::<PMINTENCLR>()
            .reg::<DBGDSCRext>()
            .reg::<DBGDSCRint>()
            .reg::<VDFSR>()
            .reg::<AMCNTENSET0>()
            .reg::<DSPSR2>()
            .reg::<HSTR>()
            .reg::<ICC_IGRPEN0>()
            .reg::<AMAIR1>()
            .reg::<HPFAR>()
            .reg::<CNTP_CVAL>()
            .reg::<CNTHPS_CVAL>()
            .reg_array::<AMEVTYPER1n>()
            .reg::<DBGPRCR>()
            .reg::<CSSELR>()
            .reg_array::<ICH_AP1Rn>()
            .reg::<TTBR1>()
            .reg::<ERXCTLR2>()
            .reg::<PMINTENSET>()
            .reg::<CLIDR>()
            .reg::<DBGOSLAR>()
            .reg::<CNTHPS_CTL>()
            .reg::<IFSR>()
            .reg::<CTR>()
            .reg::<HDFAR>()
            .reg::<TTBR0>()
            .reg::<ERRSELR>()
            .reg::<HTTBR>()
            .reg::<AMAIR0>();
    }
}
