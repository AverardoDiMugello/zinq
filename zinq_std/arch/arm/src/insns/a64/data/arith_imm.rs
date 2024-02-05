use zinq::insn::{
    semantics::{expr::*, IrBlock},
    syntax::Decodable,
    Instruction,
};
use zinq::*;

use crate::{insns::a64, Arm};

#[derive(Clone, Debug)]
pub struct ArithImm {
    raw: u32,
    sf: bool,
    op: bool,
    s: bool,
    sh: bool,
    imm: u32,
    rn: usize,
    rd: usize,
}

impl Decodable<u32> for ArithImm {
    // TODO: update this to reflect combining all the arith immediate insns
    const FIXEDBITS: u32 = 0b0001_0001_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: u32 = 0b0001_1111_1000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for ArithImm {
    type InsnSize = u32;

    fn decode(raw: &[u8]) -> Result<Self> {
        let raw = u32::from_le_bytes(
            raw.get(0..4)
                .ok_or_else(|| Error(format!("Mem read fail")))?
                .try_into()
                .unwrap(),
        );

        let sf = bit!(31 of raw);
        let op = bit!(30 of raw);
        let s = bit!(29 of raw);
        let sh = bit!(22 of raw);
        let imm12 = slice!(raw from 10 for 12);
        let rn = slice!(raw from 5 for 5);
        let rd = slice!(raw from 0 for 5);

        let imm = if sh { imm12 << 12 } else { imm12 };

        Ok(Self {
            raw,
            sf,
            op,
            s,
            sh,
            imm,
            rn: rn as usize,
            rd: rd as usize,
        })
    }

    fn name(&self) -> String {
        // Subtract?
        if self.op {
            // Signed?
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

    fn assemble(&self) -> &u32 {
        &self.raw
    }

    fn disassemble(&self) -> String {
        let rd = a64::reg_symbol(self.sf, self.rd);
        let rn = a64::reg_symbol(self.sf, self.rn);
        let shift = if self.sh {
            String::from(", LSL #12")
        } else {
            String::new()
        };

        // Subtract?
        if self.op {
            // Signed?
            if self.s {
                // CMP special case
                if self.rd == 0b11111 {
                    format!("CMP {rn}, #{0:X}{shift}", self.imm)
                } else {
                    format!("SUBS {rd}, {rn}, #{0:X}{shift}", self.imm)
                }
            } else {
                format!("SUB {rd}, {rn}, #{0:X}{shift}", self.imm)
            }
        } else {
            if self.s {
                // CMN special case
                if self.rd == 0b11111 {
                    format!("CMN {rn}, #{0:X}{shift}", self.imm)
                } else {
                    format!("ADDS {rd}, {rn}, #{0:X}{shift}", self.imm)
                }
            } else {
                format!("ADD {rd}, {rn}, #{0:X}{shift}", self.imm)
            }
        }
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        let mut code = IrBlock::new();

        let (result, n, z, c, v) = if self.sf {
            // 64-bit variant
            // operand1 = if n == 31 then SP[]<datasize-1:0> else X[n, datasize];
            let operand1 = if self.rn == 31 {
                assign_64!(operand1 <= read_proc_64!(proc.sp()), in code);
                operand1
            } else {
                assign_64!(operand1 <= read_proc_64!(&proc.r[self.rn]), in code);
                operand1
            };

            let (operand2, carry) = if self.op {
                // operand2 = Not(imm12)
                assign_64!(op2 <= not_64!(lit!(self.imm as u64)), in code);
                // carry = 1
                let carry = 1;
                (op2, carry)
            } else {
                // operand2 = imm
                assign_64!(op2 <= term_64!(lit!(self.imm as u64)), in code);
                // carry = 0
                let carry = 0;
                (op2, carry)
            };

            // (result, nzcv) = AddWithCarry(operand1, operand2, carry_in);
            a64::add_with_carry_64(operand1, operand2, carry, &mut code)
        } else {
            // 32-bit variant
            // operand1 = if n == 31 then SP[]<datasize-1:0> else X[n, datasize];
            let (rn, operand1) = if self.rn == 31 {
                assign_64!(rn <= read_proc_64!(proc.sp()), in code);
                assign_32_from!(operand1 <= trunc_64_to_32!(var!(rn)), in code);
                (rn, operand1)
            } else {
                assign_64!(rn <= read_proc_64!(&proc.r[self.rn]), in code);
                assign_32_from!(operand1 <= trunc_64_to_32!(var!(rn)), in code);
                (rn, operand1)
            };

            let (operand2, carry) = if self.op {
                // operand2 = Not(imm12)
                assign_32!(op2 <= not_32!(lit!(self.imm)), in code);
                // carry = 1
                let carry = 1;
                (op2, carry)
            } else {
                // operand2 = imm
                assign_32!(op2 <= term_32!(lit!(self.imm)), in code);
                // carry = 0
                let carry = 0;
                (op2, carry)
            };

            // (result, nzcv) = AddWithCarry(operand1, operand2, carry_in);
            let (result, n, z, c, v) = a64::add_with_carry_32(operand1, operand2, carry, &mut code);

            // Merge lower 32-bits of result into rn
            let mask = (1 << 32) - 1 as u64;
            assign_64_from!(b <= zext_32_to_64!(var!(result)), in code);
            assign_64!(a_or_b <= or_64!(var!(rn), var!(b)), in code);
            assign_64!(and_mask <= and_64!(var!(a_or_b), lit!(mask)), in code);
            assign_64!(result <= or_64!(var!(rn), var!(and_mask)), in code);
            (result, n, z, c, v)
        };

        if self.s {
            // (PSTATE.N @ PSTATE.Z @ PSTATE.C @ PSTATE.V) = nzcv
            proc_write_bool!(var!(n) => &proc.pstate.n, in code);
            proc_write_bool!(var!(z) => &proc.pstate.z, in code);
            proc_write_bool!(var!(c) => &proc.pstate.c, in code);
            proc_write_bool!(var!(v) => &proc.pstate.v, in code);
        }

        if self.rd == 31 {
            // SP_set() = ZeroExtend(result, 64)
            proc_write_64!(var!(result) => proc.sp(), in code);
        } else {
            // X_set(d, datasize) = result
            proc_write_64!(var!(result) => &proc.r[self.rd], in code);
        }

        a64::next_insn(proc, &mut code);

        code
    }
}
