use zinq::{insn::Instruction, system::Processor, Error, Result};

pub mod insns;
use insns::ArmInsn;

#[derive(Debug)]
enum Version {
    Armv8a,
}

#[derive(Debug)]
pub struct Arm {
    // Arch state
    r: [u64; 31],
    sp: [u64; 4], // One SP for each EL
    pc: u64,
    pstate: Pstate,
    // Model state
    version: Version,
}

impl Arm {
    /// Initialize an Armv8a architecture
    pub fn v8() -> Self {
        Self {
            r: [0; 31],
            sp: [0; 4],
            pc: 0,
            pstate: Pstate::new(),
            version: Version::Armv8a,
        }
    }

    /// Get a reference to the SP based on EL
    pub fn sp(&self) -> &u64 {
        if self.pstate.sp {
            &self.sp[0]
        } else {
            match (self.pstate.el[1], self.pstate.el[0]) {
                (false, false) => &self.sp[0],
                (false, true) => &self.sp[1],
                (true, false) => &self.sp[2],
                (true, true) => &self.sp[3],
            }
        }
    }

    /// Get the 64-bit value of a general purpose register
    pub fn x(&self, index: usize) -> Result<u64> {
        Ok(self
            .r
            .get(index)
            .ok_or_else(|| Error(format!("No gpr {index}")))?
            .to_owned())
    }

    /// Set a general purpose register to a 64-bit value
    pub fn set_x(&mut self, index: usize, val: u64) -> Result<()> {
        let x = self
            .r
            .get_mut(index)
            .ok_or_else(|| Error(format!("No gpr {index}")))?;
        *x = val;
        Ok(())
    }

    /// Get the 32-bit value of a general purpose register
    pub fn w(&self, index: usize) -> Option<u32> {
        self.r.get(index).and_then(|x| Some(*x as u32))
    }
}

impl Processor for Arm {
    type Insn = ArmInsn;

    fn name(&self) -> &str {
        match self.version {
            Version::Armv8a => stringify!(Armv8a),
        }
    }

    fn ip(&self) -> usize {
        self.pc as usize
    }

    fn set_ip(&mut self, addr: usize) {
        self.pc = addr as u64
    }

    fn fetch_decode(&self, addr: usize, mem: &[u8]) -> Result<ArmInsn> {
        let insn = mem
            .get(addr..addr + 4)
            .ok_or_else(|| Error(format!("Could not fetch 4 bytes from {addr}")))?;

        ArmInsn::decode(insn)
    }
}

#[derive(Debug)]
struct Pstate {
    n: bool,
    z: bool,
    c: bool,
    v: bool,
    el: [bool; 2],
    sp: bool,
}

impl Pstate {
    fn new() -> Self {
        Self {
            n: false,
            z: false,
            c: false,
            v: false,
            el: [false; 2],
            sp: false,
        }
    }
}
