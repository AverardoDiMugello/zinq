use crate::core::model::ir::*;

#[derive(Clone, Copy)]
pub enum CallParam {
    Bit(Bit),
    Bv8(Bv8),
    Bv16(Bv16),
    Bv32(Bv32),
    Bv64(Bv64),
    Bv128(Bv128),
}

#[derive(Clone, Copy)]
pub enum CallArg {
    Bit(bool),
    Bv8(u8),
    Bv16(u16),
    Bv32(u32),
    Bv64(u64),
    Bv128(u128),
}

pub enum CallResult<T, P: Processor> {
    Ok(T),
    Excep(P::ExcepAttrs),
}

pub enum CallOp<P: Processor> {
    Unit {
        f: Box<dyn for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>)>,
        args: Vec<CallParam>,
    },
    Bit {
        f: Box<dyn for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> bool>,
        args: Vec<CallParam>,
    },
    Bv8 {
        f: Box<dyn for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u8>,
        args: Vec<CallParam>,
    },
    Bv16 {
        f: Box<dyn for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u16>,
        args: Vec<CallParam>,
    },
    Bv32 {
        f: Box<dyn for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u32>,
        args: Vec<CallParam>,
    },
    Bv64 {
        f: Box<dyn for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u64>,
        args: Vec<CallParam>,
    },
    Bv128 {
        f: Box<dyn for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u128>,
        args: Vec<CallParam>,
    },
}

impl<P: Processor> IrBlock<P> {
    pub fn call0<F>(&mut self, f: F, args: Vec<CallParam>)
    where
        F: 'static + for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>),
    {
        self.ops.push((
            IRRes::none(),
            IROp::Call(CallOp::Unit {
                f: Box::new(f),
                args,
            }),
        ));
    }

    pub fn call_b<F>(&mut self, f: F, args: Vec<CallParam>) -> Bit
    where
        F: 'static + for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> bool,
    {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Call(CallOp::Bit {
                f: Box::new(f),
                args,
            }),
        ));
        Bit::Var(def)
    }

    pub fn call_i8<F>(&mut self, f: F, args: Vec<CallParam>) -> Bv8
    where
        F: 'static + for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u8,
    {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Call(CallOp::Bv8 {
                f: Box::new(f),
                args,
            }),
        ));
        Bv8::Var(def)
    }

    pub fn call_i16<F>(&mut self, f: F, args: Vec<CallParam>) -> Bv16
    where
        F: 'static + for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u16,
    {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Call(CallOp::Bv16 {
                f: Box::new(f),
                args,
            }),
        ));
        Bv16::Var(def)
    }

    pub fn call_i32<F>(&mut self, f: F, args: Vec<CallParam>) -> Bv32
    where
        F: 'static + for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u32,
    {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Call(CallOp::Bv32 {
                f: Box::new(f),
                args,
            }),
        ));
        Bv32::Var(def)
    }

    pub fn call_i64<F>(&mut self, f: F, args: Vec<CallParam>) -> Bv64
    where
        F: 'static + for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u64,
    {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Call(CallOp::Bv64 {
                f: Box::new(f),
                args,
            }),
        ));
        Bv64::Var(def)
    }

    pub fn call_i128<F>(&mut self, f: F, args: Vec<CallParam>) -> Bv128
    where
        F: 'static + for<'a> FnMut(&P, &mut ProcCtx<'a, P>, Vec<CallArg>) -> u128,
    {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Call(CallOp::Bv128 {
                f: Box::new(f),
                args,
            }),
        ));
        Bv128::Var(def)
    }
}
