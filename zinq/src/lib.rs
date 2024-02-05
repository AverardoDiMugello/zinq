use std::{error, fmt, result};

pub mod insn;
pub mod system;

#[derive(Debug)]
pub struct Error(pub String);

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zinq Error: {self:?}")
    }
}

pub type Result<T> = result::Result<T, Error>;

pub trait Emulator<S> {
    fn run(&mut self, sys: &mut S);
}
