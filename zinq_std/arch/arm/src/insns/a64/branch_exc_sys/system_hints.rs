use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct SystemHints {
    raw: u32,
}

impl Decodable<a64::InsnSize> for SystemHints {
    const FIXEDBITS: a64::InsnSize = 0b11010101000000110010000000011111;
    const FIXEDMASK: a64::InsnSize = 0b11111111111111111111000000011111;
}

impl Instruction<Arm> for SystemHints {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let op2 = bits.get(5..8)?;
        let crm = bits.get(8..12)?;
        todo!("system_hints decode")
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
        todo!("system_hints semantics")
    }
}
