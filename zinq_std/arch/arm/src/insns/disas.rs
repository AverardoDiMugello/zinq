use bitvec::prelude::*;
use capstone::{
    arch::{arm64::ArchMode, BuildsCapstone, BuildsCapstoneEndian},
    Capstone, Endian,
};

use crate::Arm;

/// Capstone-based disassembly for all A64 instructions
pub fn a64(raw: u32, proc: &Arm) -> String {
    let cap = Capstone::new()
        .arm64()
        .mode(ArchMode::Arm)
        .endian(proc.endian)
        .build()
        .expect("Unable to build Arm64 Capstone instance");

    let bytes = match proc.endian {
        Endian::Big => raw.to_be_bytes(),
        Endian::Little => raw.to_le_bytes(),
    };
    let disas = cap
        .disasm_all(&bytes, proc.pc.load())
        .expect("Unable to disassemble instruction");
    let disas = disas
        .first()
        .expect("Instruction was disassembled but could not be unpacked from Capstone");
    format!("{disas}")
}
