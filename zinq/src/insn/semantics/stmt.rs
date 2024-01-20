use super::expr::*;

/// Statements alter the execution contexts (Processor(s), Memory, and Emulation)
#[derive(Debug)]
pub enum Stmt<'p> {
    Assignment(Expr<'p>),
    WriteProc(WriteProc<'p>),
    Branch(Branch<'p>),
    MemTx(MemTx),
    Termination,
}

/// Statements for writing to processor context
#[derive(Debug)]
pub enum WriteProc<'p> {
    WriteBool { ctx_var: &'p bool, val: Term<bool> },
    Write32 { ctx_var: &'p u32, val: Term<u32> },
    Write64 { ctx_var: &'p u64, val: Term<u64> },
    Write128 { ctx_var: &'p u128, val: Term<u128> },
}

impl<'p> WriteProc<'p> {
    pub fn execute<E>(self, eval_ctx: &E)
    where
        E: EvalCtx<bool> + EvalCtx<u32> + EvalCtx<u64> + EvalCtx<u128>,
    {
        match self {
            Self::WriteBool { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(eval_ctx))
            }
            Self::Write32 { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(eval_ctx))
            }
            Self::Write64 { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(eval_ctx))
            }
            Self::Write128 { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(eval_ctx))
            }
        };
    }

    fn mutate_immutable_var<T: Copy>(var: &T, val: T) {
        let var = var as *const T;
        let var = var.cast_mut();

        unsafe {
            *var = val;
        }
    }
}

#[derive(Debug)]
pub enum Branch<'p> {
    IfElse {
        cond: ExprBool<'p>,
        true_case: usize,
        false_case: usize,
    },
    Goto32(Term<u32>),
    Goto64(Term<u64>),
}

#[derive(Debug)]
pub enum MemTx {
    Read32(usize),
    Write32(usize, Term<u32>),
    Read64(usize),
    Write64(usize, Term<u64>),
}
