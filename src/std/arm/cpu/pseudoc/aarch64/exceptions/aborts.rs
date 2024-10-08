use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TCFType {
    Sync,
    Async,
    Ignore,
}

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_effective_tcf(&mut self, el: EL, read: bool) -> TCFType {
        let tcf;
        let regime = self.translation_regime(el);
        match regime {
            Regime::EL3 => tcf = self.read::<sctlr_el3::TCF>(),
            Regime::EL2 => tcf = self.read::<sctlr_el2::TCF>(),
            Regime::EL20 => {
                tcf = if el == EL::EL0 {
                    self.read::<sctlr_el2::TCF0>()
                } else {
                    self.read::<sctlr_el2::TCF>()
                }
            }
            Regime::EL10 => {
                tcf = if el == EL::EL0 {
                    self.read::<sctlr_el1::TCF0>()
                } else {
                    self.read::<sctlr_el1::TCF>()
                }
            }
            _ => panic!("Unreachable"),
        };

        match tcf {
            0b00 => return TCFType::Ignore,
            0b01 => return TCFType::Sync,
            0b10 => {
                if self.is_feat_impl(Feat::MteAsync) {
                    return TCFType::Async;
                } else {
                    return TCFType::Ignore;
                }
            }
            0b11 => {
                if self.is_feat_impl(Feat::MteAsymFault) {
                    // Tag Check Faults cause a synchronous exception on reads or on
                    // a read/write access, and are asynchronously accumulated on writes
                    if read {
                        return TCFType::Sync;
                    } else {
                        return TCFType::Async;
                    }
                } else {
                    // Otherwise, Tag Check Faults have no effect on the PE
                    return TCFType::Ignore;
                }
            }
            _ => panic!("Unreachable"),
        }
    }
}
