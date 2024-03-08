use crate::Arm;
use zinq_macros::insn_set;

mod disas;
mod helpers {
    mod extend_reg;
    pub use extend_reg::*;
    mod integer;
    pub use integer::*;
    mod registers;
    pub use registers::*;
    // mod system;
    // pub use system::*;
    mod enums;
    pub use enums::*;
}

pub mod a64;

insn_set!(
    Arm,
    a64::data_proc::imm::arithmetic::Add,
    a64::data_proc::imm::arithmetic::Adds,
    a64::data_proc::imm::arithmetic::Sub,
    a64::data_proc::imm::arithmetic::Subs,
    a64::data_proc::imm::logical::IntegerLogicalImmediate,
    a64::data_proc::imm::pc_rel::Adr,
    a64::data_proc::imm::pc_rel::Adrp,
    a64::data_proc::reg::arithmetic_shift::IntegerArithmeticAddSubShiftedreg,
    a64::branch_exc_sys::BranchUnconditionalImmediate,
    a64::branch_exc_sys::BranchConditionalCond,
    a64::branch_exc_sys::BranchUnconditionalRegister,
    a64::load_store::MemoryPairGeneralPostIdx,
    a64::load_store::MemoryLiteralGeneral,
    a64::load_store::MemorySingleGeneralImmediateSignedPostIndex,
    a64::branch_exc_sys::SystemHints,
    a64::branch_exc_sys::SystemRegisterSystem,
);
