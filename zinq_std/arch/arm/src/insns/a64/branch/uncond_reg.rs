// use bitvec::prelude::*;

// use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

// use crate::{insns::a64, Arm};

// #[derive(Debug, Clone)]
// pub struct UncondReg {
//     raw: u32,
//     z: bool,
//     op_1: bool,
//     op_0: bool,
//     a: bool,
//     m: bool,
//     rn: usize,
//     rm: usize,
// }

// impl Decodable<a64::InsnSize> for UncondReg {
//     const FIXEDBITS: a64::InsnSize = ;
//     const FIXEDMASK: a64::InsnSize = ;
// }

// impl Instruction<Arm> for UncondReg {
//     type InsnSize = a64::InsnSize;

//     fn decode(bits: &BitSlice) -> Option<Self> {}

// }
