use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{insns::a64, Arm};

#[derive(Debug, Clone)]
pub struct UncondImm {
    raw: u32,
    op: bool,
    imm26: BitArr!(for 26),
}

impl Decodable<a64::InsnSize> for UncondImm {
    const FIXEDBITS: a64::InsnSize = 0b0001_0100_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: a64::InsnSize = 0b0111_1100_0000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for UncondImm {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let mut imm26 = BitArray::ZERO;
        imm26 |= bits.get(0..26)?;

        Some(Self {
            raw: bits.get(0..32)?.load(),
            op: *bits.get(31)?,
            imm26,
        })
    }

    fn name(&self) -> String {
        if self.op {
            String::from("BL")
        } else {
            String::from("B")
        }
    }

    fn assemble(&self) -> &Self::InsnSize {
        &self.raw
    }

    fn disassemble(&self, proc: &Arm) -> String {
        let label = proc.pc.load::<isize>() + (self.imm26.load::<isize>() << 2);
        if self.op {
            format!("BL #{label:X}")
        } else {
            format!("B #{label:X}")
        }
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        let mut code = IrBlock::new();

        // Decode
        let mut offset = bitvec![0; 28];
        *offset.get_mut(2..).unwrap() |= self.imm26;
        let offset = offset;

        let pc = code.assign(Expr::ReadProc(&proc.pc));

        if self.op {
            let ret_addr = code.assign(Expr::Binary(
                BinOp::Add,
                Term::Var(pc),
                Term::Lit(BitVec::from_element(4)),
                false,
            ));
            code.write_proc(&proc.r[30], 0, Term::Var(ret_addr));
        }

        let addr = code.assign(Expr::Binary(
            BinOp::Add,
            Term::Var(pc),
            Term::Lit(offset),
            true,
        ));

        let addr = code.assign(Expr::Slice {
            val: Term::Var(addr),
            start: 0,
            len: 64,
        });

        code.goto(Term::Var(addr));

        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Version::Armv9p4a;
    use zinq::{
        system::{Processor, System},
        Emulator,
    };
    use zinq_std_emu::StepEmu;

    fn run_test(test_case: &[u8], start_addr: usize) -> System<Arm> {
        let mut vm = System::new(Arm::new(Armv9p4a), 64);
        vm.write_mem(start_addr, test_case).unwrap();

        let proc = vm.proc_mut();
        proc.set_ip(start_addr);

        let mut emu = StepEmu::new();
        emu.max_insns(1);
        emu.run(&mut vm);
        vm
    }

    #[test]
    fn pos_offset() {
        // B #8
        let test_case = [0x02, 0x00, 0x00, 0x14];
        let vm = run_test(&test_case, 0);
        assert_eq!(vm.proc().ip(), 8);
    }

    #[test]
    fn neg_offset() {
        // B #-8
        let test_case = [0xFE, 0xFF, 0xFF, 0x17];
        let vm = run_test(&test_case, 8);
        assert_eq!(vm.proc().ip(), 0);
    }

    #[test]
    fn link_reg() {
        // BL #8
        let test_case = [0x02, 0x00, 0x00, 0x94];
        let vm = run_test(&test_case, 0);
        assert_eq!(vm.proc().x(30), Some(4));
    }
}
