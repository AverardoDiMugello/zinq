use std::fmt;

use crate::insn::Instruction;
use crate::{Error, Result};

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

    pub fn write_mem(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        self.mem
            .get_mut(offset..offset + data.len())
            .ok_or_else(|| {
                Error(format!(
                    "Unable to access memory[{offset:X}..{0}]",
                    offset + data.len()
                ))
            })?
            .copy_from_slice(data);
        Ok(())
    }
}
