use zinq::insn::{
    semantics::{expr::*, IrBlock},
    syntax::Decodable,
    Instruction,
};
use zinq::*;

use crate::{insns::a64, Arm};

#[derive(Debug, Clone)]
enum MovOp {
    K,
    N,
    Z,
}

impl From<(bool, bool)> for MovOp {
    fn from(value: (bool, bool)) -> Self {
        let opc_0 = value.0;
        let opc_1 = value.1;

        match (opc_1, opc_0) {
            (false, false) => MovOp::N,
            (false, true) => panic!("This MovImm should've never been created..."),
            (true, false) => MovOp::Z,
            (true, true) => MovOp::K,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovImm {
    raw: u32,
    sf: bool,
    opc: MovOp,
    hw_1: bool,
    hw_0: bool,
    imm16: u32,
    pos: u32,
    rd: usize,
}

impl Decodable<u32> for MovImm {
    const FIXEDBITS: u32 = 0b0001_0010_1000_0000_0000_0000_0000_0000;
    const FIXEDMASK: u32 = 0b0001_1111_1000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for MovImm {
    type InsnSize = u32;

    fn decode(raw: &[u8]) -> Result<Self> {
        let raw = u32::from_le_bytes(
            raw.get(0..4)
                .ok_or_else(|| Error(format!("Mem read fail")))?
                .try_into()
                .unwrap(),
        );

        let sf = bit!(31 of raw);
        let opc_1 = bit!(30 of raw);
        let opc_0 = bit!(29 of raw);
        let hw_1 = bit!(22 of raw);
        let hw_0 = bit!(21 of raw);
        let imm16 = slice!(raw from 5 for 16);
        let rd = slice!(raw from 0 for 4);

        let opc = MovOp::from((opc_0, opc_1));

        if !sf && hw_1 {
            return Err(Error(format!("Undefined instruction")));
        }

        let pos = ((hw_1 as u32) << 1) | (hw_0 as u32) << 4;

        Ok(Self {
            raw,
            sf,
            opc,
            hw_1,
            hw_0,
            imm16,
            pos,
            rd: rd as usize,
        })
    }

    fn name(&self) -> String {
        match self.opc {
            MovOp::K => String::from("MOVK"),
            MovOp::N => String::from("MOVN"),
            MovOp::Z => String::from("MOVZ"),
        }
    }

    fn assemble(&self) -> &u32 {
        &self.raw
    }

    fn disassemble(&self) -> String {
        self.name()
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        let mut code = IrBlock::new();

        // 64-bit variant
        if self.sf {
            let result = if let MovOp::K = self.opc {
                assign_64!(result <= read_proc_64!(&proc.r[self.rd]), in code);
                result
            } else {
                assign_64!(result <= term_64!(lit!(0)), in code);
                result
            };

            let mask = (ones![16] << self.pos) as u64;
            let b = (self.imm16 << self.pos) as u64;
            let b_and_mask = b & mask;
            assign_64!(a_and_not_mask <= and_64!(var!(result), lit!(!mask)), in code);
            assign_64!(result <= or_64!(var!(a_and_not_mask), lit!(b_and_mask)), in code);

            let result = if let MovOp::N = self.opc {
                assign_64!(result <= not_64!(var!(result)), in code);
                result
            } else {
                result
            };

            proc_write_64!(var!(result) => &proc.r[self.rd], in code);
        } else {
            // let result = if let MovOp::K = self.opc {
            //     assign_64!(rd <= read_proc_64!(&proc.r[self.rd]), in code);
            //     assign_32_from!(result <= trunc_64_to_32!(var!(rd)), in code);
            //     result
            // } else {
            //     assign_64!(result <= term_32!(lit!(0)), in code)
            // };

            // let mask = (ones![16] << self.pos) as u32;
            // let b = (self.imm16 << self.pos) as u32;
            // let b_and_mask = b & mask;
            // assign_64!(a_and_not_mask <= and_64!(var!(result), lit!(!mask)), in code);
            // assign_64!(result <= or_64!(var!(a_and_not_mask), lit!(b_and_mask)), in code);

            // let result = if let MovOp::N = self.opc {
            //     assign_64!(result <= not_64!(var!(result)), in code);
            //     result
            // } else {
            //     result
            // };

            // proc_write_64!(var!(result) => &proc.r[self.rd], in code);
        }

        a64::next_insn(proc, &mut code);

        code
    }
}
