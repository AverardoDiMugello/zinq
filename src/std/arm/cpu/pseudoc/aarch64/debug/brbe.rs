use crate::std::arm::cpu::aarch64::brbfcr_el1;
use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn branch_record_allowed(&mut self, el: EL) -> bool {
        if self.el_using_aarch32(el) {
            return false;
        }

        if self.read::<brbfcr_el1::PAUSED>() != 0 {
            return false;
        }

        if el == EL::EL3 && self.is_feat_impl(Feat::BRBEv1p1) {
            return self.read::<mdcr_el3::E3BREC>() != self.read::<mdcr_el3::E3BREW>();
        }

        if self.have_el(EL::EL3)
            && (self.read::<mdcr_el3::SBRBE>() == 0b00
                || (self.current_security_state() == SecurityState::Secure
                    && self.read::<mdcr_el3::SBRBE>() == 0b01))
        {
            return false;
        }

        match el {
            EL::EL3 => {
                return false;
            }
            EL::EL2 => {
                return self.read::<brbcr_el2::E2BRE>() != 0;
            }
            EL::EL1 => {
                return self.read::<brbcr_el1::E1BRE>() != 0;
            }
            EL::EL0 => {
                if self.el2_enabled() && self.read::<hcr_el2::TGE>() != 0 {
                    return self.read::<brbcr_el2::E0HBRE>() != 0;
                } else {
                    return self.read::<brbcr_el1::E0BRE>() != 0;
                }
            }
        }
    }
}
