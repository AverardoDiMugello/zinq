use std::marker::PhantomData;

use num::{One, Unsigned, Zero};

use super::{generic::Term, Eval, EvalCtx};

/// Expressions for converting non-bool expressions into bool expressions
#[derive(Debug)]
pub enum ToBool {
    From32(Term<u32>),
    From64(Term<u64>),
    From128(Term<u128>),
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
    E: EvalCtx<bool> + EvalCtx<T>,
{
    type Output = bool;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Eq { lhs, rhs } => lhs.eval(eval_ctx) == rhs.eval(eval_ctx),
            Self::Gt { lhs, rhs } => lhs.eval(eval_ctx) > rhs.eval(eval_ctx),
            Self::Gte { lhs, rhs } => lhs.eval(eval_ctx) >= rhs.eval(eval_ctx),
            Self::Lt { lhs, rhs } => lhs.eval(eval_ctx) < rhs.eval(eval_ctx),
            Self::Lte { lhs, rhs } => lhs.eval(eval_ctx) <= rhs.eval(eval_ctx),
            Self::Neq { lhs, rhs } => lhs.eval(eval_ctx) != rhs.eval(eval_ctx),
        }
    }
}

/// Expressions for converting non-32-bit expressions into 32-bit expressions
#[derive(Debug)]
pub enum To32 {
    FromBool(Term<bool>),
    From64(TruncConv<u64, u32>),
    From128(TruncConv<u128, u32>),
}

/// Expressions for converting non-64-bit expressions into 64-bit expressions
#[derive(Debug)]
pub enum To64 {
    FromBool(Term<bool>),
    From32(ExtConv<u32, u64>),
    From128(TruncConv<u128, u64>),
}

/// Expressions for converting non-128-bit expressions into 128-bit expressions
#[derive(Debug)]
pub enum To128 {
    FromBool(Term<bool>),
    From32(ExtConv<u32, u128>),
    From64(ExtConv<u64, u128>),
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

impl<E: EvalCtx<u32> + EvalCtx<u64>> Eval<E> for ExtConv<u32, u64> {
    type Output = u64;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(eval_ctx) as i32 as i64 as u64,
            Self::Zero(i, _) => i.eval(eval_ctx) as u64,
        }
    }
}

impl<E: EvalCtx<u32> + EvalCtx<u128>> Eval<E> for ExtConv<u32, u128> {
    type Output = u128;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(eval_ctx) as i32 as i128 as u128,
            Self::Zero(i, _) => i.eval(eval_ctx) as u128,
        }
    }
}

impl<E: EvalCtx<u64> + EvalCtx<u128>> Eval<E> for ExtConv<u64, u128> {
    type Output = u128;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(eval_ctx) as i64 as i128 as u128,
            Self::Zero(i, _) => i.eval(eval_ctx) as u128,
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

impl<E: EvalCtx<u64> + EvalCtx<u32>> Eval<E> for TruncConv<u64, u32> {
    type Output = u32;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(eval_ctx) as i64 as i32 as u32,
            Self::Zero(i, _) => i.eval(eval_ctx) as u32,
        }
    }
}

impl<E: EvalCtx<u128> + EvalCtx<u32>> Eval<E> for TruncConv<u128, u32> {
    type Output = u32;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(eval_ctx) as i128 as i32 as u32,
            Self::Zero(i, _) => i.eval(eval_ctx) as u32,
        }
    }
}

impl<E: EvalCtx<u128> + EvalCtx<u64>> Eval<E> for TruncConv<u128, u64> {
    type Output = u64;

    fn eval(self, eval_ctx: &E) -> Self::Output {
        match self {
            Self::Signed(i, _) => i.eval(eval_ctx) as i128 as i64 as u64,
            Self::Zero(i, _) => i.eval(eval_ctx) as u64,
        }
    }
}

/// Should this be here? Evaluation helper that converts to a bool
pub fn to_bool_from_num<T, E>(t: Term<T>, eval_ctx: &E) -> bool
where
    T: Zero + Copy,
    E: EvalCtx<T>,
{
    !t.eval(eval_ctx).is_zero()
}

/// Should this be here? Evaluation helper that converts from a bool
pub fn to_num_from_bool<T, E>(t: Term<bool>, eval_ctx: &E) -> T
where
    T: One + Zero + Copy,
    E: EvalCtx<bool>,
{
    if t.eval(eval_ctx) {
        T::one()
    } else {
        T::zero()
    }
}
