use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate, PC};
use crate::std::arm::cpu::{ArmCtx, Feat};

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_call_hypervisor(&mut self, immediate: u16) {
        assert!(self.have_el(EL::EL2));

        if self.using_aarch32() {
            todo!("AArch32.ITAdvance()");
        }
        // SSAdvance();
        let pref_excp_ret = self.read::<PC>() + 4; // NextInstrAddr(64);
        let vect_offset = 0x0;

        let mut except = ExceptionRecord::exception_syndrome(Exception::HypervisorCall);
        except.syndrome |= immediate as u64;

        if self.is_feat_impl(Feat::PAuthLR) {
            self.write::<pstate::PACM>(0);
        }
        if self.curr_el() == EL::EL3 {
            self.aarch64_take_exception(EL::EL3, except, pref_excp_ret, vect_offset);
        } else {
            self.aarch64_take_exception(EL::EL2, except, pref_excp_ret, vect_offset);
        }
    }

    pub fn aarch64_call_secure_monitor(&mut self, immediate: u16) {
        assert!(self.have_el(EL::EL3));
        assert!(!self.el_using_aarch32(EL::EL3));

        if !self.using_aarch32() {
            todo!("AArch32.ITAdvance()");
        }
        // TODO: HSAdvance();
        // TODO: SSAdvance();
        let pref_excp_ret = self.read::<PC>() + 4; // NextInstrAddr(64);
        let vect_offset = 0x0;

        let mut except = ExceptionRecord::exception_syndrome(Exception::MonitorCall);
        except.syndrome |= immediate as u64;
        if self.is_feat_impl(Feat::PAuthLR) {
            self.write::<pstate::PACM>(0);
        };
        self.aarch64_take_exception(EL::EL3, except, pref_excp_ret, vect_offset);
    }

    pub fn aarch64_call_supervisor(&mut self, immediate: u16) {
        if self.using_aarch32() {
            todo!("AArch32.ITAdvance()");
        }
        // TODO: SSAdvance();
        let route_to_el2 =
            self.curr_el() == EL::EL0 && self.el2_enabled() && self.read::<hcr_el2::TGE>() != 0;
        let pref_excp_ret = self.read::<PC>() + 4; // NextInstrAddr(64);
        let vect_offset = 0;

        let mut except = ExceptionRecord::exception_syndrome(Exception::SupervisorCall);
        except.syndrome |= immediate as u64;
        if self.is_feat_impl(Feat::PAuthLR) {
            self.write::<pstate::PACM>(0);
        }
        let curr_el = self.curr_el();
        if curr_el.as_num() > EL::EL1.as_num() {
            self.aarch64_take_exception(curr_el, except, pref_excp_ret, vect_offset);
        } else if route_to_el2 {
            self.aarch64_take_exception(EL::EL2, except, pref_excp_ret, vect_offset);
        } else {
            self.aarch64_take_exception(EL::EL1, except, pref_excp_ret, vect_offset);
        }
    }
}
