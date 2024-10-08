use crate::core::model::ir::*;

/// An operation that interprets the operands as an IEEE 754-2008 binary16
#[derive(Clone, Copy)]
pub enum Float16Op {
    // Arithmetic
    Add(Bv16, Bv16),
    Div(Bv16, Bv16),
    Mul(Bv16, Bv16),
    Rem(Bv16, Bv16),
    Sub(Bv16, Bv16),
    // Min/max
    Max(Bv16, Bv16),
    Min(Bv16, Bv16),
    // Conversion
    FromFloat32(Bv32),
    FromFloat64(Bv64),
}

/// An operation that interprets the operands as an IEEE 754-2008 binary32
#[derive(Clone, Copy)]
pub enum Float32Op {
    // Arithmetic
    Add(Bv32, Bv32),
    Div(Bv32, Bv32),
    Mul(Bv32, Bv32),
    Rem(Bv32, Bv32),
    Sub(Bv32, Bv32),
    // Min/max
    Max(Bv32, Bv32),
    Min(Bv32, Bv32),
    // Conversion
    FromFloat16(Bv16),
    FromFloat64(Bv64),
}

/// An operation that interprets the operands as an IEEE 754-2008 binary64
#[derive(Clone, Copy)]
pub enum Float64Op {
    // Arithmetic
    Add(Bv64, Bv64),
    Div(Bv64, Bv64),
    Mul(Bv64, Bv64),
    Rem(Bv64, Bv64),
    Sub(Bv64, Bv64),
    // Min/max
    Max(Bv64, Bv64),
    Min(Bv64, Bv64),
    // Conversion
    FromFloat16(Bv16),
    FromFloat32(Bv32),
}

impl<P: Processor> IrBlock<P> {
    pub fn add_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::Add(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn div_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::Div(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn mul_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::Mul(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn rem_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::Rem(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn sub_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::Sub(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn max_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::Max(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn min_f16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::Min(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn f32_to_f16(&mut self, val: Bv32) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::FromFloat32(val))));
        Bv16::Var(def)
    }

    pub fn f64_to_f16(&mut self, val: Bv64) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float16(Float16Op::FromFloat64(val))));
        Bv16::Var(def)
    }

    pub fn add_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::Add(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn div_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::Div(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn mul_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::Mul(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn rem_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::Rem(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn sub_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::Sub(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn max_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::Max(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn min_f32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::Min(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn f16_to_f32(&mut self, val: Bv16) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::FromFloat16(val))));
        Bv32::Var(def)
    }

    pub fn f64_to_f32(&mut self, val: Bv64) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float32(Float32Op::FromFloat64(val))));
        Bv32::Var(def)
    }

    pub fn add_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::Add(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn div_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::Div(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn mul_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::Mul(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn rem_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::Rem(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn sub_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::Sub(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn max_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::Max(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn min_f64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::Min(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn f16_to_f64(&mut self, val: Bv16) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::FromFloat16(val))));
        Bv64::Var(def)
    }

    pub fn f32_to_f64(&mut self, val: Bv32) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Float64(Float64Op::FromFloat32(val))));
        Bv64::Var(def)
    }
}
