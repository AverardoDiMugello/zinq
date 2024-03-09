use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct IntegerArithmeticAddSubShiftedreg {
    raw: u32,
    d: usize,
    datasize: usize,
    m: usize,
    n: usize,
    setflags: bool,
    shift_amount: usize,
    shift_type: ShiftType,
    sub_op: bool,
}

impl Decodable<a64::InsnSize> for IntegerArithmeticAddSubShiftedreg {
    const FIXEDBITS: a64::InsnSize = 0b00001011000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b00011111001000000000000000000000;
}

impl Instruction<Arm> for IntegerArithmeticAddSubShiftedreg {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let rd = bits.get(0..5)?;
        let rn = bits.get(5..10)?;
        let imm6 = bits.get(10..16)?;
        let rm = bits.get(16..21)?;
        let shift_0 = *bits.get(22)?;
        let shift_1 = *bits.get(23)?;
        let s = *bits.get(29)?;
        let op = *bits.get(30)?;
        let sf = *bits.get(31)?;

        if (shift_1, shift_0) == (true, true) {
            return None;
        }

        if sf == false && imm6[5] == true {
            return None;
        }

        Some(Self {
            raw,
            d: rd.load(),
            datasize: if sf { 64 } else { 32 },
            m: rm.load(),
            n: rn.load(),
            setflags: s,
            shift_amount: imm6.load(),
            shift_type: ShiftType::from((shift_1, shift_0)),
            sub_op: op,
        })
    }

    fn assemble(&self) -> a64::InsnSize {
        self.raw
    }

    fn disassemble(&self, proc: &Arm) -> String {
        disas::a64(self.raw, proc)
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
        todo!("integer_arithmetic_add_sub_shiftedreg semantics")
    }
}
