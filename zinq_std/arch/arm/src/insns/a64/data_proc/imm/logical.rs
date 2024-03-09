use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct IntegerLogicalImmediate {
    raw: u32,
    d: usize,
    datasize: usize,
    imm: BitVec,
    n: usize,
    op: LogicalOp,
    setflags: bool,
}

impl Decodable<a64::InsnSize> for IntegerLogicalImmediate {
    const FIXEDBITS: a64::InsnSize = 0b00010010000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b00011111100000000000000000000000;
}

impl Instruction<Arm> for IntegerLogicalImmediate {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let rd = bits.get(0..5)?;
        let rn = bits.get(5..10)?;
        let imms = bits.get(10..16)?;
        let immr = bits.get(16..22)?;
        let n = *bits.get(22)?;
        let opc_0 = *bits.get(29)?;
        let opc_1 = *bits.get(30)?;
        let sf = *bits.get(31)?;

        if sf == false && n != false {
            return None;
        }

        let d = rd.load();
        let n = rn.load();
        let datasize = if sf { 64 } else { 32 };
        let (op, setflags) = match (opc_1, opc_0) {
            (false, false) => (LogicalOp::AND, false),
            (false, true) => (LogicalOp::ORR, false),
            (true, false) => (LogicalOp::EOR, false),
            (true, true) => (LogicalOp::AND, true),
        };

        Some(Self {
            raw,
            d,
            datasize,
            imm: bitvec!(0; 64),
            n,
            op,
            setflags,
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
        todo!("****THIS HAS A FAKE DECODE!!!!******integer_logical_immediate semantics")
    }
}
