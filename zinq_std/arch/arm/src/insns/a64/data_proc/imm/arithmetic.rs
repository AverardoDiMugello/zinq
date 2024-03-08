use crate::insns::helpers::*;
use crate::insns::{a64, disas};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq::insn::syntax::Decodable;
use zinq::insn::Instruction;

fn decode(bits: &BitSlice) -> Option<(u32, BitVec, usize, usize, usize)> {
    let raw = bits.get(0..32)?.load::<a64::InsnSize>();
    let sf = *bits.get(31)?;
    let sh = *bits.get(22)?;
    let imm12 = bits.get(10..22)?;
    let n = bits.get(5..10)?.load();
    let d = bits.get(0..5)?.load();

    let datasize = if sf { 64 } else { 32 };
    let mut imm = BitVec::with_capacity(datasize);
    match sh {
        false => {
            // imm = ZeroExtend(imm12, datasize)
            imm.extend_from_bitslice(imm12);
            imm.resize(datasize, false);
        }
        true => {
            // imm = ZeroExtend(imm12 @ Zeros(12), datasize)
            imm.resize(12, false);
            imm.extend_from_bitslice(imm12);
            imm.resize(datasize, false);
        }
    }

    Some((raw, imm, datasize, n, d))
}

#[derive(Debug, Clone)]
pub struct Add {
    raw: u32,
    datasize: usize,
    d: usize,
    imm: BitVec,
    n: usize,
}

impl Decodable<a64::InsnSize> for Add {
    const FIXEDBITS: a64::InsnSize = 0b00010001000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b01111111100000000000000000000000;
}

impl Instruction<Arm> for Add {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let (raw, imm, datasize, n, d) = decode(bits)?;
        Some(Self {
            raw,
            d,
            datasize,
            imm,
            n,
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
        let operand1 = if self.n == 31 {
            sp_read(self.datasize, proc, code)
        } else {
            x_read(self.n, self.datasize, proc, code)
        };

        let (result, _) = add_with_carry(
            &Term::Var(operand1),
            &Term::Lit(self.imm.clone()),
            &Term::Lit(bitvec!(0)),
            self.datasize,
            code,
        );

        if self.d == 31 {
            let zext_result = code.assign(Expr::Zext {
                val: Term::Var(result),
                size: Term::Lit(BitVec::from_element(64)),
            });
            sp_set(Term::Var(zext_result), proc, code);
        } else {
            x_set(self.d, self.datasize, Term::Var(result), proc, code);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Adds {
    raw: u32,
    datasize: usize,
    d: usize,
    imm: BitVec,
    n: usize,
}

impl Decodable<a64::InsnSize> for Adds {
    const FIXEDBITS: a64::InsnSize = 0b00110001000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b01111111100000000000000000000000;
}

impl Instruction<Arm> for Adds {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let (raw, imm, datasize, n, d) = decode(bits)?;
        Some(Self {
            raw,
            d,
            datasize,
            imm,
            n,
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
        let operand1 = if self.n == 31 {
            sp_read(self.datasize, proc, code)
        } else {
            x_read(self.n, self.datasize, proc, code)
        };

        let (result, (n, z, c, v)) = add_with_carry(
            &Term::Var(operand1),
            &Term::Lit(self.imm.clone()),
            &Term::Lit(bitvec!(0)),
            self.datasize,
            code,
        );

        code.write_proc(&proc.pstate.n, Term::Var(n));
        code.write_proc(&proc.pstate.z, Term::Var(z));
        code.write_proc(&proc.pstate.c, Term::Var(c));
        code.write_proc(&proc.pstate.v, Term::Var(v));

        x_set(self.d, self.datasize, Term::Var(result), proc, code);
    }
}

#[derive(Debug, Clone)]
pub struct Sub {
    raw: u32,
    datasize: usize,
    d: usize,
    imm: BitVec,
    n: usize,
}

impl Decodable<a64::InsnSize> for Sub {
    const FIXEDBITS: a64::InsnSize = 0b01010001000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b01111111100000000000000000000000;
}

impl Instruction<Arm> for Sub {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let (raw, imm, datasize, n, d) = decode(bits)?;
        Some(Self {
            raw,
            d,
            datasize,
            imm,
            n,
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
        let operand1 = if self.n == 31 {
            sp_read(self.datasize, proc, code)
        } else {
            x_read(self.n, self.datasize, proc, code)
        };

        let operand2 = code.assign(Expr::Not(Term::Lit(self.imm.clone())));
        let (result, _) = add_with_carry(
            &Term::Var(operand1),
            &Term::Var(operand2),
            &Term::Lit(bitvec!(1)),
            self.datasize,
            code,
        );

        if self.d == 31 {
            let zext_result = code.assign(Expr::Zext {
                val: Term::Var(result),
                size: Term::Lit(BitVec::from_element(64)),
            });
            sp_set(Term::Var(zext_result), proc, code);
        } else {
            x_set(self.d, self.datasize, Term::Var(result), proc, code);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Subs {
    raw: u32,
    datasize: usize,
    d: usize,
    imm: BitVec,
    n: usize,
}

impl Decodable<a64::InsnSize> for Subs {
    const FIXEDBITS: a64::InsnSize = 0b01110001000000000000000000000000;
    const FIXEDMASK: a64::InsnSize = 0b01111111100000000000000000000000;
}

impl Instruction<Arm> for Subs {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let (raw, imm, datasize, n, d) = decode(bits)?;
        Some(Self {
            raw,
            d,
            datasize,
            imm,
            n,
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
        let operand1 = if self.n == 31 {
            sp_read(self.datasize, proc, code)
        } else {
            x_read(self.n, self.datasize, proc, code)
        };

        let operand2 = code.assign(Expr::Not(Term::Lit(self.imm.clone())));
        let (result, (n, z, c, v)) = add_with_carry(
            &Term::Var(operand1),
            &Term::Var(operand2),
            &Term::Lit(bitvec!(1)),
            self.datasize,
            code,
        );

        code.write_proc(&proc.pstate.n, Term::Var(n));
        code.write_proc(&proc.pstate.z, Term::Var(z));
        code.write_proc(&proc.pstate.c, Term::Var(c));
        code.write_proc(&proc.pstate.v, Term::Var(v));

        x_set(self.d, self.datasize, Term::Var(result), proc, code);
    }
}
