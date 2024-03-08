use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

#[derive(Debug, Clone)]
pub struct Adr {
    raw: u32,
    d: usize,
    imm: BitVec,
}

impl Decodable<a64::InsnSize> for Adr {
    const FIXEDBITS: a64::InsnSize = 0b00010000000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b10011111000000000000000000000000;
}

impl Instruction<Arm> for Adr {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let d = bits.get(0..5)?.load();
        let immhi = bits.get(5..24)?;
        let immlo = bits.get(29..31)?;

        // imm = SignExtend(immhi @ immlo, 64)
        let mut imm = BitVec::with_capacity(64);
        imm.extend_from_bitslice(immlo);
        imm.extend_from_bitslice(immhi);
        imm.resize(64, *immhi.last().unwrap());

        Some(Self { raw, d, imm })
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
        let base = code.assign(Expr::ReadProc(&proc.pc, 64));
        let sum = code.assign(Expr::Add(Term::Var(base), Term::Lit(self.imm.clone())));
        code.write_proc(&proc.r[self.d], Term::Var(sum));
    }
}

#[derive(Debug, Clone)]
pub struct Adrp {
    raw: u32,
    d: usize,
    imm: BitVec,
}

impl Decodable<a64::InsnSize> for Adrp {
    const FIXEDBITS: a64::InsnSize = 0b10010000000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b10011111000000000000000000000000;
}

impl Instruction<Arm> for Adrp {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
        let d = bits.get(0..5)?.load();
        let immhi = bits.get(5..24)?;
        let immlo = bits.get(29..31)?;

        // imm = SignExtend((immhi @ immlo) @ Zeros(12), 64)
        let mut imm = BitVec::with_capacity(64);
        imm.resize(12, false);
        imm.extend_from_bitslice(immlo);
        imm.extend_from_bitslice(immhi);
        imm.resize(64, *immhi.last().unwrap());

        Some(Self { raw, d, imm })
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
        // bits(64) base = PC[];
        let base = code.assign(Expr::ReadProc(&proc.pc, 64));
        // base<11:0> = Zeros(12);
        let base = code.assign(Expr::And(
            Term::Var(base),
            Term::Lit(BitVec::from_element(0xfffffffffffff000)),
        ));
        // X[d, 64] = base + imm;
        let sum = code.assign(Expr::Add(Term::Var(base), Term::Lit(self.imm.clone())));
        code.write_proc(&proc.r[self.d], Term::Var(sum));
    }
}
