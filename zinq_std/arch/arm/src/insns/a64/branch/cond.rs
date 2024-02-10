use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{insns::a64, Arm};

#[derive(Debug, Clone)]
pub struct Cond {
    raw: u32,
    imm19: BitArr!(for 19),
    consistent: bool,
    cond: a64::CondCode,
}

impl Decodable<a64::InsnSize> for Cond {
    const FIXEDBITS: a64::InsnSize = 0b0101_0100_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: a64::InsnSize = 0b1111_1111_0000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for Cond {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let mut imm19 = BitArray::ZERO;
        imm19 |= bits.get(5..24)?;

        Some(Self {
            raw: bits.get(0..32)?.load(),
            imm19,
            consistent: *bits.get(4)?,
            cond: a64::CondCode::from(bits.get(0..4)?.load::<u8>()),
        })
    }

    fn name(&self) -> String {
        let cond = a64::cond_symbol(self.cond);
        if self.consistent {
            format!("BC.{cond}")
        } else {
            format!("B.{cond}")
        }
    }

    fn assemble(&self) -> &Self::InsnSize {
        &self.raw
    }

    fn disassemble(&self) -> String {
        let cond = a64::cond_symbol(self.cond);
        let imm = self.imm19.load::<isize>() << 2;

        if self.consistent {
            format!("BC.{cond} #{imm:X}")
        } else {
            format!("B.{cond} #{imm:X}")
        }
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        let mut code = IrBlock::new();

        // Decode
        let mut offset = bitvec![0; 21];
        *offset.get_mut(2..).unwrap() |= self.imm19;
        let offset = offset;

        // if ConditionHolds(cond) then
        //      BranchTo(PC[] + offset, BranchType_DIR, TRUE);

        let cond_holds = a64::condition_holds(self.cond, proc, &mut code);

        let pc = code.assign(Expr::ReadProc(&proc.pc));
        let true_case = code.assign(Expr::Binary(
            BinOp::Add,
            Term::Var(pc),
            Term::Lit(offset),
            true,
        ));
        let false_case = code.assign(Expr::Binary(
            BinOp::Add,
            Term::Var(pc),
            Term::Lit(BitVec::from_element(4)),
            false,
        ));

        code.if_else(
            Term::Var(cond_holds),
            Term::Var(true_case),
            Term::Var(false_case),
        );

        code
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zinq::{
        system::{Processor, System},
        Emulator,
    };
    use zinq_std_emu::StepEmu;

    fn run_test(test_case: &[u8], start_addr: usize, x0: u64) -> System<Arm> {
        let mut vm = System::new(Arm::v8(), 64);
        // CMP X0, #1
        vm.write_mem(start_addr, &[0x1F, 0x04, 0x00, 0xF1]).unwrap();
        vm.write_mem(start_addr + 4, &test_case).unwrap();

        let proc = vm.proc_mut();
        proc.set_x(0, x0);
        proc.set_ip(start_addr);

        let mut emu = StepEmu::new();
        emu.max_insns(2);
        emu.run(&mut vm);
        vm
    }

    #[test]
    fn eq() {
        // B.EQ #8
        let test_case = [0x40, 0x00, 0x00, 0x54];
        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 8);
    }

    #[test]
    fn ne() {
        // B.NE #8
        let test_case = [0x41, 0x00, 0x00, 0x54];
        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 8);
    }

    #[test]
    fn lt() {
        // B.LT #8
        let test_case = [0x4B, 0x00, 0x00, 0x54];
        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 8);
    }

    #[test]
    fn le() {
        // B.LE #8
        let test_case = [0x4D, 0x00, 0x00, 0x54];
        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 2);
        assert_eq!(vm.proc().ip(), 8);
    }

    #[test]
    fn gt() {
        // B.GT #8
        let test_case = [0x4C, 0x00, 0x00, 0x54];
        let vm = run_test(&test_case, 0, 2);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 8);
    }

    #[test]
    fn ge() {
        // B.GE #8
        let test_case = [0x4A, 0x00, 0x00, 0x54];
        let vm = run_test(&test_case, 0, 2);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 8);
    }
}
