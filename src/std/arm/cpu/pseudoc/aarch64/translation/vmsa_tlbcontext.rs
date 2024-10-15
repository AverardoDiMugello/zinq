use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_get_s1_tlbcontext(
        &mut self,
        regime: Regime,
        ss: SecurityState,
        va: u64,
        tg: TGx,
    ) -> TLBContext {
        let mut tlbcontext;

        match regime {
            Regime::EL3 => tlbcontext = self.aarch64_tlbcontext_el3(ss, va, tg),
            Regime::EL2 => tlbcontext = self.aarch64_tlbcontext_el2(ss, va, tg),
            Regime::EL20 => tlbcontext = self.aarch64_tlbcontext_el20(ss, va, tg),
            Regime::EL10 => tlbcontext = self.aarch64_tlbcontext_el10(ss, va, tg),
            Regime::EL30 => panic!("Unreachable"),
        }

        tlbcontext.includes_s1 = true;
        // The following may be amended for EL1&0 Regime if caching of stage 2 is successful
        tlbcontext.includes_s2 = false;
        // The following may be amended if Granule Protection Check passes
        tlbcontext.includes_gpt = false;
        return tlbcontext;
    }

    pub fn aarch64_tlbcontext_el10(&mut self, ss: SecurityState, va: u64, tg: TGx) -> TLBContext {
        let ss = ss;
        let regime = Regime::EL10;
        let vmid = self.vmid();

        let mut asid;
        if self.is_feat_impl(Feat::ASID2)
            && self.is_tcr2_el1_enabled()
            && self.read::<tcr2_el1::A2>() != 0
        {
            let varange = VARange::aarch64_from_va(va);
            asid = if varange == VARange::Lower {
                self.read::<ttbr0_el1::ASID>() as u16
            } else {
                self.read::<ttbr1_el1::ASID>() as u16
            };
        } else {
            asid = if self.read::<tcr_el1::A1>() == 0 {
                self.read::<ttbr0_el1::ASID>() as u16
            } else {
                self.read::<ttbr1_el1::ASID>() as u16
            };
        }

        if self.read::<tcr_el1::AS>() == 0 {
            // asid<15:8> = Zeros(8);
            asid &= !(0b1111_1111 << 8);
        }
        let tg = tg;
        let ia = va;

        let cnp;
        if self.is_feat_impl(Feat::TTCNP) {
            if VARange::aarch64_from_va(va) == VARange::Lower {
                cnp = self.read::<ttbr0_el1::CnP>() != 0;
            } else {
                cnp = self.read::<ttbr1_el1::CnP>() != 0;
            }
        } else {
            cnp = false;
        }

        return TLBContext {
            ss,
            regime,
            // vmid,
            asid,
            tg,
            ia,
            cnp,
            ..Default::default()
        };
    }

    pub fn aarch64_tlbcontext_el2(&mut self, ss: SecurityState, va: u64, tg: TGx) -> TLBContext {
        return TLBContext {
            ss: ss,
            regime: Regime::EL2,
            tg: tg,
            ia: va,
            cnp: if self.is_feat_impl(Feat::TTCNP) {
                self.read::<ttbr0_el2::CnP>() != 0
            } else {
                false
            },
            ..Default::default()
        };
    }

    pub fn aarch64_tlbcontext_el20(&mut self, ss: SecurityState, va: u64, tg: TGx) -> TLBContext {
        let ss = ss;
        let regime = Regime::EL20;

        let mut asid;
        if self.is_feat_impl(Feat::ASID2)
            && self.is_tcr2_el2_enabled()
            && self.read::<tcr2_el2::A2>() == 0b1
        {
            let varange = VARange::aarch64_from_va(va);
            asid = if varange == VARange::Lower {
                self.read::<ttbr0_el2::ASID>() as u16
            } else {
                self.read::<ttbr1_el2::ASID>() as u16
            };
        } else {
            asid = if self.read::<tcr_el2::A1>() == 0 {
                self.read::<ttbr0_el2::ASID>() as u16
            } else {
                self.read::<ttbr1_el2::ASID>() as u16
            };
        }
        if self.read::<tcr_el2::AS>() == 0 {
            // tlbcontext.asid<15:8> = Zeros(8);
            asid &= !(0b1111_1111 << 8);
        }
        let tg = tg;
        let ia = va;

        let cnp;
        if self.is_feat_impl(Feat::TTCNP) {
            if VARange::aarch64_from_va(va) == VARange::Lower {
                cnp = self.read::<ttbr0_el2::CnP>() != 0;
            } else {
                cnp = self.read::<ttbr1_el2::CnP>() != 0;
            }
        } else {
            cnp = false;
        }

        return TLBContext {
            ss,
            regime,
            asid,
            tg,
            ia,
            cnp,
            ..Default::default()
        };
    }

    pub fn aarch64_tlbcontext_el3(&mut self, ss: SecurityState, va: u64, tg: TGx) -> TLBContext {
        return TLBContext {
            ss: ss,
            regime: Regime::EL3,
            tg: tg,
            ia: va,
            cnp: if self.is_feat_impl(Feat::TTCNP) {
                self.read::<ttbr0_el3::CnP>() != 0
            } else {
                false
            },
            ..Default::default()
        };
    }
}
