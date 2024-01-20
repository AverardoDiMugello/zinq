use zinq::insn::{
    semantics::{expr::*, IrBlock},
    syntax::Decodable,
    Instruction,
};
use zinq::{assign, goto, lit, load, var};
use zinq::{Error, Result};

use crate::Arm;

#[derive(Debug, Clone)]
pub struct Branch {
    raw: u32,
    op: bool,
    imm: isize,
}

impl Decodable<u32> for Branch {
    const FIXEDBITS: u32 = 0b0001_0100_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: u32 = 0b0111_1100_0000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for Branch {
    type InsnSize = u32;

    fn decode(raw: &[u8]) -> Result<Self> {
        let raw = u32::from_le_bytes(
            raw.get(0..4)
                .ok_or_else(|| Error(format!("Mem read fail")))?
                .try_into()
                .unwrap(),
        );

        let op = (raw >> 31) & 0b1 != 0;
        let imm26 = ((1 << 26) - 1) & (raw >> 0) as isize;

        let imm = imm26 << 2;

        Ok(Self { raw, op, imm })
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

    fn disassemble(&self) -> String {
        if self.op {
            format!("BL #{0:X}", self.imm)
        } else {
            format!("B #{0:X}", self.imm)
        }
    }

    fn size(&self) -> usize {
        4
    }

    fn semanitcs<'ctx>(&self, ctx: &'ctx Arm) -> IrBlock<'ctx> {
        let mut code = IrBlock::new();

        assign!(pc <= load!(&ctx.pc), in code);

        if self.op {
            assign!(ret_addr <= Expr64::Arith(Arith::Add { a: var!(pc), b: lit!(4) }), in code);
            code.write_ctx_64(&ctx.r[30], var!(ret_addr));
        }

        assign!(
            dst_addr <= Expr64::Arith(Arith::Add {
                a: var!(pc),
                b: lit!(self.imm as u64)
            }), in code
        );

        goto!(var!(dst_addr), in code);

        code
    }
}
