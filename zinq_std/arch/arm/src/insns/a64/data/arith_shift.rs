use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{
    insns::{a64, helpers::*},
    Arm,
};

#[derive(Clone, Debug)]
pub struct ArithShift {
    raw: u32,
    sf: bool,
    op: bool,
    s: bool,
    shift: ShiftType,
    rm: usize,
    imm6: BitArr!(for 6),
    rn: usize,
    rd: usize,
}

impl Decodable<a64::InsnSize> for ArithShift {
    const FIXEDBITS: a64::InsnSize = 0b0000_1011_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: a64::InsnSize = 0b0001_1111_0010_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for ArithShift {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let mut imm6 = BitArray::ZERO;
        imm6 |= bits.get(10..16)?;

        let insn = Self {
            raw: bits.get(0..32)?.load(),
            sf: *bits.get(31)?,
            op: *bits.get(30)?,
            s: *bits.get(29)?,
            shift: ShiftType::from((*bits.get(23)?, *bits.get(22)?)),
            rm: bits.get(16..21)?.load(),
            imm6,
            rn: bits.get(5..10)?.load(),
            rd: bits.get(0..5)?.load(),
        };

        if let ShiftType::Ror = insn.shift {
            return None;
        }

        if !insn.sf && *insn.imm6.last()? {
            return None;
        }

        Some(insn)
    }

    fn name(&self) -> String {
        // Subtract?
        if self.op {
            // Setflags?
            if self.s {
                if self.rn == 0b11111 && self.rd != 0b11111 {
                    String::from("NEGS")
                } else if self.rd == 0b11111 {
                    String::from("CMP (shifted register)")
                } else {
                    String::from("SUBS (shifted register)")
                }
            } else {
                if self.rn == 0b11111 {
                    String::from("NEG (shifted register)")
                } else {
                    String::from("SUB (shifted register)")
                }
            }
        } else {
            if self.s {
                if self.rd == 0b11111 {
                    String::from("CMN (shifted register)")
                } else {
                    String::from("ADDS (shifted register)")
                }
            } else {
                String::from("ADD (shifted register)")
            }
        }
    }

    fn assemble(&self) -> &a64::InsnSize {
        &self.raw
    }

    fn disassemble(&self, _proc: &Arm) -> String {
        let rd = reg_symbol(self.sf, self.rd);
        let rm = reg_symbol(self.sf, self.rm);
        let rn = reg_symbol(self.sf, self.rn);
        let imm = self.imm6.load::<usize>();
        let shift = if imm > 0 {
            format!(", {0} #{imm}", self.shift)
        } else {
            String::new()
        };

        // Subtract?
        if self.op {
            // Setflags?
            if self.s {
                if self.rn == 0b11111 && self.rd != 0b11111 {
                    format!("NEGS")
                } else if self.rd == 0b11111 {
                    format!("CMP {rd}, {rn}, {rm}{shift}")
                } else {
                    format!("SUBS {rd}, {rn}, {rm}{shift}")
                }
            } else {
                if self.rn == 0b11111 {
                    format!("NEG {rd}, {rn}, {rm}{shift}")
                } else {
                    format!("SUB {rd}, {rn}, {rm}{shift}")
                }
            }
        } else {
            if self.s {
                if self.rd == 0b11111 {
                    format!("CMN {rd}, {rn}, {rm}{shift}")
                } else {
                    format!("ADDS {rd}, {rn}, {rm}{shift}")
                }
            } else {
                format!("ADD {rd}, {rn}, {rm}{shift}")
            }
        }
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        // Decode
        let datasize = if self.sf { 64 } else { 32 };
        let shift_amt = self.imm6.load::<usize>();

        let mut code: IrBlock<'_> = IrBlock::new();

        // let operand1 : bits('datasize) = X_read(n, datasize);
        let operand1 = x_read(self.rn, datasize, proc, &mut code);
        // operand2 : bits('datasize) = ShiftReg(m, shift_type, shift_amount, datasize);
        let operand2 = shift_reg(self.rm, self.shift, shift_amt, datasize, proc, &mut code);

        // if sub_op then {
        //     operand2 = not_vec(operand2);
        //     carry_in = 0b1
        // } else {
        //     carry_in = 0b0
        // };
        let (operand2, carry_in) = if self.op {
            (
                code.assign(Expr::Unary(UnaOp::Not, Term::Var(operand2))),
                bitvec!(1),
            )
        } else {
            (operand2, bitvec!(0))
        };

        // (result, nzcv) = AddWithCarry(operand1, operand2, carry_in);
        let (result, (n, z, c, v)) = add_with_carry(
            Term::Var(operand1),
            Term::Var(operand2),
            Term::Lit(carry_in),
            datasize,
            &mut code,
        );

        // if setflags then {
        //     (PSTATE.N @ PSTATE.Z @ PSTATE.C @ PSTATE.V) = nzcv
        // };
        if self.s {
            code.write_proc(&proc.pstate.n, 0, Term::Var(n));
            code.write_proc(&proc.pstate.z, 0, Term::Var(z));
            code.write_proc(&proc.pstate.c, 0, Term::Var(c));
            code.write_proc(&proc.pstate.v, 0, Term::Var(v));
        }

        // X_set(d, datasize) = result
        x_set(self.rd, Term::Var(result), proc, &mut code);

        // Increment PC
        inc_pc(proc, &mut code);

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

    fn run_test(test_case: &[u8], x0: u64, x1: u64) -> System<Arm> {
        let mut vm = System::new(Arm::new(Armv9p4a), 12);
        vm.write_mem(0, test_case).unwrap();

        let proc = vm.proc_mut();
        proc.set_ip(0);
        proc.set_x(0, x0);
        proc.set_x(1, x1);

        let mut emu = StepEmu::new();
        emu.max_insns(1);
        emu.run(&mut vm);
        vm
    }

    #[test]
    fn adds_lsl_no_overflow_64() {
        // ADD X1, X1, X0, LSL 1
        let test_case = [0x21, 0x04, 0x00, 0xAB];
        let vm = run_test(&test_case, 1, 1);
        assert_eq!(vm.proc().x(1), Some(3));
    }

    #[test]
    fn adds_lsl_overflow_32() {
        // ADDS W1, W1, W0, LSL 1
        let test_case = [0x21, 0x04, 0x00, 0x2B];
        let vm = run_test(&test_case, 0xffffffff80000000, 0xffffffff00000001);
        assert_eq!(vm.proc().x(1), Some(0xffffffff00000001));
    }
}
