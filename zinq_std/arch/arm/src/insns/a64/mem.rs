use zinq::insn::{syntax::Decodable, Instruction};

use crate::Arm;

#[derive(Debug, Clone)]
pub struct StReg {
    raw: u32,
    size_bit_31: bool,
    size_bit_30: bool,
    opc_bit_23: bool,
    opc_bit_22: bool,
    rm: usize,
    option: u32,
    s: bool,
    rn: usize,
    rt: usize,
}

impl Decodable<u32> for StReg {
    const FIXEDBITS: u32 = 0b0011_1000_0010_0000_0000_1000_0000;
    const FIXEDMASK: u32 = 0b0011_1111_0010_0000_0000_1100_0000;
}

// impl Instruction<Arm> for LdStReg {
//     type InsnSize = u32;

//     fn name(&self) -> String {

//     }
// }
