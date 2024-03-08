use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct SystemRegisterSystem {
    raw: u32,
    read: bool,
    sys_crm: usize,
    sys_crn: usize,
    sys_op0: usize,
    sys_op1: usize,
    sys_op2: usize,
    t: usize,
}

impl Decodable<a64::InsnSize> for SystemRegisterSystem {
    const FIXEDBITS: a64::InsnSize = 0b11010101000100000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b11111111110100000000000000000000;
}

impl Instruction<Arm> for SystemRegisterSystem {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let rt = bits.get(0..5)?;
        let op2_0 = *bits.get(5)?;
        let op2_1 = *bits.get(6)?;
        let op2_2 = *bits.get(7)?;
        let crm = bits.get(8..12)?;
        let crn = bits.get(12..16)?;
        let op1_0 = *bits.get(16)?;
        let op1_1 = *bits.get(17)?;
        let op1_2 = *bits.get(18)?;
        let o0 = *bits.get(19)?;
        let l = *bits.get(21)?;

        Some(Self {
            raw,
            read: false,
            sys_crm: 0,
            sys_crn: 0,
            sys_op0: 0,
            sys_op1: 0,
            sys_op2: 0,
            t: 0,
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
        todo!("***FAKE!!!***system_register_system semantics")
    }
}
