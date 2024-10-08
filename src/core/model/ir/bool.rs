use crate::core::model::ir::*;

/// An operation that either produces a Bool or writes a Bool to a register
#[derive(Clone, Copy)]
pub enum BoolOp {
    // Logic
    And(Bit, Bit),
    Not(Bit),
    Or(Bit, Bit),
    Xor(Bit, Bit),
    // Bit equality
    Eq(Bit, Bit),
    Ne(Bit, Bit),
    // Ternary
    Select(Bit, Bit, Bit),
    // Int equality
    EqInt8(Bv8, Bv8),
    NeInt8(Bv8, Bv8),
    EqInt16(Bv16, Bv16),
    NeInt16(Bv16, Bv16),
    EqInt32(Bv32, Bv32),
    NeInt32(Bv32, Bv32),
    EqInt64(Bv64, Bv64),
    NeInt64(Bv64, Bv64),
    // Int8 ordering
    SLtInt8(Bv8, Bv8),
    SLeInt8(Bv8, Bv8),
    SGtInt8(Bv8, Bv8),
    SGeInt8(Bv8, Bv8),
    ULtInt8(Bv8, Bv8),
    ULeInt8(Bv8, Bv8),
    UGtInt8(Bv8, Bv8),
    UGeInt8(Bv8, Bv8),
    // Int16 ordering
    SLtInt16(Bv16, Bv16),
    SLeInt16(Bv16, Bv16),
    SGtInt16(Bv16, Bv16),
    SGeInt16(Bv16, Bv16),
    ULtInt16(Bv16, Bv16),
    ULeInt16(Bv16, Bv16),
    UGtInt16(Bv16, Bv16),
    UGeInt16(Bv16, Bv16),
    // Int32 ordering
    SLtInt32(Bv32, Bv32),
    SLeInt32(Bv32, Bv32),
    SGtInt32(Bv32, Bv32),
    SGeInt32(Bv32, Bv32),
    ULtInt32(Bv32, Bv32),
    ULeInt32(Bv32, Bv32),
    UGtInt32(Bv32, Bv32),
    UGeInt32(Bv32, Bv32),
    // Int64 ordering
    SLtInt64(Bv64, Bv64),
    SLeInt64(Bv64, Bv64),
    SGtInt64(Bv64, Bv64),
    SGeInt64(Bv64, Bv64),
    ULtInt64(Bv64, Bv64),
    ULeInt64(Bv64, Bv64),
    UGtInt64(Bv64, Bv64),
    UGeInt64(Bv64, Bv64),
    // Float equality
    EqFloat16(Bv16, Bv16),
    NeFloat16(Bv16, Bv16),
    EqFloat32(Bv32, Bv32),
    NeFloat32(Bv32, Bv32),
    EqFloat64(Bv64, Bv64),
    NeFloat64(Bv64, Bv64),
    // Float ordering
    LtFloat16(Bv16, Bv16),
    LeFloat16(Bv16, Bv16),
    GtFloat16(Bv16, Bv16),
    GeFloat16(Bv16, Bv16),
    LtFloat32(Bv32, Bv32),
    LeFloat32(Bv32, Bv32),
    GtFloat32(Bv32, Bv32),
    GeFloat32(Bv32, Bv32),
    LtFloat64(Bv64, Bv64),
    LeFloat64(Bv64, Bv64),
    GtFloat64(Bv64, Bv64),
    GeFloat64(Bv64, Bv64),
    // Bit access
    NthBitBv8(Bv8, Bv8),
    NthBitBv16(Bv16, Bv16),
    NthBitBv32(Bv32, Bv32),
    NthBitBv64(Bv64, Bv64),
}

impl<P: Processor> IrBlock<P> {
    pub fn and_b(&mut self, lhs: Bit, rhs: Bit) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::And(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn not_b(&mut self, val: Bit) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::Not(val))));
        Bit::Var(def)
    }

    pub fn or_b(&mut self, lhs: Bit, rhs: Bit) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::Or(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn xor_b(&mut self, lhs: Bit, rhs: Bit) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::Xor(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn eq_b(&mut self, lhs: Bit, rhs: Bit) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::Eq(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ne_b(&mut self, lhs: Bit, rhs: Bit) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::Ne(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn select_b(&mut self, cond: Bit, t_case: Bit, f_case: Bit) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Bool(BoolOp::Select(cond, t_case, f_case)),
        ));
        Bit::Var(def)
    }

    pub fn eq_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::EqInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ne_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NeInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn slt_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SLtInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sle_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SLeInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sgt_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SGtInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sge_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SGeInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ult_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::ULtInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ule_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::ULeInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ugt_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::UGtInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn uge_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::UGeInt8(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn eq_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::EqInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ne_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NeInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn slt_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SLtInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sle_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SLeInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sgt_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SGtInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sge_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SGeInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ult_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::ULtInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ule_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::ULeInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ugt_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::UGtInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn uge_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::UGeInt16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn eq_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::EqInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ne_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NeInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn slt_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SLtInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sle_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SLeInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sgt_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SGtInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sge_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SGeInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ult_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::ULtInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ule_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::ULeInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ugt_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::UGtInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn uge_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::UGeInt32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn eq_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::EqInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ne_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NeInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn slt_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SLtInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sle_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SLeInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sgt_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SGtInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn sge_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::SGeInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ult_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::ULtInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ule_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::ULeInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ugt_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::UGtInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn uge_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::UGeInt64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn eq_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::EqFloat16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ne_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NeFloat16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn lt_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::LtFloat16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn le_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::LeFloat16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn gt_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::GtFloat16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ge_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::GeFloat16(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn eq_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::EqFloat32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ne_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NeFloat32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn lt_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::LtFloat32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn le_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::LeFloat32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn gt_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::GtFloat32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ge_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::GeFloat32(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn eq_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::EqFloat64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ne_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NeFloat64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn lt_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::LtFloat64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn le_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::LeFloat64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn gt_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::GtFloat64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn ge_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::GeFloat64(lhs, rhs))));
        Bit::Var(def)
    }

    pub fn nth_bit_i8(&mut self, val: Bv8, n: Bv8) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NthBitBv8(val, n))));
        Bit::Var(def)
    }

    pub fn nth_bit_i16(&mut self, val: Bv16, n: Bv16) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NthBitBv16(val, n))));
        Bit::Var(def)
    }

    pub fn nth_bit_i32(&mut self, val: Bv32, n: Bv32) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NthBitBv32(val, n))));
        Bit::Var(def)
    }

    pub fn nth_bit_i64(&mut self, val: Bv64, n: Bv64) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Bool(BoolOp::NthBitBv64(val, n))));
        Bit::Var(def)
    }
}
