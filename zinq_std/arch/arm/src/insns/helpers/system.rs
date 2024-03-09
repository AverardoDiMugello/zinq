use bitvec::prelude::*;
use zinq::insn::semantics::*;

use super::enums::*;
use crate::Arm;
// IR helpers

/// Generate ConditionHolds
pub fn condition_holds<'p>(cond: CondCode, ctx: &'p Arm, code: &mut IrCtx<'p>) -> Var {
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
