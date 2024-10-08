use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate};
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn is_trivial_pacm_implementation(&mut self) -> bool {
        self.is_feat_impl(Feat::PACIMP)
            && self.cpu.impdef.bool(&"Trivial PSTATE.PACM implementation")
    }
    pub fn is_pacm_enabled(&mut self) -> bool {
        if self.is_trivial_pacm_implementation() {
            return false;
        }

        let enabled;

        // EL2 could force the behavior at EL1 and EL0 to NOP.
        if matches!(self.curr_el(), EL::EL0 | EL::EL1) && self.el2_enabled() && !self.is_in_host() {
            enabled = self.is_hcrx_el2_enabled() && self.read::<hcrx_el2::PACMEn>() != 0;
        } else {
            enabled = true;
        }

        // Otherwise, the SCTLR2_ELx bit determines the behavior.
        if enabled {
            let enpacm_bit;
            match self.curr_el() {
                EL::EL3 => {
                    enpacm_bit = self.read::<sctlr2_el3::EnPACM>() != 0;
                }
                EL::EL2 => {
                    enpacm_bit = if self.is_sctlr2_el2_enabled() {
                        self.read::<sctlr2_el2::EnPACM>() != 0
                    } else {
                        false
                    };
                }
                EL::EL1 => {
                    enpacm_bit = if self.is_sctlr2_el1_enabled() {
                        self.read::<sctlr2_el1::EnPACM>() != 0
                    } else {
                        false
                    };
                }
                EL::EL0 => {
                    if self.is_in_host() {
                        enpacm_bit = if self.is_sctlr2_el2_enabled() {
                            self.read::<sctlr2_el2::EnPACM0>() != 0
                        } else {
                            false
                        };
                    } else {
                        enpacm_bit = if self.is_sctlr2_el1_enabled() {
                            self.read::<sctlr2_el1::EnPACM0>() != 0
                        } else {
                            false
                        };
                    }
                }
            }
            return enpacm_bit;
        }

        return enabled;
    }
}
