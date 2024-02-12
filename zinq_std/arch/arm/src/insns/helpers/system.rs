use std::fmt;

use bitvec::prelude::*;
use zinq::insn::semantics::*;

use crate::Arm;

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

// IR helpers

/// Generate ConditionHolds
pub fn condition_holds<'p>(cond: CondCode, ctx: &'p Arm, code: &mut IrBlock<'p>) -> Var {
    let result = match cond {
        CondCode::EQ | CondCode::NE => {
            let z = code.assign(Expr::ReadProc(&ctx.pstate.z));
            let result = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(z),
                Term::Lit(bitvec!(1)),
                false,
            ));
            result
        }
        CondCode::CS | CondCode::CC => {
            let c = code.assign(Expr::ReadProc(&ctx.pstate.c));
            let result = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(c),
                Term::Lit(bitvec!(1)),
                false,
            ));
            result
        }
        CondCode::MI | CondCode::PL => {
            let n = code.assign(Expr::ReadProc(&ctx.pstate.n));
            let result = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(n),
                Term::Lit(bitvec!(1)),
                false,
            ));
            result
        }
        CondCode::VS | CondCode::VC => {
            let v = code.assign(Expr::ReadProc(&ctx.pstate.v));
            let result = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(v),
                Term::Lit(bitvec!(1)),
                false,
            ));
            result
        }
        CondCode::HI | CondCode::LS => {
            let c = code.assign(Expr::ReadProc(&ctx.pstate.c));
            let z = code.assign(Expr::ReadProc(&ctx.pstate.z));
            let res1 = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(c),
                Term::Lit(bitvec!(1)),
                false,
            ));
            let res2 = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(z),
                Term::Lit(bitvec!(0)),
                false,
            ));
            let result = code.assign(Expr::Binary(
                BinOp::And,
                Term::Var(res1),
                Term::Var(res2),
                false,
            ));
            result
        }
        CondCode::GE | CondCode::LT => {
            let n = code.assign(Expr::ReadProc(&ctx.pstate.n));
            let v = code.assign(Expr::ReadProc(&ctx.pstate.v));
            let result = code.assign(Expr::Binary(BinOp::Eq, Term::Var(n), Term::Var(v), false));
            result
        }
        CondCode::GT | CondCode::LE => {
            let n = code.assign(Expr::ReadProc(&ctx.pstate.n));
            let v = code.assign(Expr::ReadProc(&ctx.pstate.v));
            let res1 = code.assign(Expr::Binary(BinOp::Eq, Term::Var(n), Term::Var(v), false));

            let z = code.assign(Expr::ReadProc(&ctx.pstate.z));
            let res2 = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(z),
                Term::Lit(bitvec!(0)),
                false,
            ));

            let result = code.assign(Expr::Binary(
                BinOp::And,
                Term::Var(res1),
                Term::Var(res2),
                false,
            ));
            result
        }
        CondCode::AL | CondCode::NV => {
            let result = code.assign(Expr::Term(Term::Lit(bitvec!(1))));
            result
        }
    };

    let result = match cond {
        CondCode::NE
        | CondCode::CC
        | CondCode::PL
        | CondCode::VC
        | CondCode::LS
        | CondCode::LT
        | CondCode::LE => code.assign(Expr::Unary(UnaOp::Not, Term::Var(result))),
        _ => result,
    };

    result
}
