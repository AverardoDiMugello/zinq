use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct BranchConditionalCond {
    raw: u32,
    condition: CondCode,
    offset: BitVec,
}

impl Decodable<a64::InsnSize> for BranchConditionalCond {
    const FIXEDBITS: a64::InsnSize = 0b01010100000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b11111111000000000000000000010000;
}

impl Instruction<Arm> for BranchConditionalCond {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let cond = bits.get(0..4)?;
        let imm19 = bits.get(5..24)?;

        let mut offset = BitVec::with_capacity(64);
        offset.extend_from_bitslice(bits![0; 2]);
        offset.extend_from_bitslice(imm19);
        offset.resize(64, imm19[18]);

        Some(Self {
            raw,
            condition: CondCode::from(cond.load::<u8>()),
            offset,
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

    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrBlock<'p>) {
        todo!("branch_conditional_cond semantics")
    }
}
