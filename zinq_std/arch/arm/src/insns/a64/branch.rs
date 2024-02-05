use zinq::insn::{
    semantics::{expr::*, IrBlock},
    syntax::Decodable,
    Instruction,
};
use zinq::*;

use crate::{insns::a64::cond_symbol, Arm};

use super::{condition_holds, CondCode};

#[derive(Debug, Clone)]
pub struct UncondImm {
    raw: u32,
    op: bool,
    imm: isize,
}

impl Decodable<u32> for UncondImm {
    const FIXEDBITS: u32 = 0b0001_0100_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: u32 = 0b0111_1100_0000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for UncondImm {
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

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        let mut code = IrBlock::new();

        assign_64!(pc <= read_proc_64!(&proc.pc), in code);

        if self.op {
            assign_64!(ret_addr <= add_64!(var!(pc), lit!(4)), in code);
            proc_write_64!(var!(ret_addr) => &proc.r[30], in code);
        }

        assign_64!(dst_addr <= add_64!(var!(pc), lit!(self.imm as u64)), in code);
        assign_addr_from!(dst_addr <= to_addr_from_64!(var!(dst_addr)), in code);

        br_uncond!(var!(dst_addr), in code);

        code
    }
}

#[derive(Debug, Clone)]
pub struct Conditionally {
    raw: u32,
    imm: isize,
    consistent: bool,
    cond: u8,
}

impl Decodable<u32> for Conditionally {
    const FIXEDBITS: u32 = 0b0101_0100_0000_0000_0000_0000_0000_0000;
    const FIXEDMASK: u32 = 0b1111_1111_0000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for Conditionally {
    type InsnSize = u32;

    fn decode(raw: &[u8]) -> Result<Self> {
        let raw = u32::from_le_bytes(
            raw.get(0..4)
                .ok_or_else(|| Error(format!("Mem read fail")))?
                .try_into()
                .unwrap(),
        );

        let imm19 = slice!(raw from 5 for 19);
        println!("{imm19}");
        let consistent = bit!(4 of raw);
        let cond = slice!(raw from 0 for 4) as u8;

        let imm = imm19 << 2;

        Ok(Self {
            raw,
            imm: imm as isize,
            consistent,
            cond,
        })
    }

    fn name(&self) -> String {
        let cond = cond_symbol(CondCode::from(self.cond));
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
        let cond = cond_symbol(CondCode::from(self.cond));
        if self.consistent {
            format!("BC.{cond} #{0:X}", self.imm)
        } else {
            format!("B.{cond} #{0:X}", self.imm)
        }
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        let mut code = IrBlock::new();

        let cond_holds = condition_holds(CondCode::from(self.cond), proc, &mut code);

        assign_64!(pc <= read_proc_64!(&proc.pc), in code);
        assign_64!(true_case <= add_64!(var!(pc), lit!(self.imm as u64)), in code);
        assign_64!(false_case <= add_64!(var!(pc), lit!(4)), in code);

        assign_addr_from!(true_case <= to_addr_from_64!(var!(true_case)), in code);
        assign_addr_from!(false_case <= to_addr_from_64!(var!(false_case)), in code);

        br_cond!(var!(cond_holds), var!(true_case), var!(false_case), in code);
        code
    }
}
