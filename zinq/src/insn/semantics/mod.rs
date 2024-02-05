// use std::collections::HashMap;
use std::marker::PhantomData;

pub mod expr;
use expr::*;
pub mod stmt;
use stmt::*;

#[derive(Debug, Clone, Copy)]
pub struct Var<T>(usize, PhantomData<T>);

impl<T> Var<T> {
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct IrBlock<'p> {
    stmts: Vec<Stmt<'p>>,
}

impl<'p> IntoIterator for IrBlock<'p> {
    type Item = Stmt<'p>;
    type IntoIter = std::vec::IntoIter<Stmt<'p>>;

    fn into_iter(self) -> Self::IntoIter {
        self.stmts.into_iter()
    }
}

impl<'p> IrBlock<'p> {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }

    // Build assign statement

    pub fn assign_bool(&mut self, expr: ExprBool<'p>) -> Var<bool> {
        self.stmts.push(Stmt::Assignment(Expr::ExprBool(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_bool_from(&mut self, expr: ToBool) -> Var<bool> {
        self.stmts.push(Stmt::Assignment(Expr::ToBool(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_32(&mut self, expr: Expr32<'p>) -> Var<u32> {
        self.stmts.push(Stmt::Assignment(Expr::Expr32(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_32_from(&mut self, expr: To32) -> Var<u32> {
        self.stmts.push(Stmt::Assignment(Expr::To32(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_64(&mut self, expr: Expr64<'p>) -> Var<u64> {
        self.stmts.push(Stmt::Assignment(Expr::Expr64(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_64_from(&mut self, expr: To64) -> Var<u64> {
        self.stmts.push(Stmt::Assignment(Expr::To64(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_128(&mut self, expr: Expr128<'p>) -> Var<u128> {
        self.stmts.push(Stmt::Assignment(Expr::Expr128(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_128_from(&mut self, expr: To128) -> Var<u128> {
        self.stmts.push(Stmt::Assignment(Expr::To128(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_addr_from(&mut self, expr: ToAddr) -> Var<usize> {
        self.stmts.push(Stmt::Assignment(Expr::ToAddr(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    // Build processor write statements

    pub fn proc_write_addr(&mut self, ctx_var: &'p usize, val: Term<usize>) {
        self.stmts
            .push(Stmt::ProcWrite(ProcWrite::WriteAddr { ctx_var, val }));
    }

    pub fn proc_write_bool(&mut self, ctx_var: &'p bool, val: Term<bool>) {
        self.stmts
            .push(Stmt::ProcWrite(ProcWrite::WriteBool { ctx_var, val }));
    }

    pub fn proc_write_32(&mut self, ctx_var: &'p u32, val: Term<u32>) {
        self.stmts
            .push(Stmt::ProcWrite(ProcWrite::Write32 { ctx_var, val }));
    }

    pub fn proc_write_64(&mut self, ctx_var: &'p u64, val: Term<u64>) {
        self.stmts
            .push(Stmt::ProcWrite(ProcWrite::Write64 { ctx_var, val }));
    }

    pub fn proc_write_128(&mut self, ctx_var: &'p u128, val: Term<u128>) {
        self.stmts
            .push(Stmt::ProcWrite(ProcWrite::Write128 { ctx_var, val }));
    }

    // Build branch statements

    pub fn br_uncond(&mut self, addr: Term<usize>) {
        self.stmts.push(Stmt::Branch(Branch::Uncond(addr)));
    }

    pub fn br_cond(&mut self, cond: Term<bool>, true_case: Term<usize>, false_case: Term<usize>) {
        self.stmts.push(Stmt::Branch(Branch::Cond {
            cond,
            true_case,
            false_case,
        }));
    }

    // INCOMPLETE Build memory transaction statements

    pub fn read_mem_32(&mut self, addr: usize) -> Var<u32> {
        self.stmts.push(Stmt::MemTx(MemTx::Read32(addr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn write_mem_32(&mut self, addr: usize, word: Term<u32>) {
        self.stmts.push(Stmt::MemTx(MemTx::Write32(addr, word)))
    }

    pub fn read_mem_64(&mut self, addr: usize) -> Var<u64> {
        self.stmts.push(Stmt::MemTx(MemTx::Read64(addr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn write_mem_64(&mut self, addr: usize, word: Term<u64>) {
        self.stmts.push(Stmt::MemTx(MemTx::Write64(addr, word)))
    }
}
