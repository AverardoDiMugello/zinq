pub mod semantics;

pub mod syntax {
    use std::fmt;

    /// slice a value as val[start:start+len]
    #[macro_export]
    macro_rules! slice {
        ($val:ident from $start:literal for $len:literal) => {
            ((1 << $len) - 1) & ($val >> $start)
        };
    }

    /// the nth bit of val
    #[macro_export]
    macro_rules! bit {
        ($n:literal of $val:ident) => {
            ($val >> $n & 0b1) != 0
        };
    }

    /// a bit vector of n ones
    #[macro_export]
    macro_rules! ones {
        [$n:expr] => {
            (1 << $n) - 1
        };
    }

    /// initialize a number using bit vector-like syntax
    #[macro_export]
    macro_rules! bvec {
        [] => {

        };
    }

    pub trait Decodable<W>: fmt::Debug + Clone {
        const FIXEDBITS: W;
        const FIXEDMASK: W;
    }
}

use crate::system::Processor;
use crate::Result;

use semantics::IrBlock;
use syntax::Decodable;

pub trait Instruction<P: Processor>: Decodable<Self::InsnSize> + Sized {
    /// The size of the instruction
    type InsnSize;

    /// Attempt to decode this instruction from a reference to a specific memory location
    fn decode(raw: &[u8]) -> Result<Self>;

    /// Return a name for the instruction
    fn name(&self) -> String;

    /// Return the instruction as binary data
    fn assemble(&self) -> &Self::InsnSize;

    /// Return a string representation of the instruction
    fn disassemble(&self) -> String;

    /// Return the size of an instruction in bytes
    fn size(&self) -> usize;

    /// Return the semantics of the instruction in the IR
    fn semantics<'p>(&self, proc: &'p P) -> IrBlock<'p>;
}
