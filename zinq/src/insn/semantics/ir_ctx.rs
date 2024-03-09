use super::ir::*;

/// An IrCtx holds Stmts, providing a builder API for constructing Ir code
#[derive(Debug)]
pub struct IrCtx<'p> {
    stmts: Vec<Stmt<'p>>,
}

impl<'p> IntoIterator for IrCtx<'p> {
    type Item = Stmt<'p>;
    type IntoIter = std::vec::IntoIter<Stmt<'p>>;

    fn into_iter(self) -> Self::IntoIter {
        self.stmts.into_iter()
    }
}

impl<'p> IrCtx<'p> {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }

    pub fn assign_i1(&mut self, expr: ExprI1<'p>) -> Var<bool> {
        self.stmts.push(Stmt::Assignment(Expr::I1(expr)));
        return Var::new(self.stmts.len() - 1);
    }

    pub fn assign_i1_conv(&mut self, expr: ToI1) -> Var<bool> {
        self.stmts.push(Stmt::Assignment(Expr::ToI1(expr)));
        return Var::new(self.stmts.len() - 1);
    }

    pub fn assign_i32(&mut self, expr: ExprI32<'p>) -> Var<I32> {
        self.stmts.push(Stmt::Assignment(Expr::I32(expr)));
        return Var::new(self.stmts.len() - 1);
    }

    pub fn assign_i32_conv(&mut self, expr: ToI32) -> Var<I32> {
        self.stmts.push(Stmt::Assignment(Expr::ToI32(expr)));
        return Var::new(self.stmts.len() - 1);
    }

    pub fn assign_i64(&mut self, expr: ExprI64<'p>) -> Var<I64> {
        self.stmts.push(Stmt::Assignment(Expr::I64(expr)));
        return Var::new(self.stmts.len() - 1);
    }

    pub fn assign_i64_conv(&mut self, expr: ToI64) -> Var<I64> {
        self.stmts.push(Stmt::Assignment(Expr::ToI64(expr)));
        return Var::new(self.stmts.len() - 1);
    }

    pub fn assign_i128(&mut self, expr: ExprI128<'p>) -> Var<I128> {
        self.stmts.push(Stmt::Assignment(Expr::I128(expr)));
        return Var::new(self.stmts.len() - 1);
    }

    pub fn assign_i128_conv(&mut self, expr: ToI128) -> Var<I128> {
        self.stmts.push(Stmt::Assignment(Expr::ToI128(expr)));
        return Var::new(self.stmts.len() - 1);
    }

    pub fn write_proc_i1(&mut self, ctx_var: &'p bool, val: Term<I1>) {
        self.stmts
            .push(Stmt::WriteProc(WriteProc::WriteI1 { ctx_var, val }));
    }

    pub fn write_proc_i32(&mut self, ctx_var: &'p u32, val: Term<I32>) {
        self.stmts
            .push(Stmt::WriteProc(WriteProc::WriteI32 { ctx_var, val }));
    }

    pub fn write_proc_i64(&mut self, ctx_var: &'p u64, val: Term<I64>) {
        self.stmts
            .push(Stmt::WriteProc(WriteProc::WriteI64 { ctx_var, val }));
    }

    pub fn write_proc_i128(&mut self, ctx_var: &'p u128, val: Term<I128>) {
        self.stmts
            .push(Stmt::WriteProc(WriteProc::WriteI128 { ctx_var, val }));
    }

    pub fn br_cond(&mut self, cond: Term<I1>, t_case: Term<usize>, f_case: Term<usize>) {
        self.stmts.push(Stmt::Branch(Branch::Cond {
            cond,
            t_case,
            f_case,
        }));
    }

    pub fn br_uncond(&mut self, addr: Term<usize>) {
        self.stmts.push(Stmt::Branch(Branch::Uncond(addr)));
    }
}
