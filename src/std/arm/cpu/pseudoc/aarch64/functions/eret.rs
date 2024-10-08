use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::ArmCtx;
use crate::std::arm::cpu::{pstate, registers::aarch64::*, PC};

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_exception_return(&mut self, new_pc: u64, spsr: u64) {
        let mut new_pc = new_pc;
        if self.is_feat_impl(Feat::TME) {
            // && TSTATE.depth > 0 {
            // TODO!
            // FailTransaction(TMFailure_ERR, false);
        }

        if self.is_feat_impl(Feat::IESB) {
            let mut sync_errors = match self.curr_el() {
                EL::EL0 => panic!("Unreachable"),
                EL::EL1 => self.read::<sctlr_el1::IESB>() != 0,
                EL::EL2 => self.read::<sctlr_el2::IESB>() != 0,
                EL::EL3 => self.read::<sctlr_el3::IESB>() != 0,
            };

            if self.is_feat_impl(Feat::DoubleFault) {
                sync_errors = sync_errors
                    || ((self.read::<scr_el3::EA>() != 0 && self.read::<scr_el3::NMEA>() != 0)
                        && self.curr_el() == EL::EL3);
            }

            if sync_errors {
                // self.SynchronizeErrors();
                let iesb_req = true;
                self.take_unmasked_physical_serror_interrupts(iesb_req);
            }
        }
        let _brbe_source_allowed;
        let _brbe_source_address;
        if self.is_feat_impl(Feat::BRBE) {
            let el = self.curr_el();
            _brbe_source_allowed = self.branch_record_allowed(el);
            _brbe_source_address = self.read::<PC>();
        } else {
            _brbe_source_allowed = false;
            _brbe_source_address = 0;
        }

        if !self.is_feat_impl(Feat::ExS)
            || match self.curr_el() {
                EL::EL0 => panic!("Unreachable"),
                EL::EL1 => self.read::<sctlr_el1::EOS>() != 0,
                EL::EL2 => self.read::<sctlr_el2::EOS>() != 0,
                EL::EL3 => self.read::<sctlr_el3::EOS>() != 0,
            }
        {
            // self.SynchronizeContext();
        }

        // Attempts to change to an illegal state will invoke the Illegal Execution state mechanism
        let _source_el = self.curr_el();
        let illegal_psr_state = self.illegal_exception_return(spsr);
        self.set_pstate_from_psr(spsr, illegal_psr_state);
        // ClearExclusiveLocal(ProcessorID());
        // SendEventLocal();

        if illegal_psr_state && (spsr >> 4) & 1 != 0 {
            // If the exception return is illegal, PC[63:32,1:0] are UNKNOWN
            // new_pc<63:32> = bits(32) UNKNOWN;
            // new_pc<1:0> = bits(2) UNKNOWN;
            panic!("illegal psr state!");
        } else if self.using_aarch32() {
            // Return to AArch32
            // ELR_ELx[1:0] or ELR_ELx[0] are treated as being 0, depending on the
            // target instruction set state
            if self.read::<pstate::T>() != 0 {
                new_pc &= !(0b1); // T32
            } else {
                new_pc &= !(0b11); // A32
            }
        } else {
            // Return to AArch64
            // ELR_ELx[63:56] might include a tag
            let el = self.curr_el();
            new_pc = self.aarch64_branch_addr(new_pc, el);
        }
        if self.is_feat_impl(Feat::BRBE) {
            // TODO
            // BRBEExceptionReturn(new_pc, source_el, brbe_source_allowed, brbe_source_address);
        }
        if self.using_aarch32() {
            if self.is_feat_impl(Feat::SME) && self.read::<pstate::SM>() != 0 {
                self.reset_sve_state();
            }

            // 32 most significant bits are ignored.
            // let branch_conditional = false;
            // BranchTo(new_pc<31:0>, BranchType::ERET, branch_conditional);
            todo!("AArch32");
        } else {
            self.branch_to_addr(new_pc as usize, BranchType::ERET);
        }

        // CheckExceptionCatch(false);              // Check for debug event on exception return
    }
}
