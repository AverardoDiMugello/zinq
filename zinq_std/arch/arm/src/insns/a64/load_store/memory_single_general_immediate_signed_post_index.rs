use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct MemorySingleGeneralImmediateSignedPostIndex {
    raw: u32,
    datasize: usize,
    memop: MemOp,
    n: usize,
    nontemporal: bool,
    offset: BitVec,
    postindex: bool,
    regsize: usize,
    rt_unknown: bool,
    is_signed: bool,
    t: usize,
    tagchecked: bool,
    wb_unknown: bool,
    wback: bool,
}

impl Decodable<a64::InsnSize> for MemorySingleGeneralImmediateSignedPostIndex {
    const FIXEDBITS: a64::InsnSize = 0b00111000000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b00111110000000000000000000000000;
}

impl Instruction<Arm> for MemorySingleGeneralImmediateSignedPostIndex {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        todo!("memory_single_general_immediate_signed_post_idx decode; was a handwrite due to multipl slice options")
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
        todo!("memory_single_general_immediate_signed_post_idx semantics")
    }
}
