mod shared {
    mod debug {
        mod halting;
        pub use halting::*;
    }
    pub use debug::*;
    mod exceptions;
    pub use exceptions::*;
    mod functions {
        // mod counters;
        // pub use counters::*;
        // mod extern_aborts;
        // pub use extern_aborts::*;
        mod memory;
        pub use memory::*;
        mod mpam;
        pub use mpam::*;
        mod registers;
        pub use registers::*;
        mod sve;
        pub use sve::*;
        mod system;
        pub use system::*;
    }
    pub use functions::*;
    mod translation {
        mod gpc;
        pub use gpc::*;
        mod translation;
        pub use translation::*;
        mod vmsa;
        pub use vmsa::*;
    }
    pub use translation::*;
}
pub use shared::*;

mod aarch64 {
    mod debug {
        pub mod brbe;
    }
    pub use debug::*;
    mod exceptions {
        mod aborts;
        pub use aborts::*;
        mod r#async;
        pub use r#async::*;
        mod syscalls;
        pub use syscalls::*;
        mod takeexception;
        pub use takeexception::*;
        mod traps;
        pub use traps::*;
    }
    pub use exceptions::*;
    mod functions {
        mod cache;
        pub use cache::*;
        mod dc;
        pub use dc::*;
        mod eret;
        pub use eret::*;
        mod gcs;
        pub use gcs::*;
        mod mec;
        pub use mec::*;
        mod memory;
        pub use memory::*;
        mod pac;
        pub use pac::*;
        mod sve;
        pub use sve::*;
        mod system;
        pub use system::*;
    }
    pub use functions::*;
    mod translation {
        mod hdbss;
        pub use hdbss::*;
        mod vmsa_addrcalc;
        pub use vmsa_addrcalc::*;
        mod vmsa_faults;
        pub use vmsa_faults::*;
        mod vmsa_tlbcontext;
        pub use vmsa_tlbcontext::*;
        mod vmsa_translation;
        pub use vmsa_translation::*;
        mod vmsa_ttentry;
        pub use vmsa_ttentry::*;
        mod vmsa_walk;
        pub use vmsa_walk::*;
        mod vmsa_walkparams;
        pub use vmsa_walkparams::*;
    }
    pub use translation::*;
}
pub use aarch64::*;

// Stubs not in the ARM spec but needed

use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn s1_tlb_lookup(&self, tlbcontext: &TLBContext) -> Option<&TLBLine> {
        // println!("S1TLBLookup IA = 0x{0:x}.", tlbcontext.ia);
        self.ctx.tlb_get(tlbcontext)
    }

    pub fn s1_tlb_cache(&mut self, tlbcontext: TLBContext, tlbrecord: TLBRecord) {
        self.ctx.tlb_insert(
            tlbcontext,
            TLBLine {
                valid_name: true,
                tlbrecord,
            },
        );
    }
}
