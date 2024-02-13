use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{
    insns::{a64, disas, helpers::*},
    Arm,
};

#[derive(Debug, Clone)]
pub struct CondTest {
    raw: u32,
    b5: bool,
    op: bool,
    b40: u8,
    imm14: BitArr!(for 14),
    rt: usize,
}

impl Decodable<a64::InsnSize> for CondTest {
    const FIXEDBITS: a64::InsnSize = 0b0011_0110_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: a64::InsnSize = 0b0111_1110_0000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for CondTest {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let mut imm14 = BitArray::ZERO;
        imm14 |= bits.get(5..19)?;

        Some(Self {
            raw: bits.get(0..32)?.load(),
            b5: *bits.get(31)?,
            op: *bits.get(24)?,
            b40: bits.get(19..24)?.load(),
            imm14,
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
        let datasize = if self.b5 { 64 } else { 32 };
        let bit_pos = ((self.b5 as u8) << 5 | self.b40) as usize;
        let mut offset = bitvec![0; 16];
        *offset.get_mut(2..).unwrap() |= self.imm14;
        let offset = offset;

        let mut code: IrBlock<'_> = IrBlock::new();

        // let operand : bits('datasize) = X_read(t, datasize);
        let operand = x_read(self.rt, datasize, proc, &mut code);

        // if [operand[bit_pos]] == bit_val then {
        //      BranchTo(PC_read() + offset, BranchType_DIR, branch_conditional)
        // } else {
        //      BranchNotTaken(BranchType_DIR, branch_conditional)
        // }
        let op_at_bit_pos = code.assign(Expr::Bit {
            val: Term::Var(operand),
            idx: bit_pos,
        });
        let cond = if self.op {
            code.assign(Expr::Unary(UnaOp::IsNonZero, Term::Var(op_at_bit_pos)))
        } else {
            code.assign(Expr::Unary(UnaOp::IsZero, Term::Var(op_at_bit_pos)))
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
    fn tbz() {
        // TBZ X0, #0, 8
        let test_case = [0x40, 0x00, 0x00, 0x36];
        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 8);

        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 4);
    }

    #[test]
    fn tbnz() {
        // TBNZ X0, #0, 8
        let test_case = [0x40, 0x00, 0x00, 0x37];
        let vm = run_test(&test_case, 0, 1);
        assert_eq!(vm.proc().ip(), 8);

        let vm = run_test(&test_case, 0, 0);
        assert_eq!(vm.proc().ip(), 4);
    }
}
