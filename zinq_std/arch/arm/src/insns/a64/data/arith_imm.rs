use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{
    insns::{a64, helpers::*},
    Arm,
};

#[derive(Clone, Debug)]
pub struct ArithImm {
    raw: u32,
    sf: bool,
    op: bool,
    s: bool,
    sh: bool,
    imm12: BitArr!(for 12),
    rn: usize,
    rd: usize,
}

impl Decodable<a64::InsnSize> for ArithImm {
    // const FIXEDBITS: a64::InsnSize = bitarr![const 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0];
    // const FIXEDMASK: a64::InsnSize = bitarr![const 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0];
    const FIXEDBITS: a64::InsnSize = 0b0001_0001_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: a64::InsnSize = 0b0001_1111_1000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for ArithImm {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let mut imm12 = BitArray::ZERO;
        imm12 |= bits.get(10..22)?;

        Some(Self {
            raw: bits.get(0..32)?.load(),
            sf: *bits.get(31)?,
            op: *bits.get(30)?,
            s: *bits.get(29)?,
            sh: *bits.get(22)?,
            imm12,
            rn: bits.get(5..10)?.load(),
            rd: bits.get(0..5)?.load(),
        })
    }

    fn name(&self) -> String {
        // Subtract?
        if self.op {
            // Setflags?
            if self.s {
                // CMP special case
                if self.rd == 0b11111 {
                    String::from("CMP (immediate")
                } else {
                    String::from("SUBS (immediate)")
                }
            } else {
                String::from("SUB (immediate)")
            }
        } else {
            if self.s {
                // CMN special case
                if self.rd == 0b11111 {
                    String::from("CMN (immediate)")
                } else {
                    String::from("ADDS (immediate)")
                }
            } else {
                String::from("ADD (immediate)")
            }
        }
    }

    fn assemble(&self) -> &a64::InsnSize {
        &self.raw
    }

    fn disassemble(&self, _proc: &Arm) -> String {
        let rd = reg_symbol(self.sf, self.rd);
        let rn = reg_symbol(self.sf, self.rn);
        let shift = if self.sh {
            String::from(", LSL #12")
        } else {
            String::new()
        };
        let imm = self.imm12.load::<u32>();

        // Subtract?
        if self.op {
            // Set flags?
            if self.s {
                // CMP special case
                if self.rd == 0b11111 {
                    format!("CMP {rn}, #{imm:X}{shift}")
                } else {
                    format!("SUBS {rd}, {rn}, #{imm:X}{shift}")
                }
            } else {
                format!("SUB {rd}, {rn}, #{imm:X}{shift}")
            }
        } else {
            if self.s {
                // CMN special case
                if self.rd == 0b11111 {
                    format!("CMN {rn}, #{imm:X}{shift}")
                } else {
                    format!("ADDS {rd}, {rn}, #{imm:X}{shift}")
                }
            } else {
                format!("ADD {rd}, {rn}, #{imm:X}{shift}")
            }
        }
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        // Decode
        let datasize = if self.sf { 64 } else { 32 };
        let imm = {
            let mut imm = if self.sh {
                let mut imm = bitvec![0;12];
                imm.extend_from_bitslice(&self.imm12);
                imm
            } else {
                self.imm12.to_bitvec()
            };
            imm.resize(datasize, false);
            imm
        };

        let mut code: IrBlock<'_> = IrBlock::new();

        // let operand1 : bits('datasize) = if n == 31 then
        //   SP_read()[datasize - 1 .. 0]
        // else
        //   X_read(n, datasize);
        let operand1 = if self.rn == 31 {
            sp_read(datasize, proc, &mut code)
        } else {
            x_read(self.rn, datasize, proc, &mut code)
        };

        // operand2 : bits('datasize) = imm;
        // nzcv : bits(4) = undefined;
        // carry_in : bits(1) = undefined;
        // if sub_op then {
        //     operand2 = not_vec(operand2);
        //     carry_in = 0b1
        // } else {
        //     carry_in = 0b0
        // };
        let (operand2, carry_in) = if self.op {
            let operand2 = code.assign(Expr::Unary(UnaOp::Not, Term::Lit(imm)));
            (operand2, bitvec!(1))
        } else {
            let operand2 = code.assign(Expr::Term(Term::Lit(imm)));
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

        // if d == 31 & not_bool(setflags) then {
        //     SP_set() = ZeroExtend(result, 64)
        // } else {
        //     X_set(d, datasize) = result
        // }
        if self.rd == 31 && !self.s {
            let zext_result = code.assign(Expr::Zext {
                val: Term::Var(result),
                size: 64,
            });
            sp_set(Term::Var(zext_result), proc, &mut code);
        } else {
            x_set(self.rd, Term::Var(result), proc, &mut code);
        }

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

    fn run_test(test_case: &[u8], x0: u64) -> System<Arm> {
        let mut vm = System::new(Arm::new(Armv9p4a), 8);
        vm.write_mem(0, test_case).unwrap();

        let proc = vm.proc_mut();
        proc.set_ip(0);
        proc.set_x(0, x0);

        let mut emu = StepEmu::new();
        emu.max_insns(1);
        emu.run(&mut vm);
        vm
    }

    #[test]
    fn add_64() {
        // ADD X1, X0, #1
        let test_case = [0x01, 0x04, 0x00, 0x91];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().x(1), Some(0xaaaaaaaa00000000 + 1));
    }

    #[test]
    fn add_32() {
        // ADD W1, W0, #1
        let test_case = [0x01, 0x04, 0x00, 0x11];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().x(1), Some(1));
    }

    #[test]
    fn add_sh_64() {
        // ADD X1, X0, #1, LSL #12
        let test_case = [0x01, 0x04, 0x40, 0x91];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().x(1), Some(0xaaaaaaaa00000000 + (1 << 12)));
    }

    #[test]
    fn add_sh_32() {
        // ADD W1, W0, #1, LSL #12
        let test_case = [0x01, 0x04, 0x40, 0x11];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().x(1), Some(1 << 12));
    }

    #[test]
    fn sub_64() {
        // SUB X1, X0, #1
        let test_case = [0x01, 0x04, 0x00, 0xD1];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().x(1), Some(0xaaaaaaaa00000000 - 1));
    }

    #[test]
    fn sub_32() {
        // SUB W1, W0, #1
        let test_case = [0x01, 0x04, 0x00, 0x51];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().x(1), Some(0x00000000ffffffff));
    }

    #[test]
    fn sub_sh_64() {
        // SUB X1, X0, #1, LSL #12
        let test_case = [0x01, 0x04, 0x40, 0xD1];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().x(1), Some(0xaaaaaaaa00000000 - (1 << 12)));
    }

    #[test]
    fn sub_sh_32() {
        // SUB W1, W0, #1, LSL #12
        let test_case = [0x01, 0x04, 0x40, 0x51];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().x(1), Some(0x00000000fffff000));
    }

    #[test]
    fn flag_negative_64() {
        // ADDS X1, X0, #1
        let test_case = [0x01, 0x04, 0x00, 0xB1];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().n(), true);
    }

    #[test]
    fn flag_negative_32() {
        // ADDS W1, W0, #-1
        let test_case = [0x01, 0x04, 0x00, 0x71];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().n(), true);
    }

    #[test]
    fn flag_zero_64() {
        // SUBS X1, X0, #1
        let test_case = [0x01, 0x04, 0x00, 0xF1];
        let vm = run_test(&test_case, 1);
        assert_eq!(vm.proc().z(), true);
    }

    #[test]
    fn flag_zero_32() {
        // SUBS W1, W0, #0
        let test_case = [0x01, 0x00, 0x00, 0x71];
        let vm = run_test(&test_case, 0xaaaaaaaa00000000);
        assert_eq!(vm.proc().z(), true);
    }

    #[test]
    fn flag_carry_64() {
        // ADDS X1, X0, #1, LSL #12
        let test_case = [0x01, 0x04, 0x40, 0xB1];
        let vm = run_test(&test_case, 0xfffffffffffff000);
        assert_eq!(vm.proc().c(), true);
    }

    #[test]
    fn flag_carry_32() {
        // ADDS W1, W0, #1, LSL #12
        let test_case = [0x01, 0x04, 0x40, 0x31];
        let vm = run_test(&test_case, 0x00000000fffff000);
        assert_eq!(vm.proc().c(), true);
    }

    #[test]
    fn flag_overflow_64() {
        // ADDS X1, X0, #1
        let test_case = [0x01, 0x04, 0x00, 0xB1];
        let vm = run_test(&test_case, 0x7fffffffffffffff);
        assert_eq!(vm.proc().v(), true);
    }

    #[test]
    fn flag_overflow_32() {
        // ADDS W1, W0, #1
        let test_case = [0x01, 0x04, 0x00, 0x31];
        let vm = run_test(&test_case, 0x000000007fffffff);
        assert_eq!(vm.proc().v(), true);
    }
}
