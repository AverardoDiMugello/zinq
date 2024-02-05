use std::marker::PhantomData;

use num::Unsigned;

use super::{generic::Term, Eval, ExecCtx};

/// Expressions for converting non-bool expressions into bool expressions
#[derive(Debug)]
pub enum ToBool {
    Cmp32(Cmp<u32>),
    Cmp64(Cmp<u64>),
    Cmp128(Cmp<u128>),
}

/// Comparison expressions
#[derive(Debug)]
pub enum Cmp<T>
where
    T: PartialOrd + PartialEq + Copy,
{
    Eq { lhs: Term<T>, rhs: Term<T> },
    Gt { lhs: Term<T>, rhs: Term<T> },
    Gte { lhs: Term<T>, rhs: Term<T> },
    Lt { lhs: Term<T>, rhs: Term<T> },
    Lte { lhs: Term<T>, rhs: Term<T> },
    Neq { lhs: Term<T>, rhs: Term<T> },
}

impl<T, E> Eval<E> for Cmp<T>
where
    T: PartialOrd + PartialEq + Copy,
    E: ExecCtx<bool> + ExecCtx<T>,
{
    type Output = bool;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Eq { lhs, rhs } => lhs.eval(exec_ctx) == rhs.eval(exec_ctx),
            Self::Gt { lhs, rhs } => lhs.eval(exec_ctx) > rhs.eval(exec_ctx),
            Self::Gte { lhs, rhs } => lhs.eval(exec_ctx) >= rhs.eval(exec_ctx),
            Self::Lt { lhs, rhs } => lhs.eval(exec_ctx) < rhs.eval(exec_ctx),
            Self::Lte { lhs, rhs } => lhs.eval(exec_ctx) <= rhs.eval(exec_ctx),
            Self::Neq { lhs, rhs } => lhs.eval(exec_ctx) != rhs.eval(exec_ctx),
        }
    }
}

/// Expressions for converting non-32-bit expressions into 32-bit expressions
#[derive(Debug)]
pub enum To32 {
    From64(TruncConv<u64, u32>),
    From128(TruncConv<u128, u32>),
}

/// Expressions for converting non-64-bit expressions into 64-bit expressions
#[derive(Debug)]
pub enum To64 {
    From32(ExtConv<u32, u64>),
    From128(TruncConv<u128, u64>),
}

/// Expressions for converting non-128-bit expressions into 128-bit expressions
#[derive(Debug)]
pub enum To128 {
    From32(ExtConv<u32, u128>),
    From64(ExtConv<u64, u128>),
}

/// Expressions for converting 32-bit or 64-bit values into addresses
#[derive(Debug)]
pub enum ToAddr {
    From32(Term<u32>),
    From64(Term<u64>),
}

/// Expressions that convert from a terminal of a different type by extending it
#[derive(Debug)]
pub enum ExtConv<I, O>
where
    I: Unsigned + Copy,
    O: Unsigned + Copy,
{
    Signed(Term<I>, PhantomData<O>),
    Zero(Term<I>, PhantomData<O>),
}

/*
 * Can't figure out how to do this generically yet. Must unit test
 */

impl<E: ExecCtx<u32> + ExecCtx<u64>> Eval<E> for ExtConv<u32, u64> {
    type Output = u64;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(exec_ctx) as i32 as i64 as u64,
            Self::Zero(i, _) => i.eval(exec_ctx) as u64,
        }
    }
}

impl<E: ExecCtx<u32> + ExecCtx<u128>> Eval<E> for ExtConv<u32, u128> {
    type Output = u128;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(exec_ctx) as i32 as i128 as u128,
            Self::Zero(i, _) => i.eval(exec_ctx) as u128,
        }
    }
}

impl<E: ExecCtx<u64> + ExecCtx<u128>> Eval<E> for ExtConv<u64, u128> {
    type Output = u128;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => {
                let i = i.eval(exec_ctx);
                println!("{i}");
                let i = i as i64;
                println!("{i}");
                let i = i128::from(i);
                println!("{i}");
                let i = i as u128;
                println!("{i}");
                i
                // i128::from(i.eval(exec_ctx) as i64) as u128
            }
            Self::Zero(i, _) => u128::from(i.eval(exec_ctx)),
        }
    }
}

/// Expressions that convert from a terminal of a different type by truncating it
#[derive(Debug)]
pub enum TruncConv<I, O>
where
    I: Unsigned + Copy,
    O: Unsigned + Copy,
{
    Signed(Term<I>, PhantomData<O>),
    Zero(Term<I>, PhantomData<O>),
}

/*
 * Can't figure out how to do this generically yet. Must unit test
 */

impl<E: ExecCtx<u64> + ExecCtx<u32>> Eval<E> for TruncConv<u64, u32> {
    type Output = u32;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(exec_ctx) as i64 as i32 as u32,
            Self::Zero(i, _) => i.eval(exec_ctx) as u32,
        }
    }
}

impl<E: ExecCtx<u128> + ExecCtx<u32>> Eval<E> for TruncConv<u128, u32> {
    type Output = u32;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(exec_ctx) as i128 as i32 as u32,
            Self::Zero(i, _) => i.eval(exec_ctx) as u32,
        }
    }
}

impl<E: ExecCtx<u128> + ExecCtx<u64>> Eval<E> for TruncConv<u128, u64> {
    type Output = u64;

    fn eval(self, exec_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(exec_ctx) as i128 as i64 as u64,
            Self::Zero(i, _) => i.eval(exec_ctx) as u64,
        }
    }
}
