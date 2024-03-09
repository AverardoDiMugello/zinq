use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct BranchUnconditionalRegister {
    raw: u32,
    branch_type: BranchType,
    m: usize,
    n: usize,
    pac: bool,
    source_is_sp: bool,
    use_key_a: bool,
}

impl Decodable<a64::InsnSize> for BranchUnconditionalRegister {
    const FIXEDBITS: a64::InsnSize = 0b11010110000111110000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b11111110100111111111000000000000;
}

impl Instruction<Arm> for BranchUnconditionalRegister {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let rm = bits.get(0..5)?;
        let rn = bits.get(5..10)?;
        let m = *bits.get(10)?;
        let a = *bits.get(11)?;
        let op_0 = *bits.get(21)?;
        let op_1 = *bits.get(22)?;
        let z = *bits.get(24)?;

        let rm = rm.load();
        let pac = a == true;
        let use_key_a = m == false;
        let source_is_sp = z == true && rm == 31;

        // if !pac && rm != 0 {
        //     return None;
        // } else if pac

        Some(Self {
            raw,
            branch_type: BranchType::DIR,
            m: rm,
            n: rn.load(),
            pac,
            source_is_sp,
            use_key_a,
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
        todo!("***FAKE DECODE***branch_unconditional_register semantics")
    }
}
