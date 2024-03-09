use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct MemoryPairGeneralPostIdx {
    raw: u32,
    datasize: usize,
    memop: MemOp,
    n: usize,
    nontemporal: bool,
    offset: BitVec,
    postindex: bool,
    rt_unknown: bool,
    is_signed: bool,
    t: usize,
    t2: usize,
    tagchecked: bool,
    wb_unknown: bool,
    wback: bool,
}

impl Decodable<a64::InsnSize> for MemoryPairGeneralPostIdx {
    const FIXEDBITS: a64::InsnSize = 0b00101000000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b00111110000000000000000000000000;
}

impl Instruction<Arm> for MemoryPairGeneralPostIdx {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let rt = bits.get(0..5)?;
        let rn = bits.get(5..10)?;
        let rt2 = bits.get(10..15)?;
        let imm7 = bits.get(15..22)?;
        let l = *bits.get(22)?;
        let opc_0 = *bits.get(30)?;
        let opc_1 = *bits.get(31)?;

        let memop = if l == true { MemOp::LOAD } else { MemOp::STORE };

        Some(Self {
            raw,
            datasize: 64,
            memop,
            n: rn.load(),
            nontemporal: false,
            offset: bitvec![64],
            postindex: false,
            rt_unknown: false,
            is_signed: false,
            t: rt.load(),
            t2: rt2.load(),
            tagchecked: true,
            wb_unknown: false,
            wback: true,
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
        todo!("****FAKE DECODE!!!****memory_pair_general_post_idx semantics")
    }
}
