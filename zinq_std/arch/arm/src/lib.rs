use bitvec::prelude::*;

use zinq::{insn::Instruction, system::Processor, Error, Result};

pub mod insns;
use insns::ArmInstruction;
mod variants;
pub use variants::Version;

type Flag = BitArr!(for 1);
type El = BitArr!(for 2);
type Reg64 = BitArr!(for 64);

#[derive(Debug)]
enum Endian {
    Big,
    Little,
}

#[derive(Debug)]
pub struct Arm {
    // Arch state
    r: [Reg64; 31],
    sp: [Reg64; 4], // One SP for each EL
    pc: Reg64,
    pstate: Pstate,
    // Model state
    version: Version,
    endian: Endian,
}

impl Arm {
    /// Initialize an Arm CPU of the given version
    pub fn new(v: Version) -> Self {
        Self {
            r: [Reg64::ZERO; 31],
            sp: [Reg64::ZERO; 4],
            pc: Reg64::ZERO,
            pstate: Pstate::new(),
            version: v,
            endian: Endian::Little,
        }
    }

    /// Initialize a big endian Arm CPU of the given version
    pub fn new_be(v: Version) -> Self {
        Self {
            r: [Reg64::ZERO; 31],
            sp: [Reg64::ZERO; 4],
            pc: Reg64::ZERO,
            pstate: Pstate::new(),
            version: v,
            endian: Endian::Big,
        }
    }

    /// Get a reference to the SP based on EL
    pub fn sp(&self) -> &Reg64 {
        if self.pstate.sp[0] {
            &self.sp[0]
        } else {
            let (hi, lo) = (self.pstate.el[1], self.pstate.el[0]);
            match (hi, lo) {
                (false, false) => &self.sp[0],
                (false, true) => &self.sp[1],
                (true, false) => &self.sp[2],
                (true, true) => &self.sp[3],
            }
        }
    }

    /// Get a reference to a general purpose register
    pub fn x(&self, index: usize) -> Option<u64> {
        self.r.get(index).and_then(|x| Some(x.load()))
    }

    /// Get a mutable reference to a general purpose register
    pub fn set_x(&mut self, index: usize, val: u64) -> Option<u64> {
        self.r.get_mut(index).and_then(|x| {
            x.store(val);
            Some(val)
        })
    }

    /// Get a reference to the lower 32-bits of a general purpose register
    pub fn w(&self, index: usize) -> Option<u32> {
        self.r
            .get(index)
            .and_then(|x| Some(&x[0..32]))
            .and_then(|w| Some(w.load()))
    }

    /// Get a mutable reference to the lower 32-bits of a general purpose register
    pub fn set_w(&mut self, index: usize, val: u32) -> Option<u32> {
        self.r
            .get_mut(index)
            .and_then(|x| Some(&mut x[0..32]))
            .and_then(|w| {
                w.store(val);
                Some(val)
            })
    }

    /// Return the value of the negative condition flag (N)
    pub fn n(&self) -> bool {
        self.pstate.n[0]
    }

    /// Return the value of the zero condition flag (Z)
    pub fn z(&self) -> bool {
        self.pstate.z[0]
    }

    /// Return the value of the carry condition flag (C)
    pub fn c(&self) -> bool {
        self.pstate.c[0]
    }

    /// Return the value of the overflow condition flag (V)
    pub fn v(&self) -> bool {
        self.pstate.v[0]
    }
}

impl Processor for Arm {
    type Insn = ArmInstruction;

    fn name(&self) -> String {
        format!("{}", self.version)
    }

    fn ip(&self) -> usize {
        self.pc.load()
    }

    fn set_ip(&mut self, addr: usize) {
        self.pc.store(addr)
    }

    fn fetch_decode(&self, addr: usize, mem: &[u8]) -> Result<ArmInstruction> {
        let insn: [u8; 4] = mem
            .get(addr..addr + 4)
            .ok_or_else(|| Error(format!("Failed to fetch 4 bytes from {addr}")))?
            .try_into()
            .unwrap();

        let insn = match self.endian {
            Endian::Little => u32::from_le_bytes(insn),
            Endian::Big => panic!("Big Endian ARM Unsupported"),
        } as usize;

        ArmInstruction::decode(insn.view_bits()).ok_or_else(|| {
            Error(format!(
                "Failed to decode the 4 bytes at {addr} into an instruction"
            ))
        })
    }
}

#[derive(Debug)]
struct Pstate {
    n: Flag,
    z: Flag,
    c: Flag,
    v: Flag,
    el: El,
    sp: Flag,
}

impl Pstate {
    fn new() -> Self {
        Self {
            n: Flag::ZERO,
            z: Flag::ZERO,
            c: Flag::ZERO,
            v: Flag::ZERO,
            el: El::ZERO,
            sp: Flag::ZERO,
        }
    }
}
