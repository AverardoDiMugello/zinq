use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{
    insns::{a64, disas},
    Arm,
};

#[derive(Debug, Clone)]
pub struct UncondReg {
    raw: u32,
    z: bool,
    op_1: bool,
    op_0: bool,
    a: bool,
    m: bool,
    rn: usize,
    rm: usize,
}

impl Decodable<a64::InsnSize> for UncondReg {
    const FIXEDBITS: a64::InsnSize = 0;
    const FIXEDMASK: a64::InsnSize = 0;
}

impl Instruction<Arm> for UncondReg {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        todo!("Insn not supported")
    }

    fn assemble(&self) -> &Self::InsnSize {
        &self.raw
    }

    fn disassemble(&self, proc: &Arm) -> String {
        disas::a64(self.raw, proc)
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        todo!("Insn not supported")
    }
}
