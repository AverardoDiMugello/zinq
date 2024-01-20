use zinq::{
    insn::{semantics::IrBlock, syntax::Decodable, Instruction},
    Error, Result,
};

use crate::Arm;

pub mod a64;

#[derive(Debug, Clone)]
pub enum ArmInsn {
    ArithImm(a64::data::ArithImm),
    Branch(a64::branch::Branch),
}

impl Decodable<u32> for ArmInsn {
    const FIXEDBITS: u32 = 0;
    const FIXEDMASK: u32 = 0;
}

impl Instruction<Arm> for ArmInsn {
    type InsnSize = u32;

    fn decode(raw: &[u8]) -> Result<Self> {
        let raw_32 = u32::from_le_bytes(raw.get(0..4).unwrap().try_into().unwrap());

        if raw_32 & a64::data::ArithImm::FIXEDMASK == a64::data::ArithImm::FIXEDBITS {
            return a64::data::ArithImm::decode(&raw).and_then(|insn| Ok(ArmInsn::ArithImm(insn)));
        }

        if raw_32 & a64::branch::Branch::FIXEDMASK == a64::branch::Branch::FIXEDBITS {
            return a64::branch::Branch::decode(&raw).and_then(|insn| Ok(ArmInsn::Branch(insn)));
        }

        return Err(Error(format!("Undecodable instruction")));
    }

    fn name(&self) -> String {
        match self {
            Self::ArithImm(i) => i.name(),
            Self::Branch(i) => i.name(),
        }
    }

    fn assemble(&self) -> &Self::InsnSize {
        match self {
            Self::ArithImm(i) => i.assemble(),
            Self::Branch(i) => i.assemble(),
        }
    }

    fn disassemble(&self) -> String {
        match self {
            Self::ArithImm(i) => i.disassemble(),
            Self::Branch(i) => i.disassemble(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::ArithImm(i) => i.size(),
            Self::Branch(i) => i.size(),
        }
    }

    fn semanitcs<'p>(&self, ctx: &'p Arm) -> IrBlock<'p> {
        match self {
            Self::ArithImm(i) => i.semanitcs(ctx),
            Self::Branch(i) => i.semanitcs(ctx),
        }
    }
}
