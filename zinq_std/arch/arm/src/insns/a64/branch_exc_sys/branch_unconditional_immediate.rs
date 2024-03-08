use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct BranchUnconditionalImmediate {
    raw: u32,
    branch_type: BranchType,
    offset: BitVec,
}

impl Decodable<a64::InsnSize> for BranchUnconditionalImmediate {
    const FIXEDBITS: a64::InsnSize = 0b00010100000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b01111100000000000000000000000000;
}

impl Instruction<Arm> for BranchUnconditionalImmediate {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        // Extract
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let imm26 = bits.get(0..26)?;
        let op = *bits.get(31)?;

        // Decode
        let branch_type = if op {
            BranchType::DIRCALL
        } else {
            BranchType::DIR
        };

        // bits(64) offset = SignExtend(imm26:'00', 64);
        let mut offset = BitVec::with_capacity(64);
        offset.extend_from_bitslice(bits![0; 2]);
        offset.extend_from_bitslice(imm26);
        offset.resize(64, *imm26.last().unwrap());

        Some(Self {
            raw,
            branch_type,
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
        todo!("branch_unconditional_immediate semantics")
    }
}
