use std::marker::PhantomData;

pub mod expr;
// use expr::{convert::*, generic::*, typed::*, *};
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
        Self {
            stmts: Vec::new(),
            // vals_bool: HashMap::new(),
            // vals_32: HashMap::new(),
            // vals_64: HashMap::new(),
            // vals_128: HashMap::new(),
        }
    }

    pub fn assign_bool(&mut self, expr: ExprBool<'p>) -> Var<bool> {
        self.stmts.push(Stmt::Assignment(Expr::ExprBool(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_bool_conv(&mut self, expr: ToBool) -> Var<bool> {
        self.stmts.push(Stmt::Assignment(Expr::ToBool(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_32(&mut self, expr: Expr32<'p>) -> Var<u32> {
        self.stmts.push(Stmt::Assignment(Expr::Expr32(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_32_conv(&mut self, expr: To32) -> Var<u32> {
        self.stmts.push(Stmt::Assignment(Expr::To32(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_64(&mut self, expr: Expr64<'p>) -> Var<u64> {
        self.stmts.push(Stmt::Assignment(Expr::Expr64(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_64_conv(&mut self, expr: To64) -> Var<u64> {
        self.stmts.push(Stmt::Assignment(Expr::To64(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_128(&mut self, expr: Expr128<'p>) -> Var<u128> {
        self.stmts.push(Stmt::Assignment(Expr::Expr128(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn assign_128_conv(&mut self, expr: To128) -> Var<u128> {
        self.stmts.push(Stmt::Assignment(Expr::To128(expr)));

        Var(self.stmts.len() - 1, PhantomData)
    }

    pub fn write_proc_bool(&mut self, ctx_var: &'p bool, val: Term<bool>) {
        self.stmts
            .push(Stmt::WriteProc(WriteProc::WriteBool { ctx_var, val }));
    }

    pub fn write_proc_32(&mut self, ctx_var: &'p u32, val: Term<u32>) {
        self.stmts
            .push(Stmt::WriteProc(WriteProc::Write32 { ctx_var, val }));
    }

    pub fn write_proc_64(&mut self, ctx_var: &'p u64, val: Term<u64>) {
        self.stmts
            .push(Stmt::WriteProc(WriteProc::Write64 { ctx_var, val }));
    }

    pub fn write_proc_128(&mut self, ctx_var: &'p u128, val: Term<u128>) {
        self.stmts
            .push(Stmt::WriteProc(WriteProc::Write128 { ctx_var, val }));
    }

    pub fn goto_32(&mut self, addr: Term<u32>) {
        self.stmts.push(Stmt::Branch(Branch::Goto32(addr)));
    }

    pub fn goto_64(&mut self, addr: Term<u64>) {
        self.stmts.push(Stmt::Branch(Branch::Goto64(addr)));
    }

    pub fn if_else(&mut self, cond: ExprBool<'p>, true_case: usize, false_case: usize) {
        self.stmts.push(Stmt::Branch(Branch::IfElse {
            cond,
            true_case,
            false_case,
        }));
    }

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
