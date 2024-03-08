// pub type InsnSize = BitArr!(for 32);
pub type InsnSize = u32;

pub mod data_proc {
    pub mod imm {
        pub mod arithmetic;
        pub mod logical;
        pub mod pc_rel;
    }

    pub mod reg {
        pub mod arithmetic_shift;
    }
}

pub mod load_store {
    mod memory_single_general_immediate_signed_post_index;
    pub use memory_single_general_immediate_signed_post_index::MemorySingleGeneralImmediateSignedPostIndex;
    mod memory_pair_general_post_idx;
    pub use memory_pair_general_post_idx::MemoryPairGeneralPostIdx;
    mod memory_literal_general;
    pub use memory_literal_general::MemoryLiteralGeneral;
}

pub mod branch_exc_sys {
    mod branch_unconditional_immediate;
    pub use branch_unconditional_immediate::BranchUnconditionalImmediate;
    mod branch_conditional_cond;
    pub use branch_conditional_cond::BranchConditionalCond;
    mod branch_unconditional_register;
    pub use branch_unconditional_register::BranchUnconditionalRegister;
    mod system_hints;
    pub use system_hints::SystemHints;
    mod system_register_system;
    pub use system_register_system::SystemRegisterSystem;
}
