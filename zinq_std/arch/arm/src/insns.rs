use crate::Arm;
use zinq_macros::insn_set;

mod helpers {
    mod common;
    pub use common::*;
    mod integer;
    pub use integer::*;
    mod registers;
    pub use registers::*;
    mod system;
    pub use system::*;
}

pub mod a64;

insn_set!(
    Arm,
    a64::branch::Cond,
    a64::branch::CondCmp,
    a64::branch::CondTest,
    a64::branch::UncondImm,
    a64::data::ArithCarry,
    a64::data::ArithImm,
    a64::data::ArithShift,
    a64::data::MovImm,
);
