use std::{
    fmt::Debug,
    ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr, Sub},
};

use super::{super::Var, Eval, ExecCtx};

/// All expressions terminate in a typed literal or variable
#[derive(Debug)]
pub enum Term<T: Copy> {
    Lit(T),
    Var(Var<T>),
}

impl<T, E> Eval<E> for Term<T>
where
    T: Copy,
    E: ExecCtx<T>,
{
    type Output = T;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Lit(v) => v,
            Self::Var(v) => exec_ctx.look_up(v),
        }
    }
}

/// Logical expressions
#[derive(Debug)]
pub enum Logic<T>
where
    T: BitAnd<Output = T> + Not<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Copy,
{
    And { a: Term<T>, b: Term<T> },
    Not(Term<T>),
    Or { a: Term<T>, b: Term<T> },
    Xor { a: Term<T>, b: Term<T> },
}

impl<T, E> Eval<E> for Logic<T>
where
    T: BitAnd<Output = T> + Not<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Copy,
    E: ExecCtx<T>,
{
    type Output = T;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::And { a, b } => a.eval(exec_ctx) & b.eval(exec_ctx),
            Self::Not(a) => !a.eval(exec_ctx),
            Self::Or { a, b } => a.eval(exec_ctx) | b.eval(exec_ctx),
            Self::Xor { a, b } => a.eval(exec_ctx) ^ b.eval(exec_ctx),
        }
    }
}

/// Arithmetic expressions
#[derive(Debug)]
pub enum Arith<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    Add { a: Term<T>, b: Term<T> },
    Sub { a: Term<T>, b: Term<T> },
}

impl<T, E> Eval<E> for Arith<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
    E: ExecCtx<T>,
{
    type Output = T;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Add { a, b } => a.eval(exec_ctx) + b.eval(exec_ctx),
            Self::Sub { a, b } => a.eval(exec_ctx) - b.eval(exec_ctx),
        }
    }
}

/// Bitwise expressions
#[derive(Debug)]
pub enum Bitwise<T>
where
    T: Shl<Output = T> + Shr<Output = T> + Copy,
{
    ShiftL { val: Term<T>, amt: Term<T> },
    ShiftR { val: Term<T>, amt: Term<T> },
}

impl<T, E> Eval<E> for Bitwise<T>
where
    T: Shl<Output = T> + Shr<Output = T> + Copy,
    E: ExecCtx<T>,
{
    type Output = T;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::ShiftL { val, amt } => val.eval(exec_ctx) << amt.eval(exec_ctx),
            Self::ShiftR { val, amt } => val.eval(exec_ctx) >> amt.eval(exec_ctx),
        }
    }
}

/// Ternary expression
#[derive(Debug)]
pub struct Select<T: Copy> {
    cond: Term<bool>,
    true_case: Term<T>,
    false_case: Term<T>,
}

impl<T, E> Eval<E> for Select<T>
where
    T: Copy,
    E: ExecCtx<bool> + ExecCtx<T>,
{
    type Output = T;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        if self.cond.eval(exec_ctx) {
            self.true_case.eval(exec_ctx)
        } else {
            self.false_case.eval(exec_ctx)
        }
    }
}

/// Context read expression
#[derive(Debug)]
pub struct ReadProc<'p, T: Copy>(&'p T);

impl<'p, T: Copy> ReadProc<'p, T> {
    pub fn new(v: &'p T) -> Self {
        Self(v)
    }
}

impl<'p, T, E> Eval<E> for ReadProc<'p, T>
where
    T: Copy,
    E: ExecCtx<T>,
{
    type Output = T;

    fn eval(self, _exec_ctx: &E) -> Self::Output {
        *self.0
    }
}
