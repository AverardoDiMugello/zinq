use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    fn is_timer_condition_met<'s>(
        &mut self,
        offset: u64,
        compare_value: u64,
        imask: bool,
        intid: InterruptID,
    ) -> bool {
        let condition_met = (self.physical_count() - offset) as i64 >= compare_value as i64;
        let level = if condition_met && !imask { true } else { false };
        addr_space.set_interrupt_request_level(
            self.intr_parent.as_ref().unwrap().clone(),
            intid,
            level,
        );
        condition_met
    }

    pub fn aarch64_check_timer_conditions(&mut self) {
        use EL::*;

        let offset;
        let mut status;
        let mut imask;
        let ss = self.current_security_state();

        if self.is_feat_impl(Feat::ECV)
            && self.el2_enabled()
            && !self.el_is_in_host(EL0)
            && self.read::<cnthctl_el2::ECV>() != 0
            && self.read::<scr_el3::ECVEn>() != 0
        {
            offset = self.read::<CNTPOFF_EL2>();
        } else {
            offset = 0;
        }

        if self.read::<cntp_ctl_el0::ENABLE>() != 0 {
            imask = self.read::<cntp_ctl_el0::IMASK>() != 0;
            if self.is_feat_impl(Feat::RME)
                && matches!(ss, SecurityState::Root | SecurityState::Realm)
                && self.read::<cnthctl_el2::CNTPMASK>() != 0
            {
                imask = true;
            }
            status = self.is_timer_condition_met(
                offset,
                self.read::<CNTP_CVAL_EL0>(),
                imask,
                InterruptID::CNTP,
                addr_space,
            );
            self.write::<cntp_ctl_el0::ISTATUS, bool>(status);
        }

        if (self.have_el(EL3) || (self.have_el(EL2) && !self.is_feat_impl(Feat::SEL2)))
            && self.read::<cnthp_ctl_el2::ENABLE>() != 0
        {
            status = self.is_timer_condition_met(
                0,
                self.read::<CNTHP_CVAL_EL2>(),
                self.read::<cnthp_ctl_el2::IMASK>() != 0,
                InterruptID::CNTHP,
                addr_space,
            );
            self.write::<cnthp_ctl_el2::ISTATUS, bool>(status);
        }

        if self.have_el(EL2)
            && self.is_feat_impl(Feat::SEL2)
            && self.read::<cnthps_ctl_el2::ENABLE>() != 0
        {
            status = self.is_timer_condition_met(
                0,
                self.read::<CNTHPS_CVAL_EL2>(),
                self.read::<cnthps_ctl_el2::IMASK>() != 0,
                InterruptID::CNTHPS,
                addr_space,
            );
            self.write::<cnthps_ctl_el2::ISTATUS, bool>(status);
        }

        if self.read::<cntps_ctl_el1::ENABLE>() != 0 {
            status = self.is_timer_condition_met(
                offset,
                self.read::<CNTPS_CVAL_EL1>(),
                self.read::<cntps_ctl_el1::IMASK>() != 0,
                InterruptID::CNTPS,
                addr_space,
            );
            self.write::<cntps_ctl_el1::ISTATUS, bool>(status);
        }

        if self.read::<cntv_ctl_el0::ENABLE>() != 0 {
            imask = self.read::<cntv_ctl_el0::IMASK>() != 0;
            if self.is_feat_impl(Feat::RME)
                && matches!(ss, SecurityState::Root | SecurityState::Realm)
                && self.read::<cnthctl_el2::CNTVMASK>() != 0
            {
                imask = true;
            }
            status = self.is_timer_condition_met(
                self.read::<CNTVOFF_EL2>(),
                self.read::<CNTV_CVAL_EL0>(),
                imask,
                InterruptID::CNTV,
                addr_space,
            );
            self.write::<cntv_ctl_el0::ISTATUS, bool>(status);
        }

        if (self.is_feat_impl(Feat::VHE) && (self.have_el(EL3) || !self.is_feat_impl(Feat::SEL2)))
            && self.read::<cnthv_ctl_el2::ENABLE>() != 0
        {
            status = self.is_timer_condition_met(
                0,
                self.read::<CNTHV_CVAL_EL2>(),
                self.read::<cnthv_ctl_el2::IMASK>() != 0,
                InterruptID::CNTHV,
                addr_space,
            );
            self.write::<cnthv_ctl_el2::ISTATUS, bool>(status);
        }

        if (self.is_feat_impl(Feat::SEL2) && self.is_feat_impl(Feat::VHE))
            && self.read::<cnthvs_ctl_el2::ENABLE>() != 0
        {
            status = self.is_timer_condition_met(
                0,
                self.read::<CNTHVS_CVAL_EL2>(),
                self.read::<cnthvs_ctl_el2::IMASK>() != 0,
                InterruptID::CNTHVS,
                addr_space,
            );
            self.write::<cnthvs_ctl_el2::ISTATUS, bool>(status);
        }
    }
}
