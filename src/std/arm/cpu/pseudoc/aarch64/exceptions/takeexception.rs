use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate};
use crate::std::arm::cpu::ArmCtx;

use bitvec::prelude::*;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_take_exception(
        &mut self,
        target_el: EL,
        except: ExceptionRecord,
        pref_excp_ret: u64,
        vect_offset: u64,
    ) {
        // if Halted() then
        //     AArch64.TakeExceptionInDebugState(target_el, exception_in);
        //     return;
        let mut sync_errors = false;
        if self.is_feat_impl(Feat::IESB) {
            sync_errors = match target_el {
                EL::EL0 => panic!("Unreachable"),
                EL::EL1 => self.read::<sctlr_el1::IESB>() != 0,
                EL::EL2 => self.read::<sctlr_el2::IESB>() != 0,
                EL::EL3 => self.read::<sctlr_el3::IESB>() != 0,
            };
            if self.is_feat_impl(Feat::DoubleFault) {
                sync_errors = sync_errors
                    || ((self.read::<scr_el3::EA>() != 0 && self.read::<scr_el3::NMEA>() != 0)
                        && target_el == EL::EL3);
            }
            if sync_errors
                && (self.is_feat_impl(Feat::IESB)
                    && self
                        .cpu
                        .impdef
                        .bool(&"Has Implicit Error Synchronization Barrier before Exception"))
            {
                // self.SynchronizeErrors();
                sync_errors = false;
                self.take_unmasked_physical_serror_interrupts(false);
            }
        }
        let _sync_errors = sync_errors;

        if self.is_feat_impl(Feat::TME) {
            // TODO!
            //     && TSTATE.depth > 0 then
            // case except.exceptype of
            //     when Exception_SoftwareBreakpoint cause = TMFailure_DBG;
            //     when Exception_Breakpoint         cause = TMFailure_DBG;
            //     when Exception_Watchpoint         cause = TMFailure_DBG;
            //     when Exception_SoftwareStep       cause = TMFailure_DBG;
            //     otherwise                         cause = TMFailure_ERR;
            // FailTransaction(cause, FALSE);
        }

        let (_brbe_source_allowed, _brbe_source_address) = if self.is_feat_impl(Feat::BRBE) {
            let el = self.curr_el();
            (self.branch_record_allowed(el), pref_excp_ret)
        } else {
            (false, 0)
        };

        if !self.is_feat_impl(Feat::ExS)
            || match target_el {
                EL::EL0 => panic!("Unreachable"),
                EL::EL1 => self.read::<sctlr_el1::EIS>() != 0,
                EL::EL2 => self.read::<sctlr_el2::EIS>() != 0,
                EL::EL3 => self.read::<sctlr_el3::EIS>() != 0,
            }
        {
            // self.SynchronizeContext();
        }

        let from_32 = self.using_aarch32();
        if from_32 && self.is_feat_impl(Feat::SME) && self.read::<pstate::SM>() != 0 {
            self.reset_sve_state();
        }

        let vect_offset = if target_el.as_num() > self.curr_el().as_num() {
            let lower_32;
            if target_el == EL::EL3 {
                if self.el2_enabled() {
                    lower_32 = self.el_using_aarch32(EL::EL2);
                } else {
                    lower_32 = self.el_using_aarch32(EL::EL1);
                }
            } else if self.is_in_host() && self.curr_el() == EL::EL0 && target_el == EL::EL2 {
                lower_32 = self.el_using_aarch32(EL::EL0);
            } else {
                lower_32 = self.el_using_aarch32(target_el.next_lower());
            }
            vect_offset + (if lower_32 { 0x600 } else { 0x400 })
        } else if self.read::<pstate::SP>() != 0 {
            vect_offset + 0x200
        } else {
            vect_offset
        };

        let mut spsr = self.get_psr_from_pstate(ExceptionTargetState::AArch64);

        if self.curr_el() == EL::EL1 && target_el == EL::EL1 && self.el2_enabled() {
            if matches!(self.effective_hcr_el2_nvx(), 0b001 | 0b101 | 0b111) {
                spsr |= 1 << 3;
                spsr &= !(1 << 2);
            }
        }

        if self.is_feat_impl(Feat::BTI) && self.using_aarch32() {
            // BTYPE is only guaranteed valid for these exception types, otherwise we zero it in the SPSR
            // we are taking from. We define no options for ConstrainedUnpredictable behavior
            if !matches!(
                except.exceptype,
                Exception::SError
                    | Exception::IRQ
                    | Exception::FIQ
                    | Exception::SoftwareStep
                    | Exception::PCAlignment
                    | Exception::InstructionAbort
                    | Exception::Breakpoint
                    | Exception::VectorCatch
                    | Exception::SoftwareBreakpoint
                    | Exception::IllegalState
                    | Exception::BranchTarget
            ) {
                spsr &= !(1 << 11);
                spsr &= !(1 << 10);
            }
        }

        let mut except = except;
        if self.is_feat_impl(Feat::NV2)
            && except.exceptype == Exception::NV2DataAbort
            && target_el == EL::EL3
        {
            // External aborts are configured to be taken to EL3
            except.exceptype = Exception::DataAbort;
        }
        let except = except;

        if !(matches!(except.exceptype, Exception::IRQ | Exception::FIQ)) {
            self.aarch64_report_exception(&except, target_el);
        }

        if self.is_feat_impl(Feat::BRBE) {
            // TODO
            // bits(64) brbe_target_address = VBAR_EL[target_el]<63:11>:vect_offset<10:0>;
            // BRBEException(except, brbe_source_allowed, brbe_source_address,
            //               brbe_target_address, target_el,
            //               except.trappedsyscallinst);
        }

        if self.is_feat_impl(Feat::GCS) {
            if self.curr_el() == target_el {
                if self.get_current_exlocken() {
                    self.write::<pstate::EXLOCK>(1);
                } else {
                    self.write::<pstate::EXLOCK>(0);
                }
            } else {
                self.write::<pstate::EXLOCK>(0);
            }
        }

        self.set_el(target_el);
        self.write::<pstate::nRW>(0);
        self.write::<pstate::SP>(1);

        match self.curr_el() {
            EL::EL0 => panic!("Unreachable"),
            EL::EL1 => {
                self.write::<SPSR_EL1>(spsr);
                self.write::<ELR_EL1>(pref_excp_ret);
            }
            EL::EL2 => {
                self.write::<SPSR_EL2>(spsr);
                self.write::<ELR_EL2>(pref_excp_ret);
            }
            EL::EL3 => {
                self.write::<SPSR_EL3>(spsr);
                self.write::<ELR_EL3>(pref_excp_ret);
            }
        };

        self.write::<pstate::SS>(0);
        if self.is_feat_impl(Feat::NMI) {
            let allint = !match self.curr_el() {
                EL::EL0 => panic!("Unreachable"),
                EL::EL1 => self.read::<sctlr_el1::SPINTMASK>(),
                EL::EL2 => self.read::<sctlr_el2::SPINTMASK>(),
                EL::EL3 => self.read::<sctlr_el3::SPINTMASK>(),
            };
            self.write::<pstate::ALLINT>(allint);
        }

        self.write::<pstate::D>(1);
        self.write::<pstate::A>(1);
        self.write::<pstate::I>(1);
        self.write::<pstate::F>(1);
        self.write::<pstate::IL>(0);

        if from_32 {
            self.write::<pstate::IT>(0);
            self.write::<pstate::T>(0);
        }
        if self.is_feat_impl(Feat::PAN)
            && (self.curr_el() == EL::EL1
                || (self.curr_el() == EL::EL2 && self.el_is_in_host(EL::EL0)))
        {
            if !match self.curr_el() {
                EL::EL0 | EL::EL3 => panic!("Unreachable"),
                EL::EL1 => self.read::<sctlr_el1::SPAN>() != 0,
                EL::EL2 => self.read::<sctlr_el2::SPAN>() != 0,
            } {
                self.write::<pstate::PAN>(1);
            }
        }

        if self.is_feat_impl(Feat::UAO) {
            self.write::<pstate::UAO>(0);
        }
        if self.is_feat_impl(Feat::BTI) {
            self.set_btype(BType::Zero);
        }
        if self.is_feat_impl(Feat::SSBS) {
            let ssbs = match self.curr_el() {
                EL::EL0 => panic!("Unreachable"),
                EL::EL1 => self.read::<sctlr_el1::DSSBS>(),
                EL::EL2 => self.read::<sctlr_el2::DSSBS>(),
                EL::EL3 => self.read::<sctlr_el3::DSSBS>(),
            };
            self.write::<pstate::SSBS>(ssbs);
        }
        if self.is_feat_impl(Feat::MTE) {
            self.write::<pstate::TCO>(1);
        }
        if self.is_feat_impl(Feat::PAuthLR) {
            self.write::<pstate::PACM>(0);
        }
        if self.is_feat_impl(Feat::EBEP) {
            self.write::<pstate::PM>(1);
        }
        if self.is_feat_impl(Feat::SEBEP) {
            self.write::<pstate::PPEND>(0);
            // ShouldSetPPEND = FALSE;
        }

        // boolean branch_conditional = FALSE;
        // BranchTo(VBAR_ELx[]<63:11>:vect_offset<10:0>, BranchType_EXCEPTION, branch_conditional);
        let vbar = match self.curr_el() {
            EL::EL0 => panic!("Unreachable"),
            EL::EL1 => self.read::<VBAR_EL1>(),
            EL::EL2 => self.read::<VBAR_EL2>(),
            EL::EL3 => self.read::<VBAR_EL3>(),
        };

        let mut target = bitarr!(u64, Lsb0; 0; 64);
        (&mut target[0..11]).copy_from_bitslice(&vect_offset.view_bits::<Lsb0>()[0..11]);
        (&mut target[11..]).copy_from_bitslice(&vbar.view_bits::<Lsb0>()[11..]);
        let target = target.load::<u64>();
        self.branch_to(target as usize, BranchType::EXCEPTION, false);
    }
}
