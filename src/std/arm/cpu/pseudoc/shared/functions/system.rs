use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::registers::{aarch32::*, aarch64::*, pstate};
use crate::std::arm::cpu::ArmCtx;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EL {
    EL0,
    EL1,
    EL2,
    EL3,
}

impl EL {
    pub fn as_num(&self) -> u64 {
        match self {
            Self::EL0 => 0,
            Self::EL1 => 1,
            Self::EL2 => 2,
            Self::EL3 => 3,
        }
    }

    pub fn from_bits(hi: bool, lo: bool) -> Self {
        match (hi, lo) {
            (false, false) => EL::EL0,
            (false, true) => EL::EL1,
            (true, false) => EL::EL2,
            (true, true) => EL::EL3,
        }
    }

    pub fn next_lower(&self) -> Self {
        match self {
            Self::EL0 => panic!("Unreachable"),
            Self::EL1 => Self::EL1,
            Self::EL2 => Self::EL2,
            Self::EL3 => Self::EL3,
        }
    }
}

#[derive(Clone, Copy)]
pub enum DSBAlias {
    SSBB,
    PSSBB,
    DSB,
}

#[derive(Clone, Copy)]
pub enum ExceptionTargetState {
    AArch32,
    AArch64,
    Debug,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityState {
    NonSecure,
    Root,
    Realm,
    Secure,
}

#[derive(Clone, Copy)]
pub enum BType {
    Zero,
    One,
    Two,
    Three,
}

impl BType {
    pub fn as_num(&self) -> u64 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }

    pub fn from_bits(hi: bool, lo: bool) -> Self {
        match (hi, lo) {
            (false, false) => BType::Zero,
            (false, true) => BType::One,
            (true, false) => BType::Two,
            (true, true) => BType::Three,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ModeBits {
    M32User,
    M32FIQ,
    M32IRQ,
    M32Svc,
    M32Monitor,
    M32Abort,
    M32Hyp,
    M32Undef,
    M32System,
}

impl ModeBits {
    pub fn as_num(&self) -> u8 {
        match self {
            Self::M32User => 0b10000,
            Self::M32FIQ => 0b10001,
            Self::M32IRQ => 0b10010,
            Self::M32Svc => 0b10011,
            Self::M32Monitor => 0b10110,
            Self::M32Abort => 0b10111,
            Self::M32Hyp => 0b11010,
            Self::M32Undef => 0b11011,
            Self::M32System => 0b11111,
        }
    }
}

// TODO: condition holds?

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn have_aarch32(&self) -> bool {
        self.is_feat_impl(Feat::AA32EL0)
    }

    pub fn have_aarch32_el(&self, el: EL) -> bool {
        match el {
            EL::EL0 => self.is_feat_impl(Feat::AA32EL0),
            EL::EL1 => self.is_feat_impl(Feat::AA32EL1),
            EL::EL2 => self.is_feat_impl(Feat::AA32EL2),
            EL::EL3 => self.is_feat_impl(Feat::AA32EL3),
        }
    }

    pub fn have_aarch64(&self) -> bool {
        return self.is_feat_impl(Feat::AA64EL0)
            || self.is_feat_impl(Feat::AA64EL1)
            || self.is_feat_impl(Feat::AA64EL2)
            || self.is_feat_impl(Feat::AA64EL3);
    }

    pub fn have_el(&self, el: EL) -> bool {
        match el {
            EL::EL0 | EL::EL1 => true,
            EL::EL2 => self.is_feat_impl(Feat::AA64EL2) || self.is_feat_impl(Feat::AA32EL2),
            EL::EL3 => self.is_feat_impl(Feat::AA64EL3) || self.is_feat_impl(Feat::AA32EL3),
        }
    }

    pub fn have_el_using_security_state(&mut self, el: EL, secure: bool) -> bool {
        match el {
            EL::EL3 => self.have_el(EL::EL3),
            EL::EL2 => {
                if secure {
                    self.have_el(EL::EL2) && self.is_feat_impl(Feat::SEL2)
                } else {
                    self.have_el(EL::EL2)
                }
            }
            _ => {
                self.have_el(EL::EL3)
                    || (secure == self.cpu.impdef.bool(&"Secure-only implementation"))
            }
        }
    }

    pub fn have_secure_state(&mut self) -> bool {
        if !self.have_el(EL::EL3) {
            return self.secure_only_implementation();
        }
        if self.is_feat_impl(Feat::RME) && !self.is_feat_impl(Feat::SEL2) {
            return false;
        }
        return true;
    }

    pub fn highest_el(&mut self) -> EL {
        if self.have_el(EL::EL3) {
            return EL::EL3;
        } else if self.have_el(EL::EL2) {
            return EL::EL2;
        } else {
            return EL::EL1;
        }
    }

    pub fn current_security_state(&mut self) -> SecurityState {
        let el = self.curr_el();
        self.security_state_at_el(el)
    }

    pub fn security_state_at_el(&mut self, el: EL) -> SecurityState {
        if self.is_feat_impl(Feat::RME) {
            if el == EL::EL3 {
                return SecurityState::Root;
            } else {
                match (
                    self.read::<scr_el3::NSE>() != 0,
                    self.effective_scr_el3_ns(),
                ) {
                    (false, false) => {
                        if self.is_feat_impl(Feat::SEL2) {
                            return SecurityState::Secure;
                        } else {
                            panic!("Unreachable");
                        }
                    }
                    (false, true) => {
                        return SecurityState::NonSecure;
                    }
                    (true, true) => {
                        return SecurityState::Realm;
                    }
                    _ => {
                        panic!("Unreachable");
                    }
                }
            }
        }

        if !self.have_el(EL::EL3) {
            if self.secure_only_implementation() {
                return SecurityState::Secure;
            } else {
                return SecurityState::NonSecure;
            }
        } else if el == EL::EL3 {
            return SecurityState::Secure;
        } else {
            // For EL2 call only when EL2 is enabled in current security state
            if !self.el_using_aarch32(EL::EL3) {
                return if self.read::<scr_el3::NS>() != 0 {
                    SecurityState::NonSecure
                } else {
                    SecurityState::Secure
                };
            } else {
                return if self.read::<scr::NS>() != 0 {
                    SecurityState::NonSecure
                } else {
                    SecurityState::Secure
                };
            }
        }
    }

    pub fn is_secure_below_el3(&mut self) -> bool {
        if self.have_el(EL::EL3) {
            if !self.have_aarch64() {
                self.read::<scr::NS>() == 0
            } else {
                self.read::<scr_el3::NS>() == 0
            }
        } else if self.have_el(EL::EL2) && (!self.is_feat_impl(Feat::SEL2) || !self.have_aarch64())
        {
            false
        } else {
            self.cpu.impdef.bool("Secure-only implementation")
        }
    }

    pub fn secure_only_implementation(&mut self) -> bool {
        self.cpu.impdef.bool(&"Secure-only implementation")
    }

    pub fn non_secure_only_implementation(&mut self) -> bool {
        self.cpu.impdef.bool(&"Non-secure only implementation")
    }

    pub fn el2_enabled(&mut self) -> bool {
        return self.have_el(EL::EL2)
            && (!self.have_el(EL::EL3)
                || (if !self.have_aarch64() {
                    self.read::<scr::NS>() != 0
                } else {
                    self.read::<scr_el3::NS>() != 0
                })
                || self.is_secure_el2_enabled());
    }

    pub fn is_secure_el2_enabled(&mut self) -> bool {
        if self.have_el(EL::EL2) && self.is_feat_impl(Feat::SEL2) {
            if self.have_el(EL::EL3) {
                if !self.el_using_aarch32(EL::EL3) && self.read::<scr_el3::EEL2>() != 0 {
                    return true;
                } else {
                    return false;
                };
            } else {
                return self.secure_only_implementation();
            }
        } else {
            return false;
        }
    }

    pub fn el3_sdd_undef(&mut self) -> bool {
        false
        // TODO: external registers
        // if self.halted() && self.read::<edscr::SDD>() != 0 {
        //     return true;
        // } else {
        //     return false;
        // }
    }

    pub fn el3_sdd_undef_priority(&mut self) -> bool {
        return self.el3_sdd_undef() && self.cpu.impdef.bool(&"EL3 trap priority when SDD == '1'");
    }

    pub fn el_from_spsr(&mut self, spsr: u64) -> Option<EL> {
        if (spsr >> 4) & 1 == 0 {
            // AArch64 state
            let el = EL::from_bits((spsr >> 3) & 1 != 0, (spsr >> 2) & 1 != 0);
            let effective_nse_ns = (self.effective_scr_el3_nse(), self.effective_scr_el3_ns());
            if !self.have_aarch64() {
                return None; // No AArch64 support
            } else if !self.have_el(el) {
                return None; // Exception level not implemented
            } else if (spsr >> 1) & 1 != 0 {
                return None; // M<1> must be 0
            } else if el == EL::EL0 && spsr & 1 != 0 {
                return None; // for EL0, M<0> must be 0
            } else if self.is_feat_impl(Feat::RME)
                && el != EL::EL3
                && effective_nse_ns == (true, false)
            {
                return None; // Only EL3 valid in Root state
            } else if el == EL::EL2
                && self.have_el(EL::EL3)
                && !self.is_secure_el2_enabled()
                && self.read::<scr_el3::NS>() == 0
            {
                return None; // Unless Secure EL2 is enabled, EL2 valid only in Non-secure state
            } else {
                return Some(el);
            }
        } else if self.have_aarch32() {
            // AArch32 state
            todo!("ELFromM32");
        } else {
            return None;
        }
    }

    pub fn el_is_in_host(&mut self, el: EL) -> bool {
        if !self.is_feat_impl(Feat::VHE) || self.el_using_aarch32(EL::EL2) {
            return false;
        }
        match el {
            EL::EL3 => false,
            EL::EL2 => self.effective_hcr_el2_e2_h(),
            EL::EL1 => false,
            EL::EL0 => self.effective_hcr_el2_e2_h() && self.read::<hcr_el2::TGE>() != 0,
        }
    }

    pub fn is_in_host(&mut self) -> bool {
        let el = self.curr_el();
        self.el_is_in_host(el)
    }

    pub fn using_aarch32(&mut self) -> bool {
        return self.read::<pstate::nRW>() != 0;
    }

    pub fn el_using_aarch32(&mut self, el: EL) -> bool {
        let secure = self.is_secure_below_el3();
        return self.el_state_using_aarch32(el, secure);
    }

    pub fn el_using_aarch32_k(&mut self, el: EL) -> Option<bool> {
        let secure = self.is_secure_below_el3();
        return self.el_state_using_aarch32_k(el, secure);
    }

    pub fn el_state_using_aarch32(&mut self, el: EL, secure: bool) -> bool {
        let aarch32 = self
            .el_state_using_aarch32_k(el, secure)
            .expect("Must only be called in circumstances where result is valid");
        return aarch32;
    }

    pub fn el_state_using_aarch32_k(&mut self, el: EL, secure: bool) -> Option<bool> {
        if !self.have_aarch32_el(el) {
            // Exception level is using AArch64
            return Some(false);
        } else if secure && el == EL::EL2 {
            // Secure EL2 is using AArch64
            return Some(false);
        } else if !self.have_aarch64() {
            // Highest Exception level, therefore all levels are using AArch32
            return Some(true);
        }

        // Remainder of function deals with the interprocessing cases when highest
        // Exception level is using AArch64.

        if el == EL::EL3 {
            return Some(false);
        }

        if self.have_el(EL::EL3)
            && self.read::<scr_el3::RW>() == 0
            && (!secure || !self.is_feat_impl(Feat::SEL2) || self.read::<scr_el3::EEL2>() == 0)
        {
            // AArch32 below EL3.
            return Some(true);
        }

        if el == EL::EL2 {
            return Some(false);
        }

        if self.have_el(EL::EL2)
            && !self.el_is_in_host(EL::EL0)
            && self.read::<hcr_el2::RW>() == 0
            && (!secure || (self.is_feat_impl(Feat::SEL2) && self.read::<scr_el3::EEL2>() != 0))
        {
            // AArch32 below EL2.
            return Some(true);
        }

        if el == EL::EL1 {
            return Some(false);
        }

        // The execution state of EL0 is only known from PSTATE.nRW when executing at EL0.
        if self.curr_el() == EL::EL0 {
            return Some(self.read::<pstate::nRW>() != 0);
        } else {
            return None;
        }
    }

    pub fn effective_hcr_el2_e2_h(&mut self) -> bool {
        if !self.el2_enabled() {
            return false;
        }

        if !self.is_feat_impl(Feat::VHE) {
            return false;
        }

        if !self.is_feat_impl(Feat::E2H0) {
            return true;
        }

        return self.read::<hcr_el2::E2H>() != 0;
    }

    pub fn effective_hcr_el2_nvx(&mut self) -> u64 {
        if !self.el2_enabled() || !self.is_feat_impl(Feat::NV) {
            return 0;
        }

        let nv1 = if !self.is_feat_impl(Feat::E2H0)
            && self.cpu.impdef.bool(&"HCR_EL2.NV1 is implemented as RAZ")
        {
            false
        } else {
            self.read::<hcr_el2::NV1>() != 0
        };

        if self.read::<hcr_el2::NV>() == 0 {
            return 0;
        }

        // Treat nv1 as int from here out
        let nv1 = if nv1 { 1 } else { 0 };

        if !self.is_feat_impl(Feat::NV2) {
            return (nv1 << 1) | 1;
        }

        let nv2 = self.read::<hcr_el2::NV2>() != 0;
        let nv2 = if !nv2
            && self
                .cpu
                .impdef
                .bool(&"Programming HCR_EL2.<NV,NV2> to '10' behaves as '11'")
        {
            1
        } else {
            if nv2 {
                1
            } else {
                0
            }
        };

        return (nv2 << 2) | (nv1 << 1) | 1;
    }

    pub fn effective_scr_el3_ns(&mut self) -> bool {
        if !self.have_secure_state() {
            return true;
        } else if !self.have_el(EL::EL3) {
            return false;
        } else {
            return self.read::<scr_el3::NS>() != 0;
        }
    }

    pub fn effective_scr_el3_nse(&mut self) -> bool {
        if !self.is_feat_impl(Feat::RME) {
            false
        } else {
            self.read::<scr_el3::NSE>() != 0
        }
    }

    pub fn effective_scr_el3_rw(&mut self) -> bool {
        if !self.have_aarch64() {
            return false;
        }
        if !self.have_aarch32_el(EL::EL2) && !self.have_aarch32_el(EL::EL1) {
            return true;
        }
        if self.have_aarch32_el(EL::EL1) {
            if !self.have_aarch32_el(EL::EL2) && self.read::<scr_el3::NS>() != 0 {
                return true;
            }
            if self.is_feat_impl(Feat::SEL2)
                && self.read::<scr_el3::EEL2>() != 0
                && self.read::<scr_el3::NS>() == 0
            {
                return true;
            }
        }
        return self.read::<scr_el3::RW>() != 0;
    }

    pub fn illegal_exception_return(&mut self, spsr: u64) -> bool {
        let target = self.el_from_spsr(spsr);
        if target.is_none() {
            // println!("target.is_none()");
            return true;
        }
        let target = target.unwrap();

        if target.as_num() > self.curr_el().as_num() {
            // println!("target.as_num() > self.curr_el().as_num()");
            return true;
        }

        let spsr_mode_is_aarch32 = (spsr >> 4) & 1 != 0;
        let target_el_is_aarch32 = self.el_using_aarch32_k(target);
        if let Some(target_el_is_aarch32) = target_el_is_aarch32 {
            if spsr_mode_is_aarch32 != target_el_is_aarch32 {
                // println!("spsr_mode_is_aarch32 != target_el_is_aarch32");
                // println!("{spsr_mode_is_aarch32} != {target_el_is_aarch32}");
                return true;
            }
        }

        if self.using_aarch32() && !spsr_mode_is_aarch32 {
            // println!("self.using_aarch32() && !spsr_mode_is_aarch32");
            return true;
        }

        if self.el2_enabled() && target == EL::EL1 && self.read::<hcr_el2::TGE>() != 0 {
            if !self.is_secure_below_el3() || self.is_secure_el2_enabled() {
                // println!("!self.is_secure_below_el3() || self.is_secure_el2_enabled()");
                return true;
            }
        }

        if self.is_feat_impl(Feat::GCS)
            && self.read::<pstate::EXLOCK>() == 0
            && self.curr_el() == target
            && self.get_current_exlocken()
        {
            // println!("last case");
            return true;
        }

        return false;
    }

    pub fn get_psr_from_pstate(&mut self, _target_el_state: ExceptionTargetState) -> u64 {
        let mut spsr = 0;
        // spsr<31:28> = PSTATE.<N,Z,C,V>;
        if self.read::<pstate::N>() != 0 {
            spsr |= 1 << 31;
        }
        if self.read::<pstate::Z>() != 0 {
            spsr |= 1 << 30;
        }
        if self.read::<pstate::C>() != 0 {
            spsr |= 1 << 29;
        }
        if self.read::<pstate::V>() != 0 {
            spsr |= 1 << 28;
        }
        // if IsFeatureImplemented(FEAT_PAN) then spsr<22> = PSTATE.PAN;
        if self.is_feat_impl(Feat::PAN) {
            if self.read::<pstate::PAN>() != 0 {
                spsr |= 1 << 22;
            }
        }
        // spsr<20>    = PSTATE.IL;
        if self.read::<pstate::IL>() != 0 {
            spsr |= 1 << 20;
        }

        if self.read::<pstate::nRW>() != 0 {
            // AArch32 state
            // if IsFeatureImplemented(FEAT_SEBEP) && targetELState != AArch32_NonDebugState then
            //     spsr<33> = PSTATE.PPEND;
            // spsr<27>    = PSTATE.Q;
            // spsr<26:25> = PSTATE.IT<1:0>;
            // if IsFeatureImplemented(FEAT_SSBS) then spsr<23> = PSTATE.SSBS;
            // if IsFeatureImplemented(FEAT_DIT) then
            //     if targetELState == AArch32_NonDebugState then
            //         spsr<21> = PSTATE.DIT;
            //     else                                        // AArch64_NonDebugState or DebugState
            //         spsr<24> = PSTATE.DIT;
            // if targetELState IN {AArch64_NonDebugState, DebugState} then
            //     spsr<21> = PSTATE.SS;
            // spsr<19:16> = PSTATE.GE;
            // spsr<15:10> = PSTATE.IT<7:2>;
            // spsr<9>     = PSTATE.E;
            // spsr<8:6>   = PSTATE.<A,I,F>;                   // No PSTATE.D in AArch32 state
            // spsr<5>     = PSTATE.T;
            // assert PSTATE.M<4> == PSTATE.nRW;               // bit [4] is the discriminator
            // spsr<4:0>   = PSTATE.M;
            todo!("GetPSRFromPSTATE for AArch32");
        } else {
            // AArch64 state
            // if IsFeatureImplemented(FEAT_PAuth_LR) then spsr<35> = PSTATE.PACM;
            if self.is_feat_impl(Feat::PAuthLR) {
                if self.read::<pstate::PACM>() != 0 {
                    spsr |= 1 << 35;
                }
            }
            // if IsFeatureImplemented(FEAT_GCS) then spsr<34> = PSTATE.EXLOCK;
            if self.is_feat_impl(Feat::GCS) {
                if self.read::<pstate::EXLOCK>() != 0 {
                    spsr |= 1 << 34;
                }
            }
            // if IsFeatureImplemented(FEAT_SEBEP) then spsr<33> = PSTATE.PPEND;
            if self.is_feat_impl(Feat::SEBEP) {
                if self.read::<pstate::PPEND>() != 0 {
                    spsr |= 1 << 33;
                }
            }
            // if IsFeatureImplemented(FEAT_EBEP) then spsr<32> = PSTATE.PM;
            if self.is_feat_impl(Feat::EBEP) {
                if self.read::<pstate::PM>() != 0 {
                    spsr |= 1 << 32;
                }
            }
            // if IsFeatureImplemented(FEAT_MTE) then spsr<25> = PSTATE.TCO;
            if self.is_feat_impl(Feat::MTE) {
                if self.read::<pstate::TCO>() != 0 {
                    spsr |= 1 << 25;
                }
            }
            // if IsFeatureImplemented(FEAT_DIT) then spsr<24> = PSTATE.DIT;
            if self.is_feat_impl(Feat::DIT) {
                if self.read::<pstate::DIT>() != 0 {
                    spsr |= 1 << 24;
                }
            }
            // if IsFeatureImplemented(FEAT_UAO) then spsr<23> = PSTATE.UAO;
            if self.is_feat_impl(Feat::UAO) {
                if self.read::<pstate::UAO>() != 0 {
                    spsr |= 1 << 23;
                }
            }
            // spsr<21>    = PSTATE.SS;
            if self.read::<pstate::SS>() != 0 {
                spsr |= 1 << 21;
            }
            // if IsFeatureImplemented(FEAT_NMI) then spsr<13> = PSTATE.ALLINT;
            if self.is_feat_impl(Feat::NMI) {
                if self.read::<pstate::ALLINT>() != 0 {
                    spsr |= 1 << 13;
                }
            }
            // if IsFeatureImplemented(FEAT_SSBS) then spsr<12> = PSTATE.SSBS;
            if self.is_feat_impl(Feat::SSBS) {
                if self.read::<pstate::SSBS>() != 0 {
                    spsr |= 1 << 12;
                }
            }
            // if IsFeatureImplemented(FEAT_BTI) then spsr<11:10> = PSTATE.BTYPE;
            if self.is_feat_impl(Feat::BTI) {
                spsr |= self.btype().as_num() << 10;
            }
            // spsr<9:6>   = PSTATE.<D,A,I,F>;
            if self.read::<pstate::D>() != 0 {
                spsr |= 1 << 9;
            }
            if self.read::<pstate::A>() != 0 {
                spsr |= 1 << 8;
            }
            if self.read::<pstate::I>() != 0 {
                spsr |= 1 << 7;
            }
            if self.read::<pstate::F>() != 0 {
                spsr |= 1 << 6;
            }
            // spsr<4>     = PSTATE.nRW;
            // nRW == 0
            // spsr<3:2>   = PSTATE.EL;
            spsr |= self.curr_el().as_num() << 2;
            // spsr<0>     = PSTATE.SP;
            if self.read::<pstate::SP>() != 0 {
                spsr |= 1;
            }
        }

        spsr
    }

    pub fn set_pstate_from_psr(&mut self, spsr: u64, illegal_psr_state: bool) {
        let from_aarch64 = !self.using_aarch32();
        // TODO: PSTATE.SS = DebugExceptionReturnSS(spsr);
        // TODO: if IsFeatureImplemented(FEAT_SEBEP) then
        //     assert N == 64;
        //     ExceptionReturnPPEND(ZeroExtend(spsr, 64));
        if illegal_psr_state {
            self.write::<pstate::IL>(1);
            panic!("illegal psr state");
        } else {
            // PSTATE.IL = spsr<20>;
            self.write::<pstate::IL>((spsr >> 20) & 1);

            if (spsr >> 4) & 1 != 0 {
                todo!("AArch32");
                // AArch32 state
                // AArch32.WriteMode(spsr<4:0>);         // Sets PSTATE.EL correctly
                // if self.is_feat_impl(Feat::SSBS) { self.pstate.SSBS = spsr >> 23 & 1 != 0;}
            } else {
                // AArch64 state
                self.write::<pstate::nRW>(0);
                self.set_el(EL::from_bits((spsr >> 3) & 1 != 0, (spsr >> 2) & 1 != 0));
                self.write::<pstate::SP>(spsr & 1);

                if self.is_feat_impl(Feat::BTI) {
                    self.set_btype(BType::from_bits(
                        (spsr >> 11) & 1 != 0,
                        (spsr >> 10) & 1 != 0,
                    ));
                }
                if self.is_feat_impl(Feat::SSBS) {
                    self.write::<pstate::SSBS>((spsr >> 12) & 1);
                }
                if self.is_feat_impl(Feat::UAO) {
                    self.write::<pstate::UAO>((spsr >> 23) & 1);
                }
                if self.is_feat_impl(Feat::DIT) {
                    self.write::<pstate::DIT>((spsr >> 24) & 1);
                }
                if self.is_feat_impl(Feat::MTE) {
                    self.write::<pstate::TCO>((spsr >> 25) & 1);
                }
                if self.is_feat_impl(Feat::GCS) {
                    self.write::<pstate::EXLOCK>((spsr >> 34) & 1);
                }
                if self.is_feat_impl(Feat::PAuthLR) {
                    let is_pacm_enabled = self.is_pacm_enabled();
                    self.write::<pstate::PACM>(if is_pacm_enabled { (spsr >> 35) & 1 } else { 0 });
                }
            }
        }

        // State that is reinstated regardless of illegal exception return
        self.write::<pstate::N>((spsr >> 31) & 1);
        self.write::<pstate::Z>((spsr >> 30) & 1);
        self.write::<pstate::C>((spsr >> 29) & 1);
        self.write::<pstate::V>((spsr >> 28) & 1);

        if self.is_feat_impl(Feat::PAN) {
            self.write::<pstate::PAN>((spsr >> 22) & 1);
        }
        if self.read::<pstate::nRW>() != 0 {
            // AArch32 state
            self.write::<pstate::Q>((spsr >> 27) & 1);
            // TODO: PSTATE.IT        = RestoredITBits(spsr);
            // TODO: ShouldAdvanceIT  = FALSE;
            if self.is_feat_impl(Feat::DIT) {
                let restarting = self.restarting();
                self.write::<pstate::DIT>(if restarting || from_aarch64 {
                    (spsr >> 24) & 1
                } else {
                    (spsr >> 21) & 1
                });
            }
            self.write::<pstate::GE>((spsr >> 16) & ((1 << (19 - 16)) - 1));
            self.write::<pstate::E>((spsr >> 9) & 1);
            // No PSTATE.D in AArch32 state
            self.write::<pstate::A>((spsr >> 8) & 1);
            self.write::<pstate::I>((spsr >> 7) & 1);
            self.write::<pstate::F>((spsr >> 6) & 1);
            self.write::<pstate::T>((spsr >> 5) & 1);
        } else {
            // AArch64 state
            if self.is_feat_impl(Feat::EBEP) {
                self.write::<pstate::PM>((spsr >> 32) & 1);
            }
            if self.is_feat_impl(Feat::NMI) {
                self.write::<pstate::ALLINT>((spsr >> 13) & 1);
            }

            self.write::<pstate::D>((spsr >> 9) & 1);
            self.write::<pstate::A>((spsr >> 8) & 1);
            self.write::<pstate::I>((spsr >> 7) & 1);
            self.write::<pstate::F>((spsr >> 6) & 1);

            // No PSTATE.<Q,IT,GE,E,T> in AArch64 state, right now this means the fields are just ignored.
            // POSSIBLE TODO: turn PSTATE into an enum for AArch32 and AArch64 state
        }
        return;
    }

    pub fn synchronize_errors(&mut self) {
        // println!("SynchronizeErrors()");
    }

    pub fn synchronize_context(&mut self) {
        // println!("SynchronizeContext()");
    }

    pub fn take_unmasked_physical_serror_interrupts(&mut self, _iesb_req: bool) {
        // println!("take_unmasked_physical_serror_interrupts");
    }
}
