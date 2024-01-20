use zinq::insn::{
    semantics::{expr::*, IrBlock},
    syntax::Decodable,
    Instruction,
};
use zinq::{Error, Result};

use crate::Arm;

use super::{add_with_carry_64, reg_symbol};

// tmp
use zinq::{assign, load, store, var};

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
    const FIXEDMASK: u32 = 0b0111_1111_1000_0000_0000_0000_0000_0000;
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

        let sf = (raw >> 31 & 0b1) != 0;
        let op = (raw >> 30 & 0b1) != 0;
        let s = (raw >> 29 & 0b1) != 0;
        let sh = (raw >> 22 & 0b1) != 0;
        let imm12 = ((1 << 12) - 1) & (raw >> 10);
        let rn = ((1 << 5) - 1) & (raw >> 5);
        let rd = ((1 << 5) - 1) & (raw >> 0);

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
        let rd = reg_symbol(self.sf, self.rd);
        let rn = reg_symbol(self.sf, self.rn);
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

    fn semanitcs<'ctx>(&self, ctx: &'ctx Arm) -> IrBlock<'ctx> {
        let mut code = IrBlock::new();

        // 64-bit variant
        if self.sf {
            // operand1 = if n == 31 then SP[]<datasize-1:0> else X[n, datasize];
            let operand1 = if self.rn == 31 {
                assign!(operand1 <= load!(ctx.sp()), in code);
                operand1
            } else {
                assign!(operand1 <= load!(&ctx.r[self.rn]), in code);
                operand1
            };

            let (operand2, carry) = if self.op {
                // operand2 = Not(imm12)
                assign!(op2 <= Expr64::Logic(Logic::Not(Term::Lit(self.imm as u64))), in code);
                // carry = 1
                let carry = 1;
                (op2, carry)
            } else {
                // operand2 = imm
                assign!(op2 <= Expr64::Term(Term::Lit(self.imm as u64)), in code);
                // carry = 0
                let carry = 0;
                (op2, carry)
            };

            // (result, nzcv) = AddWithCarry(operand1, operand2, carry_in);
            let (result, n, z, c, v) = add_with_carry_64(operand1, operand2, carry, &mut code);

            if self.s {
                // (PSTATE.N @ PSTATE.Z @ PSTATE.C @ PSTATE.V) = nzcv
                store!(var!(n) => &ctx.pstate.n, in code);
                store!(var!(z) => &ctx.pstate.z, in code);
                store!(var!(c) => &ctx.pstate.c, in code);
                store!(var!(v) => &ctx.pstate.v, in code);
            }

            if self.rd == 31 {
                // SP_set() = ZeroExtend(result, 64)
                code.write_ctx_64(ctx.sp(), Term::Var(result));
            } else {
                // X_set(d, datasize) = result
                code.write_ctx_64(&ctx.r[self.rd], Term::Var(result));
            }
        } else {
            dbg!("32-bit versions of instructions are not supported yet!");
        }
        code
    }
}
