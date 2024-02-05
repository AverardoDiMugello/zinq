use super::{
    generic::{Arith, Bitwise, Logic, ReadProc, Term},
    Eval, ExecCtx,
};

/// Expressions that operate on and return boolean data types
#[derive(Debug)]
pub enum ExprBool<'ctx> {
    Term(Term<bool>),
    Equals(Term<bool>, Term<bool>),
    Logic(Logic<bool>),
    ReadProc(ReadProc<'ctx, bool>),
}

impl<'ctx, E: ExecCtx<bool>> Eval<E> for ExprBool<'ctx> {
    type Output = bool;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Term(t) => t.eval(exec_ctx),
            Self::Equals(t1, t2) => t1.eval(exec_ctx) == t2.eval(exec_ctx),
            Self::Logic(expr) => expr.eval(exec_ctx),
            Self::ReadProc(expr) => expr.eval(exec_ctx),
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
    ReadProc(ReadProc<'ctx, u32>),
}

impl<'ctx, E: ExecCtx<u32>> Eval<E> for Expr32<'ctx> {
    type Output = u32;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Term(t) => t.eval(exec_ctx),
            Self::Logic(expr) => expr.eval(exec_ctx),
            Self::Arith(expr) => expr.eval(exec_ctx),
            Self::Bitwise(expr) => expr.eval(exec_ctx),
            Self::ReadProc(expr) => expr.eval(exec_ctx),
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
    ReadProc(ReadProc<'ctx, u64>),
}

impl<'ctx, E: ExecCtx<u64>> Eval<E> for Expr64<'ctx> {
    type Output = u64;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Term(t) => t.eval(exec_ctx),
            Self::Logic(expr) => expr.eval(exec_ctx),
            Self::Arith(expr) => expr.eval(exec_ctx),
            Self::Bitwise(expr) => expr.eval(exec_ctx),
            Self::ReadProc(expr) => expr.eval(exec_ctx),
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
    ReadProc(ReadProc<'ctx, u128>),
}

impl<'ctx, E: ExecCtx<u128>> Eval<E> for Expr128<'ctx> {
    type Output = u128;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Term(t) => t.eval(exec_ctx),
            Self::Logic(expr) => expr.eval(exec_ctx),
            Self::Arith(expr) => expr.eval(exec_ctx),
            Self::Bitwise(expr) => expr.eval(exec_ctx),
            Self::ReadProc(expr) => expr.eval(exec_ctx),
        }
    }
}
