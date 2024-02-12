// pub type InsnSize = BitArr!(for 32);
pub type InsnSize = u32;

pub mod branch {
    mod cond_cmp;
    pub use cond_cmp::CondCmp;
    mod cond_test;
    pub use cond_test::CondTest;
    mod cond;
    pub use cond::Cond;
    mod uncond_imm;
    pub use uncond_imm::UncondImm;
    // mod uncond_reg;
    // pub use uncond_reg::UncondReg;
}

pub mod data {
    mod arith_imm;
    pub use arith_imm::ArithImm;
    mod mov_imm;
    pub use mov_imm::MovImm;
}

// pub mod mem;
