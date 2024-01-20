use std::fmt;

use crate::insn::Instruction;
use crate::Result;

pub trait Processor: fmt::Debug + Sized {
    type Insn: Instruction<Self>;

    fn name(&self) -> &str;
    fn ip(&self) -> usize;
    fn set_ip(&mut self, addr: usize);
    fn fetch_decode(&self, addr: usize, mem: &[u8]) -> Result<Self::Insn>;
}

pub struct System<P: Processor> {
    proc: P,
    mem: Vec<u8>,
}

impl<P: Processor> System<P> {
    pub fn new(proc: P, mem_size: usize) -> Self {
        Self {
            proc,
            mem: vec![0; mem_size],
        }
    }

    pub fn proc(&self) -> &P {
        &self.proc
    }

    pub fn proc_mut(&mut self) -> &mut P {
        &mut self.proc
    }

    pub fn mem(&self) -> &[u8] {
        &self.mem
    }

    pub fn mem_mut(&mut self) -> &mut [u8] {
        &mut self.mem
    }
}
