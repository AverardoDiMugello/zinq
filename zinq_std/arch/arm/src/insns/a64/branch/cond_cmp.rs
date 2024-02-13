use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{
    insns::{a64, disas, helpers::*},
    Arm,
};

#[derive(Debug, Clone)]
pub struct CondCmp {
    raw: u32,
    sf: bool,
    op: bool,
    imm19: BitArr!(for 19),
    rt: usize,
}

impl Decodable<a64::InsnSize> for CondCmp {
    const FIXEDBITS: a64::InsnSize = 0b0011_0100_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: a64::InsnSize = 0b0111_1110_0000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for CondCmp {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let mut imm19 = BitArray::ZERO;
        imm19 |= bits.get(5..24)?;

        Some(Self {
            raw: bits.get(0..32)?.load(),
            sf: *bits.get(31)?,
            op: *bits.get(24)?,
            imm19,
            rt: bits.get(0..5)?.load(),
        })
    }

    fn assemble(&self) -> &Self::InsnSize {
        &self.raw
    }

    fn disassemble(&self, proc: &Arm) -> String {
        disas::a64(self.raw, proc)
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        // Decode
        let datasize = if self.sf { 64 } else { 32 };
        let mut offset = bitvec![0; 21];
        *offset.get_mut(2..).unwrap() |= self.imm19;
        let offset = offset;

        let mut code: IrBlock<'_> = IrBlock::new();

        // let operand1 : bits('datasize) = X_read(t, datasize);
        let operand1 = x_read(self.rt, datasize, proc, &mut code);

        // if IsZero(operand1) == iszero then {
        //      BranchTo(PC_read() + offset, BranchType_DIR, branch_conditional)
        // } else {
        //      BranchNotTaken(BranchType_DIR, branch_conditional)
        // }
        let cond = if self.op {
            code.assign(Expr::Unary(UnaOp::IsNonZero, Term::Var(operand1)))
        } else {
            code.assign(Expr::Unary(UnaOp::IsZero, Term::Var(operand1)))
        };

        let (true_case, false_case) = pc_offset_and_next(Term::Lit(offset), proc, &mut code);

        code.if_else(Term::Var(cond), Term::Var(true_case), Term::Var(false_case));

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

    fn run_test(test_case: &[u8], start_addr: usize, x0: u64) -> System<Arm> {
        let mut vm = System::new(Arm::new(Armv9p4a), 64);
        vm.write_mem(start_addr, &test_case).unwrap();

        let proc = vm.proc_mut();
        proc.set_x(0, x0);
        proc.set_ip(start_addr);

        let mut emu = StepEmu::new();
        emu.max_insns(1);
        emu.run(&mut vm);
        vm
    }

    #[test]
    fn cbz_64() {
        // CBZ X0, #12
        let test_case = [0x60, 0x00, 0x00, 0xB4];
        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 4);
    }

    #[test]
    fn cbz_32() {
        // CBZ W0, #12
        let test_case = [0x60, 0x00, 0x00, 0x34];
        let vm = run_test(&test_case, 0, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 0xaaaaaaaa00000001);
        assert_eq!(vm.proc().ip(), 4);
    }

    #[test]
    fn cbnz_64() {
        // CBNZ X0, #12
        let test_case = [0x60, 0x00, 0x00, 0xB5];
        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 4);
    }

    #[test]
    fn cbnz_32() {
        // CBNZ W0, #12
        let test_case = [0x60, 0x00, 0x00, 0x35];
        let vm = run_test(&test_case, 0, 0xaaaaaaaa00000001);
        assert_eq!(vm.proc().ip(), 12);

        let vm = run_test(&test_case, 0, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().ip(), 4);
    }
}
