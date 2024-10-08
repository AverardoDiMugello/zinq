use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate};
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn is_hcrx_el2_enabled(&mut self) -> bool {
        if !self.is_feat_impl(Feat::HCX) {
            return false;
        }
        if self.have_el(EL::EL3) && self.read::<scr_el3::HXEn>() == 0 {
            return false;
        }

        return self.el2_enabled();
    }

    pub fn is_sctlr2_el1_enabled(&mut self) -> bool {
        if !self.is_feat_impl(Feat::SCTLR2) {
            return false;
        }
        if self.have_el(EL::EL3) && self.read::<scr_el3::SCTLR2En>() == 0 {
            return false;
        } else if self.el2_enabled()
            && (!self.is_hcrx_el2_enabled() || self.read::<hcrx_el2::SCTLR2En>() == 0)
        {
            return false;
        } else {
            return true;
        }
    }

    pub fn is_sctlr2_el2_enabled(&mut self) -> bool {
        if !self.is_feat_impl(Feat::SCTLR2) {
            return false;
        }
        if self.have_el(EL::EL3) && self.read::<scr_el3::SCTLR2En>() == 0 {
            return false;
        }

        return self.el2_enabled();
    }

    pub fn is_tcr2_el1_enabled(&mut self) -> bool {
        if !self.is_feat_impl(Feat::TCR2) {
            return false;
        }
        if self.have_el(EL::EL3) && self.read::<scr_el3::TCR2En>() == 0 {
            return false;
        } else if self.el2_enabled()
            && (!self.is_hcrx_el2_enabled() || self.read::<hcrx_el2::TCR2En>() == 0)
        {
            return false;
        } else {
            return true;
        }
    }

    pub fn is_tcr2_el2_enabled(&mut self) -> bool {
        if !self.is_feat_impl(Feat::TCR2) {
            return false;
        }
        if self.have_el(EL::EL3) && self.read::<scr_el3::TCR2En>() == 0 {
            return false;
        }

        return self.el2_enabled();
    }

    pub fn aarch64_check_daif_access(&mut self, this_instr: u32) {
        if self.curr_el() == EL::EL0 {
            if self.is_in_host() || self.read::<sctlr_el1::UMA>() == 0 {
                if self.el2_enabled() && self.read::<hcr_el2::TGE>() != 0 {
                    self.aarch64_system_access_trap(EL::EL2, 0x18, this_instr);
                } else {
                    self.aarch64_system_access_trap(EL::EL1, 0x18, this_instr);
                }
            }
        }
    }
}
