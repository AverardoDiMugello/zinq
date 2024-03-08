use std::fmt;

/*
 * Enums for integer operations
 */

#[derive(Debug, Clone, Copy)]
pub enum MoveWideOp {
    N,
    Z,
    K,
}

#[derive(Debug, Clone, Copy)]
pub enum ShiftType {
    LSL,
    LSR,
    ASR,
    ROR,
}

impl From<(bool, bool)> for ShiftType {
    fn from(value: (bool, bool)) -> Self {
        let (high, low) = value;
        match (high, low) {
            (false, false) => ShiftType::LSL,
            (false, true) => ShiftType::LSR,
            (true, false) => ShiftType::ASR,
            (true, true) => ShiftType::ROR,
        }
    }
}

impl fmt::Display for ShiftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LSL => write!(f, "LSL"),
            Self::LSR => write!(f, "LSR"),
            Self::ASR => write!(f, "ASR"),
            Self::ROR => write!(f, "ROR"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogicalOp {
    AND,
    EOR,
    ORR,
}

#[derive(Debug, Clone, Copy)]
pub enum CountOp {
    CLZ,
    CLS,
    CNT,
}

/*
 * Enums for floating point and SIMD operations
 */

#[derive(Debug, Clone, Copy)]
pub enum FPMaxMinOp {
    MAX,
    MIN,
    MAXNUM,
    MINNUM,
}

#[derive(Debug, Clone, Copy)]
pub enum FPUnaryOp {
    ABS,
    MOV,
    NEG,
    SQRT,
}

#[derive(Debug, Clone, Copy)]
pub enum FPConvOp {
    CvtFtoI,
    CvtItoF,
    MovFtoI,
    MovItoF,
    CvtFtoIJs,
}

#[derive(Debug, Clone, Copy)]
pub enum FPRounding {
    TIEEVEN,
    POSINF,
    NEGINF,
    ZERO,
    TIEAWAY,
    ODD,
}

/*
 * Enums for memory operations
 */
#[derive(Debug, Clone, Copy)]
pub enum MemOp {
    LOAD,
    STORE,
    PREFETCH,
}

#[derive(Debug, Clone, Copy)]
pub enum MemAtomicOp {
    GCSSS1,
    ADD,
    BIC,
    EOR,
    ORR,
    SMAX,
    SMIN,
    UMAX,
    UMIN,
    SWP,
    CAS,
}

#[derive(Debug, Clone, Copy)]
pub enum MOPSStage {
    Prologue,
    Main,
    Epilogue,
}

/*
 * Enums for system operations
 */

/// Arm condition codes
#[derive(Debug, Clone, Copy)]
pub enum CondCode {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    NV,
}

impl From<u8> for CondCode {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => Self::EQ,
            0b0001 => Self::NE,
            0b0010 => Self::CS,
            0b0011 => Self::CC,
            0b0100 => Self::MI,
            0b0101 => Self::PL,
            0b0110 => Self::VS,
            0b0111 => Self::VC,
            0b1000 => Self::HI,
            0b1001 => Self::LS,
            0b1010 => Self::GE,
            0b1011 => Self::LT,
            0b1100 => Self::GT,
            0b1101 => Self::LE,
            0b1110 => Self::AL,
            0b1111 => Self::NV,
            _ => panic!(
                "Invalid A64 condition code. Arm instructions should never allow this to happen."
            ),
        }
    }
}

// Assembly helpers

impl fmt::Display for CondCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CondCode::EQ => write!(f, "EQ"),
            CondCode::NE => write!(f, "NE"),
            CondCode::CS => write!(f, "CS"),
            CondCode::CC => write!(f, "CC"),
            CondCode::MI => write!(f, "MI"),
            CondCode::PL => write!(f, "PL"),
            CondCode::VS => write!(f, "VS"),
            CondCode::VC => write!(f, "VC"),
            CondCode::HI => write!(f, "HI"),
            CondCode::LS => write!(f, "LS"),
            CondCode::GE => write!(f, "GE"),
            CondCode::LT => write!(f, "LT"),
            CondCode::GT => write!(f, "GT"),
            CondCode::LE => write!(f, "LE"),
            CondCode::AL => write!(f, "AL"),
            CondCode::NV => write!(f, "NV"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchType {
    DIRCALL,
    INDCALL,
    ERET,
    DBGEXIT,
    RET,
    DIR,
    INDIR,
    EXCEPTION,
    TMFAIL,
    RESET,
    UNKNOWN,
}

#[derive(Debug, Clone, Copy)]
pub enum MBReqDomain {
    Nonshareable,
    InnerShareable,
    OuterShareable,
    FullSystem,
}

#[derive(Debug, Clone, Copy)]
pub enum MBReqTypes {
    Reads,
    Writes,
    All,
}
