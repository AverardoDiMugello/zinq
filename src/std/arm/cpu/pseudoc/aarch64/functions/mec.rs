use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_s1_amec_fault(
        &mut self,
        walkparams: &S1TTWParams,
        paspace: PASpace,
        regime: Regime,
        descriptor: u128,
    ) -> bool {
        let descriptor_amec = if walkparams.d128 {
            (descriptor >> 108) & 1 != 0
        } else {
            (descriptor >> 63) & 1 != 0
        };

        return (walkparams.emec && !walkparams.amec)
            && matches!(regime, Regime::EL2 | Regime::EL20)
            && paspace == PASpace::Realm
            && descriptor_amec;
    }

    pub fn aarch64_s1_disabled_output_mecid(
        &mut self,
        walkparams: &S1TTWParams,
        regime: Regime,
        paspace: PASpace,
    ) -> u64 {
        if !walkparams.emec {
            return 0;
        }

        if !(regime == Regime::EL2 || regime == Regime::EL20 || regime == Regime::EL10) {
            return 0;
        }

        if paspace != PASpace::Realm {
            return 0;
        }

        if regime == Regime::EL10 {
            self.read::<vmecid_p_el2::MECID>()
        } else {
            self.read::<mecid_p0_el2::MECID>()
        }
    }

    pub fn s1_output_mecid(
        &mut self,
        walkparams: &S1TTWParams,
        regime: Regime,
        varange: VARange,
        paspace: PASpace,
        descriptor: u128,
    ) -> u64 {
        if !walkparams.emec {
            return 0;
        }

        if paspace != PASpace::Realm {
            return 0;
        }

        let descriptor_amec = if walkparams.d128 {
            (descriptor >> 108) & 1 != 0
        } else {
            (descriptor >> 63) & 1 != 0
        };

        match regime {
            Regime::EL3 => self.read::<mecid_rl_a_el3::MECID>(),
            Regime::EL2 => {
                if !descriptor_amec {
                    self.read::<mecid_p0_el2::MECID>()
                } else {
                    self.read::<mecid_a0_el2::MECID>()
                }
            }
            Regime::EL20 => {
                if varange == VARange::Lower {
                    if !descriptor_amec {
                        self.read::<mecid_p0_el2::MECID>()
                    } else {
                        self.read::<mecid_a0_el2::MECID>()
                    }
                } else {
                    if !descriptor_amec {
                        self.read::<mecid_p1_el2::MECID>()
                    } else {
                        self.read::<mecid_a1_el2::MECID>()
                    }
                }
            }
            Regime::EL10 => self.read::<vmecid_p_el2::MECID>(),
            _ => panic!("Unreachable"),
        }
    }

    pub fn aarch64_s2_output_mecid(
        &mut self,
        walkparams: &S2TTWParams,
        paspace: PASpace,
        descriptor: u128,
    ) -> u64 {
        if !walkparams.emec {
            return 0;
        }

        if paspace != PASpace::Realm {
            return 0;
        }

        let descriptor_amec = if walkparams.d128 {
            (descriptor >> 108) & 1 != 0
        } else {
            (descriptor >> 63) & 1 != 0
        };

        if !descriptor_amec {
            return self.read::<vmecid_p_el2::MECID>();
        } else {
            return self.read::<vmecid_a_el2::MECID>();
        }
    }

    pub fn aarch64_tt_walk_mecid(&mut self, emec: bool, regime: Regime, ss: SecurityState) -> u64 {
        if !emec {
            return 0;
        }
        if ss != SecurityState::Realm {
            return 0;
        }
        match regime {
            Regime::EL2 => return self.read::<mecid_p0_el2::MECID>(),
            Regime::EL20 => {
                if self.read::<tcr_el2::A1>() == 0 {
                    return self.read::<mecid_p1_el2::MECID>();
                } else {
                    return self.read::<mecid_p0_el2::MECID>();
                }
            }
            // This applies to stage 1 and stage 2 translation table walks for
            // Realm EL1&0, but the stage 2 translation for a stage 1 walk
            // might later override the MECID according to AMEC configuration.
            Regime::EL10 => return self.read::<vmecid_p_el2::MECID>(),
            _ => panic!("Unreachable"),
        }
    }
}
