use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate, PC};
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_take_physical_fiq_exception(&mut self) {
        let route_to_el3 = self.have_el(EL::EL3) && self.read::<scr_el3::FIQ>() != 0;
        let route_to_el2 = (matches!(self.curr_el(), EL::EL0 | EL::EL1)
            && self.el2_enabled()
            && (self.read::<hcr_el2::TGE>() != 0 || self.read::<hcr_el2::FMO>() != 0));
        let pref_excp_ret = self.read::<PC>();
        let vect_offset = 0x100;
        let except = ExceptionRecord::exception_syndrome(Exception::FIQ);

        if route_to_el3 {
            self.aarch64_take_exception(EL::EL3, except, pref_excp_ret, vect_offset);
        } else if self.curr_el() == EL::EL2 || route_to_el2 {
            self.aarch64_take_exception(EL::EL2, except, pref_excp_ret, vect_offset);
        } else {
            self.aarch64_take_exception(EL::EL1, except, pref_excp_ret, vect_offset);
        }
    }

    pub fn aarch64_take_physical_irq_exception(&mut self) {
        let route_to_el3 = self.have_el(EL::EL3) && self.read::<scr_el3::IRQ>() != 0;
        let route_to_el2 = (matches!(self.curr_el(), EL::EL0 | EL::EL1)
            && self.el2_enabled()
            && (self.read::<hcr_el2::TGE>() != 0 || self.read::<hcr_el2::IMO>() != 0));
        let pref_excp_ret = self.read::<PC>();
        let vect_offset = 0x80;
        let except = ExceptionRecord::exception_syndrome(Exception::IRQ);

        if route_to_el3 {
            self.aarch64_take_exception(EL::EL3, except, pref_excp_ret, vect_offset);
        } else if self.curr_el() == EL::EL2 || route_to_el2 {
            self.aarch64_take_exception(EL::EL2, except, pref_excp_ret, vect_offset);
        } else {
            self.aarch64_take_exception(EL::EL1, except, pref_excp_ret, vect_offset);
        }
    }

    pub fn aarch64_take_physical_serror_exception(&mut self, _implicit_esb: bool) {
        let route_to_el3 = self.have_el(EL::EL3) && self.read::<scr_el3::EA>() != 0;
        let route_to_el2 = matches!(self.curr_el(), EL::EL0 | EL::EL1)
            && self.el2_enabled()
            && (self.read::<hcr_el2::TGE>() != 0
                || (!self.is_in_host() && self.read::<hcr_el2::AMO>() != 0));
        let pref_excp_ret = self.read::<PC>();
        let vect_offset = 0x180;

        let target_el;
        if self.curr_el() == EL::EL3 || route_to_el3 {
            target_el = EL::EL3;
        } else if self.curr_el() == EL::EL2 || route_to_el2 {
            target_el = EL::EL2;
        } else {
            target_el = EL::EL1;
        }

        let except = ExceptionRecord::exception_syndrome(Exception::SError);
        // TODO
        // let syndrome = self.aarch64_physical_serror_syndrome(implicit_esb);
        // if IsSErrorEdgeTriggered() {
        //     ClearPendingPhysicalSError();
        // }
        // except.syndrome = syndrome;
        self.aarch64_take_exception(target_el, except, pref_excp_ret, vect_offset);
    }

    pub fn aarch64_take_virtual_fiq_exception(&mut self) {
        let pref_excp_ret = self.read::<PC>();
        let vect_offset = 0x100;

        let except = ExceptionRecord::exception_syndrome(Exception::FIQ);

        self.aarch64_take_exception(EL::EL1, except, pref_excp_ret, vect_offset);
    }

    pub fn aarch64_take_virtual_irq_exception(&mut self) {
        let pref_excp_ret = self.read::<PC>();
        let vect_offset = 0x80;

        let except = ExceptionRecord::exception_syndrome(Exception::IRQ);

        self.aarch64_take_exception(EL::EL1, except, pref_excp_ret, vect_offset);
    }

    pub fn aarch64_take_virtual_serror_exception(&mut self) {
        let pref_excp_ret = self.read::<PC>();
        let vect_offset = 0x180;
        let except = ExceptionRecord::exception_syndrome(Exception::SError);

        // TODO
        // if self.is_feat_impl(Feat::RAS) {
        //     except.syndrome_24 = VSESR_EL2.IDS;
        //     except.syndrome_23_0 = VSESR_EL2.ISS;
        // } else {
        //     let syndrome = self.cpu.impdef.int(&"Virtual SError syndrome");
        //     let impdef_syndrome = (syndrome >> 24) & 1 != 0;
        //     if impdef_syndrome {
        //         except.syndrome = syndrome;
        //     }
        // }
        // ClearPendingVirtualSError();
        self.aarch64_take_exception(EL::EL1, except, pref_excp_ret, vect_offset);
    }

    pub fn aarch64_report_exception(&mut self, _except: &ExceptionRecord, _target_el: EL) {
        eprintln!("WARN: not impld: AArch64.ReportException");
    }
}
