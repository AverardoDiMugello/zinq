use super::Var;

mod convert;
pub use convert::*;
mod generic;
pub use generic::*;
mod typed;
pub use typed::*;

/// Expressions are organized based on types
/// Expr[T] are "typed" expressions that operate on and produce data of type T
/// To[T] are "conversion" expressions that operate on one type and produce data of another type T
#[derive(Debug)]
pub enum Expr<'p> {
    // Typed expressions
    ExprBool(ExprBool<'p>),
    Expr32(Expr32<'p>),
    Expr64(Expr64<'p>),
    Expr128(Expr128<'p>),
    // Conversion expressions
    ToBool(ToBool),
    To32(To32),
    To64(To64),
    To128(To128),
}

/// Look-up interface for implementors of the Eval trait. The look-up cannot fail
pub trait EvalCtx<T: Copy> {
    fn look_up(&self, v: Var<T>) -> T;
}

/// Evaluate an expression
pub trait Eval<E: EvalCtx<Self::Output>> {
    type Output: Copy;

    fn eval(self, emu_ctx: &E) -> Self::Output;
}
