use crate::core::model::proc::{ProcDecl, Register, RegisterArray};
use crate::std::arm::cpu::ArmCpu;

/// Counter-timer Physical Secure Timer Control Register
#[allow(non_camel_case_types)]
pub struct CNTPS_CTL_EL1;
impl Register for CNTPS_CTL_EL1 {
	const NAME: &'static str = "CNTPS_CTL_EL1";
	const LEN: usize = 64;
}

/// CNTPS_CTL_EL1 register fields
pub mod cntps_ctl_el1 {
	use crate::core::model::proc::RegisterView;

	/// CNTPS_CTL_EL1\[2\]
	pub struct ISTATUS;
	impl RegisterView for ISTATUS {
		type Register = super::CNTPS_CTL_EL1;
		const NAME: &'static str = "ISTATUS";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTPS_CTL_EL1\[1\]
	pub struct IMASK;
	impl RegisterView for IMASK {
		type Register = super::CNTPS_CTL_EL1;
		const NAME: &'static str = "IMASK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTPS_CTL_EL1\[0\]
	pub struct ENABLE;
	impl RegisterView for ENABLE {
		type Register = super::CNTPS_CTL_EL1;
		const NAME: &'static str = "ENABLE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Selected Error Record Miscellaneous Register 2
#[allow(non_camel_case_types)]
pub struct ERXMISC2_EL1;
impl Register for ERXMISC2_EL1 {
	const NAME: &'static str = "ERXMISC2_EL1";
	const LEN: usize = 64;
}

/// Vector Base Address Register (EL2)
#[allow(non_camel_case_types)]
pub struct VBAR_EL2;
impl Register for VBAR_EL2 {
	const NAME: &'static str = "VBAR_EL2";
	const LEN: usize = 64;
}

/// Interrupt Controller End Of Interrupt Register 1
#[allow(non_camel_case_types)]
pub struct ICC_EOIR1_EL1;
impl Register for ICC_EOIR1_EL1 {
	const NAME: &'static str = "ICC_EOIR1_EL1";
	const LEN: usize = 64;
}

/// ICC_EOIR1_EL1 register fields
pub mod icc_eoir1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_EOIR1_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_EOIR1_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Performance Monitors Event Count Saved Value Register \<n\>, n = 30 - 0
#[allow(non_camel_case_types)]
pub struct PMEVCNTSVRn_EL1;
impl RegisterArray for PMEVCNTSVRn_EL1 {
	const NAME: &'static str = "PMEVCNTSVRn_EL1";
	const ELEMS: usize = 30;
	const ELEM_LEN: usize = 64;
}

/// PMEVCNTSVRn_EL1 register fields
pub mod pmevcntsvrn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// PMEVCNTSVRn_EL1\[63:0\]
	pub struct EVCNT;
	impl RegisterArrayView for EVCNT {
		type RegisterArray = super::PMEVCNTSVRn_EL1;
		const NAME: &'static str = "EVCNT";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Interrupt Controller Deactivate Interrupt Register
#[allow(non_camel_case_types)]
pub struct ICC_DIR_EL1;
impl Register for ICC_DIR_EL1 {
	const NAME: &'static str = "ICC_DIR_EL1";
	const LEN: usize = 64;
}

/// ICC_DIR_EL1 register fields
pub mod icc_dir_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_DIR_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_DIR_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Counter-timer Secure Physical Timer TimerValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHPS_TVAL_EL2;
impl Register for CNTHPS_TVAL_EL2 {
	const NAME: &'static str = "CNTHPS_TVAL_EL2";
	const LEN: usize = 64;
}

/// CNTHPS_TVAL_EL2 register fields
pub mod cnthps_tval_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHPS_TVAL_EL2\[31:0\]
	pub struct TimerValue;
	impl RegisterView for TimerValue {
		type Register = super::CNTHPS_TVAL_EL2;
		const NAME: &'static str = "TimerValue";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Interrupt Controller Binary Point Register 1
#[allow(non_camel_case_types)]
pub struct ICC_BPR1_EL1;
impl Register for ICC_BPR1_EL1 {
	const NAME: &'static str = "ICC_BPR1_EL1";
	const LEN: usize = 64;
}

/// ICC_BPR1_EL1 register fields
pub mod icc_bpr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_BPR1_EL1\[2:0\]
	pub struct BinaryPoint;
	impl RegisterView for BinaryPoint {
		type Register = super::ICC_BPR1_EL1;
		const NAME: &'static str = "BinaryPoint";
		const OFFSET: usize = 0;
		const LEN: usize = 3;
	}
}

/// Trace Buffer Memory Attribute Register
#[allow(non_camel_case_types)]
pub struct TRBMAR_EL1;
impl Register for TRBMAR_EL1 {
	const NAME: &'static str = "TRBMAR_EL1";
	const LEN: usize = 64;
}

/// TRBMAR_EL1 register fields
pub mod trbmar_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRBMAR_EL1\[11:10\]
	pub struct PAS;
	impl RegisterView for PAS {
		type Register = super::TRBMAR_EL1;
		const NAME: &'static str = "PAS";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// TRBMAR_EL1\[9:8\]
	pub struct SH;
	impl RegisterView for SH {
		type Register = super::TRBMAR_EL1;
		const NAME: &'static str = "SH";
		const OFFSET: usize = 8;
		const LEN: usize = 2;
	}

	/// TRBMAR_EL1\[7:0\]
	pub struct Attr;
	impl RegisterView for Attr {
		type Register = super::TRBMAR_EL1;
		const NAME: &'static str = "Attr";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Instrumentation Trace Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct TRCITECR_EL2;
impl Register for TRCITECR_EL2 {
	const NAME: &'static str = "TRCITECR_EL2";
	const LEN: usize = 64;
}

/// TRCITECR_EL2 register fields
pub mod trcitecr_el2 {
	use crate::core::model::proc::RegisterView;

	/// TRCITECR_EL2\[1\]
	pub struct E2E;
	impl RegisterView for E2E {
		type Register = super::TRCITECR_EL2;
		const NAME: &'static str = "E2E";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TRCITECR_EL2\[0\]
	pub struct E0HE;
	impl RegisterView for E0HE {
		type Register = super::TRCITECR_EL2;
		const NAME: &'static str = "E0HE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace ID Register 4
pub struct TRCIDR4;
impl Register for TRCIDR4 {
	const NAME: &'static str = "TRCIDR4";
	const LEN: usize = 64;
}

/// TRCIDR4 register fields
pub mod trcidr4 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR4\[31:28\]
	pub struct NUMVMIDC;
	impl RegisterView for NUMVMIDC {
		type Register = super::TRCIDR4;
		const NAME: &'static str = "NUMVMIDC";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// TRCIDR4\[27:24\]
	pub struct NUMCIDC;
	impl RegisterView for NUMCIDC {
		type Register = super::TRCIDR4;
		const NAME: &'static str = "NUMCIDC";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// TRCIDR4\[23:20\]
	pub struct NUMSSCC;
	impl RegisterView for NUMSSCC {
		type Register = super::TRCIDR4;
		const NAME: &'static str = "NUMSSCC";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// TRCIDR4\[19:16\]
	pub struct NUMRSPAIR;
	impl RegisterView for NUMRSPAIR {
		type Register = super::TRCIDR4;
		const NAME: &'static str = "NUMRSPAIR";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// TRCIDR4\[15:12\]
	pub struct NUMPC;
	impl RegisterView for NUMPC {
		type Register = super::TRCIDR4;
		const NAME: &'static str = "NUMPC";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// TRCIDR4\[8\]
	pub struct SUPPDAC;
	impl RegisterView for SUPPDAC {
		type Register = super::TRCIDR4;
		const NAME: &'static str = "SUPPDAC";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// TRCIDR4\[7:4\]
	pub struct NUMDVC;
	impl RegisterView for NUMDVC {
		type Register = super::TRCIDR4;
		const NAME: &'static str = "NUMDVC";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// TRCIDR4\[3:0\]
	pub struct NUMACPAIRS;
	impl RegisterView for NUMACPAIRS {
		type Register = super::TRCIDR4;
		const NAME: &'static str = "NUMACPAIRS";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Auxiliary Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct ACTLR_EL2;
impl Register for ACTLR_EL2 {
	const NAME: &'static str = "ACTLR_EL2";
	const LEN: usize = 64;
}

/// Activity Monitors Counter Group Configuration Register
#[allow(non_camel_case_types)]
pub struct AMCGCR_EL0;
impl Register for AMCGCR_EL0 {
	const NAME: &'static str = "AMCGCR_EL0";
	const LEN: usize = 64;
}

/// AMCGCR_EL0 register fields
pub mod amcgcr_el0 {
	use crate::core::model::proc::RegisterView;

	/// AMCGCR_EL0\[15:8\]
	pub struct CG1NC;
	impl RegisterView for CG1NC {
		type Register = super::AMCGCR_EL0;
		const NAME: &'static str = "CG1NC";
		const OFFSET: usize = 8;
		const LEN: usize = 8;
	}

	/// AMCGCR_EL0\[7:0\]
	pub struct CG0NC;
	impl RegisterView for CG0NC {
		type Register = super::AMCGCR_EL0;
		const NAME: &'static str = "CG0NC";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Activity Monitors Count Enable Clear Register 1
#[allow(non_camel_case_types)]
pub struct AMCNTENCLR1_EL0;
impl Register for AMCNTENCLR1_EL0 {
	const NAME: &'static str = "AMCNTENCLR1_EL0";
	const LEN: usize = 64;
}

/// Accelerator Data
#[allow(non_camel_case_types)]
pub struct ACCDATA_EL1;
impl Register for ACCDATA_EL1 {
	const NAME: &'static str = "ACCDATA_EL1";
	const LEN: usize = 64;
}

/// ACCDATA_EL1 register fields
pub mod accdata_el1 {
	use crate::core::model::proc::RegisterView;

	/// ACCDATA_EL1\[31:0\]
	pub struct ACCDATA;
	impl RegisterView for ACCDATA {
		type Register = super::ACCDATA_EL1;
		const NAME: &'static str = "ACCDATA";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// AArch32 Instruction Set Attribute Register 1
#[allow(non_camel_case_types)]
pub struct ID_ISAR1_EL1;
impl Register for ID_ISAR1_EL1 {
	const NAME: &'static str = "ID_ISAR1_EL1";
	const LEN: usize = 64;
}

/// ID_ISAR1_EL1 register fields
pub mod id_isar1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_ISAR1_EL1\[31:28\]
	pub struct Jazelle;
	impl RegisterView for Jazelle {
		type Register = super::ID_ISAR1_EL1;
		const NAME: &'static str = "Jazelle";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_ISAR1_EL1\[27:24\]
	pub struct Interwork;
	impl RegisterView for Interwork {
		type Register = super::ID_ISAR1_EL1;
		const NAME: &'static str = "Interwork";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_ISAR1_EL1\[23:20\]
	pub struct Immediate;
	impl RegisterView for Immediate {
		type Register = super::ID_ISAR1_EL1;
		const NAME: &'static str = "Immediate";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_ISAR1_EL1\[19:16\]
	pub struct IfThen;
	impl RegisterView for IfThen {
		type Register = super::ID_ISAR1_EL1;
		const NAME: &'static str = "IfThen";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_ISAR1_EL1\[15:12\]
	pub struct Extend;
	impl RegisterView for Extend {
		type Register = super::ID_ISAR1_EL1;
		const NAME: &'static str = "Extend";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_ISAR1_EL1\[11:8\]
	#[allow(non_camel_case_types)]
	pub struct Except_AR;
	impl RegisterView for Except_AR {
		type Register = super::ID_ISAR1_EL1;
		const NAME: &'static str = "Except_AR";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_ISAR1_EL1\[7:4\]
	pub struct Except;
	impl RegisterView for Except {
		type Register = super::ID_ISAR1_EL1;
		const NAME: &'static str = "Except";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_ISAR1_EL1\[3:0\]
	pub struct Endian;
	impl RegisterView for Endian {
		type Register = super::ID_ISAR1_EL1;
		const NAME: &'static str = "Endian";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// System Performance Monitors Interrupt Enable Set Register
#[allow(non_camel_case_types)]
pub struct SPMINTENSET_EL1;
impl Register for SPMINTENSET_EL1 {
	const NAME: &'static str = "SPMINTENSET_EL1";
	const LEN: usize = 64;
}

/// Trace Virtual Context Identifier Comparator Control Register 1
pub struct TRCVMIDCCTLR1;
impl Register for TRCVMIDCCTLR1 {
	const NAME: &'static str = "TRCVMIDCCTLR1";
	const LEN: usize = 64;
}

/// Debug Watchpoint Value Registers, n = 63 - 0
#[allow(non_camel_case_types)]
pub struct DBGWVRn_EL1;
impl RegisterArray for DBGWVRn_EL1 {
	const NAME: &'static str = "DBGWVRn_EL1";
	const ELEMS: usize = 63;
	const ELEM_LEN: usize = 64;
}

/// DBGWVRn_EL1 register fields
pub mod dbgwvrn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// DBGWVRn_EL1\[63:57\]
	#[allow(non_camel_case_types)]
	pub struct RESS_63_57;
	impl RegisterArrayView for RESS_63_57 {
		type RegisterArray = super::DBGWVRn_EL1;
		const NAME: &'static str = "RESS_63_57";
		const OFFSET: usize = 57;
		const LEN: usize = 7;
	}
	/// DBGWVRn_EL1\[56:53\]
	#[allow(non_camel_case_types)]
	pub struct RESS_63_57_56_53;
	impl RegisterArrayView for RESS_63_57_56_53 {
		type RegisterArray = super::DBGWVRn_EL1;
		const NAME: &'static str = "RESS_63_57_56_53";
		const OFFSET: usize = 53;
		const LEN: usize = 4;
	}
	/// DBGWVRn_EL1\[52:49\]
	#[allow(non_camel_case_types)]
	pub struct RESS_63_57_56_53_52_49;
	impl RegisterArrayView for RESS_63_57_56_53_52_49 {
		type RegisterArray = super::DBGWVRn_EL1;
		const NAME: &'static str = "RESS_63_57_56_53_52_49";
		const OFFSET: usize = 49;
		const LEN: usize = 4;
	}

	/// DBGWVRn_EL1\[56:53\]
	#[allow(non_camel_case_types)]
	pub struct VA_56_53;
	impl RegisterArrayView for VA_56_53 {
		type RegisterArray = super::DBGWVRn_EL1;
		const NAME: &'static str = "VA_56_53";
		const OFFSET: usize = 53;
		const LEN: usize = 4;
	}
	/// DBGWVRn_EL1\[52:49\]
	#[allow(non_camel_case_types)]
	pub struct VA_56_53_52_49;
	impl RegisterArrayView for VA_56_53_52_49 {
		type RegisterArray = super::DBGWVRn_EL1;
		const NAME: &'static str = "VA_56_53_52_49";
		const OFFSET: usize = 49;
		const LEN: usize = 4;
	}
	/// DBGWVRn_EL1\[48:2\]
	#[allow(non_camel_case_types)]
	pub struct VA_56_53_52_49_48_2;
	impl RegisterArrayView for VA_56_53_52_49_48_2 {
		type RegisterArray = super::DBGWVRn_EL1;
		const NAME: &'static str = "VA_56_53_52_49_48_2";
		const OFFSET: usize = 2;
		const LEN: usize = 47;
	}
}

/// Architectural Feature Access Control Register
#[allow(non_camel_case_types)]
pub struct CPACR_EL1;
impl Register for CPACR_EL1 {
	const NAME: &'static str = "CPACR_EL1";
	const LEN: usize = 64;
}

/// CPACR_EL1 register fields
pub mod cpacr_el1 {
	use crate::core::model::proc::RegisterView;

	/// CPACR_EL1\[29\]
	pub struct E0POE;
	impl RegisterView for E0POE {
		type Register = super::CPACR_EL1;
		const NAME: &'static str = "E0POE";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// CPACR_EL1\[28\]
	pub struct TTA;
	impl RegisterView for TTA {
		type Register = super::CPACR_EL1;
		const NAME: &'static str = "TTA";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// CPACR_EL1\[25:24\]
	pub struct SMEN;
	impl RegisterView for SMEN {
		type Register = super::CPACR_EL1;
		const NAME: &'static str = "SMEN";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// CPACR_EL1\[21:20\]
	pub struct FPEN;
	impl RegisterView for FPEN {
		type Register = super::CPACR_EL1;
		const NAME: &'static str = "FPEN";
		const OFFSET: usize = 20;
		const LEN: usize = 2;
	}

	/// CPACR_EL1\[17:16\]
	pub struct ZEN;
	impl RegisterView for ZEN {
		type Register = super::CPACR_EL1;
		const NAME: &'static str = "ZEN";
		const OFFSET: usize = 16;
		const LEN: usize = 2;
	}
}

/// Counter-timer Kernel Control Register
#[allow(non_camel_case_types)]
pub struct CNTKCTL_EL1;
impl Register for CNTKCTL_EL1 {
	const NAME: &'static str = "CNTKCTL_EL1";
	const LEN: usize = 64;
}

/// CNTKCTL_EL1 register fields
pub mod cntkctl_el1 {
	use crate::core::model::proc::RegisterView;

	/// CNTKCTL_EL1\[17\]
	pub struct EVNTIS;
	impl RegisterView for EVNTIS {
		type Register = super::CNTKCTL_EL1;
		const NAME: &'static str = "EVNTIS";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// CNTKCTL_EL1\[9\]
	pub struct EL0PTEN;
	impl RegisterView for EL0PTEN {
		type Register = super::CNTKCTL_EL1;
		const NAME: &'static str = "EL0PTEN";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// CNTKCTL_EL1\[8\]
	pub struct EL0VTEN;
	impl RegisterView for EL0VTEN {
		type Register = super::CNTKCTL_EL1;
		const NAME: &'static str = "EL0VTEN";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// CNTKCTL_EL1\[7:4\]
	pub struct EVNTI;
	impl RegisterView for EVNTI {
		type Register = super::CNTKCTL_EL1;
		const NAME: &'static str = "EVNTI";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// CNTKCTL_EL1\[3\]
	pub struct EVNTDIR;
	impl RegisterView for EVNTDIR {
		type Register = super::CNTKCTL_EL1;
		const NAME: &'static str = "EVNTDIR";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// CNTKCTL_EL1\[2\]
	pub struct EVNTEN;
	impl RegisterView for EVNTEN {
		type Register = super::CNTKCTL_EL1;
		const NAME: &'static str = "EVNTEN";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTKCTL_EL1\[1\]
	pub struct EL0VCTEN;
	impl RegisterView for EL0VCTEN {
		type Register = super::CNTKCTL_EL1;
		const NAME: &'static str = "EL0VCTEN";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTKCTL_EL1\[0\]
	pub struct EL0PCTEN;
	impl RegisterView for EL0PCTEN {
		type Register = super::CNTKCTL_EL1;
		const NAME: &'static str = "EL0PCTEN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Virtual Context Identifier Comparator Control Register 0
pub struct TRCVMIDCCTLR0;
impl Register for TRCVMIDCCTLR0 {
	const NAME: &'static str = "TRCVMIDCCTLR0";
	const LEN: usize = 64;
}

/// Selected Error Record Group Status Register
#[allow(non_camel_case_types)]
pub struct ERXGSR_EL1;
impl Register for ERXGSR_EL1 {
	const NAME: &'static str = "ERXGSR_EL1";
	const LEN: usize = 64;
}

/// Pointer Authentication Key B for Data (bits\[127:64\]) 
#[allow(non_camel_case_types)]
pub struct APDBKeyHi_EL1;
impl Register for APDBKeyHi_EL1 {
	const NAME: &'static str = "APDBKeyHi_EL1";
	const LEN: usize = 64;
}

/// Trace ID Register
pub struct TRCTRACEIDR;
impl Register for TRCTRACEIDR {
	const NAME: &'static str = "TRCTRACEIDR";
	const LEN: usize = 64;
}

/// TRCTRACEIDR register fields
pub mod trctraceidr {
	use crate::core::model::proc::RegisterView;

	/// TRCTRACEIDR\[6:0\]
	pub struct TRACEID;
	impl RegisterView for TRACEID {
		type Register = super::TRCTRACEIDR;
		const NAME: &'static str = "TRACEID";
		const OFFSET: usize = 0;
		const LEN: usize = 7;
	}
}

/// AArch32 Media and VFP Feature Register 1
#[allow(non_camel_case_types)]
pub struct MVFR1_EL1;
impl Register for MVFR1_EL1 {
	const NAME: &'static str = "MVFR1_EL1";
	const LEN: usize = 64;
}

/// MVFR1_EL1 register fields
pub mod mvfr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// MVFR1_EL1\[31:28\]
	pub struct SIMDFMAC;
	impl RegisterView for SIMDFMAC {
		type Register = super::MVFR1_EL1;
		const NAME: &'static str = "SIMDFMAC";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// MVFR1_EL1\[27:24\]
	pub struct FPHP;
	impl RegisterView for FPHP {
		type Register = super::MVFR1_EL1;
		const NAME: &'static str = "FPHP";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// MVFR1_EL1\[23:20\]
	pub struct SIMDHP;
	impl RegisterView for SIMDHP {
		type Register = super::MVFR1_EL1;
		const NAME: &'static str = "SIMDHP";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// MVFR1_EL1\[19:16\]
	pub struct SIMDSP;
	impl RegisterView for SIMDSP {
		type Register = super::MVFR1_EL1;
		const NAME: &'static str = "SIMDSP";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// MVFR1_EL1\[15:12\]
	pub struct SIMDInt;
	impl RegisterView for SIMDInt {
		type Register = super::MVFR1_EL1;
		const NAME: &'static str = "SIMDInt";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// MVFR1_EL1\[11:8\]
	pub struct SIMDLS;
	impl RegisterView for SIMDLS {
		type Register = super::MVFR1_EL1;
		const NAME: &'static str = "SIMDLS";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// MVFR1_EL1\[7:4\]
	pub struct FPDNaN;
	impl RegisterView for FPDNaN {
		type Register = super::MVFR1_EL1;
		const NAME: &'static str = "FPDNaN";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// MVFR1_EL1\[3:0\]
	pub struct FPFtZ;
	impl RegisterView for FPFtZ {
		type Register = super::MVFR1_EL1;
		const NAME: &'static str = "FPFtZ";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Interrupt Controller Software Generated Interrupt Group 0 Register
#[allow(non_camel_case_types)]
pub struct ICC_SGI0R_EL1;
impl Register for ICC_SGI0R_EL1 {
	const NAME: &'static str = "ICC_SGI0R_EL1";
	const LEN: usize = 64;
}

/// ICC_SGI0R_EL1 register fields
pub mod icc_sgi0r_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_SGI0R_EL1\[55:48\]
	pub struct Aff3;
	impl RegisterView for Aff3 {
		type Register = super::ICC_SGI0R_EL1;
		const NAME: &'static str = "Aff3";
		const OFFSET: usize = 48;
		const LEN: usize = 8;
	}

	/// ICC_SGI0R_EL1\[47:44\]
	pub struct RS;
	impl RegisterView for RS {
		type Register = super::ICC_SGI0R_EL1;
		const NAME: &'static str = "RS";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ICC_SGI0R_EL1\[40\]
	pub struct IRM;
	impl RegisterView for IRM {
		type Register = super::ICC_SGI0R_EL1;
		const NAME: &'static str = "IRM";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// ICC_SGI0R_EL1\[39:32\]
	pub struct Aff2;
	impl RegisterView for Aff2 {
		type Register = super::ICC_SGI0R_EL1;
		const NAME: &'static str = "Aff2";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// ICC_SGI0R_EL1\[27:24\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_SGI0R_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ICC_SGI0R_EL1\[23:16\]
	pub struct Aff1;
	impl RegisterView for Aff1 {
		type Register = super::ICC_SGI0R_EL1;
		const NAME: &'static str = "Aff1";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// ICC_SGI0R_EL1\[15:0\]
	pub struct TargetList;
	impl RegisterView for TargetList {
		type Register = super::ICC_SGI0R_EL1;
		const NAME: &'static str = "TargetList";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Branch Record Buffer Target Address Injection Register
#[allow(non_camel_case_types)]
pub struct BRBTGTINJ_EL1;
impl Register for BRBTGTINJ_EL1 {
	const NAME: &'static str = "BRBTGTINJ_EL1";
	const LEN: usize = 64;
}

/// BRBTGTINJ_EL1 register fields
pub mod brbtgtinj_el1 {
	use crate::core::model::proc::RegisterView;

	/// BRBTGTINJ_EL1\[63:0\]
	pub struct ADDRESS;
	impl RegisterView for ADDRESS {
		type Register = super::BRBTGTINJ_EL1;
		const NAME: &'static str = "ADDRESS";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Interrupt Controller Software Generated Interrupt Group 1 Register
#[allow(non_camel_case_types)]
pub struct ICC_SGI1R_EL1;
impl Register for ICC_SGI1R_EL1 {
	const NAME: &'static str = "ICC_SGI1R_EL1";
	const LEN: usize = 64;
}

/// ICC_SGI1R_EL1 register fields
pub mod icc_sgi1r_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_SGI1R_EL1\[55:48\]
	pub struct Aff3;
	impl RegisterView for Aff3 {
		type Register = super::ICC_SGI1R_EL1;
		const NAME: &'static str = "Aff3";
		const OFFSET: usize = 48;
		const LEN: usize = 8;
	}

	/// ICC_SGI1R_EL1\[47:44\]
	pub struct RS;
	impl RegisterView for RS {
		type Register = super::ICC_SGI1R_EL1;
		const NAME: &'static str = "RS";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ICC_SGI1R_EL1\[40\]
	pub struct IRM;
	impl RegisterView for IRM {
		type Register = super::ICC_SGI1R_EL1;
		const NAME: &'static str = "IRM";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// ICC_SGI1R_EL1\[39:32\]
	pub struct Aff2;
	impl RegisterView for Aff2 {
		type Register = super::ICC_SGI1R_EL1;
		const NAME: &'static str = "Aff2";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// ICC_SGI1R_EL1\[27:24\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_SGI1R_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ICC_SGI1R_EL1\[23:16\]
	pub struct Aff1;
	impl RegisterView for Aff1 {
		type Register = super::ICC_SGI1R_EL1;
		const NAME: &'static str = "Aff1";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// ICC_SGI1R_EL1\[15:0\]
	pub struct TargetList;
	impl RegisterView for TargetList {
		type Register = super::ICC_SGI1R_EL1;
		const NAME: &'static str = "TargetList";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Auxiliary Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct ACTLR_EL3;
impl Register for ACTLR_EL3 {
	const NAME: &'static str = "ACTLR_EL3";
	const LEN: usize = 64;
}

/// Trace ID Register 5
pub struct TRCIDR5;
impl Register for TRCIDR5 {
	const NAME: &'static str = "TRCIDR5";
	const LEN: usize = 64;
}

/// TRCIDR5 register fields
pub mod trcidr5 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR5\[31\]
	pub struct OE;
	impl RegisterView for OE {
		type Register = super::TRCIDR5;
		const NAME: &'static str = "OE";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// TRCIDR5\[30:28\]
	pub struct NUMCNTR;
	impl RegisterView for NUMCNTR {
		type Register = super::TRCIDR5;
		const NAME: &'static str = "NUMCNTR";
		const OFFSET: usize = 28;
		const LEN: usize = 3;
	}

	/// TRCIDR5\[27:25\]
	pub struct NUMSEQSTATE;
	impl RegisterView for NUMSEQSTATE {
		type Register = super::TRCIDR5;
		const NAME: &'static str = "NUMSEQSTATE";
		const OFFSET: usize = 25;
		const LEN: usize = 3;
	}

	/// TRCIDR5\[23\]
	pub struct LPOVERRIDE;
	impl RegisterView for LPOVERRIDE {
		type Register = super::TRCIDR5;
		const NAME: &'static str = "LPOVERRIDE";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// TRCIDR5\[22\]
	pub struct ATBTRIG;
	impl RegisterView for ATBTRIG {
		type Register = super::TRCIDR5;
		const NAME: &'static str = "ATBTRIG";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// TRCIDR5\[21:16\]
	pub struct TRACEIDSIZE;
	impl RegisterView for TRACEIDSIZE {
		type Register = super::TRCIDR5;
		const NAME: &'static str = "TRACEIDSIZE";
		const OFFSET: usize = 16;
		const LEN: usize = 6;
	}

	/// TRCIDR5\[11:9\]
	pub struct NUMEXTINSEL;
	impl RegisterView for NUMEXTINSEL {
		type Register = super::TRCIDR5;
		const NAME: &'static str = "NUMEXTINSEL";
		const OFFSET: usize = 9;
		const LEN: usize = 3;
	}

	/// TRCIDR5\[8:0\]
	pub struct NUMEXTIN;
	impl RegisterView for NUMEXTIN {
		type Register = super::TRCIDR5;
		const NAME: &'static str = "NUMEXTIN";
		const OFFSET: usize = 0;
		const LEN: usize = 9;
	}
}

/// Interrupt Controller System Register Enable Register (EL1)
#[allow(non_camel_case_types)]
pub struct ICC_SRE_EL1;
impl Register for ICC_SRE_EL1 {
	const NAME: &'static str = "ICC_SRE_EL1";
	const LEN: usize = 64;
}

/// ICC_SRE_EL1 register fields
pub mod icc_sre_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_SRE_EL1\[2\]
	pub struct DIB;
	impl RegisterView for DIB {
		type Register = super::ICC_SRE_EL1;
		const NAME: &'static str = "DIB";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// ICC_SRE_EL1\[1\]
	pub struct DFB;
	impl RegisterView for DFB {
		type Register = super::ICC_SRE_EL1;
		const NAME: &'static str = "DFB";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICC_SRE_EL1\[0\]
	pub struct SRE;
	impl RegisterView for SRE {
		type Register = super::ICC_SRE_EL1;
		const NAME: &'static str = "SRE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// MPAM Virtual PARTID Mapping Register 1
#[allow(non_camel_case_types)]
pub struct MPAMVPM1_EL2;
impl Register for MPAMVPM1_EL2 {
	const NAME: &'static str = "MPAMVPM1_EL2";
	const LEN: usize = 64;
}

/// MPAMVPM1_EL2 register fields
pub mod mpamvpm1_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMVPM1_EL2\[63:48\]
	pub struct PhyPARTID7;
	impl RegisterView for PhyPARTID7 {
		type Register = super::MPAMVPM1_EL2;
		const NAME: &'static str = "PhyPARTID7";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// MPAMVPM1_EL2\[47:32\]
	pub struct PhyPARTID6;
	impl RegisterView for PhyPARTID6 {
		type Register = super::MPAMVPM1_EL2;
		const NAME: &'static str = "PhyPARTID6";
		const OFFSET: usize = 32;
		const LEN: usize = 16;
	}

	/// MPAMVPM1_EL2\[31:16\]
	pub struct PhyPARTID5;
	impl RegisterView for PhyPARTID5 {
		type Register = super::MPAMVPM1_EL2;
		const NAME: &'static str = "PhyPARTID5";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAMVPM1_EL2\[15:0\]
	pub struct PhyPARTID4;
	impl RegisterView for PhyPARTID4 {
		type Register = super::MPAMVPM1_EL2;
		const NAME: &'static str = "PhyPARTID4";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Performance Monitors Overflow Flag Status Set Register
#[allow(non_camel_case_types)]
pub struct PMOVSSET_EL0;
impl Register for PMOVSSET_EL0 {
	const NAME: &'static str = "PMOVSSET_EL0";
	const LEN: usize = 64;
}

/// PMOVSSET_EL0 register fields
pub mod pmovsset_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMOVSSET_EL0\[31\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMOVSSET_EL0;
		const NAME: &'static str = "C";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Common Event Identification Register 1
#[allow(non_camel_case_types)]
pub struct PMCEID1_EL0;
impl Register for PMCEID1_EL0 {
	const NAME: &'static str = "PMCEID1_EL0";
	const LEN: usize = 64;
}

/// Debug Watchpoint Control Registers, n = 63 - 0
#[allow(non_camel_case_types)]
pub struct DBGWCRn_EL1;
impl RegisterArray for DBGWCRn_EL1 {
	const NAME: &'static str = "DBGWCRn_EL1";
	const ELEMS: usize = 63;
	const ELEM_LEN: usize = 64;
}

/// DBGWCRn_EL1 register fields
pub mod dbgwcrn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// DBGWCRn_EL1\[31:30\]
	pub struct LBNX;
	impl RegisterArrayView for LBNX {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "LBNX";
		const OFFSET: usize = 30;
		const LEN: usize = 2;
	}

	/// DBGWCRn_EL1\[29\]
	pub struct SSCE;
	impl RegisterArrayView for SSCE {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "SSCE";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// DBGWCRn_EL1\[28:24\]
	pub struct MASK;
	impl RegisterArrayView for MASK {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "MASK";
		const OFFSET: usize = 24;
		const LEN: usize = 5;
	}

	/// DBGWCRn_EL1\[22\]
	pub struct WT2;
	impl RegisterArrayView for WT2 {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "WT2";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// DBGWCRn_EL1\[20\]
	pub struct WT;
	impl RegisterArrayView for WT {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "WT";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// DBGWCRn_EL1\[19:16\]
	pub struct LBN;
	impl RegisterArrayView for LBN {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "LBN";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// DBGWCRn_EL1\[15:14\]
	pub struct SSC;
	impl RegisterArrayView for SSC {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "SSC";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// DBGWCRn_EL1\[13\]
	pub struct HMC;
	impl RegisterArrayView for HMC {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "HMC";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// DBGWCRn_EL1\[12:5\]
	pub struct BAS;
	impl RegisterArrayView for BAS {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "BAS";
		const OFFSET: usize = 5;
		const LEN: usize = 8;
	}

	/// DBGWCRn_EL1\[4:3\]
	pub struct LSC;
	impl RegisterArrayView for LSC {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "LSC";
		const OFFSET: usize = 3;
		const LEN: usize = 2;
	}

	/// DBGWCRn_EL1\[2:1\]
	pub struct PAC;
	impl RegisterArrayView for PAC {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "PAC";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// DBGWCRn_EL1\[0\]
	pub struct E;
	impl RegisterArrayView for E {
		type RegisterArray = super::DBGWCRn_EL1;
		const NAME: &'static str = "E";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Primary MECID for EL2&0 translation regimes
#[allow(non_camel_case_types)]
pub struct MECID_P1_EL2;
impl Register for MECID_P1_EL2 {
	const NAME: &'static str = "MECID_P1_EL2";
	const LEN: usize = 64;
}

/// MECID_P1_EL2 register fields
pub mod mecid_p1_el2 {
	use crate::core::model::proc::RegisterView;

	/// MECID_P1_EL2\[15:0\]
	pub struct MECID;
	impl RegisterView for MECID {
		type Register = super::MECID_P1_EL2;
		const NAME: &'static str = "MECID";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// SME Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct SMCR_EL1;
impl Register for SMCR_EL1 {
	const NAME: &'static str = "SMCR_EL1";
	const LEN: usize = 64;
}

/// SMCR_EL1 register fields
pub mod smcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// SMCR_EL1\[31\]
	pub struct FA64;
	impl RegisterView for FA64 {
		type Register = super::SMCR_EL1;
		const NAME: &'static str = "FA64";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SMCR_EL1\[30\]
	pub struct EZT0;
	impl RegisterView for EZT0 {
		type Register = super::SMCR_EL1;
		const NAME: &'static str = "EZT0";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SMCR_EL1\[3:0\]
	pub struct LEN;
	impl RegisterView for LEN {
		type Register = super::SMCR_EL1;
		const NAME: &'static str = "LEN";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Floating-point Mode Register
pub struct FPMR;
impl Register for FPMR {
	const NAME: &'static str = "FPMR";
	const LEN: usize = 64;
}

/// FPMR register fields
pub mod fpmr {
	use crate::core::model::proc::RegisterView;

	/// FPMR\[37:32\]
	pub struct LSCALE2;
	impl RegisterView for LSCALE2 {
		type Register = super::FPMR;
		const NAME: &'static str = "LSCALE2";
		const OFFSET: usize = 32;
		const LEN: usize = 6;
	}

	/// FPMR\[31:24\]
	pub struct NSCALE;
	impl RegisterView for NSCALE {
		type Register = super::FPMR;
		const NAME: &'static str = "NSCALE";
		const OFFSET: usize = 24;
		const LEN: usize = 8;
	}

	/// FPMR\[22:16\]
	pub struct LSCALE;
	impl RegisterView for LSCALE {
		type Register = super::FPMR;
		const NAME: &'static str = "LSCALE";
		const OFFSET: usize = 16;
		const LEN: usize = 7;
	}

	/// FPMR\[15\]
	pub struct OSC;
	impl RegisterView for OSC {
		type Register = super::FPMR;
		const NAME: &'static str = "OSC";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// FPMR\[14\]
	pub struct OSM;
	impl RegisterView for OSM {
		type Register = super::FPMR;
		const NAME: &'static str = "OSM";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// FPMR\[8:6\]
	pub struct F8D;
	impl RegisterView for F8D {
		type Register = super::FPMR;
		const NAME: &'static str = "F8D";
		const OFFSET: usize = 6;
		const LEN: usize = 3;
	}

	/// FPMR\[5:3\]
	pub struct F8S2;
	impl RegisterView for F8S2 {
		type Register = super::FPMR;
		const NAME: &'static str = "F8S2";
		const OFFSET: usize = 3;
		const LEN: usize = 3;
	}

	/// FPMR\[2:0\]
	pub struct F8S1;
	impl RegisterView for F8S1 {
		type Register = super::FPMR;
		const NAME: &'static str = "F8S1";
		const OFFSET: usize = 0;
		const LEN: usize = 3;
	}
}

/// System Performance Monitor Control Register
#[allow(non_camel_case_types)]
pub struct SPMCR_EL0;
impl Register for SPMCR_EL0 {
	const NAME: &'static str = "SPMCR_EL0";
	const LEN: usize = 64;
}

/// SPMCR_EL0 register fields
pub mod spmcr_el0 {
	use crate::core::model::proc::RegisterView;

	/// SPMCR_EL0\[11\]
	pub struct TRO;
	impl RegisterView for TRO {
		type Register = super::SPMCR_EL0;
		const NAME: &'static str = "TRO";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// SPMCR_EL0\[10\]
	pub struct HDBG;
	impl RegisterView for HDBG {
		type Register = super::SPMCR_EL0;
		const NAME: &'static str = "HDBG";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// SPMCR_EL0\[9\]
	pub struct FZO;
	impl RegisterView for FZO {
		type Register = super::SPMCR_EL0;
		const NAME: &'static str = "FZO";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPMCR_EL0\[8\]
	pub struct NA;
	impl RegisterView for NA {
		type Register = super::SPMCR_EL0;
		const NAME: &'static str = "NA";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SPMCR_EL0\[4\]
	pub struct EX;
	impl RegisterView for EX {
		type Register = super::SPMCR_EL0;
		const NAME: &'static str = "EX";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// SPMCR_EL0\[1\]
	pub struct P;
	impl RegisterView for P {
		type Register = super::SPMCR_EL0;
		const NAME: &'static str = "P";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// SPMCR_EL0\[0\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::SPMCR_EL0;
		const NAME: &'static str = "E";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// MPAM ID Register (EL1)
#[allow(non_camel_case_types)]
pub struct MPAMIDR_EL1;
impl Register for MPAMIDR_EL1 {
	const NAME: &'static str = "MPAMIDR_EL1";
	const LEN: usize = 64;
}

/// MPAMIDR_EL1 register fields
pub mod mpamidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// MPAMIDR_EL1\[61\]
	#[allow(non_camel_case_types)]
	pub struct HAS_SDEFLT;
	impl RegisterView for HAS_SDEFLT {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "HAS_SDEFLT";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// MPAMIDR_EL1\[60\]
	#[allow(non_camel_case_types)]
	pub struct HAS_FORCE_NS;
	impl RegisterView for HAS_FORCE_NS {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "HAS_FORCE_NS";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// MPAMIDR_EL1\[59\]
	pub struct SP4;
	impl RegisterView for SP4 {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "SP4";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// MPAMIDR_EL1\[58\]
	#[allow(non_camel_case_types)]
	pub struct HAS_TIDR;
	impl RegisterView for HAS_TIDR {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "HAS_TIDR";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// MPAMIDR_EL1\[57\]
	#[allow(non_camel_case_types)]
	pub struct HAS_ALTSP;
	impl RegisterView for HAS_ALTSP {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "HAS_ALTSP";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// MPAMIDR_EL1\[39:32\]
	#[allow(non_camel_case_types)]
	pub struct PMG_MAX;
	impl RegisterView for PMG_MAX {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "PMG_MAX";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// MPAMIDR_EL1\[20:18\]
	#[allow(non_camel_case_types)]
	pub struct VPMR_MAX;
	impl RegisterView for VPMR_MAX {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "VPMR_MAX";
		const OFFSET: usize = 18;
		const LEN: usize = 3;
	}

	/// MPAMIDR_EL1\[17\]
	#[allow(non_camel_case_types)]
	pub struct HAS_HCR;
	impl RegisterView for HAS_HCR {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "HAS_HCR";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// MPAMIDR_EL1\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_MAX;
	impl RegisterView for PARTID_MAX {
		type Register = super::MPAMIDR_EL1;
		const NAME: &'static str = "PARTID_MAX";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Saved Program Status Register (EL1)
#[allow(non_camel_case_types)]
pub struct SPSR_EL1;
impl Register for SPSR_EL1 {
	const NAME: &'static str = "SPSR_EL1";
	const LEN: usize = 64;
}

/// SPSR_EL1 register fields
pub mod spsr_el1 {
	use crate::core::model::proc::RegisterView;

	/// SPSR_EL1\[33\]
	pub struct PPEND;
	impl RegisterView for PPEND {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "PPEND";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[31\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "N";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[30\]
	pub struct Z;
	impl RegisterView for Z {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "Z";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[29\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "C";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[28\]
	pub struct V;
	impl RegisterView for V {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[27\]
	pub struct Q;
	impl RegisterView for Q {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "Q";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[15:10\]
	pub struct IT;
	impl RegisterView for IT {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "IT";
		const OFFSET: usize = 10;
		const LEN: usize = 6;
	}

	/// SPSR_EL1\[24\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[12\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[22\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[21\]
	pub struct SS;
	impl RegisterView for SS {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "SS";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[20\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[19:16\]
	pub struct GE;
	impl RegisterView for GE {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "GE";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SPSR_EL1\[9\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "E";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[8\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "A";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[7\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "I";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[6\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "F";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[5\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "T";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[35\]
	pub struct PACM;
	impl RegisterView for PACM {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "PACM";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[34\]
	pub struct EXLOCK;
	impl RegisterView for EXLOCK {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "EXLOCK";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[32\]
	pub struct PM;
	impl RegisterView for PM {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "PM";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[25\]
	pub struct TCO;
	impl RegisterView for TCO {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "TCO";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[23\]
	pub struct UAO;
	impl RegisterView for UAO {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "UAO";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[13\]
	pub struct ALLINT;
	impl RegisterView for ALLINT {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "ALLINT";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[11:10\]
	pub struct BTYPE;
	impl RegisterView for BTYPE {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "BTYPE";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// SPSR_EL1\[9\]
	pub struct D;
	impl RegisterView for D {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "D";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_EL1\[4\]
	#[allow(non_camel_case_types)]
	pub struct M_4;
	impl RegisterView for M_4 {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "M_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// SPSR_EL1\[4\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4;
	impl RegisterView for M_4_4 {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "M_4_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// SPSR_EL1\[3:0\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4_3_0;
	impl RegisterView for M_4_4_3_0 {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "M_4_4_3_0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
	/// SPSR_EL1\[3:0\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4_3_0_3_0;
	impl RegisterView for M_4_4_3_0_3_0 {
		type Register = super::SPSR_EL1;
		const NAME: &'static str = "M_4_4_3_0_3_0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Exception Link Register (EL1)
#[allow(non_camel_case_types)]
pub struct ELR_EL1;
impl Register for ELR_EL1 {
	const NAME: &'static str = "ELR_EL1";
	const LEN: usize = 64;
}

/// EL0 Read/Write Software Thread ID Register 2
#[allow(non_camel_case_types)]
pub struct TPIDR2_EL0;
impl Register for TPIDR2_EL0 {
	const NAME: &'static str = "TPIDR2_EL0";
	const LEN: usize = 64;
}

/// Interrupt Controller Virtual Interrupt Acknowledge Register 1
#[allow(non_camel_case_types)]
pub struct ICV_IAR1_EL1;
impl Register for ICV_IAR1_EL1 {
	const NAME: &'static str = "ICV_IAR1_EL1";
	const LEN: usize = 64;
}

/// ICV_IAR1_EL1 register fields
pub mod icv_iar1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_IAR1_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICV_IAR1_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Vector Base Address Register (EL3)
#[allow(non_camel_case_types)]
pub struct VBAR_EL3;
impl Register for VBAR_EL3 {
	const NAME: &'static str = "VBAR_EL3";
	const LEN: usize = 64;
}

/// Hypervisor Debug Fine-Grained Read Trap Register
#[allow(non_camel_case_types)]
pub struct HDFGRTR_EL2;
impl Register for HDFGRTR_EL2 {
	const NAME: &'static str = "HDFGRTR_EL2";
	const LEN: usize = 64;
}

/// HDFGRTR_EL2 register fields
pub mod hdfgrtr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HDFGRTR_EL2\[63\]
	#[allow(non_camel_case_types)]
	pub struct PMBIDR_EL1;
	impl RegisterView for PMBIDR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMBIDR_EL1";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[62\]
	#[allow(non_camel_case_types)]
	pub struct nPMSNEVFR_EL1;
	impl RegisterView for nPMSNEVFR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "nPMSNEVFR_EL1";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[61\]
	#[allow(non_camel_case_types)]
	pub struct nBRBDATA;
	impl RegisterView for nBRBDATA {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "nBRBDATA";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[60\]
	#[allow(non_camel_case_types)]
	pub struct nBRBCTL;
	impl RegisterView for nBRBCTL {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "nBRBCTL";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[59\]
	#[allow(non_camel_case_types)]
	pub struct nBRBIDR;
	impl RegisterView for nBRBIDR {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "nBRBIDR";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[58\]
	#[allow(non_camel_case_types)]
	pub struct PMCEIDn_EL0;
	impl RegisterView for PMCEIDn_EL0 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMCEIDn_EL0";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[57\]
	#[allow(non_camel_case_types)]
	pub struct PMUSERENR_EL0;
	impl RegisterView for PMUSERENR_EL0 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMUSERENR_EL0";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[56\]
	#[allow(non_camel_case_types)]
	pub struct TRBTRG_EL1;
	impl RegisterView for TRBTRG_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRBTRG_EL1";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[55\]
	#[allow(non_camel_case_types)]
	pub struct TRBSR_EL1;
	impl RegisterView for TRBSR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRBSR_EL1";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[54\]
	#[allow(non_camel_case_types)]
	pub struct TRBPTR_EL1;
	impl RegisterView for TRBPTR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRBPTR_EL1";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[53\]
	#[allow(non_camel_case_types)]
	pub struct TRBMAR_EL1;
	impl RegisterView for TRBMAR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRBMAR_EL1";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[52\]
	#[allow(non_camel_case_types)]
	pub struct TRBLIMITR_EL1;
	impl RegisterView for TRBLIMITR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRBLIMITR_EL1";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[51\]
	#[allow(non_camel_case_types)]
	pub struct TRBIDR_EL1;
	impl RegisterView for TRBIDR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRBIDR_EL1";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[50\]
	#[allow(non_camel_case_types)]
	pub struct TRBBASER_EL1;
	impl RegisterView for TRBBASER_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRBBASER_EL1";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[48\]
	pub struct TRCVICTLR;
	impl RegisterView for TRCVICTLR {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCVICTLR";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[47\]
	pub struct TRCSTATR;
	impl RegisterView for TRCSTATR {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCSTATR";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[46\]
	pub struct TRCSSCSRn;
	impl RegisterView for TRCSSCSRn {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCSSCSRn";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[45\]
	pub struct TRCSEQSTR;
	impl RegisterView for TRCSEQSTR {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCSEQSTR";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[44\]
	pub struct TRCPRGCTLR;
	impl RegisterView for TRCPRGCTLR {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCPRGCTLR";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[43\]
	pub struct TRCOSLSR;
	impl RegisterView for TRCOSLSR {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCOSLSR";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[41\]
	pub struct TRCIMSPECn;
	impl RegisterView for TRCIMSPECn {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCIMSPECn";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[40\]
	pub struct TRCID;
	impl RegisterView for TRCID {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCID";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[37\]
	pub struct TRCCNTVRn;
	impl RegisterView for TRCCNTVRn {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCCNTVRn";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[36\]
	pub struct TRCCLAIM;
	impl RegisterView for TRCCLAIM {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCCLAIM";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[35\]
	pub struct TRCAUXCTLR;
	impl RegisterView for TRCAUXCTLR {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCAUXCTLR";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[34\]
	pub struct TRCAUTHSTATUS;
	impl RegisterView for TRCAUTHSTATUS {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRCAUTHSTATUS";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[33\]
	pub struct TRC;
	impl RegisterView for TRC {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "TRC";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[32\]
	#[allow(non_camel_case_types)]
	pub struct PMSLATFR_EL1;
	impl RegisterView for PMSLATFR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMSLATFR_EL1";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[31\]
	#[allow(non_camel_case_types)]
	pub struct PMSIRR_EL1;
	impl RegisterView for PMSIRR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMSIRR_EL1";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[30\]
	#[allow(non_camel_case_types)]
	pub struct PMSIDR_EL1;
	impl RegisterView for PMSIDR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMSIDR_EL1";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[29\]
	#[allow(non_camel_case_types)]
	pub struct PMSICR_EL1;
	impl RegisterView for PMSICR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMSICR_EL1";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[28\]
	#[allow(non_camel_case_types)]
	pub struct PMSFCR_EL1;
	impl RegisterView for PMSFCR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMSFCR_EL1";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[27\]
	#[allow(non_camel_case_types)]
	pub struct PMSEVFR_EL1;
	impl RegisterView for PMSEVFR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMSEVFR_EL1";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[26\]
	#[allow(non_camel_case_types)]
	pub struct PMSCR_EL1;
	impl RegisterView for PMSCR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMSCR_EL1";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[25\]
	#[allow(non_camel_case_types)]
	pub struct PMBSR_EL1;
	impl RegisterView for PMBSR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMBSR_EL1";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[24\]
	#[allow(non_camel_case_types)]
	pub struct PMBPTR_EL1;
	impl RegisterView for PMBPTR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMBPTR_EL1";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[23\]
	#[allow(non_camel_case_types)]
	pub struct PMBLIMITR_EL1;
	impl RegisterView for PMBLIMITR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMBLIMITR_EL1";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[22\]
	#[allow(non_camel_case_types)]
	pub struct PMMIR_EL1;
	impl RegisterView for PMMIR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMMIR_EL1";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[19\]
	#[allow(non_camel_case_types)]
	pub struct PMSELR_EL0;
	impl RegisterView for PMSELR_EL0 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMSELR_EL0";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[18\]
	pub struct PMOVS;
	impl RegisterView for PMOVS {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMOVS";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[17\]
	pub struct PMINTEN;
	impl RegisterView for PMINTEN {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMINTEN";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[16\]
	pub struct PMCNTEN;
	impl RegisterView for PMCNTEN {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMCNTEN";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[15\]
	#[allow(non_camel_case_types)]
	pub struct PMCCNTR_EL0;
	impl RegisterView for PMCCNTR_EL0 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMCCNTR_EL0";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[14\]
	#[allow(non_camel_case_types)]
	pub struct PMCCFILTR_EL0;
	impl RegisterView for PMCCFILTR_EL0 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMCCFILTR_EL0";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[13\]
	#[allow(non_camel_case_types)]
	pub struct PMEVTYPERn_EL0;
	impl RegisterView for PMEVTYPERn_EL0 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMEVTYPERn_EL0";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[12\]
	#[allow(non_camel_case_types)]
	pub struct PMEVCNTRn_EL0;
	impl RegisterView for PMEVCNTRn_EL0 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "PMEVCNTRn_EL0";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[11\]
	#[allow(non_camel_case_types)]
	pub struct OSDLR_EL1;
	impl RegisterView for OSDLR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "OSDLR_EL1";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[10\]
	#[allow(non_camel_case_types)]
	pub struct OSECCR_EL1;
	impl RegisterView for OSECCR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "OSECCR_EL1";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[9\]
	#[allow(non_camel_case_types)]
	pub struct OSLSR_EL1;
	impl RegisterView for OSLSR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "OSLSR_EL1";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[7\]
	#[allow(non_camel_case_types)]
	pub struct DBGPRCR_EL1;
	impl RegisterView for DBGPRCR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "DBGPRCR_EL1";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[6\]
	#[allow(non_camel_case_types)]
	pub struct DBGAUTHSTATUS_EL1;
	impl RegisterView for DBGAUTHSTATUS_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "DBGAUTHSTATUS_EL1";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[5\]
	pub struct DBGCLAIM;
	impl RegisterView for DBGCLAIM {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "DBGCLAIM";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[4\]
	#[allow(non_camel_case_types)]
	pub struct MDSCR_EL1;
	impl RegisterView for MDSCR_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "MDSCR_EL1";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[3\]
	#[allow(non_camel_case_types)]
	pub struct DBGWVRn_EL1;
	impl RegisterView for DBGWVRn_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "DBGWVRn_EL1";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[2\]
	#[allow(non_camel_case_types)]
	pub struct DBGWCRn_EL1;
	impl RegisterView for DBGWCRn_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "DBGWCRn_EL1";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[1\]
	#[allow(non_camel_case_types)]
	pub struct DBGBVRn_EL1;
	impl RegisterView for DBGBVRn_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "DBGBVRn_EL1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HDFGRTR_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct DBGBCRn_EL1;
	impl RegisterView for DBGBCRn_EL1 {
		type Register = super::HDFGRTR_EL2;
		const NAME: &'static str = "DBGBCRn_EL1";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// AArch32 Processor Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_PFR1_EL1;
impl Register for ID_PFR1_EL1 {
	const NAME: &'static str = "ID_PFR1_EL1";
	const LEN: usize = 64;
}

/// ID_PFR1_EL1 register fields
pub mod id_pfr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_PFR1_EL1\[31:28\]
	pub struct GIC;
	impl RegisterView for GIC {
		type Register = super::ID_PFR1_EL1;
		const NAME: &'static str = "GIC";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_PFR1_EL1\[27:24\]
	#[allow(non_camel_case_types)]
	pub struct Virt_frac;
	impl RegisterView for Virt_frac {
		type Register = super::ID_PFR1_EL1;
		const NAME: &'static str = "Virt_frac";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_PFR1_EL1\[23:20\]
	#[allow(non_camel_case_types)]
	pub struct Sec_frac;
	impl RegisterView for Sec_frac {
		type Register = super::ID_PFR1_EL1;
		const NAME: &'static str = "Sec_frac";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_PFR1_EL1\[19:16\]
	pub struct GenTimer;
	impl RegisterView for GenTimer {
		type Register = super::ID_PFR1_EL1;
		const NAME: &'static str = "GenTimer";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_PFR1_EL1\[15:12\]
	pub struct Virtualization;
	impl RegisterView for Virtualization {
		type Register = super::ID_PFR1_EL1;
		const NAME: &'static str = "Virtualization";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_PFR1_EL1\[11:8\]
	pub struct MProgMod;
	impl RegisterView for MProgMod {
		type Register = super::ID_PFR1_EL1;
		const NAME: &'static str = "MProgMod";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_PFR1_EL1\[7:4\]
	pub struct Security;
	impl RegisterView for Security {
		type Register = super::ID_PFR1_EL1;
		const NAME: &'static str = "Security";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_PFR1_EL1\[3:0\]
	pub struct ProgMod;
	impl RegisterView for ProgMod {
		type Register = super::ID_PFR1_EL1;
		const NAME: &'static str = "ProgMod";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Vector Base Address Register (EL1)
#[allow(non_camel_case_types)]
pub struct VBAR_EL1;
impl Register for VBAR_EL1 {
	const NAME: &'static str = "VBAR_EL1";
	const LEN: usize = 64;
}

/// Performance Monitors Snapshot Status and Capture Register
#[allow(non_camel_case_types)]
pub struct PMSSCR_EL1;
impl Register for PMSSCR_EL1 {
	const NAME: &'static str = "PMSSCR_EL1";
	const LEN: usize = 64;
}

/// PMSSCR_EL1 register fields
pub mod pmsscr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSSCR_EL1\[32\]
	pub struct NC;
	impl RegisterView for NC {
		type Register = super::PMSSCR_EL1;
		const NAME: &'static str = "NC";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// PMSSCR_EL1\[0\]
	pub struct SS;
	impl RegisterView for SS {
		type Register = super::PMSSCR_EL1;
		const NAME: &'static str = "SS";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Timestamp Control Register
pub struct TRCTSCTLR;
impl Register for TRCTSCTLR {
	const NAME: &'static str = "TRCTSCTLR";
	const LEN: usize = 64;
}

/// TRCTSCTLR register fields
pub mod trctsctlr {
	use crate::core::model::proc::RegisterView;

	/// TRCTSCTLR\[7\]
	#[allow(non_camel_case_types)]
	pub struct EVENT_TYPE;
	impl RegisterView for EVENT_TYPE {
		type Register = super::TRCTSCTLR;
		const NAME: &'static str = "EVENT_TYPE";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TRCTSCTLR\[4:0\]
	#[allow(non_camel_case_types)]
	pub struct EVENT_SEL;
	impl RegisterView for EVENT_SEL {
		type Register = super::TRCTSCTLR;
		const NAME: &'static str = "EVENT_SEL";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Profiling Buffer Write Pointer Register
#[allow(non_camel_case_types)]
pub struct PMBPTR_EL1;
impl Register for PMBPTR_EL1 {
	const NAME: &'static str = "PMBPTR_EL1";
	const LEN: usize = 64;
}

/// PMBPTR_EL1 register fields
pub mod pmbptr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMBPTR_EL1\[63:0\]
	pub struct PTR;
	impl RegisterView for PTR {
		type Register = super::PMBPTR_EL1;
		const NAME: &'static str = "PTR";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// AArch32 Memory Model Feature Register 4
#[allow(non_camel_case_types)]
pub struct ID_MMFR4_EL1;
impl Register for ID_MMFR4_EL1 {
	const NAME: &'static str = "ID_MMFR4_EL1";
	const LEN: usize = 64;
}

/// ID_MMFR4_EL1 register fields
pub mod id_mmfr4_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_MMFR4_EL1\[31:28\]
	pub struct EVT;
	impl RegisterView for EVT {
		type Register = super::ID_MMFR4_EL1;
		const NAME: &'static str = "EVT";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_MMFR4_EL1\[27:24\]
	pub struct CCIDX;
	impl RegisterView for CCIDX {
		type Register = super::ID_MMFR4_EL1;
		const NAME: &'static str = "CCIDX";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_MMFR4_EL1\[23:20\]
	pub struct LSM;
	impl RegisterView for LSM {
		type Register = super::ID_MMFR4_EL1;
		const NAME: &'static str = "LSM";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_MMFR4_EL1\[19:16\]
	pub struct HPDS;
	impl RegisterView for HPDS {
		type Register = super::ID_MMFR4_EL1;
		const NAME: &'static str = "HPDS";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_MMFR4_EL1\[15:12\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::ID_MMFR4_EL1;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_MMFR4_EL1\[11:8\]
	pub struct XNX;
	impl RegisterView for XNX {
		type Register = super::ID_MMFR4_EL1;
		const NAME: &'static str = "XNX";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_MMFR4_EL1\[7:4\]
	pub struct AC2;
	impl RegisterView for AC2 {
		type Register = super::ID_MMFR4_EL1;
		const NAME: &'static str = "AC2";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_MMFR4_EL1\[3:0\]
	pub struct SpecSEI;
	impl RegisterView for SpecSEI {
		type Register = super::ID_MMFR4_EL1;
		const NAME: &'static str = "SpecSEI";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Activity Monitors Count Enable Set Register 0
#[allow(non_camel_case_types)]
pub struct AMCNTENSET0_EL0;
impl Register for AMCNTENSET0_EL0 {
	const NAME: &'static str = "AMCNTENSET0_EL0";
	const LEN: usize = 64;
}

/// Exception Link Register (EL3)
#[allow(non_camel_case_types)]
pub struct ELR_EL3;
impl Register for ELR_EL3 {
	const NAME: &'static str = "ELR_EL3";
	const LEN: usize = 64;
}

/// Trace Buffer Trigger Counter Register
#[allow(non_camel_case_types)]
pub struct TRBTRG_EL1;
impl Register for TRBTRG_EL1 {
	const NAME: &'static str = "TRBTRG_EL1";
	const LEN: usize = 64;
}

/// TRBTRG_EL1 register fields
pub mod trbtrg_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRBTRG_EL1\[31:0\]
	pub struct TRG;
	impl RegisterView for TRG {
		type Register = super::TRBTRG_EL1;
		const NAME: &'static str = "TRG";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Saved Program Status Register (EL3)
#[allow(non_camel_case_types)]
pub struct SPSR_EL3;
impl Register for SPSR_EL3 {
	const NAME: &'static str = "SPSR_EL3";
	const LEN: usize = 64;
}

/// SPSR_EL3 register fields
pub mod spsr_el3 {
	use crate::core::model::proc::RegisterView;

	/// SPSR_EL3\[33\]
	pub struct PPEND;
	impl RegisterView for PPEND {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "PPEND";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[31\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "N";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[30\]
	pub struct Z;
	impl RegisterView for Z {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "Z";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[29\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "C";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[28\]
	pub struct V;
	impl RegisterView for V {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[27\]
	pub struct Q;
	impl RegisterView for Q {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "Q";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[15:10\]
	pub struct IT;
	impl RegisterView for IT {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "IT";
		const OFFSET: usize = 10;
		const LEN: usize = 6;
	}

	/// SPSR_EL3\[24\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[12\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[22\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[21\]
	pub struct SS;
	impl RegisterView for SS {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "SS";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[20\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[19:16\]
	pub struct GE;
	impl RegisterView for GE {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "GE";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SPSR_EL3\[9\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "E";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[8\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "A";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[7\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "I";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[6\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "F";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[5\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "T";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[35\]
	pub struct PACM;
	impl RegisterView for PACM {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "PACM";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[34\]
	pub struct EXLOCK;
	impl RegisterView for EXLOCK {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "EXLOCK";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[32\]
	pub struct PM;
	impl RegisterView for PM {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "PM";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[25\]
	pub struct TCO;
	impl RegisterView for TCO {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "TCO";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[23\]
	pub struct UAO;
	impl RegisterView for UAO {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "UAO";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[13\]
	pub struct ALLINT;
	impl RegisterView for ALLINT {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "ALLINT";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[11:10\]
	pub struct BTYPE;
	impl RegisterView for BTYPE {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "BTYPE";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// SPSR_EL3\[9\]
	pub struct D;
	impl RegisterView for D {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "D";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_EL3\[4\]
	#[allow(non_camel_case_types)]
	pub struct M_4;
	impl RegisterView for M_4 {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "M_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// SPSR_EL3\[4\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4;
	impl RegisterView for M_4_4 {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "M_4_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// SPSR_EL3\[3:0\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4_3_0;
	impl RegisterView for M_4_4_3_0 {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "M_4_4_3_0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
	/// SPSR_EL3\[3:0\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4_3_0_3_0;
	impl RegisterView for M_4_4_3_0_3_0 {
		type Register = super::SPSR_EL3;
		const NAME: &'static str = "M_4_4_3_0_3_0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// AArch64 Memory Model Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_AA64MMFR0_EL1;
impl Register for ID_AA64MMFR0_EL1 {
	const NAME: &'static str = "ID_AA64MMFR0_EL1";
	const LEN: usize = 64;
}

/// ID_AA64MMFR0_EL1 register fields
pub mod id_aa64mmfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64MMFR0_EL1\[63:60\]
	pub struct ECV;
	impl RegisterView for ECV {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "ECV";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[59:56\]
	pub struct FGT;
	impl RegisterView for FGT {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "FGT";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[47:44\]
	pub struct ExS;
	impl RegisterView for ExS {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "ExS";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[43:40\]
	#[allow(non_camel_case_types)]
	pub struct TGran4_2;
	impl RegisterView for TGran4_2 {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "TGran4_2";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[39:36\]
	#[allow(non_camel_case_types)]
	pub struct TGran64_2;
	impl RegisterView for TGran64_2 {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "TGran64_2";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[35:32\]
	#[allow(non_camel_case_types)]
	pub struct TGran16_2;
	impl RegisterView for TGran16_2 {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "TGran16_2";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[31:28\]
	pub struct TGran4;
	impl RegisterView for TGran4 {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "TGran4";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[27:24\]
	pub struct TGran64;
	impl RegisterView for TGran64 {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "TGran64";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[23:20\]
	pub struct TGran16;
	impl RegisterView for TGran16 {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "TGran16";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[19:16\]
	pub struct BigEndEL0;
	impl RegisterView for BigEndEL0 {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "BigEndEL0";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[15:12\]
	pub struct SNSMem;
	impl RegisterView for SNSMem {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "SNSMem";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[11:8\]
	pub struct BigEnd;
	impl RegisterView for BigEnd {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "BigEnd";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[7:4\]
	pub struct ASIDBits;
	impl RegisterView for ASIDBits {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "ASIDBits";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR0_EL1\[3:0\]
	pub struct PARange;
	impl RegisterView for PARange {
		type Register = super::ID_AA64MMFR0_EL1;
		const NAME: &'static str = "PARange";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Secure Configuration Register
#[allow(non_camel_case_types)]
pub struct SCR_EL3;
impl Register for SCR_EL3 {
	const NAME: &'static str = "SCR_EL3";
	const LEN: usize = 64;
}

/// SCR_EL3 register fields
pub mod scr_el3 {
	use crate::core::model::proc::RegisterView;

	/// SCR_EL3\[62\]
	pub struct NSE;
	impl RegisterView for NSE {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "NSE";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[61\]
	pub struct HACDBSEn;
	impl RegisterView for HACDBSEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "HACDBSEn";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[60\]
	pub struct HDBSSEn;
	impl RegisterView for HDBSSEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "HDBSSEn";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[59\]
	pub struct FGTEn2;
	impl RegisterView for FGTEn2 {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "FGTEn2";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[58\]
	pub struct EnDSE;
	impl RegisterView for EnDSE {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EnDSE";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[57\]
	pub struct DSE;
	impl RegisterView for DSE {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "DSE";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[55\]
	pub struct EnIDCP128;
	impl RegisterView for EnIDCP128 {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EnIDCP128";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[53\]
	pub struct PFAREn;
	impl RegisterView for PFAREn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "PFAREn";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[52\]
	pub struct TWERR;
	impl RegisterView for TWERR {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TWERR";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[51\]
	pub struct TMEA;
	impl RegisterView for TMEA {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TMEA";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[50\]
	pub struct EnFPM;
	impl RegisterView for EnFPM {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EnFPM";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[49\]
	pub struct MECEn;
	impl RegisterView for MECEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "MECEn";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[48\]
	pub struct GPF;
	impl RegisterView for GPF {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "GPF";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[47\]
	pub struct D128En;
	impl RegisterView for D128En {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "D128En";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[46\]
	pub struct AIEn;
	impl RegisterView for AIEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "AIEn";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[45\]
	pub struct PIEn;
	impl RegisterView for PIEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "PIEn";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[44\]
	pub struct SCTLR2En;
	impl RegisterView for SCTLR2En {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "SCTLR2En";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[43\]
	pub struct TCR2En;
	impl RegisterView for TCR2En {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TCR2En";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[42\]
	pub struct RCWMASKEn;
	impl RegisterView for RCWMASKEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "RCWMASKEn";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[41\]
	pub struct EnTP2;
	impl RegisterView for EnTP2 {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EnTP2";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[40\]
	pub struct TRNDR;
	impl RegisterView for TRNDR {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TRNDR";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[39\]
	pub struct GCSEn;
	impl RegisterView for GCSEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "GCSEn";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[38\]
	pub struct HXEn;
	impl RegisterView for HXEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "HXEn";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[37\]
	pub struct ADEn;
	impl RegisterView for ADEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "ADEn";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[36\]
	pub struct EnAS0;
	impl RegisterView for EnAS0 {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EnAS0";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[35\]
	pub struct AMVOFFEN;
	impl RegisterView for AMVOFFEN {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "AMVOFFEN";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[34\]
	pub struct TME;
	impl RegisterView for TME {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TME";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[33:30\]
	pub struct TWEDEL;
	impl RegisterView for TWEDEL {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TWEDEL";
		const OFFSET: usize = 30;
		const LEN: usize = 4;
	}

	/// SCR_EL3\[29\]
	pub struct TWEDEn;
	impl RegisterView for TWEDEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TWEDEn";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[28\]
	pub struct ECVEn;
	impl RegisterView for ECVEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "ECVEn";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[27\]
	pub struct FGTEn;
	impl RegisterView for FGTEn {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "FGTEn";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[26\]
	pub struct ATA;
	impl RegisterView for ATA {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "ATA";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[25\]
	pub struct EnSCXT;
	impl RegisterView for EnSCXT {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EnSCXT";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[21\]
	pub struct FIEN;
	impl RegisterView for FIEN {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "FIEN";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[20\]
	pub struct NMEA;
	impl RegisterView for NMEA {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "NMEA";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[19\]
	pub struct EASE;
	impl RegisterView for EASE {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EASE";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[18\]
	pub struct EEL2;
	impl RegisterView for EEL2 {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EEL2";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[17\]
	pub struct API;
	impl RegisterView for API {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "API";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[16\]
	pub struct APK;
	impl RegisterView for APK {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "APK";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[15\]
	pub struct TERR;
	impl RegisterView for TERR {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TERR";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[14\]
	pub struct TLOR;
	impl RegisterView for TLOR {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TLOR";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[13\]
	pub struct TWE;
	impl RegisterView for TWE {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TWE";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[12\]
	pub struct TWI;
	impl RegisterView for TWI {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "TWI";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[11\]
	pub struct ST;
	impl RegisterView for ST {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "ST";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[10\]
	pub struct RW;
	impl RegisterView for RW {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "RW";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[9\]
	pub struct SIF;
	impl RegisterView for SIF {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "SIF";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[8\]
	pub struct HCE;
	impl RegisterView for HCE {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "HCE";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[7\]
	pub struct SMD;
	impl RegisterView for SMD {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "SMD";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[3\]
	pub struct EA;
	impl RegisterView for EA {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "EA";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[2\]
	pub struct FIQ;
	impl RegisterView for FIQ {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "FIQ";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[1\]
	pub struct IRQ;
	impl RegisterView for IRQ {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "IRQ";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// SCR_EL3\[0\]
	pub struct NS;
	impl RegisterView for NS {
		type Register = super::SCR_EL3;
		const NAME: &'static str = "NS";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Hypervisor Fine-Grained Instruction Trap Register 2
#[allow(non_camel_case_types)]
pub struct HFGITR2_EL2;
impl Register for HFGITR2_EL2 {
	const NAME: &'static str = "HFGITR2_EL2";
	const LEN: usize = 64;
}

/// Monitor Debug ROM Address Register
#[allow(non_camel_case_types)]
pub struct MDRAR_EL1;
impl Register for MDRAR_EL1 {
	const NAME: &'static str = "MDRAR_EL1";
	const LEN: usize = 64;
}

/// MDRAR_EL1 register fields
pub mod mdrar_el1 {
	use crate::core::model::proc::RegisterView;

	/// MDRAR_EL1\[55:12\]
	pub struct ROMADDR;
	impl RegisterView for ROMADDR {
		type Register = super::MDRAR_EL1;
		const NAME: &'static str = "ROMADDR";
		const OFFSET: usize = 12;
		const LEN: usize = 44;
	}

	/// MDRAR_EL1\[1:0\]
	pub struct Valid;
	impl RegisterView for Valid {
		type Register = super::MDRAR_EL1;
		const NAME: &'static str = "Valid";
		const OFFSET: usize = 0;
		const LEN: usize = 2;
	}
}

/// SME Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct SMCR_EL3;
impl Register for SMCR_EL3 {
	const NAME: &'static str = "SMCR_EL3";
	const LEN: usize = 64;
}

/// SMCR_EL3 register fields
pub mod smcr_el3 {
	use crate::core::model::proc::RegisterView;

	/// SMCR_EL3\[31\]
	pub struct FA64;
	impl RegisterView for FA64 {
		type Register = super::SMCR_EL3;
		const NAME: &'static str = "FA64";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SMCR_EL3\[30\]
	pub struct EZT0;
	impl RegisterView for EZT0 {
		type Register = super::SMCR_EL3;
		const NAME: &'static str = "EZT0";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SMCR_EL3\[3:0\]
	pub struct LEN;
	impl RegisterView for LEN {
		type Register = super::SMCR_EL3;
		const NAME: &'static str = "LEN";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Counter-timer Virtual Count Register
#[allow(non_camel_case_types)]
pub struct CNTVCT_EL0;
impl Register for CNTVCT_EL0 {
	const NAME: &'static str = "CNTVCT_EL0";
	const LEN: usize = 64;
}

/// AArch64 Auxiliary Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_AA64AFR1_EL1;
impl Register for ID_AA64AFR1_EL1 {
	const NAME: &'static str = "ID_AA64AFR1_EL1";
	const LEN: usize = 64;
}

/// Virtual Deferred Interrupt Status Register (EL3)
#[allow(non_camel_case_types)]
pub struct VDISR_EL3;
impl Register for VDISR_EL3 {
	const NAME: &'static str = "VDISR_EL3";
	const LEN: usize = 64;
}

/// VDISR_EL3 register fields
pub mod vdisr_el3 {
	use crate::core::model::proc::RegisterView;

	/// VDISR_EL3\[31\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::VDISR_EL3;
		const NAME: &'static str = "A";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// VDISR_EL3\[24\]
	pub struct IDS;
	impl RegisterView for IDS {
		type Register = super::VDISR_EL3;
		const NAME: &'static str = "IDS";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// VDISR_EL3\[23:0\]
	pub struct ISS;
	impl RegisterView for ISS {
		type Register = super::VDISR_EL3;
		const NAME: &'static str = "ISS";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Instrumentation Trace Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct TRCITECR_EL1;
impl Register for TRCITECR_EL1 {
	const NAME: &'static str = "TRCITECR_EL1";
	const LEN: usize = 64;
}

/// TRCITECR_EL1 register fields
pub mod trcitecr_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRCITECR_EL1\[1\]
	pub struct E1E;
	impl RegisterView for E1E {
		type Register = super::TRCITECR_EL1;
		const NAME: &'static str = "E1E";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TRCITECR_EL1\[0\]
	pub struct E0E;
	impl RegisterView for E0E {
		type Register = super::TRCITECR_EL1;
		const NAME: &'static str = "E0E";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace ID Register 7
pub struct TRCIDR7;
impl Register for TRCIDR7 {
	const NAME: &'static str = "TRCIDR7";
	const LEN: usize = 64;
}

/// AArch64 Debug Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_AA64DFR0_EL1;
impl Register for ID_AA64DFR0_EL1 {
	const NAME: &'static str = "ID_AA64DFR0_EL1";
	const LEN: usize = 64;
}

/// ID_AA64DFR0_EL1 register fields
pub mod id_aa64dfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64DFR0_EL1\[63:60\]
	pub struct HPMN0;
	impl RegisterView for HPMN0 {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "HPMN0";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[59:56\]
	pub struct ExtTrcBuff;
	impl RegisterView for ExtTrcBuff {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "ExtTrcBuff";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[55:52\]
	pub struct BRBE;
	impl RegisterView for BRBE {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "BRBE";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[51:48\]
	pub struct MTPMU;
	impl RegisterView for MTPMU {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "MTPMU";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[47:44\]
	pub struct TraceBuffer;
	impl RegisterView for TraceBuffer {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "TraceBuffer";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[43:40\]
	pub struct TraceFilt;
	impl RegisterView for TraceFilt {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "TraceFilt";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[39:36\]
	pub struct DoubleLock;
	impl RegisterView for DoubleLock {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "DoubleLock";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[35:32\]
	pub struct PMSVer;
	impl RegisterView for PMSVer {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "PMSVer";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[31:28\]
	#[allow(non_camel_case_types)]
	pub struct CTX_CMPs;
	impl RegisterView for CTX_CMPs {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "CTX_CMPs";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[27:24\]
	pub struct SEBEP;
	impl RegisterView for SEBEP {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "SEBEP";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[23:20\]
	pub struct WRPs;
	impl RegisterView for WRPs {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "WRPs";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[19:16\]
	pub struct PMSS;
	impl RegisterView for PMSS {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "PMSS";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[15:12\]
	pub struct BRPs;
	impl RegisterView for BRPs {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "BRPs";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[11:8\]
	pub struct PMUVer;
	impl RegisterView for PMUVer {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "PMUVer";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[7:4\]
	pub struct TraceVer;
	impl RegisterView for TraceVer {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "TraceVer";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR0_EL1\[3:0\]
	pub struct DebugVer;
	impl RegisterView for DebugVer {
		type Register = super::ID_AA64DFR0_EL1;
		const NAME: &'static str = "DebugVer";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Interrupt Controller System Register Enable Register (EL3)
#[allow(non_camel_case_types)]
pub struct ICC_SRE_EL3;
impl Register for ICC_SRE_EL3 {
	const NAME: &'static str = "ICC_SRE_EL3";
	const LEN: usize = 64;
}

/// ICC_SRE_EL3 register fields
pub mod icc_sre_el3 {
	use crate::core::model::proc::RegisterView;

	/// ICC_SRE_EL3\[3\]
	pub struct Enable;
	impl RegisterView for Enable {
		type Register = super::ICC_SRE_EL3;
		const NAME: &'static str = "Enable";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// ICC_SRE_EL3\[2\]
	pub struct DIB;
	impl RegisterView for DIB {
		type Register = super::ICC_SRE_EL3;
		const NAME: &'static str = "DIB";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// ICC_SRE_EL3\[1\]
	pub struct DFB;
	impl RegisterView for DFB {
		type Register = super::ICC_SRE_EL3;
		const NAME: &'static str = "DFB";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICC_SRE_EL3\[0\]
	pub struct SRE;
	impl RegisterView for SRE {
		type Register = super::ICC_SRE_EL3;
		const NAME: &'static str = "SRE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Instruction Counter Filter Register
#[allow(non_camel_case_types)]
pub struct PMICFILTR_EL0;
impl Register for PMICFILTR_EL0 {
	const NAME: &'static str = "PMICFILTR_EL0";
	const LEN: usize = 64;
}

/// PMICFILTR_EL0 register fields
pub mod pmicfiltr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMICFILTR_EL0\[58\]
	pub struct SYNC;
	impl RegisterView for SYNC {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "SYNC";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[57:56\]
	pub struct VS;
	impl RegisterView for VS {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "VS";
		const OFFSET: usize = 56;
		const LEN: usize = 2;
	}

	/// PMICFILTR_EL0\[31\]
	pub struct P;
	impl RegisterView for P {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "P";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[30\]
	pub struct U;
	impl RegisterView for U {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "U";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[29\]
	pub struct NSK;
	impl RegisterView for NSK {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "NSK";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[28\]
	pub struct NSU;
	impl RegisterView for NSU {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "NSU";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[27\]
	pub struct NSH;
	impl RegisterView for NSH {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "NSH";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[26\]
	pub struct M;
	impl RegisterView for M {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "M";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[24\]
	pub struct SH;
	impl RegisterView for SH {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "SH";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[23\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "T";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[22\]
	pub struct RLK;
	impl RegisterView for RLK {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "RLK";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[21\]
	pub struct RLU;
	impl RegisterView for RLU {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "RLU";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[20\]
	pub struct RLH;
	impl RegisterView for RLH {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "RLH";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// PMICFILTR_EL0\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct evtCount;
	impl RegisterView for evtCount {
		type Register = super::PMICFILTR_EL0;
		const NAME: &'static str = "evtCount";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Auxiliary Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct ACTLR_EL1;
impl Register for ACTLR_EL1 {
	const NAME: &'static str = "ACTLR_EL1";
	const LEN: usize = 64;
}

/// Counter-timer Virtual Timer TimerValue Register
#[allow(non_camel_case_types)]
pub struct CNTV_TVAL_EL0;
impl Register for CNTV_TVAL_EL0 {
	const NAME: &'static str = "CNTV_TVAL_EL0";
	const LEN: usize = 64;
}

/// CNTV_TVAL_EL0 register fields
pub mod cntv_tval_el0 {
	use crate::core::model::proc::RegisterView;

	/// CNTV_TVAL_EL0\[31:0\]
	pub struct TimerValue;
	impl RegisterView for TimerValue {
		type Register = super::CNTV_TVAL_EL0;
		const NAME: &'static str = "TimerValue";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// AArch64 Processor Feature Register 2
#[allow(non_camel_case_types)]
pub struct ID_AA64PFR2_EL1;
impl Register for ID_AA64PFR2_EL1 {
	const NAME: &'static str = "ID_AA64PFR2_EL1";
	const LEN: usize = 64;
}

/// ID_AA64PFR2_EL1 register fields
pub mod id_aa64pfr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64PFR2_EL1\[35:32\]
	pub struct FPMR;
	impl RegisterView for FPMR {
		type Register = super::ID_AA64PFR2_EL1;
		const NAME: &'static str = "FPMR";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR2_EL1\[11:8\]
	pub struct MTEFAR;
	impl RegisterView for MTEFAR {
		type Register = super::ID_AA64PFR2_EL1;
		const NAME: &'static str = "MTEFAR";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR2_EL1\[7:4\]
	pub struct MTESTOREONLY;
	impl RegisterView for MTESTOREONLY {
		type Register = super::ID_AA64PFR2_EL1;
		const NAME: &'static str = "MTESTOREONLY";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR2_EL1\[3:0\]
	pub struct MTEPERM;
	impl RegisterView for MTEPERM {
		type Register = super::ID_AA64PFR2_EL1;
		const NAME: &'static str = "MTEPERM";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Trace ViewInst Start/Stop PE Comparator Control Register
pub struct TRCVIPCSSCTLR;
impl Register for TRCVIPCSSCTLR {
	const NAME: &'static str = "TRCVIPCSSCTLR";
	const LEN: usize = 64;
}

/// Interrupt Controller Virtual End Of Interrupt Register 1
#[allow(non_camel_case_types)]
pub struct ICV_EOIR1_EL1;
impl Register for ICV_EOIR1_EL1 {
	const NAME: &'static str = "ICV_EOIR1_EL1";
	const LEN: usize = 64;
}

/// ICV_EOIR1_EL1 register fields
pub mod icv_eoir1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_EOIR1_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICV_EOIR1_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// SVE Feature ID Register 0
#[allow(non_camel_case_types)]
pub struct ID_AA64ZFR0_EL1;
impl Register for ID_AA64ZFR0_EL1 {
	const NAME: &'static str = "ID_AA64ZFR0_EL1";
	const LEN: usize = 64;
}

/// ID_AA64ZFR0_EL1 register fields
pub mod id_aa64zfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64ZFR0_EL1\[59:56\]
	pub struct F64MM;
	impl RegisterView for F64MM {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "F64MM";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[55:52\]
	pub struct F32MM;
	impl RegisterView for F32MM {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "F32MM";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[47:44\]
	pub struct I8MM;
	impl RegisterView for I8MM {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "I8MM";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[43:40\]
	pub struct SM4;
	impl RegisterView for SM4 {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "SM4";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[35:32\]
	pub struct SHA3;
	impl RegisterView for SHA3 {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "SHA3";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[27:24\]
	pub struct B16B16;
	impl RegisterView for B16B16 {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "B16B16";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[23:20\]
	pub struct BF16;
	impl RegisterView for BF16 {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "BF16";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[19:16\]
	pub struct BitPerm;
	impl RegisterView for BitPerm {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "BitPerm";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[7:4\]
	pub struct AES;
	impl RegisterView for AES {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "AES";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64ZFR0_EL1\[3:0\]
	pub struct SVEver;
	impl RegisterView for SVEver {
		type Register = super::ID_AA64ZFR0_EL1;
		const NAME: &'static str = "SVEver";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Monitor DCC Status Register
#[allow(non_camel_case_types)]
pub struct MDCCSR_EL0;
impl Register for MDCCSR_EL0 {
	const NAME: &'static str = "MDCCSR_EL0";
	const LEN: usize = 64;
}

/// MDCCSR_EL0 register fields
pub mod mdccsr_el0 {
	use crate::core::model::proc::RegisterView;

	/// MDCCSR_EL0\[30\]
	pub struct RXfull;
	impl RegisterView for RXfull {
		type Register = super::MDCCSR_EL0;
		const NAME: &'static str = "RXfull";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// MDCCSR_EL0\[29\]
	pub struct TXfull;
	impl RegisterView for TXfull {
		type Register = super::MDCCSR_EL0;
		const NAME: &'static str = "TXfull";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller Empty List Register Status Register
#[allow(non_camel_case_types)]
pub struct ICH_ELRSR_EL2;
impl Register for ICH_ELRSR_EL2 {
	const NAME: &'static str = "ICH_ELRSR_EL2";
	const LEN: usize = 64;
}

/// Interrupt Controller System Register Enable Register (EL2)
#[allow(non_camel_case_types)]
pub struct ICC_SRE_EL2;
impl Register for ICC_SRE_EL2 {
	const NAME: &'static str = "ICC_SRE_EL2";
	const LEN: usize = 64;
}

/// ICC_SRE_EL2 register fields
pub mod icc_sre_el2 {
	use crate::core::model::proc::RegisterView;

	/// ICC_SRE_EL2\[3\]
	pub struct Enable;
	impl RegisterView for Enable {
		type Register = super::ICC_SRE_EL2;
		const NAME: &'static str = "Enable";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// ICC_SRE_EL2\[2\]
	pub struct DIB;
	impl RegisterView for DIB {
		type Register = super::ICC_SRE_EL2;
		const NAME: &'static str = "DIB";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// ICC_SRE_EL2\[1\]
	pub struct DFB;
	impl RegisterView for DFB {
		type Register = super::ICC_SRE_EL2;
		const NAME: &'static str = "DFB";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICC_SRE_EL2\[0\]
	pub struct SRE;
	impl RegisterView for SRE {
		type Register = super::ICC_SRE_EL2;
		const NAME: &'static str = "SRE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Saved Program Status Register (Abort mode)
#[allow(non_camel_case_types)]
pub struct SPSR_abt;
impl Register for SPSR_abt {
	const NAME: &'static str = "SPSR_abt";
	const LEN: usize = 64;
}

/// SPSR_abt register fields
pub mod spsr_abt {
	use crate::core::model::proc::RegisterView;

	/// SPSR_abt\[31\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "N";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[30\]
	pub struct Z;
	impl RegisterView for Z {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "Z";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[29\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "C";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[28\]
	pub struct V;
	impl RegisterView for V {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[27\]
	pub struct Q;
	impl RegisterView for Q {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "Q";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[15:10\]
	pub struct IT;
	impl RegisterView for IT {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "IT";
		const OFFSET: usize = 10;
		const LEN: usize = 6;
	}

	/// SPSR_abt\[24\]
	pub struct J;
	impl RegisterView for J {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "J";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[23\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[22\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[21\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[20\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[19:16\]
	pub struct GE;
	impl RegisterView for GE {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "GE";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SPSR_abt\[9\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "E";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[8\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "A";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[7\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "I";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[6\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "F";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[5\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "T";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SPSR_abt\[4:0\]
	pub struct M;
	impl RegisterView for M {
		type Register = super::SPSR_abt;
		const NAME: &'static str = "M";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Trace ID Register 6
pub struct TRCIDR6;
impl Register for TRCIDR6 {
	const NAME: &'static str = "TRCIDR6";
	const LEN: usize = 64;
}

/// TRCIDR6 register fields
pub mod trcidr6 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR6\[2\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL2;
	impl RegisterView for EXLEVEL_RL_EL2 {
		type Register = super::TRCIDR6;
		const NAME: &'static str = "EXLEVEL_RL_EL2";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// TRCIDR6\[1\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL1;
	impl RegisterView for EXLEVEL_RL_EL1 {
		type Register = super::TRCIDR6;
		const NAME: &'static str = "EXLEVEL_RL_EL1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TRCIDR6\[0\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL0;
	impl RegisterView for EXLEVEL_RL_EL0 {
		type Register = super::TRCIDR6;
		const NAME: &'static str = "EXLEVEL_RL_EL0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// MPAM Streaming Mode Register
#[allow(non_camel_case_types)]
pub struct MPAMSM_EL1;
impl Register for MPAMSM_EL1 {
	const NAME: &'static str = "MPAMSM_EL1";
	const LEN: usize = 64;
}

/// MPAMSM_EL1 register fields
pub mod mpamsm_el1 {
	use crate::core::model::proc::RegisterView;

	/// MPAMSM_EL1\[47:40\]
	#[allow(non_camel_case_types)]
	pub struct PMG_D;
	impl RegisterView for PMG_D {
		type Register = super::MPAMSM_EL1;
		const NAME: &'static str = "PMG_D";
		const OFFSET: usize = 40;
		const LEN: usize = 8;
	}

	/// MPAMSM_EL1\[31:16\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_D;
	impl RegisterView for PARTID_D {
		type Register = super::MPAMSM_EL1;
		const NAME: &'static str = "PARTID_D";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}
}

/// Sampling Filter Control Register
#[allow(non_camel_case_types)]
pub struct PMSFCR_EL1;
impl Register for PMSFCR_EL1 {
	const NAME: &'static str = "PMSFCR_EL1";
	const LEN: usize = 64;
}

/// PMSFCR_EL1 register fields
pub mod pmsfcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSFCR_EL1\[52\]
	pub struct SIMDm;
	impl RegisterView for SIMDm {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "SIMDm";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[51\]
	pub struct FPm;
	impl RegisterView for FPm {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "FPm";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[50\]
	pub struct STm;
	impl RegisterView for STm {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "STm";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[49\]
	pub struct LDm;
	impl RegisterView for LDm {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "LDm";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[48\]
	pub struct Bm;
	impl RegisterView for Bm {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "Bm";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[20\]
	pub struct SIMD;
	impl RegisterView for SIMD {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "SIMD";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[19\]
	pub struct FP;
	impl RegisterView for FP {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "FP";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[18\]
	pub struct ST;
	impl RegisterView for ST {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "ST";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[17\]
	pub struct LD;
	impl RegisterView for LD {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "LD";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[16\]
	pub struct B;
	impl RegisterView for B {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "B";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[4\]
	pub struct FDS;
	impl RegisterView for FDS {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "FDS";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[3\]
	pub struct FnE;
	impl RegisterView for FnE {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "FnE";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[2\]
	pub struct FL;
	impl RegisterView for FL {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "FL";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[1\]
	pub struct FT;
	impl RegisterView for FT {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "FT";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// PMSFCR_EL1\[0\]
	pub struct FE;
	impl RegisterView for FE {
		type Register = super::PMSFCR_EL1;
		const NAME: &'static str = "FE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Virtual Deferred Interrupt Status Register (EL2)
#[allow(non_camel_case_types)]
pub struct VDISR_EL2;
impl Register for VDISR_EL2 {
	const NAME: &'static str = "VDISR_EL2";
	const LEN: usize = 64;
}

/// VDISR_EL2 register fields
pub mod vdisr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VDISR_EL2\[31\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::VDISR_EL2;
		const NAME: &'static str = "A";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// VDISR_EL2\[24\]
	pub struct IDS;
	impl RegisterView for IDS {
		type Register = super::VDISR_EL2;
		const NAME: &'static str = "IDS";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// VDISR_EL2\[23:0\]
	pub struct ISS;
	impl RegisterView for ISS {
		type Register = super::VDISR_EL2;
		const NAME: &'static str = "ISS";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}

	/// VDISR_EL2\[15:14\]
	pub struct AET;
	impl RegisterView for AET {
		type Register = super::VDISR_EL2;
		const NAME: &'static str = "AET";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// VDISR_EL2\[12\]
	pub struct ExT;
	impl RegisterView for ExT {
		type Register = super::VDISR_EL2;
		const NAME: &'static str = "ExT";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// VDISR_EL2\[3:0\]
	pub struct FS;
	impl RegisterView for FS {
		type Register = super::VDISR_EL2;
		const NAME: &'static str = "FS";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}

	/// VDISR_EL2\[9\]
	pub struct LPAE;
	impl RegisterView for LPAE {
		type Register = super::VDISR_EL2;
		const NAME: &'static str = "LPAE";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// VDISR_EL2\[5:0\]
	pub struct STATUS;
	impl RegisterView for STATUS {
		type Register = super::VDISR_EL2;
		const NAME: &'static str = "STATUS";
		const OFFSET: usize = 0;
		const LEN: usize = 6;
	}
}

/// SME Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct SMCR_EL2;
impl Register for SMCR_EL2 {
	const NAME: &'static str = "SMCR_EL2";
	const LEN: usize = 64;
}

/// SMCR_EL2 register fields
pub mod smcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// SMCR_EL2\[31\]
	pub struct FA64;
	impl RegisterView for FA64 {
		type Register = super::SMCR_EL2;
		const NAME: &'static str = "FA64";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SMCR_EL2\[30\]
	pub struct EZT0;
	impl RegisterView for EZT0 {
		type Register = super::SMCR_EL2;
		const NAME: &'static str = "EZT0";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SMCR_EL2\[3:0\]
	pub struct LEN;
	impl RegisterView for LEN {
		type Register = super::SMCR_EL2;
		const NAME: &'static str = "LEN";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Hypervisor IPA Fault Address Register
#[allow(non_camel_case_types)]
pub struct HPFAR_EL2;
impl Register for HPFAR_EL2 {
	const NAME: &'static str = "HPFAR_EL2";
	const LEN: usize = 64;
}

/// HPFAR_EL2 register fields
pub mod hpfar_el2 {
	use crate::core::model::proc::RegisterView;

	/// HPFAR_EL2\[63\]
	pub struct NS;
	impl RegisterView for NS {
		type Register = super::HPFAR_EL2;
		const NAME: &'static str = "NS";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// HPFAR_EL2\[47:4\]
	pub struct FIPA;
	impl RegisterView for FIPA {
		type Register = super::HPFAR_EL2;
		const NAME: &'static str = "FIPA";
		const OFFSET: usize = 4;
		const LEN: usize = 44;
	}
}

/// Saved Program Status Register (EL2)
#[allow(non_camel_case_types)]
pub struct SPSR_EL2;
impl Register for SPSR_EL2 {
	const NAME: &'static str = "SPSR_EL2";
	const LEN: usize = 64;
}

/// SPSR_EL2 register fields
pub mod spsr_el2 {
	use crate::core::model::proc::RegisterView;

	/// SPSR_EL2\[33\]
	pub struct PPEND;
	impl RegisterView for PPEND {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "PPEND";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[31\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "N";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[30\]
	pub struct Z;
	impl RegisterView for Z {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "Z";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[29\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "C";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[28\]
	pub struct V;
	impl RegisterView for V {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[27\]
	pub struct Q;
	impl RegisterView for Q {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "Q";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[15:10\]
	pub struct IT;
	impl RegisterView for IT {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "IT";
		const OFFSET: usize = 10;
		const LEN: usize = 6;
	}

	/// SPSR_EL2\[24\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[12\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[22\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[21\]
	pub struct SS;
	impl RegisterView for SS {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "SS";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[20\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[19:16\]
	pub struct GE;
	impl RegisterView for GE {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "GE";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SPSR_EL2\[9\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "E";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[8\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "A";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[7\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "I";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[6\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "F";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[5\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "T";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[35\]
	pub struct PACM;
	impl RegisterView for PACM {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "PACM";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[34\]
	pub struct EXLOCK;
	impl RegisterView for EXLOCK {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "EXLOCK";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[32\]
	pub struct PM;
	impl RegisterView for PM {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "PM";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[25\]
	pub struct TCO;
	impl RegisterView for TCO {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "TCO";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[23\]
	pub struct UAO;
	impl RegisterView for UAO {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "UAO";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[13\]
	pub struct ALLINT;
	impl RegisterView for ALLINT {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "ALLINT";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[11:10\]
	pub struct BTYPE;
	impl RegisterView for BTYPE {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "BTYPE";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// SPSR_EL2\[9\]
	pub struct D;
	impl RegisterView for D {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "D";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_EL2\[4\]
	#[allow(non_camel_case_types)]
	pub struct M_4;
	impl RegisterView for M_4 {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "M_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// SPSR_EL2\[4\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4;
	impl RegisterView for M_4_4 {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "M_4_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// SPSR_EL2\[3:0\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4_3_0;
	impl RegisterView for M_4_4_3_0 {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "M_4_4_3_0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
	/// SPSR_EL2\[3:0\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4_3_0_3_0;
	impl RegisterView for M_4_4_3_0_3_0 {
		type Register = super::SPSR_EL2;
		const NAME: &'static str = "M_4_4_3_0_3_0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Granule Protection Check Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct GPCCR_EL3;
impl Register for GPCCR_EL3 {
	const NAME: &'static str = "GPCCR_EL3";
	const LEN: usize = 64;
}

/// GPCCR_EL3 register fields
pub mod gpccr_el3 {
	use crate::core::model::proc::RegisterView;

	/// GPCCR_EL3\[24\]
	pub struct APPSAA;
	impl RegisterView for APPSAA {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "APPSAA";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// GPCCR_EL3\[23:20\]
	pub struct L0GPTSZ;
	impl RegisterView for L0GPTSZ {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "L0GPTSZ";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// GPCCR_EL3\[19\]
	pub struct NSO;
	impl RegisterView for NSO {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "NSO";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// GPCCR_EL3\[18\]
	pub struct TBGPCD;
	impl RegisterView for TBGPCD {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "TBGPCD";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// GPCCR_EL3\[17\]
	pub struct GPCP;
	impl RegisterView for GPCP {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "GPCP";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// GPCCR_EL3\[16\]
	pub struct GPC;
	impl RegisterView for GPC {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "GPC";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// GPCCR_EL3\[15:14\]
	pub struct PGS;
	impl RegisterView for PGS {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "PGS";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// GPCCR_EL3\[13:12\]
	pub struct SH;
	impl RegisterView for SH {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "SH";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// GPCCR_EL3\[11:10\]
	pub struct ORGN;
	impl RegisterView for ORGN {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "ORGN";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// GPCCR_EL3\[9:8\]
	pub struct IRGN;
	impl RegisterView for IRGN {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "IRGN";
		const OFFSET: usize = 8;
		const LEN: usize = 2;
	}

	/// GPCCR_EL3\[7\]
	pub struct SPAD;
	impl RegisterView for SPAD {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "SPAD";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// GPCCR_EL3\[6\]
	pub struct NSPAD;
	impl RegisterView for NSPAD {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "NSPAD";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// GPCCR_EL3\[5\]
	pub struct RLPAD;
	impl RegisterView for RLPAD {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "RLPAD";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// GPCCR_EL3\[2:0\]
	pub struct PPS;
	impl RegisterView for PPS {
		type Register = super::GPCCR_EL3;
		const NAME: &'static str = "PPS";
		const OFFSET: usize = 0;
		const LEN: usize = 3;
	}
}

/// Performance Monitors Event Counter Selection Register
#[allow(non_camel_case_types)]
pub struct PMSELR_EL0;
impl Register for PMSELR_EL0 {
	const NAME: &'static str = "PMSELR_EL0";
	const LEN: usize = 64;
}

/// PMSELR_EL0 register fields
pub mod pmselr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMSELR_EL0\[4:0\]
	pub struct SEL;
	impl RegisterView for SEL {
		type Register = super::PMSELR_EL0;
		const NAME: &'static str = "SEL";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// System Performance Monitors Secure Control Register
#[allow(non_camel_case_types)]
pub struct SPMSCR_EL1;
impl Register for SPMSCR_EL1 {
	const NAME: &'static str = "SPMSCR_EL1";
	const LEN: usize = 64;
}

/// SPMSCR_EL1 register fields
pub mod spmscr_el1 {
	use crate::core::model::proc::RegisterView;

	/// SPMSCR_EL1\[4\]
	pub struct NAO;
	impl RegisterView for NAO {
		type Register = super::SPMSCR_EL1;
		const NAME: &'static str = "NAO";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// SPMSCR_EL1\[0\]
	pub struct SO;
	impl RegisterView for SO {
		type Register = super::SPMSCR_EL1;
		const NAME: &'static str = "SO";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Exception Link Register (EL2)
#[allow(non_camel_case_types)]
pub struct ELR_EL2;
impl Register for ELR_EL2 {
	const NAME: &'static str = "ELR_EL2";
	const LEN: usize = 64;
}

/// Trace Buffer Limit Address Register
#[allow(non_camel_case_types)]
pub struct TRBLIMITR_EL1;
impl Register for TRBLIMITR_EL1 {
	const NAME: &'static str = "TRBLIMITR_EL1";
	const LEN: usize = 64;
}

/// TRBLIMITR_EL1 register fields
pub mod trblimitr_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRBLIMITR_EL1\[63:12\]
	pub struct LIMIT;
	impl RegisterView for LIMIT {
		type Register = super::TRBLIMITR_EL1;
		const NAME: &'static str = "LIMIT";
		const OFFSET: usize = 12;
		const LEN: usize = 52;
	}

	/// TRBLIMITR_EL1\[6\]
	pub struct XE;
	impl RegisterView for XE {
		type Register = super::TRBLIMITR_EL1;
		const NAME: &'static str = "XE";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// TRBLIMITR_EL1\[5\]
	#[allow(non_camel_case_types)]
	pub struct nVM;
	impl RegisterView for nVM {
		type Register = super::TRBLIMITR_EL1;
		const NAME: &'static str = "nVM";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// TRBLIMITR_EL1\[4:3\]
	pub struct TM;
	impl RegisterView for TM {
		type Register = super::TRBLIMITR_EL1;
		const NAME: &'static str = "TM";
		const OFFSET: usize = 3;
		const LEN: usize = 2;
	}

	/// TRBLIMITR_EL1\[2:1\]
	pub struct FM;
	impl RegisterView for FM {
		type Register = super::TRBLIMITR_EL1;
		const NAME: &'static str = "FM";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// TRBLIMITR_EL1\[0\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::TRBLIMITR_EL1;
		const NAME: &'static str = "E";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// LORegion Control (EL1)
#[allow(non_camel_case_types)]
pub struct LORC_EL1;
impl Register for LORC_EL1 {
	const NAME: &'static str = "LORC_EL1";
	const LEN: usize = 64;
}

/// LORC_EL1 register fields
pub mod lorc_el1 {
	use crate::core::model::proc::RegisterView;

	/// LORC_EL1\[9:2\]
	pub struct DS;
	impl RegisterView for DS {
		type Register = super::LORC_EL1;
		const NAME: &'static str = "DS";
		const OFFSET: usize = 2;
		const LEN: usize = 8;
	}

	/// LORC_EL1\[0\]
	pub struct EN;
	impl RegisterView for EN {
		type Register = super::LORC_EL1;
		const NAME: &'static str = "EN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Hypervisor Fine-Grained Write Trap Register 2
#[allow(non_camel_case_types)]
pub struct HFGWTR2_EL2;
impl Register for HFGWTR2_EL2 {
	const NAME: &'static str = "HFGWTR2_EL2";
	const LEN: usize = 64;
}

/// HFGWTR2_EL2 register fields
pub mod hfgwtr2_el2 {
	use crate::core::model::proc::RegisterView;

	/// HFGWTR2_EL2\[2\]
	#[allow(non_camel_case_types)]
	pub struct nRCWSMASK_EL1;
	impl RegisterView for nRCWSMASK_EL1 {
		type Register = super::HFGWTR2_EL2;
		const NAME: &'static str = "nRCWSMASK_EL1";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HFGWTR2_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct nPFAR_EL1;
	impl RegisterView for nPFAR_EL1 {
		type Register = super::HFGWTR2_EL2;
		const NAME: &'static str = "nPFAR_EL1";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// LORegion End Address (EL1)
#[allow(non_camel_case_types)]
pub struct LOREA_EL1;
impl Register for LOREA_EL1 {
	const NAME: &'static str = "LOREA_EL1";
	const LEN: usize = 64;
}

/// LOREA_EL1 register fields
pub mod lorea_el1 {
	use crate::core::model::proc::RegisterView;

	/// LOREA_EL1\[55:52\]
	#[allow(non_camel_case_types)]
	pub struct EA_55_52;
	impl RegisterView for EA_55_52 {
		type Register = super::LOREA_EL1;
		const NAME: &'static str = "EA_55_52";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}
	/// LOREA_EL1\[51:48\]
	#[allow(non_camel_case_types)]
	pub struct EA_55_52_51_48;
	impl RegisterView for EA_55_52_51_48 {
		type Register = super::LOREA_EL1;
		const NAME: &'static str = "EA_55_52_51_48";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}
	/// LOREA_EL1\[47:16\]
	#[allow(non_camel_case_types)]
	pub struct EA_55_52_51_48_47_16;
	impl RegisterView for EA_55_52_51_48_47_16 {
		type Register = super::LOREA_EL1;
		const NAME: &'static str = "EA_55_52_51_48_47_16";
		const OFFSET: usize = 16;
		const LEN: usize = 32;
	}
}

/// Guarded Control Stack Control Register (EL0)
#[allow(non_camel_case_types)]
pub struct GCSCRE0_EL1;
impl Register for GCSCRE0_EL1 {
	const NAME: &'static str = "GCSCRE0_EL1";
	const LEN: usize = 64;
}

/// GCSCRE0_EL1 register fields
pub mod gcscre0_el1 {
	use crate::core::model::proc::RegisterView;

	/// GCSCRE0_EL1\[10\]
	#[allow(non_camel_case_types)]
	pub struct nTR;
	impl RegisterView for nTR {
		type Register = super::GCSCRE0_EL1;
		const NAME: &'static str = "nTR";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// GCSCRE0_EL1\[9\]
	pub struct STREn;
	impl RegisterView for STREn {
		type Register = super::GCSCRE0_EL1;
		const NAME: &'static str = "STREn";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// GCSCRE0_EL1\[8\]
	pub struct PUSHMEn;
	impl RegisterView for PUSHMEn {
		type Register = super::GCSCRE0_EL1;
		const NAME: &'static str = "PUSHMEn";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// GCSCRE0_EL1\[5\]
	pub struct RVCHKEN;
	impl RegisterView for RVCHKEN {
		type Register = super::GCSCRE0_EL1;
		const NAME: &'static str = "RVCHKEN";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// GCSCRE0_EL1\[0\]
	pub struct PCRSEL;
	impl RegisterView for PCRSEL {
		type Register = super::GCSCRE0_EL1;
		const NAME: &'static str = "PCRSEL";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace ViewInst Main Control Register
pub struct TRCVICTLR;
impl Register for TRCVICTLR {
	const NAME: &'static str = "TRCVICTLR";
	const LEN: usize = 64;
}

/// TRCVICTLR register fields
pub mod trcvictlr {
	use crate::core::model::proc::RegisterView;

	/// TRCVICTLR\[26\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL2;
	impl RegisterView for EXLEVEL_RL_EL2 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_RL_EL2";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[25\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL1;
	impl RegisterView for EXLEVEL_RL_EL1 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_RL_EL1";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[24\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL0;
	impl RegisterView for EXLEVEL_RL_EL0 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_RL_EL0";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[22\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL2;
	impl RegisterView for EXLEVEL_NS_EL2 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_NS_EL2";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[21\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL1;
	impl RegisterView for EXLEVEL_NS_EL1 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_NS_EL1";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[20\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL0;
	impl RegisterView for EXLEVEL_NS_EL0 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_NS_EL0";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[19\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL3;
	impl RegisterView for EXLEVEL_S_EL3 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_S_EL3";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[18\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL2;
	impl RegisterView for EXLEVEL_S_EL2 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_S_EL2";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[17\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL1;
	impl RegisterView for EXLEVEL_S_EL1 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_S_EL1";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[16\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL0;
	impl RegisterView for EXLEVEL_S_EL0 {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EXLEVEL_S_EL0";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[11\]
	pub struct TRCERR;
	impl RegisterView for TRCERR {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "TRCERR";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[10\]
	pub struct TRCRESET;
	impl RegisterView for TRCRESET {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "TRCRESET";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[9\]
	pub struct SSSTATUS;
	impl RegisterView for SSSTATUS {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "SSSTATUS";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[7\]
	#[allow(non_camel_case_types)]
	pub struct EVENT_TYPE;
	impl RegisterView for EVENT_TYPE {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EVENT_TYPE";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TRCVICTLR\[4:0\]
	#[allow(non_camel_case_types)]
	pub struct EVENT_SEL;
	impl RegisterView for EVENT_SEL {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "EVENT_SEL";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}

	/// TRCVICTLR\[4:0\]
	pub struct Reserved;
	impl RegisterView for Reserved {
		type Register = super::TRCVICTLR;
		const NAME: &'static str = "Reserved";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// AArch64 Memory Model Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_AA64MMFR1_EL1;
impl Register for ID_AA64MMFR1_EL1 {
	const NAME: &'static str = "ID_AA64MMFR1_EL1";
	const LEN: usize = 64;
}

/// ID_AA64MMFR1_EL1 register fields
pub mod id_aa64mmfr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64MMFR1_EL1\[63:60\]
	pub struct ECBHB;
	impl RegisterView for ECBHB {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "ECBHB";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[59:56\]
	pub struct CMOW;
	impl RegisterView for CMOW {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "CMOW";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[55:52\]
	pub struct TIDCP1;
	impl RegisterView for TIDCP1 {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "TIDCP1";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[51:48\]
	#[allow(non_camel_case_types)]
	pub struct nTLBPA;
	impl RegisterView for nTLBPA {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "nTLBPA";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[47:44\]
	pub struct AFP;
	impl RegisterView for AFP {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "AFP";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[43:40\]
	pub struct HCX;
	impl RegisterView for HCX {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "HCX";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[39:36\]
	pub struct ETS;
	impl RegisterView for ETS {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "ETS";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[35:32\]
	pub struct TWED;
	impl RegisterView for TWED {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "TWED";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[31:28\]
	pub struct XNX;
	impl RegisterView for XNX {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "XNX";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[27:24\]
	pub struct SpecSEI;
	impl RegisterView for SpecSEI {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "SpecSEI";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[23:20\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[19:16\]
	pub struct LO;
	impl RegisterView for LO {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "LO";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[15:12\]
	pub struct HPDS;
	impl RegisterView for HPDS {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "HPDS";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[11:8\]
	pub struct VH;
	impl RegisterView for VH {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "VH";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[7:4\]
	pub struct VMIDBits;
	impl RegisterView for VMIDBits {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "VMIDBits";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR1_EL1\[3:0\]
	pub struct HAFDBS;
	impl RegisterView for HAFDBS {
		type Register = super::ID_AA64MMFR1_EL1;
		const NAME: &'static str = "HAFDBS";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Trace Single-shot Comparator Control Register \<n\>, n = 7 - 0
pub struct TRCSSCCRn;
impl RegisterArray for TRCSSCCRn {
	const NAME: &'static str = "TRCSSCCRn";
	const ELEMS: usize = 7;
	const ELEM_LEN: usize = 64;
}

/// TRCSSCCRn register fields
pub mod trcssccrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCSSCCRn\[24\]
	pub struct RST;
	impl RegisterArrayView for RST {
		type RegisterArray = super::TRCSSCCRn;
		const NAME: &'static str = "RST";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}
}

/// AArch64 Auxiliary Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_AA64AFR0_EL1;
impl Register for ID_AA64AFR0_EL1 {
	const NAME: &'static str = "ID_AA64AFR0_EL1";
	const LEN: usize = 64;
}

/// Error Record ID Register
#[allow(non_camel_case_types)]
pub struct ERRIDR_EL1;
impl Register for ERRIDR_EL1 {
	const NAME: &'static str = "ERRIDR_EL1";
	const LEN: usize = 64;
}

/// ERRIDR_EL1 register fields
pub mod erridr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ERRIDR_EL1\[15:0\]
	pub struct NUM;
	impl RegisterView for NUM {
		type Register = super::ERRIDR_EL1;
		const NAME: &'static str = "NUM";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// EL1 Read/Write Software Context Number
#[allow(non_camel_case_types)]
pub struct SCXTNUM_EL1;
impl Register for SCXTNUM_EL1 {
	const NAME: &'static str = "SCXTNUM_EL1";
	const LEN: usize = 64;
}

/// EL2 Software Thread ID Register
#[allow(non_camel_case_types)]
pub struct TPIDR_EL2;
impl Register for TPIDR_EL2 {
	const NAME: &'static str = "TPIDR_EL2";
	const LEN: usize = 64;
}

/// AArch32 Processor Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_PFR0_EL1;
impl Register for ID_PFR0_EL1 {
	const NAME: &'static str = "ID_PFR0_EL1";
	const LEN: usize = 64;
}

/// ID_PFR0_EL1 register fields
pub mod id_pfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_PFR0_EL1\[31:28\]
	pub struct RAS;
	impl RegisterView for RAS {
		type Register = super::ID_PFR0_EL1;
		const NAME: &'static str = "RAS";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_PFR0_EL1\[27:24\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::ID_PFR0_EL1;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_PFR0_EL1\[23:20\]
	pub struct AMU;
	impl RegisterView for AMU {
		type Register = super::ID_PFR0_EL1;
		const NAME: &'static str = "AMU";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_PFR0_EL1\[19:16\]
	pub struct CSV2;
	impl RegisterView for CSV2 {
		type Register = super::ID_PFR0_EL1;
		const NAME: &'static str = "CSV2";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_PFR0_EL1\[15:12\]
	pub struct State3;
	impl RegisterView for State3 {
		type Register = super::ID_PFR0_EL1;
		const NAME: &'static str = "State3";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_PFR0_EL1\[11:8\]
	pub struct State2;
	impl RegisterView for State2 {
		type Register = super::ID_PFR0_EL1;
		const NAME: &'static str = "State2";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_PFR0_EL1\[7:4\]
	pub struct State1;
	impl RegisterView for State1 {
		type Register = super::ID_PFR0_EL1;
		const NAME: &'static str = "State1";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_PFR0_EL1\[3:0\]
	pub struct State0;
	impl RegisterView for State0 {
		type Register = super::ID_PFR0_EL1;
		const NAME: &'static str = "State0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Trace Sequencer State Register
pub struct TRCSEQSTR;
impl Register for TRCSEQSTR {
	const NAME: &'static str = "TRCSEQSTR";
	const LEN: usize = 64;
}

/// TRCSEQSTR register fields
pub mod trcseqstr {
	use crate::core::model::proc::RegisterView;

	/// TRCSEQSTR\[1:0\]
	pub struct STATE;
	impl RegisterView for STATE {
		type Register = super::TRCSEQSTR;
		const NAME: &'static str = "STATE";
		const OFFSET: usize = 0;
		const LEN: usize = 2;
	}
}

/// OS Lock Status Register
#[allow(non_camel_case_types)]
pub struct OSLSR_EL1;
impl Register for OSLSR_EL1 {
	const NAME: &'static str = "OSLSR_EL1";
	const LEN: usize = 64;
}

/// OSLSR_EL1 register fields
pub mod oslsr_el1 {
	use crate::core::model::proc::RegisterView;

	/// OSLSR_EL1\[0\]
	pub struct OSLM;
	impl RegisterView for OSLM {
		type Register = super::OSLSR_EL1;
		const NAME: &'static str = "OSLM";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}

	/// OSLSR_EL1\[2\]
	#[allow(non_camel_case_types)]
	pub struct nTT;
	impl RegisterView for nTT {
		type Register = super::OSLSR_EL1;
		const NAME: &'static str = "nTT";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// OSLSR_EL1\[1\]
	pub struct OSLK;
	impl RegisterView for OSLK {
		type Register = super::OSLSR_EL1;
		const NAME: &'static str = "OSLK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}
}

/// Activity Monitors Count Enable Set Register 1
#[allow(non_camel_case_types)]
pub struct AMCNTENSET1_EL0;
impl Register for AMCNTENSET1_EL0 {
	const NAME: &'static str = "AMCNTENSET1_EL0";
	const LEN: usize = 64;
}

/// AArch32 Memory Model Feature Register 5
#[allow(non_camel_case_types)]
pub struct ID_MMFR5_EL1;
impl Register for ID_MMFR5_EL1 {
	const NAME: &'static str = "ID_MMFR5_EL1";
	const LEN: usize = 64;
}

/// ID_MMFR5_EL1 register fields
pub mod id_mmfr5_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_MMFR5_EL1\[7:4\]
	#[allow(non_camel_case_types)]
	pub struct nTLBPA;
	impl RegisterView for nTLBPA {
		type Register = super::ID_MMFR5_EL1;
		const NAME: &'static str = "nTLBPA";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_MMFR5_EL1\[3:0\]
	pub struct ETS;
	impl RegisterView for ETS {
		type Register = super::ID_MMFR5_EL1;
		const NAME: &'static str = "ETS";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Streaming Mode Identification Register
#[allow(non_camel_case_types)]
pub struct SMIDR_EL1;
impl Register for SMIDR_EL1 {
	const NAME: &'static str = "SMIDR_EL1";
	const LEN: usize = 64;
}

/// SMIDR_EL1 register fields
pub mod smidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// SMIDR_EL1\[51:32\]
	pub struct Affinity2;
	impl RegisterView for Affinity2 {
		type Register = super::SMIDR_EL1;
		const NAME: &'static str = "Affinity2";
		const OFFSET: usize = 32;
		const LEN: usize = 20;
	}

	/// SMIDR_EL1\[31:24\]
	pub struct Implementer;
	impl RegisterView for Implementer {
		type Register = super::SMIDR_EL1;
		const NAME: &'static str = "Implementer";
		const OFFSET: usize = 24;
		const LEN: usize = 8;
	}

	/// SMIDR_EL1\[23:16\]
	pub struct Revision;
	impl RegisterView for Revision {
		type Register = super::SMIDR_EL1;
		const NAME: &'static str = "Revision";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// SMIDR_EL1\[15\]
	pub struct SMPS;
	impl RegisterView for SMPS {
		type Register = super::SMIDR_EL1;
		const NAME: &'static str = "SMPS";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// SMIDR_EL1\[14:13\]
	pub struct SH;
	impl RegisterView for SH {
		type Register = super::SMIDR_EL1;
		const NAME: &'static str = "SH";
		const OFFSET: usize = 13;
		const LEN: usize = 2;
	}

	/// SMIDR_EL1\[11:0\]
	pub struct Affinity;
	impl RegisterView for Affinity {
		type Register = super::SMIDR_EL1;
		const NAME: &'static str = "Affinity";
		const OFFSET: usize = 0;
		const LEN: usize = 12;
	}
}

/// Interrupt Controller Alias Software Generated Interrupt Group 1 Register
#[allow(non_camel_case_types)]
pub struct ICC_ASGI1R_EL1;
impl Register for ICC_ASGI1R_EL1 {
	const NAME: &'static str = "ICC_ASGI1R_EL1";
	const LEN: usize = 64;
}

/// ICC_ASGI1R_EL1 register fields
pub mod icc_asgi1r_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_ASGI1R_EL1\[55:48\]
	pub struct Aff3;
	impl RegisterView for Aff3 {
		type Register = super::ICC_ASGI1R_EL1;
		const NAME: &'static str = "Aff3";
		const OFFSET: usize = 48;
		const LEN: usize = 8;
	}

	/// ICC_ASGI1R_EL1\[47:44\]
	pub struct RS;
	impl RegisterView for RS {
		type Register = super::ICC_ASGI1R_EL1;
		const NAME: &'static str = "RS";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ICC_ASGI1R_EL1\[40\]
	pub struct IRM;
	impl RegisterView for IRM {
		type Register = super::ICC_ASGI1R_EL1;
		const NAME: &'static str = "IRM";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// ICC_ASGI1R_EL1\[39:32\]
	pub struct Aff2;
	impl RegisterView for Aff2 {
		type Register = super::ICC_ASGI1R_EL1;
		const NAME: &'static str = "Aff2";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// ICC_ASGI1R_EL1\[27:24\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_ASGI1R_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ICC_ASGI1R_EL1\[23:16\]
	pub struct Aff1;
	impl RegisterView for Aff1 {
		type Register = super::ICC_ASGI1R_EL1;
		const NAME: &'static str = "Aff1";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// ICC_ASGI1R_EL1\[15:0\]
	pub struct TargetList;
	impl RegisterView for TargetList {
		type Register = super::ICC_ASGI1R_EL1;
		const NAME: &'static str = "TargetList";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Interrupt Controller Virtual Active Priorities Group 1 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICV_AP1Rn_EL1;
impl RegisterArray for ICV_AP1Rn_EL1 {
	const NAME: &'static str = "ICV_AP1Rn_EL1";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// ICV_AP1Rn_EL1 register fields
pub mod icv_ap1rn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// ICV_AP1Rn_EL1\[63\]
	pub struct NMI;
	impl RegisterArrayView for NMI {
		type RegisterArray = super::ICV_AP1Rn_EL1;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}
}

/// Trace ID Register 2
pub struct TRCIDR2;
impl Register for TRCIDR2 {
	const NAME: &'static str = "TRCIDR2";
	const LEN: usize = 64;
}

/// TRCIDR2 register fields
pub mod trcidr2 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR2\[31\]
	pub struct WFXMODE;
	impl RegisterView for WFXMODE {
		type Register = super::TRCIDR2;
		const NAME: &'static str = "WFXMODE";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// TRCIDR2\[30:29\]
	pub struct VMIDOPT;
	impl RegisterView for VMIDOPT {
		type Register = super::TRCIDR2;
		const NAME: &'static str = "VMIDOPT";
		const OFFSET: usize = 29;
		const LEN: usize = 2;
	}

	/// TRCIDR2\[28:25\]
	pub struct CCSIZE;
	impl RegisterView for CCSIZE {
		type Register = super::TRCIDR2;
		const NAME: &'static str = "CCSIZE";
		const OFFSET: usize = 25;
		const LEN: usize = 4;
	}

	/// TRCIDR2\[24:20\]
	pub struct DVSIZE;
	impl RegisterView for DVSIZE {
		type Register = super::TRCIDR2;
		const NAME: &'static str = "DVSIZE";
		const OFFSET: usize = 20;
		const LEN: usize = 5;
	}

	/// TRCIDR2\[19:15\]
	pub struct DASIZE;
	impl RegisterView for DASIZE {
		type Register = super::TRCIDR2;
		const NAME: &'static str = "DASIZE";
		const OFFSET: usize = 15;
		const LEN: usize = 5;
	}

	/// TRCIDR2\[14:10\]
	pub struct VMIDSIZE;
	impl RegisterView for VMIDSIZE {
		type Register = super::TRCIDR2;
		const NAME: &'static str = "VMIDSIZE";
		const OFFSET: usize = 10;
		const LEN: usize = 5;
	}

	/// TRCIDR2\[9:5\]
	pub struct CIDSIZE;
	impl RegisterView for CIDSIZE {
		type Register = super::TRCIDR2;
		const NAME: &'static str = "CIDSIZE";
		const OFFSET: usize = 5;
		const LEN: usize = 5;
	}

	/// TRCIDR2\[4:0\]
	pub struct IASIZE;
	impl RegisterView for IASIZE {
		type Register = super::TRCIDR2;
		const NAME: &'static str = "IASIZE";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Trace Sequencer State Transition Control Register \<n\>, n = 2 - 0
pub struct TRCSEQEVRn;
impl RegisterArray for TRCSEQEVRn {
	const NAME: &'static str = "TRCSEQEVRn";
	const ELEMS: usize = 2;
	const ELEM_LEN: usize = 64;
}

/// TRCSEQEVRn register fields
pub mod trcseqevrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCSEQEVRn\[15\]
	#[allow(non_camel_case_types)]
	pub struct B_TYPE;
	impl RegisterArrayView for B_TYPE {
		type RegisterArray = super::TRCSEQEVRn;
		const NAME: &'static str = "B_TYPE";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// TRCSEQEVRn\[12:8\]
	#[allow(non_camel_case_types)]
	pub struct B_SEL;
	impl RegisterArrayView for B_SEL {
		type RegisterArray = super::TRCSEQEVRn;
		const NAME: &'static str = "B_SEL";
		const OFFSET: usize = 8;
		const LEN: usize = 5;
	}

	/// TRCSEQEVRn\[7\]
	#[allow(non_camel_case_types)]
	pub struct F_TYPE;
	impl RegisterArrayView for F_TYPE {
		type RegisterArray = super::TRCSEQEVRn;
		const NAME: &'static str = "F_TYPE";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TRCSEQEVRn\[4:0\]
	#[allow(non_camel_case_types)]
	pub struct F_SEL;
	impl RegisterArrayView for F_SEL {
		type RegisterArray = super::TRCSEQEVRn;
		const NAME: &'static str = "F_SEL";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Saved Program Status Register (FIQ mode)
#[allow(non_camel_case_types)]
pub struct SPSR_fiq;
impl Register for SPSR_fiq {
	const NAME: &'static str = "SPSR_fiq";
	const LEN: usize = 64;
}

/// SPSR_fiq register fields
pub mod spsr_fiq {
	use crate::core::model::proc::RegisterView;

	/// SPSR_fiq\[31\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "N";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[30\]
	pub struct Z;
	impl RegisterView for Z {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "Z";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[29\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "C";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[28\]
	pub struct V;
	impl RegisterView for V {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[27\]
	pub struct Q;
	impl RegisterView for Q {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "Q";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[15:10\]
	pub struct IT;
	impl RegisterView for IT {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "IT";
		const OFFSET: usize = 10;
		const LEN: usize = 6;
	}

	/// SPSR_fiq\[24\]
	pub struct J;
	impl RegisterView for J {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "J";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[23\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[22\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[21\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[20\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[19:16\]
	pub struct GE;
	impl RegisterView for GE {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "GE";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SPSR_fiq\[9\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "E";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[8\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "A";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[7\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "I";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[6\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "F";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[5\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "T";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SPSR_fiq\[4:0\]
	pub struct M;
	impl RegisterView for M {
		type Register = super::SPSR_fiq;
		const NAME: &'static str = "M";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Debug Data Transfer Register, Receive
#[allow(non_camel_case_types)]
pub struct DBGDTRRX_EL0;
impl Register for DBGDTRRX_EL0 {
	const NAME: &'static str = "DBGDTRRX_EL0";
	const LEN: usize = 64;
}

/// Auxiliary Memory Attribute Indirection Register (EL2)
#[allow(non_camel_case_types)]
pub struct AMAIR_EL2;
impl Register for AMAIR_EL2 {
	const NAME: &'static str = "AMAIR_EL2";
	const LEN: usize = 64;
}

/// AArch64 Debug Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_AA64DFR1_EL1;
impl Register for ID_AA64DFR1_EL1 {
	const NAME: &'static str = "ID_AA64DFR1_EL1";
	const LEN: usize = 64;
}

/// ID_AA64DFR1_EL1 register fields
pub mod id_aa64dfr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64DFR1_EL1\[63:56\]
	#[allow(non_camel_case_types)]
	pub struct ABL_CMPs;
	impl RegisterView for ABL_CMPs {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "ABL_CMPs";
		const OFFSET: usize = 56;
		const LEN: usize = 8;
	}

	/// ID_AA64DFR1_EL1\[55:52\]
	pub struct DPFZS;
	impl RegisterView for DPFZS {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "DPFZS";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR1_EL1\[51:48\]
	pub struct EBEP;
	impl RegisterView for EBEP {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "EBEP";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR1_EL1\[47:44\]
	pub struct ITE;
	impl RegisterView for ITE {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "ITE";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR1_EL1\[43:40\]
	pub struct ABLE;
	impl RegisterView for ABLE {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "ABLE";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR1_EL1\[39:36\]
	pub struct PMICNTR;
	impl RegisterView for PMICNTR {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "PMICNTR";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR1_EL1\[35:32\]
	pub struct SPMU;
	impl RegisterView for SPMU {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "SPMU";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR1_EL1\[31:24\]
	#[allow(non_camel_case_types)]
	pub struct CTX_CMPs;
	impl RegisterView for CTX_CMPs {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "CTX_CMPs";
		const OFFSET: usize = 24;
		const LEN: usize = 8;
	}

	/// ID_AA64DFR1_EL1\[23:16\]
	pub struct WRPs;
	impl RegisterView for WRPs {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "WRPs";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// ID_AA64DFR1_EL1\[15:8\]
	pub struct BRPs;
	impl RegisterView for BRPs {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "BRPs";
		const OFFSET: usize = 8;
		const LEN: usize = 8;
	}

	/// ID_AA64DFR1_EL1\[7:0\]
	pub struct SYSPMUID;
	impl RegisterView for SYSPMUID {
		type Register = super::ID_AA64DFR1_EL1;
		const NAME: &'static str = "SYSPMUID";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Counter-timer Virtual Timer Control Register
#[allow(non_camel_case_types)]
pub struct CNTV_CTL_EL0;
impl Register for CNTV_CTL_EL0 {
	const NAME: &'static str = "CNTV_CTL_EL0";
	const LEN: usize = 64;
}

/// CNTV_CTL_EL0 register fields
pub mod cntv_ctl_el0 {
	use crate::core::model::proc::RegisterView;

	/// CNTV_CTL_EL0\[2\]
	pub struct ISTATUS;
	impl RegisterView for ISTATUS {
		type Register = super::CNTV_CTL_EL0;
		const NAME: &'static str = "ISTATUS";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTV_CTL_EL0\[1\]
	pub struct IMASK;
	impl RegisterView for IMASK {
		type Register = super::CNTV_CTL_EL0;
		const NAME: &'static str = "IMASK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTV_CTL_EL0\[0\]
	pub struct ENABLE;
	impl RegisterView for ENABLE {
		type Register = super::CNTV_CTL_EL0;
		const NAME: &'static str = "ENABLE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Exception Syndrome Register (EL2)
#[allow(non_camel_case_types)]
pub struct ESR_EL2;
impl Register for ESR_EL2 {
	const NAME: &'static str = "ESR_EL2";
	const LEN: usize = 64;
}

/// ESR_EL2 register fields
pub mod esr_el2 {
	use crate::core::model::proc::RegisterView;

	/// ESR_EL2\[55:32\]
	pub struct ISS2;
	impl RegisterView for ISS2 {
		type Register = super::ESR_EL2;
		const NAME: &'static str = "ISS2";
		const OFFSET: usize = 32;
		const LEN: usize = 24;
	}

	/// ESR_EL2\[31:26\]
	pub struct EC;
	impl RegisterView for EC {
		type Register = super::ESR_EL2;
		const NAME: &'static str = "EC";
		const OFFSET: usize = 26;
		const LEN: usize = 6;
	}

	/// ESR_EL2\[25\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::ESR_EL2;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// ESR_EL2\[24:0\]
	pub struct ISS;
	impl RegisterView for ISS {
		type Register = super::ESR_EL2;
		const NAME: &'static str = "ISS";
		const OFFSET: usize = 0;
		const LEN: usize = 25;
	}
}

/// System Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct SCTLR2_EL3;
impl Register for SCTLR2_EL3 {
	const NAME: &'static str = "SCTLR2_EL3";
	const LEN: usize = 64;
}

/// SCTLR2_EL3 register fields
pub mod sctlr2_el3 {
	use crate::core::model::proc::RegisterView;

	/// SCTLR2_EL3\[11\]
	pub struct CPTM;
	impl RegisterView for CPTM {
		type Register = super::SCTLR2_EL3;
		const NAME: &'static str = "CPTM";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL3\[9\]
	pub struct CPTA;
	impl RegisterView for CPTA {
		type Register = super::SCTLR2_EL3;
		const NAME: &'static str = "CPTA";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL3\[7\]
	pub struct EnPACM;
	impl RegisterView for EnPACM {
		type Register = super::SCTLR2_EL3;
		const NAME: &'static str = "EnPACM";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL3\[4\]
	pub struct EnANERR;
	impl RegisterView for EnANERR {
		type Register = super::SCTLR2_EL3;
		const NAME: &'static str = "EnANERR";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL3\[3\]
	pub struct EnADERR;
	impl RegisterView for EnADERR {
		type Register = super::SCTLR2_EL3;
		const NAME: &'static str = "EnADERR";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL3\[1\]
	pub struct EMEC;
	impl RegisterView for EMEC {
		type Register = super::SCTLR2_EL3;
		const NAME: &'static str = "EMEC";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}
}

/// AArch64 Floating-point Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_AA64FPFR0_EL1;
impl Register for ID_AA64FPFR0_EL1 {
	const NAME: &'static str = "ID_AA64FPFR0_EL1";
	const LEN: usize = 64;
}

/// ID_AA64FPFR0_EL1 register fields
pub mod id_aa64fpfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64FPFR0_EL1\[31\]
	pub struct F8CVT;
	impl RegisterView for F8CVT {
		type Register = super::ID_AA64FPFR0_EL1;
		const NAME: &'static str = "F8CVT";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// ID_AA64FPFR0_EL1\[30\]
	pub struct F8FMA;
	impl RegisterView for F8FMA {
		type Register = super::ID_AA64FPFR0_EL1;
		const NAME: &'static str = "F8FMA";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// ID_AA64FPFR0_EL1\[29\]
	pub struct F8DP4;
	impl RegisterView for F8DP4 {
		type Register = super::ID_AA64FPFR0_EL1;
		const NAME: &'static str = "F8DP4";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// ID_AA64FPFR0_EL1\[28\]
	pub struct F8DP2;
	impl RegisterView for F8DP2 {
		type Register = super::ID_AA64FPFR0_EL1;
		const NAME: &'static str = "F8DP2";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// ID_AA64FPFR0_EL1\[1\]
	pub struct F8E4M3;
	impl RegisterView for F8E4M3 {
		type Register = super::ID_AA64FPFR0_EL1;
		const NAME: &'static str = "F8E4M3";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ID_AA64FPFR0_EL1\[0\]
	pub struct F8E5M2;
	impl RegisterView for F8E5M2 {
		type Register = super::ID_AA64FPFR0_EL1;
		const NAME: &'static str = "F8E5M2";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Event Type Registers, n = 30 - 0
#[allow(non_camel_case_types)]
pub struct PMEVTYPERn_EL0;
impl RegisterArray for PMEVTYPERn_EL0 {
	const NAME: &'static str = "PMEVTYPERn_EL0";
	const ELEMS: usize = 30;
	const ELEM_LEN: usize = 64;
}

/// PMEVTYPERn_EL0 register fields
pub mod pmevtypern_el0 {
	use crate::core::model::proc::RegisterArrayView;

	/// PMEVTYPERn_EL0\[63:61\]
	pub struct TC;
	impl RegisterArrayView for TC {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "TC";
		const OFFSET: usize = 61;
		const LEN: usize = 3;
	}

	/// PMEVTYPERn_EL0\[60\]
	pub struct TE;
	impl RegisterArrayView for TE {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "TE";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[58\]
	pub struct SYNC;
	impl RegisterArrayView for SYNC {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "SYNC";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[57:56\]
	pub struct VS;
	impl RegisterArrayView for VS {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "VS";
		const OFFSET: usize = 56;
		const LEN: usize = 2;
	}

	/// PMEVTYPERn_EL0\[55:54\]
	pub struct TLC;
	impl RegisterArrayView for TLC {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "TLC";
		const OFFSET: usize = 54;
		const LEN: usize = 2;
	}

	/// PMEVTYPERn_EL0\[43:32\]
	pub struct TH;
	impl RegisterArrayView for TH {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "TH";
		const OFFSET: usize = 32;
		const LEN: usize = 12;
	}

	/// PMEVTYPERn_EL0\[31\]
	pub struct P;
	impl RegisterArrayView for P {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "P";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[30\]
	pub struct U;
	impl RegisterArrayView for U {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "U";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[29\]
	pub struct NSK;
	impl RegisterArrayView for NSK {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "NSK";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[28\]
	pub struct NSU;
	impl RegisterArrayView for NSU {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "NSU";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[27\]
	pub struct NSH;
	impl RegisterArrayView for NSH {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "NSH";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[26\]
	pub struct M;
	impl RegisterArrayView for M {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "M";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[25\]
	pub struct MT;
	impl RegisterArrayView for MT {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "MT";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[24\]
	pub struct SH;
	impl RegisterArrayView for SH {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "SH";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[23\]
	pub struct T;
	impl RegisterArrayView for T {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "T";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[22\]
	pub struct RLK;
	impl RegisterArrayView for RLK {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "RLK";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[21\]
	pub struct RLU;
	impl RegisterArrayView for RLU {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "RLU";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[20\]
	pub struct RLH;
	impl RegisterArrayView for RLH {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "RLH";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// PMEVTYPERn_EL0\[15:10\]
	#[allow(non_camel_case_types)]
	pub struct evtCount_15_10;
	impl RegisterArrayView for evtCount_15_10 {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "evtCount_15_10";
		const OFFSET: usize = 10;
		const LEN: usize = 6;
	}
	/// PMEVTYPERn_EL0\[9:0\]
	#[allow(non_camel_case_types)]
	pub struct evtCount_15_10_9_0;
	impl RegisterArrayView for evtCount_15_10_9_0 {
		type RegisterArray = super::PMEVTYPERn_EL0;
		const NAME: &'static str = "evtCount_15_10_9_0";
		const OFFSET: usize = 0;
		const LEN: usize = 10;
	}
}

/// System Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct SCTLR2_EL2;
impl Register for SCTLR2_EL2 {
	const NAME: &'static str = "SCTLR2_EL2";
	const LEN: usize = 64;
}

/// SCTLR2_EL2 register fields
pub mod sctlr2_el2 {
	use crate::core::model::proc::RegisterView;

	/// SCTLR2_EL2\[12\]
	pub struct CPTM0;
	impl RegisterView for CPTM0 {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "CPTM0";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[11\]
	pub struct CPTM;
	impl RegisterView for CPTM {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "CPTM";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[10\]
	pub struct CPTA0;
	impl RegisterView for CPTA0 {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "CPTA0";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[9\]
	pub struct CPTA;
	impl RegisterView for CPTA {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "CPTA";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[8\]
	pub struct EnPACM0;
	impl RegisterView for EnPACM0 {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "EnPACM0";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[7\]
	pub struct EnPACM;
	impl RegisterView for EnPACM {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "EnPACM";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[6\]
	pub struct EnIDCP128;
	impl RegisterView for EnIDCP128 {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "EnIDCP128";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[5\]
	pub struct EASE;
	impl RegisterView for EASE {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "EASE";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[4\]
	pub struct EnANERR;
	impl RegisterView for EnANERR {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "EnANERR";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[3\]
	pub struct EnADERR;
	impl RegisterView for EnADERR {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "EnADERR";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[2\]
	pub struct NMEA;
	impl RegisterView for NMEA {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "NMEA";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL2\[1\]
	pub struct EMEC;
	impl RegisterView for EMEC {
		type Register = super::SCTLR2_EL2;
		const NAME: &'static str = "EMEC";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}
}

/// Exception Syndrome Register (EL3)
#[allow(non_camel_case_types)]
pub struct ESR_EL3;
impl Register for ESR_EL3 {
	const NAME: &'static str = "ESR_EL3";
	const LEN: usize = 64;
}

/// ESR_EL3 register fields
pub mod esr_el3 {
	use crate::core::model::proc::RegisterView;

	/// ESR_EL3\[55:32\]
	pub struct ISS2;
	impl RegisterView for ISS2 {
		type Register = super::ESR_EL3;
		const NAME: &'static str = "ISS2";
		const OFFSET: usize = 32;
		const LEN: usize = 24;
	}

	/// ESR_EL3\[31:26\]
	pub struct EC;
	impl RegisterView for EC {
		type Register = super::ESR_EL3;
		const NAME: &'static str = "EC";
		const OFFSET: usize = 26;
		const LEN: usize = 6;
	}

	/// ESR_EL3\[25\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::ESR_EL3;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// ESR_EL3\[24:0\]
	pub struct ISS;
	impl RegisterView for ISS {
		type Register = super::ESR_EL3;
		const NAME: &'static str = "ISS";
		const OFFSET: usize = 0;
		const LEN: usize = 25;
	}
}

/// Branch Record Buffer Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct BRBCR_EL2;
impl Register for BRBCR_EL2 {
	const NAME: &'static str = "BRBCR_EL2";
	const LEN: usize = 64;
}

/// BRBCR_EL2 register fields
pub mod brbcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// BRBCR_EL2\[23\]
	pub struct EXCEPTION;
	impl RegisterView for EXCEPTION {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "EXCEPTION";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// BRBCR_EL2\[22\]
	pub struct ERTN;
	impl RegisterView for ERTN {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "ERTN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// BRBCR_EL2\[9\]
	pub struct FZPSS;
	impl RegisterView for FZPSS {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "FZPSS";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// BRBCR_EL2\[8\]
	pub struct FZP;
	impl RegisterView for FZP {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "FZP";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// BRBCR_EL2\[6:5\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 5;
		const LEN: usize = 2;
	}

	/// BRBCR_EL2\[4\]
	pub struct MPRED;
	impl RegisterView for MPRED {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "MPRED";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// BRBCR_EL2\[3\]
	pub struct CC;
	impl RegisterView for CC {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "CC";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// BRBCR_EL2\[1\]
	pub struct E2BRE;
	impl RegisterView for E2BRE {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "E2BRE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// BRBCR_EL2\[0\]
	pub struct E0HBRE;
	impl RegisterView for E0HBRE {
		type Register = super::BRBCR_EL2;
		const NAME: &'static str = "E0HBRE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Auxiliary Memory Attribute Indirection Register (EL3)
#[allow(non_camel_case_types)]
pub struct AMAIR_EL3;
impl Register for AMAIR_EL3 {
	const NAME: &'static str = "AMAIR_EL3";
	const LEN: usize = 64;
}

/// System Performance Monitors Overflow Flag Status Clear Register
#[allow(non_camel_case_types)]
pub struct SPMOVSCLR_EL0;
impl Register for SPMOVSCLR_EL0 {
	const NAME: &'static str = "SPMOVSCLR_EL0";
	const LEN: usize = 64;
}

/// Random Allocation Tag Seed Register.
#[allow(non_camel_case_types)]
pub struct RGSR_EL1;
impl Register for RGSR_EL1 {
	const NAME: &'static str = "RGSR_EL1";
	const LEN: usize = 64;
}

/// RGSR_EL1 register fields
pub mod rgsr_el1 {
	use crate::core::model::proc::RegisterView;

	/// RGSR_EL1\[55:8\]
	pub struct SEED;
	impl RegisterView for SEED {
		type Register = super::RGSR_EL1;
		const NAME: &'static str = "SEED";
		const OFFSET: usize = 8;
		const LEN: usize = 48;
	}

	/// RGSR_EL1\[3:0\]
	pub struct TAG;
	impl RegisterView for TAG {
		type Register = super::RGSR_EL1;
		const NAME: &'static str = "TAG";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Hypervisor Fine-Grained Read Trap Register 2
#[allow(non_camel_case_types)]
pub struct HFGRTR2_EL2;
impl Register for HFGRTR2_EL2 {
	const NAME: &'static str = "HFGRTR2_EL2";
	const LEN: usize = 64;
}

/// HFGRTR2_EL2 register fields
pub mod hfgrtr2_el2 {
	use crate::core::model::proc::RegisterView;

	/// HFGRTR2_EL2\[2\]
	#[allow(non_camel_case_types)]
	pub struct nRCWSMASK_EL1;
	impl RegisterView for nRCWSMASK_EL1 {
		type Register = super::HFGRTR2_EL2;
		const NAME: &'static str = "nRCWSMASK_EL1";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HFGRTR2_EL2\[1\]
	#[allow(non_camel_case_types)]
	pub struct nERXGSR_EL1;
	impl RegisterView for nERXGSR_EL1 {
		type Register = super::HFGRTR2_EL2;
		const NAME: &'static str = "nERXGSR_EL1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HFGRTR2_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct nPFAR_EL1;
	impl RegisterView for nPFAR_EL1 {
		type Register = super::HFGRTR2_EL2;
		const NAME: &'static str = "nPFAR_EL1";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller Virtual End Of Interrupt Register 0
#[allow(non_camel_case_types)]
pub struct ICV_EOIR0_EL1;
impl Register for ICV_EOIR0_EL1 {
	const NAME: &'static str = "ICV_EOIR0_EL1";
	const LEN: usize = 64;
}

/// ICV_EOIR0_EL1 register fields
pub mod icv_eoir0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_EOIR0_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICV_EOIR0_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Interrupt Controller VGIC Type Register
#[allow(non_camel_case_types)]
pub struct ICH_VTR_EL2;
impl Register for ICH_VTR_EL2 {
	const NAME: &'static str = "ICH_VTR_EL2";
	const LEN: usize = 64;
}

/// ICH_VTR_EL2 register fields
pub mod ich_vtr_el2 {
	use crate::core::model::proc::RegisterView;

	/// ICH_VTR_EL2\[31:29\]
	pub struct PRIbits;
	impl RegisterView for PRIbits {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "PRIbits";
		const OFFSET: usize = 29;
		const LEN: usize = 3;
	}

	/// ICH_VTR_EL2\[28:26\]
	pub struct PREbits;
	impl RegisterView for PREbits {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "PREbits";
		const OFFSET: usize = 26;
		const LEN: usize = 3;
	}

	/// ICH_VTR_EL2\[25:23\]
	pub struct IDbits;
	impl RegisterView for IDbits {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "IDbits";
		const OFFSET: usize = 23;
		const LEN: usize = 3;
	}

	/// ICH_VTR_EL2\[22\]
	pub struct SEIS;
	impl RegisterView for SEIS {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "SEIS";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// ICH_VTR_EL2\[21\]
	pub struct A3V;
	impl RegisterView for A3V {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "A3V";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// ICH_VTR_EL2\[20\]
	#[allow(non_camel_case_types)]
	pub struct nV4;
	impl RegisterView for nV4 {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "nV4";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// ICH_VTR_EL2\[19\]
	pub struct TDS;
	impl RegisterView for TDS {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "TDS";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// ICH_VTR_EL2\[18\]
	pub struct DVIM;
	impl RegisterView for DVIM {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "DVIM";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// ICH_VTR_EL2\[4:0\]
	pub struct ListRegs;
	impl RegisterView for ListRegs {
		type Register = super::ICH_VTR_EL2;
		const NAME: &'static str = "ListRegs";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Trace ID Register 3
pub struct TRCIDR3;
impl Register for TRCIDR3 {
	const NAME: &'static str = "TRCIDR3";
	const LEN: usize = 64;
}

/// TRCIDR3 register fields
pub mod trcidr3 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR3\[31\]
	pub struct NOOVERFLOW;
	impl RegisterView for NOOVERFLOW {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "NOOVERFLOW";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[13:12\]
	pub struct NUMPROC;
	impl RegisterView for NUMPROC {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "NUMPROC";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// TRCIDR3\[27\]
	pub struct SYSSTALL;
	impl RegisterView for SYSSTALL {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "SYSSTALL";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[26\]
	pub struct STALLCTL;
	impl RegisterView for STALLCTL {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "STALLCTL";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[25\]
	pub struct SYNCPR;
	impl RegisterView for SYNCPR {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "SYNCPR";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[24\]
	pub struct TRCERR;
	impl RegisterView for TRCERR {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "TRCERR";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[22\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL2;
	impl RegisterView for EXLEVEL_NS_EL2 {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "EXLEVEL_NS_EL2";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[21\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL1;
	impl RegisterView for EXLEVEL_NS_EL1 {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "EXLEVEL_NS_EL1";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[20\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL0;
	impl RegisterView for EXLEVEL_NS_EL0 {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "EXLEVEL_NS_EL0";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[19\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL3;
	impl RegisterView for EXLEVEL_S_EL3 {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "EXLEVEL_S_EL3";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[18\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL2;
	impl RegisterView for EXLEVEL_S_EL2 {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "EXLEVEL_S_EL2";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[17\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL1;
	impl RegisterView for EXLEVEL_S_EL1 {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "EXLEVEL_S_EL1";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[16\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL0;
	impl RegisterView for EXLEVEL_S_EL0 {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "EXLEVEL_S_EL0";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// TRCIDR3\[11:0\]
	pub struct CCITMIN;
	impl RegisterView for CCITMIN {
		type Register = super::TRCIDR3;
		const NAME: &'static str = "CCITMIN";
		const OFFSET: usize = 0;
		const LEN: usize = 12;
	}
}

/// EL0 Read/Write Software Context Number
#[allow(non_camel_case_types)]
pub struct SCXTNUM_EL0;
impl Register for SCXTNUM_EL0 {
	const NAME: &'static str = "SCXTNUM_EL0";
	const LEN: usize = 64;
}

/// EL3 Software Thread ID Register
#[allow(non_camel_case_types)]
pub struct TPIDR_EL3;
impl Register for TPIDR_EL3 {
	const NAME: &'static str = "TPIDR_EL3";
	const LEN: usize = 64;
}

/// Multiple tag transfer ID Register
#[allow(non_camel_case_types)]
pub struct GMID_EL1;
impl Register for GMID_EL1 {
	const NAME: &'static str = "GMID_EL1";
	const LEN: usize = 64;
}

/// GMID_EL1 register fields
pub mod gmid_el1 {
	use crate::core::model::proc::RegisterView;

	/// GMID_EL1\[3:0\]
	pub struct BS;
	impl RegisterView for BS {
		type Register = super::GMID_EL1;
		const NAME: &'static str = "BS";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Branch Record Buffer Function Control Register
#[allow(non_camel_case_types)]
pub struct BRBFCR_EL1;
impl Register for BRBFCR_EL1 {
	const NAME: &'static str = "BRBFCR_EL1";
	const LEN: usize = 64;
}

/// BRBFCR_EL1 register fields
pub mod brbfcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// BRBFCR_EL1\[29:28\]
	pub struct BANK;
	impl RegisterView for BANK {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "BANK";
		const OFFSET: usize = 28;
		const LEN: usize = 2;
	}

	/// BRBFCR_EL1\[22\]
	pub struct CONDDIR;
	impl RegisterView for CONDDIR {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "CONDDIR";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// BRBFCR_EL1\[21\]
	pub struct DIRCALL;
	impl RegisterView for DIRCALL {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "DIRCALL";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// BRBFCR_EL1\[20\]
	pub struct INDCALL;
	impl RegisterView for INDCALL {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "INDCALL";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// BRBFCR_EL1\[19\]
	pub struct RTN;
	impl RegisterView for RTN {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "RTN";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// BRBFCR_EL1\[18\]
	pub struct INDIRECT;
	impl RegisterView for INDIRECT {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "INDIRECT";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// BRBFCR_EL1\[17\]
	pub struct DIRECT;
	impl RegisterView for DIRECT {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "DIRECT";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// BRBFCR_EL1\[16\]
	pub struct EnI;
	impl RegisterView for EnI {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "EnI";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// BRBFCR_EL1\[7\]
	pub struct PAUSED;
	impl RegisterView for PAUSED {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "PAUSED";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// BRBFCR_EL1\[6\]
	pub struct LASTFAILED;
	impl RegisterView for LASTFAILED {
		type Register = super::BRBFCR_EL1;
		const NAME: &'static str = "LASTFAILED";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}
}

/// Sampling Latency Filter Register
#[allow(non_camel_case_types)]
pub struct PMSLATFR_EL1;
impl Register for PMSLATFR_EL1 {
	const NAME: &'static str = "PMSLATFR_EL1";
	const LEN: usize = 64;
}

/// PMSLATFR_EL1 register fields
pub mod pmslatfr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSLATFR_EL1\[15:0\]
	pub struct MINLAT;
	impl RegisterView for MINLAT {
		type Register = super::PMSLATFR_EL1;
		const NAME: &'static str = "MINLAT";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Performance Monitors Count Enable Clear Register
#[allow(non_camel_case_types)]
pub struct PMCNTENCLR_EL0;
impl Register for PMCNTENCLR_EL0 {
	const NAME: &'static str = "PMCNTENCLR_EL0";
	const LEN: usize = 64;
}

/// PMCNTENCLR_EL0 register fields
pub mod pmcntenclr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMCNTENCLR_EL0\[31\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMCNTENCLR_EL0;
		const NAME: &'static str = "C";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller Non-maskable Interrupt Acknowledge Register 1
#[allow(non_camel_case_types)]
pub struct ICC_NMIAR1_EL1;
impl Register for ICC_NMIAR1_EL1 {
	const NAME: &'static str = "ICC_NMIAR1_EL1";
	const LEN: usize = 64;
}

/// ICC_NMIAR1_EL1 register fields
pub mod icc_nmiar1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_NMIAR1_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_NMIAR1_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// EL1 Software Thread ID Register
#[allow(non_camel_case_types)]
pub struct TPIDR_EL1;
impl Register for TPIDR_EL1 {
	const NAME: &'static str = "TPIDR_EL1";
	const LEN: usize = 64;
}

/// Selected Error Record Miscellaneous Register 3
#[allow(non_camel_case_types)]
pub struct ERXMISC3_EL1;
impl Register for ERXMISC3_EL1 {
	const NAME: &'static str = "ERXMISC3_EL1";
	const LEN: usize = 64;
}

/// EL2 Read/Write Software Context Number
#[allow(non_camel_case_types)]
pub struct SCXTNUM_EL2;
impl Register for SCXTNUM_EL2 {
	const NAME: &'static str = "SCXTNUM_EL2";
	const LEN: usize = 64;
}

/// OS Lock Data Transfer Register, Transmit
#[allow(non_camel_case_types)]
pub struct OSDTRTX_EL1;
impl Register for OSDTRTX_EL1 {
	const NAME: &'static str = "OSDTRTX_EL1";
	const LEN: usize = 64;
}

/// Trace Single-shot Processing Element Comparator Input Control Register \<n\>, n = 7 - 0
pub struct TRCSSPCICRn;
impl RegisterArray for TRCSSPCICRn {
	const NAME: &'static str = "TRCSSPCICRn";
	const ELEMS: usize = 7;
	const ELEM_LEN: usize = 64;
}

/// Sampling Event Filter Register
#[allow(non_camel_case_types)]
pub struct PMSEVFR_EL1;
impl Register for PMSEVFR_EL1 {
	const NAME: &'static str = "PMSEVFR_EL1";
	const LEN: usize = 64;
}

/// PMSEVFR_EL1 register fields
pub mod pmsevfr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSEVFR_EL1\[63\]
	#[allow(non_camel_case_types)]
	pub struct E_63;
	impl RegisterView for E_63 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[62\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62;
	impl RegisterView for E_63_62 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[61\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61;
	impl RegisterView for E_63_62_61 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[60\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60;
	impl RegisterView for E_63_62_61_60 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[59\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59;
	impl RegisterView for E_63_62_61_60_59 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[58\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58;
	impl RegisterView for E_63_62_61_60_59_58 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[57\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57;
	impl RegisterView for E_63_62_61_60_59_58_57 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[56\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56;
	impl RegisterView for E_63_62_61_60_59_58_57_56 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[55\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[54\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[53\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[52\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[51\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[50\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[49\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[48\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[31\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[30\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[29\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[28\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[27\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[26\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[25\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[25\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[24\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[24\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[23\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[22\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[21\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[20\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[19\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[18\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[17\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[16\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[15\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[14\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[13\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[12\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[11\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[10\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[9\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[8\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[7\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[6\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[5\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[4\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[3\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[2\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}
	/// PMSEVFR_EL1\[1\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2_1;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2_1 {
		type Register = super::PMSEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2_1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller End Of Interrupt Register 0
#[allow(non_camel_case_types)]
pub struct ICC_EOIR0_EL1;
impl Register for ICC_EOIR0_EL1 {
	const NAME: &'static str = "ICC_EOIR0_EL1";
	const LEN: usize = 64;
}

/// ICC_EOIR0_EL1 register fields
pub mod icc_eoir0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_EOIR0_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_EOIR0_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Counter-timer Virtual Timer Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHV_CTL_EL2;
impl Register for CNTHV_CTL_EL2 {
	const NAME: &'static str = "CNTHV_CTL_EL2";
	const LEN: usize = 64;
}

/// CNTHV_CTL_EL2 register fields
pub mod cnthv_ctl_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHV_CTL_EL2\[2\]
	pub struct ISTATUS;
	impl RegisterView for ISTATUS {
		type Register = super::CNTHV_CTL_EL2;
		const NAME: &'static str = "ISTATUS";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTHV_CTL_EL2\[1\]
	pub struct IMASK;
	impl RegisterView for IMASK {
		type Register = super::CNTHV_CTL_EL2;
		const NAME: &'static str = "IMASK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTHV_CTL_EL2\[0\]
	pub struct ENABLE;
	impl RegisterView for ENABLE {
		type Register = super::CNTHV_CTL_EL2;
		const NAME: &'static str = "ENABLE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// System PMU Implementation Identification Register
#[allow(non_camel_case_types)]
pub struct SPMIIDR_EL1;
impl Register for SPMIIDR_EL1 {
	const NAME: &'static str = "SPMIIDR_EL1";
	const LEN: usize = 64;
}

/// SPMIIDR_EL1 register fields
pub mod spmiidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// SPMIIDR_EL1\[31:20\]
	pub struct ProductID;
	impl RegisterView for ProductID {
		type Register = super::SPMIIDR_EL1;
		const NAME: &'static str = "ProductID";
		const OFFSET: usize = 20;
		const LEN: usize = 12;
	}

	/// SPMIIDR_EL1\[19:16\]
	pub struct Variant;
	impl RegisterView for Variant {
		type Register = super::SPMIIDR_EL1;
		const NAME: &'static str = "Variant";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SPMIIDR_EL1\[15:12\]
	pub struct Revision;
	impl RegisterView for Revision {
		type Register = super::SPMIIDR_EL1;
		const NAME: &'static str = "Revision";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// SPMIIDR_EL1\[6:0\]
	pub struct Implementer;
	impl RegisterView for Implementer {
		type Register = super::SPMIIDR_EL1;
		const NAME: &'static str = "Implementer";
		const OFFSET: usize = 0;
		const LEN: usize = 7;
	}
}

/// Trace ID Register 1
pub struct TRCIDR1;
impl Register for TRCIDR1 {
	const NAME: &'static str = "TRCIDR1";
	const LEN: usize = 64;
}

/// TRCIDR1 register fields
pub mod trcidr1 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR1\[31:24\]
	pub struct DESIGNER;
	impl RegisterView for DESIGNER {
		type Register = super::TRCIDR1;
		const NAME: &'static str = "DESIGNER";
		const OFFSET: usize = 24;
		const LEN: usize = 8;
	}

	/// TRCIDR1\[11:8\]
	pub struct TRCARCHMAJ;
	impl RegisterView for TRCARCHMAJ {
		type Register = super::TRCIDR1;
		const NAME: &'static str = "TRCARCHMAJ";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// TRCIDR1\[7:4\]
	pub struct TRCARCHMIN;
	impl RegisterView for TRCARCHMIN {
		type Register = super::TRCIDR1;
		const NAME: &'static str = "TRCARCHMIN";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// TRCIDR1\[3:0\]
	pub struct REVISION;
	impl RegisterView for REVISION {
		type Register = super::TRCIDR1;
		const NAME: &'static str = "REVISION";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Virtualization Multiprocessor ID Register
#[allow(non_camel_case_types)]
pub struct VMPIDR_EL2;
impl Register for VMPIDR_EL2 {
	const NAME: &'static str = "VMPIDR_EL2";
	const LEN: usize = 64;
}

/// VMPIDR_EL2 register fields
pub mod vmpidr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VMPIDR_EL2\[39:32\]
	pub struct Aff3;
	impl RegisterView for Aff3 {
		type Register = super::VMPIDR_EL2;
		const NAME: &'static str = "Aff3";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// VMPIDR_EL2\[30\]
	pub struct U;
	impl RegisterView for U {
		type Register = super::VMPIDR_EL2;
		const NAME: &'static str = "U";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// VMPIDR_EL2\[24\]
	pub struct MT;
	impl RegisterView for MT {
		type Register = super::VMPIDR_EL2;
		const NAME: &'static str = "MT";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// VMPIDR_EL2\[23:16\]
	pub struct Aff2;
	impl RegisterView for Aff2 {
		type Register = super::VMPIDR_EL2;
		const NAME: &'static str = "Aff2";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// VMPIDR_EL2\[15:8\]
	pub struct Aff1;
	impl RegisterView for Aff1 {
		type Register = super::VMPIDR_EL2;
		const NAME: &'static str = "Aff1";
		const OFFSET: usize = 8;
		const LEN: usize = 8;
	}

	/// VMPIDR_EL2\[7:0\]
	pub struct Aff0;
	impl RegisterView for Aff0 {
		type Register = super::VMPIDR_EL2;
		const NAME: &'static str = "Aff0";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// AArch32 Instruction Set Attribute Register 0
#[allow(non_camel_case_types)]
pub struct ID_ISAR0_EL1;
impl Register for ID_ISAR0_EL1 {
	const NAME: &'static str = "ID_ISAR0_EL1";
	const LEN: usize = 64;
}

/// ID_ISAR0_EL1 register fields
pub mod id_isar0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_ISAR0_EL1\[27:24\]
	pub struct Divide;
	impl RegisterView for Divide {
		type Register = super::ID_ISAR0_EL1;
		const NAME: &'static str = "Divide";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_ISAR0_EL1\[23:20\]
	pub struct Debug;
	impl RegisterView for Debug {
		type Register = super::ID_ISAR0_EL1;
		const NAME: &'static str = "Debug";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_ISAR0_EL1\[19:16\]
	pub struct Coproc;
	impl RegisterView for Coproc {
		type Register = super::ID_ISAR0_EL1;
		const NAME: &'static str = "Coproc";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_ISAR0_EL1\[15:12\]
	pub struct CmpBranch;
	impl RegisterView for CmpBranch {
		type Register = super::ID_ISAR0_EL1;
		const NAME: &'static str = "CmpBranch";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_ISAR0_EL1\[11:8\]
	pub struct BitField;
	impl RegisterView for BitField {
		type Register = super::ID_ISAR0_EL1;
		const NAME: &'static str = "BitField";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_ISAR0_EL1\[7:4\]
	pub struct BitCount;
	impl RegisterView for BitCount {
		type Register = super::ID_ISAR0_EL1;
		const NAME: &'static str = "BitCount";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_ISAR0_EL1\[3:0\]
	pub struct Swap;
	impl RegisterView for Swap {
		type Register = super::ID_ISAR0_EL1;
		const NAME: &'static str = "Swap";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Debug Power Control Register
#[allow(non_camel_case_types)]
pub struct DBGPRCR_EL1;
impl Register for DBGPRCR_EL1 {
	const NAME: &'static str = "DBGPRCR_EL1";
	const LEN: usize = 64;
}

/// DBGPRCR_EL1 register fields
pub mod dbgprcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// DBGPRCR_EL1\[0\]
	pub struct CORENPDRQ;
	impl RegisterView for CORENPDRQ {
		type Register = super::DBGPRCR_EL1;
		const NAME: &'static str = "CORENPDRQ";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Auxiliary Memory Attribute Indirection Register (EL1)
#[allow(non_camel_case_types)]
pub struct AMAIR_EL1;
impl Register for AMAIR_EL1 {
	const NAME: &'static str = "AMAIR_EL1";
	const LEN: usize = 64;
}

/// Interrupt Controller Binary Point Register 0
#[allow(non_camel_case_types)]
pub struct ICC_BPR0_EL1;
impl Register for ICC_BPR0_EL1 {
	const NAME: &'static str = "ICC_BPR0_EL1";
	const LEN: usize = 64;
}

/// ICC_BPR0_EL1 register fields
pub mod icc_bpr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_BPR0_EL1\[2:0\]
	pub struct BinaryPoint;
	impl RegisterView for BinaryPoint {
		type Register = super::ICC_BPR0_EL1;
		const NAME: &'static str = "BinaryPoint";
		const OFFSET: usize = 0;
		const LEN: usize = 3;
	}
}

/// Exception Syndrome Register (EL1)
#[allow(non_camel_case_types)]
pub struct ESR_EL1;
impl Register for ESR_EL1 {
	const NAME: &'static str = "ESR_EL1";
	const LEN: usize = 64;
}

/// ESR_EL1 register fields
pub mod esr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ESR_EL1\[55:32\]
	pub struct ISS2;
	impl RegisterView for ISS2 {
		type Register = super::ESR_EL1;
		const NAME: &'static str = "ISS2";
		const OFFSET: usize = 32;
		const LEN: usize = 24;
	}

	/// ESR_EL1\[31:26\]
	pub struct EC;
	impl RegisterView for EC {
		type Register = super::ESR_EL1;
		const NAME: &'static str = "EC";
		const OFFSET: usize = 26;
		const LEN: usize = 6;
	}

	/// ESR_EL1\[25\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::ESR_EL1;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// ESR_EL1\[24:0\]
	pub struct ISS;
	impl RegisterView for ISS {
		type Register = super::ESR_EL1;
		const NAME: &'static str = "ISS";
		const OFFSET: usize = 0;
		const LEN: usize = 25;
	}
}

/// Activity Monitors Count Enable Clear Register 0
#[allow(non_camel_case_types)]
pub struct AMCNTENCLR0_EL0;
impl Register for AMCNTENCLR0_EL0 {
	const NAME: &'static str = "AMCNTENCLR0_EL0";
	const LEN: usize = 64;
}

/// Pointer Authentication Key B for Instruction (bits\[63:0\]) 
#[allow(non_camel_case_types)]
pub struct APIBKeyLo_EL1;
impl Register for APIBKeyLo_EL1 {
	const NAME: &'static str = "APIBKeyLo_EL1";
	const LEN: usize = 64;
}

/// AArch32 Media and VFP Feature Register 0
#[allow(non_camel_case_types)]
pub struct MVFR0_EL1;
impl Register for MVFR0_EL1 {
	const NAME: &'static str = "MVFR0_EL1";
	const LEN: usize = 64;
}

/// MVFR0_EL1 register fields
pub mod mvfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// MVFR0_EL1\[31:28\]
	pub struct FPRound;
	impl RegisterView for FPRound {
		type Register = super::MVFR0_EL1;
		const NAME: &'static str = "FPRound";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// MVFR0_EL1\[27:24\]
	pub struct FPShVec;
	impl RegisterView for FPShVec {
		type Register = super::MVFR0_EL1;
		const NAME: &'static str = "FPShVec";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// MVFR0_EL1\[23:20\]
	pub struct FPSqrt;
	impl RegisterView for FPSqrt {
		type Register = super::MVFR0_EL1;
		const NAME: &'static str = "FPSqrt";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// MVFR0_EL1\[19:16\]
	pub struct FPDivide;
	impl RegisterView for FPDivide {
		type Register = super::MVFR0_EL1;
		const NAME: &'static str = "FPDivide";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// MVFR0_EL1\[15:12\]
	pub struct FPTrap;
	impl RegisterView for FPTrap {
		type Register = super::MVFR0_EL1;
		const NAME: &'static str = "FPTrap";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// MVFR0_EL1\[11:8\]
	pub struct FPDP;
	impl RegisterView for FPDP {
		type Register = super::MVFR0_EL1;
		const NAME: &'static str = "FPDP";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// MVFR0_EL1\[7:4\]
	pub struct FPSP;
	impl RegisterView for FPSP {
		type Register = super::MVFR0_EL1;
		const NAME: &'static str = "FPSP";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// MVFR0_EL1\[3:0\]
	pub struct SIMDReg;
	impl RegisterView for SIMDReg {
		type Register = super::MVFR0_EL1;
		const NAME: &'static str = "SIMDReg";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// System Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct SCTLR2_EL1;
impl Register for SCTLR2_EL1 {
	const NAME: &'static str = "SCTLR2_EL1";
	const LEN: usize = 64;
}

/// SCTLR2_EL1 register fields
pub mod sctlr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// SCTLR2_EL1\[12\]
	pub struct CPTM0;
	impl RegisterView for CPTM0 {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "CPTM0";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[11\]
	pub struct CPTM;
	impl RegisterView for CPTM {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "CPTM";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[10\]
	pub struct CPTA0;
	impl RegisterView for CPTA0 {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "CPTA0";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[9\]
	pub struct CPTA;
	impl RegisterView for CPTA {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "CPTA";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[8\]
	pub struct EnPACM0;
	impl RegisterView for EnPACM0 {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "EnPACM0";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[7\]
	pub struct EnPACM;
	impl RegisterView for EnPACM {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "EnPACM";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[6\]
	pub struct EnIDCP128;
	impl RegisterView for EnIDCP128 {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "EnIDCP128";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[5\]
	pub struct EASE;
	impl RegisterView for EASE {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "EASE";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[4\]
	pub struct EnANERR;
	impl RegisterView for EnANERR {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "EnANERR";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[3\]
	pub struct EnADERR;
	impl RegisterView for EnADERR {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "EnADERR";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// SCTLR2_EL1\[2\]
	pub struct NMEA;
	impl RegisterView for NMEA {
		type Register = super::SCTLR2_EL1;
		const NAME: &'static str = "NMEA";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}
}

/// Branch Record Buffer Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct BRBCR_EL1;
impl Register for BRBCR_EL1 {
	const NAME: &'static str = "BRBCR_EL1";
	const LEN: usize = 64;
}

/// BRBCR_EL1 register fields
pub mod brbcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// BRBCR_EL1\[23\]
	pub struct EXCEPTION;
	impl RegisterView for EXCEPTION {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "EXCEPTION";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// BRBCR_EL1\[22\]
	pub struct ERTN;
	impl RegisterView for ERTN {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "ERTN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// BRBCR_EL1\[9\]
	pub struct FZPSS;
	impl RegisterView for FZPSS {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "FZPSS";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// BRBCR_EL1\[8\]
	pub struct FZP;
	impl RegisterView for FZP {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "FZP";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// BRBCR_EL1\[6:5\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 5;
		const LEN: usize = 2;
	}

	/// BRBCR_EL1\[4\]
	pub struct MPRED;
	impl RegisterView for MPRED {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "MPRED";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// BRBCR_EL1\[3\]
	pub struct CC;
	impl RegisterView for CC {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "CC";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// BRBCR_EL1\[1\]
	pub struct E1BRE;
	impl RegisterView for E1BRE {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "E1BRE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// BRBCR_EL1\[0\]
	pub struct E0BRE;
	impl RegisterView for E0BRE {
		type Register = super::BRBCR_EL1;
		const NAME: &'static str = "E0BRE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// MPAM Virtual PARTID Mapping Register 0
#[allow(non_camel_case_types)]
pub struct MPAMVPM0_EL2;
impl Register for MPAMVPM0_EL2 {
	const NAME: &'static str = "MPAMVPM0_EL2";
	const LEN: usize = 64;
}

/// MPAMVPM0_EL2 register fields
pub mod mpamvpm0_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMVPM0_EL2\[63:48\]
	pub struct PhyPARTID3;
	impl RegisterView for PhyPARTID3 {
		type Register = super::MPAMVPM0_EL2;
		const NAME: &'static str = "PhyPARTID3";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// MPAMVPM0_EL2\[47:32\]
	pub struct PhyPARTID2;
	impl RegisterView for PhyPARTID2 {
		type Register = super::MPAMVPM0_EL2;
		const NAME: &'static str = "PhyPARTID2";
		const OFFSET: usize = 32;
		const LEN: usize = 16;
	}

	/// MPAMVPM0_EL2\[31:16\]
	pub struct PhyPARTID1;
	impl RegisterView for PhyPARTID1 {
		type Register = super::MPAMVPM0_EL2;
		const NAME: &'static str = "PhyPARTID1";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAMVPM0_EL2\[15:0\]
	pub struct PhyPARTID0;
	impl RegisterView for PhyPARTID0 {
		type Register = super::MPAMVPM0_EL2;
		const NAME: &'static str = "PhyPARTID0";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Counter-timer Physical Count Register
#[allow(non_camel_case_types)]
pub struct CNTPCT_EL0;
impl Register for CNTPCT_EL0 {
	const NAME: &'static str = "CNTPCT_EL0";
	const LEN: usize = 64;
}

/// MPAM Hypervisor Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct MPAMHCR_EL2;
impl Register for MPAMHCR_EL2 {
	const NAME: &'static str = "MPAMHCR_EL2";
	const LEN: usize = 64;
}

/// MPAMHCR_EL2 register fields
pub mod mpamhcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMHCR_EL2\[31\]
	#[allow(non_camel_case_types)]
	pub struct TRAP_MPAMIDR_EL1;
	impl RegisterView for TRAP_MPAMIDR_EL1 {
		type Register = super::MPAMHCR_EL2;
		const NAME: &'static str = "TRAP_MPAMIDR_EL1";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// MPAMHCR_EL2\[8\]
	#[allow(non_camel_case_types)]
	pub struct GSTAPP_PLK;
	impl RegisterView for GSTAPP_PLK {
		type Register = super::MPAMHCR_EL2;
		const NAME: &'static str = "GSTAPP_PLK";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// MPAMHCR_EL2\[1\]
	#[allow(non_camel_case_types)]
	pub struct EL1_VPMEN;
	impl RegisterView for EL1_VPMEN {
		type Register = super::MPAMHCR_EL2;
		const NAME: &'static str = "EL1_VPMEN";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// MPAMHCR_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct EL0_VPMEN;
	impl RegisterView for EL0_VPMEN {
		type Register = super::MPAMHCR_EL2;
		const NAME: &'static str = "EL0_VPMEN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace ID Register 0
pub struct TRCIDR0;
impl Register for TRCIDR0 {
	const NAME: &'static str = "TRCIDR0";
	const LEN: usize = 64;
}

/// TRCIDR0 register fields
pub mod trcidr0 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR0\[30\]
	pub struct COMMTRANS;
	impl RegisterView for COMMTRANS {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "COMMTRANS";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[29\]
	pub struct COMMOPT;
	impl RegisterView for COMMOPT {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "COMMOPT";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[28:24\]
	pub struct TSSIZE;
	impl RegisterView for TSSIZE {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "TSSIZE";
		const OFFSET: usize = 24;
		const LEN: usize = 5;
	}

	/// TRCIDR0\[23\]
	pub struct TSMARK;
	impl RegisterView for TSMARK {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "TSMARK";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[22\]
	pub struct ITE;
	impl RegisterView for ITE {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "ITE";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[17\]
	pub struct TRCEXDATA;
	impl RegisterView for TRCEXDATA {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "TRCEXDATA";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[16:15\]
	pub struct QSUPP;
	impl RegisterView for QSUPP {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "QSUPP";
		const OFFSET: usize = 15;
		const LEN: usize = 2;
	}

	/// TRCIDR0\[14\]
	pub struct QFILT;
	impl RegisterView for QFILT {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "QFILT";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[13:12\]
	pub struct CONDTYPE;
	impl RegisterView for CONDTYPE {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "CONDTYPE";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// TRCIDR0\[11:10\]
	pub struct NUMEVENT;
	impl RegisterView for NUMEVENT {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "NUMEVENT";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// TRCIDR0\[9\]
	pub struct RETSTACK;
	impl RegisterView for RETSTACK {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "RETSTACK";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[7\]
	pub struct TRCCCI;
	impl RegisterView for TRCCCI {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "TRCCCI";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[6\]
	pub struct TRCCOND;
	impl RegisterView for TRCCOND {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "TRCCOND";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[5\]
	pub struct TRCBB;
	impl RegisterView for TRCBB {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "TRCBB";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// TRCIDR0\[4:3\]
	pub struct TRCDATA;
	impl RegisterView for TRCDATA {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "TRCDATA";
		const OFFSET: usize = 3;
		const LEN: usize = 2;
	}

	/// TRCIDR0\[2:1\]
	pub struct INSTP0;
	impl RegisterView for INSTP0 {
		type Register = super::TRCIDR0;
		const NAME: &'static str = "INSTP0";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}
}

/// Counter-timer Physical Timer TimerValue Register
#[allow(non_camel_case_types)]
pub struct CNTP_TVAL_EL0;
impl Register for CNTP_TVAL_EL0 {
	const NAME: &'static str = "CNTP_TVAL_EL0";
	const LEN: usize = 64;
}

/// CNTP_TVAL_EL0 register fields
pub mod cntp_tval_el0 {
	use crate::core::model::proc::RegisterView;

	/// CNTP_TVAL_EL0\[31:0\]
	pub struct TimerValue;
	impl RegisterView for TimerValue {
		type Register = super::CNTP_TVAL_EL0;
		const NAME: &'static str = "TimerValue";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Interrupt Controller Active Priorities Group 1 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICC_AP1Rn_EL1;
impl RegisterArray for ICC_AP1Rn_EL1 {
	const NAME: &'static str = "ICC_AP1Rn_EL1";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// ICC_AP1Rn_EL1 register fields
pub mod icc_ap1rn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// ICC_AP1Rn_EL1\[63\]
	pub struct NMI;
	impl RegisterArrayView for NMI {
		type RegisterArray = super::ICC_AP1Rn_EL1;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}
}

/// EL0 Read/Write Software Thread ID Register
#[allow(non_camel_case_types)]
pub struct TPIDR_EL0;
impl Register for TPIDR_EL0 {
	const NAME: &'static str = "TPIDR_EL0";
	const LEN: usize = 64;
}

/// EL3 Read/Write Software Context Number
#[allow(non_camel_case_types)]
pub struct SCXTNUM_EL3;
impl Register for SCXTNUM_EL3 {
	const NAME: &'static str = "SCXTNUM_EL3";
	const LEN: usize = 64;
}

/// Interrupt Controller Virtual Interrupt Acknowledge Register 0
#[allow(non_camel_case_types)]
pub struct ICV_IAR0_EL1;
impl Register for ICV_IAR0_EL1 {
	const NAME: &'static str = "ICV_IAR0_EL1";
	const LEN: usize = 64;
}

/// ICV_IAR0_EL1 register fields
pub mod icv_iar0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_IAR0_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICV_IAR0_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Primary MECID for EL2 and EL2&0 translation regimes
#[allow(non_camel_case_types)]
pub struct MECID_P0_EL2;
impl Register for MECID_P0_EL2 {
	const NAME: &'static str = "MECID_P0_EL2";
	const LEN: usize = 64;
}

/// MECID_P0_EL2 register fields
pub mod mecid_p0_el2 {
	use crate::core::model::proc::RegisterView;

	/// MECID_P0_EL2\[15:0\]
	pub struct MECID;
	impl RegisterView for MECID {
		type Register = super::MECID_P0_EL2;
		const NAME: &'static str = "MECID";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Profiling Buffer Limit Address Register
#[allow(non_camel_case_types)]
pub struct PMBLIMITR_EL1;
impl Register for PMBLIMITR_EL1 {
	const NAME: &'static str = "PMBLIMITR_EL1";
	const LEN: usize = 64;
}

/// PMBLIMITR_EL1 register fields
pub mod pmblimitr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMBLIMITR_EL1\[63:12\]
	pub struct LIMIT;
	impl RegisterView for LIMIT {
		type Register = super::PMBLIMITR_EL1;
		const NAME: &'static str = "LIMIT";
		const OFFSET: usize = 12;
		const LEN: usize = 52;
	}

	/// PMBLIMITR_EL1\[5\]
	pub struct PMFZ;
	impl RegisterView for PMFZ {
		type Register = super::PMBLIMITR_EL1;
		const NAME: &'static str = "PMFZ";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// PMBLIMITR_EL1\[2:1\]
	pub struct FM;
	impl RegisterView for FM {
		type Register = super::PMBLIMITR_EL1;
		const NAME: &'static str = "FM";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// PMBLIMITR_EL1\[0\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::PMBLIMITR_EL1;
		const NAME: &'static str = "E";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Common Event Identification Register 0
#[allow(non_camel_case_types)]
pub struct PMCEID0_EL0;
impl Register for PMCEID0_EL0 {
	const NAME: &'static str = "PMCEID0_EL0";
	const LEN: usize = 64;
}

/// Hardware Dirty State Tracking Structure Base Register
#[allow(non_camel_case_types)]
pub struct HDBSSBR_EL2;
impl Register for HDBSSBR_EL2 {
	const NAME: &'static str = "HDBSSBR_EL2";
	const LEN: usize = 64;
}

/// HDBSSBR_EL2 register fields
pub mod hdbssbr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HDBSSBR_EL2\[55:12\]
	pub struct BADDR;
	impl RegisterView for BADDR {
		type Register = super::HDBSSBR_EL2;
		const NAME: &'static str = "BADDR";
		const OFFSET: usize = 12;
		const LEN: usize = 44;
	}

	/// HDBSSBR_EL2\[3:0\]
	pub struct SZ;
	impl RegisterView for SZ {
		type Register = super::HDBSSBR_EL2;
		const NAME: &'static str = "SZ";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Saved Program Status Register (IRQ mode)
#[allow(non_camel_case_types)]
pub struct SPSR_irq;
impl Register for SPSR_irq {
	const NAME: &'static str = "SPSR_irq";
	const LEN: usize = 64;
}

/// SPSR_irq register fields
pub mod spsr_irq {
	use crate::core::model::proc::RegisterView;

	/// SPSR_irq\[31\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "N";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[30\]
	pub struct Z;
	impl RegisterView for Z {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "Z";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[29\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "C";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[28\]
	pub struct V;
	impl RegisterView for V {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[27\]
	pub struct Q;
	impl RegisterView for Q {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "Q";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[15:10\]
	pub struct IT;
	impl RegisterView for IT {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "IT";
		const OFFSET: usize = 10;
		const LEN: usize = 6;
	}

	/// SPSR_irq\[24\]
	pub struct J;
	impl RegisterView for J {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "J";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[23\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[22\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[21\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[20\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[19:16\]
	pub struct GE;
	impl RegisterView for GE {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "GE";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SPSR_irq\[9\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "E";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[8\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "A";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[7\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "I";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[6\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "F";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[5\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "T";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SPSR_irq\[4:0\]
	pub struct M;
	impl RegisterView for M {
		type Register = super::SPSR_irq;
		const NAME: &'static str = "M";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Trace Branch Broadcast Control Register
pub struct TRCBBCTLR;
impl Register for TRCBBCTLR {
	const NAME: &'static str = "TRCBBCTLR";
	const LEN: usize = 64;
}

/// TRCBBCTLR register fields
pub mod trcbbctlr {
	use crate::core::model::proc::RegisterView;

	/// TRCBBCTLR\[8\]
	pub struct MODE;
	impl RegisterView for MODE {
		type Register = super::TRCBBCTLR;
		const NAME: &'static str = "MODE";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}
}

/// AArch64 Processor Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_AA64PFR1_EL1;
impl Register for ID_AA64PFR1_EL1 {
	const NAME: &'static str = "ID_AA64PFR1_EL1";
	const LEN: usize = 64;
}

/// ID_AA64PFR1_EL1 register fields
pub mod id_aa64pfr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64PFR1_EL1\[63:60\]
	pub struct PFAR;
	impl RegisterView for PFAR {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "PFAR";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[59:56\]
	pub struct DF2;
	impl RegisterView for DF2 {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "DF2";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[55:52\]
	pub struct MTEX;
	impl RegisterView for MTEX {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "MTEX";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[51:48\]
	pub struct THE;
	impl RegisterView for THE {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "THE";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[47:44\]
	pub struct GCS;
	impl RegisterView for GCS {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "GCS";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[43:40\]
	#[allow(non_camel_case_types)]
	pub struct MTE_frac;
	impl RegisterView for MTE_frac {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "MTE_frac";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[39:36\]
	pub struct NMI;
	impl RegisterView for NMI {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[35:32\]
	#[allow(non_camel_case_types)]
	pub struct CSV2_frac;
	impl RegisterView for CSV2_frac {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "CSV2_frac";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[31:28\]
	#[allow(non_camel_case_types)]
	pub struct RNDR_trap;
	impl RegisterView for RNDR_trap {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "RNDR_trap";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[27:24\]
	pub struct SME;
	impl RegisterView for SME {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "SME";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[19:16\]
	#[allow(non_camel_case_types)]
	pub struct MPAM_frac;
	impl RegisterView for MPAM_frac {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "MPAM_frac";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[15:12\]
	#[allow(non_camel_case_types)]
	pub struct RAS_frac;
	impl RegisterView for RAS_frac {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "RAS_frac";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[11:8\]
	pub struct MTE;
	impl RegisterView for MTE {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "MTE";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[7:4\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR1_EL1\[3:0\]
	pub struct BT;
	impl RegisterView for BT {
		type Register = super::ID_AA64PFR1_EL1;
		const NAME: &'static str = "BT";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Debug Saved Program Status Register
#[allow(non_camel_case_types)]
pub struct DSPSR_EL0;
impl Register for DSPSR_EL0 {
	const NAME: &'static str = "DSPSR_EL0";
	const LEN: usize = 64;
}

/// DSPSR_EL0 register fields
pub mod dspsr_el0 {
	use crate::core::model::proc::RegisterView;

	/// DSPSR_EL0\[33\]
	pub struct PPEND;
	impl RegisterView for PPEND {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "PPEND";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[31\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "N";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[30\]
	pub struct Z;
	impl RegisterView for Z {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "Z";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[29\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "C";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[28\]
	pub struct V;
	impl RegisterView for V {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[27\]
	pub struct Q;
	impl RegisterView for Q {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "Q";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[15:10\]
	pub struct IT;
	impl RegisterView for IT {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "IT";
		const OFFSET: usize = 10;
		const LEN: usize = 6;
	}

	/// DSPSR_EL0\[24\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[12\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[22\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[21\]
	pub struct SS;
	impl RegisterView for SS {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "SS";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[20\]
	pub struct IL;
	impl RegisterView for IL {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "IL";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[19:16\]
	pub struct GE;
	impl RegisterView for GE {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "GE";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// DSPSR_EL0\[9\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "E";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[8\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "A";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[7\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "I";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[6\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "F";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[5\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "T";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[35\]
	pub struct PACM;
	impl RegisterView for PACM {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "PACM";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[34\]
	pub struct EXLOCK;
	impl RegisterView for EXLOCK {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "EXLOCK";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[32\]
	pub struct PM;
	impl RegisterView for PM {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "PM";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[25\]
	pub struct TCO;
	impl RegisterView for TCO {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "TCO";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[23\]
	pub struct UAO;
	impl RegisterView for UAO {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "UAO";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[13\]
	pub struct ALLINT;
	impl RegisterView for ALLINT {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "ALLINT";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[11:10\]
	pub struct BTYPE;
	impl RegisterView for BTYPE {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "BTYPE";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// DSPSR_EL0\[9\]
	pub struct D;
	impl RegisterView for D {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "D";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// DSPSR_EL0\[4\]
	#[allow(non_camel_case_types)]
	pub struct M_4;
	impl RegisterView for M_4 {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "M_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// DSPSR_EL0\[4\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4;
	impl RegisterView for M_4_4 {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "M_4_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// DSPSR_EL0\[3:0\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4_3_0;
	impl RegisterView for M_4_4_3_0 {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "M_4_4_3_0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
	/// DSPSR_EL0\[3:0\]
	#[allow(non_camel_case_types)]
	pub struct M_4_4_3_0_3_0;
	impl RegisterView for M_4_4_3_0_3_0 {
		type Register = super::DSPSR_EL0;
		const NAME: &'static str = "M_4_4_3_0_3_0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Hardware Accelerator for Cleaning Dirty State Base Register
#[allow(non_camel_case_types)]
pub struct HACDBSBR_EL2;
impl Register for HACDBSBR_EL2 {
	const NAME: &'static str = "HACDBSBR_EL2";
	const LEN: usize = 64;
}

/// HACDBSBR_EL2 register fields
pub mod hacdbsbr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HACDBSBR_EL2\[55:12\]
	pub struct BADDR;
	impl RegisterView for BADDR {
		type Register = super::HACDBSBR_EL2;
		const NAME: &'static str = "BADDR";
		const OFFSET: usize = 12;
		const LEN: usize = 44;
	}

	/// HACDBSBR_EL2\[11\]
	pub struct EN;
	impl RegisterView for EN {
		type Register = super::HACDBSBR_EL2;
		const NAME: &'static str = "EN";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HACDBSBR_EL2\[3:0\]
	pub struct SZ;
	impl RegisterView for SZ {
		type Register = super::HACDBSBR_EL2;
		const NAME: &'static str = "SZ";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Current Cache Size ID Register 2
#[allow(non_camel_case_types)]
pub struct CCSIDR2_EL1;
impl Register for CCSIDR2_EL1 {
	const NAME: &'static str = "CCSIDR2_EL1";
	const LEN: usize = 64;
}

/// CCSIDR2_EL1 register fields
pub mod ccsidr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// CCSIDR2_EL1\[23:0\]
	pub struct NumSets;
	impl RegisterView for NumSets {
		type Register = super::CCSIDR2_EL1;
		const NAME: &'static str = "NumSets";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Memory Attribute Indirection Register (EL1)
#[allow(non_camel_case_types)]
pub struct MAIR_EL1;
impl Register for MAIR_EL1 {
	const NAME: &'static str = "MAIR_EL1";
	const LEN: usize = 64;
}

/// Tag Control Register.
#[allow(non_camel_case_types)]
pub struct GCR_EL1;
impl Register for GCR_EL1 {
	const NAME: &'static str = "GCR_EL1";
	const LEN: usize = 64;
}

/// GCR_EL1 register fields
pub mod gcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// GCR_EL1\[16\]
	pub struct RRND;
	impl RegisterView for RRND {
		type Register = super::GCR_EL1;
		const NAME: &'static str = "RRND";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// GCR_EL1\[15:0\]
	pub struct Exclude;
	impl RegisterView for Exclude {
		type Register = super::GCR_EL1;
		const NAME: &'static str = "Exclude";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Trace Auxiliary Control Register
pub struct TRCAUXCTLR;
impl Register for TRCAUXCTLR {
	const NAME: &'static str = "TRCAUXCTLR";
	const LEN: usize = 64;
}

/// Debug Link Register
#[allow(non_camel_case_types)]
pub struct DLR_EL0;
impl Register for DLR_EL0 {
	const NAME: &'static str = "DLR_EL0";
	const LEN: usize = 64;
}

/// AArch32 Debug Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_DFR0_EL1;
impl Register for ID_DFR0_EL1 {
	const NAME: &'static str = "ID_DFR0_EL1";
	const LEN: usize = 64;
}

/// ID_DFR0_EL1 register fields
pub mod id_dfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_DFR0_EL1\[31:28\]
	pub struct TraceFilt;
	impl RegisterView for TraceFilt {
		type Register = super::ID_DFR0_EL1;
		const NAME: &'static str = "TraceFilt";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_DFR0_EL1\[27:24\]
	pub struct PerfMon;
	impl RegisterView for PerfMon {
		type Register = super::ID_DFR0_EL1;
		const NAME: &'static str = "PerfMon";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_DFR0_EL1\[23:20\]
	pub struct MProfDbg;
	impl RegisterView for MProfDbg {
		type Register = super::ID_DFR0_EL1;
		const NAME: &'static str = "MProfDbg";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_DFR0_EL1\[19:16\]
	pub struct MMapTrc;
	impl RegisterView for MMapTrc {
		type Register = super::ID_DFR0_EL1;
		const NAME: &'static str = "MMapTrc";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_DFR0_EL1\[15:12\]
	pub struct CopTrc;
	impl RegisterView for CopTrc {
		type Register = super::ID_DFR0_EL1;
		const NAME: &'static str = "CopTrc";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_DFR0_EL1\[11:8\]
	pub struct MMapDbg;
	impl RegisterView for MMapDbg {
		type Register = super::ID_DFR0_EL1;
		const NAME: &'static str = "MMapDbg";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_DFR0_EL1\[7:4\]
	pub struct CopSDbg;
	impl RegisterView for CopSDbg {
		type Register = super::ID_DFR0_EL1;
		const NAME: &'static str = "CopSDbg";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_DFR0_EL1\[3:0\]
	pub struct CopDbg;
	impl RegisterView for CopDbg {
		type Register = super::ID_DFR0_EL1;
		const NAME: &'static str = "CopDbg";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// AArch64 Memory Model Feature Register 3
#[allow(non_camel_case_types)]
pub struct ID_AA64MMFR3_EL1;
impl Register for ID_AA64MMFR3_EL1 {
	const NAME: &'static str = "ID_AA64MMFR3_EL1";
	const LEN: usize = 64;
}

/// ID_AA64MMFR3_EL1 register fields
pub mod id_aa64mmfr3_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64MMFR3_EL1\[63:60\]
	#[allow(non_camel_case_types)]
	pub struct Spec_FPACC;
	impl RegisterView for Spec_FPACC {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "Spec_FPACC";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[59:56\]
	pub struct ADERR;
	impl RegisterView for ADERR {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "ADERR";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[55:52\]
	pub struct SDERR;
	impl RegisterView for SDERR {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "SDERR";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[47:44\]
	pub struct ANERR;
	impl RegisterView for ANERR {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "ANERR";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[43:40\]
	pub struct SNERR;
	impl RegisterView for SNERR {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "SNERR";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[39:36\]
	#[allow(non_camel_case_types)]
	pub struct D128_2;
	impl RegisterView for D128_2 {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "D128_2";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[35:32\]
	pub struct D128;
	impl RegisterView for D128 {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "D128";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[31:28\]
	pub struct MEC;
	impl RegisterView for MEC {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "MEC";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[27:24\]
	pub struct AIE;
	impl RegisterView for AIE {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "AIE";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[23:20\]
	pub struct S2POE;
	impl RegisterView for S2POE {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "S2POE";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[19:16\]
	pub struct S1POE;
	impl RegisterView for S1POE {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "S1POE";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[15:12\]
	pub struct S2PIE;
	impl RegisterView for S2PIE {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "S2PIE";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[11:8\]
	pub struct S1PIE;
	impl RegisterView for S1PIE {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "S1PIE";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[7:4\]
	pub struct SCTLRX;
	impl RegisterView for SCTLRX {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "SCTLRX";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR3_EL1\[3:0\]
	pub struct TCRX;
	impl RegisterView for TCRX {
		type Register = super::ID_AA64MMFR3_EL1;
		const NAME: &'static str = "TCRX";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Interrupt Controller Virtual Interrupt Group 0 Enable Register
#[allow(non_camel_case_types)]
pub struct ICV_IGRPEN0_EL1;
impl Register for ICV_IGRPEN0_EL1 {
	const NAME: &'static str = "ICV_IGRPEN0_EL1";
	const LEN: usize = 64;
}

/// ICV_IGRPEN0_EL1 register fields
pub mod icv_igrpen0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_IGRPEN0_EL1\[0\]
	pub struct Enable;
	impl RegisterView for Enable {
		type Register = super::ICV_IGRPEN0_EL1;
		const NAME: &'static str = "Enable";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Extended Hypervisor Configuration Register
#[allow(non_camel_case_types)]
pub struct HCRX_EL2;
impl Register for HCRX_EL2 {
	const NAME: &'static str = "HCRX_EL2";
	const LEN: usize = 64;
}

/// HCRX_EL2 register fields
pub mod hcrx_el2 {
	use crate::core::model::proc::RegisterView;

	/// HCRX_EL2\[24\]
	pub struct PACMEn;
	impl RegisterView for PACMEn {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "PACMEn";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[23\]
	pub struct EnFPM;
	impl RegisterView for EnFPM {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "EnFPM";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[22\]
	pub struct GCSEn;
	impl RegisterView for GCSEn {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "GCSEn";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[21\]
	pub struct EnIDCP128;
	impl RegisterView for EnIDCP128 {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "EnIDCP128";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[20\]
	pub struct EnSDERR;
	impl RegisterView for EnSDERR {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "EnSDERR";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[19\]
	pub struct TMEA;
	impl RegisterView for TMEA {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "TMEA";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[18\]
	pub struct EnSNERR;
	impl RegisterView for EnSNERR {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "EnSNERR";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[17\]
	pub struct D128En;
	impl RegisterView for D128En {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "D128En";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[16\]
	pub struct PTTWI;
	impl RegisterView for PTTWI {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "PTTWI";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[15\]
	pub struct SCTLR2En;
	impl RegisterView for SCTLR2En {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "SCTLR2En";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[14\]
	pub struct TCR2En;
	impl RegisterView for TCR2En {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "TCR2En";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[11\]
	pub struct MSCEn;
	impl RegisterView for MSCEn {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "MSCEn";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[10\]
	pub struct MCE2;
	impl RegisterView for MCE2 {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "MCE2";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[9\]
	pub struct CMOW;
	impl RegisterView for CMOW {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "CMOW";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[8\]
	pub struct VFNMI;
	impl RegisterView for VFNMI {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "VFNMI";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[7\]
	pub struct VINMI;
	impl RegisterView for VINMI {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "VINMI";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[6\]
	pub struct TALLINT;
	impl RegisterView for TALLINT {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "TALLINT";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[5\]
	pub struct SMPME;
	impl RegisterView for SMPME {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "SMPME";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[4\]
	pub struct FGTnXS;
	impl RegisterView for FGTnXS {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "FGTnXS";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[3\]
	pub struct FnXS;
	impl RegisterView for FnXS {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "FnXS";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[2\]
	pub struct EnASR;
	impl RegisterView for EnASR {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "EnASR";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[1\]
	pub struct EnALS;
	impl RegisterView for EnALS {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "EnALS";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HCRX_EL2\[0\]
	pub struct EnAS0;
	impl RegisterView for EnAS0 {
		type Register = super::HCRX_EL2;
		const NAME: &'static str = "EnAS0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Current Cache Size ID Register
#[allow(non_camel_case_types)]
pub struct CCSIDR_EL1;
impl Register for CCSIDR_EL1 {
	const NAME: &'static str = "CCSIDR_EL1";
	const LEN: usize = 64;
}

/// CCSIDR_EL1 register fields
pub mod ccsidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// CCSIDR_EL1\[27:13\]
	pub struct NumSets;
	impl RegisterView for NumSets {
		type Register = super::CCSIDR_EL1;
		const NAME: &'static str = "NumSets";
		const OFFSET: usize = 13;
		const LEN: usize = 15;
	}

	/// CCSIDR_EL1\[12:3\]
	pub struct Associativity;
	impl RegisterView for Associativity {
		type Register = super::CCSIDR_EL1;
		const NAME: &'static str = "Associativity";
		const OFFSET: usize = 3;
		const LEN: usize = 10;
	}

	/// CCSIDR_EL1\[2:0\]
	pub struct LineSize;
	impl RegisterView for LineSize {
		type Register = super::CCSIDR_EL1;
		const NAME: &'static str = "LineSize";
		const OFFSET: usize = 0;
		const LEN: usize = 3;
	}
}

/// AArch32 Processor Feature Register 2
#[allow(non_camel_case_types)]
pub struct ID_PFR2_EL1;
impl Register for ID_PFR2_EL1 {
	const NAME: &'static str = "ID_PFR2_EL1";
	const LEN: usize = 64;
}

/// ID_PFR2_EL1 register fields
pub mod id_pfr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_PFR2_EL1\[11:8\]
	#[allow(non_camel_case_types)]
	pub struct RAS_frac;
	impl RegisterView for RAS_frac {
		type Register = super::ID_PFR2_EL1;
		const NAME: &'static str = "RAS_frac";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_PFR2_EL1\[7:4\]
	pub struct SSBS;
	impl RegisterView for SSBS {
		type Register = super::ID_PFR2_EL1;
		const NAME: &'static str = "SSBS";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_PFR2_EL1\[3:0\]
	pub struct CSV3;
	impl RegisterView for CSV3 {
		type Register = super::ID_PFR2_EL1;
		const NAME: &'static str = "CSV3";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Debug Data Transfer Register, Transmit
#[allow(non_camel_case_types)]
pub struct DBGDTRTX_EL0;
impl Register for DBGDTRTX_EL0 {
	const NAME: &'static str = "DBGDTRTX_EL0";
	const LEN: usize = 64;
}

/// Trace Context Identifier Comparator Control Register 0
pub struct TRCCIDCCTLR0;
impl Register for TRCCIDCCTLR0 {
	const NAME: &'static str = "TRCCIDCCTLR0";
	const LEN: usize = 64;
}

/// Trace Context Identifier Comparator Control Register 1
pub struct TRCCIDCCTLR1;
impl Register for TRCCIDCCTLR1 {
	const NAME: &'static str = "TRCCIDCCTLR1";
	const LEN: usize = 64;
}

/// Interrupt Controller Interrupt Group 0 Enable Register
#[allow(non_camel_case_types)]
pub struct ICC_IGRPEN0_EL1;
impl Register for ICC_IGRPEN0_EL1 {
	const NAME: &'static str = "ICC_IGRPEN0_EL1";
	const LEN: usize = 64;
}

/// ICC_IGRPEN0_EL1 register fields
pub mod icc_igrpen0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_IGRPEN0_EL1\[0\]
	pub struct Enable;
	impl RegisterView for Enable {
		type Register = super::ICC_IGRPEN0_EL1;
		const NAME: &'static str = "Enable";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Branch Record Buffer Target Address Register \<n\>, n = 31 - 0
#[allow(non_camel_case_types)]
pub struct BRBTGTn_EL1;
impl RegisterArray for BRBTGTn_EL1 {
	const NAME: &'static str = "BRBTGTn_EL1";
	const ELEMS: usize = 31;
	const ELEM_LEN: usize = 64;
}

/// BRBTGTn_EL1 register fields
pub mod brbtgtn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// BRBTGTn_EL1\[63:0\]
	pub struct ADDRESS;
	impl RegisterArrayView for ADDRESS {
		type RegisterArray = super::BRBTGTn_EL1;
		const NAME: &'static str = "ADDRESS";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Reset Vector Base Address Register (if EL2 and EL3 not implemented)
#[allow(non_camel_case_types)]
pub struct RVBAR_EL1;
impl Register for RVBAR_EL1 {
	const NAME: &'static str = "RVBAR_EL1";
	const LEN: usize = 64;
}

/// RVBAR_EL1 register fields
pub mod rvbar_el1 {
	use crate::core::model::proc::RegisterView;

	/// RVBAR_EL1\[63:0\]
	pub struct ResetAddress;
	impl RegisterView for ResetAddress {
		type Register = super::RVBAR_EL1;
		const NAME: &'static str = "ResetAddress";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Pointer Authentication Key B for Instruction (bits\[127:64\]) 
#[allow(non_camel_case_types)]
pub struct APIBKeyHi_EL1;
impl Register for APIBKeyHi_EL1 {
	const NAME: &'static str = "APIBKeyHi_EL1";
	const LEN: usize = 64;
}

/// Activity Monitors User Enable Register
#[allow(non_camel_case_types)]
pub struct AMUSERENR_EL0;
impl Register for AMUSERENR_EL0 {
	const NAME: &'static str = "AMUSERENR_EL0";
	const LEN: usize = 64;
}

/// AMUSERENR_EL0 register fields
pub mod amuserenr_el0 {
	use crate::core::model::proc::RegisterView;

	/// AMUSERENR_EL0\[0\]
	pub struct EN;
	impl RegisterView for EN {
		type Register = super::AMUSERENR_EL0;
		const NAME: &'static str = "EN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// System Performance Monitors Event Filter Control Register, n = 63 - 0
#[allow(non_camel_case_types)]
pub struct SPMEVFILTRn_EL0;
impl RegisterArray for SPMEVFILTRn_EL0 {
	const NAME: &'static str = "SPMEVFILTRn_EL0";
	const ELEMS: usize = 63;
	const ELEM_LEN: usize = 64;
}

/// Extended Memory Attribute Indirection Register (EL1)
#[allow(non_camel_case_types)]
pub struct MAIR2_EL1;
impl Register for MAIR2_EL1 {
	const NAME: &'static str = "MAIR2_EL1";
	const LEN: usize = 64;
}

/// Trace Virtual Context Identifier Comparator Value Register \<n\>, n = 7 - 0
pub struct TRCVMIDCVRn;
impl RegisterArray for TRCVMIDCVRn {
	const NAME: &'static str = "TRCVMIDCVRn";
	const ELEMS: usize = 7;
	const ELEM_LEN: usize = 64;
}

/// TRCVMIDCVRn register fields
pub mod trcvmidcvrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCVMIDCVRn\[63:0\]
	pub struct VALUE;
	impl RegisterArrayView for VALUE {
		type RegisterArray = super::TRCVMIDCVRn;
		const NAME: &'static str = "VALUE";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Interrupt Controller Interrupt Acknowledge Register 0
#[allow(non_camel_case_types)]
pub struct ICC_IAR0_EL1;
impl Register for ICC_IAR0_EL1 {
	const NAME: &'static str = "ICC_IAR0_EL1";
	const LEN: usize = 64;
}

/// ICC_IAR0_EL1 register fields
pub mod icc_iar0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_IAR0_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_IAR0_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Interrupt Controller Highest Priority Pending Interrupt Register 1
#[allow(non_camel_case_types)]
pub struct ICC_HPPIR1_EL1;
impl Register for ICC_HPPIR1_EL1 {
	const NAME: &'static str = "ICC_HPPIR1_EL1";
	const LEN: usize = 64;
}

/// ICC_HPPIR1_EL1 register fields
pub mod icc_hppir1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_HPPIR1_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_HPPIR1_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// OS Lock Data Transfer Register, Receive
#[allow(non_camel_case_types)]
pub struct OSDTRRX_EL1;
impl Register for OSDTRRX_EL1 {
	const NAME: &'static str = "OSDTRRX_EL1";
	const LEN: usize = 64;
}

/// AArch32 Instruction Set Attribute Register 2
#[allow(non_camel_case_types)]
pub struct ID_ISAR2_EL1;
impl Register for ID_ISAR2_EL1 {
	const NAME: &'static str = "ID_ISAR2_EL1";
	const LEN: usize = 64;
}

/// ID_ISAR2_EL1 register fields
pub mod id_isar2_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_ISAR2_EL1\[31:28\]
	pub struct Reversal;
	impl RegisterView for Reversal {
		type Register = super::ID_ISAR2_EL1;
		const NAME: &'static str = "Reversal";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_ISAR2_EL1\[27:24\]
	#[allow(non_camel_case_types)]
	pub struct PSR_AR;
	impl RegisterView for PSR_AR {
		type Register = super::ID_ISAR2_EL1;
		const NAME: &'static str = "PSR_AR";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_ISAR2_EL1\[23:20\]
	pub struct MultU;
	impl RegisterView for MultU {
		type Register = super::ID_ISAR2_EL1;
		const NAME: &'static str = "MultU";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_ISAR2_EL1\[19:16\]
	pub struct MultS;
	impl RegisterView for MultS {
		type Register = super::ID_ISAR2_EL1;
		const NAME: &'static str = "MultS";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_ISAR2_EL1\[15:12\]
	pub struct Mult;
	impl RegisterView for Mult {
		type Register = super::ID_ISAR2_EL1;
		const NAME: &'static str = "Mult";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_ISAR2_EL1\[11:8\]
	pub struct MultiAccessInt;
	impl RegisterView for MultiAccessInt {
		type Register = super::ID_ISAR2_EL1;
		const NAME: &'static str = "MultiAccessInt";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_ISAR2_EL1\[7:4\]
	pub struct MemHint;
	impl RegisterView for MemHint {
		type Register = super::ID_ISAR2_EL1;
		const NAME: &'static str = "MemHint";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_ISAR2_EL1\[3:0\]
	pub struct LoadStore;
	impl RegisterView for LoadStore {
		type Register = super::ID_ISAR2_EL1;
		const NAME: &'static str = "LoadStore";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Extended Memory Attribute Indirection Register (EL3)
#[allow(non_camel_case_types)]
pub struct MAIR2_EL3;
impl Register for MAIR2_EL3 {
	const NAME: &'static str = "MAIR2_EL3";
	const LEN: usize = 64;
}

/// Memory Attribute Indirection Register (EL2)
#[allow(non_camel_case_types)]
pub struct MAIR_EL2;
impl Register for MAIR_EL2 {
	const NAME: &'static str = "MAIR_EL2";
	const LEN: usize = 64;
}

/// Primary MECID for EL1&0 stage 2 translation regime
#[allow(non_camel_case_types)]
pub struct VMECID_P_EL2;
impl Register for VMECID_P_EL2 {
	const NAME: &'static str = "VMECID_P_EL2";
	const LEN: usize = 64;
}

/// VMECID_P_EL2 register fields
pub mod vmecid_p_el2 {
	use crate::core::model::proc::RegisterView;

	/// VMECID_P_EL2\[15:0\]
	pub struct MECID;
	impl RegisterView for MECID {
		type Register = super::VMECID_P_EL2;
		const NAME: &'static str = "MECID";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// System Performance Monitors Select Register
#[allow(non_camel_case_types)]
pub struct SPMSELR_EL0;
impl Register for SPMSELR_EL0 {
	const NAME: &'static str = "SPMSELR_EL0";
	const LEN: usize = 64;
}

/// SPMSELR_EL0 register fields
pub mod spmselr_el0 {
	use crate::core::model::proc::RegisterView;

	/// SPMSELR_EL0\[9:4\]
	pub struct SYSPMUSEL;
	impl RegisterView for SYSPMUSEL {
		type Register = super::SPMSELR_EL0;
		const NAME: &'static str = "SYSPMUSEL";
		const OFFSET: usize = 4;
		const LEN: usize = 6;
	}

	/// SPMSELR_EL0\[1:0\]
	pub struct BANK;
	impl RegisterView for BANK {
		type Register = super::SPMSELR_EL0;
		const NAME: &'static str = "BANK";
		const OFFSET: usize = 0;
		const LEN: usize = 2;
	}
}

/// Debug CLAIM Tag Clear Register
#[allow(non_camel_case_types)]
pub struct DBGCLAIMCLR_EL1;
impl Register for DBGCLAIMCLR_EL1 {
	const NAME: &'static str = "DBGCLAIMCLR_EL1";
	const LEN: usize = 64;
}

/// DBGCLAIMCLR_EL1 register fields
pub mod dbgclaimclr_el1 {
	use crate::core::model::proc::RegisterView;

	/// DBGCLAIMCLR_EL1\[7:0\]
	pub struct CLAIM;
	impl RegisterView for CLAIM {
		type Register = super::DBGCLAIMCLR_EL1;
		const NAME: &'static str = "CLAIM";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Branch Record Buffer Information Register \<n\>, n = 31 - 0
#[allow(non_camel_case_types)]
pub struct BRBINFn_EL1;
impl RegisterArray for BRBINFn_EL1 {
	const NAME: &'static str = "BRBINFn_EL1";
	const ELEMS: usize = 31;
	const ELEM_LEN: usize = 64;
}

/// BRBINFn_EL1 register fields
pub mod brbinfn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// BRBINFn_EL1\[46\]
	pub struct CCU;
	impl RegisterArrayView for CCU {
		type RegisterArray = super::BRBINFn_EL1;
		const NAME: &'static str = "CCU";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// BRBINFn_EL1\[45:32\]
	pub struct CC;
	impl RegisterArrayView for CC {
		type RegisterArray = super::BRBINFn_EL1;
		const NAME: &'static str = "CC";
		const OFFSET: usize = 32;
		const LEN: usize = 14;
	}

	/// BRBINFn_EL1\[17\]
	pub struct LASTFAILED;
	impl RegisterArrayView for LASTFAILED {
		type RegisterArray = super::BRBINFn_EL1;
		const NAME: &'static str = "LASTFAILED";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// BRBINFn_EL1\[16\]
	pub struct T;
	impl RegisterArrayView for T {
		type RegisterArray = super::BRBINFn_EL1;
		const NAME: &'static str = "T";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// BRBINFn_EL1\[13:8\]
	pub struct TYPE;
	impl RegisterArrayView for TYPE {
		type RegisterArray = super::BRBINFn_EL1;
		const NAME: &'static str = "TYPE";
		const OFFSET: usize = 8;
		const LEN: usize = 6;
	}

	/// BRBINFn_EL1\[7:6\]
	pub struct EL;
	impl RegisterArrayView for EL {
		type RegisterArray = super::BRBINFn_EL1;
		const NAME: &'static str = "EL";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// BRBINFn_EL1\[5\]
	pub struct MPRED;
	impl RegisterArrayView for MPRED {
		type RegisterArray = super::BRBINFn_EL1;
		const NAME: &'static str = "MPRED";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// BRBINFn_EL1\[1:0\]
	pub struct VALID;
	impl RegisterArrayView for VALID {
		type RegisterArray = super::BRBINFn_EL1;
		const NAME: &'static str = "VALID";
		const OFFSET: usize = 0;
		const LEN: usize = 2;
	}
}

/// Performance Monitors Selected Event Type Register
#[allow(non_camel_case_types)]
pub struct PMXEVTYPER_EL0;
impl Register for PMXEVTYPER_EL0 {
	const NAME: &'static str = "PMXEVTYPER_EL0";
	const LEN: usize = 64;
}

/// System Performance Monitors Zero with Mask
#[allow(non_camel_case_types)]
pub struct SPMZR_EL0;
impl Register for SPMZR_EL0 {
	const NAME: &'static str = "SPMZR_EL0";
	const LEN: usize = 64;
}

/// Reset Vector Base Address Register (if EL3 implemented)
#[allow(non_camel_case_types)]
pub struct RVBAR_EL3;
impl Register for RVBAR_EL3 {
	const NAME: &'static str = "RVBAR_EL3";
	const LEN: usize = 64;
}

/// RVBAR_EL3 register fields
pub mod rvbar_el3 {
	use crate::core::model::proc::RegisterView;

	/// RVBAR_EL3\[63:0\]
	pub struct ResetAddress;
	impl RegisterView for ResetAddress {
		type Register = super::RVBAR_EL3;
		const NAME: &'static str = "ResetAddress";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Counter-timer Physical Timer CompareValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHP_CVAL_EL2;
impl Register for CNTHP_CVAL_EL2 {
	const NAME: &'static str = "CNTHP_CVAL_EL2";
	const LEN: usize = 64;
}

/// CNTHP_CVAL_EL2 register fields
pub mod cnthp_cval_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHP_CVAL_EL2\[63:0\]
	pub struct CompareValue;
	impl RegisterView for CompareValue {
		type Register = super::CNTHP_CVAL_EL2;
		const NAME: &'static str = "CompareValue";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Branch Record Buffer Information Injection Register
#[allow(non_camel_case_types)]
pub struct BRBINFINJ_EL1;
impl Register for BRBINFINJ_EL1 {
	const NAME: &'static str = "BRBINFINJ_EL1";
	const LEN: usize = 64;
}

/// BRBINFINJ_EL1 register fields
pub mod brbinfinj_el1 {
	use crate::core::model::proc::RegisterView;

	/// BRBINFINJ_EL1\[46\]
	pub struct CCU;
	impl RegisterView for CCU {
		type Register = super::BRBINFINJ_EL1;
		const NAME: &'static str = "CCU";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// BRBINFINJ_EL1\[45:32\]
	pub struct CC;
	impl RegisterView for CC {
		type Register = super::BRBINFINJ_EL1;
		const NAME: &'static str = "CC";
		const OFFSET: usize = 32;
		const LEN: usize = 14;
	}

	/// BRBINFINJ_EL1\[17\]
	pub struct LASTFAILED;
	impl RegisterView for LASTFAILED {
		type Register = super::BRBINFINJ_EL1;
		const NAME: &'static str = "LASTFAILED";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// BRBINFINJ_EL1\[16\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::BRBINFINJ_EL1;
		const NAME: &'static str = "T";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// BRBINFINJ_EL1\[13:8\]
	pub struct TYPE;
	impl RegisterView for TYPE {
		type Register = super::BRBINFINJ_EL1;
		const NAME: &'static str = "TYPE";
		const OFFSET: usize = 8;
		const LEN: usize = 6;
	}

	/// BRBINFINJ_EL1\[7:6\]
	pub struct EL;
	impl RegisterView for EL {
		type Register = super::BRBINFINJ_EL1;
		const NAME: &'static str = "EL";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// BRBINFINJ_EL1\[5\]
	pub struct MPRED;
	impl RegisterView for MPRED {
		type Register = super::BRBINFINJ_EL1;
		const NAME: &'static str = "MPRED";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// BRBINFINJ_EL1\[1:0\]
	pub struct VALID;
	impl RegisterView for VALID {
		type Register = super::BRBINFINJ_EL1;
		const NAME: &'static str = "VALID";
		const OFFSET: usize = 0;
		const LEN: usize = 2;
	}
}

/// Selected Error Record Miscellaneous Register 1
#[allow(non_camel_case_types)]
pub struct ERXMISC1_EL1;
impl Register for ERXMISC1_EL1 {
	const NAME: &'static str = "ERXMISC1_EL1";
	const LEN: usize = 64;
}

/// Trace Address Comparator Access Type Register \<n\>, n = 15 - 0
pub struct TRCACATRn;
impl RegisterArray for TRCACATRn {
	const NAME: &'static str = "TRCACATRn";
	const ELEMS: usize = 15;
	const ELEM_LEN: usize = 64;
}

/// TRCACATRn register fields
pub mod trcacatrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCACATRn\[18\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL2;
	impl RegisterArrayView for EXLEVEL_RL_EL2 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_RL_EL2";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[17\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL1;
	impl RegisterArrayView for EXLEVEL_RL_EL1 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_RL_EL1";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[16\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_RL_EL0;
	impl RegisterArrayView for EXLEVEL_RL_EL0 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_RL_EL0";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[14\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL2;
	impl RegisterArrayView for EXLEVEL_NS_EL2 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_NS_EL2";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[13\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL1;
	impl RegisterArrayView for EXLEVEL_NS_EL1 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_NS_EL1";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[12\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_NS_EL0;
	impl RegisterArrayView for EXLEVEL_NS_EL0 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_NS_EL0";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[11\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL3;
	impl RegisterArrayView for EXLEVEL_S_EL3 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_S_EL3";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[10\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL2;
	impl RegisterArrayView for EXLEVEL_S_EL2 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_S_EL2";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[9\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL1;
	impl RegisterArrayView for EXLEVEL_S_EL1 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_S_EL1";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[8\]
	#[allow(non_camel_case_types)]
	pub struct EXLEVEL_S_EL0;
	impl RegisterArrayView for EXLEVEL_S_EL0 {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "EXLEVEL_S_EL0";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// TRCACATRn\[6:4\]
	pub struct CONTEXT;
	impl RegisterArrayView for CONTEXT {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "CONTEXT";
		const OFFSET: usize = 4;
		const LEN: usize = 3;
	}

	/// TRCACATRn\[3:2\]
	pub struct CONTEXTTYPE;
	impl RegisterArrayView for CONTEXTTYPE {
		type RegisterArray = super::TRCACATRn;
		const NAME: &'static str = "CONTEXTTYPE";
		const OFFSET: usize = 2;
		const LEN: usize = 2;
	}
}

/// Performance Monitors Interrupt Enable Clear Register
#[allow(non_camel_case_types)]
pub struct PMINTENCLR_EL1;
impl Register for PMINTENCLR_EL1 {
	const NAME: &'static str = "PMINTENCLR_EL1";
	const LEN: usize = 64;
}

/// PMINTENCLR_EL1 register fields
pub mod pmintenclr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMINTENCLR_EL1\[31\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMINTENCLR_EL1;
		const NAME: &'static str = "C";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
}

/// Reset Vector Base Address Register (if EL3 not implemented)
#[allow(non_camel_case_types)]
pub struct RVBAR_EL2;
impl Register for RVBAR_EL2 {
	const NAME: &'static str = "RVBAR_EL2";
	const LEN: usize = 64;
}

/// RVBAR_EL2 register fields
pub mod rvbar_el2 {
	use crate::core::model::proc::RegisterView;

	/// RVBAR_EL2\[63:0\]
	pub struct ResetAddress;
	impl RegisterView for ResetAddress {
		type Register = super::RVBAR_EL2;
		const NAME: &'static str = "ResetAddress";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Activity Monitors Counter Group 1 Identification Register
#[allow(non_camel_case_types)]
pub struct AMCG1IDR_EL0;
impl Register for AMCG1IDR_EL0 {
	const NAME: &'static str = "AMCG1IDR_EL0";
	const LEN: usize = 64;
}

/// Interrupt Controller Virtual Binary Point Register 0
#[allow(non_camel_case_types)]
pub struct ICV_BPR0_EL1;
impl Register for ICV_BPR0_EL1 {
	const NAME: &'static str = "ICV_BPR0_EL1";
	const LEN: usize = 64;
}

/// ICV_BPR0_EL1 register fields
pub mod icv_bpr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_BPR0_EL1\[2:0\]
	pub struct BinaryPoint;
	impl RegisterView for BinaryPoint {
		type Register = super::ICV_BPR0_EL1;
		const NAME: &'static str = "BinaryPoint";
		const OFFSET: usize = 0;
		const LEN: usize = 3;
	}
}

/// Counter-timer Physical Secure Timer TimerValue Register
#[allow(non_camel_case_types)]
pub struct CNTPS_TVAL_EL1;
impl Register for CNTPS_TVAL_EL1 {
	const NAME: &'static str = "CNTPS_TVAL_EL1";
	const LEN: usize = 64;
}

/// CNTPS_TVAL_EL1 register fields
pub mod cntps_tval_el1 {
	use crate::core::model::proc::RegisterView;

	/// CNTPS_TVAL_EL1\[31:0\]
	pub struct TimerValue;
	impl RegisterView for TimerValue {
		type Register = super::CNTPS_TVAL_EL1;
		const NAME: &'static str = "TimerValue";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Activity Monitors Event Type Registers 0, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct AMEVTYPER0n_EL0;
impl RegisterArray for AMEVTYPER0n_EL0 {
	const NAME: &'static str = "AMEVTYPER0n_EL0";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// AMEVTYPER0n_EL0 register fields
pub mod amevtyper0n_el0 {
	use crate::core::model::proc::RegisterArrayView;

	/// AMEVTYPER0n_EL0\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct evtCount;
	impl RegisterArrayView for evtCount {
		type RegisterArray = super::AMEVTYPER0n_EL0;
		const NAME: &'static str = "evtCount";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Activity Monitors Event Type Registers 1, n = 15 - 0
#[allow(non_camel_case_types)]
pub struct AMEVTYPER1n_EL0;
impl RegisterArray for AMEVTYPER1n_EL0 {
	const NAME: &'static str = "AMEVTYPER1n_EL0";
	const ELEMS: usize = 15;
	const ELEM_LEN: usize = 64;
}

/// AMEVTYPER1n_EL0 register fields
pub mod amevtyper1n_el0 {
	use crate::core::model::proc::RegisterArrayView;

	/// AMEVTYPER1n_EL0\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct evtCount;
	impl RegisterArrayView for evtCount {
		type RegisterArray = super::AMEVTYPER1n_EL0;
		const NAME: &'static str = "evtCount";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// MPAM Virtual PARTID Mapping Register 2
#[allow(non_camel_case_types)]
pub struct MPAMVPM2_EL2;
impl Register for MPAMVPM2_EL2 {
	const NAME: &'static str = "MPAMVPM2_EL2";
	const LEN: usize = 64;
}

/// MPAMVPM2_EL2 register fields
pub mod mpamvpm2_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMVPM2_EL2\[63:48\]
	pub struct PhyPARTID11;
	impl RegisterView for PhyPARTID11 {
		type Register = super::MPAMVPM2_EL2;
		const NAME: &'static str = "PhyPARTID11";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// MPAMVPM2_EL2\[47:32\]
	pub struct PhyPARTID10;
	impl RegisterView for PhyPARTID10 {
		type Register = super::MPAMVPM2_EL2;
		const NAME: &'static str = "PhyPARTID10";
		const OFFSET: usize = 32;
		const LEN: usize = 16;
	}

	/// MPAMVPM2_EL2\[31:16\]
	pub struct PhyPARTID9;
	impl RegisterView for PhyPARTID9 {
		type Register = super::MPAMVPM2_EL2;
		const NAME: &'static str = "PhyPARTID9";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAMVPM2_EL2\[15:0\]
	pub struct PhyPARTID8;
	impl RegisterView for PhyPARTID8 {
		type Register = super::MPAMVPM2_EL2;
		const NAME: &'static str = "PhyPARTID8";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Memory Attribute Indirection Register (EL3)
#[allow(non_camel_case_types)]
pub struct MAIR_EL3;
impl Register for MAIR_EL3 {
	const NAME: &'static str = "MAIR_EL3";
	const LEN: usize = 64;
}

/// AArch32 Media and VFP Feature Register 2
#[allow(non_camel_case_types)]
pub struct MVFR2_EL1;
impl Register for MVFR2_EL1 {
	const NAME: &'static str = "MVFR2_EL1";
	const LEN: usize = 64;
}

/// MVFR2_EL1 register fields
pub mod mvfr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// MVFR2_EL1\[7:4\]
	pub struct FPMisc;
	impl RegisterView for FPMisc {
		type Register = super::MVFR2_EL1;
		const NAME: &'static str = "FPMisc";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// MVFR2_EL1\[3:0\]
	pub struct SIMDMisc;
	impl RegisterView for SIMDMisc {
		type Register = super::MVFR2_EL1;
		const NAME: &'static str = "SIMDMisc";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Extended Memory Attribute Indirection Register (EL2)
#[allow(non_camel_case_types)]
pub struct MAIR2_EL2;
impl Register for MAIR2_EL2 {
	const NAME: &'static str = "MAIR2_EL2";
	const LEN: usize = 64;
}

/// MEC Identification Register
#[allow(non_camel_case_types)]
pub struct MECIDR_EL2;
impl Register for MECIDR_EL2 {
	const NAME: &'static str = "MECIDR_EL2";
	const LEN: usize = 64;
}

/// MECIDR_EL2 register fields
pub mod mecidr_el2 {
	use crate::core::model::proc::RegisterView;

	/// MECIDR_EL2\[3:0\]
	pub struct MECIDWidthm1;
	impl RegisterView for MECIDWidthm1 {
		type Register = super::MECIDR_EL2;
		const NAME: &'static str = "MECIDWidthm1";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Counter-timer Secure Virtual Timer CompareValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHVS_CVAL_EL2;
impl Register for CNTHVS_CVAL_EL2 {
	const NAME: &'static str = "CNTHVS_CVAL_EL2";
	const LEN: usize = 64;
}

/// CNTHVS_CVAL_EL2 register fields
pub mod cnthvs_cval_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHVS_CVAL_EL2\[63:0\]
	pub struct CompareValue;
	impl RegisterView for CompareValue {
		type Register = super::CNTHVS_CVAL_EL2;
		const NAME: &'static str = "CompareValue";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Trace External Input Select Register \<n\>, n = 3 - 0
pub struct TRCEXTINSELRn;
impl RegisterArray for TRCEXTINSELRn {
	const NAME: &'static str = "TRCEXTINSELRn";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// TRCEXTINSELRn register fields
pub mod trcextinselrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCEXTINSELRn\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct evtCount;
	impl RegisterArrayView for evtCount {
		type RegisterArray = super::TRCEXTINSELRn;
		const NAME: &'static str = "evtCount";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Interrupt Controller Virtual Active Priorities Group 0 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICV_AP0Rn_EL1;
impl RegisterArray for ICV_AP0Rn_EL1 {
	const NAME: &'static str = "ICV_AP0Rn_EL1";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// Hardware Dirty State Tracking Structure Producer Register
#[allow(non_camel_case_types)]
pub struct HDBSSPROD_EL2;
impl Register for HDBSSPROD_EL2 {
	const NAME: &'static str = "HDBSSPROD_EL2";
	const LEN: usize = 64;
}

/// HDBSSPROD_EL2 register fields
pub mod hdbssprod_el2 {
	use crate::core::model::proc::RegisterView;

	/// HDBSSPROD_EL2\[31:26\]
	pub struct FSC;
	impl RegisterView for FSC {
		type Register = super::HDBSSPROD_EL2;
		const NAME: &'static str = "FSC";
		const OFFSET: usize = 26;
		const LEN: usize = 6;
	}

	/// HDBSSPROD_EL2\[18:0\]
	pub struct INDEX;
	impl RegisterView for INDEX {
		type Register = super::HDBSSPROD_EL2;
		const NAME: &'static str = "INDEX";
		const OFFSET: usize = 0;
		const LEN: usize = 19;
	}
}

/// Activity Monitors Event Counter Virtual Offset Registers 0, n = 15 - 0
#[allow(non_camel_case_types)]
pub struct AMEVCNTVOFF0n_EL2;
impl RegisterArray for AMEVCNTVOFF0n_EL2 {
	const NAME: &'static str = "AMEVCNTVOFF0n_EL2";
	const ELEMS: usize = 15;
	const ELEM_LEN: usize = 64;
}

/// System Performance Monitors Event Count Register, n = 63 - 0
#[allow(non_camel_case_types)]
pub struct SPMEVCNTRn_EL0;
impl RegisterArray for SPMEVCNTRn_EL0 {
	const NAME: &'static str = "SPMEVCNTRn_EL0";
	const ELEMS: usize = 63;
	const ELEM_LEN: usize = 64;
}

/// SPMEVCNTRn_EL0 register fields
pub mod spmevcntrn_el0 {
	use crate::core::model::proc::RegisterArrayView;

	/// SPMEVCNTRn_EL0\[63:0\]
	pub struct CNTR;
	impl RegisterArrayView for CNTR {
		type RegisterArray = super::SPMEVCNTRn_EL0;
		const NAME: &'static str = "CNTR";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Interrupt Controller Interrupt Acknowledge Register 1
#[allow(non_camel_case_types)]
pub struct ICC_IAR1_EL1;
impl Register for ICC_IAR1_EL1 {
	const NAME: &'static str = "ICC_IAR1_EL1";
	const LEN: usize = 64;
}

/// ICC_IAR1_EL1 register fields
pub mod icc_iar1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_IAR1_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_IAR1_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Interrupt Status Register
#[allow(non_camel_case_types)]
pub struct ISR_EL1;
impl Register for ISR_EL1 {
	const NAME: &'static str = "ISR_EL1";
	const LEN: usize = 64;
}

/// ISR_EL1 register fields
pub mod isr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ISR_EL1\[10\]
	pub struct IS;
	impl RegisterView for IS {
		type Register = super::ISR_EL1;
		const NAME: &'static str = "IS";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// ISR_EL1\[9\]
	pub struct FS;
	impl RegisterView for FS {
		type Register = super::ISR_EL1;
		const NAME: &'static str = "FS";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// ISR_EL1\[8\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::ISR_EL1;
		const NAME: &'static str = "A";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// ISR_EL1\[7\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::ISR_EL1;
		const NAME: &'static str = "I";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// ISR_EL1\[6\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::ISR_EL1;
		const NAME: &'static str = "F";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller Highest Priority Pending Interrupt Register 0
#[allow(non_camel_case_types)]
pub struct ICC_HPPIR0_EL1;
impl Register for ICC_HPPIR0_EL1 {
	const NAME: &'static str = "ICC_HPPIR0_EL1";
	const LEN: usize = 64;
}

/// ICC_HPPIR0_EL1 register fields
pub mod icc_hppir0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_HPPIR0_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICC_HPPIR0_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Activity Monitors Event Counter Virtual Offset Registers 1, n = 15 - 0
#[allow(non_camel_case_types)]
pub struct AMEVCNTVOFF1n_EL2;
impl RegisterArray for AMEVCNTVOFF1n_EL2 {
	const NAME: &'static str = "AMEVCNTVOFF1n_EL2";
	const ELEMS: usize = 15;
	const ELEM_LEN: usize = 64;
}

/// Translation Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct TCR_EL1;
impl Register for TCR_EL1 {
	const NAME: &'static str = "TCR_EL1";
	const LEN: usize = 64;
}

/// TCR_EL1 register fields
pub mod tcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// TCR_EL1\[61\]
	pub struct MTX1;
	impl RegisterView for MTX1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "MTX1";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[60\]
	pub struct MTX0;
	impl RegisterView for MTX0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "MTX0";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[59\]
	pub struct DS;
	impl RegisterView for DS {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "DS";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[58\]
	pub struct TCMA1;
	impl RegisterView for TCMA1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "TCMA1";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[57\]
	pub struct TCMA0;
	impl RegisterView for TCMA0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "TCMA0";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[56\]
	pub struct E0PD1;
	impl RegisterView for E0PD1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "E0PD1";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[55\]
	pub struct E0PD0;
	impl RegisterView for E0PD0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "E0PD0";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[54\]
	pub struct NFD1;
	impl RegisterView for NFD1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "NFD1";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[53\]
	pub struct NFD0;
	impl RegisterView for NFD0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "NFD0";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[52\]
	pub struct TBID1;
	impl RegisterView for TBID1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "TBID1";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[51\]
	pub struct TBID0;
	impl RegisterView for TBID0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "TBID0";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[50\]
	pub struct HWU162;
	impl RegisterView for HWU162 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HWU162";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[49\]
	pub struct HWU161;
	impl RegisterView for HWU161 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HWU161";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[48\]
	pub struct HWU160;
	impl RegisterView for HWU160 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HWU160";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[47\]
	pub struct HWU159;
	impl RegisterView for HWU159 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HWU159";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[46\]
	pub struct HWU062;
	impl RegisterView for HWU062 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HWU062";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[45\]
	pub struct HWU061;
	impl RegisterView for HWU061 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HWU061";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[44\]
	pub struct HWU060;
	impl RegisterView for HWU060 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HWU060";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[43\]
	pub struct HWU059;
	impl RegisterView for HWU059 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HWU059";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[42\]
	pub struct HPD1;
	impl RegisterView for HPD1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HPD1";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[41\]
	pub struct HPD0;
	impl RegisterView for HPD0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HPD0";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[40\]
	pub struct HD;
	impl RegisterView for HD {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HD";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[39\]
	pub struct HA;
	impl RegisterView for HA {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "HA";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[38\]
	pub struct TBI1;
	impl RegisterView for TBI1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "TBI1";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[37\]
	pub struct TBI0;
	impl RegisterView for TBI0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "TBI0";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[36\]
	pub struct AS;
	impl RegisterView for AS {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "AS";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[34:32\]
	pub struct IPS;
	impl RegisterView for IPS {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "IPS";
		const OFFSET: usize = 32;
		const LEN: usize = 3;
	}

	/// TCR_EL1\[31:30\]
	pub struct TG1;
	impl RegisterView for TG1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "TG1";
		const OFFSET: usize = 30;
		const LEN: usize = 2;
	}

	/// TCR_EL1\[29:28\]
	pub struct SH1;
	impl RegisterView for SH1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "SH1";
		const OFFSET: usize = 28;
		const LEN: usize = 2;
	}

	/// TCR_EL1\[27:26\]
	pub struct ORGN1;
	impl RegisterView for ORGN1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "ORGN1";
		const OFFSET: usize = 26;
		const LEN: usize = 2;
	}

	/// TCR_EL1\[25:24\]
	pub struct IRGN1;
	impl RegisterView for IRGN1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "IRGN1";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// TCR_EL1\[23\]
	pub struct EPD1;
	impl RegisterView for EPD1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "EPD1";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[22\]
	pub struct A1;
	impl RegisterView for A1 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "A1";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[21:16\]
	pub struct T1SZ;
	impl RegisterView for T1SZ {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "T1SZ";
		const OFFSET: usize = 16;
		const LEN: usize = 6;
	}

	/// TCR_EL1\[15:14\]
	pub struct TG0;
	impl RegisterView for TG0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "TG0";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// TCR_EL1\[13:12\]
	pub struct SH0;
	impl RegisterView for SH0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "SH0";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// TCR_EL1\[11:10\]
	pub struct ORGN0;
	impl RegisterView for ORGN0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "ORGN0";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// TCR_EL1\[9:8\]
	pub struct IRGN0;
	impl RegisterView for IRGN0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "IRGN0";
		const OFFSET: usize = 8;
		const LEN: usize = 2;
	}

	/// TCR_EL1\[7\]
	pub struct EPD0;
	impl RegisterView for EPD0 {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "EPD0";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TCR_EL1\[5:0\]
	pub struct T0SZ;
	impl RegisterView for T0SZ {
		type Register = super::TCR_EL1;
		const NAME: &'static str = "T0SZ";
		const OFFSET: usize = 0;
		const LEN: usize = 6;
	}
}

/// Trace Resources Status Register
pub struct TRCRSR;
impl Register for TRCRSR {
	const NAME: &'static str = "TRCRSR";
	const LEN: usize = 64;
}

/// TRCRSR register fields
pub mod trcrsr {
	use crate::core::model::proc::RegisterView;

	/// TRCRSR\[12\]
	pub struct TA;
	impl RegisterView for TA {
		type Register = super::TRCRSR;
		const NAME: &'static str = "TA";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}
}

/// AArch32 Instruction Set Attribute Register 3
#[allow(non_camel_case_types)]
pub struct ID_ISAR3_EL1;
impl Register for ID_ISAR3_EL1 {
	const NAME: &'static str = "ID_ISAR3_EL1";
	const LEN: usize = 64;
}

/// ID_ISAR3_EL1 register fields
pub mod id_isar3_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_ISAR3_EL1\[31:28\]
	pub struct T32EE;
	impl RegisterView for T32EE {
		type Register = super::ID_ISAR3_EL1;
		const NAME: &'static str = "T32EE";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_ISAR3_EL1\[27:24\]
	pub struct TrueNOP;
	impl RegisterView for TrueNOP {
		type Register = super::ID_ISAR3_EL1;
		const NAME: &'static str = "TrueNOP";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_ISAR3_EL1\[23:20\]
	pub struct T32Copy;
	impl RegisterView for T32Copy {
		type Register = super::ID_ISAR3_EL1;
		const NAME: &'static str = "T32Copy";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_ISAR3_EL1\[19:16\]
	pub struct TabBranch;
	impl RegisterView for TabBranch {
		type Register = super::ID_ISAR3_EL1;
		const NAME: &'static str = "TabBranch";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_ISAR3_EL1\[15:12\]
	pub struct SynchPrim;
	impl RegisterView for SynchPrim {
		type Register = super::ID_ISAR3_EL1;
		const NAME: &'static str = "SynchPrim";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_ISAR3_EL1\[11:8\]
	pub struct SVC;
	impl RegisterView for SVC {
		type Register = super::ID_ISAR3_EL1;
		const NAME: &'static str = "SVC";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_ISAR3_EL1\[7:4\]
	pub struct SIMD;
	impl RegisterView for SIMD {
		type Register = super::ID_ISAR3_EL1;
		const NAME: &'static str = "SIMD";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_ISAR3_EL1\[3:0\]
	pub struct Saturate;
	impl RegisterView for Saturate {
		type Register = super::ID_ISAR3_EL1;
		const NAME: &'static str = "Saturate";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Trace Device Configuration Register
pub struct TRCDEVID;
impl Register for TRCDEVID {
	const NAME: &'static str = "TRCDEVID";
	const LEN: usize = 64;
}

/// Selected Error Record Miscellaneous Register 0
#[allow(non_camel_case_types)]
pub struct ERXMISC0_EL1;
impl Register for ERXMISC0_EL1 {
	const NAME: &'static str = "ERXMISC0_EL1";
	const LEN: usize = 64;
}

/// Interrupt Controller Deactivate Virtual Interrupt Register
#[allow(non_camel_case_types)]
pub struct ICV_DIR_EL1;
impl Register for ICV_DIR_EL1 {
	const NAME: &'static str = "ICV_DIR_EL1";
	const LEN: usize = 64;
}

/// ICV_DIR_EL1 register fields
pub mod icv_dir_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_DIR_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICV_DIR_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Interrupt Controller Interrupt Group 1 Enable Register (EL3)
#[allow(non_camel_case_types)]
pub struct ICC_IGRPEN1_EL3;
impl Register for ICC_IGRPEN1_EL3 {
	const NAME: &'static str = "ICC_IGRPEN1_EL3";
	const LEN: usize = 64;
}

/// ICC_IGRPEN1_EL3 register fields
pub mod icc_igrpen1_el3 {
	use crate::core::model::proc::RegisterView;

	/// ICC_IGRPEN1_EL3\[1\]
	pub struct EnableGrp1S;
	impl RegisterView for EnableGrp1S {
		type Register = super::ICC_IGRPEN1_EL3;
		const NAME: &'static str = "EnableGrp1S";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICC_IGRPEN1_EL3\[0\]
	pub struct EnableGrp1NS;
	impl RegisterView for EnableGrp1NS {
		type Register = super::ICC_IGRPEN1_EL3;
		const NAME: &'static str = "EnableGrp1NS";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// MPAM Virtual Partition Mapping Valid Register
#[allow(non_camel_case_types)]
pub struct MPAMVPMV_EL2;
impl Register for MPAMVPMV_EL2 {
	const NAME: &'static str = "MPAMVPMV_EL2";
	const LEN: usize = 64;
}

/// Performance Monitors Extended Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct PMECR_EL1;
impl Register for PMECR_EL1 {
	const NAME: &'static str = "PMECR_EL1";
	const LEN: usize = 64;
}

/// PMECR_EL1 register fields
pub mod pmecr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMECR_EL1\[4:3\]
	pub struct SSE;
	impl RegisterView for SSE {
		type Register = super::PMECR_EL1;
		const NAME: &'static str = "SSE";
		const OFFSET: usize = 3;
		const LEN: usize = 2;
	}

	/// PMECR_EL1\[2\]
	pub struct KPME;
	impl RegisterView for KPME {
		type Register = super::PMECR_EL1;
		const NAME: &'static str = "KPME";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// PMECR_EL1\[1:0\]
	pub struct PMEE;
	impl RegisterView for PMEE {
		type Register = super::PMECR_EL1;
		const NAME: &'static str = "PMEE";
		const OFFSET: usize = 0;
		const LEN: usize = 2;
	}
}

/// Interrupt Controller Virtual Binary Point Register 1
#[allow(non_camel_case_types)]
pub struct ICV_BPR1_EL1;
impl Register for ICV_BPR1_EL1 {
	const NAME: &'static str = "ICV_BPR1_EL1";
	const LEN: usize = 64;
}

/// ICV_BPR1_EL1 register fields
pub mod icv_bpr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_BPR1_EL1\[2:0\]
	pub struct BinaryPoint;
	impl RegisterView for BinaryPoint {
		type Register = super::ICV_BPR1_EL1;
		const NAME: &'static str = "BinaryPoint";
		const OFFSET: usize = 0;
		const LEN: usize = 3;
	}
}

/// Physical Fault Address Register (EL1)
#[allow(non_camel_case_types)]
pub struct PFAR_EL1;
impl Register for PFAR_EL1 {
	const NAME: &'static str = "PFAR_EL1";
	const LEN: usize = 64;
}

/// PFAR_EL1 register fields
pub mod pfar_el1 {
	use crate::core::model::proc::RegisterView;

	/// PFAR_EL1\[63\]
	pub struct NS;
	impl RegisterView for NS {
		type Register = super::PFAR_EL1;
		const NAME: &'static str = "NS";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// PFAR_EL1\[62\]
	pub struct NSE;
	impl RegisterView for NSE {
		type Register = super::PFAR_EL1;
		const NAME: &'static str = "NSE";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// PFAR_EL1\[55:52\]
	#[allow(non_camel_case_types)]
	pub struct PA_55_52;
	impl RegisterView for PA_55_52 {
		type Register = super::PFAR_EL1;
		const NAME: &'static str = "PA_55_52";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}
	/// PFAR_EL1\[51:48\]
	#[allow(non_camel_case_types)]
	pub struct PA_55_52_51_48;
	impl RegisterView for PA_55_52_51_48 {
		type Register = super::PFAR_EL1;
		const NAME: &'static str = "PA_55_52_51_48";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}
}

/// Trace Counter Reload Value Register \<n\>, n = 3 - 0
pub struct TRCCNTRLDVRn;
impl RegisterArray for TRCCNTRLDVRn {
	const NAME: &'static str = "TRCCNTRLDVRn";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// TRCCNTRLDVRn register fields
pub mod trccntrldvrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCCNTRLDVRn\[15:0\]
	pub struct VALUE;
	impl RegisterArrayView for VALUE {
		type RegisterArray = super::TRCCNTRLDVRn;
		const NAME: &'static str = "VALUE";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Counter-timer Physical Timer Control Register
#[allow(non_camel_case_types)]
pub struct CNTP_CTL_EL0;
impl Register for CNTP_CTL_EL0 {
	const NAME: &'static str = "CNTP_CTL_EL0";
	const LEN: usize = 64;
}

/// CNTP_CTL_EL0 register fields
pub mod cntp_ctl_el0 {
	use crate::core::model::proc::RegisterView;

	/// CNTP_CTL_EL0\[2\]
	pub struct ISTATUS;
	impl RegisterView for ISTATUS {
		type Register = super::CNTP_CTL_EL0;
		const NAME: &'static str = "ISTATUS";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTP_CTL_EL0\[1\]
	pub struct IMASK;
	impl RegisterView for IMASK {
		type Register = super::CNTP_CTL_EL0;
		const NAME: &'static str = "IMASK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTP_CTL_EL0\[0\]
	pub struct ENABLE;
	impl RegisterView for ENABLE {
		type Register = super::CNTP_CTL_EL0;
		const NAME: &'static str = "ENABLE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Streaming Mode Priority Register
#[allow(non_camel_case_types)]
pub struct SMPRI_EL1;
impl Register for SMPRI_EL1 {
	const NAME: &'static str = "SMPRI_EL1";
	const LEN: usize = 64;
}

/// SMPRI_EL1 register fields
pub mod smpri_el1 {
	use crate::core::model::proc::RegisterView;

	/// SMPRI_EL1\[3:0\]
	pub struct Priority;
	impl RegisterView for Priority {
		type Register = super::SMPRI_EL1;
		const NAME: &'static str = "Priority";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Performance Monitors Cycle Count Filter Register
#[allow(non_camel_case_types)]
pub struct PMCCFILTR_EL0;
impl Register for PMCCFILTR_EL0 {
	const NAME: &'static str = "PMCCFILTR_EL0";
	const LEN: usize = 64;
}

/// PMCCFILTR_EL0 register fields
pub mod pmccfiltr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMCCFILTR_EL0\[57:56\]
	pub struct VS;
	impl RegisterView for VS {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "VS";
		const OFFSET: usize = 56;
		const LEN: usize = 2;
	}

	/// PMCCFILTR_EL0\[31\]
	pub struct P;
	impl RegisterView for P {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "P";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[30\]
	pub struct U;
	impl RegisterView for U {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "U";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[29\]
	pub struct NSK;
	impl RegisterView for NSK {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "NSK";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[28\]
	pub struct NSU;
	impl RegisterView for NSU {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "NSU";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[27\]
	pub struct NSH;
	impl RegisterView for NSH {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "NSH";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[26\]
	pub struct M;
	impl RegisterView for M {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "M";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[24\]
	pub struct SH;
	impl RegisterView for SH {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "SH";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[23\]
	pub struct T;
	impl RegisterView for T {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "T";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[22\]
	pub struct RLK;
	impl RegisterView for RLK {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "RLK";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[21\]
	pub struct RLU;
	impl RegisterView for RLU {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "RLU";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// PMCCFILTR_EL0\[20\]
	pub struct RLH;
	impl RegisterView for RLH {
		type Register = super::PMCCFILTR_EL0;
		const NAME: &'static str = "RLH";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}
}

/// MPAM Virtual PARTID Mapping Register 3
#[allow(non_camel_case_types)]
pub struct MPAMVPM3_EL2;
impl Register for MPAMVPM3_EL2 {
	const NAME: &'static str = "MPAMVPM3_EL2";
	const LEN: usize = 64;
}

/// MPAMVPM3_EL2 register fields
pub mod mpamvpm3_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMVPM3_EL2\[63:48\]
	pub struct PhyPARTID15;
	impl RegisterView for PhyPARTID15 {
		type Register = super::MPAMVPM3_EL2;
		const NAME: &'static str = "PhyPARTID15";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// MPAMVPM3_EL2\[47:32\]
	pub struct PhyPARTID14;
	impl RegisterView for PhyPARTID14 {
		type Register = super::MPAMVPM3_EL2;
		const NAME: &'static str = "PhyPARTID14";
		const OFFSET: usize = 32;
		const LEN: usize = 16;
	}

	/// MPAMVPM3_EL2\[31:16\]
	pub struct PhyPARTID13;
	impl RegisterView for PhyPARTID13 {
		type Register = super::MPAMVPM3_EL2;
		const NAME: &'static str = "PhyPARTID13";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAMVPM3_EL2\[15:0\]
	pub struct PhyPARTID12;
	impl RegisterView for PhyPARTID12 {
		type Register = super::MPAMVPM3_EL2;
		const NAME: &'static str = "PhyPARTID12";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Trace ViewInst Include/Exclude Control Register
pub struct TRCVIIECTLR;
impl Register for TRCVIIECTLR {
	const NAME: &'static str = "TRCVIIECTLR";
	const LEN: usize = 64;
}

/// OS Lock Access Register
#[allow(non_camel_case_types)]
pub struct OSLAR_EL1;
impl Register for OSLAR_EL1 {
	const NAME: &'static str = "OSLAR_EL1";
	const LEN: usize = 64;
}

/// OSLAR_EL1 register fields
pub mod oslar_el1 {
	use crate::core::model::proc::RegisterView;

	/// OSLAR_EL1\[0\]
	pub struct OSLK;
	impl RegisterView for OSLK {
		type Register = super::OSLAR_EL1;
		const NAME: &'static str = "OSLK";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Data Cache Zero ID Register
#[allow(non_camel_case_types)]
pub struct DCZID_EL0;
impl Register for DCZID_EL0 {
	const NAME: &'static str = "DCZID_EL0";
	const LEN: usize = 64;
}

/// DCZID_EL0 register fields
pub mod dczid_el0 {
	use crate::core::model::proc::RegisterView;

	/// DCZID_EL0\[4\]
	pub struct DZP;
	impl RegisterView for DZP {
		type Register = super::DCZID_EL0;
		const NAME: &'static str = "DZP";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// DCZID_EL0\[3:0\]
	pub struct BS;
	impl RegisterView for BS {
		type Register = super::DCZID_EL0;
		const NAME: &'static str = "BS";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Hypervisor Auxiliary Control Register
#[allow(non_camel_case_types)]
pub struct HACR_EL2;
impl Register for HACR_EL2 {
	const NAME: &'static str = "HACR_EL2";
	const LEN: usize = 64;
}

/// Cache Type Register
#[allow(non_camel_case_types)]
pub struct CTR_EL0;
impl Register for CTR_EL0 {
	const NAME: &'static str = "CTR_EL0";
	const LEN: usize = 64;
}

/// CTR_EL0 register fields
pub mod ctr_el0 {
	use crate::core::model::proc::RegisterView;

	/// CTR_EL0\[37:32\]
	pub struct TminLine;
	impl RegisterView for TminLine {
		type Register = super::CTR_EL0;
		const NAME: &'static str = "TminLine";
		const OFFSET: usize = 32;
		const LEN: usize = 6;
	}

	/// CTR_EL0\[29\]
	pub struct DIC;
	impl RegisterView for DIC {
		type Register = super::CTR_EL0;
		const NAME: &'static str = "DIC";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// CTR_EL0\[28\]
	pub struct IDC;
	impl RegisterView for IDC {
		type Register = super::CTR_EL0;
		const NAME: &'static str = "IDC";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// CTR_EL0\[27:24\]
	pub struct CWG;
	impl RegisterView for CWG {
		type Register = super::CTR_EL0;
		const NAME: &'static str = "CWG";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// CTR_EL0\[23:20\]
	pub struct ERG;
	impl RegisterView for ERG {
		type Register = super::CTR_EL0;
		const NAME: &'static str = "ERG";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// CTR_EL0\[19:16\]
	pub struct DminLine;
	impl RegisterView for DminLine {
		type Register = super::CTR_EL0;
		const NAME: &'static str = "DminLine";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// CTR_EL0\[15:14\]
	pub struct L1Ip;
	impl RegisterView for L1Ip {
		type Register = super::CTR_EL0;
		const NAME: &'static str = "L1Ip";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// CTR_EL0\[3:0\]
	pub struct IminLine;
	impl RegisterView for IminLine {
		type Register = super::CTR_EL0;
		const NAME: &'static str = "IminLine";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// AArch32 Auxiliary Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_AFR0_EL1;
impl Register for ID_AFR0_EL1 {
	const NAME: &'static str = "ID_AFR0_EL1";
	const LEN: usize = 64;
}

/// System Performance Monitors Count Enable Set Register
#[allow(non_camel_case_types)]
pub struct SPMCNTENSET_EL0;
impl Register for SPMCNTENSET_EL0 {
	const NAME: &'static str = "SPMCNTENSET_EL0";
	const LEN: usize = 64;
}

/// Debug Feature Register 2
#[allow(non_camel_case_types)]
pub struct ID_AA64DFR2_EL1;
impl Register for ID_AA64DFR2_EL1 {
	const NAME: &'static str = "ID_AA64DFR2_EL1";
	const LEN: usize = 64;
}

/// ID_AA64DFR2_EL1 register fields
pub mod id_aa64dfr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64DFR2_EL1\[7:4\]
	pub struct BWE;
	impl RegisterView for BWE {
		type Register = super::ID_AA64DFR2_EL1;
		const NAME: &'static str = "BWE";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64DFR2_EL1\[3:0\]
	pub struct STEP;
	impl RegisterView for STEP {
		type Register = super::ID_AA64DFR2_EL1;
		const NAME: &'static str = "STEP";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Physical Address Register
#[allow(non_camel_case_types)]
pub struct PAR_EL1;
impl Register for PAR_EL1 {
	const NAME: &'static str = "PAR_EL1";
	const LEN: usize = 64;
}

/// PAR_EL1 register fields
pub mod par_el1 {
	use crate::core::model::proc::RegisterView;

	/// PAR_EL1\[51:48\]
	#[allow(non_camel_case_types)]
	pub struct PA_51_48;
	impl RegisterView for PA_51_48 {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "PA_51_48";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}
	/// PAR_EL1\[51:48\]
	#[allow(non_camel_case_types)]
	pub struct PA_51_48_51_48;
	impl RegisterView for PA_51_48_51_48 {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "PA_51_48_51_48";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}
	/// PAR_EL1\[47:12\]
	#[allow(non_camel_case_types)]
	pub struct PA_51_48_51_48_47_12;
	impl RegisterView for PA_51_48_51_48_47_12 {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "PA_51_48_51_48_47_12";
		const OFFSET: usize = 12;
		const LEN: usize = 36;
	}
	/// PAR_EL1\[47:12\]
	#[allow(non_camel_case_types)]
	pub struct PA_51_48_51_48_47_12_47_12;
	impl RegisterView for PA_51_48_51_48_47_12_47_12 {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "PA_51_48_51_48_47_12_47_12";
		const OFFSET: usize = 12;
		const LEN: usize = 36;
	}

	/// PAR_EL1\[64\]
	pub struct D128;
	impl RegisterView for D128 {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "D128";
		const OFFSET: usize = 64;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[63:56\]
	pub struct ATTR;
	impl RegisterView for ATTR {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "ATTR";
		const OFFSET: usize = 56;
		const LEN: usize = 8;
	}

	/// PAR_EL1\[11\]
	pub struct NSE;
	impl RegisterView for NSE {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "NSE";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[9\]
	pub struct NS;
	impl RegisterView for NS {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "NS";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[8:7\]
	pub struct SH;
	impl RegisterView for SH {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "SH";
		const OFFSET: usize = 7;
		const LEN: usize = 2;
	}

	/// PAR_EL1\[0\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "F";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[15\]
	pub struct DirtyBit;
	impl RegisterView for DirtyBit {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "DirtyBit";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[14\]
	pub struct Overlay;
	impl RegisterView for Overlay {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "Overlay";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[13\]
	pub struct TopLevel;
	impl RegisterView for TopLevel {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "TopLevel";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[12\]
	pub struct AssuredOnly;
	impl RegisterView for AssuredOnly {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "AssuredOnly";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[9\]
	pub struct S;
	impl RegisterView for S {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "S";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[8\]
	pub struct PTW;
	impl RegisterView for PTW {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "PTW";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// PAR_EL1\[6:1\]
	pub struct FST;
	impl RegisterView for FST {
		type Register = super::PAR_EL1;
		const NAME: &'static str = "FST";
		const OFFSET: usize = 1;
		const LEN: usize = 6;
	}
}

/// Sampling Data Source Filter Register
#[allow(non_camel_case_types)]
pub struct PMSDSFR_EL1;
impl Register for PMSDSFR_EL1 {
	const NAME: &'static str = "PMSDSFR_EL1";
	const LEN: usize = 64;
}

/// Multiprocessor Affinity Register
#[allow(non_camel_case_types)]
pub struct MPIDR_EL1;
impl Register for MPIDR_EL1 {
	const NAME: &'static str = "MPIDR_EL1";
	const LEN: usize = 64;
}

/// MPIDR_EL1 register fields
pub mod mpidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// MPIDR_EL1\[39:32\]
	pub struct Aff3;
	impl RegisterView for Aff3 {
		type Register = super::MPIDR_EL1;
		const NAME: &'static str = "Aff3";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// MPIDR_EL1\[30\]
	pub struct U;
	impl RegisterView for U {
		type Register = super::MPIDR_EL1;
		const NAME: &'static str = "U";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// MPIDR_EL1\[24\]
	pub struct MT;
	impl RegisterView for MT {
		type Register = super::MPIDR_EL1;
		const NAME: &'static str = "MT";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// MPIDR_EL1\[23:16\]
	pub struct Aff2;
	impl RegisterView for Aff2 {
		type Register = super::MPIDR_EL1;
		const NAME: &'static str = "Aff2";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// MPIDR_EL1\[15:8\]
	pub struct Aff1;
	impl RegisterView for Aff1 {
		type Register = super::MPIDR_EL1;
		const NAME: &'static str = "Aff1";
		const OFFSET: usize = 8;
		const LEN: usize = 8;
	}

	/// MPIDR_EL1\[7:0\]
	pub struct Aff0;
	impl RegisterView for Aff0 {
		type Register = super::MPIDR_EL1;
		const NAME: &'static str = "Aff0";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Trace Cycle Count Control Register
pub struct TRCCCCTLR;
impl Register for TRCCCCTLR {
	const NAME: &'static str = "TRCCCCTLR";
	const LEN: usize = 64;
}

/// TRCCCCTLR register fields
pub mod trcccctlr {
	use crate::core::model::proc::RegisterView;

	/// TRCCCCTLR\[11:0\]
	pub struct THRESHOLD;
	impl RegisterView for THRESHOLD {
		type Register = super::TRCCCCTLR;
		const NAME: &'static str = "THRESHOLD";
		const OFFSET: usize = 0;
		const LEN: usize = 12;
	}
}

/// Translation Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct TCR_EL2;
impl Register for TCR_EL2 {
	const NAME: &'static str = "TCR_EL2";
	const LEN: usize = 64;
}

/// TCR_EL2 register fields
pub mod tcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// TCR_EL2\[33\]
	pub struct MTX;
	impl RegisterView for MTX {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "MTX";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[59\]
	pub struct DS;
	impl RegisterView for DS {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "DS";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[30\]
	pub struct TCMA;
	impl RegisterView for TCMA {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TCMA";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[29\]
	pub struct TBID;
	impl RegisterView for TBID {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TBID";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[28\]
	pub struct HWU62;
	impl RegisterView for HWU62 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU62";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[27\]
	pub struct HWU61;
	impl RegisterView for HWU61 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU61";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[26\]
	pub struct HWU60;
	impl RegisterView for HWU60 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU60";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[25\]
	pub struct HWU59;
	impl RegisterView for HWU59 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU59";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[24\]
	pub struct HPD;
	impl RegisterView for HPD {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HPD";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[40\]
	pub struct HD;
	impl RegisterView for HD {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HD";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[39\]
	pub struct HA;
	impl RegisterView for HA {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HA";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[20\]
	pub struct TBI;
	impl RegisterView for TBI {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TBI";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[18:16\]
	pub struct PS;
	impl RegisterView for PS {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "PS";
		const OFFSET: usize = 16;
		const LEN: usize = 3;
	}

	/// TCR_EL2\[15:14\]
	pub struct TG0;
	impl RegisterView for TG0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TG0";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// TCR_EL2\[13:12\]
	pub struct SH0;
	impl RegisterView for SH0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "SH0";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// TCR_EL2\[11:10\]
	pub struct ORGN0;
	impl RegisterView for ORGN0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "ORGN0";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// TCR_EL2\[9:8\]
	pub struct IRGN0;
	impl RegisterView for IRGN0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "IRGN0";
		const OFFSET: usize = 8;
		const LEN: usize = 2;
	}

	/// TCR_EL2\[5:0\]
	pub struct T0SZ;
	impl RegisterView for T0SZ {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "T0SZ";
		const OFFSET: usize = 0;
		const LEN: usize = 6;
	}

	/// TCR_EL2\[61\]
	pub struct MTX1;
	impl RegisterView for MTX1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "MTX1";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[60\]
	pub struct MTX0;
	impl RegisterView for MTX0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "MTX0";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[58\]
	pub struct TCMA1;
	impl RegisterView for TCMA1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TCMA1";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[57\]
	pub struct TCMA0;
	impl RegisterView for TCMA0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TCMA0";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[56\]
	pub struct E0PD1;
	impl RegisterView for E0PD1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "E0PD1";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[55\]
	pub struct E0PD0;
	impl RegisterView for E0PD0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "E0PD0";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[54\]
	pub struct NFD1;
	impl RegisterView for NFD1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "NFD1";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[53\]
	pub struct NFD0;
	impl RegisterView for NFD0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "NFD0";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[52\]
	pub struct TBID1;
	impl RegisterView for TBID1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TBID1";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[51\]
	pub struct TBID0;
	impl RegisterView for TBID0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TBID0";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[50\]
	pub struct HWU162;
	impl RegisterView for HWU162 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU162";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[49\]
	pub struct HWU161;
	impl RegisterView for HWU161 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU161";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[48\]
	pub struct HWU160;
	impl RegisterView for HWU160 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU160";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[47\]
	pub struct HWU159;
	impl RegisterView for HWU159 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU159";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[46\]
	pub struct HWU062;
	impl RegisterView for HWU062 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU062";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[45\]
	pub struct HWU061;
	impl RegisterView for HWU061 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU061";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[44\]
	pub struct HWU060;
	impl RegisterView for HWU060 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU060";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[43\]
	pub struct HWU059;
	impl RegisterView for HWU059 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HWU059";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[42\]
	pub struct HPD1;
	impl RegisterView for HPD1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HPD1";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[41\]
	pub struct HPD0;
	impl RegisterView for HPD0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "HPD0";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[38\]
	pub struct TBI1;
	impl RegisterView for TBI1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TBI1";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[37\]
	pub struct TBI0;
	impl RegisterView for TBI0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TBI0";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[36\]
	pub struct AS;
	impl RegisterView for AS {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "AS";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[34:32\]
	pub struct IPS;
	impl RegisterView for IPS {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "IPS";
		const OFFSET: usize = 32;
		const LEN: usize = 3;
	}

	/// TCR_EL2\[31:30\]
	pub struct TG1;
	impl RegisterView for TG1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "TG1";
		const OFFSET: usize = 30;
		const LEN: usize = 2;
	}

	/// TCR_EL2\[29:28\]
	pub struct SH1;
	impl RegisterView for SH1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "SH1";
		const OFFSET: usize = 28;
		const LEN: usize = 2;
	}

	/// TCR_EL2\[27:26\]
	pub struct ORGN1;
	impl RegisterView for ORGN1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "ORGN1";
		const OFFSET: usize = 26;
		const LEN: usize = 2;
	}

	/// TCR_EL2\[25:24\]
	pub struct IRGN1;
	impl RegisterView for IRGN1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "IRGN1";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// TCR_EL2\[23\]
	pub struct EPD1;
	impl RegisterView for EPD1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "EPD1";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[22\]
	pub struct A1;
	impl RegisterView for A1 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "A1";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// TCR_EL2\[21:16\]
	pub struct T1SZ;
	impl RegisterView for T1SZ {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "T1SZ";
		const OFFSET: usize = 16;
		const LEN: usize = 6;
	}

	/// TCR_EL2\[7\]
	pub struct EPD0;
	impl RegisterView for EPD0 {
		type Register = super::TCR_EL2;
		const NAME: &'static str = "EPD0";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}
}

/// Auxiliary ID Register
#[allow(non_camel_case_types)]
pub struct AIDR_EL1;
impl Register for AIDR_EL1 {
	const NAME: &'static str = "AIDR_EL1";
	const LEN: usize = 64;
}

/// AArch64 Processor Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_AA64PFR0_EL1;
impl Register for ID_AA64PFR0_EL1 {
	const NAME: &'static str = "ID_AA64PFR0_EL1";
	const LEN: usize = 64;
}

/// ID_AA64PFR0_EL1 register fields
pub mod id_aa64pfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64PFR0_EL1\[63:60\]
	pub struct CSV3;
	impl RegisterView for CSV3 {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "CSV3";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[59:56\]
	pub struct CSV2;
	impl RegisterView for CSV2 {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "CSV2";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[55:52\]
	pub struct RME;
	impl RegisterView for RME {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "RME";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[51:48\]
	pub struct DIT;
	impl RegisterView for DIT {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "DIT";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[47:44\]
	pub struct AMU;
	impl RegisterView for AMU {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "AMU";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[43:40\]
	pub struct MPAM;
	impl RegisterView for MPAM {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "MPAM";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[39:36\]
	pub struct SEL2;
	impl RegisterView for SEL2 {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "SEL2";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[35:32\]
	pub struct SVE;
	impl RegisterView for SVE {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "SVE";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[31:28\]
	pub struct RAS;
	impl RegisterView for RAS {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "RAS";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[27:24\]
	pub struct GIC;
	impl RegisterView for GIC {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "GIC";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[23:20\]
	pub struct AdvSIMD;
	impl RegisterView for AdvSIMD {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "AdvSIMD";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[19:16\]
	pub struct FP;
	impl RegisterView for FP {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "FP";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[15:12\]
	pub struct EL3;
	impl RegisterView for EL3 {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "EL3";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[11:8\]
	pub struct EL2;
	impl RegisterView for EL2 {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "EL2";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[7:4\]
	pub struct EL1;
	impl RegisterView for EL1 {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "EL1";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64PFR0_EL1\[3:0\]
	pub struct EL0;
	impl RegisterView for EL0 {
		type Register = super::ID_AA64PFR0_EL1;
		const NAME: &'static str = "EL0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Pointer Authentication Key B for Data (bits\[63:0\]) 
#[allow(non_camel_case_types)]
pub struct APDBKeyLo_EL1;
impl Register for APDBKeyLo_EL1 {
	const NAME: &'static str = "APDBKeyLo_EL1";
	const LEN: usize = 64;
}

/// Interrupt Controller Virtual Interrupt Group 1 Enable Register
#[allow(non_camel_case_types)]
pub struct ICV_IGRPEN1_EL1;
impl Register for ICV_IGRPEN1_EL1 {
	const NAME: &'static str = "ICV_IGRPEN1_EL1";
	const LEN: usize = 64;
}

/// ICV_IGRPEN1_EL1 register fields
pub mod icv_igrpen1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_IGRPEN1_EL1\[0\]
	pub struct Enable;
	impl RegisterView for Enable {
		type Register = super::ICV_IGRPEN1_EL1;
		const NAME: &'static str = "Enable";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Buffer ID Register
#[allow(non_camel_case_types)]
pub struct TRBIDR_EL1;
impl Register for TRBIDR_EL1 {
	const NAME: &'static str = "TRBIDR_EL1";
	const LEN: usize = 64;
}

/// TRBIDR_EL1 register fields
pub mod trbidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRBIDR_EL1\[15:12\]
	pub struct MPAM;
	impl RegisterView for MPAM {
		type Register = super::TRBIDR_EL1;
		const NAME: &'static str = "MPAM";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// TRBIDR_EL1\[11:8\]
	pub struct EA;
	impl RegisterView for EA {
		type Register = super::TRBIDR_EL1;
		const NAME: &'static str = "EA";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// TRBIDR_EL1\[5\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::TRBIDR_EL1;
		const NAME: &'static str = "F";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// TRBIDR_EL1\[4\]
	pub struct P;
	impl RegisterView for P {
		type Register = super::TRBIDR_EL1;
		const NAME: &'static str = "P";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// TRBIDR_EL1\[3:0\]
	pub struct Align;
	impl RegisterView for Align {
		type Register = super::TRBIDR_EL1;
		const NAME: &'static str = "Align";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Streaming Mode Priority Mapping Register
#[allow(non_camel_case_types)]
pub struct SMPRIMAP_EL2;
impl Register for SMPRIMAP_EL2 {
	const NAME: &'static str = "SMPRIMAP_EL2";
	const LEN: usize = 64;
}

/// SMPRIMAP_EL2 register fields
pub mod smprimap_el2 {
	use crate::core::model::proc::RegisterView;

	/// SMPRIMAP_EL2\[63:60\]
	pub struct P15;
	impl RegisterView for P15 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P15";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[59:56\]
	pub struct P14;
	impl RegisterView for P14 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P14";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[55:52\]
	pub struct P13;
	impl RegisterView for P13 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P13";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[51:48\]
	pub struct P12;
	impl RegisterView for P12 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P12";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[47:44\]
	pub struct P11;
	impl RegisterView for P11 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P11";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[43:40\]
	pub struct P10;
	impl RegisterView for P10 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P10";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[39:36\]
	pub struct P9;
	impl RegisterView for P9 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P9";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[35:32\]
	pub struct P8;
	impl RegisterView for P8 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P8";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[31:28\]
	pub struct P7;
	impl RegisterView for P7 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P7";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[27:24\]
	pub struct P6;
	impl RegisterView for P6 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P6";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[23:20\]
	pub struct P5;
	impl RegisterView for P5 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P5";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[19:16\]
	pub struct P4;
	impl RegisterView for P4 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P4";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[15:12\]
	pub struct P3;
	impl RegisterView for P3 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P3";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[11:8\]
	pub struct P2;
	impl RegisterView for P2 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P2";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[7:4\]
	pub struct P1;
	impl RegisterView for P1 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P1";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// SMPRIMAP_EL2\[3:0\]
	pub struct P0;
	impl RegisterView for P0 {
		type Register = super::SMPRIMAP_EL2;
		const NAME: &'static str = "P0";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// System Performance Monitors Configuration Register
#[allow(non_camel_case_types)]
pub struct SPMCFGR_EL1;
impl Register for SPMCFGR_EL1 {
	const NAME: &'static str = "SPMCFGR_EL1";
	const LEN: usize = 64;
}

/// SPMCFGR_EL1 register fields
pub mod spmcfgr_el1 {
	use crate::core::model::proc::RegisterView;

	/// SPMCFGR_EL1\[31:28\]
	pub struct NCG;
	impl RegisterView for NCG {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "NCG";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// SPMCFGR_EL1\[24\]
	pub struct HDBG;
	impl RegisterView for HDBG {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "HDBG";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SPMCFGR_EL1\[23\]
	pub struct TRO;
	impl RegisterView for TRO {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "TRO";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SPMCFGR_EL1\[22\]
	pub struct SS;
	impl RegisterView for SS {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "SS";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SPMCFGR_EL1\[21\]
	pub struct FZO;
	impl RegisterView for FZO {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "FZO";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SPMCFGR_EL1\[20\]
	pub struct MSI;
	impl RegisterView for MSI {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "MSI";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SPMCFGR_EL1\[17\]
	pub struct NA;
	impl RegisterView for NA {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "NA";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// SPMCFGR_EL1\[16\]
	pub struct EX;
	impl RegisterView for EX {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "EX";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// SPMCFGR_EL1\[13:8\]
	pub struct SIZE;
	impl RegisterView for SIZE {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "SIZE";
		const OFFSET: usize = 8;
		const LEN: usize = 6;
	}

	/// SPMCFGR_EL1\[7:0\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::SPMCFGR_EL1;
		const NAME: &'static str = "N";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Virtualization Translation Control Register
#[allow(non_camel_case_types)]
pub struct VTCR_EL2;
impl Register for VTCR_EL2 {
	const NAME: &'static str = "VTCR_EL2";
	const LEN: usize = 64;
}

/// VTCR_EL2 register fields
pub mod vtcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VTCR_EL2\[45\]
	pub struct HDBSS;
	impl RegisterView for HDBSS {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "HDBSS";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[44\]
	pub struct HAFT;
	impl RegisterView for HAFT {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "HAFT";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[41\]
	pub struct TL0;
	impl RegisterView for TL0 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "TL0";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[40\]
	pub struct GCSH;
	impl RegisterView for GCSH {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "GCSH";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[38\]
	pub struct D128;
	impl RegisterView for D128 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "D128";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[37\]
	pub struct S2POE;
	impl RegisterView for S2POE {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "S2POE";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[36\]
	pub struct S2PIE;
	impl RegisterView for S2PIE {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "S2PIE";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[35\]
	pub struct TL1;
	impl RegisterView for TL1 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "TL1";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[34\]
	pub struct AssuredOnly;
	impl RegisterView for AssuredOnly {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "AssuredOnly";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[33\]
	pub struct SL2;
	impl RegisterView for SL2 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "SL2";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[32\]
	pub struct DS;
	impl RegisterView for DS {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "DS";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[30\]
	pub struct NSA;
	impl RegisterView for NSA {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "NSA";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[29\]
	pub struct NSW;
	impl RegisterView for NSW {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "NSW";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[28\]
	pub struct HWU62;
	impl RegisterView for HWU62 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "HWU62";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[27\]
	pub struct HWU61;
	impl RegisterView for HWU61 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "HWU61";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[26\]
	pub struct HWU60;
	impl RegisterView for HWU60 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "HWU60";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[25\]
	pub struct HWU59;
	impl RegisterView for HWU59 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "HWU59";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[22\]
	pub struct HD;
	impl RegisterView for HD {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "HD";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[21\]
	pub struct HA;
	impl RegisterView for HA {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "HA";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[19\]
	pub struct VS;
	impl RegisterView for VS {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "VS";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// VTCR_EL2\[18:16\]
	pub struct PS;
	impl RegisterView for PS {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "PS";
		const OFFSET: usize = 16;
		const LEN: usize = 3;
	}

	/// VTCR_EL2\[15:14\]
	pub struct TG0;
	impl RegisterView for TG0 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "TG0";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// VTCR_EL2\[13:12\]
	pub struct SH0;
	impl RegisterView for SH0 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "SH0";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// VTCR_EL2\[11:10\]
	pub struct ORGN0;
	impl RegisterView for ORGN0 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "ORGN0";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// VTCR_EL2\[9:8\]
	pub struct IRGN0;
	impl RegisterView for IRGN0 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "IRGN0";
		const OFFSET: usize = 8;
		const LEN: usize = 2;
	}

	/// VTCR_EL2\[7:6\]
	pub struct SL0;
	impl RegisterView for SL0 {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "SL0";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// VTCR_EL2\[5:0\]
	pub struct T0SZ;
	impl RegisterView for T0SZ {
		type Register = super::VTCR_EL2;
		const NAME: &'static str = "T0SZ";
		const OFFSET: usize = 0;
		const LEN: usize = 6;
	}
}

/// OS Lock Exception Catch Control Register
#[allow(non_camel_case_types)]
pub struct OSECCR_EL1;
impl Register for OSECCR_EL1 {
	const NAME: &'static str = "OSECCR_EL1";
	const LEN: usize = 64;
}

/// OSECCR_EL1 register fields
pub mod oseccr_el1 {
	use crate::core::model::proc::RegisterView;

	/// OSECCR_EL1\[31:0\]
	pub struct EDECCR;
	impl RegisterView for EDECCR {
		type Register = super::OSECCR_EL1;
		const NAME: &'static str = "EDECCR";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// AArch64 Memory Model Feature Register 2
#[allow(non_camel_case_types)]
pub struct ID_AA64MMFR2_EL1;
impl Register for ID_AA64MMFR2_EL1 {
	const NAME: &'static str = "ID_AA64MMFR2_EL1";
	const LEN: usize = 64;
}

/// ID_AA64MMFR2_EL1 register fields
pub mod id_aa64mmfr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64MMFR2_EL1\[63:60\]
	pub struct E0PD;
	impl RegisterView for E0PD {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "E0PD";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[59:56\]
	pub struct EVT;
	impl RegisterView for EVT {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "EVT";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[55:52\]
	pub struct BBM;
	impl RegisterView for BBM {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "BBM";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[51:48\]
	pub struct TTL;
	impl RegisterView for TTL {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "TTL";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[43:40\]
	pub struct FWB;
	impl RegisterView for FWB {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "FWB";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[39:36\]
	pub struct IDS;
	impl RegisterView for IDS {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "IDS";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[35:32\]
	pub struct AT;
	impl RegisterView for AT {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "AT";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[31:28\]
	pub struct ST;
	impl RegisterView for ST {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "ST";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[27:24\]
	pub struct NV;
	impl RegisterView for NV {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "NV";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[23:20\]
	pub struct CCIDX;
	impl RegisterView for CCIDX {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "CCIDX";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[19:16\]
	pub struct VARange;
	impl RegisterView for VARange {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "VARange";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[15:12\]
	pub struct IESB;
	impl RegisterView for IESB {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "IESB";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[11:8\]
	pub struct LSM;
	impl RegisterView for LSM {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "LSM";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[7:4\]
	pub struct UAO;
	impl RegisterView for UAO {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "UAO";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR2_EL1\[3:0\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::ID_AA64MMFR2_EL1;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Debug Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_DFR1_EL1;
impl Register for ID_DFR1_EL1 {
	const NAME: &'static str = "ID_DFR1_EL1";
	const LEN: usize = 64;
}

/// ID_DFR1_EL1 register fields
pub mod id_dfr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_DFR1_EL1\[7:4\]
	pub struct HPMN0;
	impl RegisterView for HPMN0 {
		type Register = super::ID_DFR1_EL1;
		const NAME: &'static str = "HPMN0";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_DFR1_EL1\[3:0\]
	pub struct MTPMU;
	impl RegisterView for MTPMU {
		type Register = super::ID_DFR1_EL1;
		const NAME: &'static str = "MTPMU";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Counter-timer Hypervisor Physical Timer Control Register
#[allow(non_camel_case_types)]
pub struct CNTHP_CTL_EL2;
impl Register for CNTHP_CTL_EL2 {
	const NAME: &'static str = "CNTHP_CTL_EL2";
	const LEN: usize = 64;
}

/// CNTHP_CTL_EL2 register fields
pub mod cnthp_ctl_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHP_CTL_EL2\[2\]
	pub struct ISTATUS;
	impl RegisterView for ISTATUS {
		type Register = super::CNTHP_CTL_EL2;
		const NAME: &'static str = "ISTATUS";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTHP_CTL_EL2\[1\]
	pub struct IMASK;
	impl RegisterView for IMASK {
		type Register = super::CNTHP_CTL_EL2;
		const NAME: &'static str = "IMASK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTHP_CTL_EL2\[0\]
	pub struct ENABLE;
	impl RegisterView for ENABLE {
		type Register = super::CNTHP_CTL_EL2;
		const NAME: &'static str = "ENABLE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller Active Priorities Group 0 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICC_AP0Rn_EL1;
impl RegisterArray for ICC_AP0Rn_EL1 {
	const NAME: &'static str = "ICC_AP0Rn_EL1";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// Physical Fault Address Register (EL2)
#[allow(non_camel_case_types)]
pub struct PFAR_EL2;
impl Register for PFAR_EL2 {
	const NAME: &'static str = "PFAR_EL2";
	const LEN: usize = 64;
}

/// PFAR_EL2 register fields
pub mod pfar_el2 {
	use crate::core::model::proc::RegisterView;

	/// PFAR_EL2\[63\]
	pub struct NS;
	impl RegisterView for NS {
		type Register = super::PFAR_EL2;
		const NAME: &'static str = "NS";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// PFAR_EL2\[62\]
	pub struct NSE;
	impl RegisterView for NSE {
		type Register = super::PFAR_EL2;
		const NAME: &'static str = "NSE";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// PFAR_EL2\[55:52\]
	#[allow(non_camel_case_types)]
	pub struct PA_55_52;
	impl RegisterView for PA_55_52 {
		type Register = super::PFAR_EL2;
		const NAME: &'static str = "PA_55_52";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}
	/// PFAR_EL2\[51:48\]
	#[allow(non_camel_case_types)]
	pub struct PA_55_52_51_48;
	impl RegisterView for PA_55_52_51_48 {
		type Register = super::PFAR_EL2;
		const NAME: &'static str = "PA_55_52_51_48";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}
}

/// Debug Data Transfer Register, half-duplex
#[allow(non_camel_case_types)]
pub struct DBGDTR_EL0;
impl Register for DBGDTR_EL0 {
	const NAME: &'static str = "DBGDTR_EL0";
	const LEN: usize = 64;
}

/// DBGDTR_EL0 register fields
pub mod dbgdtr_el0 {
	use crate::core::model::proc::RegisterView;

	/// DBGDTR_EL0\[63:32\]
	pub struct HighWord;
	impl RegisterView for HighWord {
		type Register = super::DBGDTR_EL0;
		const NAME: &'static str = "HighWord";
		const OFFSET: usize = 32;
		const LEN: usize = 32;
	}

	/// DBGDTR_EL0\[31:0\]
	pub struct LowWord;
	impl RegisterView for LowWord {
		type Register = super::DBGDTR_EL0;
		const NAME: &'static str = "LowWord";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Interrupt Controller Interrupt Group 1 Enable Register
#[allow(non_camel_case_types)]
pub struct ICC_IGRPEN1_EL1;
impl Register for ICC_IGRPEN1_EL1 {
	const NAME: &'static str = "ICC_IGRPEN1_EL1";
	const LEN: usize = 64;
}

/// ICC_IGRPEN1_EL1 register fields
pub mod icc_igrpen1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_IGRPEN1_EL1\[0\]
	pub struct Enable;
	impl RegisterView for Enable {
		type Register = super::ICC_IGRPEN1_EL1;
		const NAME: &'static str = "Enable";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Counter-timer Physical Offset Register
#[allow(non_camel_case_types)]
pub struct CNTPOFF_EL2;
impl Register for CNTPOFF_EL2 {
	const NAME: &'static str = "CNTPOFF_EL2";
	const LEN: usize = 64;
}

/// Trace IMP DEF Register 0
pub struct TRCIMSPEC0;
impl Register for TRCIMSPEC0 {
	const NAME: &'static str = "TRCIMSPEC0";
	const LEN: usize = 64;
}

/// TRCIMSPEC0 register fields
pub mod trcimspec0 {
	use crate::core::model::proc::RegisterView;

	/// TRCIMSPEC0\[7:4\]
	pub struct EN;
	impl RegisterView for EN {
		type Register = super::TRCIMSPEC0;
		const NAME: &'static str = "EN";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// TRCIMSPEC0\[3:0\]
	pub struct SUPPORT;
	impl RegisterView for SUPPORT {
		type Register = super::TRCIMSPEC0;
		const NAME: &'static str = "SUPPORT";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Hypervisor Fine-Grained Read Trap Register
#[allow(non_camel_case_types)]
pub struct HFGRTR_EL2;
impl Register for HFGRTR_EL2 {
	const NAME: &'static str = "HFGRTR_EL2";
	const LEN: usize = 64;
}

/// HFGRTR_EL2 register fields
pub mod hfgrtr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HFGRTR_EL2\[63\]
	#[allow(non_camel_case_types)]
	pub struct nAMAIR2_EL1;
	impl RegisterView for nAMAIR2_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nAMAIR2_EL1";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[62\]
	#[allow(non_camel_case_types)]
	pub struct nMAIR2_EL1;
	impl RegisterView for nMAIR2_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nMAIR2_EL1";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[61\]
	#[allow(non_camel_case_types)]
	pub struct nS2POR_EL1;
	impl RegisterView for nS2POR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nS2POR_EL1";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[60\]
	#[allow(non_camel_case_types)]
	pub struct nPOR_EL1;
	impl RegisterView for nPOR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nPOR_EL1";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[59\]
	#[allow(non_camel_case_types)]
	pub struct nPOR_EL0;
	impl RegisterView for nPOR_EL0 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nPOR_EL0";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[58\]
	#[allow(non_camel_case_types)]
	pub struct nPIR_EL1;
	impl RegisterView for nPIR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nPIR_EL1";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[57\]
	#[allow(non_camel_case_types)]
	pub struct nPIRE0_EL1;
	impl RegisterView for nPIRE0_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nPIRE0_EL1";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[56\]
	#[allow(non_camel_case_types)]
	pub struct nRCWMASK_EL1;
	impl RegisterView for nRCWMASK_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nRCWMASK_EL1";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[55\]
	#[allow(non_camel_case_types)]
	pub struct nTPIDR2_EL0;
	impl RegisterView for nTPIDR2_EL0 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nTPIDR2_EL0";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[54\]
	#[allow(non_camel_case_types)]
	pub struct nSMPRI_EL1;
	impl RegisterView for nSMPRI_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nSMPRI_EL1";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[53\]
	#[allow(non_camel_case_types)]
	pub struct nGCS_EL1;
	impl RegisterView for nGCS_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nGCS_EL1";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[52\]
	#[allow(non_camel_case_types)]
	pub struct nGCS_EL0;
	impl RegisterView for nGCS_EL0 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nGCS_EL0";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[50\]
	#[allow(non_camel_case_types)]
	pub struct nACCDATA_EL1;
	impl RegisterView for nACCDATA_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "nACCDATA_EL1";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[49\]
	#[allow(non_camel_case_types)]
	pub struct ERXADDR_EL1;
	impl RegisterView for ERXADDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERXADDR_EL1";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[48\]
	#[allow(non_camel_case_types)]
	pub struct ERXPFGCDN_EL1;
	impl RegisterView for ERXPFGCDN_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERXPFGCDN_EL1";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[47\]
	#[allow(non_camel_case_types)]
	pub struct ERXPFGCTL_EL1;
	impl RegisterView for ERXPFGCTL_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERXPFGCTL_EL1";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[46\]
	#[allow(non_camel_case_types)]
	pub struct ERXPFGF_EL1;
	impl RegisterView for ERXPFGF_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERXPFGF_EL1";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[45\]
	#[allow(non_camel_case_types)]
	pub struct ERXMISCn_EL1;
	impl RegisterView for ERXMISCn_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERXMISCn_EL1";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[44\]
	#[allow(non_camel_case_types)]
	pub struct ERXSTATUS_EL1;
	impl RegisterView for ERXSTATUS_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERXSTATUS_EL1";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[43\]
	#[allow(non_camel_case_types)]
	pub struct ERXCTLR_EL1;
	impl RegisterView for ERXCTLR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERXCTLR_EL1";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[42\]
	#[allow(non_camel_case_types)]
	pub struct ERXFR_EL1;
	impl RegisterView for ERXFR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERXFR_EL1";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[41\]
	#[allow(non_camel_case_types)]
	pub struct ERRSELR_EL1;
	impl RegisterView for ERRSELR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERRSELR_EL1";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[40\]
	#[allow(non_camel_case_types)]
	pub struct ERRIDR_EL1;
	impl RegisterView for ERRIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ERRIDR_EL1";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[39\]
	#[allow(non_camel_case_types)]
	pub struct ICC_IGRPENn_EL1;
	impl RegisterView for ICC_IGRPENn_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ICC_IGRPENn_EL1";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[38\]
	#[allow(non_camel_case_types)]
	pub struct VBAR_EL1;
	impl RegisterView for VBAR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "VBAR_EL1";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[37\]
	#[allow(non_camel_case_types)]
	pub struct TTBR1_EL1;
	impl RegisterView for TTBR1_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "TTBR1_EL1";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[36\]
	#[allow(non_camel_case_types)]
	pub struct TTBR0_EL1;
	impl RegisterView for TTBR0_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "TTBR0_EL1";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[35\]
	#[allow(non_camel_case_types)]
	pub struct TPIDR_EL0;
	impl RegisterView for TPIDR_EL0 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "TPIDR_EL0";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[34\]
	#[allow(non_camel_case_types)]
	pub struct TPIDRRO_EL0;
	impl RegisterView for TPIDRRO_EL0 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "TPIDRRO_EL0";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[33\]
	#[allow(non_camel_case_types)]
	pub struct TPIDR_EL1;
	impl RegisterView for TPIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "TPIDR_EL1";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[32\]
	#[allow(non_camel_case_types)]
	pub struct TCR_EL1;
	impl RegisterView for TCR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "TCR_EL1";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[31\]
	#[allow(non_camel_case_types)]
	pub struct SCXTNUM_EL0;
	impl RegisterView for SCXTNUM_EL0 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "SCXTNUM_EL0";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[30\]
	#[allow(non_camel_case_types)]
	pub struct SCXTNUM_EL1;
	impl RegisterView for SCXTNUM_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "SCXTNUM_EL1";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[29\]
	#[allow(non_camel_case_types)]
	pub struct SCTLR_EL1;
	impl RegisterView for SCTLR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "SCTLR_EL1";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[28\]
	#[allow(non_camel_case_types)]
	pub struct REVIDR_EL1;
	impl RegisterView for REVIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "REVIDR_EL1";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[27\]
	#[allow(non_camel_case_types)]
	pub struct PAR_EL1;
	impl RegisterView for PAR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "PAR_EL1";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[26\]
	#[allow(non_camel_case_types)]
	pub struct MPIDR_EL1;
	impl RegisterView for MPIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "MPIDR_EL1";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[25\]
	#[allow(non_camel_case_types)]
	pub struct MIDR_EL1;
	impl RegisterView for MIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "MIDR_EL1";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[24\]
	#[allow(non_camel_case_types)]
	pub struct MAIR_EL1;
	impl RegisterView for MAIR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "MAIR_EL1";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[23\]
	#[allow(non_camel_case_types)]
	pub struct LORSA_EL1;
	impl RegisterView for LORSA_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "LORSA_EL1";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[22\]
	#[allow(non_camel_case_types)]
	pub struct LORN_EL1;
	impl RegisterView for LORN_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "LORN_EL1";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[21\]
	#[allow(non_camel_case_types)]
	pub struct LORID_EL1;
	impl RegisterView for LORID_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "LORID_EL1";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[20\]
	#[allow(non_camel_case_types)]
	pub struct LOREA_EL1;
	impl RegisterView for LOREA_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "LOREA_EL1";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[19\]
	#[allow(non_camel_case_types)]
	pub struct LORC_EL1;
	impl RegisterView for LORC_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "LORC_EL1";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[18\]
	#[allow(non_camel_case_types)]
	pub struct ISR_EL1;
	impl RegisterView for ISR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ISR_EL1";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[17\]
	#[allow(non_camel_case_types)]
	pub struct FAR_EL1;
	impl RegisterView for FAR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "FAR_EL1";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[16\]
	#[allow(non_camel_case_types)]
	pub struct ESR_EL1;
	impl RegisterView for ESR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "ESR_EL1";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[15\]
	#[allow(non_camel_case_types)]
	pub struct DCZID_EL0;
	impl RegisterView for DCZID_EL0 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "DCZID_EL0";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[14\]
	#[allow(non_camel_case_types)]
	pub struct CTR_EL0;
	impl RegisterView for CTR_EL0 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "CTR_EL0";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[13\]
	#[allow(non_camel_case_types)]
	pub struct CSSELR_EL1;
	impl RegisterView for CSSELR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "CSSELR_EL1";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[12\]
	#[allow(non_camel_case_types)]
	pub struct CPACR_EL1;
	impl RegisterView for CPACR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "CPACR_EL1";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[11\]
	#[allow(non_camel_case_types)]
	pub struct CONTEXTIDR_EL1;
	impl RegisterView for CONTEXTIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "CONTEXTIDR_EL1";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[10\]
	#[allow(non_camel_case_types)]
	pub struct CLIDR_EL1;
	impl RegisterView for CLIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "CLIDR_EL1";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[9\]
	#[allow(non_camel_case_types)]
	pub struct CCSIDR_EL1;
	impl RegisterView for CCSIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "CCSIDR_EL1";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[8\]
	pub struct APIBKey;
	impl RegisterView for APIBKey {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "APIBKey";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[7\]
	pub struct APIAKey;
	impl RegisterView for APIAKey {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "APIAKey";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[6\]
	pub struct APGAKey;
	impl RegisterView for APGAKey {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "APGAKey";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[5\]
	pub struct APDBKey;
	impl RegisterView for APDBKey {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "APDBKey";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[4\]
	pub struct APDAKey;
	impl RegisterView for APDAKey {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "APDAKey";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[3\]
	#[allow(non_camel_case_types)]
	pub struct AMAIR_EL1;
	impl RegisterView for AMAIR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "AMAIR_EL1";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[2\]
	#[allow(non_camel_case_types)]
	pub struct AIDR_EL1;
	impl RegisterView for AIDR_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "AIDR_EL1";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[1\]
	#[allow(non_camel_case_types)]
	pub struct AFSR1_EL1;
	impl RegisterView for AFSR1_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "AFSR1_EL1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HFGRTR_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct AFSR0_EL1;
	impl RegisterView for AFSR0_EL1 {
		type Register = super::HFGRTR_EL2;
		const NAME: &'static str = "AFSR0_EL1";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Translation Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct TCR_EL3;
impl Register for TCR_EL3 {
	const NAME: &'static str = "TCR_EL3";
	const LEN: usize = 64;
}

/// TCR_EL3 register fields
pub mod tcr_el3 {
	use crate::core::model::proc::RegisterView;

	/// TCR_EL3\[43\]
	pub struct DisCH0;
	impl RegisterView for DisCH0 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "DisCH0";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[42\]
	pub struct HAFT;
	impl RegisterView for HAFT {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "HAFT";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[41\]
	pub struct PTTWI;
	impl RegisterView for PTTWI {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "PTTWI";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[38\]
	pub struct D128;
	impl RegisterView for D128 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "D128";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[37\]
	pub struct AIE;
	impl RegisterView for AIE {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "AIE";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[36\]
	pub struct POE;
	impl RegisterView for POE {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "POE";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[35\]
	pub struct PIE;
	impl RegisterView for PIE {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "PIE";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[34\]
	pub struct PnCH;
	impl RegisterView for PnCH {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "PnCH";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[33\]
	pub struct MTX;
	impl RegisterView for MTX {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "MTX";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[32\]
	pub struct DS;
	impl RegisterView for DS {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "DS";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[30\]
	pub struct TCMA;
	impl RegisterView for TCMA {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "TCMA";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[29\]
	pub struct TBID;
	impl RegisterView for TBID {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "TBID";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[28\]
	pub struct HWU62;
	impl RegisterView for HWU62 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "HWU62";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[27\]
	pub struct HWU61;
	impl RegisterView for HWU61 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "HWU61";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[26\]
	pub struct HWU60;
	impl RegisterView for HWU60 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "HWU60";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[25\]
	pub struct HWU59;
	impl RegisterView for HWU59 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "HWU59";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[24\]
	pub struct HPD;
	impl RegisterView for HPD {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "HPD";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[22\]
	pub struct HD;
	impl RegisterView for HD {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "HD";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[21\]
	pub struct HA;
	impl RegisterView for HA {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "HA";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[20\]
	pub struct TBI;
	impl RegisterView for TBI {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "TBI";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// TCR_EL3\[18:16\]
	pub struct PS;
	impl RegisterView for PS {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "PS";
		const OFFSET: usize = 16;
		const LEN: usize = 3;
	}

	/// TCR_EL3\[15:14\]
	pub struct TG0;
	impl RegisterView for TG0 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "TG0";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// TCR_EL3\[13:12\]
	pub struct SH0;
	impl RegisterView for SH0 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "SH0";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// TCR_EL3\[11:10\]
	pub struct ORGN0;
	impl RegisterView for ORGN0 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "ORGN0";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// TCR_EL3\[9:8\]
	pub struct IRGN0;
	impl RegisterView for IRGN0 {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "IRGN0";
		const OFFSET: usize = 8;
		const LEN: usize = 2;
	}

	/// TCR_EL3\[5:0\]
	pub struct T0SZ;
	impl RegisterView for T0SZ {
		type Register = super::TCR_EL3;
		const NAME: &'static str = "T0SZ";
		const OFFSET: usize = 0;
		const LEN: usize = 6;
	}
}

/// Floating-point Status Register
pub struct FPSR;
impl Register for FPSR {
	const NAME: &'static str = "FPSR";
	const LEN: usize = 64;
}

/// FPSR register fields
pub mod fpsr {
	use crate::core::model::proc::RegisterView;

	/// FPSR\[31\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::FPSR;
		const NAME: &'static str = "N";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// FPSR\[30\]
	pub struct Z;
	impl RegisterView for Z {
		type Register = super::FPSR;
		const NAME: &'static str = "Z";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// FPSR\[29\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::FPSR;
		const NAME: &'static str = "C";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// FPSR\[28\]
	pub struct V;
	impl RegisterView for V {
		type Register = super::FPSR;
		const NAME: &'static str = "V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// FPSR\[27\]
	pub struct QC;
	impl RegisterView for QC {
		type Register = super::FPSR;
		const NAME: &'static str = "QC";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// FPSR\[7\]
	pub struct IDC;
	impl RegisterView for IDC {
		type Register = super::FPSR;
		const NAME: &'static str = "IDC";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// FPSR\[4\]
	pub struct IXC;
	impl RegisterView for IXC {
		type Register = super::FPSR;
		const NAME: &'static str = "IXC";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// FPSR\[3\]
	pub struct UFC;
	impl RegisterView for UFC {
		type Register = super::FPSR;
		const NAME: &'static str = "UFC";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// FPSR\[2\]
	pub struct OFC;
	impl RegisterView for OFC {
		type Register = super::FPSR;
		const NAME: &'static str = "OFC";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// FPSR\[1\]
	pub struct DZC;
	impl RegisterView for DZC {
		type Register = super::FPSR;
		const NAME: &'static str = "DZC";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// FPSR\[0\]
	pub struct IOC;
	impl RegisterView for IOC {
		type Register = super::FPSR;
		const NAME: &'static str = "IOC";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Selected Pseudo-fault Generation Countdown Register
#[allow(non_camel_case_types)]
pub struct ERXPFGCDN_EL1;
impl Register for ERXPFGCDN_EL1 {
	const NAME: &'static str = "ERXPFGCDN_EL1";
	const LEN: usize = 64;
}

/// Sampling Interval Reload Register
#[allow(non_camel_case_types)]
pub struct PMSIRR_EL1;
impl Register for PMSIRR_EL1 {
	const NAME: &'static str = "PMSIRR_EL1";
	const LEN: usize = 64;
}

/// PMSIRR_EL1 register fields
pub mod pmsirr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSIRR_EL1\[31:8\]
	pub struct INTERVAL;
	impl RegisterView for INTERVAL {
		type Register = super::PMSIRR_EL1;
		const NAME: &'static str = "INTERVAL";
		const OFFSET: usize = 8;
		const LEN: usize = 24;
	}

	/// PMSIRR_EL1\[0\]
	pub struct RND;
	impl RegisterView for RND {
		type Register = super::PMSIRR_EL1;
		const NAME: &'static str = "RND";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Hardware Accelerator for Cleaning Dirty State Consumer Register
#[allow(non_camel_case_types)]
pub struct HACDBSCONS_EL2;
impl Register for HACDBSCONS_EL2 {
	const NAME: &'static str = "HACDBSCONS_EL2";
	const LEN: usize = 64;
}

/// HACDBSCONS_EL2 register fields
pub mod hacdbscons_el2 {
	use crate::core::model::proc::RegisterView;

	/// HACDBSCONS_EL2\[63:62\]
	#[allow(non_camel_case_types)]
	pub struct ERR_REASON;
	impl RegisterView for ERR_REASON {
		type Register = super::HACDBSCONS_EL2;
		const NAME: &'static str = "ERR_REASON";
		const OFFSET: usize = 62;
		const LEN: usize = 2;
	}

	/// HACDBSCONS_EL2\[18:0\]
	pub struct INDEX;
	impl RegisterView for INDEX {
		type Register = super::HACDBSCONS_EL2;
		const NAME: &'static str = "INDEX";
		const OFFSET: usize = 0;
		const LEN: usize = 19;
	}
}

/// Counter-timer Virtual Timer CompareValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHV_CVAL_EL2;
impl Register for CNTHV_CVAL_EL2 {
	const NAME: &'static str = "CNTHV_CVAL_EL2";
	const LEN: usize = 64;
}

/// CNTHV_CVAL_EL2 register fields
pub mod cnthv_cval_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHV_CVAL_EL2\[63:0\]
	pub struct CompareValue;
	impl RegisterView for CompareValue {
		type Register = super::CNTHV_CVAL_EL2;
		const NAME: &'static str = "CompareValue";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Translation Table Base Register 0 (EL2)
#[allow(non_camel_case_types)]
pub struct TTBR0_EL2;
impl Register for TTBR0_EL2 {
	const NAME: &'static str = "TTBR0_EL2";
	const LEN: usize = 64;
}

/// TTBR0_EL2 register fields
pub mod ttbr0_el2 {
	use crate::core::model::proc::RegisterView;

	/// TTBR0_EL2\[63:48\]
	pub struct ASID;
	impl RegisterView for ASID {
		type Register = super::TTBR0_EL2;
		const NAME: &'static str = "ASID";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// TTBR0_EL2\[2:1\]
	pub struct SKL;
	impl RegisterView for SKL {
		type Register = super::TTBR0_EL2;
		const NAME: &'static str = "SKL";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// TTBR0_EL2\[0\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::TTBR0_EL2;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}

	/// TTBR0_EL2\[87:80\]
	#[allow(non_camel_case_types)]
	pub struct BADDR_87_80;
	impl RegisterView for BADDR_87_80 {
		type Register = super::TTBR0_EL2;
		const NAME: &'static str = "BADDR_87_80";
		const OFFSET: usize = 80;
		const LEN: usize = 8;
	}
	/// TTBR0_EL2\[47:1\]
	#[allow(non_camel_case_types)]
	pub struct BADDR_87_80_47_1;
	impl RegisterView for BADDR_87_80_47_1 {
		type Register = super::TTBR0_EL2;
		const NAME: &'static str = "BADDR_87_80_47_1";
		const OFFSET: usize = 1;
		const LEN: usize = 47;
	}
}

/// LORegion Start Address (EL1)
#[allow(non_camel_case_types)]
pub struct LORSA_EL1;
impl Register for LORSA_EL1 {
	const NAME: &'static str = "LORSA_EL1";
	const LEN: usize = 64;
}

/// LORSA_EL1 register fields
pub mod lorsa_el1 {
	use crate::core::model::proc::RegisterView;

	/// LORSA_EL1\[55:16\]
	pub struct SA;
	impl RegisterView for SA {
		type Register = super::LORSA_EL1;
		const NAME: &'static str = "SA";
		const OFFSET: usize = 16;
		const LEN: usize = 40;
	}

	/// LORSA_EL1\[0\]
	pub struct Valid;
	impl RegisterView for Valid {
		type Register = super::LORSA_EL1;
		const NAME: &'static str = "Valid";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct ICC_CTLR_EL3;
impl Register for ICC_CTLR_EL3 {
	const NAME: &'static str = "ICC_CTLR_EL3";
	const LEN: usize = 64;
}

/// ICC_CTLR_EL3 register fields
pub mod icc_ctlr_el3 {
	use crate::core::model::proc::RegisterView;

	/// ICC_CTLR_EL3\[19\]
	pub struct ExtRange;
	impl RegisterView for ExtRange {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "ExtRange";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[18\]
	pub struct RSS;
	impl RegisterView for RSS {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "RSS";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[17\]
	#[allow(non_camel_case_types)]
	pub struct nDS;
	impl RegisterView for nDS {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "nDS";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[15\]
	pub struct A3V;
	impl RegisterView for A3V {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "A3V";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[14\]
	pub struct SEIS;
	impl RegisterView for SEIS {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "SEIS";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[13:11\]
	pub struct IDbits;
	impl RegisterView for IDbits {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "IDbits";
		const OFFSET: usize = 11;
		const LEN: usize = 3;
	}

	/// ICC_CTLR_EL3\[10:8\]
	pub struct PRIbits;
	impl RegisterView for PRIbits {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "PRIbits";
		const OFFSET: usize = 8;
		const LEN: usize = 3;
	}

	/// ICC_CTLR_EL3\[6\]
	pub struct PMHE;
	impl RegisterView for PMHE {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "PMHE";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[5\]
	pub struct RM;
	impl RegisterView for RM {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "RM";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[4\]
	#[allow(non_camel_case_types)]
	pub struct EOImode_EL1NS;
	impl RegisterView for EOImode_EL1NS {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "EOImode_EL1NS";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[3\]
	#[allow(non_camel_case_types)]
	pub struct EOImode_EL1S;
	impl RegisterView for EOImode_EL1S {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "EOImode_EL1S";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[2\]
	#[allow(non_camel_case_types)]
	pub struct EOImode_EL3;
	impl RegisterView for EOImode_EL3 {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "EOImode_EL3";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[1\]
	#[allow(non_camel_case_types)]
	pub struct CBPR_EL1NS;
	impl RegisterView for CBPR_EL1NS {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "CBPR_EL1NS";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL3\[0\]
	#[allow(non_camel_case_types)]
	pub struct CBPR_EL1S;
	impl RegisterView for CBPR_EL1S {
		type Register = super::ICC_CTLR_EL3;
		const NAME: &'static str = "CBPR_EL1S";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Filter Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct TRFCR_EL1;
impl Register for TRFCR_EL1 {
	const NAME: &'static str = "TRFCR_EL1";
	const LEN: usize = 64;
}

/// TRFCR_EL1 register fields
pub mod trfcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRFCR_EL1\[6:5\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::TRFCR_EL1;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 5;
		const LEN: usize = 2;
	}

	/// TRFCR_EL1\[1\]
	pub struct E1TRE;
	impl RegisterView for E1TRE {
		type Register = super::TRFCR_EL1;
		const NAME: &'static str = "E1TRE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TRFCR_EL1\[0\]
	pub struct E0TRE;
	impl RegisterView for E0TRE {
		type Register = super::TRFCR_EL1;
		const NAME: &'static str = "E0TRE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Extended Auxiliary Memory Attribute Indirection Register (EL2)
#[allow(non_camel_case_types)]
pub struct AMAIR2_EL2;
impl Register for AMAIR2_EL2 {
	const NAME: &'static str = "AMAIR2_EL2";
	const LEN: usize = 64;
}

/// Branch Record Buffer ID0 Register
#[allow(non_camel_case_types)]
pub struct BRBIDR0_EL1;
impl Register for BRBIDR0_EL1 {
	const NAME: &'static str = "BRBIDR0_EL1";
	const LEN: usize = 64;
}

/// BRBIDR0_EL1 register fields
pub mod brbidr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// BRBIDR0_EL1\[15:12\]
	pub struct CC;
	impl RegisterView for CC {
		type Register = super::BRBIDR0_EL1;
		const NAME: &'static str = "CC";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// BRBIDR0_EL1\[11:8\]
	pub struct FORMAT;
	impl RegisterView for FORMAT {
		type Register = super::BRBIDR0_EL1;
		const NAME: &'static str = "FORMAT";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// BRBIDR0_EL1\[7:0\]
	pub struct NUMREC;
	impl RegisterView for NUMREC {
		type Register = super::BRBIDR0_EL1;
		const NAME: &'static str = "NUMREC";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Sampling Interval Counter Register
#[allow(non_camel_case_types)]
pub struct PMSICR_EL1;
impl Register for PMSICR_EL1 {
	const NAME: &'static str = "PMSICR_EL1";
	const LEN: usize = 64;
}

/// PMSICR_EL1 register fields
pub mod pmsicr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSICR_EL1\[63:56\]
	pub struct ECOUNT;
	impl RegisterView for ECOUNT {
		type Register = super::PMSICR_EL1;
		const NAME: &'static str = "ECOUNT";
		const OFFSET: usize = 56;
		const LEN: usize = 8;
	}

	/// PMSICR_EL1\[31:0\]
	pub struct COUNT;
	impl RegisterView for COUNT {
		type Register = super::PMSICR_EL1;
		const NAME: &'static str = "COUNT";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// System Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct SCTLR_EL1;
impl Register for SCTLR_EL1 {
	const NAME: &'static str = "SCTLR_EL1";
	const LEN: usize = 64;
}

/// SCTLR_EL1 register fields
pub mod sctlr_el1 {
	use crate::core::model::proc::RegisterView;

	/// SCTLR_EL1\[63\]
	pub struct TIDCP;
	impl RegisterView for TIDCP {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TIDCP";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[62\]
	pub struct SPINTMASK;
	impl RegisterView for SPINTMASK {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "SPINTMASK";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[61\]
	pub struct NMI;
	impl RegisterView for NMI {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[60\]
	pub struct EnTP2;
	impl RegisterView for EnTP2 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnTP2";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[59\]
	pub struct TCSO;
	impl RegisterView for TCSO {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TCSO";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[58\]
	pub struct TCSO0;
	impl RegisterView for TCSO0 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TCSO0";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[57\]
	pub struct EPAN;
	impl RegisterView for EPAN {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EPAN";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[56\]
	pub struct EnALS;
	impl RegisterView for EnALS {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnALS";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[55\]
	pub struct EnAS0;
	impl RegisterView for EnAS0 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnAS0";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[54\]
	pub struct EnASR;
	impl RegisterView for EnASR {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnASR";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[53\]
	pub struct TME;
	impl RegisterView for TME {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TME";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[52\]
	pub struct TME0;
	impl RegisterView for TME0 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TME0";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[51\]
	pub struct TMT;
	impl RegisterView for TMT {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TMT";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[50\]
	pub struct TMT0;
	impl RegisterView for TMT0 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TMT0";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[49:46\]
	pub struct TWEDEL;
	impl RegisterView for TWEDEL {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TWEDEL";
		const OFFSET: usize = 46;
		const LEN: usize = 4;
	}

	/// SCTLR_EL1\[45\]
	pub struct TWEDEn;
	impl RegisterView for TWEDEn {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TWEDEn";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[44\]
	pub struct DSSBS;
	impl RegisterView for DSSBS {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "DSSBS";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[43\]
	pub struct ATA;
	impl RegisterView for ATA {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "ATA";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[42\]
	pub struct ATA0;
	impl RegisterView for ATA0 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "ATA0";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[41:40\]
	pub struct TCF;
	impl RegisterView for TCF {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TCF";
		const OFFSET: usize = 40;
		const LEN: usize = 2;
	}

	/// SCTLR_EL1\[39:38\]
	pub struct TCF0;
	impl RegisterView for TCF0 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TCF0";
		const OFFSET: usize = 38;
		const LEN: usize = 2;
	}

	/// SCTLR_EL1\[37\]
	pub struct ITFSB;
	impl RegisterView for ITFSB {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "ITFSB";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[36\]
	pub struct BT1;
	impl RegisterView for BT1 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "BT1";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[35\]
	pub struct BT0;
	impl RegisterView for BT0 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "BT0";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[34\]
	pub struct EnFPM;
	impl RegisterView for EnFPM {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnFPM";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[33\]
	pub struct MSCEn;
	impl RegisterView for MSCEn {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "MSCEn";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[32\]
	pub struct CMOW;
	impl RegisterView for CMOW {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "CMOW";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[31\]
	pub struct EnIA;
	impl RegisterView for EnIA {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnIA";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[30\]
	pub struct EnIB;
	impl RegisterView for EnIB {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnIB";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[29\]
	pub struct LSMAOE;
	impl RegisterView for LSMAOE {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "LSMAOE";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[28\]
	#[allow(non_camel_case_types)]
	pub struct nTLSMD;
	impl RegisterView for nTLSMD {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "nTLSMD";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[27\]
	pub struct EnDA;
	impl RegisterView for EnDA {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnDA";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[26\]
	pub struct UCI;
	impl RegisterView for UCI {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "UCI";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[25\]
	pub struct EE;
	impl RegisterView for EE {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EE";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[24\]
	pub struct E0E;
	impl RegisterView for E0E {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "E0E";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[23\]
	pub struct SPAN;
	impl RegisterView for SPAN {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "SPAN";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[22\]
	pub struct EIS;
	impl RegisterView for EIS {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EIS";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[21\]
	pub struct IESB;
	impl RegisterView for IESB {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "IESB";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[20\]
	pub struct TSCXT;
	impl RegisterView for TSCXT {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "TSCXT";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[19\]
	pub struct WXN;
	impl RegisterView for WXN {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "WXN";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[18\]
	#[allow(non_camel_case_types)]
	pub struct nTWE;
	impl RegisterView for nTWE {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "nTWE";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[16\]
	#[allow(non_camel_case_types)]
	pub struct nTWI;
	impl RegisterView for nTWI {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "nTWI";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[15\]
	pub struct UCT;
	impl RegisterView for UCT {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "UCT";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[14\]
	pub struct DZE;
	impl RegisterView for DZE {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "DZE";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[13\]
	pub struct EnDB;
	impl RegisterView for EnDB {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnDB";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[12\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "I";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[11\]
	pub struct EOS;
	impl RegisterView for EOS {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EOS";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[10\]
	pub struct EnRCTX;
	impl RegisterView for EnRCTX {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "EnRCTX";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[9\]
	pub struct UMA;
	impl RegisterView for UMA {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "UMA";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[8\]
	pub struct SED;
	impl RegisterView for SED {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "SED";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[7\]
	pub struct ITD;
	impl RegisterView for ITD {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "ITD";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[6\]
	#[allow(non_camel_case_types)]
	pub struct nAA;
	impl RegisterView for nAA {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "nAA";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[5\]
	pub struct CP15BEN;
	impl RegisterView for CP15BEN {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "CP15BEN";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[4\]
	pub struct SA0;
	impl RegisterView for SA0 {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "SA0";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[3\]
	pub struct SA;
	impl RegisterView for SA {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "SA";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[2\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "C";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[1\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "A";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// SCTLR_EL1\[0\]
	pub struct M;
	impl RegisterView for M {
		type Register = super::SCTLR_EL1;
		const NAME: &'static str = "M";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// MPAM1 Register (EL1)
#[allow(non_camel_case_types)]
pub struct MPAM1_EL1;
impl Register for MPAM1_EL1 {
	const NAME: &'static str = "MPAM1_EL1";
	const LEN: usize = 64;
}

/// MPAM1_EL1 register fields
pub mod mpam1_el1 {
	use crate::core::model::proc::RegisterView;

	/// MPAM1_EL1\[63\]
	pub struct MPAMEN;
	impl RegisterView for MPAMEN {
		type Register = super::MPAM1_EL1;
		const NAME: &'static str = "MPAMEN";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// MPAM1_EL1\[60\]
	#[allow(non_camel_case_types)]
	pub struct FORCED_NS;
	impl RegisterView for FORCED_NS {
		type Register = super::MPAM1_EL1;
		const NAME: &'static str = "FORCED_NS";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// MPAM1_EL1\[54\]
	#[allow(non_camel_case_types)]
	pub struct ALTSP_FRCD;
	impl RegisterView for ALTSP_FRCD {
		type Register = super::MPAM1_EL1;
		const NAME: &'static str = "ALTSP_FRCD";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// MPAM1_EL1\[47:40\]
	#[allow(non_camel_case_types)]
	pub struct PMG_D;
	impl RegisterView for PMG_D {
		type Register = super::MPAM1_EL1;
		const NAME: &'static str = "PMG_D";
		const OFFSET: usize = 40;
		const LEN: usize = 8;
	}

	/// MPAM1_EL1\[39:32\]
	#[allow(non_camel_case_types)]
	pub struct PMG_I;
	impl RegisterView for PMG_I {
		type Register = super::MPAM1_EL1;
		const NAME: &'static str = "PMG_I";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// MPAM1_EL1\[31:16\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_D;
	impl RegisterView for PARTID_D {
		type Register = super::MPAM1_EL1;
		const NAME: &'static str = "PARTID_D";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAM1_EL1\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_I;
	impl RegisterView for PARTID_I {
		type Register = super::MPAM1_EL1;
		const NAME: &'static str = "PARTID_I";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Interrupt Controller Hyp Control Register
#[allow(non_camel_case_types)]
pub struct ICH_HCR_EL2;
impl Register for ICH_HCR_EL2 {
	const NAME: &'static str = "ICH_HCR_EL2";
	const LEN: usize = 64;
}

/// ICH_HCR_EL2 register fields
pub mod ich_hcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// ICH_HCR_EL2\[31:27\]
	pub struct EOIcount;
	impl RegisterView for EOIcount {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "EOIcount";
		const OFFSET: usize = 27;
		const LEN: usize = 5;
	}

	/// ICH_HCR_EL2\[15\]
	pub struct DVIM;
	impl RegisterView for DVIM {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "DVIM";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[14\]
	pub struct TDIR;
	impl RegisterView for TDIR {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "TDIR";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[13\]
	pub struct TSEI;
	impl RegisterView for TSEI {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "TSEI";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[12\]
	pub struct TALL1;
	impl RegisterView for TALL1 {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "TALL1";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[11\]
	pub struct TALL0;
	impl RegisterView for TALL0 {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "TALL0";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[10\]
	pub struct TC;
	impl RegisterView for TC {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "TC";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[8\]
	#[allow(non_camel_case_types)]
	pub struct vSGIEOICount;
	impl RegisterView for vSGIEOICount {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "vSGIEOICount";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[7\]
	pub struct VGrp1DIE;
	impl RegisterView for VGrp1DIE {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "VGrp1DIE";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[6\]
	pub struct VGrp1EIE;
	impl RegisterView for VGrp1EIE {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "VGrp1EIE";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[5\]
	pub struct VGrp0DIE;
	impl RegisterView for VGrp0DIE {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "VGrp0DIE";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[4\]
	pub struct VGrp0EIE;
	impl RegisterView for VGrp0EIE {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "VGrp0EIE";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[3\]
	pub struct NPIE;
	impl RegisterView for NPIE {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "NPIE";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[2\]
	pub struct LRENPIE;
	impl RegisterView for LRENPIE {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "LRENPIE";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[1\]
	pub struct UIE;
	impl RegisterView for UIE {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "UIE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICH_HCR_EL2\[0\]
	pub struct En;
	impl RegisterView for En {
		type Register = super::ICH_HCR_EL2;
		const NAME: &'static str = "En";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Instruction Fault Status Register (EL2)
#[allow(non_camel_case_types)]
pub struct IFSR32_EL2;
impl Register for IFSR32_EL2 {
	const NAME: &'static str = "IFSR32_EL2";
	const LEN: usize = 64;
}

/// IFSR32_EL2 register fields
pub mod ifsr32_el2 {
	use crate::core::model::proc::RegisterView;

	/// IFSR32_EL2\[16\]
	pub struct FnV;
	impl RegisterView for FnV {
		type Register = super::IFSR32_EL2;
		const NAME: &'static str = "FnV";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// IFSR32_EL2\[12\]
	pub struct ExT;
	impl RegisterView for ExT {
		type Register = super::IFSR32_EL2;
		const NAME: &'static str = "ExT";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// IFSR32_EL2\[3:0\]
	pub struct FS;
	impl RegisterView for FS {
		type Register = super::IFSR32_EL2;
		const NAME: &'static str = "FS";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}

	/// IFSR32_EL2\[9\]
	pub struct LPAE;
	impl RegisterView for LPAE {
		type Register = super::IFSR32_EL2;
		const NAME: &'static str = "LPAE";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// IFSR32_EL2\[5:0\]
	pub struct STATUS;
	impl RegisterView for STATUS {
		type Register = super::IFSR32_EL2;
		const NAME: &'static str = "STATUS";
		const OFFSET: usize = 0;
		const LEN: usize = 6;
	}
}

/// Pointer Authentication Key A for Instruction (bits\[127:64\]) 
#[allow(non_camel_case_types)]
pub struct APIAKeyHi_EL1;
impl Register for APIAKeyHi_EL1 {
	const NAME: &'static str = "APIAKeyHi_EL1";
	const LEN: usize = 64;
}

/// Architectural Feature Trap Register (EL3)
#[allow(non_camel_case_types)]
pub struct CPTR_EL3;
impl Register for CPTR_EL3 {
	const NAME: &'static str = "CPTR_EL3";
	const LEN: usize = 64;
}

/// CPTR_EL3 register fields
pub mod cptr_el3 {
	use crate::core::model::proc::RegisterView;

	/// CPTR_EL3\[31\]
	pub struct TCPAC;
	impl RegisterView for TCPAC {
		type Register = super::CPTR_EL3;
		const NAME: &'static str = "TCPAC";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// CPTR_EL3\[30\]
	pub struct TAM;
	impl RegisterView for TAM {
		type Register = super::CPTR_EL3;
		const NAME: &'static str = "TAM";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// CPTR_EL3\[20\]
	pub struct TTA;
	impl RegisterView for TTA {
		type Register = super::CPTR_EL3;
		const NAME: &'static str = "TTA";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// CPTR_EL3\[12\]
	pub struct ESM;
	impl RegisterView for ESM {
		type Register = super::CPTR_EL3;
		const NAME: &'static str = "ESM";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// CPTR_EL3\[10\]
	pub struct TFP;
	impl RegisterView for TFP {
		type Register = super::CPTR_EL3;
		const NAME: &'static str = "TFP";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// CPTR_EL3\[8\]
	pub struct EZ;
	impl RegisterView for EZ {
		type Register = super::CPTR_EL3;
		const NAME: &'static str = "EZ";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}
}

/// System Performance Monitors Device Architecture Register
#[allow(non_camel_case_types)]
pub struct SPMDEVARCH_EL1;
impl Register for SPMDEVARCH_EL1 {
	const NAME: &'static str = "SPMDEVARCH_EL1";
	const LEN: usize = 64;
}

/// SPMDEVARCH_EL1 register fields
pub mod spmdevarch_el1 {
	use crate::core::model::proc::RegisterView;

	/// SPMDEVARCH_EL1\[31:21\]
	pub struct ARCHITECT;
	impl RegisterView for ARCHITECT {
		type Register = super::SPMDEVARCH_EL1;
		const NAME: &'static str = "ARCHITECT";
		const OFFSET: usize = 21;
		const LEN: usize = 11;
	}

	/// SPMDEVARCH_EL1\[20\]
	pub struct PRESENT;
	impl RegisterView for PRESENT {
		type Register = super::SPMDEVARCH_EL1;
		const NAME: &'static str = "PRESENT";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SPMDEVARCH_EL1\[19:16\]
	pub struct REVISION;
	impl RegisterView for REVISION {
		type Register = super::SPMDEVARCH_EL1;
		const NAME: &'static str = "REVISION";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// SPMDEVARCH_EL1\[15:12\]
	pub struct ARCHVER;
	impl RegisterView for ARCHVER {
		type Register = super::SPMDEVARCH_EL1;
		const NAME: &'static str = "ARCHVER";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// SPMDEVARCH_EL1\[11:0\]
	pub struct ARCHPART;
	impl RegisterView for ARCHPART {
		type Register = super::SPMDEVARCH_EL1;
		const NAME: &'static str = "ARCHPART";
		const OFFSET: usize = 0;
		const LEN: usize = 12;
	}
}

/// Interrupt Controller Virtual Machine Control Register
#[allow(non_camel_case_types)]
pub struct ICH_VMCR_EL2;
impl Register for ICH_VMCR_EL2 {
	const NAME: &'static str = "ICH_VMCR_EL2";
	const LEN: usize = 64;
}

/// ICH_VMCR_EL2 register fields
pub mod ich_vmcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// ICH_VMCR_EL2\[31:24\]
	pub struct VPMR;
	impl RegisterView for VPMR {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VPMR";
		const OFFSET: usize = 24;
		const LEN: usize = 8;
	}

	/// ICH_VMCR_EL2\[23:21\]
	pub struct VBPR0;
	impl RegisterView for VBPR0 {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VBPR0";
		const OFFSET: usize = 21;
		const LEN: usize = 3;
	}

	/// ICH_VMCR_EL2\[20:18\]
	pub struct VBPR1;
	impl RegisterView for VBPR1 {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VBPR1";
		const OFFSET: usize = 18;
		const LEN: usize = 3;
	}

	/// ICH_VMCR_EL2\[9\]
	pub struct VEOIM;
	impl RegisterView for VEOIM {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VEOIM";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// ICH_VMCR_EL2\[4\]
	pub struct VCBPR;
	impl RegisterView for VCBPR {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VCBPR";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// ICH_VMCR_EL2\[3\]
	pub struct VFIQEn;
	impl RegisterView for VFIQEn {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VFIQEn";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// ICH_VMCR_EL2\[2\]
	pub struct VAckCtl;
	impl RegisterView for VAckCtl {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VAckCtl";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// ICH_VMCR_EL2\[1\]
	pub struct VENG1;
	impl RegisterView for VENG1 {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VENG1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICH_VMCR_EL2\[0\]
	pub struct VENG0;
	impl RegisterView for VENG0 {
		type Register = super::ICH_VMCR_EL2;
		const NAME: &'static str = "VENG0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Q Element Control Register
pub struct TRCQCTLR;
impl Register for TRCQCTLR {
	const NAME: &'static str = "TRCQCTLR";
	const LEN: usize = 64;
}

/// TRCQCTLR register fields
pub mod trcqctlr {
	use crate::core::model::proc::RegisterView;

	/// TRCQCTLR\[8\]
	pub struct MODE;
	impl RegisterView for MODE {
		type Register = super::TRCQCTLR;
		const NAME: &'static str = "MODE";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Machine Identification Register
#[allow(non_camel_case_types)]
pub struct PMMIR_EL1;
impl Register for PMMIR_EL1 {
	const NAME: &'static str = "PMMIR_EL1";
	const LEN: usize = 64;
}

/// PMMIR_EL1 register fields
pub mod pmmir_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMMIR_EL1\[28\]
	pub struct SME;
	impl RegisterView for SME {
		type Register = super::PMMIR_EL1;
		const NAME: &'static str = "SME";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// PMMIR_EL1\[27:24\]
	pub struct EDGE;
	impl RegisterView for EDGE {
		type Register = super::PMMIR_EL1;
		const NAME: &'static str = "EDGE";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// PMMIR_EL1\[23:20\]
	pub struct THWIDTH;
	impl RegisterView for THWIDTH {
		type Register = super::PMMIR_EL1;
		const NAME: &'static str = "THWIDTH";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// PMMIR_EL1\[19:16\]
	#[allow(non_camel_case_types)]
	pub struct BUS_WIDTH;
	impl RegisterView for BUS_WIDTH {
		type Register = super::PMMIR_EL1;
		const NAME: &'static str = "BUS_WIDTH";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// PMMIR_EL1\[15:8\]
	#[allow(non_camel_case_types)]
	pub struct BUS_SLOTS;
	impl RegisterView for BUS_SLOTS {
		type Register = super::PMMIR_EL1;
		const NAME: &'static str = "BUS_SLOTS";
		const OFFSET: usize = 8;
		const LEN: usize = 8;
	}

	/// PMMIR_EL1\[7:0\]
	pub struct SLOTS;
	impl RegisterView for SLOTS {
		type Register = super::PMMIR_EL1;
		const NAME: &'static str = "SLOTS";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Selected Error Record Feature Register
#[allow(non_camel_case_types)]
pub struct ERXFR_EL1;
impl Register for ERXFR_EL1 {
	const NAME: &'static str = "ERXFR_EL1";
	const LEN: usize = 64;
}

/// Architectural Feature Trap Register (EL2)
#[allow(non_camel_case_types)]
pub struct CPTR_EL2;
impl Register for CPTR_EL2 {
	const NAME: &'static str = "CPTR_EL2";
	const LEN: usize = 64;
}

/// CPTR_EL2 register fields
pub mod cptr_el2 {
	use crate::core::model::proc::RegisterView;

	/// CPTR_EL2\[31\]
	pub struct TCPAC;
	impl RegisterView for TCPAC {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "TCPAC";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// CPTR_EL2\[30\]
	pub struct TAM;
	impl RegisterView for TAM {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "TAM";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// CPTR_EL2\[29\]
	pub struct E0POE;
	impl RegisterView for E0POE {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "E0POE";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// CPTR_EL2\[20\]
	pub struct TTA;
	impl RegisterView for TTA {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "TTA";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// CPTR_EL2\[25:24\]
	pub struct SMEN;
	impl RegisterView for SMEN {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "SMEN";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// CPTR_EL2\[21:20\]
	pub struct FPEN;
	impl RegisterView for FPEN {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "FPEN";
		const OFFSET: usize = 20;
		const LEN: usize = 2;
	}

	/// CPTR_EL2\[17:16\]
	pub struct ZEN;
	impl RegisterView for ZEN {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "ZEN";
		const OFFSET: usize = 16;
		const LEN: usize = 2;
	}

	/// CPTR_EL2\[12\]
	pub struct TSM;
	impl RegisterView for TSM {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "TSM";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// CPTR_EL2\[10\]
	pub struct TFP;
	impl RegisterView for TFP {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "TFP";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// CPTR_EL2\[8\]
	pub struct TZ;
	impl RegisterView for TZ {
		type Register = super::CPTR_EL2;
		const NAME: &'static str = "TZ";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}
}

/// Counter-timer Secure Physical Timer CompareValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHPS_CVAL_EL2;
impl Register for CNTHPS_CVAL_EL2 {
	const NAME: &'static str = "CNTHPS_CVAL_EL2";
	const LEN: usize = 64;
}

/// CNTHPS_CVAL_EL2 register fields
pub mod cnthps_cval_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHPS_CVAL_EL2\[63:0\]
	pub struct CompareValue;
	impl RegisterView for CompareValue {
		type Register = super::CNTHPS_CVAL_EL2;
		const NAME: &'static str = "CompareValue";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Virtualization Secure Translation Table Base Register
#[allow(non_camel_case_types)]
pub struct VSTTBR_EL2;
impl Register for VSTTBR_EL2 {
	const NAME: &'static str = "VSTTBR_EL2";
	const LEN: usize = 64;
}

/// VSTTBR_EL2 register fields
pub mod vsttbr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VSTTBR_EL2\[47:1\]
	pub struct BADDR;
	impl RegisterView for BADDR {
		type Register = super::VSTTBR_EL2;
		const NAME: &'static str = "BADDR";
		const OFFSET: usize = 1;
		const LEN: usize = 47;
	}

	/// VSTTBR_EL2\[2:1\]
	pub struct SKL;
	impl RegisterView for SKL {
		type Register = super::VSTTBR_EL2;
		const NAME: &'static str = "SKL";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// VSTTBR_EL2\[0\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::VSTTBR_EL2;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// System Performance Monitors Root and Realm Control Register
#[allow(non_camel_case_types)]
pub struct SPMROOTCR_EL3;
impl Register for SPMROOTCR_EL3 {
	const NAME: &'static str = "SPMROOTCR_EL3";
	const LEN: usize = 64;
}

/// SPMROOTCR_EL3 register fields
pub mod spmrootcr_el3 {
	use crate::core::model::proc::RegisterView;

	/// SPMROOTCR_EL3\[3\]
	pub struct NAO;
	impl RegisterView for NAO {
		type Register = super::SPMROOTCR_EL3;
		const NAME: &'static str = "NAO";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// SPMROOTCR_EL3\[1\]
	pub struct RLO;
	impl RegisterView for RLO {
		type Register = super::SPMROOTCR_EL3;
		const NAME: &'static str = "RLO";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// SPMROOTCR_EL3\[0\]
	pub struct RTO;
	impl RegisterView for RTO {
		type Register = super::SPMROOTCR_EL3;
		const NAME: &'static str = "RTO";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Status Register
pub struct TRCSTATR;
impl Register for TRCSTATR {
	const NAME: &'static str = "TRCSTATR";
	const LEN: usize = 64;
}

/// TRCSTATR register fields
pub mod trcstatr {
	use crate::core::model::proc::RegisterView;

	/// TRCSTATR\[1\]
	pub struct PMSTABLE;
	impl RegisterView for PMSTABLE {
		type Register = super::TRCSTATR;
		const NAME: &'static str = "PMSTABLE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TRCSTATR\[0\]
	pub struct IDLE;
	impl RegisterView for IDLE {
		type Register = super::TRCSTATR;
		const NAME: &'static str = "IDLE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Hypervisor System Trap Register
#[allow(non_camel_case_types)]
pub struct HSTR_EL2;
impl Register for HSTR_EL2 {
	const NAME: &'static str = "HSTR_EL2";
	const LEN: usize = 64;
}

/// HSTR_EL2 register fields
pub mod hstr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HSTR_EL2\[15\]
	pub struct T15;
	impl RegisterView for T15 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T15";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[13\]
	pub struct T13;
	impl RegisterView for T13 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T13";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[12\]
	pub struct T12;
	impl RegisterView for T12 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T12";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[11\]
	pub struct T11;
	impl RegisterView for T11 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T11";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[10\]
	pub struct T10;
	impl RegisterView for T10 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T10";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[9\]
	pub struct T9;
	impl RegisterView for T9 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T9";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[8\]
	pub struct T8;
	impl RegisterView for T8 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T8";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[7\]
	pub struct T7;
	impl RegisterView for T7 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T7";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[6\]
	pub struct T6;
	impl RegisterView for T6 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T6";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[5\]
	pub struct T5;
	impl RegisterView for T5 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T5";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[3\]
	pub struct T3;
	impl RegisterView for T3 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T3";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[2\]
	pub struct T2;
	impl RegisterView for T2 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T2";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[1\]
	pub struct T1;
	impl RegisterView for T1 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HSTR_EL2\[0\]
	pub struct T0;
	impl RegisterView for T0 {
		type Register = super::HSTR_EL2;
		const NAME: &'static str = "T0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Pointer Authentication Key A for Code (bits\[63:0\]) 
#[allow(non_camel_case_types)]
pub struct APGAKeyLo_EL1;
impl Register for APGAKeyLo_EL1 {
	const NAME: &'static str = "APGAKeyLo_EL1";
	const LEN: usize = 64;
}

/// Trace Buffer Base Address Register
#[allow(non_camel_case_types)]
pub struct TRBBASER_EL1;
impl Register for TRBBASER_EL1 {
	const NAME: &'static str = "TRBBASER_EL1";
	const LEN: usize = 64;
}

/// TRBBASER_EL1 register fields
pub mod trbbaser_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRBBASER_EL1\[63:12\]
	pub struct BASE;
	impl RegisterView for BASE {
		type Register = super::TRBBASER_EL1;
		const NAME: &'static str = "BASE";
		const OFFSET: usize = 12;
		const LEN: usize = 52;
	}
}

/// AArch32 Memory Model Feature Register 3
#[allow(non_camel_case_types)]
pub struct ID_MMFR3_EL1;
impl Register for ID_MMFR3_EL1 {
	const NAME: &'static str = "ID_MMFR3_EL1";
	const LEN: usize = 64;
}

/// ID_MMFR3_EL1 register fields
pub mod id_mmfr3_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_MMFR3_EL1\[31:28\]
	pub struct Supersec;
	impl RegisterView for Supersec {
		type Register = super::ID_MMFR3_EL1;
		const NAME: &'static str = "Supersec";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_MMFR3_EL1\[27:24\]
	pub struct CMemSz;
	impl RegisterView for CMemSz {
		type Register = super::ID_MMFR3_EL1;
		const NAME: &'static str = "CMemSz";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_MMFR3_EL1\[23:20\]
	pub struct CohWalk;
	impl RegisterView for CohWalk {
		type Register = super::ID_MMFR3_EL1;
		const NAME: &'static str = "CohWalk";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_MMFR3_EL1\[19:16\]
	pub struct PAN;
	impl RegisterView for PAN {
		type Register = super::ID_MMFR3_EL1;
		const NAME: &'static str = "PAN";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_MMFR3_EL1\[15:12\]
	pub struct MaintBcst;
	impl RegisterView for MaintBcst {
		type Register = super::ID_MMFR3_EL1;
		const NAME: &'static str = "MaintBcst";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_MMFR3_EL1\[11:8\]
	pub struct BPMaint;
	impl RegisterView for BPMaint {
		type Register = super::ID_MMFR3_EL1;
		const NAME: &'static str = "BPMaint";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_MMFR3_EL1\[7:4\]
	pub struct CMaintSW;
	impl RegisterView for CMaintSW {
		type Register = super::ID_MMFR3_EL1;
		const NAME: &'static str = "CMaintSW";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_MMFR3_EL1\[3:0\]
	pub struct CMaintVA;
	impl RegisterView for CMaintVA {
		type Register = super::ID_MMFR3_EL1;
		const NAME: &'static str = "CMaintVA";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Performance Monitors Instruction Counter Register
#[allow(non_camel_case_types)]
pub struct PMICNTR_EL0;
impl Register for PMICNTR_EL0 {
	const NAME: &'static str = "PMICNTR_EL0";
	const LEN: usize = 64;
}

/// PMICNTR_EL0 register fields
pub mod pmicntr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMICNTR_EL0\[63:0\]
	pub struct ICNT;
	impl RegisterView for ICNT {
		type Register = super::PMICNTR_EL0;
		const NAME: &'static str = "ICNT";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Performance Monitors Instruction Address Register
#[allow(non_camel_case_types)]
pub struct PMIAR_EL1;
impl Register for PMIAR_EL1 {
	const NAME: &'static str = "PMIAR_EL1";
	const LEN: usize = 64;
}

/// PMIAR_EL1 register fields
pub mod pmiar_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMIAR_EL1\[63:0\]
	pub struct ADDRESS;
	impl RegisterView for ADDRESS {
		type Register = super::PMIAR_EL1;
		const NAME: &'static str = "ADDRESS";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Reset Management Register (EL1)
#[allow(non_camel_case_types)]
pub struct RMR_EL1;
impl Register for RMR_EL1 {
	const NAME: &'static str = "RMR_EL1";
	const LEN: usize = 64;
}

/// RMR_EL1 register fields
pub mod rmr_el1 {
	use crate::core::model::proc::RegisterView;

	/// RMR_EL1\[1\]
	pub struct RR;
	impl RegisterView for RR {
		type Register = super::RMR_EL1;
		const NAME: &'static str = "RR";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// RMR_EL1\[0\]
	pub struct AA64;
	impl RegisterView for AA64 {
		type Register = super::RMR_EL1;
		const NAME: &'static str = "AA64";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Extended Auxiliary Memory Attribute Indirection Register (EL3)
#[allow(non_camel_case_types)]
pub struct AMAIR2_EL3;
impl Register for AMAIR2_EL3 {
	const NAME: &'static str = "AMAIR2_EL3";
	const LEN: usize = 64;
}

/// Permission Indirection Register 0 (EL1)
#[allow(non_camel_case_types)]
pub struct PIRE0_EL1;
impl Register for PIRE0_EL1 {
	const NAME: &'static str = "PIRE0_EL1";
	const LEN: usize = 64;
}

/// Translation Table Base Register 0 (EL3)
#[allow(non_camel_case_types)]
pub struct TTBR0_EL3;
impl Register for TTBR0_EL3 {
	const NAME: &'static str = "TTBR0_EL3";
	const LEN: usize = 64;
}

/// TTBR0_EL3 register fields
pub mod ttbr0_el3 {
	use crate::core::model::proc::RegisterView;

	/// TTBR0_EL3\[47:1\]
	pub struct BADDR;
	impl RegisterView for BADDR {
		type Register = super::TTBR0_EL3;
		const NAME: &'static str = "BADDR";
		const OFFSET: usize = 1;
		const LEN: usize = 47;
	}

	/// TTBR0_EL3\[2:1\]
	pub struct SKL;
	impl RegisterView for SKL {
		type Register = super::TTBR0_EL3;
		const NAME: &'static str = "SKL";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// TTBR0_EL3\[0\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::TTBR0_EL3;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Filter Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct TRFCR_EL2;
impl Register for TRFCR_EL2 {
	const NAME: &'static str = "TRFCR_EL2";
	const LEN: usize = 64;
}

/// TRFCR_EL2 register fields
pub mod trfcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// TRFCR_EL2\[6:5\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::TRFCR_EL2;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 5;
		const LEN: usize = 2;
	}

	/// TRFCR_EL2\[3\]
	pub struct CX;
	impl RegisterView for CX {
		type Register = super::TRFCR_EL2;
		const NAME: &'static str = "CX";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// TRFCR_EL2\[1\]
	pub struct E2TRE;
	impl RegisterView for E2TRE {
		type Register = super::TRFCR_EL2;
		const NAME: &'static str = "E2TRE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TRFCR_EL2\[0\]
	pub struct E0HTRE;
	impl RegisterView for E0HTRE {
		type Register = super::TRFCR_EL2;
		const NAME: &'static str = "E0HTRE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Translation Table Base Register 0 (EL1)
#[allow(non_camel_case_types)]
pub struct TTBR0_EL1;
impl Register for TTBR0_EL1 {
	const NAME: &'static str = "TTBR0_EL1";
	const LEN: usize = 64;
}

/// TTBR0_EL1 register fields
pub mod ttbr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// TTBR0_EL1\[47:1\]
	#[allow(non_camel_case_types)]
	pub struct BADDR_47_1;
	impl RegisterView for BADDR_47_1 {
		type Register = super::TTBR0_EL1;
		const NAME: &'static str = "BADDR_47_1";
		const OFFSET: usize = 1;
		const LEN: usize = 47;
	}
	/// TTBR0_EL1\[47:5\]
	#[allow(non_camel_case_types)]
	pub struct BADDR_47_1_47_5;
	impl RegisterView for BADDR_47_1_47_5 {
		type Register = super::TTBR0_EL1;
		const NAME: &'static str = "BADDR_47_1_47_5";
		const OFFSET: usize = 5;
		const LEN: usize = 43;
	}

	/// TTBR0_EL1\[63:48\]
	pub struct ASID;
	impl RegisterView for ASID {
		type Register = super::TTBR0_EL1;
		const NAME: &'static str = "ASID";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// TTBR0_EL1\[2:1\]
	pub struct SKL;
	impl RegisterView for SKL {
		type Register = super::TTBR0_EL1;
		const NAME: &'static str = "SKL";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// TTBR0_EL1\[0\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::TTBR0_EL1;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Extended Auxiliary Memory Attribute Indirection Register (EL1)
#[allow(non_camel_case_types)]
pub struct AMAIR2_EL1;
impl Register for AMAIR2_EL1 {
	const NAME: &'static str = "AMAIR2_EL1";
	const LEN: usize = 64;
}

/// Software Read Check Write Instruction Mask (EL1)
#[allow(non_camel_case_types)]
pub struct RCWSMASK_EL1;
impl Register for RCWSMASK_EL1 {
	const NAME: &'static str = "RCWSMASK_EL1";
	const LEN: usize = 64;
}

/// RCWSMASK_EL1 register fields
pub mod rcwsmask_el1 {
	use crate::core::model::proc::RegisterView;

	/// RCWSMASK_EL1\[63:0\]
	pub struct RCWSMASK;
	impl RegisterView for RCWSMASK {
		type Register = super::RCWSMASK_EL1;
		const NAME: &'static str = "RCWSMASK";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Reset Management Register (EL3)
#[allow(non_camel_case_types)]
pub struct RMR_EL3;
impl Register for RMR_EL3 {
	const NAME: &'static str = "RMR_EL3";
	const LEN: usize = 64;
}

/// RMR_EL3 register fields
pub mod rmr_el3 {
	use crate::core::model::proc::RegisterView;

	/// RMR_EL3\[1\]
	pub struct RR;
	impl RegisterView for RR {
		type Register = super::RMR_EL3;
		const NAME: &'static str = "RR";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// RMR_EL3\[0\]
	pub struct AA64;
	impl RegisterView for AA64 {
		type Register = super::RMR_EL3;
		const NAME: &'static str = "AA64";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Activity Monitors Configuration Register
#[allow(non_camel_case_types)]
pub struct AMCFGR_EL0;
impl Register for AMCFGR_EL0 {
	const NAME: &'static str = "AMCFGR_EL0";
	const LEN: usize = 64;
}

/// AMCFGR_EL0 register fields
pub mod amcfgr_el0 {
	use crate::core::model::proc::RegisterView;

	/// AMCFGR_EL0\[31:28\]
	pub struct NCG;
	impl RegisterView for NCG {
		type Register = super::AMCFGR_EL0;
		const NAME: &'static str = "NCG";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// AMCFGR_EL0\[24\]
	pub struct HDBG;
	impl RegisterView for HDBG {
		type Register = super::AMCFGR_EL0;
		const NAME: &'static str = "HDBG";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// AMCFGR_EL0\[13:8\]
	pub struct SIZE;
	impl RegisterView for SIZE {
		type Register = super::AMCFGR_EL0;
		const NAME: &'static str = "SIZE";
		const OFFSET: usize = 8;
		const LEN: usize = 6;
	}

	/// AMCFGR_EL0\[7:0\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::AMCFGR_EL0;
		const NAME: &'static str = "N";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// LORegionID (EL1)
#[allow(non_camel_case_types)]
pub struct LORID_EL1;
impl Register for LORID_EL1 {
	const NAME: &'static str = "LORID_EL1";
	const LEN: usize = 64;
}

/// LORID_EL1 register fields
pub mod lorid_el1 {
	use crate::core::model::proc::RegisterView;

	/// LORID_EL1\[23:16\]
	pub struct LD;
	impl RegisterView for LD {
		type Register = super::LORID_EL1;
		const NAME: &'static str = "LD";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// LORID_EL1\[7:0\]
	pub struct LR;
	impl RegisterView for LR {
		type Register = super::LORID_EL1;
		const NAME: &'static str = "LR";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// System Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct SCTLR_EL2;
impl Register for SCTLR_EL2 {
	const NAME: &'static str = "SCTLR_EL2";
	const LEN: usize = 64;
}

/// SCTLR_EL2 register fields
pub mod sctlr_el2 {
	use crate::core::model::proc::RegisterView;

	/// SCTLR_EL2\[63\]
	pub struct TIDCP;
	impl RegisterView for TIDCP {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TIDCP";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[62\]
	pub struct SPINTMASK;
	impl RegisterView for SPINTMASK {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "SPINTMASK";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[61\]
	pub struct NMI;
	impl RegisterView for NMI {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[60\]
	pub struct EnTP2;
	impl RegisterView for EnTP2 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnTP2";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[59\]
	pub struct TCSO;
	impl RegisterView for TCSO {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TCSO";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[58\]
	pub struct TCSO0;
	impl RegisterView for TCSO0 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TCSO0";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[57\]
	pub struct EPAN;
	impl RegisterView for EPAN {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EPAN";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[56\]
	pub struct EnALS;
	impl RegisterView for EnALS {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnALS";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[55\]
	pub struct EnAS0;
	impl RegisterView for EnAS0 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnAS0";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[54\]
	pub struct EnASR;
	impl RegisterView for EnASR {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnASR";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[53\]
	pub struct TME;
	impl RegisterView for TME {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TME";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[52\]
	pub struct TME0;
	impl RegisterView for TME0 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TME0";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[51\]
	pub struct TMT;
	impl RegisterView for TMT {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TMT";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[50\]
	pub struct TMT0;
	impl RegisterView for TMT0 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TMT0";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[49:46\]
	pub struct TWEDEL;
	impl RegisterView for TWEDEL {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TWEDEL";
		const OFFSET: usize = 46;
		const LEN: usize = 4;
	}

	/// SCTLR_EL2\[45\]
	pub struct TWEDEn;
	impl RegisterView for TWEDEn {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TWEDEn";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[44\]
	pub struct DSSBS;
	impl RegisterView for DSSBS {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "DSSBS";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[43\]
	pub struct ATA;
	impl RegisterView for ATA {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "ATA";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[42\]
	pub struct ATA0;
	impl RegisterView for ATA0 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "ATA0";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[41:40\]
	pub struct TCF;
	impl RegisterView for TCF {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TCF";
		const OFFSET: usize = 40;
		const LEN: usize = 2;
	}

	/// SCTLR_EL2\[39:38\]
	pub struct TCF0;
	impl RegisterView for TCF0 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TCF0";
		const OFFSET: usize = 38;
		const LEN: usize = 2;
	}

	/// SCTLR_EL2\[37\]
	pub struct ITFSB;
	impl RegisterView for ITFSB {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "ITFSB";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[36\]
	pub struct BT;
	impl RegisterView for BT {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "BT";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[35\]
	pub struct BT0;
	impl RegisterView for BT0 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "BT0";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[34\]
	pub struct EnFPM;
	impl RegisterView for EnFPM {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnFPM";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[33\]
	pub struct MSCEn;
	impl RegisterView for MSCEn {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "MSCEn";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[32\]
	pub struct CMOW;
	impl RegisterView for CMOW {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "CMOW";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[31\]
	pub struct EnIA;
	impl RegisterView for EnIA {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnIA";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[30\]
	pub struct EnIB;
	impl RegisterView for EnIB {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnIB";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[29\]
	pub struct LSMAOE;
	impl RegisterView for LSMAOE {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "LSMAOE";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[28\]
	#[allow(non_camel_case_types)]
	pub struct nTLSMD;
	impl RegisterView for nTLSMD {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "nTLSMD";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[27\]
	pub struct EnDA;
	impl RegisterView for EnDA {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnDA";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[26\]
	pub struct UCI;
	impl RegisterView for UCI {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "UCI";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[25\]
	pub struct EE;
	impl RegisterView for EE {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EE";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[24\]
	pub struct E0E;
	impl RegisterView for E0E {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "E0E";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[23\]
	pub struct SPAN;
	impl RegisterView for SPAN {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "SPAN";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[22\]
	pub struct EIS;
	impl RegisterView for EIS {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EIS";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[21\]
	pub struct IESB;
	impl RegisterView for IESB {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "IESB";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[20\]
	pub struct TSCXT;
	impl RegisterView for TSCXT {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "TSCXT";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[19\]
	pub struct WXN;
	impl RegisterView for WXN {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "WXN";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[18\]
	#[allow(non_camel_case_types)]
	pub struct nTWE;
	impl RegisterView for nTWE {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "nTWE";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[16\]
	#[allow(non_camel_case_types)]
	pub struct nTWI;
	impl RegisterView for nTWI {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "nTWI";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[15\]
	pub struct UCT;
	impl RegisterView for UCT {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "UCT";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[14\]
	pub struct DZE;
	impl RegisterView for DZE {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "DZE";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[13\]
	pub struct EnDB;
	impl RegisterView for EnDB {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnDB";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[12\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "I";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[11\]
	pub struct EOS;
	impl RegisterView for EOS {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EOS";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[10\]
	pub struct EnRCTX;
	impl RegisterView for EnRCTX {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "EnRCTX";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[8\]
	pub struct SED;
	impl RegisterView for SED {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "SED";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[7\]
	pub struct ITD;
	impl RegisterView for ITD {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "ITD";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[6\]
	#[allow(non_camel_case_types)]
	pub struct nAA;
	impl RegisterView for nAA {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "nAA";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[5\]
	pub struct CP15BEN;
	impl RegisterView for CP15BEN {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "CP15BEN";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[4\]
	pub struct SA0;
	impl RegisterView for SA0 {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "SA0";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[3\]
	pub struct SA;
	impl RegisterView for SA {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "SA";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[2\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "C";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[1\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "A";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// SCTLR_EL2\[0\]
	pub struct M;
	impl RegisterView for M {
		type Register = super::SCTLR_EL2;
		const NAME: &'static str = "M";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// AArch32 Secure Debug Enable Register
#[allow(non_camel_case_types)]
pub struct SDER32_EL3;
impl Register for SDER32_EL3 {
	const NAME: &'static str = "SDER32_EL3";
	const LEN: usize = 64;
}

/// SDER32_EL3 register fields
pub mod sder32_el3 {
	use crate::core::model::proc::RegisterView;

	/// SDER32_EL3\[1\]
	pub struct SUNIDEN;
	impl RegisterView for SUNIDEN {
		type Register = super::SDER32_EL3;
		const NAME: &'static str = "SUNIDEN";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// SDER32_EL3\[0\]
	pub struct SUIDEN;
	impl RegisterView for SUIDEN {
		type Register = super::SDER32_EL3;
		const NAME: &'static str = "SUIDEN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// System Performance Monitors Count Enable Clear Register
#[allow(non_camel_case_types)]
pub struct SPMCNTENCLR_EL0;
impl Register for SPMCNTENCLR_EL0 {
	const NAME: &'static str = "SPMCNTENCLR_EL0";
	const LEN: usize = 64;
}

/// Branch Record Buffer Timestamp Register
#[allow(non_camel_case_types)]
pub struct BRBTS_EL1;
impl Register for BRBTS_EL1 {
	const NAME: &'static str = "BRBTS_EL1";
	const LEN: usize = 64;
}

/// BRBTS_EL1 register fields
pub mod brbts_el1 {
	use crate::core::model::proc::RegisterView;

	/// BRBTS_EL1\[63:0\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::BRBTS_EL1;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Physical Fault Address Register (EL3)
#[allow(non_camel_case_types)]
pub struct MFAR_EL3;
impl Register for MFAR_EL3 {
	const NAME: &'static str = "MFAR_EL3";
	const LEN: usize = 64;
}

/// MFAR_EL3 register fields
pub mod mfar_el3 {
	use crate::core::model::proc::RegisterView;

	/// MFAR_EL3\[63\]
	pub struct NS;
	impl RegisterView for NS {
		type Register = super::MFAR_EL3;
		const NAME: &'static str = "NS";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// MFAR_EL3\[62\]
	pub struct NSE;
	impl RegisterView for NSE {
		type Register = super::MFAR_EL3;
		const NAME: &'static str = "NSE";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// MFAR_EL3\[55:52\]
	#[allow(non_camel_case_types)]
	pub struct FPA_55_52;
	impl RegisterView for FPA_55_52 {
		type Register = super::MFAR_EL3;
		const NAME: &'static str = "FPA_55_52";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}
	/// MFAR_EL3\[51:48\]
	#[allow(non_camel_case_types)]
	pub struct FPA_55_52_51_48;
	impl RegisterView for FPA_55_52_51_48 {
		type Register = super::MFAR_EL3;
		const NAME: &'static str = "FPA_55_52_51_48";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// MFAR_EL3\[55:52\]
	#[allow(non_camel_case_types)]
	pub struct PA_55_52;
	impl RegisterView for PA_55_52 {
		type Register = super::MFAR_EL3;
		const NAME: &'static str = "PA_55_52";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}
	/// MFAR_EL3\[51:48\]
	#[allow(non_camel_case_types)]
	pub struct PA_55_52_51_48;
	impl RegisterView for PA_55_52_51_48 {
		type Register = super::MFAR_EL3;
		const NAME: &'static str = "PA_55_52_51_48";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}
}

/// Trace Event Control 0 Register
pub struct TRCEVENTCTL0R;
impl Register for TRCEVENTCTL0R {
	const NAME: &'static str = "TRCEVENTCTL0R";
	const LEN: usize = 64;
}

/// TRCEVENTCTL0R register fields
pub mod trceventctl0r {
	use crate::core::model::proc::RegisterView;

	/// TRCEVENTCTL0R\[31\]
	#[allow(non_camel_case_types)]
	pub struct EVENT3_TYPE;
	impl RegisterView for EVENT3_TYPE {
		type Register = super::TRCEVENTCTL0R;
		const NAME: &'static str = "EVENT3_TYPE";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// TRCEVENTCTL0R\[28:24\]
	#[allow(non_camel_case_types)]
	pub struct EVENT3_SEL;
	impl RegisterView for EVENT3_SEL {
		type Register = super::TRCEVENTCTL0R;
		const NAME: &'static str = "EVENT3_SEL";
		const OFFSET: usize = 24;
		const LEN: usize = 5;
	}

	/// TRCEVENTCTL0R\[23\]
	#[allow(non_camel_case_types)]
	pub struct EVENT2_TYPE;
	impl RegisterView for EVENT2_TYPE {
		type Register = super::TRCEVENTCTL0R;
		const NAME: &'static str = "EVENT2_TYPE";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// TRCEVENTCTL0R\[20:16\]
	#[allow(non_camel_case_types)]
	pub struct EVENT2_SEL;
	impl RegisterView for EVENT2_SEL {
		type Register = super::TRCEVENTCTL0R;
		const NAME: &'static str = "EVENT2_SEL";
		const OFFSET: usize = 16;
		const LEN: usize = 5;
	}

	/// TRCEVENTCTL0R\[15\]
	#[allow(non_camel_case_types)]
	pub struct EVENT1_TYPE;
	impl RegisterView for EVENT1_TYPE {
		type Register = super::TRCEVENTCTL0R;
		const NAME: &'static str = "EVENT1_TYPE";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// TRCEVENTCTL0R\[12:8\]
	#[allow(non_camel_case_types)]
	pub struct EVENT1_SEL;
	impl RegisterView for EVENT1_SEL {
		type Register = super::TRCEVENTCTL0R;
		const NAME: &'static str = "EVENT1_SEL";
		const OFFSET: usize = 8;
		const LEN: usize = 5;
	}

	/// TRCEVENTCTL0R\[7\]
	#[allow(non_camel_case_types)]
	pub struct EVENT0_TYPE;
	impl RegisterView for EVENT0_TYPE {
		type Register = super::TRCEVENTCTL0R;
		const NAME: &'static str = "EVENT0_TYPE";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TRCEVENTCTL0R\[4:0\]
	#[allow(non_camel_case_types)]
	pub struct EVENT0_SEL;
	impl RegisterView for EVENT0_SEL {
		type Register = super::TRCEVENTCTL0R;
		const NAME: &'static str = "EVENT0_SEL";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Performance Monitors Instruction Count Saved Value Register
#[allow(non_camel_case_types)]
pub struct PMICNTSVR_EL1;
impl Register for PMICNTSVR_EL1 {
	const NAME: &'static str = "PMICNTSVR_EL1";
	const LEN: usize = 64;
}

/// PMICNTSVR_EL1 register fields
pub mod pmicntsvr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMICNTSVR_EL1\[63:0\]
	pub struct ICNT;
	impl RegisterView for ICNT {
		type Register = super::PMICNTSVR_EL1;
		const NAME: &'static str = "ICNT";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Virtualization Secure Translation Control Register
#[allow(non_camel_case_types)]
pub struct VSTCR_EL2;
impl Register for VSTCR_EL2 {
	const NAME: &'static str = "VSTCR_EL2";
	const LEN: usize = 64;
}

/// VSTCR_EL2 register fields
pub mod vstcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VSTCR_EL2\[33\]
	pub struct SL2;
	impl RegisterView for SL2 {
		type Register = super::VSTCR_EL2;
		const NAME: &'static str = "SL2";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// VSTCR_EL2\[30\]
	pub struct SA;
	impl RegisterView for SA {
		type Register = super::VSTCR_EL2;
		const NAME: &'static str = "SA";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// VSTCR_EL2\[29\]
	pub struct SW;
	impl RegisterView for SW {
		type Register = super::VSTCR_EL2;
		const NAME: &'static str = "SW";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// VSTCR_EL2\[15:14\]
	pub struct TG0;
	impl RegisterView for TG0 {
		type Register = super::VSTCR_EL2;
		const NAME: &'static str = "TG0";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// VSTCR_EL2\[7:6\]
	pub struct SL0;
	impl RegisterView for SL0 {
		type Register = super::VSTCR_EL2;
		const NAME: &'static str = "SL0";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// VSTCR_EL2\[5:0\]
	pub struct T0SZ;
	impl RegisterView for T0SZ {
		type Register = super::VSTCR_EL2;
		const NAME: &'static str = "T0SZ";
		const OFFSET: usize = 0;
		const LEN: usize = 6;
	}
}

/// MPAM Virtual PARTID Mapping Register 6
#[allow(non_camel_case_types)]
pub struct MPAMVPM6_EL2;
impl Register for MPAMVPM6_EL2 {
	const NAME: &'static str = "MPAMVPM6_EL2";
	const LEN: usize = 64;
}

/// MPAMVPM6_EL2 register fields
pub mod mpamvpm6_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMVPM6_EL2\[63:48\]
	pub struct PhyPARTID27;
	impl RegisterView for PhyPARTID27 {
		type Register = super::MPAMVPM6_EL2;
		const NAME: &'static str = "PhyPARTID27";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// MPAMVPM6_EL2\[47:32\]
	pub struct PhyPARTID26;
	impl RegisterView for PhyPARTID26 {
		type Register = super::MPAMVPM6_EL2;
		const NAME: &'static str = "PhyPARTID26";
		const OFFSET: usize = 32;
		const LEN: usize = 16;
	}

	/// MPAMVPM6_EL2\[31:16\]
	pub struct PhyPARTID25;
	impl RegisterView for PhyPARTID25 {
		type Register = super::MPAMVPM6_EL2;
		const NAME: &'static str = "PhyPARTID25";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAMVPM6_EL2\[15:0\]
	pub struct PhyPARTID24;
	impl RegisterView for PhyPARTID24 {
		type Register = super::MPAMVPM6_EL2;
		const NAME: &'static str = "PhyPARTID24";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Debug Authentication Status Register
#[allow(non_camel_case_types)]
pub struct DBGAUTHSTATUS_EL1;
impl Register for DBGAUTHSTATUS_EL1 {
	const NAME: &'static str = "DBGAUTHSTATUS_EL1";
	const LEN: usize = 64;
}

/// DBGAUTHSTATUS_EL1 register fields
pub mod dbgauthstatus_el1 {
	use crate::core::model::proc::RegisterView;

	/// DBGAUTHSTATUS_EL1\[27:26\]
	pub struct RTNID;
	impl RegisterView for RTNID {
		type Register = super::DBGAUTHSTATUS_EL1;
		const NAME: &'static str = "RTNID";
		const OFFSET: usize = 26;
		const LEN: usize = 2;
	}

	/// DBGAUTHSTATUS_EL1\[25:24\]
	pub struct RTID;
	impl RegisterView for RTID {
		type Register = super::DBGAUTHSTATUS_EL1;
		const NAME: &'static str = "RTID";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// DBGAUTHSTATUS_EL1\[15:14\]
	pub struct RLNID;
	impl RegisterView for RLNID {
		type Register = super::DBGAUTHSTATUS_EL1;
		const NAME: &'static str = "RLNID";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// DBGAUTHSTATUS_EL1\[13:12\]
	pub struct RLID;
	impl RegisterView for RLID {
		type Register = super::DBGAUTHSTATUS_EL1;
		const NAME: &'static str = "RLID";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// DBGAUTHSTATUS_EL1\[7:6\]
	pub struct SNID;
	impl RegisterView for SNID {
		type Register = super::DBGAUTHSTATUS_EL1;
		const NAME: &'static str = "SNID";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// DBGAUTHSTATUS_EL1\[5:4\]
	pub struct SID;
	impl RegisterView for SID {
		type Register = super::DBGAUTHSTATUS_EL1;
		const NAME: &'static str = "SID";
		const OFFSET: usize = 4;
		const LEN: usize = 2;
	}

	/// DBGAUTHSTATUS_EL1\[3:2\]
	pub struct NSNID;
	impl RegisterView for NSNID {
		type Register = super::DBGAUTHSTATUS_EL1;
		const NAME: &'static str = "NSNID";
		const OFFSET: usize = 2;
		const LEN: usize = 2;
	}

	/// DBGAUTHSTATUS_EL1\[1:0\]
	pub struct NSID;
	impl RegisterView for NSID {
		type Register = super::DBGAUTHSTATUS_EL1;
		const NAME: &'static str = "NSID";
		const OFFSET: usize = 0;
		const LEN: usize = 2;
	}
}

/// Counter-timer Self-Synchronized Virtual Count Register
#[allow(non_camel_case_types)]
pub struct CNTVCTSS_EL0;
impl Register for CNTVCTSS_EL0 {
	const NAME: &'static str = "CNTVCTSS_EL0";
	const LEN: usize = 64;
}

/// AArch64 Instruction Set Attribute Register 2
#[allow(non_camel_case_types)]
pub struct ID_AA64ISAR2_EL1;
impl Register for ID_AA64ISAR2_EL1 {
	const NAME: &'static str = "ID_AA64ISAR2_EL1";
	const LEN: usize = 64;
}

/// ID_AA64ISAR2_EL1 register fields
pub mod id_aa64isar2_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64ISAR2_EL1\[63:60\]
	pub struct ATS1A;
	impl RegisterView for ATS1A {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "ATS1A";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[59:56\]
	pub struct LUT;
	impl RegisterView for LUT {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "LUT";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[55:52\]
	pub struct CSSC;
	impl RegisterView for CSSC {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "CSSC";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[51:48\]
	pub struct RPRFM;
	impl RegisterView for RPRFM {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "RPRFM";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[43:40\]
	pub struct PRFMSLC;
	impl RegisterView for PRFMSLC {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "PRFMSLC";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[39:36\]
	#[allow(non_camel_case_types)]
	pub struct SYSINSTR_128;
	impl RegisterView for SYSINSTR_128 {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "SYSINSTR_128";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[35:32\]
	#[allow(non_camel_case_types)]
	pub struct SYSREG_128;
	impl RegisterView for SYSREG_128 {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "SYSREG_128";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[31:28\]
	pub struct CLRBHB;
	impl RegisterView for CLRBHB {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "CLRBHB";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[27:24\]
	#[allow(non_camel_case_types)]
	pub struct PAC_frac;
	impl RegisterView for PAC_frac {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "PAC_frac";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[23:20\]
	pub struct BC;
	impl RegisterView for BC {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "BC";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[19:16\]
	pub struct MOPS;
	impl RegisterView for MOPS {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "MOPS";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[15:12\]
	pub struct APA3;
	impl RegisterView for APA3 {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "APA3";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[11:8\]
	pub struct GPA3;
	impl RegisterView for GPA3 {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "GPA3";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[7:4\]
	pub struct RPRES;
	impl RegisterView for RPRES {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "RPRES";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR2_EL1\[3:0\]
	pub struct WFxT;
	impl RegisterView for WFxT {
		type Register = super::ID_AA64ISAR2_EL1;
		const NAME: &'static str = "WFxT";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Alternate MECID for EL1&0 stage 2 translation regime
#[allow(non_camel_case_types)]
pub struct VMECID_A_EL2;
impl Register for VMECID_A_EL2 {
	const NAME: &'static str = "VMECID_A_EL2";
	const LEN: usize = 64;
}

/// VMECID_A_EL2 register fields
pub mod vmecid_a_el2 {
	use crate::core::model::proc::RegisterView;

	/// VMECID_A_EL2\[15:0\]
	pub struct MECID;
	impl RegisterView for MECID {
		type Register = super::VMECID_A_EL2;
		const NAME: &'static str = "MECID";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// AArch32 Instruction Set Attribute Register 6
#[allow(non_camel_case_types)]
pub struct ID_ISAR6_EL1;
impl Register for ID_ISAR6_EL1 {
	const NAME: &'static str = "ID_ISAR6_EL1";
	const LEN: usize = 64;
}

/// ID_ISAR6_EL1 register fields
pub mod id_isar6_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_ISAR6_EL1\[31:28\]
	pub struct CLRBHB;
	impl RegisterView for CLRBHB {
		type Register = super::ID_ISAR6_EL1;
		const NAME: &'static str = "CLRBHB";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_ISAR6_EL1\[27:24\]
	pub struct I8MM;
	impl RegisterView for I8MM {
		type Register = super::ID_ISAR6_EL1;
		const NAME: &'static str = "I8MM";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_ISAR6_EL1\[23:20\]
	pub struct BF16;
	impl RegisterView for BF16 {
		type Register = super::ID_ISAR6_EL1;
		const NAME: &'static str = "BF16";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_ISAR6_EL1\[19:16\]
	pub struct SPECRES;
	impl RegisterView for SPECRES {
		type Register = super::ID_ISAR6_EL1;
		const NAME: &'static str = "SPECRES";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_ISAR6_EL1\[15:12\]
	pub struct SB;
	impl RegisterView for SB {
		type Register = super::ID_ISAR6_EL1;
		const NAME: &'static str = "SB";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_ISAR6_EL1\[11:8\]
	pub struct FHM;
	impl RegisterView for FHM {
		type Register = super::ID_ISAR6_EL1;
		const NAME: &'static str = "FHM";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_ISAR6_EL1\[7:4\]
	pub struct DP;
	impl RegisterView for DP {
		type Register = super::ID_ISAR6_EL1;
		const NAME: &'static str = "DP";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_ISAR6_EL1\[3:0\]
	pub struct JSCVT;
	impl RegisterView for JSCVT {
		type Register = super::ID_ISAR6_EL1;
		const NAME: &'static str = "JSCVT";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 1
#[allow(non_camel_case_types)]
pub struct ICV_HPPIR1_EL1;
impl Register for ICV_HPPIR1_EL1 {
	const NAME: &'static str = "ICV_HPPIR1_EL1";
	const LEN: usize = 64;
}

/// ICV_HPPIR1_EL1 register fields
pub mod icv_hppir1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_HPPIR1_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICV_HPPIR1_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Revision ID Register
#[allow(non_camel_case_types)]
pub struct REVIDR_EL1;
impl Register for REVIDR_EL1 {
	const NAME: &'static str = "REVIDR_EL1";
	const LEN: usize = 64;
}

/// Hypervisor Fine-Grained Write Trap Register
#[allow(non_camel_case_types)]
pub struct HFGWTR_EL2;
impl Register for HFGWTR_EL2 {
	const NAME: &'static str = "HFGWTR_EL2";
	const LEN: usize = 64;
}

/// HFGWTR_EL2 register fields
pub mod hfgwtr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HFGWTR_EL2\[63\]
	#[allow(non_camel_case_types)]
	pub struct nAMAIR2_EL1;
	impl RegisterView for nAMAIR2_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nAMAIR2_EL1";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[62\]
	#[allow(non_camel_case_types)]
	pub struct nMAIR2_EL1;
	impl RegisterView for nMAIR2_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nMAIR2_EL1";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[61\]
	#[allow(non_camel_case_types)]
	pub struct nS2POR_EL1;
	impl RegisterView for nS2POR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nS2POR_EL1";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[60\]
	#[allow(non_camel_case_types)]
	pub struct nPOR_EL1;
	impl RegisterView for nPOR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nPOR_EL1";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[59\]
	#[allow(non_camel_case_types)]
	pub struct nPOR_EL0;
	impl RegisterView for nPOR_EL0 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nPOR_EL0";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[58\]
	#[allow(non_camel_case_types)]
	pub struct nPIR_EL1;
	impl RegisterView for nPIR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nPIR_EL1";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[57\]
	#[allow(non_camel_case_types)]
	pub struct nPIRE0_EL1;
	impl RegisterView for nPIRE0_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nPIRE0_EL1";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[56\]
	#[allow(non_camel_case_types)]
	pub struct nRCWMASK_EL1;
	impl RegisterView for nRCWMASK_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nRCWMASK_EL1";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[55\]
	#[allow(non_camel_case_types)]
	pub struct nTPIDR2_EL0;
	impl RegisterView for nTPIDR2_EL0 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nTPIDR2_EL0";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[54\]
	#[allow(non_camel_case_types)]
	pub struct nSMPRI_EL1;
	impl RegisterView for nSMPRI_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nSMPRI_EL1";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[53\]
	#[allow(non_camel_case_types)]
	pub struct nGCS_EL1;
	impl RegisterView for nGCS_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nGCS_EL1";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[52\]
	#[allow(non_camel_case_types)]
	pub struct nGCS_EL0;
	impl RegisterView for nGCS_EL0 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nGCS_EL0";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[50\]
	#[allow(non_camel_case_types)]
	pub struct nACCDATA_EL1;
	impl RegisterView for nACCDATA_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "nACCDATA_EL1";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[49\]
	#[allow(non_camel_case_types)]
	pub struct ERXADDR_EL1;
	impl RegisterView for ERXADDR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ERXADDR_EL1";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[48\]
	#[allow(non_camel_case_types)]
	pub struct ERXPFGCDN_EL1;
	impl RegisterView for ERXPFGCDN_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ERXPFGCDN_EL1";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[47\]
	#[allow(non_camel_case_types)]
	pub struct ERXPFGCTL_EL1;
	impl RegisterView for ERXPFGCTL_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ERXPFGCTL_EL1";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[45\]
	#[allow(non_camel_case_types)]
	pub struct ERXMISCn_EL1;
	impl RegisterView for ERXMISCn_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ERXMISCn_EL1";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[44\]
	#[allow(non_camel_case_types)]
	pub struct ERXSTATUS_EL1;
	impl RegisterView for ERXSTATUS_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ERXSTATUS_EL1";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[43\]
	#[allow(non_camel_case_types)]
	pub struct ERXCTLR_EL1;
	impl RegisterView for ERXCTLR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ERXCTLR_EL1";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[41\]
	#[allow(non_camel_case_types)]
	pub struct ERRSELR_EL1;
	impl RegisterView for ERRSELR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ERRSELR_EL1";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[39\]
	#[allow(non_camel_case_types)]
	pub struct ICC_IGRPENn_EL1;
	impl RegisterView for ICC_IGRPENn_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ICC_IGRPENn_EL1";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[38\]
	#[allow(non_camel_case_types)]
	pub struct VBAR_EL1;
	impl RegisterView for VBAR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "VBAR_EL1";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[37\]
	#[allow(non_camel_case_types)]
	pub struct TTBR1_EL1;
	impl RegisterView for TTBR1_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "TTBR1_EL1";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[36\]
	#[allow(non_camel_case_types)]
	pub struct TTBR0_EL1;
	impl RegisterView for TTBR0_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "TTBR0_EL1";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[35\]
	#[allow(non_camel_case_types)]
	pub struct TPIDR_EL0;
	impl RegisterView for TPIDR_EL0 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "TPIDR_EL0";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[34\]
	#[allow(non_camel_case_types)]
	pub struct TPIDRRO_EL0;
	impl RegisterView for TPIDRRO_EL0 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "TPIDRRO_EL0";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[33\]
	#[allow(non_camel_case_types)]
	pub struct TPIDR_EL1;
	impl RegisterView for TPIDR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "TPIDR_EL1";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[32\]
	#[allow(non_camel_case_types)]
	pub struct TCR_EL1;
	impl RegisterView for TCR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "TCR_EL1";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[31\]
	#[allow(non_camel_case_types)]
	pub struct SCXTNUM_EL0;
	impl RegisterView for SCXTNUM_EL0 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "SCXTNUM_EL0";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[30\]
	#[allow(non_camel_case_types)]
	pub struct SCXTNUM_EL1;
	impl RegisterView for SCXTNUM_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "SCXTNUM_EL1";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[29\]
	#[allow(non_camel_case_types)]
	pub struct SCTLR_EL1;
	impl RegisterView for SCTLR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "SCTLR_EL1";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[27\]
	#[allow(non_camel_case_types)]
	pub struct PAR_EL1;
	impl RegisterView for PAR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "PAR_EL1";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[24\]
	#[allow(non_camel_case_types)]
	pub struct MAIR_EL1;
	impl RegisterView for MAIR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "MAIR_EL1";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[23\]
	#[allow(non_camel_case_types)]
	pub struct LORSA_EL1;
	impl RegisterView for LORSA_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "LORSA_EL1";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[22\]
	#[allow(non_camel_case_types)]
	pub struct LORN_EL1;
	impl RegisterView for LORN_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "LORN_EL1";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[20\]
	#[allow(non_camel_case_types)]
	pub struct LOREA_EL1;
	impl RegisterView for LOREA_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "LOREA_EL1";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[19\]
	#[allow(non_camel_case_types)]
	pub struct LORC_EL1;
	impl RegisterView for LORC_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "LORC_EL1";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[17\]
	#[allow(non_camel_case_types)]
	pub struct FAR_EL1;
	impl RegisterView for FAR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "FAR_EL1";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[16\]
	#[allow(non_camel_case_types)]
	pub struct ESR_EL1;
	impl RegisterView for ESR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "ESR_EL1";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[13\]
	#[allow(non_camel_case_types)]
	pub struct CSSELR_EL1;
	impl RegisterView for CSSELR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "CSSELR_EL1";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[12\]
	#[allow(non_camel_case_types)]
	pub struct CPACR_EL1;
	impl RegisterView for CPACR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "CPACR_EL1";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[11\]
	#[allow(non_camel_case_types)]
	pub struct CONTEXTIDR_EL1;
	impl RegisterView for CONTEXTIDR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "CONTEXTIDR_EL1";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[8\]
	pub struct APIBKey;
	impl RegisterView for APIBKey {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "APIBKey";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[7\]
	pub struct APIAKey;
	impl RegisterView for APIAKey {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "APIAKey";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[6\]
	pub struct APGAKey;
	impl RegisterView for APGAKey {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "APGAKey";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[5\]
	pub struct APDBKey;
	impl RegisterView for APDBKey {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "APDBKey";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[4\]
	pub struct APDAKey;
	impl RegisterView for APDAKey {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "APDAKey";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[3\]
	#[allow(non_camel_case_types)]
	pub struct AMAIR_EL1;
	impl RegisterView for AMAIR_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "AMAIR_EL1";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[1\]
	#[allow(non_camel_case_types)]
	pub struct AFSR1_EL1;
	impl RegisterView for AFSR1_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "AFSR1_EL1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HFGWTR_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct AFSR0_EL1;
	impl RegisterView for AFSR0_EL1 {
		type Register = super::HFGWTR_EL2;
		const NAME: &'static str = "AFSR0_EL1";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// AArch32 Secure Debug Enable Register
#[allow(non_camel_case_types)]
pub struct SDER32_EL2;
impl Register for SDER32_EL2 {
	const NAME: &'static str = "SDER32_EL2";
	const LEN: usize = 64;
}

/// SDER32_EL2 register fields
pub mod sder32_el2 {
	use crate::core::model::proc::RegisterView;

	/// SDER32_EL2\[1\]
	pub struct SUNIDEN;
	impl RegisterView for SUNIDEN {
		type Register = super::SDER32_EL2;
		const NAME: &'static str = "SUNIDEN";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// SDER32_EL2\[0\]
	pub struct SUIDEN;
	impl RegisterView for SUIDEN {
		type Register = super::SDER32_EL2;
		const NAME: &'static str = "SUIDEN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Fine-Grained Write Traps EL3
#[allow(non_camel_case_types)]
pub struct FGWTE3_EL3;
impl Register for FGWTE3_EL3 {
	const NAME: &'static str = "FGWTE3_EL3";
	const LEN: usize = 64;
}

/// FGWTE3_EL3 register fields
pub mod fgwte3_el3 {
	use crate::core::model::proc::RegisterView;

	/// FGWTE3_EL3\[21\]
	#[allow(non_camel_case_types)]
	pub struct VBAR_EL3;
	impl RegisterView for VBAR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "VBAR_EL3";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[20\]
	#[allow(non_camel_case_types)]
	pub struct TTBR0_EL3;
	impl RegisterView for TTBR0_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "TTBR0_EL3";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[19\]
	#[allow(non_camel_case_types)]
	pub struct TPIDR_EL3;
	impl RegisterView for TPIDR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "TPIDR_EL3";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[18\]
	#[allow(non_camel_case_types)]
	pub struct TCR_EL3;
	impl RegisterView for TCR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "TCR_EL3";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[17\]
	#[allow(non_camel_case_types)]
	pub struct SPMROOTCR_EL3;
	impl RegisterView for SPMROOTCR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "SPMROOTCR_EL3";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[16\]
	#[allow(non_camel_case_types)]
	pub struct SCTLR2_EL3;
	impl RegisterView for SCTLR2_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "SCTLR2_EL3";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[15\]
	#[allow(non_camel_case_types)]
	pub struct SCTLR_EL3;
	impl RegisterView for SCTLR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "SCTLR_EL3";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[14\]
	#[allow(non_camel_case_types)]
	pub struct PIR_EL3;
	impl RegisterView for PIR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "PIR_EL3";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[13\]
	#[allow(non_camel_case_types)]
	pub struct MPAM3_EL3;
	impl RegisterView for MPAM3_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "MPAM3_EL3";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[12\]
	#[allow(non_camel_case_types)]
	pub struct MECID_RL_A_EL3;
	impl RegisterView for MECID_RL_A_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "MECID_RL_A_EL3";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[11\]
	#[allow(non_camel_case_types)]
	pub struct MDCR_EL3;
	impl RegisterView for MDCR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "MDCR_EL3";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[10\]
	#[allow(non_camel_case_types)]
	pub struct MAIR2_EL3;
	impl RegisterView for MAIR2_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "MAIR2_EL3";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[9\]
	#[allow(non_camel_case_types)]
	pub struct MAIR_EL3;
	impl RegisterView for MAIR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "MAIR_EL3";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[8\]
	#[allow(non_camel_case_types)]
	pub struct GPTBR_EL3;
	impl RegisterView for GPTBR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "GPTBR_EL3";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[7\]
	#[allow(non_camel_case_types)]
	pub struct GPCCR_EL3;
	impl RegisterView for GPCCR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "GPCCR_EL3";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[6\]
	#[allow(non_camel_case_types)]
	pub struct GCSPR_EL3;
	impl RegisterView for GCSPR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "GCSPR_EL3";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[5\]
	#[allow(non_camel_case_types)]
	pub struct GCSCR_EL3;
	impl RegisterView for GCSCR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "GCSCR_EL3";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[4\]
	#[allow(non_camel_case_types)]
	pub struct AMAIR2_EL3;
	impl RegisterView for AMAIR2_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "AMAIR2_EL3";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[3\]
	#[allow(non_camel_case_types)]
	pub struct AMAIR_EL3;
	impl RegisterView for AMAIR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "AMAIR_EL3";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[2\]
	#[allow(non_camel_case_types)]
	pub struct AFSR1_EL3;
	impl RegisterView for AFSR1_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "AFSR1_EL3";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[1\]
	#[allow(non_camel_case_types)]
	pub struct AFSR0_EL3;
	impl RegisterView for AFSR0_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "AFSR0_EL3";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// FGWTE3_EL3\[0\]
	#[allow(non_camel_case_types)]
	pub struct ACTLR_EL3;
	impl RegisterView for ACTLR_EL3 {
		type Register = super::FGWTE3_EL3;
		const NAME: &'static str = "ACTLR_EL3";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Selected Event Count Register
#[allow(non_camel_case_types)]
pub struct PMXEVCNTR_EL0;
impl Register for PMXEVCNTR_EL0 {
	const NAME: &'static str = "PMXEVCNTR_EL0";
	const LEN: usize = 64;
}

/// System Performance Monitors Event Type Register, n = 63 - 0
#[allow(non_camel_case_types)]
pub struct SPMEVTYPERn_EL0;
impl RegisterArray for SPMEVTYPERn_EL0 {
	const NAME: &'static str = "SPMEVTYPERn_EL0";
	const ELEMS: usize = 63;
	const ELEM_LEN: usize = 64;
}

/// System Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct SCTLR_EL3;
impl Register for SCTLR_EL3 {
	const NAME: &'static str = "SCTLR_EL3";
	const LEN: usize = 64;
}

/// SCTLR_EL3 register fields
pub mod sctlr_el3 {
	use crate::core::model::proc::RegisterView;

	/// SCTLR_EL3\[62\]
	pub struct SPINTMASK;
	impl RegisterView for SPINTMASK {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "SPINTMASK";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[61\]
	pub struct NMI;
	impl RegisterView for NMI {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[59\]
	pub struct TCSO;
	impl RegisterView for TCSO {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "TCSO";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[53\]
	pub struct TME;
	impl RegisterView for TME {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "TME";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[51\]
	pub struct TMT;
	impl RegisterView for TMT {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "TMT";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[44\]
	pub struct DSSBS;
	impl RegisterView for DSSBS {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "DSSBS";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[43\]
	pub struct ATA;
	impl RegisterView for ATA {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "ATA";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[41:40\]
	pub struct TCF;
	impl RegisterView for TCF {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "TCF";
		const OFFSET: usize = 40;
		const LEN: usize = 2;
	}

	/// SCTLR_EL3\[37\]
	pub struct ITFSB;
	impl RegisterView for ITFSB {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "ITFSB";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[36\]
	pub struct BT;
	impl RegisterView for BT {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "BT";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[31\]
	pub struct EnIA;
	impl RegisterView for EnIA {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "EnIA";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[30\]
	pub struct EnIB;
	impl RegisterView for EnIB {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "EnIB";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[27\]
	pub struct EnDA;
	impl RegisterView for EnDA {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "EnDA";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[25\]
	pub struct EE;
	impl RegisterView for EE {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "EE";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[22\]
	pub struct EIS;
	impl RegisterView for EIS {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "EIS";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[21\]
	pub struct IESB;
	impl RegisterView for IESB {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "IESB";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[19\]
	pub struct WXN;
	impl RegisterView for WXN {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "WXN";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[13\]
	pub struct EnDB;
	impl RegisterView for EnDB {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "EnDB";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[12\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "I";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[11\]
	pub struct EOS;
	impl RegisterView for EOS {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "EOS";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[6\]
	#[allow(non_camel_case_types)]
	pub struct nAA;
	impl RegisterView for nAA {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "nAA";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[3\]
	pub struct SA;
	impl RegisterView for SA {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "SA";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[2\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "C";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[1\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "A";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// SCTLR_EL3\[0\]
	pub struct M;
	impl RegisterView for M {
		type Register = super::SCTLR_EL3;
		const NAME: &'static str = "M";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Hypervisor Debug Fine-Grained Write Trap Register 2
#[allow(non_camel_case_types)]
pub struct HDFGWTR2_EL2;
impl Register for HDFGWTR2_EL2 {
	const NAME: &'static str = "HDFGWTR2_EL2";
	const LEN: usize = 64;
}

/// HDFGWTR2_EL2 register fields
pub mod hdfgwtr2_el2 {
	use crate::core::model::proc::RegisterView;

	/// HDFGWTR2_EL2\[23\]
	#[allow(non_camel_case_types)]
	pub struct nMDSTEPOP_EL1;
	impl RegisterView for nMDSTEPOP_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nMDSTEPOP_EL1";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[22\]
	#[allow(non_camel_case_types)]
	pub struct nTRBMPAM_EL1;
	impl RegisterView for nTRBMPAM_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nTRBMPAM_EL1";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[21\]
	#[allow(non_camel_case_types)]
	pub struct nPMZR_EL0;
	impl RegisterView for nPMZR_EL0 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nPMZR_EL0";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[20\]
	#[allow(non_camel_case_types)]
	pub struct nTRCITECR_EL1;
	impl RegisterView for nTRCITECR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nTRCITECR_EL1";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[19\]
	#[allow(non_camel_case_types)]
	pub struct nPMSDSFR_EL1;
	impl RegisterView for nPMSDSFR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nPMSDSFR_EL1";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[16\]
	#[allow(non_camel_case_types)]
	pub struct nSPMSCR_EL1;
	impl RegisterView for nSPMSCR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMSCR_EL1";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[15\]
	#[allow(non_camel_case_types)]
	pub struct nSPMACCESSR_EL1;
	impl RegisterView for nSPMACCESSR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMACCESSR_EL1";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[14\]
	#[allow(non_camel_case_types)]
	pub struct nSPMCR_EL0;
	impl RegisterView for nSPMCR_EL0 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMCR_EL0";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[13\]
	#[allow(non_camel_case_types)]
	pub struct nSPMOVS;
	impl RegisterView for nSPMOVS {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMOVS";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[12\]
	#[allow(non_camel_case_types)]
	pub struct nSPMINTEN;
	impl RegisterView for nSPMINTEN {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMINTEN";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[11\]
	#[allow(non_camel_case_types)]
	pub struct nSPMCNTEN;
	impl RegisterView for nSPMCNTEN {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMCNTEN";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[10\]
	#[allow(non_camel_case_types)]
	pub struct nSPMSELR_EL0;
	impl RegisterView for nSPMSELR_EL0 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMSELR_EL0";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[9\]
	#[allow(non_camel_case_types)]
	pub struct nSPMEVTYPERn_EL0;
	impl RegisterView for nSPMEVTYPERn_EL0 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMEVTYPERn_EL0";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[8\]
	#[allow(non_camel_case_types)]
	pub struct nSPMEVCNTRn_EL0;
	impl RegisterView for nSPMEVCNTRn_EL0 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nSPMEVCNTRn_EL0";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[7\]
	#[allow(non_camel_case_types)]
	pub struct nPMSSCR_EL1;
	impl RegisterView for nPMSSCR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nPMSSCR_EL1";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[5\]
	#[allow(non_camel_case_types)]
	pub struct nMDSELR_EL1;
	impl RegisterView for nMDSELR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nMDSELR_EL1";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[4\]
	#[allow(non_camel_case_types)]
	pub struct nPMUACR_EL1;
	impl RegisterView for nPMUACR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nPMUACR_EL1";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[3\]
	#[allow(non_camel_case_types)]
	pub struct nPMICFILTR_EL0;
	impl RegisterView for nPMICFILTR_EL0 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nPMICFILTR_EL0";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[2\]
	#[allow(non_camel_case_types)]
	pub struct nPMICNTR_EL0;
	impl RegisterView for nPMICNTR_EL0 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nPMICNTR_EL0";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[1\]
	#[allow(non_camel_case_types)]
	pub struct nPMIAR_EL1;
	impl RegisterView for nPMIAR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nPMIAR_EL1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HDFGWTR2_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct nPMECR_EL1;
	impl RegisterView for nPMECR_EL1 {
		type Register = super::HDFGWTR2_EL2;
		const NAME: &'static str = "nPMECR_EL1";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Reset Management Register (EL2)
#[allow(non_camel_case_types)]
pub struct RMR_EL2;
impl Register for RMR_EL2 {
	const NAME: &'static str = "RMR_EL2";
	const LEN: usize = 64;
}

/// RMR_EL2 register fields
pub mod rmr_el2 {
	use crate::core::model::proc::RegisterView;

	/// RMR_EL2\[1\]
	pub struct RR;
	impl RegisterView for RR {
		type Register = super::RMR_EL2;
		const NAME: &'static str = "RR";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// RMR_EL2\[0\]
	pub struct AA64;
	impl RegisterView for AA64 {
		type Register = super::RMR_EL2;
		const NAME: &'static str = "AA64";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Counter-timer Virtual Timer CompareValue Register
#[allow(non_camel_case_types)]
pub struct CNTV_CVAL_EL0;
impl Register for CNTV_CVAL_EL0 {
	const NAME: &'static str = "CNTV_CVAL_EL0";
	const LEN: usize = 64;
}

/// CNTV_CVAL_EL0 register fields
pub mod cntv_cval_el0 {
	use crate::core::model::proc::RegisterView;

	/// CNTV_CVAL_EL0\[63:0\]
	pub struct CompareValue;
	impl RegisterView for CompareValue {
		type Register = super::CNTV_CVAL_EL0;
		const NAME: &'static str = "CompareValue";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Error Record Select Register
#[allow(non_camel_case_types)]
pub struct ERRSELR_EL1;
impl Register for ERRSELR_EL1 {
	const NAME: &'static str = "ERRSELR_EL1";
	const LEN: usize = 64;
}

/// ERRSELR_EL1 register fields
pub mod errselr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ERRSELR_EL1\[15:0\]
	pub struct SEL;
	impl RegisterView for SEL {
		type Register = super::ERRSELR_EL1;
		const NAME: &'static str = "SEL";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Granule Protection Table Base Register
#[allow(non_camel_case_types)]
pub struct GPTBR_EL3;
impl Register for GPTBR_EL3 {
	const NAME: &'static str = "GPTBR_EL3";
	const LEN: usize = 64;
}

/// GPTBR_EL3 register fields
pub mod gptbr_el3 {
	use crate::core::model::proc::RegisterView;

	/// GPTBR_EL3\[39:0\]
	pub struct BADDR;
	impl RegisterView for BADDR {
		type Register = super::GPTBR_EL3;
		const NAME: &'static str = "BADDR";
		const OFFSET: usize = 0;
		const LEN: usize = 40;
	}
}

/// Performance Monitors Software Increment Register
#[allow(non_camel_case_types)]
pub struct PMSWINC_EL0;
impl Register for PMSWINC_EL0 {
	const NAME: &'static str = "PMSWINC_EL0";
	const LEN: usize = 64;
}

/// Permission Indirection Register 0 (EL2)
#[allow(non_camel_case_types)]
pub struct PIRE0_EL2;
impl Register for PIRE0_EL2 {
	const NAME: &'static str = "PIRE0_EL2";
	const LEN: usize = 64;
}

/// Interrupt Controller Running Priority Register
#[allow(non_camel_case_types)]
pub struct ICC_RPR_EL1;
impl Register for ICC_RPR_EL1 {
	const NAME: &'static str = "ICC_RPR_EL1";
	const LEN: usize = 64;
}

/// ICC_RPR_EL1 register fields
pub mod icc_rpr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_RPR_EL1\[63\]
	pub struct NMI;
	impl RegisterView for NMI {
		type Register = super::ICC_RPR_EL1;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// ICC_RPR_EL1\[62\]
	#[allow(non_camel_case_types)]
	pub struct NMI_NS;
	impl RegisterView for NMI_NS {
		type Register = super::ICC_RPR_EL1;
		const NAME: &'static str = "NMI_NS";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// ICC_RPR_EL1\[7:0\]
	pub struct Priority;
	impl RegisterView for Priority {
		type Register = super::ICC_RPR_EL1;
		const NAME: &'static str = "Priority";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Trace OS Lock Status Register
pub struct TRCOSLSR;
impl Register for TRCOSLSR {
	const NAME: &'static str = "TRCOSLSR";
	const LEN: usize = 64;
}

/// TRCOSLSR register fields
pub mod trcoslsr {
	use crate::core::model::proc::RegisterView;

	/// TRCOSLSR\[0\]
	pub struct OSLM;
	impl RegisterView for OSLM {
		type Register = super::TRCOSLSR;
		const NAME: &'static str = "OSLM";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}

	/// TRCOSLSR\[1\]
	pub struct OSLK;
	impl RegisterView for OSLK {
		type Register = super::TRCOSLSR;
		const NAME: &'static str = "OSLK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller Interrupt Priority Mask Register
#[allow(non_camel_case_types)]
pub struct ICC_PMR_EL1;
impl Register for ICC_PMR_EL1 {
	const NAME: &'static str = "ICC_PMR_EL1";
	const LEN: usize = 64;
}

/// ICC_PMR_EL1 register fields
pub mod icc_pmr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_PMR_EL1\[7:0\]
	pub struct Priority;
	impl RegisterView for Priority {
		type Register = super::ICC_PMR_EL1;
		const NAME: &'static str = "Priority";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Interrupt Controller Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct ICC_CTLR_EL1;
impl Register for ICC_CTLR_EL1 {
	const NAME: &'static str = "ICC_CTLR_EL1";
	const LEN: usize = 64;
}

/// ICC_CTLR_EL1 register fields
pub mod icc_ctlr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICC_CTLR_EL1\[19\]
	pub struct ExtRange;
	impl RegisterView for ExtRange {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "ExtRange";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL1\[18\]
	pub struct RSS;
	impl RegisterView for RSS {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "RSS";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL1\[15\]
	pub struct A3V;
	impl RegisterView for A3V {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "A3V";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL1\[14\]
	pub struct SEIS;
	impl RegisterView for SEIS {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "SEIS";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL1\[13:11\]
	pub struct IDbits;
	impl RegisterView for IDbits {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "IDbits";
		const OFFSET: usize = 11;
		const LEN: usize = 3;
	}

	/// ICC_CTLR_EL1\[10:8\]
	pub struct PRIbits;
	impl RegisterView for PRIbits {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "PRIbits";
		const OFFSET: usize = 8;
		const LEN: usize = 3;
	}

	/// ICC_CTLR_EL1\[6\]
	pub struct PMHE;
	impl RegisterView for PMHE {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "PMHE";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL1\[1\]
	pub struct EOImode;
	impl RegisterView for EOImode {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "EOImode";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICC_CTLR_EL1\[0\]
	pub struct CBPR;
	impl RegisterView for CBPR {
		type Register = super::ICC_CTLR_EL1;
		const NAME: &'static str = "CBPR";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Branch Record Buffer Source Address Register \<n\>, n = 31 - 0
#[allow(non_camel_case_types)]
pub struct BRBSRCn_EL1;
impl RegisterArray for BRBSRCn_EL1 {
	const NAME: &'static str = "BRBSRCn_EL1";
	const ELEMS: usize = 31;
	const ELEM_LEN: usize = 64;
}

/// BRBSRCn_EL1 register fields
pub mod brbsrcn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// BRBSRCn_EL1\[63:0\]
	pub struct ADDRESS;
	impl RegisterArrayView for ADDRESS {
		type RegisterArray = super::BRBSRCn_EL1;
		const NAME: &'static str = "ADDRESS";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// SME Feature ID Register 0
#[allow(non_camel_case_types)]
pub struct ID_AA64SMFR0_EL1;
impl Register for ID_AA64SMFR0_EL1 {
	const NAME: &'static str = "ID_AA64SMFR0_EL1";
	const LEN: usize = 64;
}

/// ID_AA64SMFR0_EL1 register fields
pub mod id_aa64smfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64SMFR0_EL1\[63\]
	pub struct FA64;
	impl RegisterView for FA64 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "FA64";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[60\]
	pub struct LUTv2;
	impl RegisterView for LUTv2 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "LUTv2";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[59:56\]
	pub struct SMEver;
	impl RegisterView for SMEver {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "SMEver";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64SMFR0_EL1\[55:52\]
	pub struct I16I64;
	impl RegisterView for I16I64 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "I16I64";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64SMFR0_EL1\[48\]
	pub struct F64F64;
	impl RegisterView for F64F64 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "F64F64";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[47:44\]
	pub struct I16I32;
	impl RegisterView for I16I32 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "I16I32";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64SMFR0_EL1\[43\]
	pub struct B16B16;
	impl RegisterView for B16B16 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "B16B16";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[42\]
	pub struct F16F16;
	impl RegisterView for F16F16 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "F16F16";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[41\]
	pub struct F8F16;
	impl RegisterView for F8F16 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "F8F16";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[40\]
	pub struct F8F32;
	impl RegisterView for F8F32 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "F8F32";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[39:36\]
	pub struct I8I32;
	impl RegisterView for I8I32 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "I8I32";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64SMFR0_EL1\[35\]
	pub struct F16F32;
	impl RegisterView for F16F32 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "F16F32";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[34\]
	pub struct B16F32;
	impl RegisterView for B16F32 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "B16F32";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[33\]
	pub struct BI32I32;
	impl RegisterView for BI32I32 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "BI32I32";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[32\]
	pub struct F32F32;
	impl RegisterView for F32F32 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "F32F32";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[30\]
	pub struct SF8FMA;
	impl RegisterView for SF8FMA {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "SF8FMA";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[29\]
	pub struct SF8DP4;
	impl RegisterView for SF8DP4 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "SF8DP4";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// ID_AA64SMFR0_EL1\[28\]
	pub struct SF8DP2;
	impl RegisterView for SF8DP2 {
		type Register = super::ID_AA64SMFR0_EL1;
		const NAME: &'static str = "SF8DP2";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}
}

/// SVE Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct ZCR_EL3;
impl Register for ZCR_EL3 {
	const NAME: &'static str = "ZCR_EL3";
	const LEN: usize = 64;
}

/// ZCR_EL3 register fields
pub mod zcr_el3 {
	use crate::core::model::proc::RegisterView;

	/// ZCR_EL3\[3:0\]
	pub struct LEN;
	impl RegisterView for LEN {
		type Register = super::ZCR_EL3;
		const NAME: &'static str = "LEN";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Selected Error Record Primary Status Register
#[allow(non_camel_case_types)]
pub struct ERXSTATUS_EL1;
impl Register for ERXSTATUS_EL1 {
	const NAME: &'static str = "ERXSTATUS_EL1";
	const LEN: usize = 64;
}

/// Translation Table Base Register 1 (EL1)
#[allow(non_camel_case_types)]
pub struct TTBR1_EL1;
impl Register for TTBR1_EL1 {
	const NAME: &'static str = "TTBR1_EL1";
	const LEN: usize = 64;
}

/// TTBR1_EL1 register fields
pub mod ttbr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// TTBR1_EL1\[47:1\]
	#[allow(non_camel_case_types)]
	pub struct BADDR_47_1;
	impl RegisterView for BADDR_47_1 {
		type Register = super::TTBR1_EL1;
		const NAME: &'static str = "BADDR_47_1";
		const OFFSET: usize = 1;
		const LEN: usize = 47;
	}
	/// TTBR1_EL1\[47:5\]
	#[allow(non_camel_case_types)]
	pub struct BADDR_47_1_47_5;
	impl RegisterView for BADDR_47_1_47_5 {
		type Register = super::TTBR1_EL1;
		const NAME: &'static str = "BADDR_47_1_47_5";
		const OFFSET: usize = 5;
		const LEN: usize = 43;
	}

	/// TTBR1_EL1\[63:48\]
	pub struct ASID;
	impl RegisterView for ASID {
		type Register = super::TTBR1_EL1;
		const NAME: &'static str = "ASID";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// TTBR1_EL1\[2:1\]
	pub struct SKL;
	impl RegisterView for SKL {
		type Register = super::TTBR1_EL1;
		const NAME: &'static str = "SKL";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// TTBR1_EL1\[0\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::TTBR1_EL1;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Counter-timer Virtual Offset Register
#[allow(non_camel_case_types)]
pub struct CNTVOFF_EL2;
impl Register for CNTVOFF_EL2 {
	const NAME: &'static str = "CNTVOFF_EL2";
	const LEN: usize = 64;
}

/// Extended Translation Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct TCR2_EL1;
impl Register for TCR2_EL1 {
	const NAME: &'static str = "TCR2_EL1";
	const LEN: usize = 64;
}

/// TCR2_EL1 register fields
pub mod tcr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// TCR2_EL1\[18\]
	pub struct FNG1;
	impl RegisterView for FNG1 {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "FNG1";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[17\]
	pub struct FNG0;
	impl RegisterView for FNG0 {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "FNG0";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[16\]
	pub struct A2;
	impl RegisterView for A2 {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "A2";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[15\]
	pub struct DisCH1;
	impl RegisterView for DisCH1 {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "DisCH1";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[14\]
	pub struct DisCH0;
	impl RegisterView for DisCH0 {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "DisCH0";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[11\]
	pub struct HAFT;
	impl RegisterView for HAFT {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "HAFT";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[10\]
	pub struct PTTWI;
	impl RegisterView for PTTWI {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "PTTWI";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[5\]
	pub struct D128;
	impl RegisterView for D128 {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "D128";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[4\]
	pub struct AIE;
	impl RegisterView for AIE {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "AIE";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[3\]
	pub struct POE;
	impl RegisterView for POE {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "POE";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[2\]
	pub struct E0POE;
	impl RegisterView for E0POE {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "E0POE";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[1\]
	pub struct PIE;
	impl RegisterView for PIE {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "PIE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TCR2_EL1\[0\]
	pub struct PnCH;
	impl RegisterView for PnCH {
		type Register = super::TCR2_EL1;
		const NAME: &'static str = "PnCH";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Monitor Debug Configuration Register (EL2)
#[allow(non_camel_case_types)]
pub struct MDCR_EL2;
impl Register for MDCR_EL2 {
	const NAME: &'static str = "MDCR_EL2";
	const LEN: usize = 64;
}

/// MDCR_EL2 register fields
pub mod mdcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// MDCR_EL2\[50\]
	pub struct EnSTEPOP;
	impl RegisterView for EnSTEPOP {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "EnSTEPOP";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[43\]
	pub struct EBWE;
	impl RegisterView for EBWE {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "EBWE";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[41:40\]
	pub struct PMEE;
	impl RegisterView for PMEE {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "PMEE";
		const OFFSET: usize = 40;
		const LEN: usize = 2;
	}

	/// MDCR_EL2\[36\]
	pub struct HPMFZS;
	impl RegisterView for HPMFZS {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "HPMFZS";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[31:30\]
	pub struct PMSSE;
	impl RegisterView for PMSSE {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "PMSSE";
		const OFFSET: usize = 30;
		const LEN: usize = 2;
	}

	/// MDCR_EL2\[29\]
	pub struct HPMFZO;
	impl RegisterView for HPMFZO {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "HPMFZO";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[28\]
	pub struct MTPME;
	impl RegisterView for MTPME {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "MTPME";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[27\]
	pub struct TDCC;
	impl RegisterView for TDCC {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TDCC";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[26\]
	pub struct HLP;
	impl RegisterView for HLP {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "HLP";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[25:24\]
	pub struct E2TB;
	impl RegisterView for E2TB {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "E2TB";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// MDCR_EL2\[23\]
	pub struct HCCD;
	impl RegisterView for HCCD {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "HCCD";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[19\]
	pub struct TTRF;
	impl RegisterView for TTRF {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TTRF";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[17\]
	pub struct HPMD;
	impl RegisterView for HPMD {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "HPMD";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[15\]
	pub struct EnSPM;
	impl RegisterView for EnSPM {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "EnSPM";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[14\]
	pub struct TPMS;
	impl RegisterView for TPMS {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TPMS";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[13:12\]
	pub struct E2PB;
	impl RegisterView for E2PB {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "E2PB";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// MDCR_EL2\[11\]
	pub struct TDRA;
	impl RegisterView for TDRA {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TDRA";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[10\]
	pub struct TDOSA;
	impl RegisterView for TDOSA {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TDOSA";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[9\]
	pub struct TDA;
	impl RegisterView for TDA {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TDA";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[8\]
	pub struct TDE;
	impl RegisterView for TDE {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TDE";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[7\]
	pub struct HPME;
	impl RegisterView for HPME {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "HPME";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[6\]
	pub struct TPM;
	impl RegisterView for TPM {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TPM";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[5\]
	pub struct TPMCR;
	impl RegisterView for TPMCR {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "TPMCR";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// MDCR_EL2\[4:0\]
	pub struct HPMN;
	impl RegisterView for HPMN {
		type Register = super::MDCR_EL2;
		const NAME: &'static str = "HPMN";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Instrumentation Trace Extension External Debug Control Register
pub struct TRCITEEDCR;
impl Register for TRCITEEDCR {
	const NAME: &'static str = "TRCITEEDCR";
	const LEN: usize = 64;
}

/// TRCITEEDCR register fields
pub mod trciteedcr {
	use crate::core::model::proc::RegisterView;

	/// TRCITEEDCR\[6\]
	pub struct RL;
	impl RegisterView for RL {
		type Register = super::TRCITEEDCR;
		const NAME: &'static str = "RL";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// TRCITEEDCR\[5\]
	pub struct S;
	impl RegisterView for S {
		type Register = super::TRCITEEDCR;
		const NAME: &'static str = "S";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// TRCITEEDCR\[4\]
	pub struct NS;
	impl RegisterView for NS {
		type Register = super::TRCITEEDCR;
		const NAME: &'static str = "NS";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// TRCITEEDCR\[3\]
	pub struct E3;
	impl RegisterView for E3 {
		type Register = super::TRCITEEDCR;
		const NAME: &'static str = "E3";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}
}

/// System Performance Monitors Access Register (EL3)
#[allow(non_camel_case_types)]
pub struct SPMACCESSR_EL3;
impl Register for SPMACCESSR_EL3 {
	const NAME: &'static str = "SPMACCESSR_EL3";
	const LEN: usize = 64;
}

/// MPAM Virtual PARTID Mapping Register 7
#[allow(non_camel_case_types)]
pub struct MPAMVPM7_EL2;
impl Register for MPAMVPM7_EL2 {
	const NAME: &'static str = "MPAMVPM7_EL2";
	const LEN: usize = 64;
}

/// MPAMVPM7_EL2 register fields
pub mod mpamvpm7_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMVPM7_EL2\[63:48\]
	pub struct PhyPARTID31;
	impl RegisterView for PhyPARTID31 {
		type Register = super::MPAMVPM7_EL2;
		const NAME: &'static str = "PhyPARTID31";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// MPAMVPM7_EL2\[47:32\]
	pub struct PhyPARTID30;
	impl RegisterView for PhyPARTID30 {
		type Register = super::MPAMVPM7_EL2;
		const NAME: &'static str = "PhyPARTID30";
		const OFFSET: usize = 32;
		const LEN: usize = 16;
	}

	/// MPAMVPM7_EL2\[31:16\]
	pub struct PhyPARTID29;
	impl RegisterView for PhyPARTID29 {
		type Register = super::MPAMVPM7_EL2;
		const NAME: &'static str = "PhyPARTID29";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAMVPM7_EL2\[15:0\]
	pub struct PhyPARTID28;
	impl RegisterView for PhyPARTID28 {
		type Register = super::MPAMVPM7_EL2;
		const NAME: &'static str = "PhyPARTID28";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Permission Overlay Register 2 (EL2)
#[allow(non_camel_case_types)]
pub struct POR_EL2;
impl Register for POR_EL2 {
	const NAME: &'static str = "POR_EL2";
	const LEN: usize = 64;
}

/// Guarded Control Stack Pointer Register (EL3)
#[allow(non_camel_case_types)]
pub struct GCSPR_EL3;
impl Register for GCSPR_EL3 {
	const NAME: &'static str = "GCSPR_EL3";
	const LEN: usize = 64;
}

/// GCSPR_EL3 register fields
pub mod gcspr_el3 {
	use crate::core::model::proc::RegisterView;

	/// GCSPR_EL3\[63:3\]
	pub struct PTR;
	impl RegisterView for PTR {
		type Register = super::GCSPR_EL3;
		const NAME: &'static str = "PTR";
		const OFFSET: usize = 3;
		const LEN: usize = 61;
	}
}

/// Fault Address Register (EL2)
#[allow(non_camel_case_types)]
pub struct FAR_EL2;
impl Register for FAR_EL2 {
	const NAME: &'static str = "FAR_EL2";
	const LEN: usize = 64;
}

/// Tag Fault Status Register (EL2)
#[allow(non_camel_case_types)]
pub struct TFSR_EL2;
impl Register for TFSR_EL2 {
	const NAME: &'static str = "TFSR_EL2";
	const LEN: usize = 64;
}

/// TFSR_EL2 register fields
pub mod tfsr_el2 {
	use crate::core::model::proc::RegisterView;

	/// TFSR_EL2\[1\]
	pub struct TF1;
	impl RegisterView for TF1 {
		type Register = super::TFSR_EL2;
		const NAME: &'static str = "TF1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TFSR_EL2\[0\]
	pub struct TF0;
	impl RegisterView for TF0 {
		type Register = super::TFSR_EL2;
		const NAME: &'static str = "TF0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Counter Control Register \<n\>, n = 3 - 0
pub struct TRCCNTCTLRn;
impl RegisterArray for TRCCNTCTLRn {
	const NAME: &'static str = "TRCCNTCTLRn";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// TRCCNTCTLRn register fields
pub mod trccntctlrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCCNTCTLRn\[17\]
	pub struct CNTCHAIN;
	impl RegisterArrayView for CNTCHAIN {
		type RegisterArray = super::TRCCNTCTLRn;
		const NAME: &'static str = "CNTCHAIN";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// TRCCNTCTLRn\[16\]
	pub struct RLDSELF;
	impl RegisterArrayView for RLDSELF {
		type RegisterArray = super::TRCCNTCTLRn;
		const NAME: &'static str = "RLDSELF";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// TRCCNTCTLRn\[15\]
	#[allow(non_camel_case_types)]
	pub struct RLDEVENT_TYPE;
	impl RegisterArrayView for RLDEVENT_TYPE {
		type RegisterArray = super::TRCCNTCTLRn;
		const NAME: &'static str = "RLDEVENT_TYPE";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// TRCCNTCTLRn\[12:8\]
	#[allow(non_camel_case_types)]
	pub struct RLDEVENT_SEL;
	impl RegisterArrayView for RLDEVENT_SEL {
		type RegisterArray = super::TRCCNTCTLRn;
		const NAME: &'static str = "RLDEVENT_SEL";
		const OFFSET: usize = 8;
		const LEN: usize = 5;
	}

	/// TRCCNTCTLRn\[7\]
	#[allow(non_camel_case_types)]
	pub struct CNTEVENT_TYPE;
	impl RegisterArrayView for CNTEVENT_TYPE {
		type RegisterArray = super::TRCCNTCTLRn;
		const NAME: &'static str = "CNTEVENT_TYPE";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TRCCNTCTLRn\[4:0\]
	#[allow(non_camel_case_types)]
	pub struct CNTEVENT_SEL;
	impl RegisterArrayView for CNTEVENT_SEL {
		type RegisterArray = super::TRCCNTCTLRn;
		const NAME: &'static str = "CNTEVENT_SEL";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Interrupt Controller Virtual Highest Priority Pending Interrupt Register 0
#[allow(non_camel_case_types)]
pub struct ICV_HPPIR0_EL1;
impl Register for ICV_HPPIR0_EL1 {
	const NAME: &'static str = "ICV_HPPIR0_EL1";
	const LEN: usize = 64;
}

/// ICV_HPPIR0_EL1 register fields
pub mod icv_hppir0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_HPPIR0_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICV_HPPIR0_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Hypervisor Debug Fine-Grained Read Trap Register 2
#[allow(non_camel_case_types)]
pub struct HDFGRTR2_EL2;
impl Register for HDFGRTR2_EL2 {
	const NAME: &'static str = "HDFGRTR2_EL2";
	const LEN: usize = 64;
}

/// HDFGRTR2_EL2 register fields
pub mod hdfgrtr2_el2 {
	use crate::core::model::proc::RegisterView;

	/// HDFGRTR2_EL2\[23\]
	#[allow(non_camel_case_types)]
	pub struct nMDSTEPOP_EL1;
	impl RegisterView for nMDSTEPOP_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nMDSTEPOP_EL1";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[22\]
	#[allow(non_camel_case_types)]
	pub struct nTRBMPAM_EL1;
	impl RegisterView for nTRBMPAM_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nTRBMPAM_EL1";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[20\]
	#[allow(non_camel_case_types)]
	pub struct nTRCITECR_EL1;
	impl RegisterView for nTRCITECR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nTRCITECR_EL1";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[19\]
	#[allow(non_camel_case_types)]
	pub struct nPMSDSFR_EL1;
	impl RegisterView for nPMSDSFR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nPMSDSFR_EL1";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[18\]
	#[allow(non_camel_case_types)]
	pub struct nSPMDEVAFF_EL1;
	impl RegisterView for nSPMDEVAFF_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMDEVAFF_EL1";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[17\]
	#[allow(non_camel_case_types)]
	pub struct nSPMID;
	impl RegisterView for nSPMID {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMID";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[16\]
	#[allow(non_camel_case_types)]
	pub struct nSPMSCR_EL1;
	impl RegisterView for nSPMSCR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMSCR_EL1";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[15\]
	#[allow(non_camel_case_types)]
	pub struct nSPMACCESSR_EL1;
	impl RegisterView for nSPMACCESSR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMACCESSR_EL1";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[14\]
	#[allow(non_camel_case_types)]
	pub struct nSPMCR_EL0;
	impl RegisterView for nSPMCR_EL0 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMCR_EL0";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[13\]
	#[allow(non_camel_case_types)]
	pub struct nSPMOVS;
	impl RegisterView for nSPMOVS {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMOVS";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[12\]
	#[allow(non_camel_case_types)]
	pub struct nSPMINTEN;
	impl RegisterView for nSPMINTEN {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMINTEN";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[11\]
	#[allow(non_camel_case_types)]
	pub struct nSPMCNTEN;
	impl RegisterView for nSPMCNTEN {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMCNTEN";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[10\]
	#[allow(non_camel_case_types)]
	pub struct nSPMSELR_EL0;
	impl RegisterView for nSPMSELR_EL0 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMSELR_EL0";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[9\]
	#[allow(non_camel_case_types)]
	pub struct nSPMEVTYPERn_EL0;
	impl RegisterView for nSPMEVTYPERn_EL0 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMEVTYPERn_EL0";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[8\]
	#[allow(non_camel_case_types)]
	pub struct nSPMEVCNTRn_EL0;
	impl RegisterView for nSPMEVCNTRn_EL0 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nSPMEVCNTRn_EL0";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[7\]
	#[allow(non_camel_case_types)]
	pub struct nPMSSCR_EL1;
	impl RegisterView for nPMSSCR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nPMSSCR_EL1";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[6\]
	#[allow(non_camel_case_types)]
	pub struct nPMSSDATA;
	impl RegisterView for nPMSSDATA {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nPMSSDATA";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[5\]
	#[allow(non_camel_case_types)]
	pub struct nMDSELR_EL1;
	impl RegisterView for nMDSELR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nMDSELR_EL1";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[4\]
	#[allow(non_camel_case_types)]
	pub struct nPMUACR_EL1;
	impl RegisterView for nPMUACR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nPMUACR_EL1";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[3\]
	#[allow(non_camel_case_types)]
	pub struct nPMICFILTR_EL0;
	impl RegisterView for nPMICFILTR_EL0 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nPMICFILTR_EL0";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[2\]
	#[allow(non_camel_case_types)]
	pub struct nPMICNTR_EL0;
	impl RegisterView for nPMICNTR_EL0 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nPMICNTR_EL0";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[1\]
	#[allow(non_camel_case_types)]
	pub struct nPMIAR_EL1;
	impl RegisterView for nPMIAR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nPMIAR_EL1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HDFGRTR2_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct nPMECR_EL1;
	impl RegisterView for nPMECR_EL1 {
		type Register = super::HDFGRTR2_EL2;
		const NAME: &'static str = "nPMECR_EL1";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Tag Fault Status Register (EL3)
#[allow(non_camel_case_types)]
pub struct TFSR_EL3;
impl Register for TFSR_EL3 {
	const NAME: &'static str = "TFSR_EL3";
	const LEN: usize = 64;
}

/// TFSR_EL3 register fields
pub mod tfsr_el3 {
	use crate::core::model::proc::RegisterView;

	/// TFSR_EL3\[0\]
	pub struct TF0;
	impl RegisterView for TF0 {
		type Register = super::TFSR_EL3;
		const NAME: &'static str = "TF0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Event Control 1 Register
pub struct TRCEVENTCTL1R;
impl Register for TRCEVENTCTL1R {
	const NAME: &'static str = "TRCEVENTCTL1R";
	const LEN: usize = 64;
}

/// TRCEVENTCTL1R register fields
pub mod trceventctl1r {
	use crate::core::model::proc::RegisterView;

	/// TRCEVENTCTL1R\[13\]
	pub struct OE;
	impl RegisterView for OE {
		type Register = super::TRCEVENTCTL1R;
		const NAME: &'static str = "OE";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// TRCEVENTCTL1R\[12\]
	pub struct LPOVERRIDE;
	impl RegisterView for LPOVERRIDE {
		type Register = super::TRCEVENTCTL1R;
		const NAME: &'static str = "LPOVERRIDE";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// TRCEVENTCTL1R\[11\]
	pub struct ATB;
	impl RegisterView for ATB {
		type Register = super::TRCEVENTCTL1R;
		const NAME: &'static str = "ATB";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}
}

/// Fault Address Register (EL3)
#[allow(non_camel_case_types)]
pub struct FAR_EL3;
impl Register for FAR_EL3 {
	const NAME: &'static str = "FAR_EL3";
	const LEN: usize = 64;
}

/// Guarded Control Stack Pointer Register (EL2)
#[allow(non_camel_case_types)]
pub struct GCSPR_EL2;
impl Register for GCSPR_EL2 {
	const NAME: &'static str = "GCSPR_EL2";
	const LEN: usize = 64;
}

/// GCSPR_EL2 register fields
pub mod gcspr_el2 {
	use crate::core::model::proc::RegisterView;

	/// GCSPR_EL2\[63:3\]
	pub struct PTR;
	impl RegisterView for PTR {
		type Register = super::GCSPR_EL2;
		const NAME: &'static str = "PTR";
		const OFFSET: usize = 3;
		const LEN: usize = 61;
	}
}

/// Permission Overlay Register 3 (EL3)
#[allow(non_camel_case_types)]
pub struct POR_EL3;
impl Register for POR_EL3 {
	const NAME: &'static str = "POR_EL3";
	const LEN: usize = 64;
}

/// Hypervisor Fine-Grained Instruction Trap Register
#[allow(non_camel_case_types)]
pub struct HFGITR_EL2;
impl Register for HFGITR_EL2 {
	const NAME: &'static str = "HFGITR_EL2";
	const LEN: usize = 64;
}

/// HFGITR_EL2 register fields
pub mod hfgitr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HFGITR_EL2\[62\]
	pub struct ATS1E1A;
	impl RegisterView for ATS1E1A {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ATS1E1A";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[60\]
	pub struct COSPRCTX;
	impl RegisterView for COSPRCTX {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "COSPRCTX";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[59\]
	#[allow(non_camel_case_types)]
	pub struct nGCSEPP;
	impl RegisterView for nGCSEPP {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "nGCSEPP";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[58\]
	#[allow(non_camel_case_types)]
	pub struct nGCSSTR_EL1;
	impl RegisterView for nGCSSTR_EL1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "nGCSSTR_EL1";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[57\]
	#[allow(non_camel_case_types)]
	pub struct nGCSPUSHM_EL1;
	impl RegisterView for nGCSPUSHM_EL1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "nGCSPUSHM_EL1";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[56\]
	#[allow(non_camel_case_types)]
	pub struct nBRBIALL;
	impl RegisterView for nBRBIALL {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "nBRBIALL";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[55\]
	#[allow(non_camel_case_types)]
	pub struct nBRBINJ;
	impl RegisterView for nBRBINJ {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "nBRBINJ";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[54\]
	pub struct DCCVAC;
	impl RegisterView for DCCVAC {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCCVAC";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[53\]
	#[allow(non_camel_case_types)]
	pub struct SVC_EL1;
	impl RegisterView for SVC_EL1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "SVC_EL1";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[52\]
	#[allow(non_camel_case_types)]
	pub struct SVC_EL0;
	impl RegisterView for SVC_EL0 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "SVC_EL0";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[51\]
	pub struct ERET;
	impl RegisterView for ERET {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ERET";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[50\]
	pub struct CPPRCTX;
	impl RegisterView for CPPRCTX {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "CPPRCTX";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[49\]
	pub struct DVPRCTX;
	impl RegisterView for DVPRCTX {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DVPRCTX";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[48\]
	pub struct CFPRCTX;
	impl RegisterView for CFPRCTX {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "CFPRCTX";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[47\]
	pub struct TLBIVAALE1;
	impl RegisterView for TLBIVAALE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAALE1";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[46\]
	pub struct TLBIVALE1;
	impl RegisterView for TLBIVALE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVALE1";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[45\]
	pub struct TLBIVAAE1;
	impl RegisterView for TLBIVAAE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAAE1";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[44\]
	pub struct TLBIASIDE1;
	impl RegisterView for TLBIASIDE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIASIDE1";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[43\]
	pub struct TLBIVAE1;
	impl RegisterView for TLBIVAE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAE1";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[42\]
	pub struct TLBIVMALLE1;
	impl RegisterView for TLBIVMALLE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVMALLE1";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[41\]
	pub struct TLBIRVAALE1;
	impl RegisterView for TLBIRVAALE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAALE1";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[40\]
	pub struct TLBIRVALE1;
	impl RegisterView for TLBIRVALE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVALE1";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[39\]
	pub struct TLBIRVAAE1;
	impl RegisterView for TLBIRVAAE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAAE1";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[38\]
	pub struct TLBIRVAE1;
	impl RegisterView for TLBIRVAE1 {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAE1";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[37\]
	pub struct TLBIRVAALE1IS;
	impl RegisterView for TLBIRVAALE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAALE1IS";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[36\]
	pub struct TLBIRVALE1IS;
	impl RegisterView for TLBIRVALE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVALE1IS";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[35\]
	pub struct TLBIRVAAE1IS;
	impl RegisterView for TLBIRVAAE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAAE1IS";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[34\]
	pub struct TLBIRVAE1IS;
	impl RegisterView for TLBIRVAE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAE1IS";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[33\]
	pub struct TLBIVAALE1IS;
	impl RegisterView for TLBIVAALE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAALE1IS";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[32\]
	pub struct TLBIVALE1IS;
	impl RegisterView for TLBIVALE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVALE1IS";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[31\]
	pub struct TLBIVAAE1IS;
	impl RegisterView for TLBIVAAE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAAE1IS";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[30\]
	pub struct TLBIASIDE1IS;
	impl RegisterView for TLBIASIDE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIASIDE1IS";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[29\]
	pub struct TLBIVAE1IS;
	impl RegisterView for TLBIVAE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAE1IS";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[28\]
	pub struct TLBIVMALLE1IS;
	impl RegisterView for TLBIVMALLE1IS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVMALLE1IS";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[27\]
	pub struct TLBIRVAALE1OS;
	impl RegisterView for TLBIRVAALE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAALE1OS";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[26\]
	pub struct TLBIRVALE1OS;
	impl RegisterView for TLBIRVALE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVALE1OS";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[25\]
	pub struct TLBIRVAAE1OS;
	impl RegisterView for TLBIRVAAE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAAE1OS";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[24\]
	pub struct TLBIRVAE1OS;
	impl RegisterView for TLBIRVAE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIRVAE1OS";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[23\]
	pub struct TLBIVAALE1OS;
	impl RegisterView for TLBIVAALE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAALE1OS";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[22\]
	pub struct TLBIVALE1OS;
	impl RegisterView for TLBIVALE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVALE1OS";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[21\]
	pub struct TLBIVAAE1OS;
	impl RegisterView for TLBIVAAE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAAE1OS";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[20\]
	pub struct TLBIASIDE1OS;
	impl RegisterView for TLBIASIDE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIASIDE1OS";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[19\]
	pub struct TLBIVAE1OS;
	impl RegisterView for TLBIVAE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVAE1OS";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[18\]
	pub struct TLBIVMALLE1OS;
	impl RegisterView for TLBIVMALLE1OS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "TLBIVMALLE1OS";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[17\]
	pub struct ATS1E1WP;
	impl RegisterView for ATS1E1WP {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ATS1E1WP";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[16\]
	pub struct ATS1E1RP;
	impl RegisterView for ATS1E1RP {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ATS1E1RP";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[15\]
	pub struct ATS1E0W;
	impl RegisterView for ATS1E0W {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ATS1E0W";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[14\]
	pub struct ATS1E0R;
	impl RegisterView for ATS1E0R {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ATS1E0R";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[13\]
	pub struct ATS1E1W;
	impl RegisterView for ATS1E1W {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ATS1E1W";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[12\]
	pub struct ATS1E1R;
	impl RegisterView for ATS1E1R {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ATS1E1R";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[11\]
	pub struct DCZVA;
	impl RegisterView for DCZVA {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCZVA";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[10\]
	pub struct DCCIVAC;
	impl RegisterView for DCCIVAC {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCCIVAC";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[9\]
	pub struct DCCVADP;
	impl RegisterView for DCCVADP {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCCVADP";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[8\]
	pub struct DCCVAP;
	impl RegisterView for DCCVAP {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCCVAP";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[7\]
	pub struct DCCVAU;
	impl RegisterView for DCCVAU {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCCVAU";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[6\]
	pub struct DCCISW;
	impl RegisterView for DCCISW {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCCISW";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[5\]
	pub struct DCCSW;
	impl RegisterView for DCCSW {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCCSW";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[4\]
	pub struct DCISW;
	impl RegisterView for DCISW {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCISW";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[3\]
	pub struct DCIVAC;
	impl RegisterView for DCIVAC {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "DCIVAC";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[2\]
	pub struct ICIVAU;
	impl RegisterView for ICIVAU {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ICIVAU";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[1\]
	pub struct ICIALLU;
	impl RegisterView for ICIALLU {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ICIALLU";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HFGITR_EL2\[0\]
	pub struct ICIALLUIS;
	impl RegisterView for ICIALLUIS {
		type Register = super::HFGITR_EL2;
		const NAME: &'static str = "ICIALLUIS";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// AArch64 Instruction Set Attribute Register 3
#[allow(non_camel_case_types)]
pub struct ID_AA64ISAR3_EL1;
impl Register for ID_AA64ISAR3_EL1 {
	const NAME: &'static str = "ID_AA64ISAR3_EL1";
	const LEN: usize = 64;
}

/// ID_AA64ISAR3_EL1 register fields
pub mod id_aa64isar3_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64ISAR3_EL1\[15:12\]
	pub struct PACM;
	impl RegisterView for PACM {
		type Register = super::ID_AA64ISAR3_EL1;
		const NAME: &'static str = "PACM";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR3_EL1\[11:8\]
	pub struct TLBIW;
	impl RegisterView for TLBIW {
		type Register = super::ID_AA64ISAR3_EL1;
		const NAME: &'static str = "TLBIW";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR3_EL1\[7:4\]
	pub struct FAMINMAX;
	impl RegisterView for FAMINMAX {
		type Register = super::ID_AA64ISAR3_EL1;
		const NAME: &'static str = "FAMINMAX";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR3_EL1\[3:0\]
	pub struct CPA;
	impl RegisterView for CPA {
		type Register = super::ID_AA64ISAR3_EL1;
		const NAME: &'static str = "CPA";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// System Performance Monitors Access Register (EL2)
#[allow(non_camel_case_types)]
pub struct SPMACCESSR_EL2;
impl Register for SPMACCESSR_EL2 {
	const NAME: &'static str = "SPMACCESSR_EL2";
	const LEN: usize = 64;
}

/// Monitor Debug Step Opcode Register
#[allow(non_camel_case_types)]
pub struct MDSTEPOP_EL1;
impl Register for MDSTEPOP_EL1 {
	const NAME: &'static str = "MDSTEPOP_EL1";
	const LEN: usize = 64;
}

/// MDSTEPOP_EL1 register fields
pub mod mdstepop_el1 {
	use crate::core::model::proc::RegisterView;

	/// MDSTEPOP_EL1\[31:0\]
	pub struct OPCODE;
	impl RegisterView for OPCODE {
		type Register = super::MDSTEPOP_EL1;
		const NAME: &'static str = "OPCODE";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Selected Error Record Address Register
#[allow(non_camel_case_types)]
pub struct ERXADDR_EL1;
impl Register for ERXADDR_EL1 {
	const NAME: &'static str = "ERXADDR_EL1";
	const LEN: usize = 64;
}

/// Performance Monitors User Access Control Register
#[allow(non_camel_case_types)]
pub struct PMUACR_EL1;
impl Register for PMUACR_EL1 {
	const NAME: &'static str = "PMUACR_EL1";
	const LEN: usize = 64;
}

/// PMUACR_EL1 register fields
pub mod pmuacr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMUACR_EL1\[31\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMUACR_EL1;
		const NAME: &'static str = "C";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
}

/// Monitor Debug Configuration Register (EL3)
#[allow(non_camel_case_types)]
pub struct MDCR_EL3;
impl Register for MDCR_EL3 {
	const NAME: &'static str = "MDCR_EL3";
	const LEN: usize = 64;
}

/// MDCR_EL3 register fields
pub mod mdcr_el3 {
	use crate::core::model::proc::RegisterView;

	/// MDCR_EL3\[50\]
	pub struct EnSTEPOP;
	impl RegisterView for EnSTEPOP {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EnSTEPOP";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[49:48\]
	pub struct ETBAD;
	impl RegisterView for ETBAD {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "ETBAD";
		const OFFSET: usize = 48;
		const LEN: usize = 2;
	}

	/// MDCR_EL3\[47\]
	pub struct EnITE;
	impl RegisterView for EnITE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EnITE";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[46:45\]
	pub struct EPMSSAD;
	impl RegisterView for EPMSSAD {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EPMSSAD";
		const OFFSET: usize = 45;
		const LEN: usize = 2;
	}

	/// MDCR_EL3\[44\]
	pub struct EnPMSS;
	impl RegisterView for EnPMSS {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EnPMSS";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[43\]
	pub struct EBWE;
	impl RegisterView for EBWE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EBWE";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[42\]
	pub struct EnPMS3;
	impl RegisterView for EnPMS3 {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EnPMS3";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[41:40\]
	pub struct PMEE;
	impl RegisterView for PMEE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "PMEE";
		const OFFSET: usize = 40;
		const LEN: usize = 2;
	}

	/// MDCR_EL3\[39\]
	pub struct EnTB2;
	impl RegisterView for EnTB2 {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EnTB2";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[38\]
	pub struct E3BREC;
	impl RegisterView for E3BREC {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "E3BREC";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[37\]
	pub struct E3BREW;
	impl RegisterView for E3BREW {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "E3BREW";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[36\]
	pub struct EnPMSN;
	impl RegisterView for EnPMSN {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EnPMSN";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[35\]
	pub struct MPMX;
	impl RegisterView for MPMX {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "MPMX";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[34\]
	pub struct MCCD;
	impl RegisterView for MCCD {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "MCCD";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[33:32\]
	pub struct SBRBE;
	impl RegisterView for SBRBE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "SBRBE";
		const OFFSET: usize = 32;
		const LEN: usize = 2;
	}

	/// MDCR_EL3\[31:30\]
	pub struct PMSSE;
	impl RegisterView for PMSSE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "PMSSE";
		const OFFSET: usize = 30;
		const LEN: usize = 2;
	}

	/// MDCR_EL3\[28\]
	pub struct MTPME;
	impl RegisterView for MTPME {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "MTPME";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[27\]
	pub struct TDCC;
	impl RegisterView for TDCC {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "TDCC";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[26\]
	pub struct NSTBE;
	impl RegisterView for NSTBE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "NSTBE";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[25:24\]
	pub struct NSTB;
	impl RegisterView for NSTB {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "NSTB";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// MDCR_EL3\[23\]
	pub struct SCCD;
	impl RegisterView for SCCD {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "SCCD";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[22\]
	pub struct ETAD;
	impl RegisterView for ETAD {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "ETAD";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[21\]
	pub struct EPMAD;
	impl RegisterView for EPMAD {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EPMAD";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[20\]
	pub struct EDAD;
	impl RegisterView for EDAD {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EDAD";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[19\]
	pub struct TTRF;
	impl RegisterView for TTRF {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "TTRF";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[18\]
	pub struct STE;
	impl RegisterView for STE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "STE";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[17\]
	pub struct SPME;
	impl RegisterView for SPME {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "SPME";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[16\]
	pub struct SDD;
	impl RegisterView for SDD {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "SDD";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[15:14\]
	pub struct SPD32;
	impl RegisterView for SPD32 {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "SPD32";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// MDCR_EL3\[13:12\]
	pub struct NSPB;
	impl RegisterView for NSPB {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "NSPB";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// MDCR_EL3\[11\]
	pub struct NSPBE;
	impl RegisterView for NSPBE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "NSPBE";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[10\]
	pub struct TDOSA;
	impl RegisterView for TDOSA {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "TDOSA";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[9\]
	pub struct TDA;
	impl RegisterView for TDA {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "TDA";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[7\]
	pub struct EnPM2;
	impl RegisterView for EnPM2 {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EnPM2";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[6\]
	pub struct TPM;
	impl RegisterView for TPM {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "TPM";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[4\]
	pub struct EDADE;
	impl RegisterView for EDADE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EDADE";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[3\]
	pub struct ETADE;
	impl RegisterView for ETADE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "ETADE";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[2\]
	pub struct EPMADE;
	impl RegisterView for EPMADE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "EPMADE";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// MDCR_EL3\[0\]
	pub struct RLTE;
	impl RegisterView for RLTE {
		type Register = super::MDCR_EL3;
		const NAME: &'static str = "RLTE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Statistical Profiling Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct PMSCR_EL1;
impl Register for PMSCR_EL1 {
	const NAME: &'static str = "PMSCR_EL1";
	const LEN: usize = 64;
}

/// PMSCR_EL1 register fields
pub mod pmscr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSCR_EL1\[7:6\]
	pub struct PCT;
	impl RegisterView for PCT {
		type Register = super::PMSCR_EL1;
		const NAME: &'static str = "PCT";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// PMSCR_EL1\[5\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::PMSCR_EL1;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// PMSCR_EL1\[4\]
	pub struct PA;
	impl RegisterView for PA {
		type Register = super::PMSCR_EL1;
		const NAME: &'static str = "PA";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// PMSCR_EL1\[3\]
	pub struct CX;
	impl RegisterView for CX {
		type Register = super::PMSCR_EL1;
		const NAME: &'static str = "CX";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// PMSCR_EL1\[1\]
	pub struct E1SPE;
	impl RegisterView for E1SPE {
		type Register = super::PMSCR_EL1;
		const NAME: &'static str = "E1SPE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// PMSCR_EL1\[0\]
	pub struct E0SPE;
	impl RegisterView for E0SPE {
		type Register = super::PMSCR_EL1;
		const NAME: &'static str = "E0SPE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Single-shot Comparator Control Status Register \<n\>, n = 7 - 0
pub struct TRCSSCSRn;
impl RegisterArray for TRCSSCSRn {
	const NAME: &'static str = "TRCSSCSRn";
	const ELEMS: usize = 7;
	const ELEM_LEN: usize = 64;
}

/// TRCSSCSRn register fields
pub mod trcsscsrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCSSCSRn\[31\]
	pub struct STATUS;
	impl RegisterArrayView for STATUS {
		type RegisterArray = super::TRCSSCSRn;
		const NAME: &'static str = "STATUS";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// TRCSSCSRn\[30\]
	pub struct PENDING;
	impl RegisterArrayView for PENDING {
		type RegisterArray = super::TRCSSCSRn;
		const NAME: &'static str = "PENDING";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// TRCSSCSRn\[3\]
	pub struct PC;
	impl RegisterArrayView for PC {
		type RegisterArray = super::TRCSSCSRn;
		const NAME: &'static str = "PC";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// TRCSSCSRn\[2\]
	pub struct DV;
	impl RegisterArrayView for DV {
		type RegisterArray = super::TRCSSCSRn;
		const NAME: &'static str = "DV";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// TRCSSCSRn\[1\]
	pub struct DA;
	impl RegisterArrayView for DA {
		type RegisterArray = super::TRCSSCSRn;
		const NAME: &'static str = "DA";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TRCSSCSRn\[0\]
	pub struct INST;
	impl RegisterArrayView for INST {
		type RegisterArray = super::TRCSSCSRn;
		const NAME: &'static str = "INST";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// SVE Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct ZCR_EL2;
impl Register for ZCR_EL2 {
	const NAME: &'static str = "ZCR_EL2";
	const LEN: usize = 64;
}

/// ZCR_EL2 register fields
pub mod zcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// ZCR_EL2\[3:0\]
	pub struct LEN;
	impl RegisterView for LEN {
		type Register = super::ZCR_EL2;
		const NAME: &'static str = "LEN";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Selected Error Record Control Register
#[allow(non_camel_case_types)]
pub struct ERXCTLR_EL1;
impl Register for ERXCTLR_EL1 {
	const NAME: &'static str = "ERXCTLR_EL1";
	const LEN: usize = 64;
}

/// Debug CLAIM Tag Set Register
#[allow(non_camel_case_types)]
pub struct DBGCLAIMSET_EL1;
impl Register for DBGCLAIMSET_EL1 {
	const NAME: &'static str = "DBGCLAIMSET_EL1";
	const LEN: usize = 64;
}

/// DBGCLAIMSET_EL1 register fields
pub mod dbgclaimset_el1 {
	use crate::core::model::proc::RegisterView;

	/// DBGCLAIMSET_EL1\[7:0\]
	pub struct CLAIM;
	impl RegisterView for CLAIM {
		type Register = super::DBGCLAIMSET_EL1;
		const NAME: &'static str = "CLAIM";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Debug Vector Catch Register
#[allow(non_camel_case_types)]
pub struct DBGVCR32_EL2;
impl Register for DBGVCR32_EL2 {
	const NAME: &'static str = "DBGVCR32_EL2";
	const LEN: usize = 64;
}

/// DBGVCR32_EL2 register fields
pub mod dbgvcr32_el2 {
	use crate::core::model::proc::RegisterView;

	/// DBGVCR32_EL2\[31\]
	pub struct NSF;
	impl RegisterView for NSF {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "NSF";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[30\]
	pub struct NSI;
	impl RegisterView for NSI {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "NSI";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[28\]
	pub struct NSD;
	impl RegisterView for NSD {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "NSD";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[27\]
	pub struct NSP;
	impl RegisterView for NSP {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "NSP";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[26\]
	pub struct NSS;
	impl RegisterView for NSS {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "NSS";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[25\]
	pub struct NSU;
	impl RegisterView for NSU {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "NSU";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[7\]
	pub struct SF;
	impl RegisterView for SF {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "SF";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[6\]
	pub struct SI;
	impl RegisterView for SI {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "SI";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[4\]
	pub struct SD;
	impl RegisterView for SD {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "SD";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[3\]
	pub struct SP;
	impl RegisterView for SP {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "SP";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[2\]
	pub struct SS;
	impl RegisterView for SS {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "SS";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[1\]
	pub struct SU;
	impl RegisterView for SU {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "SU";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[7\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "F";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[6\]
	pub struct I;
	impl RegisterView for I {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "I";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[4\]
	pub struct D;
	impl RegisterView for D {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "D";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[3\]
	pub struct P;
	impl RegisterView for P {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "P";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[2\]
	pub struct S;
	impl RegisterView for S {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "S";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// DBGVCR32_EL2\[1\]
	pub struct U;
	impl RegisterView for U {
		type Register = super::DBGVCR32_EL2;
		const NAME: &'static str = "U";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}
}

/// Read Check Write Instruction Mask (EL1)
#[allow(non_camel_case_types)]
pub struct RCWMASK_EL1;
impl Register for RCWMASK_EL1 {
	const NAME: &'static str = "RCWMASK_EL1";
	const LEN: usize = 64;
}

/// RCWMASK_EL1 register fields
pub mod rcwmask_el1 {
	use crate::core::model::proc::RegisterView;

	/// RCWMASK_EL1\[63:0\]
	pub struct RCWMASK;
	impl RegisterView for RCWMASK {
		type Register = super::RCWMASK_EL1;
		const NAME: &'static str = "RCWMASK";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Trace Sequencer Reset Control Register
pub struct TRCSEQRSTEVR;
impl Register for TRCSEQRSTEVR {
	const NAME: &'static str = "TRCSEQRSTEVR";
	const LEN: usize = 64;
}

/// TRCSEQRSTEVR register fields
pub mod trcseqrstevr {
	use crate::core::model::proc::RegisterView;

	/// TRCSEQRSTEVR\[7\]
	#[allow(non_camel_case_types)]
	pub struct RST_TYPE;
	impl RegisterView for RST_TYPE {
		type Register = super::TRCSEQRSTEVR;
		const NAME: &'static str = "RST_TYPE";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TRCSEQRSTEVR\[4:0\]
	#[allow(non_camel_case_types)]
	pub struct RST_SEL;
	impl RegisterView for RST_SEL {
		type Register = super::TRCSEQRSTEVR;
		const NAME: &'static str = "RST_SEL";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Extended Translation Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct TCR2_EL2;
impl Register for TCR2_EL2 {
	const NAME: &'static str = "TCR2_EL2";
	const LEN: usize = 64;
}

/// TCR2_EL2 register fields
pub mod tcr2_el2 {
	use crate::core::model::proc::RegisterView;

	/// TCR2_EL2\[12\]
	pub struct AMEC0;
	impl RegisterView for AMEC0 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "AMEC0";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[11\]
	pub struct HAFT;
	impl RegisterView for HAFT {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "HAFT";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[10\]
	pub struct PTTWI;
	impl RegisterView for PTTWI {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "PTTWI";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[4\]
	pub struct AIE;
	impl RegisterView for AIE {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "AIE";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[3\]
	pub struct POE;
	impl RegisterView for POE {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "POE";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[1\]
	pub struct PIE;
	impl RegisterView for PIE {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "PIE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[0\]
	pub struct PnCH;
	impl RegisterView for PnCH {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "PnCH";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[18\]
	pub struct FNG1;
	impl RegisterView for FNG1 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "FNG1";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[17\]
	pub struct FNG0;
	impl RegisterView for FNG0 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "FNG0";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[16\]
	pub struct A2;
	impl RegisterView for A2 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "A2";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[15\]
	pub struct DisCH1;
	impl RegisterView for DisCH1 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "DisCH1";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[14\]
	pub struct DisCH0;
	impl RegisterView for DisCH0 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "DisCH0";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[13\]
	pub struct AMEC1;
	impl RegisterView for AMEC1 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "AMEC1";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[9:8\]
	pub struct SKL1;
	impl RegisterView for SKL1 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "SKL1";
		const OFFSET: usize = 8;
		const LEN: usize = 2;
	}

	/// TCR2_EL2\[7:6\]
	pub struct SKL0;
	impl RegisterView for SKL0 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "SKL0";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// TCR2_EL2\[5\]
	pub struct D128;
	impl RegisterView for D128 {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "D128";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// TCR2_EL2\[2\]
	pub struct E0POE;
	impl RegisterView for E0POE {
		type Register = super::TCR2_EL2;
		const NAME: &'static str = "E0POE";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}
}

/// Translation Table Base Register 1 (EL2)
#[allow(non_camel_case_types)]
pub struct TTBR1_EL2;
impl Register for TTBR1_EL2 {
	const NAME: &'static str = "TTBR1_EL2";
	const LEN: usize = 64;
}

/// TTBR1_EL2 register fields
pub mod ttbr1_el2 {
	use crate::core::model::proc::RegisterView;

	/// TTBR1_EL2\[47:1\]
	#[allow(non_camel_case_types)]
	pub struct BADDR_47_1;
	impl RegisterView for BADDR_47_1 {
		type Register = super::TTBR1_EL2;
		const NAME: &'static str = "BADDR_47_1";
		const OFFSET: usize = 1;
		const LEN: usize = 47;
	}
	/// TTBR1_EL2\[47:5\]
	#[allow(non_camel_case_types)]
	pub struct BADDR_47_1_47_5;
	impl RegisterView for BADDR_47_1_47_5 {
		type Register = super::TTBR1_EL2;
		const NAME: &'static str = "BADDR_47_1_47_5";
		const OFFSET: usize = 5;
		const LEN: usize = 43;
	}

	/// TTBR1_EL2\[63:48\]
	pub struct ASID;
	impl RegisterView for ASID {
		type Register = super::TTBR1_EL2;
		const NAME: &'static str = "ASID";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// TTBR1_EL2\[2:1\]
	pub struct SKL;
	impl RegisterView for SKL {
		type Register = super::TTBR1_EL2;
		const NAME: &'static str = "SKL";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// TTBR1_EL2\[0\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::TTBR1_EL2;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace IMP DEF Register \<n\>, n = 7 - 1
pub struct TRCIMSPECn;
impl RegisterArray for TRCIMSPECn {
	const NAME: &'static str = "TRCIMSPECn";
	const ELEMS: usize = 6;
	const ELEM_LEN: usize = 64;
}

/// Profiling Buffer ID Register
#[allow(non_camel_case_types)]
pub struct PMBIDR_EL1;
impl Register for PMBIDR_EL1 {
	const NAME: &'static str = "PMBIDR_EL1";
	const LEN: usize = 64;
}

/// PMBIDR_EL1 register fields
pub mod pmbidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMBIDR_EL1\[11:8\]
	pub struct EA;
	impl RegisterView for EA {
		type Register = super::PMBIDR_EL1;
		const NAME: &'static str = "EA";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// PMBIDR_EL1\[5\]
	pub struct F;
	impl RegisterView for F {
		type Register = super::PMBIDR_EL1;
		const NAME: &'static str = "F";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// PMBIDR_EL1\[4\]
	pub struct P;
	impl RegisterView for P {
		type Register = super::PMBIDR_EL1;
		const NAME: &'static str = "P";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// PMBIDR_EL1\[3:0\]
	pub struct Align;
	impl RegisterView for Align {
		type Register = super::PMBIDR_EL1;
		const NAME: &'static str = "Align";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Performance Monitors Zero with Mask
#[allow(non_camel_case_types)]
pub struct PMZR_EL0;
impl Register for PMZR_EL0 {
	const NAME: &'static str = "PMZR_EL0";
	const LEN: usize = 64;
}

/// PMZR_EL0 register fields
pub mod pmzr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMZR_EL0\[31\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMZR_EL0;
		const NAME: &'static str = "C";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
}

/// Permission Overlay Register 1 (EL1)
#[allow(non_camel_case_types)]
pub struct POR_EL1;
impl Register for POR_EL1 {
	const NAME: &'static str = "POR_EL1";
	const LEN: usize = 64;
}

/// Guarded Control Stack Pointer Register (EL0)
#[allow(non_camel_case_types)]
pub struct GCSPR_EL0;
impl Register for GCSPR_EL0 {
	const NAME: &'static str = "GCSPR_EL0";
	const LEN: usize = 64;
}

/// GCSPR_EL0 register fields
pub mod gcspr_el0 {
	use crate::core::model::proc::RegisterView;

	/// GCSPR_EL0\[63:3\]
	pub struct PTR;
	impl RegisterView for PTR {
		type Register = super::GCSPR_EL0;
		const NAME: &'static str = "PTR";
		const OFFSET: usize = 3;
		const LEN: usize = 61;
	}
}

/// Fault Address Register (EL1)
#[allow(non_camel_case_types)]
pub struct FAR_EL1;
impl Register for FAR_EL1 {
	const NAME: &'static str = "FAR_EL1";
	const LEN: usize = 64;
}

/// MPAM0 Register (EL1)
#[allow(non_camel_case_types)]
pub struct MPAM0_EL1;
impl Register for MPAM0_EL1 {
	const NAME: &'static str = "MPAM0_EL1";
	const LEN: usize = 64;
}

/// MPAM0_EL1 register fields
pub mod mpam0_el1 {
	use crate::core::model::proc::RegisterView;

	/// MPAM0_EL1\[47:40\]
	#[allow(non_camel_case_types)]
	pub struct PMG_D;
	impl RegisterView for PMG_D {
		type Register = super::MPAM0_EL1;
		const NAME: &'static str = "PMG_D";
		const OFFSET: usize = 40;
		const LEN: usize = 8;
	}

	/// MPAM0_EL1\[39:32\]
	#[allow(non_camel_case_types)]
	pub struct PMG_I;
	impl RegisterView for PMG_I {
		type Register = super::MPAM0_EL1;
		const NAME: &'static str = "PMG_I";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// MPAM0_EL1\[31:16\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_D;
	impl RegisterView for PARTID_D {
		type Register = super::MPAM0_EL1;
		const NAME: &'static str = "PARTID_D";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAM0_EL1\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_I;
	impl RegisterView for PARTID_I {
		type Register = super::MPAM0_EL1;
		const NAME: &'static str = "PARTID_I";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Counter-timer Physical Timer CompareValue Register
#[allow(non_camel_case_types)]
pub struct CNTP_CVAL_EL0;
impl Register for CNTP_CVAL_EL0 {
	const NAME: &'static str = "CNTP_CVAL_EL0";
	const LEN: usize = 64;
}

/// CNTP_CVAL_EL0 register fields
pub mod cntp_cval_el0 {
	use crate::core::model::proc::RegisterView;

	/// CNTP_CVAL_EL0\[63:0\]
	pub struct CompareValue;
	impl RegisterView for CompareValue {
		type Register = super::CNTP_CVAL_EL0;
		const NAME: &'static str = "CompareValue";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Tag Fault Status Register (EL1)
#[allow(non_camel_case_types)]
pub struct TFSR_EL1;
impl Register for TFSR_EL1 {
	const NAME: &'static str = "TFSR_EL1";
	const LEN: usize = 64;
}

/// TFSR_EL1 register fields
pub mod tfsr_el1 {
	use crate::core::model::proc::RegisterView;

	/// TFSR_EL1\[1\]
	pub struct TF1;
	impl RegisterView for TF1 {
		type Register = super::TFSR_EL1;
		const NAME: &'static str = "TF1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TFSR_EL1\[0\]
	pub struct TF0;
	impl RegisterView for TF0 {
		type Register = super::TFSR_EL1;
		const NAME: &'static str = "TF0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Interrupt Controller Hyp Active Priorities Group 1 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICH_AP1Rn_EL2;
impl RegisterArray for ICH_AP1Rn_EL2 {
	const NAME: &'static str = "ICH_AP1Rn_EL2";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// ICH_AP1Rn_EL2 register fields
pub mod ich_ap1rn_el2 {
	use crate::core::model::proc::RegisterArrayView;

	/// ICH_AP1Rn_EL2\[63\]
	pub struct NMI;
	impl RegisterArrayView for NMI {
		type RegisterArray = super::ICH_AP1Rn_EL2;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}
}

/// Pointer Authentication Key A for Data (bits\[63:0\]) 
#[allow(non_camel_case_types)]
pub struct APDAKeyLo_EL1;
impl Register for APDAKeyLo_EL1 {
	const NAME: &'static str = "APDAKeyLo_EL1";
	const LEN: usize = 64;
}

/// Stage 2 Permission Indirection Register (EL2)
#[allow(non_camel_case_types)]
pub struct S2PIR_EL2;
impl Register for S2PIR_EL2 {
	const NAME: &'static str = "S2PIR_EL2";
	const LEN: usize = 64;
}

/// Guarded Control Stack Pointer Register (EL1)
#[allow(non_camel_case_types)]
pub struct GCSPR_EL1;
impl Register for GCSPR_EL1 {
	const NAME: &'static str = "GCSPR_EL1";
	const LEN: usize = 64;
}

/// GCSPR_EL1 register fields
pub mod gcspr_el1 {
	use crate::core::model::proc::RegisterView;

	/// GCSPR_EL1\[63:3\]
	pub struct PTR;
	impl RegisterView for PTR {
		type Register = super::GCSPR_EL1;
		const NAME: &'static str = "PTR";
		const OFFSET: usize = 3;
		const LEN: usize = 61;
	}
}

/// Permission Overlay Register 0 (EL0)
#[allow(non_camel_case_types)]
pub struct POR_EL0;
impl Register for POR_EL0 {
	const NAME: &'static str = "POR_EL0";
	const LEN: usize = 64;
}

/// Trace Counter Value Register \<n\>, n = 3 - 0
pub struct TRCCNTVRn;
impl RegisterArray for TRCCNTVRn {
	const NAME: &'static str = "TRCCNTVRn";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// TRCCNTVRn register fields
pub mod trccntvrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCCNTVRn\[15:0\]
	pub struct VALUE;
	impl RegisterArrayView for VALUE {
		type RegisterArray = super::TRCCNTVRn;
		const NAME: &'static str = "VALUE";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// System Performance Monitors Access Register (EL1)
#[allow(non_camel_case_types)]
pub struct SPMACCESSR_EL1;
impl Register for SPMACCESSR_EL1 {
	const NAME: &'static str = "SPMACCESSR_EL1";
	const LEN: usize = 64;
}

/// Tag Fault Status Register (EL0).
#[allow(non_camel_case_types)]
pub struct TFSRE0_EL1;
impl Register for TFSRE0_EL1 {
	const NAME: &'static str = "TFSRE0_EL1";
	const LEN: usize = 64;
}

/// TFSRE0_EL1 register fields
pub mod tfsre0_el1 {
	use crate::core::model::proc::RegisterView;

	/// TFSRE0_EL1\[1\]
	pub struct TF1;
	impl RegisterView for TF1 {
		type Register = super::TFSRE0_EL1;
		const NAME: &'static str = "TF1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// TFSRE0_EL1\[0\]
	pub struct TF0;
	impl RegisterView for TF0 {
		type Register = super::TFSRE0_EL1;
		const NAME: &'static str = "TF0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Activity Monitors Control Register
#[allow(non_camel_case_types)]
pub struct AMCR_EL0;
impl Register for AMCR_EL0 {
	const NAME: &'static str = "AMCR_EL0";
	const LEN: usize = 64;
}

/// AMCR_EL0 register fields
pub mod amcr_el0 {
	use crate::core::model::proc::RegisterView;

	/// AMCR_EL0\[17\]
	pub struct CG1RZ;
	impl RegisterView for CG1RZ {
		type Register = super::AMCR_EL0;
		const NAME: &'static str = "CG1RZ";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// AMCR_EL0\[10\]
	pub struct HDBG;
	impl RegisterView for HDBG {
		type Register = super::AMCR_EL0;
		const NAME: &'static str = "HDBG";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Interrupt Enable Set Register
#[allow(non_camel_case_types)]
pub struct PMINTENSET_EL1;
impl Register for PMINTENSET_EL1 {
	const NAME: &'static str = "PMINTENSET_EL1";
	const LEN: usize = 64;
}

/// PMINTENSET_EL1 register fields
pub mod pmintenset_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMINTENSET_EL1\[31\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMINTENSET_EL1;
		const NAME: &'static str = "C";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
}

/// Statistical Profiling Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct PMSCR_EL2;
impl Register for PMSCR_EL2 {
	const NAME: &'static str = "PMSCR_EL2";
	const LEN: usize = 64;
}

/// PMSCR_EL2 register fields
pub mod pmscr_el2 {
	use crate::core::model::proc::RegisterView;

	/// PMSCR_EL2\[7:6\]
	pub struct PCT;
	impl RegisterView for PCT {
		type Register = super::PMSCR_EL2;
		const NAME: &'static str = "PCT";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// PMSCR_EL2\[5\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::PMSCR_EL2;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// PMSCR_EL2\[4\]
	pub struct PA;
	impl RegisterView for PA {
		type Register = super::PMSCR_EL2;
		const NAME: &'static str = "PA";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// PMSCR_EL2\[3\]
	pub struct CX;
	impl RegisterView for CX {
		type Register = super::PMSCR_EL2;
		const NAME: &'static str = "CX";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// PMSCR_EL2\[1\]
	pub struct E2SPE;
	impl RegisterView for E2SPE {
		type Register = super::PMSCR_EL2;
		const NAME: &'static str = "E2SPE";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// PMSCR_EL2\[0\]
	pub struct E0HSPE;
	impl RegisterView for E0HSPE {
		type Register = super::PMSCR_EL2;
		const NAME: &'static str = "E0HSPE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// SVE Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct ZCR_EL1;
impl Register for ZCR_EL1 {
	const NAME: &'static str = "ZCR_EL1";
	const LEN: usize = 64;
}

/// ZCR_EL1 register fields
pub mod zcr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ZCR_EL1\[3:0\]
	pub struct LEN;
	impl RegisterView for LEN {
		type Register = super::ZCR_EL1;
		const NAME: &'static str = "LEN";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// AArch32 Memory Model Feature Register 2
#[allow(non_camel_case_types)]
pub struct ID_MMFR2_EL1;
impl Register for ID_MMFR2_EL1 {
	const NAME: &'static str = "ID_MMFR2_EL1";
	const LEN: usize = 64;
}

/// ID_MMFR2_EL1 register fields
pub mod id_mmfr2_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_MMFR2_EL1\[31:28\]
	pub struct HWAccFlg;
	impl RegisterView for HWAccFlg {
		type Register = super::ID_MMFR2_EL1;
		const NAME: &'static str = "HWAccFlg";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_MMFR2_EL1\[27:24\]
	pub struct WFIStall;
	impl RegisterView for WFIStall {
		type Register = super::ID_MMFR2_EL1;
		const NAME: &'static str = "WFIStall";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_MMFR2_EL1\[23:20\]
	pub struct MemBarr;
	impl RegisterView for MemBarr {
		type Register = super::ID_MMFR2_EL1;
		const NAME: &'static str = "MemBarr";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_MMFR2_EL1\[19:16\]
	pub struct UniTLB;
	impl RegisterView for UniTLB {
		type Register = super::ID_MMFR2_EL1;
		const NAME: &'static str = "UniTLB";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_MMFR2_EL1\[15:12\]
	pub struct HvdTLB;
	impl RegisterView for HvdTLB {
		type Register = super::ID_MMFR2_EL1;
		const NAME: &'static str = "HvdTLB";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_MMFR2_EL1\[11:8\]
	pub struct L1HvdRng;
	impl RegisterView for L1HvdRng {
		type Register = super::ID_MMFR2_EL1;
		const NAME: &'static str = "L1HvdRng";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_MMFR2_EL1\[7:4\]
	pub struct L1HvdBG;
	impl RegisterView for L1HvdBG {
		type Register = super::ID_MMFR2_EL1;
		const NAME: &'static str = "L1HvdBG";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_MMFR2_EL1\[3:0\]
	pub struct L1HvdFG;
	impl RegisterView for L1HvdFG {
		type Register = super::ID_MMFR2_EL1;
		const NAME: &'static str = "L1HvdFG";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Context ID Register (EL1)
#[allow(non_camel_case_types)]
pub struct CONTEXTIDR_EL1;
impl Register for CONTEXTIDR_EL1 {
	const NAME: &'static str = "CONTEXTIDR_EL1";
	const LEN: usize = 64;
}

/// CONTEXTIDR_EL1 register fields
pub mod contextidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// CONTEXTIDR_EL1\[31:0\]
	pub struct PROCID;
	impl RegisterView for PROCID {
		type Register = super::CONTEXTIDR_EL1;
		const NAME: &'static str = "PROCID";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// MPAM Virtual PARTID Mapping Register 5
#[allow(non_camel_case_types)]
pub struct MPAMVPM5_EL2;
impl Register for MPAMVPM5_EL2 {
	const NAME: &'static str = "MPAMVPM5_EL2";
	const LEN: usize = 64;
}

/// MPAMVPM5_EL2 register fields
pub mod mpamvpm5_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMVPM5_EL2\[63:48\]
	pub struct PhyPARTID23;
	impl RegisterView for PhyPARTID23 {
		type Register = super::MPAMVPM5_EL2;
		const NAME: &'static str = "PhyPARTID23";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// MPAMVPM5_EL2\[47:32\]
	pub struct PhyPARTID22;
	impl RegisterView for PhyPARTID22 {
		type Register = super::MPAMVPM5_EL2;
		const NAME: &'static str = "PhyPARTID22";
		const OFFSET: usize = 32;
		const LEN: usize = 16;
	}

	/// MPAMVPM5_EL2\[31:16\]
	pub struct PhyPARTID21;
	impl RegisterView for PhyPARTID21 {
		type Register = super::MPAMVPM5_EL2;
		const NAME: &'static str = "PhyPARTID21";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAMVPM5_EL2\[15:0\]
	pub struct PhyPARTID20;
	impl RegisterView for PhyPARTID20 {
		type Register = super::MPAMVPM5_EL2;
		const NAME: &'static str = "PhyPARTID20";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Main ID Register
#[allow(non_camel_case_types)]
pub struct MIDR_EL1;
impl Register for MIDR_EL1 {
	const NAME: &'static str = "MIDR_EL1";
	const LEN: usize = 64;
}

/// MIDR_EL1 register fields
pub mod midr_el1 {
	use crate::core::model::proc::RegisterView;

	/// MIDR_EL1\[31:24\]
	pub struct Implementer;
	impl RegisterView for Implementer {
		type Register = super::MIDR_EL1;
		const NAME: &'static str = "Implementer";
		const OFFSET: usize = 24;
		const LEN: usize = 8;
	}

	/// MIDR_EL1\[23:20\]
	pub struct Variant;
	impl RegisterView for Variant {
		type Register = super::MIDR_EL1;
		const NAME: &'static str = "Variant";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// MIDR_EL1\[19:16\]
	pub struct Architecture;
	impl RegisterView for Architecture {
		type Register = super::MIDR_EL1;
		const NAME: &'static str = "Architecture";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// MIDR_EL1\[15:4\]
	pub struct PartNum;
	impl RegisterView for PartNum {
		type Register = super::MIDR_EL1;
		const NAME: &'static str = "PartNum";
		const OFFSET: usize = 4;
		const LEN: usize = 12;
	}

	/// MIDR_EL1\[3:0\]
	pub struct Revision;
	impl RegisterView for Revision {
		type Register = super::MIDR_EL1;
		const NAME: &'static str = "Revision";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// LORegion Number (EL1)
#[allow(non_camel_case_types)]
pub struct LORN_EL1;
impl Register for LORN_EL1 {
	const NAME: &'static str = "LORN_EL1";
	const LEN: usize = 64;
}

/// LORN_EL1 register fields
pub mod lorn_el1 {
	use crate::core::model::proc::RegisterView;

	/// LORN_EL1\[7:0\]
	pub struct Num;
	impl RegisterView for Num {
		type Register = super::LORN_EL1;
		const NAME: &'static str = "Num";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Virtual SError Exception Syndrome Register
#[allow(non_camel_case_types)]
pub struct VSESR_EL2;
impl Register for VSESR_EL2 {
	const NAME: &'static str = "VSESR_EL2";
	const LEN: usize = 64;
}

/// VSESR_EL2 register fields
pub mod vsesr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VSESR_EL2\[15:14\]
	pub struct AET;
	impl RegisterView for AET {
		type Register = super::VSESR_EL2;
		const NAME: &'static str = "AET";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// VSESR_EL2\[12\]
	pub struct ExT;
	impl RegisterView for ExT {
		type Register = super::VSESR_EL2;
		const NAME: &'static str = "ExT";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// VSESR_EL2\[24\]
	pub struct IDS;
	impl RegisterView for IDS {
		type Register = super::VSESR_EL2;
		const NAME: &'static str = "IDS";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// VSESR_EL2\[23:0\]
	pub struct ISS;
	impl RegisterView for ISS {
		type Register = super::VSESR_EL2;
		const NAME: &'static str = "ISS";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Auxiliary Fault Status Register 1 (EL2)
#[allow(non_camel_case_types)]
pub struct AFSR1_EL2;
impl Register for AFSR1_EL2 {
	const NAME: &'static str = "AFSR1_EL2";
	const LEN: usize = 64;
}

/// Alternate MECID for EL2 and EL2&0 translation regimes
#[allow(non_camel_case_types)]
pub struct MECID_A0_EL2;
impl Register for MECID_A0_EL2 {
	const NAME: &'static str = "MECID_A0_EL2";
	const LEN: usize = 64;
}

/// MECID_A0_EL2 register fields
pub mod mecid_a0_el2 {
	use crate::core::model::proc::RegisterView;

	/// MECID_A0_EL2\[15:0\]
	pub struct MECID;
	impl RegisterView for MECID {
		type Register = super::MECID_A0_EL2;
		const NAME: &'static str = "MECID";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// MPAM2 Register (EL2)
#[allow(non_camel_case_types)]
pub struct MPAM2_EL2;
impl Register for MPAM2_EL2 {
	const NAME: &'static str = "MPAM2_EL2";
	const LEN: usize = 64;
}

/// MPAM2_EL2 register fields
pub mod mpam2_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAM2_EL2\[63\]
	pub struct MPAMEN;
	impl RegisterView for MPAMEN {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "MPAMEN";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// MPAM2_EL2\[58\]
	pub struct TIDR;
	impl RegisterView for TIDR {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "TIDR";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// MPAM2_EL2\[56\]
	#[allow(non_camel_case_types)]
	pub struct ALTSP_HFC;
	impl RegisterView for ALTSP_HFC {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "ALTSP_HFC";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// MPAM2_EL2\[55\]
	#[allow(non_camel_case_types)]
	pub struct ALTSP_EL2;
	impl RegisterView for ALTSP_EL2 {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "ALTSP_EL2";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// MPAM2_EL2\[54\]
	#[allow(non_camel_case_types)]
	pub struct ALTSP_FRCD;
	impl RegisterView for ALTSP_FRCD {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "ALTSP_FRCD";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// MPAM2_EL2\[50\]
	pub struct EnMPAMSM;
	impl RegisterView for EnMPAMSM {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "EnMPAMSM";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// MPAM2_EL2\[49\]
	pub struct TRAPMPAM0EL1;
	impl RegisterView for TRAPMPAM0EL1 {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "TRAPMPAM0EL1";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// MPAM2_EL2\[48\]
	pub struct TRAPMPAM1EL1;
	impl RegisterView for TRAPMPAM1EL1 {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "TRAPMPAM1EL1";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// MPAM2_EL2\[47:40\]
	#[allow(non_camel_case_types)]
	pub struct PMG_D;
	impl RegisterView for PMG_D {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "PMG_D";
		const OFFSET: usize = 40;
		const LEN: usize = 8;
	}

	/// MPAM2_EL2\[39:32\]
	#[allow(non_camel_case_types)]
	pub struct PMG_I;
	impl RegisterView for PMG_I {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "PMG_I";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// MPAM2_EL2\[31:16\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_D;
	impl RegisterView for PARTID_D {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "PARTID_D";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAM2_EL2\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_I;
	impl RegisterView for PARTID_I {
		type Register = super::MPAM2_EL2;
		const NAME: &'static str = "PARTID_I";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Interrupt Controller End of Interrupt Status Register
#[allow(non_camel_case_types)]
pub struct ICH_EISR_EL2;
impl Register for ICH_EISR_EL2 {
	const NAME: &'static str = "ICH_EISR_EL2";
	const LEN: usize = 64;
}

/// System Performance Monitors Device Affinity Register
#[allow(non_camel_case_types)]
pub struct SPMDEVAFF_EL1;
impl Register for SPMDEVAFF_EL1 {
	const NAME: &'static str = "SPMDEVAFF_EL1";
	const LEN: usize = 64;
}

/// SPMDEVAFF_EL1 register fields
pub mod spmdevaff_el1 {
	use crate::core::model::proc::RegisterView;

	/// SPMDEVAFF_EL1\[39:32\]
	pub struct Aff3;
	impl RegisterView for Aff3 {
		type Register = super::SPMDEVAFF_EL1;
		const NAME: &'static str = "Aff3";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// SPMDEVAFF_EL1\[31\]
	pub struct F0V;
	impl RegisterView for F0V {
		type Register = super::SPMDEVAFF_EL1;
		const NAME: &'static str = "F0V";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// SPMDEVAFF_EL1\[30\]
	pub struct U;
	impl RegisterView for U {
		type Register = super::SPMDEVAFF_EL1;
		const NAME: &'static str = "U";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// SPMDEVAFF_EL1\[24\]
	pub struct MT;
	impl RegisterView for MT {
		type Register = super::SPMDEVAFF_EL1;
		const NAME: &'static str = "MT";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// SPMDEVAFF_EL1\[23:16\]
	pub struct Aff2;
	impl RegisterView for Aff2 {
		type Register = super::SPMDEVAFF_EL1;
		const NAME: &'static str = "Aff2";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// SPMDEVAFF_EL1\[15:8\]
	pub struct Aff1;
	impl RegisterView for Aff1 {
		type Register = super::SPMDEVAFF_EL1;
		const NAME: &'static str = "Aff1";
		const OFFSET: usize = 8;
		const LEN: usize = 8;
	}

	/// SPMDEVAFF_EL1\[7:0\]
	pub struct Aff0;
	impl RegisterView for Aff0 {
		type Register = super::SPMDEVAFF_EL1;
		const NAME: &'static str = "Aff0";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Trace Address Comparator Value Register \<n\>, n = 15 - 0
pub struct TRCACVRn;
impl RegisterArray for TRCACVRn {
	const NAME: &'static str = "TRCACVRn";
	const ELEMS: usize = 15;
	const ELEM_LEN: usize = 64;
}

/// TRCACVRn register fields
pub mod trcacvrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCACVRn\[63:0\]
	pub struct ADDRESS;
	impl RegisterArrayView for ADDRESS {
		type RegisterArray = super::TRCACVRn;
		const NAME: &'static str = "ADDRESS";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Stack Pointer (EL2)
#[allow(non_camel_case_types)]
pub struct SP_EL2;
impl Register for SP_EL2 {
	const NAME: &'static str = "SP_EL2";
	const LEN: usize = 64;
}

/// Pointer Authentication Key A for Data (bits\[127:64\]) 
#[allow(non_camel_case_types)]
pub struct APDAKeyHi_EL1;
impl Register for APDAKeyHi_EL1 {
	const NAME: &'static str = "APDAKeyHi_EL1";
	const LEN: usize = 64;
}

/// Debug Breakpoint Value Registers, n = 63 - 0
#[allow(non_camel_case_types)]
pub struct DBGBVRn_EL1;
impl RegisterArray for DBGBVRn_EL1 {
	const NAME: &'static str = "DBGBVRn_EL1";
	const ELEMS: usize = 63;
	const ELEM_LEN: usize = 64;
}

/// DBGBVRn_EL1 register fields
pub mod dbgbvrn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// DBGBVRn_EL1\[31:0\]
	pub struct ContextID;
	impl RegisterArrayView for ContextID {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "ContextID";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}

	/// DBGBVRn_EL1\[63:32\]
	pub struct ContextID2;
	impl RegisterArrayView for ContextID2 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "ContextID2";
		const OFFSET: usize = 32;
		const LEN: usize = 32;
	}

	/// DBGBVRn_EL1\[63:57\]
	#[allow(non_camel_case_types)]
	pub struct RESS_63_57;
	impl RegisterArrayView for RESS_63_57 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "RESS_63_57";
		const OFFSET: usize = 57;
		const LEN: usize = 7;
	}
	/// DBGBVRn_EL1\[56:53\]
	#[allow(non_camel_case_types)]
	pub struct RESS_63_57_56_53;
	impl RegisterArrayView for RESS_63_57_56_53 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "RESS_63_57_56_53";
		const OFFSET: usize = 53;
		const LEN: usize = 4;
	}
	/// DBGBVRn_EL1\[52:49\]
	#[allow(non_camel_case_types)]
	pub struct RESS_63_57_56_53_52_49;
	impl RegisterArrayView for RESS_63_57_56_53_52_49 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "RESS_63_57_56_53_52_49";
		const OFFSET: usize = 49;
		const LEN: usize = 4;
	}

	/// DBGBVRn_EL1\[56:53\]
	#[allow(non_camel_case_types)]
	pub struct VA_56_53;
	impl RegisterArrayView for VA_56_53 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "VA_56_53";
		const OFFSET: usize = 53;
		const LEN: usize = 4;
	}
	/// DBGBVRn_EL1\[52:49\]
	#[allow(non_camel_case_types)]
	pub struct VA_56_53_52_49;
	impl RegisterArrayView for VA_56_53_52_49 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "VA_56_53_52_49";
		const OFFSET: usize = 49;
		const LEN: usize = 4;
	}
	/// DBGBVRn_EL1\[48:2\]
	#[allow(non_camel_case_types)]
	pub struct VA_56_53_52_49_48_2;
	impl RegisterArrayView for VA_56_53_52_49_48_2 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "VA_56_53_52_49_48_2";
		const OFFSET: usize = 2;
		const LEN: usize = 47;
	}

	/// DBGBVRn_EL1\[47:40\]
	#[allow(non_camel_case_types)]
	pub struct VMID_47_40;
	impl RegisterArrayView for VMID_47_40 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "VMID_47_40";
		const OFFSET: usize = 40;
		const LEN: usize = 8;
	}
	/// DBGBVRn_EL1\[47:40\]
	#[allow(non_camel_case_types)]
	pub struct VMID_47_40_47_40;
	impl RegisterArrayView for VMID_47_40_47_40 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "VMID_47_40_47_40";
		const OFFSET: usize = 40;
		const LEN: usize = 8;
	}
	/// DBGBVRn_EL1\[39:32\]
	#[allow(non_camel_case_types)]
	pub struct VMID_47_40_47_40_39_32;
	impl RegisterArrayView for VMID_47_40_47_40_39_32 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "VMID_47_40_47_40_39_32";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}
	/// DBGBVRn_EL1\[39:32\]
	#[allow(non_camel_case_types)]
	pub struct VMID_47_40_47_40_39_32_39_32;
	impl RegisterArrayView for VMID_47_40_47_40_39_32_39_32 {
		type RegisterArray = super::DBGBVRn_EL1;
		const NAME: &'static str = "VMID_47_40_47_40_39_32_39_32";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}
}

/// Counter-timer Secure Virtual Timer Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHVS_CTL_EL2;
impl Register for CNTHVS_CTL_EL2 {
	const NAME: &'static str = "CNTHVS_CTL_EL2";
	const LEN: usize = 64;
}

/// CNTHVS_CTL_EL2 register fields
pub mod cnthvs_ctl_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHVS_CTL_EL2\[2\]
	pub struct ISTATUS;
	impl RegisterView for ISTATUS {
		type Register = super::CNTHVS_CTL_EL2;
		const NAME: &'static str = "ISTATUS";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTHVS_CTL_EL2\[1\]
	pub struct IMASK;
	impl RegisterView for IMASK {
		type Register = super::CNTHVS_CTL_EL2;
		const NAME: &'static str = "IMASK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTHVS_CTL_EL2\[0\]
	pub struct ENABLE;
	impl RegisterView for ENABLE {
		type Register = super::CNTHVS_CTL_EL2;
		const NAME: &'static str = "ENABLE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Permission Indirection Register 2 (EL2)
#[allow(non_camel_case_types)]
pub struct PIR_EL2;
impl Register for PIR_EL2 {
	const NAME: &'static str = "PIR_EL2";
	const LEN: usize = 64;
}

///  Permission Indirection Register 3 (EL3)
#[allow(non_camel_case_types)]
pub struct PIR_EL3;
impl Register for PIR_EL3 {
	const NAME: &'static str = "PIR_EL3";
	const LEN: usize = 64;
}

/// Realm PA space Alternate MECID for EL3 stage 1 translation regime
#[allow(non_camel_case_types)]
pub struct MECID_RL_A_EL3;
impl Register for MECID_RL_A_EL3 {
	const NAME: &'static str = "MECID_RL_A_EL3";
	const LEN: usize = 64;
}

/// MECID_RL_A_EL3 register fields
pub mod mecid_rl_a_el3 {
	use crate::core::model::proc::RegisterView;

	/// MECID_RL_A_EL3\[15:0\]
	pub struct MECID;
	impl RegisterView for MECID {
		type Register = super::MECID_RL_A_EL3;
		const NAME: &'static str = "MECID";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Interrupt Controller List Registers, n = 15 - 0
#[allow(non_camel_case_types)]
pub struct ICH_LRn_EL2;
impl RegisterArray for ICH_LRn_EL2 {
	const NAME: &'static str = "ICH_LRn_EL2";
	const ELEMS: usize = 15;
	const ELEM_LEN: usize = 64;
}

/// ICH_LRn_EL2 register fields
pub mod ich_lrn_el2 {
	use crate::core::model::proc::RegisterArrayView;

	/// ICH_LRn_EL2\[63:62\]
	pub struct State;
	impl RegisterArrayView for State {
		type RegisterArray = super::ICH_LRn_EL2;
		const NAME: &'static str = "State";
		const OFFSET: usize = 62;
		const LEN: usize = 2;
	}

	/// ICH_LRn_EL2\[61\]
	pub struct HW;
	impl RegisterArrayView for HW {
		type RegisterArray = super::ICH_LRn_EL2;
		const NAME: &'static str = "HW";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// ICH_LRn_EL2\[60\]
	pub struct Group;
	impl RegisterArrayView for Group {
		type RegisterArray = super::ICH_LRn_EL2;
		const NAME: &'static str = "Group";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// ICH_LRn_EL2\[59\]
	pub struct NMI;
	impl RegisterArrayView for NMI {
		type RegisterArray = super::ICH_LRn_EL2;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// ICH_LRn_EL2\[55:48\]
	pub struct Priority;
	impl RegisterArrayView for Priority {
		type RegisterArray = super::ICH_LRn_EL2;
		const NAME: &'static str = "Priority";
		const OFFSET: usize = 48;
		const LEN: usize = 8;
	}

	/// ICH_LRn_EL2\[44:32\]
	#[allow(non_camel_case_types)]
	pub struct pINTID;
	impl RegisterArrayView for pINTID {
		type RegisterArray = super::ICH_LRn_EL2;
		const NAME: &'static str = "pINTID";
		const OFFSET: usize = 32;
		const LEN: usize = 13;
	}

	/// ICH_LRn_EL2\[31:0\]
	#[allow(non_camel_case_types)]
	pub struct vINTID;
	impl RegisterArrayView for vINTID {
		type RegisterArray = super::ICH_LRn_EL2;
		const NAME: &'static str = "vINTID";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Performance Monitors Count Enable Set Register
#[allow(non_camel_case_types)]
pub struct PMCNTENSET_EL0;
impl Register for PMCNTENSET_EL0 {
	const NAME: &'static str = "PMCNTENSET_EL0";
	const LEN: usize = 64;
}

/// PMCNTENSET_EL0 register fields
pub mod pmcntenset_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMCNTENSET_EL0\[31\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMCNTENSET_EL0;
		const NAME: &'static str = "C";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
}

/// Stage 2 Permission Overlay Register (EL1)
#[allow(non_camel_case_types)]
pub struct S2POR_EL1;
impl Register for S2POR_EL1 {
	const NAME: &'static str = "S2POR_EL1";
	const LEN: usize = 64;
}

/// Stack Pointer (EL3)
#[allow(non_camel_case_types)]
pub struct SP_EL3;
impl Register for SP_EL3 {
	const NAME: &'static str = "SP_EL3";
	const LEN: usize = 64;
}

/// System Performance Monitors Event Filter Control Register 2, n = 63 - 0
#[allow(non_camel_case_types)]
pub struct SPMEVFILT2Rn_EL0;
impl RegisterArray for SPMEVFILT2Rn_EL0 {
	const NAME: &'static str = "SPMEVFILT2Rn_EL0";
	const ELEMS: usize = 63;
	const ELEM_LEN: usize = 64;
}

/// Hypervisor Configuration Register
#[allow(non_camel_case_types)]
pub struct HCR_EL2;
impl Register for HCR_EL2 {
	const NAME: &'static str = "HCR_EL2";
	const LEN: usize = 64;
}

/// HCR_EL2 register fields
pub mod hcr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HCR_EL2\[63:60\]
	pub struct TWEDEL;
	impl RegisterView for TWEDEL {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TWEDEL";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// HCR_EL2\[59\]
	pub struct TWEDEn;
	impl RegisterView for TWEDEn {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TWEDEn";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[58\]
	pub struct TID5;
	impl RegisterView for TID5 {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TID5";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[57\]
	pub struct DCT;
	impl RegisterView for DCT {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "DCT";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[56\]
	pub struct ATA;
	impl RegisterView for ATA {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "ATA";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[55\]
	pub struct TTLBOS;
	impl RegisterView for TTLBOS {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TTLBOS";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[54\]
	pub struct TTLBIS;
	impl RegisterView for TTLBIS {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TTLBIS";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[53\]
	pub struct EnSCXT;
	impl RegisterView for EnSCXT {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "EnSCXT";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[52\]
	pub struct TOCU;
	impl RegisterView for TOCU {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TOCU";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[51\]
	pub struct AMVOFFEN;
	impl RegisterView for AMVOFFEN {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "AMVOFFEN";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[50\]
	pub struct TICAB;
	impl RegisterView for TICAB {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TICAB";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[49\]
	pub struct TID4;
	impl RegisterView for TID4 {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TID4";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[48\]
	pub struct GPF;
	impl RegisterView for GPF {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "GPF";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[47\]
	pub struct FIEN;
	impl RegisterView for FIEN {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "FIEN";
		const OFFSET: usize = 47;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[46\]
	pub struct FWB;
	impl RegisterView for FWB {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "FWB";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[45\]
	pub struct NV2;
	impl RegisterView for NV2 {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "NV2";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[44\]
	pub struct AT;
	impl RegisterView for AT {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "AT";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[43\]
	pub struct NV1;
	impl RegisterView for NV1 {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "NV1";
		const OFFSET: usize = 43;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[42\]
	pub struct NV;
	impl RegisterView for NV {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "NV";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[41\]
	pub struct API;
	impl RegisterView for API {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "API";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[40\]
	pub struct APK;
	impl RegisterView for APK {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "APK";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[39\]
	pub struct TME;
	impl RegisterView for TME {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TME";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[38\]
	pub struct MIOCNCE;
	impl RegisterView for MIOCNCE {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "MIOCNCE";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[37\]
	pub struct TEA;
	impl RegisterView for TEA {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TEA";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[36\]
	pub struct TERR;
	impl RegisterView for TERR {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TERR";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[35\]
	pub struct TLOR;
	impl RegisterView for TLOR {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TLOR";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[34\]
	pub struct E2H;
	impl RegisterView for E2H {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "E2H";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[33\]
	pub struct ID;
	impl RegisterView for ID {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "ID";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[32\]
	pub struct CD;
	impl RegisterView for CD {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "CD";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[31\]
	pub struct RW;
	impl RegisterView for RW {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "RW";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[30\]
	pub struct TRVM;
	impl RegisterView for TRVM {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TRVM";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[29\]
	pub struct HCD;
	impl RegisterView for HCD {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "HCD";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[28\]
	pub struct TDZ;
	impl RegisterView for TDZ {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TDZ";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[27\]
	pub struct TGE;
	impl RegisterView for TGE {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TGE";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[26\]
	pub struct TVM;
	impl RegisterView for TVM {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TVM";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[25\]
	pub struct TTLB;
	impl RegisterView for TTLB {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TTLB";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[24\]
	pub struct TPU;
	impl RegisterView for TPU {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TPU";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[23\]
	pub struct TPCP;
	impl RegisterView for TPCP {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TPCP";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[23\]
	pub struct TPC;
	impl RegisterView for TPC {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TPC";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[22\]
	pub struct TSW;
	impl RegisterView for TSW {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TSW";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[21\]
	pub struct TACR;
	impl RegisterView for TACR {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TACR";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[20\]
	pub struct TIDCP;
	impl RegisterView for TIDCP {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TIDCP";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[19\]
	pub struct TSC;
	impl RegisterView for TSC {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TSC";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[18\]
	pub struct TID3;
	impl RegisterView for TID3 {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TID3";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[17\]
	pub struct TID2;
	impl RegisterView for TID2 {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TID2";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[16\]
	pub struct TID1;
	impl RegisterView for TID1 {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TID1";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[15\]
	pub struct TID0;
	impl RegisterView for TID0 {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TID0";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[14\]
	pub struct TWE;
	impl RegisterView for TWE {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TWE";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[13\]
	pub struct TWI;
	impl RegisterView for TWI {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "TWI";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[12\]
	pub struct DC;
	impl RegisterView for DC {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "DC";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[11:10\]
	pub struct BSU;
	impl RegisterView for BSU {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "BSU";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// HCR_EL2\[9\]
	pub struct FB;
	impl RegisterView for FB {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "FB";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[8\]
	pub struct VSE;
	impl RegisterView for VSE {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "VSE";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[7\]
	pub struct VI;
	impl RegisterView for VI {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "VI";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[6\]
	pub struct VF;
	impl RegisterView for VF {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "VF";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[5\]
	pub struct AMO;
	impl RegisterView for AMO {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "AMO";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[4\]
	pub struct IMO;
	impl RegisterView for IMO {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "IMO";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[3\]
	pub struct FMO;
	impl RegisterView for FMO {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "FMO";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[2\]
	pub struct PTW;
	impl RegisterView for PTW {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "PTW";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[1\]
	pub struct SWIO;
	impl RegisterView for SWIO {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "SWIO";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HCR_EL2\[0\]
	pub struct VM;
	impl RegisterView for VM {
		type Register = super::HCR_EL2;
		const NAME: &'static str = "VM";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Virtual SError Exception Syndrome Register (EL3)
#[allow(non_camel_case_types)]
pub struct VSESR_EL3;
impl Register for VSESR_EL3 {
	const NAME: &'static str = "VSESR_EL3";
	const LEN: usize = 64;
}

/// VSESR_EL3 register fields
pub mod vsesr_el3 {
	use crate::core::model::proc::RegisterView;

	/// VSESR_EL3\[24\]
	pub struct IDS;
	impl RegisterView for IDS {
		type Register = super::VSESR_EL3;
		const NAME: &'static str = "IDS";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// VSESR_EL3\[23:0\]
	pub struct ISS;
	impl RegisterView for ISS {
		type Register = super::VSESR_EL3;
		const NAME: &'static str = "ISS";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Auxiliary Fault Status Register 1 (EL3)
#[allow(non_camel_case_types)]
pub struct AFSR1_EL3;
impl Register for AFSR1_EL3 {
	const NAME: &'static str = "AFSR1_EL3";
	const LEN: usize = 64;
}

/// AArch32 Instruction Set Attribute Register 5
#[allow(non_camel_case_types)]
pub struct ID_ISAR5_EL1;
impl Register for ID_ISAR5_EL1 {
	const NAME: &'static str = "ID_ISAR5_EL1";
	const LEN: usize = 64;
}

/// ID_ISAR5_EL1 register fields
pub mod id_isar5_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_ISAR5_EL1\[31:28\]
	pub struct VCMA;
	impl RegisterView for VCMA {
		type Register = super::ID_ISAR5_EL1;
		const NAME: &'static str = "VCMA";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_ISAR5_EL1\[27:24\]
	pub struct RDM;
	impl RegisterView for RDM {
		type Register = super::ID_ISAR5_EL1;
		const NAME: &'static str = "RDM";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_ISAR5_EL1\[19:16\]
	pub struct CRC32;
	impl RegisterView for CRC32 {
		type Register = super::ID_ISAR5_EL1;
		const NAME: &'static str = "CRC32";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_ISAR5_EL1\[15:12\]
	pub struct SHA2;
	impl RegisterView for SHA2 {
		type Register = super::ID_ISAR5_EL1;
		const NAME: &'static str = "SHA2";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_ISAR5_EL1\[11:8\]
	pub struct SHA1;
	impl RegisterView for SHA1 {
		type Register = super::ID_ISAR5_EL1;
		const NAME: &'static str = "SHA1";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_ISAR5_EL1\[7:4\]
	pub struct AES;
	impl RegisterView for AES {
		type Register = super::ID_ISAR5_EL1;
		const NAME: &'static str = "AES";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_ISAR5_EL1\[3:0\]
	pub struct SEVL;
	impl RegisterView for SEVL {
		type Register = super::ID_ISAR5_EL1;
		const NAME: &'static str = "SEVL";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Trace Programming Control Register
pub struct TRCPRGCTLR;
impl Register for TRCPRGCTLR {
	const NAME: &'static str = "TRCPRGCTLR";
	const LEN: usize = 64;
}

/// TRCPRGCTLR register fields
pub mod trcprgctlr {
	use crate::core::model::proc::RegisterView;

	/// TRCPRGCTLR\[0\]
	pub struct EN;
	impl RegisterView for EN {
		type Register = super::TRCPRGCTLR;
		const NAME: &'static str = "EN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// AArch64 Instruction Set Attribute Register 1
#[allow(non_camel_case_types)]
pub struct ID_AA64ISAR1_EL1;
impl Register for ID_AA64ISAR1_EL1 {
	const NAME: &'static str = "ID_AA64ISAR1_EL1";
	const LEN: usize = 64;
}

/// ID_AA64ISAR1_EL1 register fields
pub mod id_aa64isar1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64ISAR1_EL1\[63:60\]
	pub struct LS64;
	impl RegisterView for LS64 {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "LS64";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[59:56\]
	pub struct XS;
	impl RegisterView for XS {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "XS";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[55:52\]
	pub struct I8MM;
	impl RegisterView for I8MM {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "I8MM";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[51:48\]
	pub struct DGH;
	impl RegisterView for DGH {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "DGH";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[47:44\]
	pub struct BF16;
	impl RegisterView for BF16 {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "BF16";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[43:40\]
	pub struct SPECRES;
	impl RegisterView for SPECRES {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "SPECRES";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[39:36\]
	pub struct SB;
	impl RegisterView for SB {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "SB";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[35:32\]
	pub struct FRINTTS;
	impl RegisterView for FRINTTS {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "FRINTTS";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[31:28\]
	pub struct GPI;
	impl RegisterView for GPI {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "GPI";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[27:24\]
	pub struct GPA;
	impl RegisterView for GPA {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "GPA";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[23:20\]
	pub struct LRCPC;
	impl RegisterView for LRCPC {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "LRCPC";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[19:16\]
	pub struct FCMA;
	impl RegisterView for FCMA {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "FCMA";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[15:12\]
	pub struct JSCVT;
	impl RegisterView for JSCVT {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "JSCVT";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[11:8\]
	pub struct API;
	impl RegisterView for API {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "API";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[7:4\]
	pub struct APA;
	impl RegisterView for APA {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "APA";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR1_EL1\[3:0\]
	pub struct DPB;
	impl RegisterView for DPB {
		type Register = super::ID_AA64ISAR1_EL1;
		const NAME: &'static str = "DPB";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Monitor DCC Interrupt Enable Register
#[allow(non_camel_case_types)]
pub struct MDCCINT_EL1;
impl Register for MDCCINT_EL1 {
	const NAME: &'static str = "MDCCINT_EL1";
	const LEN: usize = 64;
}

/// MDCCINT_EL1 register fields
pub mod mdccint_el1 {
	use crate::core::model::proc::RegisterView;

	/// MDCCINT_EL1\[30\]
	pub struct RX;
	impl RegisterView for RX {
		type Register = super::MDCCINT_EL1;
		const NAME: &'static str = "RX";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// MDCCINT_EL1\[29\]
	pub struct TX;
	impl RegisterView for TX {
		type Register = super::MDCCINT_EL1;
		const NAME: &'static str = "TX";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}
}

/// Debug Breakpoint Control Registers, n = 63 - 0
#[allow(non_camel_case_types)]
pub struct DBGBCRn_EL1;
impl RegisterArray for DBGBCRn_EL1 {
	const NAME: &'static str = "DBGBCRn_EL1";
	const ELEMS: usize = 63;
	const ELEM_LEN: usize = 64;
}

/// DBGBCRn_EL1 register fields
pub mod dbgbcrn_el1 {
	use crate::core::model::proc::RegisterArrayView;

	/// DBGBCRn_EL1\[31:30\]
	pub struct LBNX;
	impl RegisterArrayView for LBNX {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "LBNX";
		const OFFSET: usize = 30;
		const LEN: usize = 2;
	}

	/// DBGBCRn_EL1\[29\]
	pub struct SSCE;
	impl RegisterArrayView for SSCE {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "SSCE";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// DBGBCRn_EL1\[28:24\]
	pub struct MASK;
	impl RegisterArrayView for MASK {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "MASK";
		const OFFSET: usize = 24;
		const LEN: usize = 5;
	}

	/// DBGBCRn_EL1\[23:20\]
	pub struct BT;
	impl RegisterArrayView for BT {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "BT";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// DBGBCRn_EL1\[19:16\]
	pub struct LBN;
	impl RegisterArrayView for LBN {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "LBN";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// DBGBCRn_EL1\[15:14\]
	pub struct SSC;
	impl RegisterArrayView for SSC {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "SSC";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// DBGBCRn_EL1\[13\]
	pub struct HMC;
	impl RegisterArrayView for HMC {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "HMC";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// DBGBCRn_EL1\[8:5\]
	pub struct BAS;
	impl RegisterArrayView for BAS {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "BAS";
		const OFFSET: usize = 5;
		const LEN: usize = 4;
	}

	/// DBGBCRn_EL1\[3\]
	pub struct BT2;
	impl RegisterArrayView for BT2 {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "BT2";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// DBGBCRn_EL1\[2:1\]
	pub struct PMC;
	impl RegisterArrayView for PMC {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "PMC";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// DBGBCRn_EL1\[0\]
	pub struct E;
	impl RegisterArrayView for E {
		type RegisterArray = super::DBGBCRn_EL1;
		const NAME: &'static str = "E";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Claim Tag Clear Register
pub struct TRCCLAIMCLR;
impl Register for TRCCLAIMCLR {
	const NAME: &'static str = "TRCCLAIMCLR";
	const LEN: usize = 64;
}

/// Trace Resource Selection Control Register \<n\>, n = 31 - 2
pub struct TRCRSCTLRn;
impl RegisterArray for TRCRSCTLRn {
	const NAME: &'static str = "TRCRSCTLRn";
	const ELEMS: usize = 29;
	const ELEM_LEN: usize = 64;
}

/// TRCRSCTLRn register fields
pub mod trcrsctlrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCRSCTLRn\[21\]
	pub struct PAIRINV;
	impl RegisterArrayView for PAIRINV {
		type RegisterArray = super::TRCRSCTLRn;
		const NAME: &'static str = "PAIRINV";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// TRCRSCTLRn\[20\]
	pub struct INV;
	impl RegisterArrayView for INV {
		type RegisterArray = super::TRCRSCTLRn;
		const NAME: &'static str = "INV";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// TRCRSCTLRn\[19:16\]
	pub struct GROUP;
	impl RegisterArrayView for GROUP {
		type RegisterArray = super::TRCRSCTLRn;
		const NAME: &'static str = "GROUP";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// TRCRSCTLRn\[15:0\]
	pub struct SELECT;
	impl RegisterArrayView for SELECT {
		type RegisterArray = super::TRCRSCTLRn;
		const NAME: &'static str = "SELECT";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// System Performance Monitors Overflow Flag Status Set Register
#[allow(non_camel_case_types)]
pub struct SPMOVSSET_EL0;
impl Register for SPMOVSSET_EL0 {
	const NAME: &'static str = "SPMOVSSET_EL0";
	const LEN: usize = 64;
}

/// Performance Monitors User Enable Register
#[allow(non_camel_case_types)]
pub struct PMUSERENR_EL0;
impl Register for PMUSERENR_EL0 {
	const NAME: &'static str = "PMUSERENR_EL0";
	const LEN: usize = 64;
}

/// PMUSERENR_EL0 register fields
pub mod pmuserenr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMUSERENR_EL0\[6\]
	pub struct TID;
	impl RegisterView for TID {
		type Register = super::PMUSERENR_EL0;
		const NAME: &'static str = "TID";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// PMUSERENR_EL0\[5\]
	pub struct IR;
	impl RegisterView for IR {
		type Register = super::PMUSERENR_EL0;
		const NAME: &'static str = "IR";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// PMUSERENR_EL0\[4\]
	pub struct UEN;
	impl RegisterView for UEN {
		type Register = super::PMUSERENR_EL0;
		const NAME: &'static str = "UEN";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// PMUSERENR_EL0\[3\]
	pub struct ER;
	impl RegisterView for ER {
		type Register = super::PMUSERENR_EL0;
		const NAME: &'static str = "ER";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// PMUSERENR_EL0\[2\]
	pub struct CR;
	impl RegisterView for CR {
		type Register = super::PMUSERENR_EL0;
		const NAME: &'static str = "CR";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// PMUSERENR_EL0\[1\]
	pub struct SW;
	impl RegisterView for SW {
		type Register = super::PMUSERENR_EL0;
		const NAME: &'static str = "SW";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// PMUSERENR_EL0\[0\]
	pub struct EN;
	impl RegisterView for EN {
		type Register = super::PMUSERENR_EL0;
		const NAME: &'static str = "EN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Context ID Register (EL2)
#[allow(non_camel_case_types)]
pub struct CONTEXTIDR_EL2;
impl Register for CONTEXTIDR_EL2 {
	const NAME: &'static str = "CONTEXTIDR_EL2";
	const LEN: usize = 64;
}

/// CONTEXTIDR_EL2 register fields
pub mod contextidr_el2 {
	use crate::core::model::proc::RegisterView;

	/// CONTEXTIDR_EL2\[31:0\]
	pub struct PROCID;
	impl RegisterView for PROCID {
		type Register = super::CONTEXTIDR_EL2;
		const NAME: &'static str = "PROCID";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Counter-timer Physical Secure Timer CompareValue Register
#[allow(non_camel_case_types)]
pub struct CNTPS_CVAL_EL1;
impl Register for CNTPS_CVAL_EL1 {
	const NAME: &'static str = "CNTPS_CVAL_EL1";
	const LEN: usize = 64;
}

/// CNTPS_CVAL_EL1 register fields
pub mod cntps_cval_el1 {
	use crate::core::model::proc::RegisterView;

	/// CNTPS_CVAL_EL1\[63:0\]
	pub struct CompareValue;
	impl RegisterView for CompareValue {
		type Register = super::CNTPS_CVAL_EL1;
		const NAME: &'static str = "CompareValue";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Trace Claim Tag Set Register
pub struct TRCCLAIMSET;
impl Register for TRCCLAIMSET {
	const NAME: &'static str = "TRCCLAIMSET";
	const LEN: usize = 64;
}

/// Floating-point Control Register
pub struct FPCR;
impl Register for FPCR {
	const NAME: &'static str = "FPCR";
	const LEN: usize = 64;
}

/// FPCR register fields
pub mod fpcr {
	use crate::core::model::proc::RegisterView;

	/// FPCR\[26\]
	pub struct AHP;
	impl RegisterView for AHP {
		type Register = super::FPCR;
		const NAME: &'static str = "AHP";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// FPCR\[25\]
	pub struct DN;
	impl RegisterView for DN {
		type Register = super::FPCR;
		const NAME: &'static str = "DN";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// FPCR\[24\]
	pub struct FZ;
	impl RegisterView for FZ {
		type Register = super::FPCR;
		const NAME: &'static str = "FZ";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// FPCR\[23:22\]
	pub struct RMode;
	impl RegisterView for RMode {
		type Register = super::FPCR;
		const NAME: &'static str = "RMode";
		const OFFSET: usize = 22;
		const LEN: usize = 2;
	}

	/// FPCR\[21:20\]
	pub struct Stride;
	impl RegisterView for Stride {
		type Register = super::FPCR;
		const NAME: &'static str = "Stride";
		const OFFSET: usize = 20;
		const LEN: usize = 2;
	}

	/// FPCR\[19\]
	pub struct FZ16;
	impl RegisterView for FZ16 {
		type Register = super::FPCR;
		const NAME: &'static str = "FZ16";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// FPCR\[18:16\]
	pub struct Len;
	impl RegisterView for Len {
		type Register = super::FPCR;
		const NAME: &'static str = "Len";
		const OFFSET: usize = 16;
		const LEN: usize = 3;
	}

	/// FPCR\[15\]
	pub struct IDE;
	impl RegisterView for IDE {
		type Register = super::FPCR;
		const NAME: &'static str = "IDE";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// FPCR\[13\]
	pub struct EBF;
	impl RegisterView for EBF {
		type Register = super::FPCR;
		const NAME: &'static str = "EBF";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// FPCR\[12\]
	pub struct IXE;
	impl RegisterView for IXE {
		type Register = super::FPCR;
		const NAME: &'static str = "IXE";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// FPCR\[11\]
	pub struct UFE;
	impl RegisterView for UFE {
		type Register = super::FPCR;
		const NAME: &'static str = "UFE";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// FPCR\[10\]
	pub struct OFE;
	impl RegisterView for OFE {
		type Register = super::FPCR;
		const NAME: &'static str = "OFE";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// FPCR\[9\]
	pub struct DZE;
	impl RegisterView for DZE {
		type Register = super::FPCR;
		const NAME: &'static str = "DZE";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// FPCR\[8\]
	pub struct IOE;
	impl RegisterView for IOE {
		type Register = super::FPCR;
		const NAME: &'static str = "IOE";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// FPCR\[2\]
	pub struct NEP;
	impl RegisterView for NEP {
		type Register = super::FPCR;
		const NAME: &'static str = "NEP";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// FPCR\[1\]
	pub struct AH;
	impl RegisterView for AH {
		type Register = super::FPCR;
		const NAME: &'static str = "AH";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// FPCR\[0\]
	pub struct FIZ;
	impl RegisterView for FIZ {
		type Register = super::FPCR;
		const NAME: &'static str = "FIZ";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Auxiliary Fault Status Register 1 (EL1)
#[allow(non_camel_case_types)]
pub struct AFSR1_EL1;
impl Register for AFSR1_EL1 {
	const NAME: &'static str = "AFSR1_EL1";
	const LEN: usize = 64;
}

/// Counter-timer Self-Synchronized Physical Count Register
#[allow(non_camel_case_types)]
pub struct CNTPCTSS_EL0;
impl Register for CNTPCTSS_EL0 {
	const NAME: &'static str = "CNTPCTSS_EL0";
	const LEN: usize = 64;
}

/// Counter-timer Secure Virtual Timer TimerValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHVS_TVAL_EL2;
impl Register for CNTHVS_TVAL_EL2 {
	const NAME: &'static str = "CNTHVS_TVAL_EL2";
	const LEN: usize = 64;
}

/// CNTHVS_TVAL_EL2 register fields
pub mod cnthvs_tval_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHVS_TVAL_EL2\[31:0\]
	pub struct TimerValue;
	impl RegisterView for TimerValue {
		type Register = super::CNTHVS_TVAL_EL2;
		const NAME: &'static str = "TimerValue";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Stack Pointer (EL1)
#[allow(non_camel_case_types)]
pub struct SP_EL1;
impl Register for SP_EL1 {
	const NAME: &'static str = "SP_EL1";
	const LEN: usize = 64;
}

/// Trace Buffer Write Pointer Register
#[allow(non_camel_case_types)]
pub struct TRBPTR_EL1;
impl Register for TRBPTR_EL1 {
	const NAME: &'static str = "TRBPTR_EL1";
	const LEN: usize = 64;
}

/// TRBPTR_EL1 register fields
pub mod trbptr_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRBPTR_EL1\[63:0\]
	pub struct PTR;
	impl RegisterView for PTR {
		type Register = super::TRBPTR_EL1;
		const NAME: &'static str = "PTR";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Cache Level ID Register
#[allow(non_camel_case_types)]
pub struct CLIDR_EL1;
impl Register for CLIDR_EL1 {
	const NAME: &'static str = "CLIDR_EL1";
	const LEN: usize = 64;
}

/// CLIDR_EL1 register fields
pub mod clidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// CLIDR_EL1\[32:30\]
	pub struct ICB;
	impl RegisterView for ICB {
		type Register = super::CLIDR_EL1;
		const NAME: &'static str = "ICB";
		const OFFSET: usize = 30;
		const LEN: usize = 3;
	}

	/// CLIDR_EL1\[29:27\]
	pub struct LoUU;
	impl RegisterView for LoUU {
		type Register = super::CLIDR_EL1;
		const NAME: &'static str = "LoUU";
		const OFFSET: usize = 27;
		const LEN: usize = 3;
	}

	/// CLIDR_EL1\[26:24\]
	pub struct LoC;
	impl RegisterView for LoC {
		type Register = super::CLIDR_EL1;
		const NAME: &'static str = "LoC";
		const OFFSET: usize = 24;
		const LEN: usize = 3;
	}

	/// CLIDR_EL1\[23:21\]
	pub struct LoUIS;
	impl RegisterView for LoUIS {
		type Register = super::CLIDR_EL1;
		const NAME: &'static str = "LoUIS";
		const OFFSET: usize = 21;
		const LEN: usize = 3;
	}
}

/// Permission Indirection Register 1 (EL1)
#[allow(non_camel_case_types)]
pub struct PIR_EL1;
impl Register for PIR_EL1 {
	const NAME: &'static str = "PIR_EL1";
	const LEN: usize = 64;
}

/// Trace ViewInst Start/Stop Control Register
pub struct TRCVISSCTLR;
impl Register for TRCVISSCTLR {
	const NAME: &'static str = "TRCVISSCTLR";
	const LEN: usize = 64;
}

/// Random Number
pub struct RNDR;
impl Register for RNDR {
	const NAME: &'static str = "RNDR";
	const LEN: usize = 64;
}

/// RNDR register fields
pub mod rndr {
	use crate::core::model::proc::RegisterView;

	/// RNDR\[63:0\]
	pub struct RNDR;
	impl RegisterView for RNDR {
		type Register = super::RNDR;
		const NAME: &'static str = "RNDR";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// AArch64 Memory Model Feature Register 4
#[allow(non_camel_case_types)]
pub struct ID_AA64MMFR4_EL1;
impl Register for ID_AA64MMFR4_EL1 {
	const NAME: &'static str = "ID_AA64MMFR4_EL1";
	const LEN: usize = 64;
}

/// ID_AA64MMFR4_EL1 register fields
pub mod id_aa64mmfr4_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64MMFR4_EL1\[39:36\]
	pub struct E3DSE;
	impl RegisterView for E3DSE {
		type Register = super::ID_AA64MMFR4_EL1;
		const NAME: &'static str = "E3DSE";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR4_EL1\[27:24\]
	pub struct E2H0;
	impl RegisterView for E2H0 {
		type Register = super::ID_AA64MMFR4_EL1;
		const NAME: &'static str = "E2H0";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR4_EL1\[23:20\]
	#[allow(non_camel_case_types)]
	pub struct NV_frac;
	impl RegisterView for NV_frac {
		type Register = super::ID_AA64MMFR4_EL1;
		const NAME: &'static str = "NV_frac";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR4_EL1\[19:16\]
	pub struct FGWTE3;
	impl RegisterView for FGWTE3 {
		type Register = super::ID_AA64MMFR4_EL1;
		const NAME: &'static str = "FGWTE3";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR4_EL1\[15:12\]
	pub struct HACDBS;
	impl RegisterView for HACDBS {
		type Register = super::ID_AA64MMFR4_EL1;
		const NAME: &'static str = "HACDBS";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR4_EL1\[11:8\]
	pub struct ASID2;
	impl RegisterView for ASID2 {
		type Register = super::ID_AA64MMFR4_EL1;
		const NAME: &'static str = "ASID2";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64MMFR4_EL1\[7:4\]
	pub struct EIESB;
	impl RegisterView for EIESB {
		type Register = super::ID_AA64MMFR4_EL1;
		const NAME: &'static str = "EIESB";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}
}

/// Deferred Interrupt Status Register
#[allow(non_camel_case_types)]
pub struct DISR_EL1;
impl Register for DISR_EL1 {
	const NAME: &'static str = "DISR_EL1";
	const LEN: usize = 64;
}

/// DISR_EL1 register fields
pub mod disr_el1 {
	use crate::core::model::proc::RegisterView;

	/// DISR_EL1\[31\]
	pub struct A;
	impl RegisterView for A {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "A";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// DISR_EL1\[24\]
	pub struct IDS;
	impl RegisterView for IDS {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "IDS";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// DISR_EL1\[17:16\]
	pub struct WU;
	impl RegisterView for WU {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "WU";
		const OFFSET: usize = 16;
		const LEN: usize = 2;
	}

	/// DISR_EL1\[12:10\]
	pub struct AET;
	impl RegisterView for AET {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "AET";
		const OFFSET: usize = 10;
		const LEN: usize = 3;
	}

	/// DISR_EL1\[9\]
	pub struct EA;
	impl RegisterView for EA {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "EA";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// DISR_EL1\[7\]
	pub struct WnRV;
	impl RegisterView for WnRV {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "WnRV";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// DISR_EL1\[6\]
	pub struct WnR;
	impl RegisterView for WnR {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "WnR";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// DISR_EL1\[5:0\]
	pub struct DFSC;
	impl RegisterView for DFSC {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "DFSC";
		const OFFSET: usize = 0;
		const LEN: usize = 6;
	}

	/// DISR_EL1\[23:0\]
	pub struct ISS;
	impl RegisterView for ISS {
		type Register = super::DISR_EL1;
		const NAME: &'static str = "ISS";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// AArch32 Memory Model Feature Register 0
#[allow(non_camel_case_types)]
pub struct ID_MMFR0_EL1;
impl Register for ID_MMFR0_EL1 {
	const NAME: &'static str = "ID_MMFR0_EL1";
	const LEN: usize = 64;
}

/// ID_MMFR0_EL1 register fields
pub mod id_mmfr0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_MMFR0_EL1\[31:28\]
	pub struct InnerShr;
	impl RegisterView for InnerShr {
		type Register = super::ID_MMFR0_EL1;
		const NAME: &'static str = "InnerShr";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_MMFR0_EL1\[27:24\]
	pub struct FCSE;
	impl RegisterView for FCSE {
		type Register = super::ID_MMFR0_EL1;
		const NAME: &'static str = "FCSE";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_MMFR0_EL1\[23:20\]
	pub struct AuxReg;
	impl RegisterView for AuxReg {
		type Register = super::ID_MMFR0_EL1;
		const NAME: &'static str = "AuxReg";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_MMFR0_EL1\[19:16\]
	pub struct TCM;
	impl RegisterView for TCM {
		type Register = super::ID_MMFR0_EL1;
		const NAME: &'static str = "TCM";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_MMFR0_EL1\[15:12\]
	pub struct ShareLvl;
	impl RegisterView for ShareLvl {
		type Register = super::ID_MMFR0_EL1;
		const NAME: &'static str = "ShareLvl";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_MMFR0_EL1\[11:8\]
	pub struct OuterShr;
	impl RegisterView for OuterShr {
		type Register = super::ID_MMFR0_EL1;
		const NAME: &'static str = "OuterShr";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_MMFR0_EL1\[7:4\]
	pub struct PMSA;
	impl RegisterView for PMSA {
		type Register = super::ID_MMFR0_EL1;
		const NAME: &'static str = "PMSA";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_MMFR0_EL1\[3:0\]
	pub struct VMSA;
	impl RegisterView for VMSA {
		type Register = super::ID_MMFR0_EL1;
		const NAME: &'static str = "VMSA";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Stack Pointer (EL0)
#[allow(non_camel_case_types)]
pub struct SP_EL0;
impl Register for SP_EL0 {
	const NAME: &'static str = "SP_EL0";
	const LEN: usize = 64;
}

/// Counter-timer Physical Timer TimerValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHP_TVAL_EL2;
impl Register for CNTHP_TVAL_EL2 {
	const NAME: &'static str = "CNTHP_TVAL_EL2";
	const LEN: usize = 64;
}

/// CNTHP_TVAL_EL2 register fields
pub mod cnthp_tval_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHP_TVAL_EL2\[31:0\]
	pub struct TimerValue;
	impl RegisterView for TimerValue {
		type Register = super::CNTHP_TVAL_EL2;
		const NAME: &'static str = "TimerValue";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// System PMU Counter Group Configuration Register \<n\>, n = 1 - 0
#[allow(non_camel_case_types)]
pub struct SPMCGCRn_EL1;
impl RegisterArray for SPMCGCRn_EL1 {
	const NAME: &'static str = "SPMCGCRn_EL1";
	const ELEMS: usize = 1;
	const ELEM_LEN: usize = 64;
}

/// Trace Buffer Status/syndrome Register (EL1)
#[allow(non_camel_case_types)]
pub struct TRBSR_EL1;
impl Register for TRBSR_EL1 {
	const NAME: &'static str = "TRBSR_EL1";
	const LEN: usize = 64;
}

/// TRBSR_EL1 register fields
pub mod trbsr_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRBSR_EL1\[55:32\]
	pub struct MSS2;
	impl RegisterView for MSS2 {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "MSS2";
		const OFFSET: usize = 32;
		const LEN: usize = 24;
	}

	/// TRBSR_EL1\[31:26\]
	pub struct EC;
	impl RegisterView for EC {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "EC";
		const OFFSET: usize = 26;
		const LEN: usize = 6;
	}

	/// TRBSR_EL1\[23\]
	pub struct DAT;
	impl RegisterView for DAT {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "DAT";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// TRBSR_EL1\[22\]
	pub struct IRQ;
	impl RegisterView for IRQ {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "IRQ";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}

	/// TRBSR_EL1\[21\]
	pub struct TRG;
	impl RegisterView for TRG {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "TRG";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// TRBSR_EL1\[20\]
	pub struct WRAP;
	impl RegisterView for WRAP {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "WRAP";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// TRBSR_EL1\[18\]
	pub struct EA;
	impl RegisterView for EA {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "EA";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// TRBSR_EL1\[17\]
	pub struct S;
	impl RegisterView for S {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "S";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// TRBSR_EL1\[15:0\]
	pub struct MSS;
	impl RegisterView for MSS {
		type Register = super::TRBSR_EL1;
		const NAME: &'static str = "MSS";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Trace Configuration Register
pub struct TRCCONFIGR;
impl Register for TRCCONFIGR {
	const NAME: &'static str = "TRCCONFIGR";
	const LEN: usize = 64;
}

/// TRCCONFIGR register fields
pub mod trcconfigr {
	use crate::core::model::proc::RegisterView;

	/// TRCCONFIGR\[18\]
	pub struct ITO;
	impl RegisterView for ITO {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "ITO";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// TRCCONFIGR\[15\]
	pub struct VMIDOPT;
	impl RegisterView for VMIDOPT {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "VMIDOPT";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// TRCCONFIGR\[14:13\]
	pub struct QE;
	impl RegisterView for QE {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "QE";
		const OFFSET: usize = 13;
		const LEN: usize = 2;
	}

	/// TRCCONFIGR\[12\]
	pub struct RS;
	impl RegisterView for RS {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "RS";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// TRCCONFIGR\[11\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// TRCCONFIGR\[7\]
	pub struct VMID;
	impl RegisterView for VMID {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "VMID";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// TRCCONFIGR\[6\]
	pub struct CID;
	impl RegisterView for CID {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "CID";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// TRCCONFIGR\[4\]
	pub struct CCI;
	impl RegisterView for CCI {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "CCI";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// TRCCONFIGR\[3\]
	pub struct BB;
	impl RegisterView for BB {
		type Register = super::TRCCONFIGR;
		const NAME: &'static str = "BB";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}
}

/// Trace Device Architecture Register
pub struct TRCDEVARCH;
impl Register for TRCDEVARCH {
	const NAME: &'static str = "TRCDEVARCH";
	const LEN: usize = 64;
}

/// TRCDEVARCH register fields
pub mod trcdevarch {
	use crate::core::model::proc::RegisterView;

	/// TRCDEVARCH\[31:21\]
	pub struct ARCHITECT;
	impl RegisterView for ARCHITECT {
		type Register = super::TRCDEVARCH;
		const NAME: &'static str = "ARCHITECT";
		const OFFSET: usize = 21;
		const LEN: usize = 11;
	}

	/// TRCDEVARCH\[20\]
	pub struct PRESENT;
	impl RegisterView for PRESENT {
		type Register = super::TRCDEVARCH;
		const NAME: &'static str = "PRESENT";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// TRCDEVARCH\[19:16\]
	pub struct REVISION;
	impl RegisterView for REVISION {
		type Register = super::TRCDEVARCH;
		const NAME: &'static str = "REVISION";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// TRCDEVARCH\[15:12\]
	pub struct ARCHVER;
	impl RegisterView for ARCHVER {
		type Register = super::TRCDEVARCH;
		const NAME: &'static str = "ARCHVER";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// TRCDEVARCH\[11:0\]
	pub struct ARCHPART;
	impl RegisterView for ARCHPART {
		type Register = super::TRCDEVARCH;
		const NAME: &'static str = "ARCHPART";
		const OFFSET: usize = 0;
		const LEN: usize = 12;
	}
}

/// Auxiliary Fault Status Register 0 (EL1)
#[allow(non_camel_case_types)]
pub struct AFSR0_EL1;
impl Register for AFSR0_EL1 {
	const NAME: &'static str = "AFSR0_EL1";
	const LEN: usize = 64;
}

/// Random Number Full Entropy
pub struct RNDRRS;
impl Register for RNDRRS {
	const NAME: &'static str = "RNDRRS";
	const LEN: usize = 64;
}

/// RNDRRS register fields
pub mod rndrrs {
	use crate::core::model::proc::RegisterView;

	/// RNDRRS\[63:0\]
	pub struct RNDRRS;
	impl RegisterView for RNDRRS {
		type Register = super::RNDRRS;
		const NAME: &'static str = "RNDRRS";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Interrupt Controller Maintenance Interrupt State Register
#[allow(non_camel_case_types)]
pub struct ICH_MISR_EL2;
impl Register for ICH_MISR_EL2 {
	const NAME: &'static str = "ICH_MISR_EL2";
	const LEN: usize = 64;
}

/// ICH_MISR_EL2 register fields
pub mod ich_misr_el2 {
	use crate::core::model::proc::RegisterView;

	/// ICH_MISR_EL2\[7\]
	pub struct VGrp1D;
	impl RegisterView for VGrp1D {
		type Register = super::ICH_MISR_EL2;
		const NAME: &'static str = "VGrp1D";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// ICH_MISR_EL2\[6\]
	pub struct VGrp1E;
	impl RegisterView for VGrp1E {
		type Register = super::ICH_MISR_EL2;
		const NAME: &'static str = "VGrp1E";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// ICH_MISR_EL2\[5\]
	pub struct VGrp0D;
	impl RegisterView for VGrp0D {
		type Register = super::ICH_MISR_EL2;
		const NAME: &'static str = "VGrp0D";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// ICH_MISR_EL2\[4\]
	pub struct VGrp0E;
	impl RegisterView for VGrp0E {
		type Register = super::ICH_MISR_EL2;
		const NAME: &'static str = "VGrp0E";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// ICH_MISR_EL2\[3\]
	pub struct NP;
	impl RegisterView for NP {
		type Register = super::ICH_MISR_EL2;
		const NAME: &'static str = "NP";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// ICH_MISR_EL2\[2\]
	pub struct LRENP;
	impl RegisterView for LRENP {
		type Register = super::ICH_MISR_EL2;
		const NAME: &'static str = "LRENP";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// ICH_MISR_EL2\[1\]
	pub struct U;
	impl RegisterView for U {
		type Register = super::ICH_MISR_EL2;
		const NAME: &'static str = "U";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICH_MISR_EL2\[0\]
	pub struct EOI;
	impl RegisterView for EOI {
		type Register = super::ICH_MISR_EL2;
		const NAME: &'static str = "EOI";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Sampling Profiling ID Register
#[allow(non_camel_case_types)]
pub struct PMSIDR_EL1;
impl Register for PMSIDR_EL1 {
	const NAME: &'static str = "PMSIDR_EL1";
	const LEN: usize = 64;
}

/// PMSIDR_EL1 register fields
pub mod pmsidr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSIDR_EL1\[32\]
	pub struct SME;
	impl RegisterView for SME {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "SME";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[31:28\]
	pub struct ALTCLK;
	impl RegisterView for ALTCLK {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "ALTCLK";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// PMSIDR_EL1\[27\]
	pub struct FPF;
	impl RegisterView for FPF {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "FPF";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[26\]
	pub struct EFT;
	impl RegisterView for EFT {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "EFT";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[25\]
	pub struct CRR;
	impl RegisterView for CRR {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "CRR";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[24\]
	pub struct PBT;
	impl RegisterView for PBT {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "PBT";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[23:20\]
	pub struct Format;
	impl RegisterView for Format {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "Format";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// PMSIDR_EL1\[19:16\]
	pub struct CountSize;
	impl RegisterView for CountSize {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "CountSize";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// PMSIDR_EL1\[15:12\]
	pub struct MaxSize;
	impl RegisterView for MaxSize {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "MaxSize";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// PMSIDR_EL1\[11:8\]
	pub struct Interval;
	impl RegisterView for Interval {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "Interval";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// PMSIDR_EL1\[7\]
	pub struct FDS;
	impl RegisterView for FDS {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "FDS";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[6\]
	pub struct FnE;
	impl RegisterView for FnE {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "FnE";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[5\]
	pub struct ERnd;
	impl RegisterView for ERnd {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "ERnd";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[4\]
	pub struct LDS;
	impl RegisterView for LDS {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "LDS";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[3\]
	pub struct ArchInst;
	impl RegisterView for ArchInst {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "ArchInst";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[2\]
	pub struct FL;
	impl RegisterView for FL {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "FL";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[1\]
	pub struct FT;
	impl RegisterView for FT {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "FT";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// PMSIDR_EL1\[0\]
	pub struct FE;
	impl RegisterView for FE {
		type Register = super::PMSIDR_EL1;
		const NAME: &'static str = "FE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Stall Control Register
pub struct TRCSTALLCTLR;
impl Register for TRCSTALLCTLR {
	const NAME: &'static str = "TRCSTALLCTLR";
	const LEN: usize = 64;
}

/// TRCSTALLCTLR register fields
pub mod trcstallctlr {
	use crate::core::model::proc::RegisterView;

	/// TRCSTALLCTLR\[13\]
	pub struct NOOVERFLOW;
	impl RegisterView for NOOVERFLOW {
		type Register = super::TRCSTALLCTLR;
		const NAME: &'static str = "NOOVERFLOW";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// TRCSTALLCTLR\[8\]
	pub struct ISTALL;
	impl RegisterView for ISTALL {
		type Register = super::TRCSTALLCTLR;
		const NAME: &'static str = "ISTALL";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// TRCSTALLCTLR\[3:0\]
	pub struct LEVEL;
	impl RegisterView for LEVEL {
		type Register = super::TRCSTALLCTLR;
		const NAME: &'static str = "LEVEL";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Performance Monitors Cycle Count Register
#[allow(non_camel_case_types)]
pub struct PMCCNTR_EL0;
impl Register for PMCCNTR_EL0 {
	const NAME: &'static str = "PMCCNTR_EL0";
	const LEN: usize = 64;
}

/// PMCCNTR_EL0 register fields
pub mod pmccntr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMCCNTR_EL0\[63:0\]
	pub struct CCNT;
	impl RegisterView for CCNT {
		type Register = super::PMCCNTR_EL0;
		const NAME: &'static str = "CCNT";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Trace Context Identifier Comparator Value Registers \<n\>, n = 7 - 0
pub struct TRCCIDCVRn;
impl RegisterArray for TRCCIDCVRn {
	const NAME: &'static str = "TRCCIDCVRn";
	const ELEMS: usize = 7;
	const ELEM_LEN: usize = 64;
}

/// TRCCIDCVRn register fields
pub mod trccidcvrn {
	use crate::core::model::proc::RegisterArrayView;

	/// TRCCIDCVRn\[63:0\]
	pub struct VALUE;
	impl RegisterArrayView for VALUE {
		type RegisterArray = super::TRCCIDCVRn;
		const NAME: &'static str = "VALUE";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// System Performance Monitors Interrupt Enable Clear Register
#[allow(non_camel_case_types)]
pub struct SPMINTENCLR_EL1;
impl Register for SPMINTENCLR_EL1 {
	const NAME: &'static str = "SPMINTENCLR_EL1";
	const LEN: usize = 64;
}

/// Branch Record Buffer Source Address Injection Register
#[allow(non_camel_case_types)]
pub struct BRBSRCINJ_EL1;
impl Register for BRBSRCINJ_EL1 {
	const NAME: &'static str = "BRBSRCINJ_EL1";
	const LEN: usize = 64;
}

/// BRBSRCINJ_EL1 register fields
pub mod brbsrcinj_el1 {
	use crate::core::model::proc::RegisterView;

	/// BRBSRCINJ_EL1\[63:0\]
	pub struct ADDRESS;
	impl RegisterView for ADDRESS {
		type Register = super::BRBSRCINJ_EL1;
		const NAME: &'static str = "ADDRESS";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Virtualization Processor ID Register
#[allow(non_camel_case_types)]
pub struct VPIDR_EL2;
impl Register for VPIDR_EL2 {
	const NAME: &'static str = "VPIDR_EL2";
	const LEN: usize = 64;
}

/// VPIDR_EL2 register fields
pub mod vpidr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VPIDR_EL2\[31:24\]
	pub struct Implementer;
	impl RegisterView for Implementer {
		type Register = super::VPIDR_EL2;
		const NAME: &'static str = "Implementer";
		const OFFSET: usize = 24;
		const LEN: usize = 8;
	}

	/// VPIDR_EL2\[23:20\]
	pub struct Variant;
	impl RegisterView for Variant {
		type Register = super::VPIDR_EL2;
		const NAME: &'static str = "Variant";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// VPIDR_EL2\[19:16\]
	pub struct Architecture;
	impl RegisterView for Architecture {
		type Register = super::VPIDR_EL2;
		const NAME: &'static str = "Architecture";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// VPIDR_EL2\[15:4\]
	pub struct PartNum;
	impl RegisterView for PartNum {
		type Register = super::VPIDR_EL2;
		const NAME: &'static str = "PartNum";
		const OFFSET: usize = 4;
		const LEN: usize = 12;
	}

	/// VPIDR_EL2\[3:0\]
	pub struct Revision;
	impl RegisterView for Revision {
		type Register = super::VPIDR_EL2;
		const NAME: &'static str = "Revision";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Virtual Nested Control Register
#[allow(non_camel_case_types)]
pub struct VNCR_EL2;
impl Register for VNCR_EL2 {
	const NAME: &'static str = "VNCR_EL2";
	const LEN: usize = 64;
}

/// VNCR_EL2 register fields
pub mod vncr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VNCR_EL2\[63:57\]
	pub struct RESS;
	impl RegisterView for RESS {
		type Register = super::VNCR_EL2;
		const NAME: &'static str = "RESS";
		const OFFSET: usize = 57;
		const LEN: usize = 7;
	}

	/// VNCR_EL2\[56:12\]
	pub struct BADDR;
	impl RegisterView for BADDR {
		type Register = super::VNCR_EL2;
		const NAME: &'static str = "BADDR";
		const OFFSET: usize = 12;
		const LEN: usize = 45;
	}
}

/// Cache Size Selection Register
#[allow(non_camel_case_types)]
pub struct CSSELR_EL1;
impl Register for CSSELR_EL1 {
	const NAME: &'static str = "CSSELR_EL1";
	const LEN: usize = 64;
}

/// CSSELR_EL1 register fields
pub mod csselr_el1 {
	use crate::core::model::proc::RegisterView;

	/// CSSELR_EL1\[4\]
	pub struct TnD;
	impl RegisterView for TnD {
		type Register = super::CSSELR_EL1;
		const NAME: &'static str = "TnD";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// CSSELR_EL1\[3:1\]
	pub struct Level;
	impl RegisterView for Level {
		type Register = super::CSSELR_EL1;
		const NAME: &'static str = "Level";
		const OFFSET: usize = 1;
		const LEN: usize = 3;
	}

	/// CSSELR_EL1\[0\]
	pub struct InD;
	impl RegisterView for InD {
		type Register = super::CSSELR_EL1;
		const NAME: &'static str = "InD";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace ID Register 13
pub struct TRCIDR13;
impl Register for TRCIDR13 {
	const NAME: &'static str = "TRCIDR13";
	const LEN: usize = 64;
}

/// TRCIDR13 register fields
pub mod trcidr13 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR13\[31:0\]
	pub struct NUMCONDSPC;
	impl RegisterView for NUMCONDSPC {
		type Register = super::TRCIDR13;
		const NAME: &'static str = "NUMCONDSPC";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Guarded Control Stack Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct GCSCR_EL2;
impl Register for GCSCR_EL2 {
	const NAME: &'static str = "GCSCR_EL2";
	const LEN: usize = 64;
}

/// GCSCR_EL2 register fields
pub mod gcscr_el2 {
	use crate::core::model::proc::RegisterView;

	/// GCSCR_EL2\[9\]
	pub struct STREn;
	impl RegisterView for STREn {
		type Register = super::GCSCR_EL2;
		const NAME: &'static str = "STREn";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// GCSCR_EL2\[8\]
	pub struct PUSHMEn;
	impl RegisterView for PUSHMEn {
		type Register = super::GCSCR_EL2;
		const NAME: &'static str = "PUSHMEn";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// GCSCR_EL2\[6\]
	pub struct EXLOCKEN;
	impl RegisterView for EXLOCKEN {
		type Register = super::GCSCR_EL2;
		const NAME: &'static str = "EXLOCKEN";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// GCSCR_EL2\[5\]
	pub struct RVCHKEN;
	impl RegisterView for RVCHKEN {
		type Register = super::GCSCR_EL2;
		const NAME: &'static str = "RVCHKEN";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// GCSCR_EL2\[0\]
	pub struct PCRSEL;
	impl RegisterView for PCRSEL {
		type Register = super::GCSCR_EL2;
		const NAME: &'static str = "PCRSEL";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Selected Pseudo-fault Generation Control Register
#[allow(non_camel_case_types)]
pub struct ERXPFGCTL_EL1;
impl Register for ERXPFGCTL_EL1 {
	const NAME: &'static str = "ERXPFGCTL_EL1";
	const LEN: usize = 64;
}

/// Monitor Debug System Control Register
#[allow(non_camel_case_types)]
pub struct MDSCR_EL1;
impl Register for MDSCR_EL1 {
	const NAME: &'static str = "MDSCR_EL1";
	const LEN: usize = 64;
}

/// MDSCR_EL1 register fields
pub mod mdscr_el1 {
	use crate::core::model::proc::RegisterView;

	/// MDSCR_EL1\[50\]
	pub struct EnSTEPOP;
	impl RegisterView for EnSTEPOP {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "EnSTEPOP";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[35\]
	pub struct EHBWE;
	impl RegisterView for EHBWE {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "EHBWE";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[34\]
	pub struct EnSPM;
	impl RegisterView for EnSPM {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "EnSPM";
		const OFFSET: usize = 34;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[33\]
	pub struct TTA;
	impl RegisterView for TTA {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "TTA";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[32\]
	pub struct EMBWE;
	impl RegisterView for EMBWE {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "EMBWE";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[31\]
	pub struct TFO;
	impl RegisterView for TFO {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "TFO";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[30\]
	pub struct RXfull;
	impl RegisterView for RXfull {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "RXfull";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[29\]
	pub struct TXfull;
	impl RegisterView for TXfull {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "TXfull";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[27\]
	pub struct RXO;
	impl RegisterView for RXO {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "RXO";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[26\]
	pub struct TXU;
	impl RegisterView for TXU {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "TXU";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[23:22\]
	pub struct INTdis;
	impl RegisterView for INTdis {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "INTdis";
		const OFFSET: usize = 22;
		const LEN: usize = 2;
	}

	/// MDSCR_EL1\[21\]
	pub struct TDA;
	impl RegisterView for TDA {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "TDA";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[19\]
	pub struct SC2;
	impl RegisterView for SC2 {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "SC2";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[15\]
	pub struct MDE;
	impl RegisterView for MDE {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "MDE";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[14\]
	pub struct HDE;
	impl RegisterView for HDE {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "HDE";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[13\]
	pub struct KDE;
	impl RegisterView for KDE {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "KDE";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[12\]
	pub struct TDCC;
	impl RegisterView for TDCC {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "TDCC";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[6\]
	pub struct ERR;
	impl RegisterView for ERR {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "ERR";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// MDSCR_EL1\[0\]
	pub struct SS;
	impl RegisterView for SS {
		type Register = super::MDSCR_EL1;
		const NAME: &'static str = "SS";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Hypervisor Debug Fine-Grained Write Trap Register
#[allow(non_camel_case_types)]
pub struct HDFGWTR_EL2;
impl Register for HDFGWTR_EL2 {
	const NAME: &'static str = "HDFGWTR_EL2";
	const LEN: usize = 64;
}

/// HDFGWTR_EL2 register fields
pub mod hdfgwtr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HDFGWTR_EL2\[62\]
	#[allow(non_camel_case_types)]
	pub struct nPMSNEVFR_EL1;
	impl RegisterView for nPMSNEVFR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "nPMSNEVFR_EL1";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[61\]
	#[allow(non_camel_case_types)]
	pub struct nBRBDATA;
	impl RegisterView for nBRBDATA {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "nBRBDATA";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[60\]
	#[allow(non_camel_case_types)]
	pub struct nBRBCTL;
	impl RegisterView for nBRBCTL {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "nBRBCTL";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[57\]
	#[allow(non_camel_case_types)]
	pub struct PMUSERENR_EL0;
	impl RegisterView for PMUSERENR_EL0 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMUSERENR_EL0";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[56\]
	#[allow(non_camel_case_types)]
	pub struct TRBTRG_EL1;
	impl RegisterView for TRBTRG_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRBTRG_EL1";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[55\]
	#[allow(non_camel_case_types)]
	pub struct TRBSR_EL1;
	impl RegisterView for TRBSR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRBSR_EL1";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[54\]
	#[allow(non_camel_case_types)]
	pub struct TRBPTR_EL1;
	impl RegisterView for TRBPTR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRBPTR_EL1";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[53\]
	#[allow(non_camel_case_types)]
	pub struct TRBMAR_EL1;
	impl RegisterView for TRBMAR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRBMAR_EL1";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[52\]
	#[allow(non_camel_case_types)]
	pub struct TRBLIMITR_EL1;
	impl RegisterView for TRBLIMITR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRBLIMITR_EL1";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[50\]
	#[allow(non_camel_case_types)]
	pub struct TRBBASER_EL1;
	impl RegisterView for TRBBASER_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRBBASER_EL1";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[49\]
	#[allow(non_camel_case_types)]
	pub struct TRFCR_EL1;
	impl RegisterView for TRFCR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRFCR_EL1";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[48\]
	pub struct TRCVICTLR;
	impl RegisterView for TRCVICTLR {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCVICTLR";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[46\]
	pub struct TRCSSCSRn;
	impl RegisterView for TRCSSCSRn {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCSSCSRn";
		const OFFSET: usize = 46;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[45\]
	pub struct TRCSEQSTR;
	impl RegisterView for TRCSEQSTR {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCSEQSTR";
		const OFFSET: usize = 45;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[44\]
	pub struct TRCPRGCTLR;
	impl RegisterView for TRCPRGCTLR {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCPRGCTLR";
		const OFFSET: usize = 44;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[42\]
	pub struct TRCOSLAR;
	impl RegisterView for TRCOSLAR {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCOSLAR";
		const OFFSET: usize = 42;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[41\]
	pub struct TRCIMSPECn;
	impl RegisterView for TRCIMSPECn {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCIMSPECn";
		const OFFSET: usize = 41;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[37\]
	pub struct TRCCNTVRn;
	impl RegisterView for TRCCNTVRn {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCCNTVRn";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[36\]
	pub struct TRCCLAIM;
	impl RegisterView for TRCCLAIM {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCCLAIM";
		const OFFSET: usize = 36;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[35\]
	pub struct TRCAUXCTLR;
	impl RegisterView for TRCAUXCTLR {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRCAUXCTLR";
		const OFFSET: usize = 35;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[33\]
	pub struct TRC;
	impl RegisterView for TRC {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "TRC";
		const OFFSET: usize = 33;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[32\]
	#[allow(non_camel_case_types)]
	pub struct PMSLATFR_EL1;
	impl RegisterView for PMSLATFR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMSLATFR_EL1";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[31\]
	#[allow(non_camel_case_types)]
	pub struct PMSIRR_EL1;
	impl RegisterView for PMSIRR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMSIRR_EL1";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[29\]
	#[allow(non_camel_case_types)]
	pub struct PMSICR_EL1;
	impl RegisterView for PMSICR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMSICR_EL1";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[28\]
	#[allow(non_camel_case_types)]
	pub struct PMSFCR_EL1;
	impl RegisterView for PMSFCR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMSFCR_EL1";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[27\]
	#[allow(non_camel_case_types)]
	pub struct PMSEVFR_EL1;
	impl RegisterView for PMSEVFR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMSEVFR_EL1";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[26\]
	#[allow(non_camel_case_types)]
	pub struct PMSCR_EL1;
	impl RegisterView for PMSCR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMSCR_EL1";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[25\]
	#[allow(non_camel_case_types)]
	pub struct PMBSR_EL1;
	impl RegisterView for PMBSR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMBSR_EL1";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[24\]
	#[allow(non_camel_case_types)]
	pub struct PMBPTR_EL1;
	impl RegisterView for PMBPTR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMBPTR_EL1";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[23\]
	#[allow(non_camel_case_types)]
	pub struct PMBLIMITR_EL1;
	impl RegisterView for PMBLIMITR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMBLIMITR_EL1";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[21\]
	#[allow(non_camel_case_types)]
	pub struct PMCR_EL0;
	impl RegisterView for PMCR_EL0 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMCR_EL0";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[20\]
	#[allow(non_camel_case_types)]
	pub struct PMSWINC_EL0;
	impl RegisterView for PMSWINC_EL0 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMSWINC_EL0";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[19\]
	#[allow(non_camel_case_types)]
	pub struct PMSELR_EL0;
	impl RegisterView for PMSELR_EL0 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMSELR_EL0";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[18\]
	pub struct PMOVS;
	impl RegisterView for PMOVS {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMOVS";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[17\]
	pub struct PMINTEN;
	impl RegisterView for PMINTEN {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMINTEN";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[16\]
	pub struct PMCNTEN;
	impl RegisterView for PMCNTEN {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMCNTEN";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[15\]
	#[allow(non_camel_case_types)]
	pub struct PMCCNTR_EL0;
	impl RegisterView for PMCCNTR_EL0 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMCCNTR_EL0";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[14\]
	#[allow(non_camel_case_types)]
	pub struct PMCCFILTR_EL0;
	impl RegisterView for PMCCFILTR_EL0 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMCCFILTR_EL0";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[13\]
	#[allow(non_camel_case_types)]
	pub struct PMEVTYPERn_EL0;
	impl RegisterView for PMEVTYPERn_EL0 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMEVTYPERn_EL0";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[12\]
	#[allow(non_camel_case_types)]
	pub struct PMEVCNTRn_EL0;
	impl RegisterView for PMEVCNTRn_EL0 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "PMEVCNTRn_EL0";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[11\]
	#[allow(non_camel_case_types)]
	pub struct OSDLR_EL1;
	impl RegisterView for OSDLR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "OSDLR_EL1";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[10\]
	#[allow(non_camel_case_types)]
	pub struct OSECCR_EL1;
	impl RegisterView for OSECCR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "OSECCR_EL1";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[8\]
	#[allow(non_camel_case_types)]
	pub struct OSLAR_EL1;
	impl RegisterView for OSLAR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "OSLAR_EL1";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[7\]
	#[allow(non_camel_case_types)]
	pub struct DBGPRCR_EL1;
	impl RegisterView for DBGPRCR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "DBGPRCR_EL1";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[5\]
	pub struct DBGCLAIM;
	impl RegisterView for DBGCLAIM {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "DBGCLAIM";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[4\]
	#[allow(non_camel_case_types)]
	pub struct MDSCR_EL1;
	impl RegisterView for MDSCR_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "MDSCR_EL1";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[3\]
	#[allow(non_camel_case_types)]
	pub struct DBGWVRn_EL1;
	impl RegisterView for DBGWVRn_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "DBGWVRn_EL1";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[2\]
	#[allow(non_camel_case_types)]
	pub struct DBGWCRn_EL1;
	impl RegisterView for DBGWCRn_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "DBGWCRn_EL1";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[1\]
	#[allow(non_camel_case_types)]
	pub struct DBGBVRn_EL1;
	impl RegisterView for DBGBVRn_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "DBGBVRn_EL1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// HDFGWTR_EL2\[0\]
	#[allow(non_camel_case_types)]
	pub struct DBGBCRn_EL1;
	impl RegisterView for DBGBCRn_EL1 {
		type Register = super::HDFGWTR_EL2;
		const NAME: &'static str = "DBGBCRn_EL1";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// AArch32 Memory Model Feature Register 1
#[allow(non_camel_case_types)]
pub struct ID_MMFR1_EL1;
impl Register for ID_MMFR1_EL1 {
	const NAME: &'static str = "ID_MMFR1_EL1";
	const LEN: usize = 64;
}

/// ID_MMFR1_EL1 register fields
pub mod id_mmfr1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_MMFR1_EL1\[31:28\]
	pub struct BPred;
	impl RegisterView for BPred {
		type Register = super::ID_MMFR1_EL1;
		const NAME: &'static str = "BPred";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_MMFR1_EL1\[27:24\]
	pub struct L1TstCln;
	impl RegisterView for L1TstCln {
		type Register = super::ID_MMFR1_EL1;
		const NAME: &'static str = "L1TstCln";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_MMFR1_EL1\[23:20\]
	pub struct L1Uni;
	impl RegisterView for L1Uni {
		type Register = super::ID_MMFR1_EL1;
		const NAME: &'static str = "L1Uni";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_MMFR1_EL1\[19:16\]
	pub struct L1Hvd;
	impl RegisterView for L1Hvd {
		type Register = super::ID_MMFR1_EL1;
		const NAME: &'static str = "L1Hvd";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_MMFR1_EL1\[15:12\]
	pub struct L1UniSW;
	impl RegisterView for L1UniSW {
		type Register = super::ID_MMFR1_EL1;
		const NAME: &'static str = "L1UniSW";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_MMFR1_EL1\[11:8\]
	pub struct L1HvdSW;
	impl RegisterView for L1HvdSW {
		type Register = super::ID_MMFR1_EL1;
		const NAME: &'static str = "L1HvdSW";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_MMFR1_EL1\[7:4\]
	pub struct L1UniVA;
	impl RegisterView for L1UniVA {
		type Register = super::ID_MMFR1_EL1;
		const NAME: &'static str = "L1UniVA";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_MMFR1_EL1\[3:0\]
	pub struct L1HvdVA;
	impl RegisterView for L1HvdVA {
		type Register = super::ID_MMFR1_EL1;
		const NAME: &'static str = "L1HvdVA";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Trace Buffer MPAM Configuration Register
#[allow(non_camel_case_types)]
pub struct TRBMPAM_EL1;
impl Register for TRBMPAM_EL1 {
	const NAME: &'static str = "TRBMPAM_EL1";
	const LEN: usize = 64;
}

/// TRBMPAM_EL1 register fields
pub mod trbmpam_el1 {
	use crate::core::model::proc::RegisterView;

	/// TRBMPAM_EL1\[26\]
	pub struct EN;
	impl RegisterView for EN {
		type Register = super::TRBMPAM_EL1;
		const NAME: &'static str = "EN";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// TRBMPAM_EL1\[25:24\]
	#[allow(non_camel_case_types)]
	pub struct MPAM_SP;
	impl RegisterView for MPAM_SP {
		type Register = super::TRBMPAM_EL1;
		const NAME: &'static str = "MPAM_SP";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// TRBMPAM_EL1\[23:16\]
	pub struct PMG;
	impl RegisterView for PMG {
		type Register = super::TRBMPAM_EL1;
		const NAME: &'static str = "PMG";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// TRBMPAM_EL1\[15:0\]
	pub struct PARTID;
	impl RegisterView for PARTID {
		type Register = super::TRBMPAM_EL1;
		const NAME: &'static str = "PARTID";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Guarded Control Stack Control Register (EL3)
#[allow(non_camel_case_types)]
pub struct GCSCR_EL3;
impl Register for GCSCR_EL3 {
	const NAME: &'static str = "GCSCR_EL3";
	const LEN: usize = 64;
}

/// GCSCR_EL3 register fields
pub mod gcscr_el3 {
	use crate::core::model::proc::RegisterView;

	/// GCSCR_EL3\[9\]
	pub struct STREn;
	impl RegisterView for STREn {
		type Register = super::GCSCR_EL3;
		const NAME: &'static str = "STREn";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// GCSCR_EL3\[8\]
	pub struct PUSHMEn;
	impl RegisterView for PUSHMEn {
		type Register = super::GCSCR_EL3;
		const NAME: &'static str = "PUSHMEn";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// GCSCR_EL3\[6\]
	pub struct EXLOCKEN;
	impl RegisterView for EXLOCKEN {
		type Register = super::GCSCR_EL3;
		const NAME: &'static str = "EXLOCKEN";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// GCSCR_EL3\[5\]
	pub struct RVCHKEN;
	impl RegisterView for RVCHKEN {
		type Register = super::GCSCR_EL3;
		const NAME: &'static str = "RVCHKEN";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// GCSCR_EL3\[0\]
	pub struct PCRSEL;
	impl RegisterView for PCRSEL {
		type Register = super::GCSCR_EL3;
		const NAME: &'static str = "PCRSEL";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace ID Register 12
pub struct TRCIDR12;
impl Register for TRCIDR12 {
	const NAME: &'static str = "TRCIDR12";
	const LEN: usize = 64;
}

/// TRCIDR12 register fields
pub mod trcidr12 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR12\[31:0\]
	pub struct NUMCONDKEY;
	impl RegisterView for NUMCONDKEY {
		type Register = super::TRCIDR12;
		const NAME: &'static str = "NUMCONDKEY";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Hypervisor Activity Monitors Fine-Grained Read Trap Register
#[allow(non_camel_case_types)]
pub struct HAFGRTR_EL2;
impl Register for HAFGRTR_EL2 {
	const NAME: &'static str = "HAFGRTR_EL2";
	const LEN: usize = 64;
}

/// HAFGRTR_EL2 register fields
pub mod hafgrtr_el2 {
	use crate::core::model::proc::RegisterView;

	/// HAFGRTR_EL2\[17\]
	pub struct AMCNTEN1;
	impl RegisterView for AMCNTEN1 {
		type Register = super::HAFGRTR_EL2;
		const NAME: &'static str = "AMCNTEN1";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// HAFGRTR_EL2\[0\]
	pub struct AMCNTEN0;
	impl RegisterView for AMCNTEN0 {
		type Register = super::HAFGRTR_EL2;
		const NAME: &'static str = "AMCNTEN0";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Virtualization Translation Table Base Register
#[allow(non_camel_case_types)]
pub struct VTTBR_EL2;
impl Register for VTTBR_EL2 {
	const NAME: &'static str = "VTTBR_EL2";
	const LEN: usize = 64;
}

/// VTTBR_EL2 register fields
pub mod vttbr_el2 {
	use crate::core::model::proc::RegisterView;

	/// VTTBR_EL2\[47:5\]
	pub struct BADDR;
	impl RegisterView for BADDR {
		type Register = super::VTTBR_EL2;
		const NAME: &'static str = "BADDR";
		const OFFSET: usize = 5;
		const LEN: usize = 43;
	}

	/// VTTBR_EL2\[63:48\]
	pub struct VMID;
	impl RegisterView for VMID {
		type Register = super::VTTBR_EL2;
		const NAME: &'static str = "VMID";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// VTTBR_EL2\[2:1\]
	pub struct SKL;
	impl RegisterView for SKL {
		type Register = super::VTTBR_EL2;
		const NAME: &'static str = "SKL";
		const OFFSET: usize = 1;
		const LEN: usize = 2;
	}

	/// VTTBR_EL2\[0\]
	pub struct CnP;
	impl RegisterView for CnP {
		type Register = super::VTTBR_EL2;
		const NAME: &'static str = "CnP";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// OS Double Lock Register
#[allow(non_camel_case_types)]
pub struct OSDLR_EL1;
impl Register for OSDLR_EL1 {
	const NAME: &'static str = "OSDLR_EL1";
	const LEN: usize = 64;
}

/// OSDLR_EL1 register fields
pub mod osdlr_el1 {
	use crate::core::model::proc::RegisterView;

	/// OSDLR_EL1\[0\]
	pub struct DLK;
	impl RegisterView for DLK {
		type Register = super::OSDLR_EL1;
		const NAME: &'static str = "DLK";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Cycle Count Saved Value Register
#[allow(non_camel_case_types)]
pub struct PMCCNTSVR_EL1;
impl Register for PMCCNTSVR_EL1 {
	const NAME: &'static str = "PMCCNTSVR_EL1";
	const LEN: usize = 64;
}

/// PMCCNTSVR_EL1 register fields
pub mod pmccntsvr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMCCNTSVR_EL1\[63:0\]
	pub struct CCNT;
	impl RegisterView for CCNT {
		type Register = super::PMCCNTSVR_EL1;
		const NAME: &'static str = "CCNT";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Performance Monitors Overflow Flag Status Clear Register
#[allow(non_camel_case_types)]
pub struct PMOVSCLR_EL0;
impl Register for PMOVSCLR_EL0 {
	const NAME: &'static str = "PMOVSCLR_EL0";
	const LEN: usize = 64;
}

/// PMOVSCLR_EL0 register fields
pub mod pmovsclr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMOVSCLR_EL0\[31\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMOVSCLR_EL0;
		const NAME: &'static str = "C";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
}

/// Counter-timer Secure Physical Timer Control Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHPS_CTL_EL2;
impl Register for CNTHPS_CTL_EL2 {
	const NAME: &'static str = "CNTHPS_CTL_EL2";
	const LEN: usize = 64;
}

/// CNTHPS_CTL_EL2 register fields
pub mod cnthps_ctl_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHPS_CTL_EL2\[2\]
	pub struct ISTATUS;
	impl RegisterView for ISTATUS {
		type Register = super::CNTHPS_CTL_EL2;
		const NAME: &'static str = "ISTATUS";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTHPS_CTL_EL2\[1\]
	pub struct IMASK;
	impl RegisterView for IMASK {
		type Register = super::CNTHPS_CTL_EL2;
		const NAME: &'static str = "IMASK";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTHPS_CTL_EL2\[0\]
	pub struct ENABLE;
	impl RegisterView for ENABLE {
		type Register = super::CNTHPS_CTL_EL2;
		const NAME: &'static str = "ENABLE";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Performance Monitors Control Register
#[allow(non_camel_case_types)]
pub struct PMCR_EL0;
impl Register for PMCR_EL0 {
	const NAME: &'static str = "PMCR_EL0";
	const LEN: usize = 64;
}

/// PMCR_EL0 register fields
pub mod pmcr_el0 {
	use crate::core::model::proc::RegisterView;

	/// PMCR_EL0\[32\]
	pub struct FZS;
	impl RegisterView for FZS {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "FZS";
		const OFFSET: usize = 32;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[31:24\]
	pub struct IMP;
	impl RegisterView for IMP {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "IMP";
		const OFFSET: usize = 24;
		const LEN: usize = 8;
	}

	/// PMCR_EL0\[23:16\]
	pub struct IDCODE;
	impl RegisterView for IDCODE {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "IDCODE";
		const OFFSET: usize = 16;
		const LEN: usize = 8;
	}

	/// PMCR_EL0\[15:11\]
	pub struct N;
	impl RegisterView for N {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "N";
		const OFFSET: usize = 11;
		const LEN: usize = 5;
	}

	/// PMCR_EL0\[9\]
	pub struct FZO;
	impl RegisterView for FZO {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "FZO";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[7\]
	pub struct LP;
	impl RegisterView for LP {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "LP";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[6\]
	pub struct LC;
	impl RegisterView for LC {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "LC";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[5\]
	pub struct DP;
	impl RegisterView for DP {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "DP";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[4\]
	pub struct X;
	impl RegisterView for X {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "X";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[3\]
	pub struct D;
	impl RegisterView for D {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "D";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[2\]
	pub struct C;
	impl RegisterView for C {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "C";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[1\]
	pub struct P;
	impl RegisterView for P {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "P";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// PMCR_EL0\[0\]
	pub struct E;
	impl RegisterView for E {
		type Register = super::PMCR_EL0;
		const NAME: &'static str = "E";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace Synchronization Period Register
pub struct TRCSYNCPR;
impl Register for TRCSYNCPR {
	const NAME: &'static str = "TRCSYNCPR";
	const LEN: usize = 64;
}

/// TRCSYNCPR register fields
pub mod trcsyncpr {
	use crate::core::model::proc::RegisterView;

	/// TRCSYNCPR\[4:0\]
	pub struct PERIOD;
	impl RegisterView for PERIOD {
		type Register = super::TRCSYNCPR;
		const NAME: &'static str = "PERIOD";
		const OFFSET: usize = 0;
		const LEN: usize = 5;
	}
}

/// Auxiliary Fault Status Register 0 (EL2)
#[allow(non_camel_case_types)]
pub struct AFSR0_EL2;
impl Register for AFSR0_EL2 {
	const NAME: &'static str = "AFSR0_EL2";
	const LEN: usize = 64;
}

/// Breakpoint and Watchpoint Selection Register
#[allow(non_camel_case_types)]
pub struct MDSELR_EL1;
impl Register for MDSELR_EL1 {
	const NAME: &'static str = "MDSELR_EL1";
	const LEN: usize = 64;
}

/// MDSELR_EL1 register fields
pub mod mdselr_el1 {
	use crate::core::model::proc::RegisterView;

	/// MDSELR_EL1\[5:4\]
	pub struct BANK;
	impl RegisterView for BANK {
		type Register = super::MDSELR_EL1;
		const NAME: &'static str = "BANK";
		const OFFSET: usize = 4;
		const LEN: usize = 2;
	}
}

/// Sampling Inverted Event Filter Register
#[allow(non_camel_case_types)]
pub struct PMSNEVFR_EL1;
impl Register for PMSNEVFR_EL1 {
	const NAME: &'static str = "PMSNEVFR_EL1";
	const LEN: usize = 64;
}

/// PMSNEVFR_EL1 register fields
pub mod pmsnevfr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMSNEVFR_EL1\[63\]
	#[allow(non_camel_case_types)]
	pub struct E_63;
	impl RegisterView for E_63 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[62\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62;
	impl RegisterView for E_63_62 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[61\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61;
	impl RegisterView for E_63_62_61 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[60\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60;
	impl RegisterView for E_63_62_61_60 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[59\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59;
	impl RegisterView for E_63_62_61_60_59 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59";
		const OFFSET: usize = 59;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[58\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58;
	impl RegisterView for E_63_62_61_60_59_58 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58";
		const OFFSET: usize = 58;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[57\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57;
	impl RegisterView for E_63_62_61_60_59_58_57 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[56\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56;
	impl RegisterView for E_63_62_61_60_59_58_57_56 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[55\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[54\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54";
		const OFFSET: usize = 54;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[53\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53";
		const OFFSET: usize = 53;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[52\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[51\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51";
		const OFFSET: usize = 51;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[50\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50";
		const OFFSET: usize = 50;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[49\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49";
		const OFFSET: usize = 49;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[48\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48";
		const OFFSET: usize = 48;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[31\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[30\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[29\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[28\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[27\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[26\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[25\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[25\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25";
		const OFFSET: usize = 25;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[24\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[24\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24";
		const OFFSET: usize = 24;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[23\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23";
		const OFFSET: usize = 23;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[22\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22";
		const OFFSET: usize = 22;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[21\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21";
		const OFFSET: usize = 21;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[20\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20";
		const OFFSET: usize = 20;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[19\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[18\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[17\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[16\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[15\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[14\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[13\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[12\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[11\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[10\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10";
		const OFFSET: usize = 10;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[9\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[8\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[7\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[6\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[5\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[4\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[3\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[2\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}
	/// PMSNEVFR_EL1\[1\]
	#[allow(non_camel_case_types)]
	pub struct E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2_1;
	impl RegisterView for E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2_1 {
		type Register = super::PMSNEVFR_EL1;
		const NAME: &'static str = "E_63_62_61_60_59_58_57_56_55_54_53_52_51_50_49_48_31_30_29_28_27_26_25_25_24_24_23_22_21_20_19_18_17_16_15_14_13_12_11_10_9_8_7_6_5_4_3_2_1";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}
}

/// Selected Pseudo-fault Generation Feature Register
#[allow(non_camel_case_types)]
pub struct ERXPFGF_EL1;
impl Register for ERXPFGF_EL1 {
	const NAME: &'static str = "ERXPFGF_EL1";
	const LEN: usize = 64;
}

/// Alternate MECID for EL2&0 translation regimes.
#[allow(non_camel_case_types)]
pub struct MECID_A1_EL2;
impl Register for MECID_A1_EL2 {
	const NAME: &'static str = "MECID_A1_EL2";
	const LEN: usize = 64;
}

/// MECID_A1_EL2 register fields
pub mod mecid_a1_el2 {
	use crate::core::model::proc::RegisterView;

	/// MECID_A1_EL2\[15:0\]
	pub struct MECID;
	impl RegisterView for MECID {
		type Register = super::MECID_A1_EL2;
		const NAME: &'static str = "MECID";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// MPAM Virtual PARTID Mapping Register 4
#[allow(non_camel_case_types)]
pub struct MPAMVPM4_EL2;
impl Register for MPAMVPM4_EL2 {
	const NAME: &'static str = "MPAMVPM4_EL2";
	const LEN: usize = 64;
}

/// MPAMVPM4_EL2 register fields
pub mod mpamvpm4_el2 {
	use crate::core::model::proc::RegisterView;

	/// MPAMVPM4_EL2\[63:48\]
	pub struct PhyPARTID19;
	impl RegisterView for PhyPARTID19 {
		type Register = super::MPAMVPM4_EL2;
		const NAME: &'static str = "PhyPARTID19";
		const OFFSET: usize = 48;
		const LEN: usize = 16;
	}

	/// MPAMVPM4_EL2\[47:32\]
	pub struct PhyPARTID18;
	impl RegisterView for PhyPARTID18 {
		type Register = super::MPAMVPM4_EL2;
		const NAME: &'static str = "PhyPARTID18";
		const OFFSET: usize = 32;
		const LEN: usize = 16;
	}

	/// MPAMVPM4_EL2\[31:16\]
	pub struct PhyPARTID17;
	impl RegisterView for PhyPARTID17 {
		type Register = super::MPAMVPM4_EL2;
		const NAME: &'static str = "PhyPARTID17";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAMVPM4_EL2\[15:0\]
	pub struct PhyPARTID16;
	impl RegisterView for PhyPARTID16 {
		type Register = super::MPAMVPM4_EL2;
		const NAME: &'static str = "PhyPARTID16";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Interrupt Controller Virtual Control Register
#[allow(non_camel_case_types)]
pub struct ICV_CTLR_EL1;
impl Register for ICV_CTLR_EL1 {
	const NAME: &'static str = "ICV_CTLR_EL1";
	const LEN: usize = 64;
}

/// ICV_CTLR_EL1 register fields
pub mod icv_ctlr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_CTLR_EL1\[19\]
	pub struct ExtRange;
	impl RegisterView for ExtRange {
		type Register = super::ICV_CTLR_EL1;
		const NAME: &'static str = "ExtRange";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// ICV_CTLR_EL1\[18\]
	pub struct RSS;
	impl RegisterView for RSS {
		type Register = super::ICV_CTLR_EL1;
		const NAME: &'static str = "RSS";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// ICV_CTLR_EL1\[15\]
	pub struct A3V;
	impl RegisterView for A3V {
		type Register = super::ICV_CTLR_EL1;
		const NAME: &'static str = "A3V";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// ICV_CTLR_EL1\[14\]
	pub struct SEIS;
	impl RegisterView for SEIS {
		type Register = super::ICV_CTLR_EL1;
		const NAME: &'static str = "SEIS";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// ICV_CTLR_EL1\[13:11\]
	pub struct IDbits;
	impl RegisterView for IDbits {
		type Register = super::ICV_CTLR_EL1;
		const NAME: &'static str = "IDbits";
		const OFFSET: usize = 11;
		const LEN: usize = 3;
	}

	/// ICV_CTLR_EL1\[10:8\]
	pub struct PRIbits;
	impl RegisterView for PRIbits {
		type Register = super::ICV_CTLR_EL1;
		const NAME: &'static str = "PRIbits";
		const OFFSET: usize = 8;
		const LEN: usize = 3;
	}

	/// ICV_CTLR_EL1\[1\]
	pub struct EOImode;
	impl RegisterView for EOImode {
		type Register = super::ICV_CTLR_EL1;
		const NAME: &'static str = "EOImode";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// ICV_CTLR_EL1\[0\]
	pub struct CBPR;
	impl RegisterView for CBPR {
		type Register = super::ICV_CTLR_EL1;
		const NAME: &'static str = "CBPR";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Floating-Point Exception Control Register
#[allow(non_camel_case_types)]
pub struct FPEXC32_EL2;
impl Register for FPEXC32_EL2 {
	const NAME: &'static str = "FPEXC32_EL2";
	const LEN: usize = 64;
}

/// FPEXC32_EL2 register fields
pub mod fpexc32_el2 {
	use crate::core::model::proc::RegisterView;

	/// FPEXC32_EL2\[31\]
	pub struct EX;
	impl RegisterView for EX {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "EX";
		const OFFSET: usize = 31;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[30\]
	pub struct EN;
	impl RegisterView for EN {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "EN";
		const OFFSET: usize = 30;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[29\]
	pub struct DEX;
	impl RegisterView for DEX {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "DEX";
		const OFFSET: usize = 29;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[28\]
	pub struct FP2V;
	impl RegisterView for FP2V {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "FP2V";
		const OFFSET: usize = 28;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[27\]
	pub struct VV;
	impl RegisterView for VV {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "VV";
		const OFFSET: usize = 27;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[26\]
	pub struct TFV;
	impl RegisterView for TFV {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "TFV";
		const OFFSET: usize = 26;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[10:8\]
	pub struct VECITR;
	impl RegisterView for VECITR {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "VECITR";
		const OFFSET: usize = 8;
		const LEN: usize = 3;
	}

	/// FPEXC32_EL2\[7\]
	pub struct IDF;
	impl RegisterView for IDF {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "IDF";
		const OFFSET: usize = 7;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[4\]
	pub struct IXF;
	impl RegisterView for IXF {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "IXF";
		const OFFSET: usize = 4;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[3\]
	pub struct UFF;
	impl RegisterView for UFF {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "UFF";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[2\]
	pub struct OFF;
	impl RegisterView for OFF {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "OFF";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[1\]
	pub struct DZF;
	impl RegisterView for DZF {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "DZF";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// FPEXC32_EL2\[0\]
	pub struct IOF;
	impl RegisterView for IOF {
		type Register = super::FPEXC32_EL2;
		const NAME: &'static str = "IOF";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// EL0 Read-Only Software Thread ID Register
#[allow(non_camel_case_types)]
pub struct TPIDRRO_EL0;
impl Register for TPIDRRO_EL0 {
	const NAME: &'static str = "TPIDRRO_EL0";
	const LEN: usize = 64;
}

/// Counter-timer Virtual Timer TimerValue Register (EL2)
#[allow(non_camel_case_types)]
pub struct CNTHV_TVAL_EL2;
impl Register for CNTHV_TVAL_EL2 {
	const NAME: &'static str = "CNTHV_TVAL_EL2";
	const LEN: usize = 64;
}

/// CNTHV_TVAL_EL2 register fields
pub mod cnthv_tval_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHV_TVAL_EL2\[31:0\]
	pub struct TimerValue;
	impl RegisterView for TimerValue {
		type Register = super::CNTHV_TVAL_EL2;
		const NAME: &'static str = "TimerValue";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Trace ID Register 8
pub struct TRCIDR8;
impl Register for TRCIDR8 {
	const NAME: &'static str = "TRCIDR8";
	const LEN: usize = 64;
}

/// TRCIDR8 register fields
pub mod trcidr8 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR8\[31:0\]
	pub struct MAXSPEC;
	impl RegisterView for MAXSPEC {
		type Register = super::TRCIDR8;
		const NAME: &'static str = "MAXSPEC";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Counter-timer Hypervisor Control Register
#[allow(non_camel_case_types)]
pub struct CNTHCTL_EL2;
impl Register for CNTHCTL_EL2 {
	const NAME: &'static str = "CNTHCTL_EL2";
	const LEN: usize = 64;
}

/// CNTHCTL_EL2 register fields
pub mod cnthctl_el2 {
	use crate::core::model::proc::RegisterView;

	/// CNTHCTL_EL2\[19\]
	pub struct CNTPMASK;
	impl RegisterView for CNTPMASK {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "CNTPMASK";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[18\]
	pub struct CNTVMASK;
	impl RegisterView for CNTVMASK {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "CNTVMASK";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[17\]
	pub struct EVNTIS;
	impl RegisterView for EVNTIS {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EVNTIS";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[16\]
	pub struct EL1NVVCT;
	impl RegisterView for EL1NVVCT {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL1NVVCT";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[15\]
	pub struct EL1NVPCT;
	impl RegisterView for EL1NVPCT {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL1NVPCT";
		const OFFSET: usize = 15;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[14\]
	pub struct EL1TVCT;
	impl RegisterView for EL1TVCT {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL1TVCT";
		const OFFSET: usize = 14;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[13\]
	pub struct EL1TVT;
	impl RegisterView for EL1TVT {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL1TVT";
		const OFFSET: usize = 13;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[12\]
	pub struct ECV;
	impl RegisterView for ECV {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "ECV";
		const OFFSET: usize = 12;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[11\]
	pub struct EL1PTEN;
	impl RegisterView for EL1PTEN {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL1PTEN";
		const OFFSET: usize = 11;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[0\]
	pub struct EL1PCTEN;
	impl RegisterView for EL1PCTEN {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL1PCTEN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[9\]
	pub struct EL0PTEN;
	impl RegisterView for EL0PTEN {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL0PTEN";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[8\]
	pub struct EL0VTEN;
	impl RegisterView for EL0VTEN {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL0VTEN";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[7:4\]
	pub struct EVNTI;
	impl RegisterView for EVNTI {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EVNTI";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// CNTHCTL_EL2\[3\]
	pub struct EVNTDIR;
	impl RegisterView for EVNTDIR {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EVNTDIR";
		const OFFSET: usize = 3;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[2\]
	pub struct EVNTEN;
	impl RegisterView for EVNTEN {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EVNTEN";
		const OFFSET: usize = 2;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[1\]
	pub struct EL0VCTEN;
	impl RegisterView for EL0VCTEN {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL0VCTEN";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[0\]
	pub struct EL0PCTEN;
	impl RegisterView for EL0PCTEN {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL0PCTEN";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}

	/// CNTHCTL_EL2\[1\]
	pub struct EL1PCEN;
	impl RegisterView for EL1PCEN {
		type Register = super::CNTHCTL_EL2;
		const NAME: &'static str = "EL1PCEN";
		const OFFSET: usize = 1;
		const LEN: usize = 1;
	}
}

/// Guarded Control Stack Control Register (EL1)
#[allow(non_camel_case_types)]
pub struct GCSCR_EL1;
impl Register for GCSCR_EL1 {
	const NAME: &'static str = "GCSCR_EL1";
	const LEN: usize = 64;
}

/// GCSCR_EL1 register fields
pub mod gcscr_el1 {
	use crate::core::model::proc::RegisterView;

	/// GCSCR_EL1\[9\]
	pub struct STREn;
	impl RegisterView for STREn {
		type Register = super::GCSCR_EL1;
		const NAME: &'static str = "STREn";
		const OFFSET: usize = 9;
		const LEN: usize = 1;
	}

	/// GCSCR_EL1\[8\]
	pub struct PUSHMEn;
	impl RegisterView for PUSHMEn {
		type Register = super::GCSCR_EL1;
		const NAME: &'static str = "PUSHMEn";
		const OFFSET: usize = 8;
		const LEN: usize = 1;
	}

	/// GCSCR_EL1\[6\]
	pub struct EXLOCKEN;
	impl RegisterView for EXLOCKEN {
		type Register = super::GCSCR_EL1;
		const NAME: &'static str = "EXLOCKEN";
		const OFFSET: usize = 6;
		const LEN: usize = 1;
	}

	/// GCSCR_EL1\[5\]
	pub struct RVCHKEN;
	impl RegisterView for RVCHKEN {
		type Register = super::GCSCR_EL1;
		const NAME: &'static str = "RVCHKEN";
		const OFFSET: usize = 5;
		const LEN: usize = 1;
	}

	/// GCSCR_EL1\[0\]
	pub struct PCRSEL;
	impl RegisterView for PCRSEL {
		type Register = super::GCSCR_EL1;
		const NAME: &'static str = "PCRSEL";
		const OFFSET: usize = 0;
		const LEN: usize = 1;
	}
}

/// Trace ID Register 10
pub struct TRCIDR10;
impl Register for TRCIDR10 {
	const NAME: &'static str = "TRCIDR10";
	const LEN: usize = 64;
}

/// TRCIDR10 register fields
pub mod trcidr10 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR10\[31:0\]
	pub struct NUMP1KEY;
	impl RegisterView for NUMP1KEY {
		type Register = super::TRCIDR10;
		const NAME: &'static str = "NUMP1KEY";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Pointer Authentication Key A for Code (bits\[127:64\]) 
#[allow(non_camel_case_types)]
pub struct APGAKeyHi_EL1;
impl Register for APGAKeyHi_EL1 {
	const NAME: &'static str = "APGAKeyHi_EL1";
	const LEN: usize = 64;
}

/// Performance Monitors Event Count Registers, n = 30 - 0
#[allow(non_camel_case_types)]
pub struct PMEVCNTRn_EL0;
impl RegisterArray for PMEVCNTRn_EL0 {
	const NAME: &'static str = "PMEVCNTRn_EL0";
	const ELEMS: usize = 30;
	const ELEM_LEN: usize = 64;
}

/// PMEVCNTRn_EL0 register fields
pub mod pmevcntrn_el0 {
	use crate::core::model::proc::RegisterArrayView;

	/// PMEVCNTRn_EL0\[31:0\]
	pub struct EVCNT;
	impl RegisterArrayView for EVCNT {
		type RegisterArray = super::PMEVCNTRn_EL0;
		const NAME: &'static str = "EVCNT";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Domain Access Control Register
#[allow(non_camel_case_types)]
pub struct DACR32_EL2;
impl Register for DACR32_EL2 {
	const NAME: &'static str = "DACR32_EL2";
	const LEN: usize = 64;
}

/// Interrupt Controller Virtual Interrupt Priority Mask Register
#[allow(non_camel_case_types)]
pub struct ICV_PMR_EL1;
impl Register for ICV_PMR_EL1 {
	const NAME: &'static str = "ICV_PMR_EL1";
	const LEN: usize = 64;
}

/// ICV_PMR_EL1 register fields
pub mod icv_pmr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_PMR_EL1\[7:0\]
	pub struct Priority;
	impl RegisterView for Priority {
		type Register = super::ICV_PMR_EL1;
		const NAME: &'static str = "Priority";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Activity Monitors Event Counter Registers 0, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct AMEVCNTR0n_EL0;
impl RegisterArray for AMEVCNTR0n_EL0 {
	const NAME: &'static str = "AMEVCNTR0n_EL0";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// AMEVCNTR0n_EL0 register fields
pub mod amevcntr0n_el0 {
	use crate::core::model::proc::RegisterArrayView;

	/// AMEVCNTR0n_EL0\[63:0\]
	pub struct ACNT;
	impl RegisterArrayView for ACNT {
		type RegisterArray = super::AMEVCNTR0n_EL0;
		const NAME: &'static str = "ACNT";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Activity Monitors Event Counter Registers 1, n = 15 - 0
#[allow(non_camel_case_types)]
pub struct AMEVCNTR1n_EL0;
impl RegisterArray for AMEVCNTR1n_EL0 {
	const NAME: &'static str = "AMEVCNTR1n_EL0";
	const ELEMS: usize = 15;
	const ELEM_LEN: usize = 64;
}

/// AMEVCNTR1n_EL0 register fields
pub mod amevcntr1n_el0 {
	use crate::core::model::proc::RegisterArrayView;

	/// AMEVCNTR1n_EL0\[63:0\]
	pub struct ACNT;
	impl RegisterArrayView for ACNT {
		type RegisterArray = super::AMEVCNTR1n_EL0;
		const NAME: &'static str = "ACNT";
		const OFFSET: usize = 0;
		const LEN: usize = 64;
	}
}

/// Interrupt Controller Virtual Running Priority Register
#[allow(non_camel_case_types)]
pub struct ICV_RPR_EL1;
impl Register for ICV_RPR_EL1 {
	const NAME: &'static str = "ICV_RPR_EL1";
	const LEN: usize = 64;
}

/// ICV_RPR_EL1 register fields
pub mod icv_rpr_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_RPR_EL1\[63\]
	pub struct NMI;
	impl RegisterView for NMI {
		type Register = super::ICV_RPR_EL1;
		const NAME: &'static str = "NMI";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// ICV_RPR_EL1\[7:0\]
	pub struct Priority;
	impl RegisterView for Priority {
		type Register = super::ICV_RPR_EL1;
		const NAME: &'static str = "Priority";
		const OFFSET: usize = 0;
		const LEN: usize = 8;
	}
}

/// Trace ID Register 11
pub struct TRCIDR11;
impl Register for TRCIDR11 {
	const NAME: &'static str = "TRCIDR11";
	const LEN: usize = 64;
}

/// TRCIDR11 register fields
pub mod trcidr11 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR11\[31:0\]
	pub struct NUMP1SPC;
	impl RegisterView for NUMP1SPC {
		type Register = super::TRCIDR11;
		const NAME: &'static str = "NUMP1SPC";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Profiling Buffer Status/syndrome Register (EL1)
#[allow(non_camel_case_types)]
pub struct PMBSR_EL1;
impl Register for PMBSR_EL1 {
	const NAME: &'static str = "PMBSR_EL1";
	const LEN: usize = 64;
}

/// PMBSR_EL1 register fields
pub mod pmbsr_el1 {
	use crate::core::model::proc::RegisterView;

	/// PMBSR_EL1\[40\]
	pub struct TopLevel;
	impl RegisterView for TopLevel {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "TopLevel";
		const OFFSET: usize = 40;
		const LEN: usize = 1;
	}

	/// PMBSR_EL1\[39\]
	pub struct AssuredOnly;
	impl RegisterView for AssuredOnly {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "AssuredOnly";
		const OFFSET: usize = 39;
		const LEN: usize = 1;
	}

	/// PMBSR_EL1\[38\]
	pub struct Overlay;
	impl RegisterView for Overlay {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "Overlay";
		const OFFSET: usize = 38;
		const LEN: usize = 1;
	}

	/// PMBSR_EL1\[37\]
	pub struct DirtyBit;
	impl RegisterView for DirtyBit {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "DirtyBit";
		const OFFSET: usize = 37;
		const LEN: usize = 1;
	}

	/// PMBSR_EL1\[31:26\]
	pub struct EC;
	impl RegisterView for EC {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "EC";
		const OFFSET: usize = 26;
		const LEN: usize = 6;
	}

	/// PMBSR_EL1\[19\]
	pub struct DL;
	impl RegisterView for DL {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "DL";
		const OFFSET: usize = 19;
		const LEN: usize = 1;
	}

	/// PMBSR_EL1\[18\]
	pub struct EA;
	impl RegisterView for EA {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "EA";
		const OFFSET: usize = 18;
		const LEN: usize = 1;
	}

	/// PMBSR_EL1\[17\]
	pub struct S;
	impl RegisterView for S {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "S";
		const OFFSET: usize = 17;
		const LEN: usize = 1;
	}

	/// PMBSR_EL1\[16\]
	pub struct COLL;
	impl RegisterView for COLL {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "COLL";
		const OFFSET: usize = 16;
		const LEN: usize = 1;
	}

	/// PMBSR_EL1\[15:0\]
	pub struct MSS;
	impl RegisterView for MSS {
		type Register = super::PMBSR_EL1;
		const NAME: &'static str = "MSS";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// Counter-timer Frequency Register
#[allow(non_camel_case_types)]
pub struct CNTFRQ_EL0;
impl Register for CNTFRQ_EL0 {
	const NAME: &'static str = "CNTFRQ_EL0";
	const LEN: usize = 64;
}

/// Trace ID Register 9
pub struct TRCIDR9;
impl Register for TRCIDR9 {
	const NAME: &'static str = "TRCIDR9";
	const LEN: usize = 64;
}

/// TRCIDR9 register fields
pub mod trcidr9 {
	use crate::core::model::proc::RegisterView;

	/// TRCIDR9\[31:0\]
	pub struct NUMP0KEY;
	impl RegisterView for NUMP0KEY {
		type Register = super::TRCIDR9;
		const NAME: &'static str = "NUMP0KEY";
		const OFFSET: usize = 0;
		const LEN: usize = 32;
	}
}

/// Trace Authentication Status Register
pub struct TRCAUTHSTATUS;
impl Register for TRCAUTHSTATUS {
	const NAME: &'static str = "TRCAUTHSTATUS";
	const LEN: usize = 64;
}

/// TRCAUTHSTATUS register fields
pub mod trcauthstatus {
	use crate::core::model::proc::RegisterView;

	/// TRCAUTHSTATUS\[27:26\]
	pub struct RTNID;
	impl RegisterView for RTNID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "RTNID";
		const OFFSET: usize = 26;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[25:24\]
	pub struct RTID;
	impl RegisterView for RTID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "RTID";
		const OFFSET: usize = 24;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[15:14\]
	pub struct RLNID;
	impl RegisterView for RLNID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "RLNID";
		const OFFSET: usize = 14;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[13:12\]
	pub struct RLID;
	impl RegisterView for RLID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "RLID";
		const OFFSET: usize = 12;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[11:10\]
	pub struct HNID;
	impl RegisterView for HNID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "HNID";
		const OFFSET: usize = 10;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[9:8\]
	pub struct HID;
	impl RegisterView for HID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "HID";
		const OFFSET: usize = 8;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[7:6\]
	pub struct SNID;
	impl RegisterView for SNID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "SNID";
		const OFFSET: usize = 6;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[5:4\]
	pub struct SID;
	impl RegisterView for SID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "SID";
		const OFFSET: usize = 4;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[3:2\]
	pub struct NSNID;
	impl RegisterView for NSNID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "NSNID";
		const OFFSET: usize = 2;
		const LEN: usize = 2;
	}

	/// TRCAUTHSTATUS\[1:0\]
	pub struct NSID;
	impl RegisterView for NSID {
		type Register = super::TRCAUTHSTATUS;
		const NAME: &'static str = "NSID";
		const OFFSET: usize = 0;
		const LEN: usize = 2;
	}
}

/// Interrupt Controller Virtual Non-maskable Interrupt Acknowledge Register 1
#[allow(non_camel_case_types)]
pub struct ICV_NMIAR1_EL1;
impl Register for ICV_NMIAR1_EL1 {
	const NAME: &'static str = "ICV_NMIAR1_EL1";
	const LEN: usize = 64;
}

/// ICV_NMIAR1_EL1 register fields
pub mod icv_nmiar1_el1 {
	use crate::core::model::proc::RegisterView;

	/// ICV_NMIAR1_EL1\[23:0\]
	pub struct INTID;
	impl RegisterView for INTID {
		type Register = super::ICV_NMIAR1_EL1;
		const NAME: &'static str = "INTID";
		const OFFSET: usize = 0;
		const LEN: usize = 24;
	}
}

/// Pointer Authentication Key A for Instruction (bits\[63:0\]) 
#[allow(non_camel_case_types)]
pub struct APIAKeyLo_EL1;
impl Register for APIAKeyLo_EL1 {
	const NAME: &'static str = "APIAKeyLo_EL1";
	const LEN: usize = 64;
}

/// AArch64 Instruction Set Attribute Register 0
#[allow(non_camel_case_types)]
pub struct ID_AA64ISAR0_EL1;
impl Register for ID_AA64ISAR0_EL1 {
	const NAME: &'static str = "ID_AA64ISAR0_EL1";
	const LEN: usize = 64;
}

/// ID_AA64ISAR0_EL1 register fields
pub mod id_aa64isar0_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_AA64ISAR0_EL1\[63:60\]
	pub struct RNDR;
	impl RegisterView for RNDR {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "RNDR";
		const OFFSET: usize = 60;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[59:56\]
	pub struct TLB;
	impl RegisterView for TLB {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "TLB";
		const OFFSET: usize = 56;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[55:52\]
	pub struct TS;
	impl RegisterView for TS {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "TS";
		const OFFSET: usize = 52;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[51:48\]
	pub struct FHM;
	impl RegisterView for FHM {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "FHM";
		const OFFSET: usize = 48;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[47:44\]
	pub struct DP;
	impl RegisterView for DP {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "DP";
		const OFFSET: usize = 44;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[43:40\]
	pub struct SM4;
	impl RegisterView for SM4 {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "SM4";
		const OFFSET: usize = 40;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[39:36\]
	pub struct SM3;
	impl RegisterView for SM3 {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "SM3";
		const OFFSET: usize = 36;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[35:32\]
	pub struct SHA3;
	impl RegisterView for SHA3 {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "SHA3";
		const OFFSET: usize = 32;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[31:28\]
	pub struct RDM;
	impl RegisterView for RDM {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "RDM";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[27:24\]
	pub struct TME;
	impl RegisterView for TME {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "TME";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[23:20\]
	pub struct Atomic;
	impl RegisterView for Atomic {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "Atomic";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[19:16\]
	pub struct CRC32;
	impl RegisterView for CRC32 {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "CRC32";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[15:12\]
	pub struct SHA2;
	impl RegisterView for SHA2 {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "SHA2";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[11:8\]
	pub struct SHA1;
	impl RegisterView for SHA1 {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "SHA1";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_AA64ISAR0_EL1\[7:4\]
	pub struct AES;
	impl RegisterView for AES {
		type Register = super::ID_AA64ISAR0_EL1;
		const NAME: &'static str = "AES";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}
}

/// Interrupt Controller Hyp Active Priorities Group 0 Registers, n = 3 - 0
#[allow(non_camel_case_types)]
pub struct ICH_AP0Rn_EL2;
impl RegisterArray for ICH_AP0Rn_EL2 {
	const NAME: &'static str = "ICH_AP0Rn_EL2";
	const ELEMS: usize = 3;
	const ELEM_LEN: usize = 64;
}

/// MPAM3 Register (EL3)
#[allow(non_camel_case_types)]
pub struct MPAM3_EL3;
impl Register for MPAM3_EL3 {
	const NAME: &'static str = "MPAM3_EL3";
	const LEN: usize = 64;
}

/// MPAM3_EL3 register fields
pub mod mpam3_el3 {
	use crate::core::model::proc::RegisterView;

	/// MPAM3_EL3\[63\]
	pub struct MPAMEN;
	impl RegisterView for MPAMEN {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "MPAMEN";
		const OFFSET: usize = 63;
		const LEN: usize = 1;
	}

	/// MPAM3_EL3\[62\]
	pub struct TRAPLOWER;
	impl RegisterView for TRAPLOWER {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "TRAPLOWER";
		const OFFSET: usize = 62;
		const LEN: usize = 1;
	}

	/// MPAM3_EL3\[61\]
	pub struct SDEFLT;
	impl RegisterView for SDEFLT {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "SDEFLT";
		const OFFSET: usize = 61;
		const LEN: usize = 1;
	}

	/// MPAM3_EL3\[60\]
	#[allow(non_camel_case_types)]
	pub struct FORCE_NS;
	impl RegisterView for FORCE_NS {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "FORCE_NS";
		const OFFSET: usize = 60;
		const LEN: usize = 1;
	}

	/// MPAM3_EL3\[57\]
	#[allow(non_camel_case_types)]
	pub struct ALTSP_HEN;
	impl RegisterView for ALTSP_HEN {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "ALTSP_HEN";
		const OFFSET: usize = 57;
		const LEN: usize = 1;
	}

	/// MPAM3_EL3\[56\]
	#[allow(non_camel_case_types)]
	pub struct ALTSP_HFC;
	impl RegisterView for ALTSP_HFC {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "ALTSP_HFC";
		const OFFSET: usize = 56;
		const LEN: usize = 1;
	}

	/// MPAM3_EL3\[55\]
	#[allow(non_camel_case_types)]
	pub struct ALTSP_EL3;
	impl RegisterView for ALTSP_EL3 {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "ALTSP_EL3";
		const OFFSET: usize = 55;
		const LEN: usize = 1;
	}

	/// MPAM3_EL3\[52\]
	#[allow(non_camel_case_types)]
	pub struct RT_ALTSP_NS;
	impl RegisterView for RT_ALTSP_NS {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "RT_ALTSP_NS";
		const OFFSET: usize = 52;
		const LEN: usize = 1;
	}

	/// MPAM3_EL3\[47:40\]
	#[allow(non_camel_case_types)]
	pub struct PMG_D;
	impl RegisterView for PMG_D {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "PMG_D";
		const OFFSET: usize = 40;
		const LEN: usize = 8;
	}

	/// MPAM3_EL3\[39:32\]
	#[allow(non_camel_case_types)]
	pub struct PMG_I;
	impl RegisterView for PMG_I {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "PMG_I";
		const OFFSET: usize = 32;
		const LEN: usize = 8;
	}

	/// MPAM3_EL3\[31:16\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_D;
	impl RegisterView for PARTID_D {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "PARTID_D";
		const OFFSET: usize = 16;
		const LEN: usize = 16;
	}

	/// MPAM3_EL3\[15:0\]
	#[allow(non_camel_case_types)]
	pub struct PARTID_I;
	impl RegisterView for PARTID_I {
		type Register = super::MPAM3_EL3;
		const NAME: &'static str = "PARTID_I";
		const OFFSET: usize = 0;
		const LEN: usize = 16;
	}
}

/// AArch32 Instruction Set Attribute Register 4
#[allow(non_camel_case_types)]
pub struct ID_ISAR4_EL1;
impl Register for ID_ISAR4_EL1 {
	const NAME: &'static str = "ID_ISAR4_EL1";
	const LEN: usize = 64;
}

/// ID_ISAR4_EL1 register fields
pub mod id_isar4_el1 {
	use crate::core::model::proc::RegisterView;

	/// ID_ISAR4_EL1\[31:28\]
	#[allow(non_camel_case_types)]
	pub struct SWP_frac;
	impl RegisterView for SWP_frac {
		type Register = super::ID_ISAR4_EL1;
		const NAME: &'static str = "SWP_frac";
		const OFFSET: usize = 28;
		const LEN: usize = 4;
	}

	/// ID_ISAR4_EL1\[27:24\]
	#[allow(non_camel_case_types)]
	pub struct PSR_M;
	impl RegisterView for PSR_M {
		type Register = super::ID_ISAR4_EL1;
		const NAME: &'static str = "PSR_M";
		const OFFSET: usize = 24;
		const LEN: usize = 4;
	}

	/// ID_ISAR4_EL1\[23:20\]
	#[allow(non_camel_case_types)]
	pub struct SynchPrim_frac;
	impl RegisterView for SynchPrim_frac {
		type Register = super::ID_ISAR4_EL1;
		const NAME: &'static str = "SynchPrim_frac";
		const OFFSET: usize = 20;
		const LEN: usize = 4;
	}

	/// ID_ISAR4_EL1\[19:16\]
	pub struct Barrier;
	impl RegisterView for Barrier {
		type Register = super::ID_ISAR4_EL1;
		const NAME: &'static str = "Barrier";
		const OFFSET: usize = 16;
		const LEN: usize = 4;
	}

	/// ID_ISAR4_EL1\[15:12\]
	pub struct SMC;
	impl RegisterView for SMC {
		type Register = super::ID_ISAR4_EL1;
		const NAME: &'static str = "SMC";
		const OFFSET: usize = 12;
		const LEN: usize = 4;
	}

	/// ID_ISAR4_EL1\[11:8\]
	pub struct Writeback;
	impl RegisterView for Writeback {
		type Register = super::ID_ISAR4_EL1;
		const NAME: &'static str = "Writeback";
		const OFFSET: usize = 8;
		const LEN: usize = 4;
	}

	/// ID_ISAR4_EL1\[7:4\]
	pub struct WithShifts;
	impl RegisterView for WithShifts {
		type Register = super::ID_ISAR4_EL1;
		const NAME: &'static str = "WithShifts";
		const OFFSET: usize = 4;
		const LEN: usize = 4;
	}

	/// ID_ISAR4_EL1\[3:0\]
	pub struct Unpriv;
	impl RegisterView for Unpriv {
		type Register = super::ID_ISAR4_EL1;
		const NAME: &'static str = "Unpriv";
		const OFFSET: usize = 0;
		const LEN: usize = 4;
	}
}

/// Auxiliary Fault Status Register 0 (EL3)
#[allow(non_camel_case_types)]
pub struct AFSR0_EL3;
impl Register for AFSR0_EL3 {
	const NAME: &'static str = "AFSR0_EL3";
	const LEN: usize = 64;
}
impl ArmCpu {
	pub fn decl_aarch64_sysregs(&self, decl: &mut ProcDecl<ArmCpu>) {		decl		.reg::<CNTPS_CTL_EL1>()
		.reg::<ERXMISC2_EL1>()
		.reg::<VBAR_EL2>()
		.reg::<ICC_EOIR1_EL1>()
		.reg_array::<PMEVCNTSVRn_EL1>()
		.reg::<ICC_DIR_EL1>()
		.reg::<CNTHPS_TVAL_EL2>()
		.reg::<ICC_BPR1_EL1>()
		.reg::<TRBMAR_EL1>()
		.reg::<TRCITECR_EL2>()
		.reg::<TRCIDR4>()
		.reg::<ACTLR_EL2>()
		.reg::<AMCGCR_EL0>()
		.reg::<AMCNTENCLR1_EL0>()
		.reg::<ACCDATA_EL1>()
		.reg::<ID_ISAR1_EL1>()
		.reg::<SPMINTENSET_EL1>()
		.reg::<TRCVMIDCCTLR1>()
		.reg_array::<DBGWVRn_EL1>()
		.reg::<CPACR_EL1>()
		.reg::<CNTKCTL_EL1>()
		.reg::<TRCVMIDCCTLR0>()
		.reg::<ERXGSR_EL1>()
		.reg::<APDBKeyHi_EL1>()
		.reg::<TRCTRACEIDR>()
		.reg::<MVFR1_EL1>()
		.reg::<ICC_SGI0R_EL1>()
		.reg::<BRBTGTINJ_EL1>()
		.reg::<ICC_SGI1R_EL1>()
		.reg::<ACTLR_EL3>()
		.reg::<TRCIDR5>()
		.reg::<ICC_SRE_EL1>()
		.reg::<MPAMVPM1_EL2>()
		.reg::<PMOVSSET_EL0>()
		.reg::<PMCEID1_EL0>()
		.reg_array::<DBGWCRn_EL1>()
		.reg::<MECID_P1_EL2>()
		.reg::<SMCR_EL1>()
		.reg::<FPMR>()
		.reg::<SPMCR_EL0>()
		.reg::<MPAMIDR_EL1>()
		.reg::<SPSR_EL1>()
		.reg::<ELR_EL1>()
		.reg::<TPIDR2_EL0>()
		.reg::<ICV_IAR1_EL1>()
		.reg::<VBAR_EL3>()
		.reg::<HDFGRTR_EL2>()
		.reg::<ID_PFR1_EL1>()
		.reg::<VBAR_EL1>()
		.reg::<PMSSCR_EL1>()
		.reg::<TRCTSCTLR>()
		.reg::<PMBPTR_EL1>()
		.reg::<ID_MMFR4_EL1>()
		.reg::<AMCNTENSET0_EL0>()
		.reg::<ELR_EL3>()
		.reg::<TRBTRG_EL1>()
		.reg::<SPSR_EL3>()
		.reg::<ID_AA64MMFR0_EL1>()
		.reg::<SCR_EL3>()
		.reg::<HFGITR2_EL2>()
		.reg::<MDRAR_EL1>()
		.reg::<SMCR_EL3>()
		.reg::<CNTVCT_EL0>()
		.reg::<ID_AA64AFR1_EL1>()
		.reg::<VDISR_EL3>()
		.reg::<TRCITECR_EL1>()
		.reg::<TRCIDR7>()
		.reg::<ID_AA64DFR0_EL1>()
		.reg::<ICC_SRE_EL3>()
		.reg::<PMICFILTR_EL0>()
		.reg::<ACTLR_EL1>()
		.reg::<CNTV_TVAL_EL0>()
		.reg::<ID_AA64PFR2_EL1>()
		.reg::<TRCVIPCSSCTLR>()
		.reg::<ICV_EOIR1_EL1>()
		.reg::<ID_AA64ZFR0_EL1>()
		.reg::<MDCCSR_EL0>()
		.reg::<ICH_ELRSR_EL2>()
		.reg::<ICC_SRE_EL2>()
		.reg::<SPSR_abt>()
		.reg::<TRCIDR6>()
		.reg::<MPAMSM_EL1>()
		.reg::<PMSFCR_EL1>()
		.reg::<VDISR_EL2>()
		.reg::<SMCR_EL2>()
		.reg::<HPFAR_EL2>()
		.reg::<SPSR_EL2>()
		.reg::<GPCCR_EL3>()
		.reg::<PMSELR_EL0>()
		.reg::<SPMSCR_EL1>()
		.reg::<ELR_EL2>()
		.reg::<TRBLIMITR_EL1>()
		.reg::<LORC_EL1>()
		.reg::<HFGWTR2_EL2>()
		.reg::<LOREA_EL1>()
		.reg::<GCSCRE0_EL1>()
		.reg::<TRCVICTLR>()
		.reg::<ID_AA64MMFR1_EL1>()
		.reg_array::<TRCSSCCRn>()
		.reg::<ID_AA64AFR0_EL1>()
		.reg::<ERRIDR_EL1>()
		.reg::<SCXTNUM_EL1>()
		.reg::<TPIDR_EL2>()
		.reg::<ID_PFR0_EL1>()
		.reg::<TRCSEQSTR>()
		.reg::<OSLSR_EL1>()
		.reg::<AMCNTENSET1_EL0>()
		.reg::<ID_MMFR5_EL1>()
		.reg::<SMIDR_EL1>()
		.reg::<ICC_ASGI1R_EL1>()
		.reg_array::<ICV_AP1Rn_EL1>()
		.reg::<TRCIDR2>()
		.reg_array::<TRCSEQEVRn>()
		.reg::<SPSR_fiq>()
		.reg::<DBGDTRRX_EL0>()
		.reg::<AMAIR_EL2>()
		.reg::<ID_AA64DFR1_EL1>()
		.reg::<CNTV_CTL_EL0>()
		.reg::<ESR_EL2>()
		.reg::<SCTLR2_EL3>()
		.reg::<ID_AA64FPFR0_EL1>()
		.reg_array::<PMEVTYPERn_EL0>()
		.reg::<SCTLR2_EL2>()
		.reg::<ESR_EL3>()
		.reg::<BRBCR_EL2>()
		.reg::<AMAIR_EL3>()
		.reg::<SPMOVSCLR_EL0>()
		.reg::<RGSR_EL1>()
		.reg::<HFGRTR2_EL2>()
		.reg::<ICV_EOIR0_EL1>()
		.reg::<ICH_VTR_EL2>()
		.reg::<TRCIDR3>()
		.reg::<SCXTNUM_EL0>()
		.reg::<TPIDR_EL3>()
		.reg::<GMID_EL1>()
		.reg::<BRBFCR_EL1>()
		.reg::<PMSLATFR_EL1>()
		.reg::<PMCNTENCLR_EL0>()
		.reg::<ICC_NMIAR1_EL1>()
		.reg::<TPIDR_EL1>()
		.reg::<ERXMISC3_EL1>()
		.reg::<SCXTNUM_EL2>()
		.reg::<OSDTRTX_EL1>()
		.reg_array::<TRCSSPCICRn>()
		.reg::<PMSEVFR_EL1>()
		.reg::<ICC_EOIR0_EL1>()
		.reg::<CNTHV_CTL_EL2>()
		.reg::<SPMIIDR_EL1>()
		.reg::<TRCIDR1>()
		.reg::<VMPIDR_EL2>()
		.reg::<ID_ISAR0_EL1>()
		.reg::<DBGPRCR_EL1>()
		.reg::<AMAIR_EL1>()
		.reg::<ICC_BPR0_EL1>()
		.reg::<ESR_EL1>()
		.reg::<AMCNTENCLR0_EL0>()
		.reg::<APIBKeyLo_EL1>()
		.reg::<MVFR0_EL1>()
		.reg::<SCTLR2_EL1>()
		.reg::<BRBCR_EL1>()
		.reg::<MPAMVPM0_EL2>()
		.reg::<CNTPCT_EL0>()
		.reg::<MPAMHCR_EL2>()
		.reg::<TRCIDR0>()
		.reg::<CNTP_TVAL_EL0>()
		.reg_array::<ICC_AP1Rn_EL1>()
		.reg::<TPIDR_EL0>()
		.reg::<SCXTNUM_EL3>()
		.reg::<ICV_IAR0_EL1>()
		.reg::<MECID_P0_EL2>()
		.reg::<PMBLIMITR_EL1>()
		.reg::<PMCEID0_EL0>()
		.reg::<HDBSSBR_EL2>()
		.reg::<SPSR_irq>()
		.reg::<TRCBBCTLR>()
		.reg::<ID_AA64PFR1_EL1>()
		.reg::<DSPSR_EL0>()
		.reg::<HACDBSBR_EL2>()
		.reg::<CCSIDR2_EL1>()
		.reg::<MAIR_EL1>()
		.reg::<GCR_EL1>()
		.reg::<TRCAUXCTLR>()
		.reg::<DLR_EL0>()
		.reg::<ID_DFR0_EL1>()
		.reg::<ID_AA64MMFR3_EL1>()
		.reg::<ICV_IGRPEN0_EL1>()
		.reg::<HCRX_EL2>()
		.reg::<CCSIDR_EL1>()
		.reg::<ID_PFR2_EL1>()
		.reg::<DBGDTRTX_EL0>()
		.reg::<TRCCIDCCTLR0>()
		.reg::<TRCCIDCCTLR1>()
		.reg::<ICC_IGRPEN0_EL1>()
		.reg_array::<BRBTGTn_EL1>()
		.reg::<RVBAR_EL1>()
		.reg::<APIBKeyHi_EL1>()
		.reg::<AMUSERENR_EL0>()
		.reg_array::<SPMEVFILTRn_EL0>()
		.reg::<MAIR2_EL1>()
		.reg_array::<TRCVMIDCVRn>()
		.reg::<ICC_IAR0_EL1>()
		.reg::<ICC_HPPIR1_EL1>()
		.reg::<OSDTRRX_EL1>()
		.reg::<ID_ISAR2_EL1>()
		.reg::<MAIR2_EL3>()
		.reg::<MAIR_EL2>()
		.reg::<VMECID_P_EL2>()
		.reg::<SPMSELR_EL0>()
		.reg::<DBGCLAIMCLR_EL1>()
		.reg_array::<BRBINFn_EL1>()
		.reg::<PMXEVTYPER_EL0>()
		.reg::<SPMZR_EL0>()
		.reg::<RVBAR_EL3>()
		.reg::<CNTHP_CVAL_EL2>()
		.reg::<BRBINFINJ_EL1>()
		.reg::<ERXMISC1_EL1>()
		.reg_array::<TRCACATRn>()
		.reg::<PMINTENCLR_EL1>()
		.reg::<RVBAR_EL2>()
		.reg::<AMCG1IDR_EL0>()
		.reg::<ICV_BPR0_EL1>()
		.reg::<CNTPS_TVAL_EL1>()
		.reg_array::<AMEVTYPER0n_EL0>()
		.reg_array::<AMEVTYPER1n_EL0>()
		.reg::<MPAMVPM2_EL2>()
		.reg::<MAIR_EL3>()
		.reg::<MVFR2_EL1>()
		.reg::<MAIR2_EL2>()
		.reg::<MECIDR_EL2>()
		.reg::<CNTHVS_CVAL_EL2>()
		.reg_array::<TRCEXTINSELRn>()
		.reg_array::<ICV_AP0Rn_EL1>()
		.reg::<HDBSSPROD_EL2>()
		.reg_array::<AMEVCNTVOFF0n_EL2>()
		.reg_array::<SPMEVCNTRn_EL0>()
		.reg::<ICC_IAR1_EL1>()
		.reg::<ISR_EL1>()
		.reg::<ICC_HPPIR0_EL1>()
		.reg_array::<AMEVCNTVOFF1n_EL2>()
		.reg::<TCR_EL1>()
		.reg::<TRCRSR>()
		.reg::<ID_ISAR3_EL1>()
		.reg::<TRCDEVID>()
		.reg::<ERXMISC0_EL1>()
		.reg::<ICV_DIR_EL1>()
		.reg::<ICC_IGRPEN1_EL3>()
		.reg::<MPAMVPMV_EL2>()
		.reg::<PMECR_EL1>()
		.reg::<ICV_BPR1_EL1>()
		.reg::<PFAR_EL1>()
		.reg_array::<TRCCNTRLDVRn>()
		.reg::<CNTP_CTL_EL0>()
		.reg::<SMPRI_EL1>()
		.reg::<PMCCFILTR_EL0>()
		.reg::<MPAMVPM3_EL2>()
		.reg::<TRCVIIECTLR>()
		.reg::<OSLAR_EL1>()
		.reg::<DCZID_EL0>()
		.reg::<HACR_EL2>()
		.reg::<CTR_EL0>()
		.reg::<ID_AFR0_EL1>()
		.reg::<SPMCNTENSET_EL0>()
		.reg::<ID_AA64DFR2_EL1>()
		.reg::<PAR_EL1>()
		.reg::<PMSDSFR_EL1>()
		.reg::<MPIDR_EL1>()
		.reg::<TRCCCCTLR>()
		.reg::<TCR_EL2>()
		.reg::<AIDR_EL1>()
		.reg::<ID_AA64PFR0_EL1>()
		.reg::<APDBKeyLo_EL1>()
		.reg::<ICV_IGRPEN1_EL1>()
		.reg::<TRBIDR_EL1>()
		.reg::<SMPRIMAP_EL2>()
		.reg::<SPMCFGR_EL1>()
		.reg::<VTCR_EL2>()
		.reg::<OSECCR_EL1>()
		.reg::<ID_AA64MMFR2_EL1>()
		.reg::<ID_DFR1_EL1>()
		.reg::<CNTHP_CTL_EL2>()
		.reg_array::<ICC_AP0Rn_EL1>()
		.reg::<PFAR_EL2>()
		.reg::<DBGDTR_EL0>()
		.reg::<ICC_IGRPEN1_EL1>()
		.reg::<CNTPOFF_EL2>()
		.reg::<TRCIMSPEC0>()
		.reg::<HFGRTR_EL2>()
		.reg::<TCR_EL3>()
		.reg::<FPSR>()
		.reg::<ERXPFGCDN_EL1>()
		.reg::<PMSIRR_EL1>()
		.reg::<HACDBSCONS_EL2>()
		.reg::<CNTHV_CVAL_EL2>()
		.reg::<TTBR0_EL2>()
		.reg::<LORSA_EL1>()
		.reg::<ICC_CTLR_EL3>()
		.reg::<TRFCR_EL1>()
		.reg::<AMAIR2_EL2>()
		.reg::<BRBIDR0_EL1>()
		.reg::<PMSICR_EL1>()
		.reg::<SCTLR_EL1>()
		.reg::<MPAM1_EL1>()
		.reg::<ICH_HCR_EL2>()
		.reg::<IFSR32_EL2>()
		.reg::<APIAKeyHi_EL1>()
		.reg::<CPTR_EL3>()
		.reg::<SPMDEVARCH_EL1>()
		.reg::<ICH_VMCR_EL2>()
		.reg::<TRCQCTLR>()
		.reg::<PMMIR_EL1>()
		.reg::<ERXFR_EL1>()
		.reg::<CPTR_EL2>()
		.reg::<CNTHPS_CVAL_EL2>()
		.reg::<VSTTBR_EL2>()
		.reg::<SPMROOTCR_EL3>()
		.reg::<TRCSTATR>()
		.reg::<HSTR_EL2>()
		.reg::<APGAKeyLo_EL1>()
		.reg::<TRBBASER_EL1>()
		.reg::<ID_MMFR3_EL1>()
		.reg::<PMICNTR_EL0>()
		.reg::<PMIAR_EL1>()
		.reg::<RMR_EL1>()
		.reg::<AMAIR2_EL3>()
		.reg::<PIRE0_EL1>()
		.reg::<TTBR0_EL3>()
		.reg::<TRFCR_EL2>()
		.reg::<TTBR0_EL1>()
		.reg::<AMAIR2_EL1>()
		.reg::<RCWSMASK_EL1>()
		.reg::<RMR_EL3>()
		.reg::<AMCFGR_EL0>()
		.reg::<LORID_EL1>()
		.reg::<SCTLR_EL2>()
		.reg::<SDER32_EL3>()
		.reg::<SPMCNTENCLR_EL0>()
		.reg::<BRBTS_EL1>()
		.reg::<MFAR_EL3>()
		.reg::<TRCEVENTCTL0R>()
		.reg::<PMICNTSVR_EL1>()
		.reg::<VSTCR_EL2>()
		.reg::<MPAMVPM6_EL2>()
		.reg::<DBGAUTHSTATUS_EL1>()
		.reg::<CNTVCTSS_EL0>()
		.reg::<ID_AA64ISAR2_EL1>()
		.reg::<VMECID_A_EL2>()
		.reg::<ID_ISAR6_EL1>()
		.reg::<ICV_HPPIR1_EL1>()
		.reg::<REVIDR_EL1>()
		.reg::<HFGWTR_EL2>()
		.reg::<SDER32_EL2>()
		.reg::<FGWTE3_EL3>()
		.reg::<PMXEVCNTR_EL0>()
		.reg_array::<SPMEVTYPERn_EL0>()
		.reg::<SCTLR_EL3>()
		.reg::<HDFGWTR2_EL2>()
		.reg::<RMR_EL2>()
		.reg::<CNTV_CVAL_EL0>()
		.reg::<ERRSELR_EL1>()
		.reg::<GPTBR_EL3>()
		.reg::<PMSWINC_EL0>()
		.reg::<PIRE0_EL2>()
		.reg::<ICC_RPR_EL1>()
		.reg::<TRCOSLSR>()
		.reg::<ICC_PMR_EL1>()
		.reg::<ICC_CTLR_EL1>()
		.reg_array::<BRBSRCn_EL1>()
		.reg::<ID_AA64SMFR0_EL1>()
		.reg::<ZCR_EL3>()
		.reg::<ERXSTATUS_EL1>()
		.reg::<TTBR1_EL1>()
		.reg::<CNTVOFF_EL2>()
		.reg::<TCR2_EL1>()
		.reg::<MDCR_EL2>()
		.reg::<TRCITEEDCR>()
		.reg::<SPMACCESSR_EL3>()
		.reg::<MPAMVPM7_EL2>()
		.reg::<POR_EL2>()
		.reg::<GCSPR_EL3>()
		.reg::<FAR_EL2>()
		.reg::<TFSR_EL2>()
		.reg_array::<TRCCNTCTLRn>()
		.reg::<ICV_HPPIR0_EL1>()
		.reg::<HDFGRTR2_EL2>()
		.reg::<TFSR_EL3>()
		.reg::<TRCEVENTCTL1R>()
		.reg::<FAR_EL3>()
		.reg::<GCSPR_EL2>()
		.reg::<POR_EL3>()
		.reg::<HFGITR_EL2>()
		.reg::<ID_AA64ISAR3_EL1>()
		.reg::<SPMACCESSR_EL2>()
		.reg::<MDSTEPOP_EL1>()
		.reg::<ERXADDR_EL1>()
		.reg::<PMUACR_EL1>()
		.reg::<MDCR_EL3>()
		.reg::<PMSCR_EL1>()
		.reg_array::<TRCSSCSRn>()
		.reg::<ZCR_EL2>()
		.reg::<ERXCTLR_EL1>()
		.reg::<DBGCLAIMSET_EL1>()
		.reg::<DBGVCR32_EL2>()
		.reg::<RCWMASK_EL1>()
		.reg::<TRCSEQRSTEVR>()
		.reg::<TCR2_EL2>()
		.reg::<TTBR1_EL2>()
		.reg_array::<TRCIMSPECn>()
		.reg::<PMBIDR_EL1>()
		.reg::<PMZR_EL0>()
		.reg::<POR_EL1>()
		.reg::<GCSPR_EL0>()
		.reg::<FAR_EL1>()
		.reg::<MPAM0_EL1>()
		.reg::<CNTP_CVAL_EL0>()
		.reg::<TFSR_EL1>()
		.reg_array::<ICH_AP1Rn_EL2>()
		.reg::<APDAKeyLo_EL1>()
		.reg::<S2PIR_EL2>()
		.reg::<GCSPR_EL1>()
		.reg::<POR_EL0>()
		.reg_array::<TRCCNTVRn>()
		.reg::<SPMACCESSR_EL1>()
		.reg::<TFSRE0_EL1>()
		.reg::<AMCR_EL0>()
		.reg::<PMINTENSET_EL1>()
		.reg::<PMSCR_EL2>()
		.reg::<ZCR_EL1>()
		.reg::<ID_MMFR2_EL1>()
		.reg::<CONTEXTIDR_EL1>()
		.reg::<MPAMVPM5_EL2>()
		.reg::<MIDR_EL1>()
		.reg::<LORN_EL1>()
		.reg::<VSESR_EL2>()
		.reg::<AFSR1_EL2>()
		.reg::<MECID_A0_EL2>()
		.reg::<MPAM2_EL2>()
		.reg::<ICH_EISR_EL2>()
		.reg::<SPMDEVAFF_EL1>()
		.reg_array::<TRCACVRn>()
		.reg::<SP_EL2>()
		.reg::<APDAKeyHi_EL1>()
		.reg_array::<DBGBVRn_EL1>()
		.reg::<CNTHVS_CTL_EL2>()
		.reg::<PIR_EL2>()
		.reg::<PIR_EL3>()
		.reg::<MECID_RL_A_EL3>()
		.reg_array::<ICH_LRn_EL2>()
		.reg::<PMCNTENSET_EL0>()
		.reg::<S2POR_EL1>()
		.reg::<SP_EL3>()
		.reg_array::<SPMEVFILT2Rn_EL0>()
		.reg::<HCR_EL2>()
		.reg::<VSESR_EL3>()
		.reg::<AFSR1_EL3>()
		.reg::<ID_ISAR5_EL1>()
		.reg::<TRCPRGCTLR>()
		.reg::<ID_AA64ISAR1_EL1>()
		.reg::<MDCCINT_EL1>()
		.reg_array::<DBGBCRn_EL1>()
		.reg::<TRCCLAIMCLR>()
		.reg_array::<TRCRSCTLRn>()
		.reg::<SPMOVSSET_EL0>()
		.reg::<PMUSERENR_EL0>()
		.reg::<CONTEXTIDR_EL2>()
		.reg::<CNTPS_CVAL_EL1>()
		.reg::<TRCCLAIMSET>()
		.reg::<FPCR>()
		.reg::<AFSR1_EL1>()
		.reg::<CNTPCTSS_EL0>()
		.reg::<CNTHVS_TVAL_EL2>()
		.reg::<SP_EL1>()
		.reg::<TRBPTR_EL1>()
		.reg::<CLIDR_EL1>()
		.reg::<PIR_EL1>()
		.reg::<TRCVISSCTLR>()
		.reg::<RNDR>()
		.reg::<ID_AA64MMFR4_EL1>()
		.reg::<DISR_EL1>()
		.reg::<ID_MMFR0_EL1>()
		.reg::<SP_EL0>()
		.reg::<CNTHP_TVAL_EL2>()
		.reg_array::<SPMCGCRn_EL1>()
		.reg::<TRBSR_EL1>()
		.reg::<TRCCONFIGR>()
		.reg::<TRCDEVARCH>()
		.reg::<AFSR0_EL1>()
		.reg::<RNDRRS>()
		.reg::<ICH_MISR_EL2>()
		.reg::<PMSIDR_EL1>()
		.reg::<TRCSTALLCTLR>()
		.reg::<PMCCNTR_EL0>()
		.reg_array::<TRCCIDCVRn>()
		.reg::<SPMINTENCLR_EL1>()
		.reg::<BRBSRCINJ_EL1>()
		.reg::<VPIDR_EL2>()
		.reg::<VNCR_EL2>()
		.reg::<CSSELR_EL1>()
		.reg::<TRCIDR13>()
		.reg::<GCSCR_EL2>()
		.reg::<ERXPFGCTL_EL1>()
		.reg::<MDSCR_EL1>()
		.reg::<HDFGWTR_EL2>()
		.reg::<ID_MMFR1_EL1>()
		.reg::<TRBMPAM_EL1>()
		.reg::<GCSCR_EL3>()
		.reg::<TRCIDR12>()
		.reg::<HAFGRTR_EL2>()
		.reg::<VTTBR_EL2>()
		.reg::<OSDLR_EL1>()
		.reg::<PMCCNTSVR_EL1>()
		.reg::<PMOVSCLR_EL0>()
		.reg::<CNTHPS_CTL_EL2>()
		.reg::<PMCR_EL0>()
		.reg::<TRCSYNCPR>()
		.reg::<AFSR0_EL2>()
		.reg::<MDSELR_EL1>()
		.reg::<PMSNEVFR_EL1>()
		.reg::<ERXPFGF_EL1>()
		.reg::<MECID_A1_EL2>()
		.reg::<MPAMVPM4_EL2>()
		.reg::<ICV_CTLR_EL1>()
		.reg::<FPEXC32_EL2>()
		.reg::<TPIDRRO_EL0>()
		.reg::<CNTHV_TVAL_EL2>()
		.reg::<TRCIDR8>()
		.reg::<CNTHCTL_EL2>()
		.reg::<GCSCR_EL1>()
		.reg::<TRCIDR10>()
		.reg::<APGAKeyHi_EL1>()
		.reg_array::<PMEVCNTRn_EL0>()
		.reg::<DACR32_EL2>()
		.reg::<ICV_PMR_EL1>()
		.reg_array::<AMEVCNTR0n_EL0>()
		.reg_array::<AMEVCNTR1n_EL0>()
		.reg::<ICV_RPR_EL1>()
		.reg::<TRCIDR11>()
		.reg::<PMBSR_EL1>()
		.reg::<CNTFRQ_EL0>()
		.reg::<TRCIDR9>()
		.reg::<TRCAUTHSTATUS>()
		.reg::<ICV_NMIAR1_EL1>()
		.reg::<APIAKeyLo_EL1>()
		.reg::<ID_AA64ISAR0_EL1>()
		.reg_array::<ICH_AP0Rn_EL2>()
		.reg::<MPAM3_EL3>()
		.reg::<ID_ISAR4_EL1>()
		.reg::<AFSR0_EL3>()
;
	}
}
