use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn s1_translation_regime(&mut self, el: EL) -> EL {
        if el != EL::EL0 {
            el
        } else if self.have_el(EL::EL3)
            && self.el_using_aarch32(EL::EL3)
            // && !self.read::<scr::NS>() != 0
            // TODO: external registers
            && !false
        {
            EL::EL3
        } else if self.is_feat_impl(Feat::VHE) && self.el_is_in_host(el) {
            EL::EL2
        } else {
            EL::EL1
        }
    }
}
