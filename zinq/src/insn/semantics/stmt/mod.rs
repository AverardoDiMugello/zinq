pub mod macros;

use super::expr::*;

/// Statements alter the execution contexts (Processor(s), Memory, and Emulation)
#[derive(Debug)]
pub enum Stmt<'p> {
    Assignment(Expr<'p>),
    ProcWrite(ProcWrite<'p>),
    Branch(Branch),
    MemTx(MemTx),
}

/// Statements for writing to processor context
#[derive(Debug)]
pub enum ProcWrite<'p> {
    WriteAddr {
        ctx_var: &'p usize,
        val: Term<usize>,
    },
    WriteBool {
        ctx_var: &'p bool,
        val: Term<bool>,
    },
    Write32 {
        ctx_var: &'p u32,
        val: Term<u32>,
    },
    Write64 {
        ctx_var: &'p u64,
        val: Term<u64>,
    },
    Write128 {
        ctx_var: &'p u128,
        val: Term<u128>,
    },
}

impl<'p> ProcWrite<'p> {
    pub fn execute<E>(self, exec_ctx: &E)
    where
        E: ExecCtx<bool> + ExecCtx<u32> + ExecCtx<u64> + ExecCtx<u128> + ExecCtx<usize>,
    {
        match self {
            Self::WriteAddr { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(exec_ctx))
            }
            Self::WriteBool { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(exec_ctx))
            }
            Self::Write32 { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(exec_ctx))
            }
            Self::Write64 { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(exec_ctx))
            }
            Self::Write128 { ctx_var, val } => {
                Self::mutate_immutable_var(ctx_var, val.eval(exec_ctx))
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
pub enum Branch {
    Cond {
        cond: Term<bool>,
        true_case: Term<usize>,
        false_case: Term<usize>,
    },
    Uncond(Term<usize>),
}

#[derive(Debug)]
pub enum MemTx {
    Read32(usize),
    Write32(usize, Term<u32>),
    Read64(usize),
    Write64(usize, Term<u64>),
}
