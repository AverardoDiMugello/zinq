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
        mod tlbi;
        pub use tlbi::*;
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
        let bit_widths_to_check = match tlbcontext.tg {
            TGx::TG4KB => [12, 21, 30, 39],
            TGx::TG16KB => [14, 25, 36, 47],
            TGx::TG64KB => [16, 29, 42, 64],
        };

        for (i, bit_width) in bit_widths_to_check.into_iter().enumerate() {
            let level = (3 - i) as i64;
            let ia_to_check = tlbcontext.ia & !((1 << bit_width) - 1) & !(0b1111_1111 << 56);
            // println!(
            //     "S1TLBLookup IA = 0x{0:x}, ia_base = 0x{ia_to_check:x}, TGx = {1:?}, level = {level}.",
            //     tlbcontext.ia,
            //     tlbcontext.tg,
            // );
            let tlbline = self.ctx.tlb_get(&ia_to_check);
            if tlbline.is_some_and(|tlbline| tlbline.tlbrecord.walkstate.level == level) {
                // println!("Hit: 0x{ia_to_check:x}");
                return tlbline;
            }
        }
        None
    }

    pub fn s1_tlb_cache(&mut self, tlbrecord: TLBRecord) {
        // This is the bit-width of the address range described by this translation
        let bits_described = tlbrecord
            .context
            .tg
            .translation_size(tlbrecord.context.isd128, tlbrecord.walkstate.level);

        // Any IA whose first (IAsize - bits_described) bits equals this value, shares a translation with this record (until invalidated).
        let ia_base = tlbrecord.context.ia & !((1 << bits_described) - 1) & !(0b1111_1111 << 56);

        // println!(
        //     "S1TLBCache ia = 0x{0:x}, ia_base = 0x{ia_base:x}, bits_described = {bits_described}, TGx = {1:?}, level = {2}.",
        //     tlbrecord.context.ia,
        //     tlbrecord.context.tg,
        //     tlbrecord.walkstate.level,
        // );

        self.ctx.tlb_insert(
            ia_base,
            TLBLine {
                valid_name: true,
                tlbrecord,
            },
        );
    }
}
