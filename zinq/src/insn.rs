use bitvec::prelude::*;

pub mod semantics;
pub mod syntax;

use crate::system::Processor;

use semantics::IrBlock;
use syntax::Decodable;

pub trait Instruction<P: Processor>: Decodable<Self::InsnSize> + Sized {
    /// The size of the instruction
    type InsnSize;

    /// Attempt to decode this instruction from a reference to a collection of bits
    fn decode(bits: &BitSlice) -> Option<Self>;

    /// Return a name for the instruction
    fn name(&self) -> String;

    /// Return the instruction as binary data
    fn assemble(&self) -> &Self::InsnSize;

    /// Return a string representation of the instruction
    fn disassemble(&self, proc: &P) -> String;

    /// Return the size of an instruction in bytes
    fn size(&self) -> usize;

    /// Return the semantics of the instruction in the IR
    fn semantics<'p>(&self, proc: &'p P) -> IrBlock<'p>;
}
