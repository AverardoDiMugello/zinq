use bitvec::slice::BitSlice;

pub mod semantics;
pub mod syntax;

use crate::system::Processor;

use semantics::IrBlock;
use syntax::Decodable;

pub trait Instruction<P: Processor>: Decodable<Self::InsnSize> + Sized {
    type InsnSize;

    /// Attempt to decode this instruction from a reference to bit-addressable memory
    fn decode(raw: &BitSlice) -> Option<Self>;

    /// Return the instruction as binary data
    fn assemble(&self) -> Self::InsnSize;

    /// Return a string representation of the instruction
    fn disassemble(&self, proc: &P) -> String;

    /// Return the size of an instruction in bytes
    fn size(&self) -> usize;

    /// Append the semantics of the instruction to the given IR block
    fn semantics<'p>(&self, proc: &'p P, code: &mut IrBlock<'p>);
}
