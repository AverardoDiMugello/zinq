use super::{
    generic::{Arith, Bitwise, Logic, ReadCtx, Term},
    Eval, EvalCtx,
};

/// Expressions that operate on and return boolean data types
#[derive(Debug)]
pub enum ExprBool<'ctx> {
    Term(Term<bool>),
    Logic(Logic<bool>),
    ReadCtx(ReadCtx<'ctx, bool>),
}

impl<'ctx, E: EvalCtx<bool>> Eval<E> for ExprBool<'ctx> {
    type Output = bool;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Term(t) => t.eval(eval_ctx),
            Self::Logic(expr) => expr.eval(eval_ctx),
            Self::ReadCtx(expr) => expr.read(),
        }
    }
}

/// Expressions that operate on and return 32-bit data types
#[derive(Debug)]
pub enum Expr32<'ctx> {
    Term(Term<u32>),
    Logic(Logic<u32>),
    Arith(Arith<u32>),
    Bitwise(Bitwise<u32>),
    ReadCtx(ReadCtx<'ctx, u32>),
}

impl<'ctx, E: EvalCtx<u32>> Eval<E> for Expr32<'ctx> {
    type Output = u32;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Term(t) => t.eval(eval_ctx),
            Self::Logic(expr) => expr.eval(eval_ctx),
            Self::Arith(expr) => expr.eval(eval_ctx),
            Self::Bitwise(expr) => expr.eval(eval_ctx),
            Self::ReadCtx(expr) => expr.read(),
        }
    }
}

/// Expressions that operate on and return 64-bit data types
#[derive(Debug)]
pub enum Expr64<'ctx> {
    Term(Term<u64>),
    Logic(Logic<u64>),
    Arith(Arith<u64>),
    Bitwise(Bitwise<u64>),
    ReadCtx(ReadCtx<'ctx, u64>),
}

impl<'ctx, E: EvalCtx<u64>> Eval<E> for Expr64<'ctx> {
    type Output = u64;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Term(t) => t.eval(eval_ctx),
            Self::Logic(expr) => expr.eval(eval_ctx),
            Self::Arith(expr) => expr.eval(eval_ctx),
            Self::Bitwise(expr) => expr.eval(eval_ctx),
            Self::ReadCtx(expr) => expr.read(),
        }
    }
}

/// Expressions that operate on and return 128-bit data types
#[derive(Debug)]
pub enum Expr128<'ctx> {
    Term(Term<u128>),
    Logic(Logic<u128>),
    Arith(Arith<u128>),
    Bitwise(Bitwise<u128>),
    ReadCtx(ReadCtx<'ctx, u128>),
}

impl<'ctx, E: EvalCtx<u128>> Eval<E> for Expr128<'ctx> {
    type Output = u128;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Term(t) => t.eval(eval_ctx),
            Self::Logic(expr) => expr.eval(eval_ctx),
            Self::Arith(expr) => expr.eval(eval_ctx),
            Self::Bitwise(expr) => expr.eval(eval_ctx),
            Self::ReadCtx(expr) => expr.read(),
        }
    }
}
