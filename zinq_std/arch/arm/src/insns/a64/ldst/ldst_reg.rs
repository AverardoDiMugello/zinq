// val execute_aarch64_instrs_memory_single_general_register

/*
WIP!
 */

use bitvec::prelude::*;

use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{insns::a64, Arm};

#[derive(Debug, Clone)]
pub struct LdstReg {
    raw: u32,
    size_1: bool,
    size_0: bool,
    unknown: bool,
}
