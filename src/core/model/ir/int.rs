use crate::core::model::ir::*;

/// An operation that either produces an Int8 or writes an Int8 to a register or memory
#[derive(Clone, Copy)]
pub enum Int8Op {
    // Logic
    And(Bv8, Bv8),
    Not(Bv8),
    Or(Bv8, Bv8),
    Xor(Bv8, Bv8),
    // Bit manipulation
    Asr(Bv8, Bv8),
    // Note: when these two instructions were made, they were the only trinary IROps. A trinary
    // op forces the entire enum and every enum that uses it to be at least 3 * usize big, even
    // though the overwhelming majority of variants could be at most 2 * usize big. In order to
    // keep size down, the third argument, which is the bit shift, can only be a literal u8.
    // This keeps the size at 2 * usize + u8. Maybe this will become moot in the future if more
    // trinary ops are needed.
    Fshl(Bv8, Bv8, u8),
    Fshr(Bv8, Bv8, u8),
    Lsl(Bv8, Bv8),
    Lsr(Bv8, Bv8),
    Ror(Bv8, Bv8),
    BitRev(Bv8),
    Ctls(Bv8),
    Ctlz(Bv8),
    Ctset(Bv8),
    Cttz(Bv8),
    // Arithmetic
    Abs(Bv8),
    SAdd(Bv8, Bv8),
    SDiv(Bv8, Bv8),
    SMul(Bv8, Bv8),
    SRem(Bv8, Bv8),
    SSub(Bv8, Bv8),
    UAdd(Bv8, Bv8),
    UDiv(Bv8, Bv8),
    UMul(Bv8, Bv8),
    URem(Bv8, Bv8),
    USub(Bv8, Bv8),
    // Min/max
    SMax(Bv8, Bv8),
    SMin(Bv8, Bv8),
    UMax(Bv8, Bv8),
    UMin(Bv8, Bv8),
    // Ternary
    // Note: we now have a full-fledge trinary op
    Select(Bit, Bv8, Bv8),
    // Conversion
    All(Bit),
    FromBit(Bit),
    ReplBit(Bit, Bv8),
    Split(Bv16),
    TruncBv16(Bv16),
    TruncBv32(Bv32),
    TruncBv64(Bv64),
}

/// An operation that either produces an Int16 or writes an Int16 to a register or memory
#[derive(Clone, Copy)]
pub enum Int16Op {
    // Logic
    And(Bv16, Bv16),
    Not(Bv16),
    Or(Bv16, Bv16),
    Xor(Bv16, Bv16),
    // Bit manipulation
    Asr(Bv16, Bv16),
    Fshl(Bv16, Bv16, u8), // See note at Int8Op on trinary ops
    Fshr(Bv16, Bv16, u8),
    Lsl(Bv16, Bv16),
    Lsr(Bv16, Bv16),
    Ror(Bv16, Bv16),
    BitRev(Bv16),
    ByteRev(Bv16),
    Ctls(Bv16),
    Ctlz(Bv16),
    Cttz(Bv16),
    Ctset(Bv16),
    // Arithmetic
    Abs(Bv16),
    SAdd(Bv16, Bv16),
    SDiv(Bv16, Bv16),
    SMul(Bv16, Bv16),
    SRem(Bv16, Bv16),
    SSub(Bv16, Bv16),
    UAdd(Bv16, Bv16),
    UDiv(Bv16, Bv16),
    UMul(Bv16, Bv16),
    URem(Bv16, Bv16),
    USub(Bv16, Bv16),
    // Min/max
    SMax(Bv16, Bv16),
    SMin(Bv16, Bv16),
    UMax(Bv16, Bv16),
    UMin(Bv16, Bv16),
    // Ternary
    Select(Bit, Bv16, Bv16),
    // Conversion
    All(Bit),
    FromBit(Bit),
    ReplBit(Bit, Bv16),
    Concat(Bv8, Bv8),
    Split(Bv32),
    SextBv8(Bv8),
    TruncBv32(Bv32),
    TruncBv64(Bv64),
    ZextBv8(Bv8),
}

/// An operation that either produces an Int32 or writes an Int32 to a register or memory
#[derive(Clone, Copy)]
pub enum Int32Op {
    // Logic
    And(Bv32, Bv32),
    Not(Bv32),
    Or(Bv32, Bv32),
    Xor(Bv32, Bv32),
    // Bit manipulation
    Asr(Bv32, Bv32),
    Fshl(Bv32, Bv32, u8), // See note at Int8Op on trinary ops
    Fshr(Bv32, Bv32, u8),
    Lsl(Bv32, Bv32),
    Lsr(Bv32, Bv32),
    Ror(Bv32, Bv32),
    BitRev(Bv32),
    ByteRev(Bv32),
    Ctls(Bv32),
    Ctlz(Bv32),
    Ctset(Bv32),
    Cttz(Bv32),
    // Arithmetic
    Abs(Bv32),
    SAdd(Bv32, Bv32),
    SDiv(Bv32, Bv32),
    SMul(Bv32, Bv32),
    SRem(Bv32, Bv32),
    SSub(Bv32, Bv32),
    UAdd(Bv32, Bv32),
    UDiv(Bv32, Bv32),
    UMul(Bv32, Bv32),
    URem(Bv32, Bv32),
    USub(Bv32, Bv32),
    // Min/max
    SMax(Bv32, Bv32),
    SMin(Bv32, Bv32),
    UMax(Bv32, Bv32),
    UMin(Bv32, Bv32),
    // Ternary
    Select(Bit, Bv32, Bv32),
    // Conversion
    // Concat i8 to i32 blows up the size of the Int32Op enum and all enums that use it
    // Only supporting half-size concats for now
    // Concat(Int8, Int8, Int8, Int8),
    All(Bit),
    FromBit(Bit),
    ReplBit(Bit, Bv32),
    Concat(Bv16, Bv16),
    Split(Bv64),
    SextBv8(Bv8),
    SextBv16(Bv16),
    TruncBv64(Bv64),
    ZextBv8(Bv8),
    ZextBv16(Bv16),
}

/// An operation that either produces an Int64 or writes an Int64 to a register or memory
#[derive(Clone, Copy)]
pub enum Int64Op {
    // Logic
    And(Bv64, Bv64),
    Not(Bv64),
    Or(Bv64, Bv64),
    Xor(Bv64, Bv64),
    // Bit manipulation
    Asr(Bv64, Bv64),
    Fshl(Bv64, Bv64, u8), // See note at Int8Op on trinary ops
    Fshr(Bv64, Bv64, u8),
    Lsl(Bv64, Bv64),
    Lsr(Bv64, Bv64),
    Ror(Bv64, Bv64),
    BitRev(Bv64),
    ByteRev(Bv64),
    Ctls(Bv64),
    Ctlz(Bv64),
    Ctset(Bv64),
    Cttz(Bv64),
    // Arithmetic
    Abs(Bv64),
    SAdd(Bv64, Bv64),
    SDiv(Bv64, Bv64),
    SMul(Bv64, Bv64),
    SRem(Bv64, Bv64),
    SSub(Bv64, Bv64),
    UAdd(Bv64, Bv64),
    UDiv(Bv64, Bv64),
    UMul(Bv64, Bv64),
    URem(Bv64, Bv64),
    USub(Bv64, Bv64),
    // Min/max
    SMax(Bv64, Bv64),
    SMin(Bv64, Bv64),
    UMax(Bv64, Bv64),
    UMin(Bv64, Bv64),
    // Ternary
    Select(Bit, Bv64, Bv64),
    // Conversion
    // Concat i8 to i64 blows up the size of the Int64Op enum and all enums that use it
    // Only supporting half-size concats for now
    // Concat(Int8, Int8, Int8, Int8, Int8, Int8, Int8, Int8),
    // Concat(Int16, Int16, Int16, Int16),
    All(Bit),
    FromBit(Bit),
    ReplBit(Bit, Bv64),
    Concat(Bv32, Bv32),
    Split(Bv128),
    SextBv8(Bv8),
    SextBv16(Bv16),
    SextBv32(Bv32),
    TruncBv128(Bv128),
    ZextBv8(Bv8),
    ZextBv16(Bv16),
    ZextBv32(Bv32),
}

#[derive(Clone, Copy)]
pub enum Int128Op {
    // Bare minimum needed to support the ARM UMULH and SMULH instructions
    Lsr(Bv128, Bv128),
    SMul(Bv128, Bv128),
    UMul(Bv128, Bv128),
    SextBv8(Bv8),
    SextBv16(Bv16),
    SextBv32(Bv32),
    SextBv64(Bv64),
    ZextBv8(Bv8),
    ZextBv16(Bv16),
    ZextBv32(Bv32),
    ZextBv64(Bv64),
    // Bare minimum needed to support ARM SIMDFP
    Select(Bit, Bv128, Bv128),
    ExtBv16Elem0(Bv128),
    ExtBv32Elem0(Bv128),
    ExtBv64Elem0(Bv128),
    InsBv16Elem0(Bv128, Bv16),
    InsBv32Elem0(Bv128, Bv32),
    InsBv64Elem0(Bv128, Bv64),
}

impl<P: Processor> IrBlock<P> {
    pub fn and_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::And(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn not_i8(&mut self, val: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Not(val))));
        Bv8::Var(def)
    }

    pub fn or_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Or(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn xor_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Xor(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn asr_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Asr(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn fshl_i8(&mut self, hi: Bv8, lo: Bv8, shift: u8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Fshl(hi, lo, shift))));
        Bv8::Var(def)
    }

    pub fn fshr_i8(&mut self, hi: Bv8, lo: Bv8, shift: u8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Fshr(hi, lo, shift))));
        Bv8::Var(def)
    }

    pub fn lsl_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Lsl(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn lsr_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Lsr(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn ror_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Ror(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn bit_rev_i8(&mut self, val: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::BitRev(val))));
        Bv8::Var(def)
    }

    pub fn ctls_i8(&mut self, val: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Ctls(val))));
        Bv8::Var(def)
    }

    pub fn ctlz_i8(&mut self, val: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Ctlz(val))));
        Bv8::Var(def)
    }

    pub fn cttz_i8(&mut self, val: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Cttz(val))));
        Bv8::Var(def)
    }

    pub fn abs_i8(&mut self, val: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::Abs(val))));
        Bv8::Var(def)
    }

    pub fn sadd_i8(&mut self, lhs: Bv8, rhs: Bv8) -> (Bv8, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int8(Int8Op::SAdd(lhs, rhs)),
        ));
        (Bv8::Var(result), Bit::Var(overflow))
    }

    pub fn sdiv_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::SDiv(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn smul_i8(&mut self, lhs: Bv8, rhs: Bv8) -> (Bv8, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int8(Int8Op::SMul(lhs, rhs)),
        ));
        (Bv8::Var(result), Bit::Var(overflow))
    }

    pub fn srem_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::SRem(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn ssub_i8(&mut self, lhs: Bv8, rhs: Bv8) -> (Bv8, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int8(Int8Op::SSub(lhs, rhs)),
        ));
        (Bv8::Var(result), Bit::Var(overflow))
    }

    pub fn uadd_i8(&mut self, lhs: Bv8, rhs: Bv8) -> (Bv8, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int8(Int8Op::UAdd(lhs, rhs)),
        ));
        (Bv8::Var(result), Bit::Var(overflow))
    }

    pub fn udiv_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::UDiv(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn umul_i8(&mut self, lhs: Bv8, rhs: Bv8) -> (Bv8, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int8(Int8Op::UMul(lhs, rhs)),
        ));
        (Bv8::Var(result), Bit::Var(overflow))
    }

    pub fn urem_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::URem(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn usub_i8(&mut self, lhs: Bv8, rhs: Bv8) -> (Bv8, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int8(Int8Op::USub(lhs, rhs)),
        ));
        (Bv8::Var(result), Bit::Var(overflow))
    }

    pub fn smax_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::SMax(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn smin_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::SMin(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn umax_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::UMax(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn umin_i8(&mut self, lhs: Bv8, rhs: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::UMin(lhs, rhs))));
        Bv8::Var(def)
    }

    pub fn select_i8(&mut self, cond: Bit, t_case: Bv8, f_case: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Int8(Int8Op::Select(cond, t_case, f_case)),
        ));
        Bv8::Var(def)
    }

    pub fn all_b_in_i8(&mut self, bit: Bit) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::All(bit))));
        Bv8::Var(def)
    }

    pub fn repl_b_in_i8(&mut self, bit: Bit, n: Bv8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::ReplBit(bit, n))));
        Bv8::Var(def)
    }

    pub fn b_to_i8(&mut self, val: Bit) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::FromBit(val))));
        Bv8::Var(def)
    }

    pub fn split_i16_to_i8(&mut self, val: Bv16) -> (Bv8, Bv8) {
        let hi = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let lo = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops
            .push((IRRes::two(hi, lo), IROp::Int8(Int8Op::Split(val))));
        (Bv8::Var(hi), Bv8::Var(lo))
    }

    pub fn trunc_i16_to_i8(&mut self, val: Bv16) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::TruncBv16(val))));
        Bv8::Var(def)
    }

    pub fn trunc_i32_to_i8(&mut self, val: Bv32) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::TruncBv32(val))));
        Bv8::Var(def)
    }

    pub fn trunc_i64_to_i8(&mut self, val: Bv64) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int8(Int8Op::TruncBv64(val))));
        Bv8::Var(def)
    }

    pub fn and_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::And(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn not_i16(&mut self, val: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Not(val))));
        Bv16::Var(def)
    }

    pub fn or_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Or(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn xor_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Xor(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn asr_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Asr(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn fshl_i16(&mut self, hi: Bv16, lo: Bv16, shift: u8) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Fshl(hi, lo, shift))));
        Bv16::Var(def)
    }

    pub fn fshr_i16(&mut self, hi: Bv16, lo: Bv16, shift: u8) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Fshr(hi, lo, shift))));
        Bv16::Var(def)
    }

    pub fn lsl_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Lsl(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn lsr_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Lsr(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn ror_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Ror(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn bit_rev_i16(&mut self, val: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::BitRev(val))));
        Bv16::Var(def)
    }

    pub fn byte_rev_i16(&mut self, val: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::ByteRev(val))));
        Bv16::Var(def)
    }

    pub fn ctls_i16(&mut self, val: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Ctls(val))));
        Bv16::Var(def)
    }

    pub fn ctlz_i16(&mut self, val: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Ctlz(val))));
        Bv16::Var(def)
    }

    pub fn ctset_i16(&mut self, val: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Ctset(val))));
        Bv16::Var(def)
    }

    pub fn cttz_i16(&mut self, val: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Cttz(val))));
        Bv16::Var(def)
    }

    pub fn abs_i16(&mut self, val: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Abs(val))));
        Bv16::Var(def)
    }

    pub fn sadd_i16(&mut self, lhs: Bv16, rhs: Bv16) -> (Bv16, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int16(Int16Op::SAdd(lhs, rhs)),
        ));
        (Bv16::Var(result), Bit::Var(overflow))
    }

    pub fn sdiv_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::SDiv(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn smul_i16(&mut self, lhs: Bv16, rhs: Bv16) -> (Bv16, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int16(Int16Op::SMul(lhs, rhs)),
        ));
        (Bv16::Var(result), Bit::Var(overflow))
    }

    pub fn srem_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::SRem(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn ssub_i16(&mut self, lhs: Bv16, rhs: Bv16) -> (Bv16, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int16(Int16Op::SSub(lhs, rhs)),
        ));
        (Bv16::Var(result), Bit::Var(overflow))
    }

    pub fn uadd_i16(&mut self, lhs: Bv16, rhs: Bv16) -> (Bv16, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int16(Int16Op::UAdd(lhs, rhs)),
        ));
        (Bv16::Var(result), Bit::Var(overflow))
    }

    pub fn udiv_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::UDiv(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn umul_i16(&mut self, lhs: Bv16, rhs: Bv16) -> (Bv16, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int16(Int16Op::UMul(lhs, rhs)),
        ));
        (Bv16::Var(result), Bit::Var(overflow))
    }

    pub fn urem_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::URem(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn usub_i16(&mut self, lhs: Bv16, rhs: Bv16) -> (Bv16, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int16(Int16Op::USub(lhs, rhs)),
        ));
        (Bv16::Var(result), Bit::Var(overflow))
    }

    pub fn smax_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::SMax(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn smin_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::SMin(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn umax_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::UMax(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn umin_i16(&mut self, lhs: Bv16, rhs: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::UMin(lhs, rhs))));
        Bv16::Var(def)
    }

    pub fn select_i16(&mut self, cond: Bit, t_case: Bv16, f_case: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Int16(Int16Op::Select(cond, t_case, f_case)),
        ));
        Bv16::Var(def)
    }

    pub fn all_b_in_i16(&mut self, bit: Bit) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::All(bit))));
        Bv16::Var(def)
    }

    pub fn b_to_i16(&mut self, val: Bit) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::FromBit(val))));
        Bv16::Var(def)
    }

    pub fn repl_b_in_i16(&mut self, bit: Bit, n: Bv16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::ReplBit(bit, n))));
        Bv16::Var(def)
    }

    pub fn concat_i8_to_i16(&mut self, hi: Bv8, lo: Bv8) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::Concat(hi, lo))));
        Bv16::Var(def)
    }

    pub fn split_i32_to_i16(&mut self, val: Bv32) -> (Bv16, Bv16) {
        let hi = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let lo = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops
            .push((IRRes::two(hi, lo), IROp::Int16(Int16Op::Split(val))));
        (Bv16::Var(hi), Bv16::Var(lo))
    }

    pub fn sext_i8_to_i16(&mut self, val: Bv8) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::SextBv8(val))));
        Bv16::Var(def)
    }

    pub fn trunc_i32_to_i16(&mut self, val: Bv32) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::TruncBv32(val))));
        Bv16::Var(def)
    }

    pub fn trunc_i64_to_i16(&mut self, val: Bv64) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::TruncBv64(val))));
        Bv16::Var(def)
    }

    pub fn zext_i8_to_i16(&mut self, val: Bv8) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int16(Int16Op::SextBv8(val))));
        Bv16::Var(def)
    }

    pub fn and_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::And(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn not_i32(&mut self, val: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Not(val))));
        Bv32::Var(def)
    }

    pub fn or_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Or(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn xor_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Xor(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn asr_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Asr(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn fshl_i32(&mut self, hi: Bv32, lo: Bv32, shift: u8) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Fshl(hi, lo, shift))));
        Bv32::Var(def)
    }

    pub fn fshr_i32(&mut self, hi: Bv32, lo: Bv32, shift: u8) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Fshr(hi, lo, shift))));
        Bv32::Var(def)
    }

    pub fn lsl_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Lsl(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn lsr_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Lsr(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn ror_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Ror(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn bit_rev_i32(&mut self, val: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::BitRev(val))));
        Bv32::Var(def)
    }

    pub fn byte_rev_i32(&mut self, val: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::ByteRev(val))));
        Bv32::Var(def)
    }

    pub fn ctls_i32(&mut self, val: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Ctls(val))));
        Bv32::Var(def)
    }

    pub fn ctlz_i32(&mut self, val: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Ctlz(val))));
        Bv32::Var(def)
    }

    pub fn ctset_i32(&mut self, val: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Ctset(val))));
        Bv32::Var(def)
    }

    pub fn cttz_i32(&mut self, val: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Cttz(val))));
        Bv32::Var(def)
    }

    pub fn abs_i32(&mut self, val: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Abs(val))));
        Bv32::Var(def)
    }

    pub fn sadd_i32(&mut self, lhs: Bv32, rhs: Bv32) -> (Bv32, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int32(Int32Op::SAdd(lhs, rhs)),
        ));
        (Bv32::Var(result), Bit::Var(overflow))
    }

    pub fn sdiv_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::SDiv(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn smul_i32(&mut self, lhs: Bv32, rhs: Bv32) -> (Bv32, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int32(Int32Op::SMul(lhs, rhs)),
        ));
        (Bv32::Var(result), Bit::Var(overflow))
    }

    pub fn srem_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::SRem(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn ssub_i32(&mut self, lhs: Bv32, rhs: Bv32) -> (Bv32, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int32(Int32Op::SSub(lhs, rhs)),
        ));
        (Bv32::Var(result), Bit::Var(overflow))
    }

    pub fn uadd_i32(&mut self, lhs: Bv32, rhs: Bv32) -> (Bv32, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int32(Int32Op::UAdd(lhs, rhs)),
        ));
        (Bv32::Var(result), Bit::Var(overflow))
    }

    pub fn udiv_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::UDiv(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn umul_i32(&mut self, lhs: Bv32, rhs: Bv32) -> (Bv32, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int32(Int32Op::UMul(lhs, rhs)),
        ));
        (Bv32::Var(result), Bit::Var(overflow))
    }

    pub fn urem_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::URem(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn usub_i32(&mut self, lhs: Bv32, rhs: Bv32) -> (Bv32, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int32(Int32Op::USub(lhs, rhs)),
        ));
        (Bv32::Var(result), Bit::Var(overflow))
    }

    pub fn smax_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::SMax(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn smin_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::SMin(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn umax_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::UMax(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn umin_i32(&mut self, lhs: Bv32, rhs: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::UMin(lhs, rhs))));
        Bv32::Var(def)
    }

    pub fn select_i32(&mut self, cond: Bit, t_case: Bv32, f_case: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Int32(Int32Op::Select(cond, t_case, f_case)),
        ));
        Bv32::Var(def)
    }

    // Concat i8 to i32 blows up the size of the Int32Op enum and all enums that use it
    // Only supporting half-size concats for now

    // pub fn concat_i8_to_i32(
    //     &mut self,
    //     byte3: Bv8,
    //     byte2: Bv8,
    //     byte1: Bv8,
    //     byte0: Bv8,
    // ) -> Bv32 {
    //     let def = Def {
    //         node: self.idx,
    //         line: self.ops.len(),
    //         retval: 0,
    //     };
    //     self.ops.push((
    //         IRRes::one(def),
    //         IROp::Int32(Int32Op::Concat(byte3, byte2, byte1, byte0)),
    //     ));
    //     Bv32::Var(def)
    // }

    pub fn all_b_in_i32(&mut self, bit: Bit) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::All(bit))));
        Bv32::Var(def)
    }

    pub fn b_to_i32(&mut self, val: Bit) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::FromBit(val))));
        Bv32::Var(def)
    }

    pub fn repl_b_in_i32(&mut self, bit: Bit, n: Bv32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::ReplBit(bit, n))));
        Bv32::Var(def)
    }

    pub fn concat_i16_to_i32(&mut self, hi: Bv16, lo: Bv16) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::Concat(hi, lo))));
        Bv32::Var(def)
    }

    pub fn split_i64_to_i32(&mut self, val: Bv64) -> (Bv32, Bv32) {
        let hi = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let lo = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops
            .push((IRRes::two(hi, lo), IROp::Int32(Int32Op::Split(val))));
        (Bv32::Var(hi), Bv32::Var(lo))
    }

    pub fn sext_i8_to_i32(&mut self, val: Bv8) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::SextBv8(val))));
        Bv32::Var(def)
    }

    pub fn sext_i16_to_i32(&mut self, val: Bv16) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::SextBv16(val))));
        Bv32::Var(def)
    }

    pub fn trunc_i64_to_i32(&mut self, val: Bv64) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::TruncBv64(val))));
        Bv32::Var(def)
    }

    pub fn zext_i8_to_i32(&mut self, val: Bv8) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::SextBv8(val))));
        Bv32::Var(def)
    }

    pub fn zext_i16_to_i32(&mut self, val: Bv16) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int32(Int32Op::ZextBv16(val))));
        Bv32::Var(def)
    }

    pub fn and_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::And(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn not_i64(&mut self, val: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Not(val))));
        Bv64::Var(def)
    }

    pub fn or_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Or(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn xor_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Xor(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn asr_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Asr(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn fshl_i64(&mut self, hi: Bv64, lo: Bv64, shift: u8) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Fshl(hi, lo, shift))));
        Bv64::Var(def)
    }

    pub fn fshr_i64(&mut self, hi: Bv64, lo: Bv64, shift: u8) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Fshr(hi, lo, shift))));
        Bv64::Var(def)
    }

    pub fn lsl_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Lsl(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn lsr_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Lsr(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn ror_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Ror(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn bit_rev_i64(&mut self, val: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::BitRev(val))));
        Bv64::Var(def)
    }

    pub fn byte_rev_i64(&mut self, val: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::ByteRev(val))));
        Bv64::Var(def)
    }

    pub fn ctls_i64(&mut self, val: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Ctls(val))));
        Bv64::Var(def)
    }

    pub fn ctlz_i64(&mut self, val: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Ctlz(val))));
        Bv64::Var(def)
    }

    pub fn ctset_i64(&mut self, val: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Ctset(val))));
        Bv64::Var(def)
    }

    pub fn cttz_i64(&mut self, val: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Cttz(val))));
        Bv64::Var(def)
    }

    pub fn abs_i64(&mut self, val: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Abs(val))));
        Bv64::Var(def)
    }

    pub fn sadd_i64(&mut self, lhs: Bv64, rhs: Bv64) -> (Bv64, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int64(Int64Op::SAdd(lhs, rhs)),
        ));
        (Bv64::Var(result), Bit::Var(overflow))
    }

    pub fn sdiv_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::SDiv(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn smul_i64(&mut self, lhs: Bv64, rhs: Bv64) -> (Bv64, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int64(Int64Op::SMul(lhs, rhs)),
        ));
        (Bv64::Var(result), Bit::Var(overflow))
    }

    pub fn srem_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::SRem(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn ssub_i64(&mut self, lhs: Bv64, rhs: Bv64) -> (Bv64, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int64(Int64Op::SSub(lhs, rhs)),
        ));
        (Bv64::Var(result), Bit::Var(overflow))
    }

    pub fn uadd_i64(&mut self, lhs: Bv64, rhs: Bv64) -> (Bv64, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int64(Int64Op::UAdd(lhs, rhs)),
        ));
        (Bv64::Var(result), Bit::Var(overflow))
    }

    pub fn udiv_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::UDiv(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn umul_i64(&mut self, lhs: Bv64, rhs: Bv64) -> (Bv64, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int64(Int64Op::UMul(lhs, rhs)),
        ));
        (Bv64::Var(result), Bit::Var(overflow))
    }

    pub fn urem_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::URem(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn usub_i64(&mut self, lhs: Bv64, rhs: Bv64) -> (Bv64, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int64(Int64Op::USub(lhs, rhs)),
        ));
        (Bv64::Var(result), Bit::Var(overflow))
    }

    pub fn smax_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::SMax(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn smin_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::SMin(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn umax_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::UMax(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn umin_i64(&mut self, lhs: Bv64, rhs: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::UMin(lhs, rhs))));
        Bv64::Var(def)
    }

    pub fn select_i64(&mut self, cond: Bit, t_case: Bv64, f_case: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Int64(Int64Op::Select(cond, t_case, f_case)),
        ));
        Bv64::Var(def)
    }

    // Concat i8 to i64 blows up the size of the Int64Op enum and all enums that use it
    // Only supporting half-size concats for now

    // pub fn concat_i8_to_i64(
    //     &mut self,
    //     byte7: Bv8,
    //     byte6: Bv8,
    //     byte5: Bv8,
    //     byte4: Bv8,
    //     byte3: Bv8,
    //     byte2: Bv8,
    //     byte1: Bv8,
    //     byte0: Bv8,
    // ) -> Bv64 {
    //     let def = Def {
    //         node: self.idx,
    //         line: self.ops.len(),
    //         retval: 0,
    //     };
    //     self.ops.push((
    //         IRRes::one(def),
    //         IROp::Int64(Int64Op::Concat(
    //             byte7, byte6, byte5, byte4, byte3, byte2, byte1, byte0,
    //         )),
    //     ));
    //     Bv64::Var(def)
    // }

    // pub fn concat_i16_to_i64(&mut self, hw3: Bv16, hw2: Bv16, hw1: Bv16, hw0: Bv16) -> Bv64 {
    //     let def = Def {
    //         node: self.idx,
    //         line: self.ops.len(),
    //         retval: 0,
    //     };
    //     self.ops.push((
    //         IRRes::one(def),
    //         IROp::Int64(Int64Op::Concat(hw3, hw2, hw1, hw0)),
    //     ));
    //     Bv64::Var(def)
    // }

    pub fn concat_i32_to_i64(&mut self, hi: Bv32, lo: Bv32) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::Concat(hi, lo))));
        Bv64::Var(def)
    }

    pub fn all_b_in_i64(&mut self, bit: Bit) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::All(bit))));
        Bv64::Var(def)
    }

    pub fn b_to_i64(&mut self, val: Bit) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::FromBit(val))));
        Bv64::Var(def)
    }

    pub fn split_i128_to_i64(&mut self, val: Bv128) -> (Bv64, Bv64) {
        let hi = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let lo = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops
            .push((IRRes::two(hi, lo), IROp::Int64(Int64Op::Split(val))));
        (Bv64::Var(hi), Bv64::Var(lo))
    }

    pub fn repl_b_in_i64(&mut self, bit: Bit, n: Bv64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::ReplBit(bit, n))));
        Bv64::Var(def)
    }

    pub fn sext_i8_to_i64(&mut self, val: Bv8) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::SextBv8(val))));
        Bv64::Var(def)
    }

    pub fn sext_i16_to_i64(&mut self, val: Bv16) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::SextBv16(val))));
        Bv64::Var(def)
    }

    pub fn sext_i32_to_i64(&mut self, val: Bv32) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::SextBv32(val))));
        Bv64::Var(def)
    }

    pub fn trunc_i128_to_i64(&mut self, val: Bv128) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::TruncBv128(val))));
        Bv64::Var(def)
    }

    pub fn zext_i8_to_i64(&mut self, val: Bv8) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::ZextBv8(val))));
        Bv64::Var(def)
    }

    pub fn zext_i16_to_i64(&mut self, val: Bv16) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::ZextBv16(val))));
        Bv64::Var(def)
    }

    pub fn zext_i32_to_i64(&mut self, val: Bv32) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int64(Int64Op::ZextBv32(val))));
        Bv64::Var(def)
    }

    // TODO: Int128 for real
    pub fn lsr_i128(&mut self, lhs: Bv128, rhs: Bv128) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::Lsr(lhs, rhs))));
        Bv128::Var(def)
    }

    pub fn smul_i128(&mut self, lhs: Bv128, rhs: Bv128) -> (Bv128, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int128(Int128Op::SMul(lhs, rhs)),
        ));
        (Bv128::Var(result), Bit::Var(overflow))
    }

    pub fn umul_i128(&mut self, lhs: Bv128, rhs: Bv128) -> (Bv128, Bit) {
        let result = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        let overflow = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 1,
        };
        self.ops.push((
            IRRes::two(result, overflow),
            IROp::Int128(Int128Op::UMul(lhs, rhs)),
        ));
        (Bv128::Var(result), Bit::Var(overflow))
    }

    pub fn sext_i8_to_i128(&mut self, val: Bv8) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::SextBv8(val))));
        Bv128::Var(def)
    }

    pub fn sext_i16_to_i128(&mut self, val: Bv16) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::SextBv16(val))));
        Bv128::Var(def)
    }

    pub fn sext_i32_to_i128(&mut self, val: Bv32) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::SextBv32(val))));
        Bv128::Var(def)
    }

    pub fn sext_i64_to_i128(&mut self, val: Bv64) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::SextBv64(val))));
        Bv128::Var(def)
    }

    pub fn zext_i8_to_i128(&mut self, val: Bv8) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::ZextBv8(val))));
        Bv128::Var(def)
    }

    pub fn zext_i16_to_i128(&mut self, val: Bv16) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::ZextBv16(val))));
        Bv128::Var(def)
    }

    pub fn zext_i32_to_i128(&mut self, val: Bv32) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::ZextBv32(val))));
        Bv128::Var(def)
    }

    pub fn zext_i64_to_i128(&mut self, val: Bv64) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::ZextBv64(val))));
        Bv128::Var(def)
    }

    pub fn select_i128(&mut self, cond: Bit, t_case: Bv128, f_case: Bv128) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Int128(Int128Op::Select(cond, t_case, f_case)),
        ));
        Bv128::Var(def)
    }

    pub fn extract_bv16_from_elem0_bv128(&mut self, val: Bv128) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::ExtBv16Elem0(val))));
        Bv16::Var(def)
    }

    pub fn extract_bv32_from_elem0_bv128(&mut self, val: Bv128) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::ExtBv32Elem0(val))));
        Bv32::Var(def)
    }

    pub fn extract_bv64_from_elem0_bv128(&mut self, val: Bv128) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::Int128(Int128Op::ExtBv64Elem0(val))));
        Bv64::Var(def)
    }

    pub fn insert_bv16_in_elem0_bv128(&mut self, base: Bv128, elem: Bv16) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Int128(Int128Op::InsBv16Elem0(base, elem)),
        ));
        Bv128::Var(def)
    }

    pub fn insert_bv32_in_elem0_bv128(&mut self, base: Bv128, elem: Bv32) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Int128(Int128Op::InsBv32Elem0(base, elem)),
        ));
        Bv128::Var(def)
    }

    pub fn insert_bv64_in_elem0_bv128(&mut self, base: Bv128, elem: Bv64) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops.push((
            IRRes::one(def),
            IROp::Int128(Int128Op::InsBv64Elem0(base, elem)),
        ));
        Bv128::Var(def)
    }
}
