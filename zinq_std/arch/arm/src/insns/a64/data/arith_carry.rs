use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{
    insns::{a64, helpers::*},
    Arm,
};

#[derive(Clone, Debug)]
pub struct ArithCarry {
    raw: u32,
    sf: bool,
    op: bool,
    s: bool,
    rm: usize,
    rn: usize,
    rd: usize,
}

impl Decodable<a64::InsnSize> for ArithCarry {
    const FIXEDBITS: a64::InsnSize = 0b0001_1010_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: a64::InsnSize = 0b0001_1111_1110_0000_1111_1100_0000_0000;
}

impl Instruction<Arm> for ArithCarry {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        Some(Self {
            raw: bits.get(0..32)?.load(),
            sf: *bits.get(31)?,
            op: *bits.get(30)?,
            s: *bits.get(29)?,
            rm: bits.get(16..21)?.load(),
            rn: bits.get(5..10)?.load(),
            rd: bits.get(0..5)?.load(),
        })
    }

    fn name(&self) -> String {
        // Subtract?
        if self.op {
            // Setflags?
            if self.s {
                if self.rn == 0b11111 {
                    String::from("NGCS")
                } else {
                    String::from("SBCS")
                }
            } else {
                if self.rn == 0b11111 {
                    String::from("NGC")
                } else {
                    String::from("SBC")
                }
            }
        } else {
            if self.s {
                String::from("ADCS")
            } else {
                String::from("ADC)")
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

        // Subtract?
        if self.op {
            // Set flags?
            if self.s {
                // CMP special case
                if self.rn == 0b11111 {
                    format!("NGCS {rd}, {rm}")
                } else {
                    format!("SBCS {rd}, {rn}, {rm}")
                }
            } else {
                if self.rn == 0b11111 {
                    format!("NGC {rd}, {rm}")
                } else {
                    format!("SBC {rd}, {rn}, {rm}")
                }
            }
        } else {
            if self.s {
                format!("ADCS {rd}, {rn}, {rm}")
            } else {
                format!("ADC {rd}, {rn}, {rm}")
            }
        }
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        // Decode
        let datasize = if self.sf { 64 } else { 32 };

        let mut code: IrBlock<'_> = IrBlock::new();

        // let operand1 : bits('datasize) = X_read(n, datasize);
        let operand1 = x_read(self.rn, datasize, proc, &mut code);
        // operand2 : bits('datasize) = X_read(m, datasize);
        let operand2 = x_read(self.rm, datasize, proc, &mut code);

        // if sub_op then {
        //      operand2 = not_vec(operand2)
        // };
        let operand2 = if self.op {
            code.assign(Expr::Unary(UnaOp::Not, Term::Var(operand2)))
        } else {
            operand2
        };

        // (result, nzcv) = AddWithCarry(operand1, operand2, PSTATE.C);
        let pstate_c = code.assign(Expr::ReadProc(&proc.pstate.c));
        let (result, (n, z, c, v)) = add_with_carry(
            Term::Var(operand1),
            Term::Var(operand2),
            Term::Var(pstate_c),
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

    fn run_test(test_case: &[u8], carry_set: bool) -> System<Arm> {
        let mut vm = System::new(Arm::new(Armv9p4a), 12);
        // ADDS X0, X0, #1
        let first_insn = [0x00, 0x04, 0x00, 0xB1];
        vm.write_mem(0, &first_insn).unwrap();
        vm.write_mem(4, test_case).unwrap();

        let proc = vm.proc_mut();
        proc.set_ip(0);
        if carry_set {
            proc.set_x(0, 0xffffffffffffffff);
        } else {
            proc.set_x(0, 0);
        }

        let mut emu = StepEmu::new();
        emu.max_insns(2);
        emu.run(&mut vm);
        vm
    }

    #[test]
    fn adc_64() {
        // ADC X0, X0, X0
        let test_case = [0x00, 0x00, 0x00, 0x9A];
        let vm = run_test(&test_case, false);
        assert_eq!(vm.proc().x(0), Some(2));

        let vm = run_test(&test_case, true);
        assert_eq!(vm.proc().x(0), Some(1));
    }

    #[test]
    fn sbc_64() {
        // SBC X0, X0, X0
        let test_case = [0x00, 0x00, 0x00, 0xDA];
        let vm = run_test(&test_case, false);
        assert_eq!(vm.proc().x(0), Some(0xffffffffffffffff));

        let vm = run_test(&test_case, true);
        assert_eq!(vm.proc().x(0), Some(0));
    }
}
