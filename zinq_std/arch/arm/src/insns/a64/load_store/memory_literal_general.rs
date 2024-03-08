use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct MemoryLiteralGeneral {
    raw: u32,
    memop: MemOp,
    nontemporal: bool,
    offset: BitVec,
    is_signed: bool,
    size: usize,
    t: usize,
    tagchecked: bool,
}

impl Decodable<a64::InsnSize> for MemoryLiteralGeneral {
    const FIXEDBITS: a64::InsnSize = 0b00011000000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b00111111000000000000000000000000;
}

impl Instruction<Arm> for MemoryLiteralGeneral {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let rt = bits.get(0..5)?;
        let imm19 = bits.get(5..24)?;
        let opc_0 = *bits.get(30)?;
        let opc_1 = *bits.get(31)?;
        todo!("memory_literal_general decode")
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

    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrBlock<'p>) {
        todo!("memory_literal_general semantics")
    }
}
