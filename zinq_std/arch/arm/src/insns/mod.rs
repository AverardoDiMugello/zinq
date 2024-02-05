use zinq_macros::insn_set;

use crate::Arm;

pub mod a64;

insn_set!(
    Arm,
    a64::data::ArithImm,
    a64::data::MovImm,
    a64::branch::UncondImm,
    a64::branch::Conditionally
);
