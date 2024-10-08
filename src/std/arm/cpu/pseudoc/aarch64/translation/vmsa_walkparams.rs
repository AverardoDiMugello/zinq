use std::cmp::min;

use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_block_bbm_support_level(&mut self) -> Option<u64> {
        if !self.is_feat_impl(Feat::BBM) {
            return None;
        } else {
            return Some(self.cpu.impdef.int(&"Block BBM support level"));
        }
    }

    pub fn aarch64_get_s1_ttw_params(
        &mut self,
        regime: Regime,
        ss: SecurityState,
        va: u64,
    ) -> S1TTWParams {
        let varange = VARange::aarch64_from_va(va);

        return match regime {
            Regime::EL10 => self.aarch64_s1_ttw_params_el10(varange),
            Regime::EL20 => self.aarch64_s1_ttw_params_el20(ss, varange),
            Regime::EL2 => self.aarch64_s1_ttw_params_el2(ss),
            Regime::EL3 => self.aarch64_s1_ttw_params_el3(),
            Regime::EL30 => panic!("Unreachable"),
        };
    }

    pub fn aarch64_get_s2_ttw_params(
        &mut self,
        ss: SecurityState,
        ipaspace: PASpace,
        s1aarch64: bool,
    ) -> S2TTWParams {
        if ss == SecurityState::NonSecure {
            self.aarch64_nss2_ttw_params(s1aarch64)
        } else if self.is_feat_impl(Feat::SEL2) && ss == SecurityState::Secure {
            self.aarch64_ss2_ttw_params(ipaspace, s1aarch64)
        } else if ss == SecurityState::Realm {
            self.aarch64_rls2_ttw_params(s1aarch64)
        } else {
            panic!("Unreachable");
        }
    }

    pub fn aarch64_have_s1_tg(&mut self, tgx: TGx) -> bool {
        match tgx {
            TGx::TG4KB => return self.cpu.impdef.bool(&"Has 4K Translation Granule"),
            TGx::TG16KB => return self.cpu.impdef.bool(&"Has 16K Translation Granule"),
            TGx::TG64KB => return self.cpu.impdef.bool(&"Has 64K Translation Granule"),
        }
    }

    pub fn aarch64_have_s2_tg(&mut self, tgx: TGx) -> bool {
        if self.is_feat_impl(Feat::GTG) {
            match tgx {
                TGx::TG4KB => return self.cpu.impdef.bool(&"Has Stage 2 4K Translation Granule"),
                TGx::TG16KB => return self.cpu.impdef.bool(&"Has Stage 2 16K Translation Granule"),
                TGx::TG64KB => return self.cpu.impdef.bool(&"Has Stage 2 64K Translation Granule"),
            }
        } else {
            return self.aarch64_have_s1_tg(tgx);
        }
    }

    pub fn aarch64_max_tx_sz(&mut self, tgx: TGx) -> u64 {
        if self.is_feat_impl(Feat::TTST) {
            match tgx {
                TGx::TG4KB => return 48,
                TGx::TG16KB => return 48,
                TGx::TG64KB => return 47,
            }
        }
        return 39;
    }

    pub fn aarch64_nss2_ttw_params(&mut self, _s1aarch64: bool) -> S2TTWParams {
        let vm = (self.read::<hcr_el2::VM>() != 0) | (self.read::<hcr_el2::DC>() != 0);
        let tg0 = self.read::<vtcr_el2::TG0>();
        let tgx = self.aarch64_s2_decode_tg0(tg0);
        let txsz = self.read::<vtcr_el2::T0SZ>();
        let ps = self.read::<vtcr_el2::PS>();
        let irgn = self.read::<vtcr_el2::IRGN0>();
        let orgn = self.read::<vtcr_el2::ORGN0>();
        let sh = self.read::<vtcr_el2::SH0>();
        let ee = self.read::<sctlr_el2::EE>() != 0;
        let d128 = if self.is_feat_impl(Feat::D128) {
            self.read::<vtcr_el2::D128>() != 0
        } else {
            false
        };
        let ha = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<vtcr_el2::HA>() != 0
        } else {
            false
        };
        let hd = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<vtcr_el2::HD>() != 0
        } else {
            false
        };

        return S2TTWParams {
            vm,
            tgx,
            txsz,
            ps,
            irgn,
            orgn,
            sh,
            ee,
            d128,
            ha,
            hd,
            skl: if d128 {
                self.read::<vttbr_el2::SKL>()
            } else {
                0
            },
            sl0: if d128 {
                0
            } else {
                self.read::<vtcr_el2::SL0>()
            },
            ptw: if (self.read::<hcr_el2::TGE>() != 0) == false {
                self.read::<hcr_el2::PTW>() != 0
            } else {
                false
            },
            fwb: if self.is_feat_impl(Feat::S2FWB) {
                self.read::<hcr_el2::FWB>() != 0
            } else {
                false
            },
            ds: if matches!(tgx, TGx::TG4KB | TGx::TG16KB) && self.is_feat_impl(Feat::LPA2) {
                self.read::<vtcr_el2::DS>() != 0
            } else {
                false
            },
            sl2: if tgx == TGx::TG4KB && self.is_feat_impl(Feat::LPA2) {
                self.read::<vtcr_el2::SL2>() != 0 && self.read::<vtcr_el2::DS>() != 0
            } else {
                false
            },
            cmow: if self.is_feat_impl(Feat::CMOW) && self.is_hcrx_el2_enabled() {
                self.read::<hcrx_el2::CMOW>() != 0
            } else {
                false
            },
            s2pie: if d128 {
                true
            } else {
                if self.is_feat_impl(Feat::S2PIE) {
                    self.read::<vtcr_el2::S2PIE>() != 0
                } else {
                    false
                }
            },
            s2pir: if self.is_feat_impl(Feat::S2PIE) {
                self.read::<S2PIR_EL2>()
            } else {
                0
            },
            assuredonly: if self.is_feat_impl(Feat::THE) && !d128 {
                self.read::<vtcr_el2::AssuredOnly>() != 0
            } else {
                false
            },
            tl0: if self.is_feat_impl(Feat::THE) {
                self.read::<vtcr_el2::TL0>() != 0
            } else {
                false
            },
            tl1: if self.is_feat_impl(Feat::THE) {
                self.read::<vtcr_el2::TL1>() != 0
            } else {
                false
            },
            haft: if self.is_feat_impl(Feat::HAFT) && ha {
                self.read::<vtcr_el2::HAFT>() != 0
            } else {
                false
            },
            hdbss: if self.is_feat_impl(Feat::HDBSS)
                && ha
                && hd
                && (!self.have_el(EL::EL3) || self.read::<scr_el3::HDBSSEn>() != 0)
            {
                self.read::<vtcr_el2::HDBSS>() != 0
            } else {
                false
            },
            // Uninit by pseudocode...
            sw: false,
            nsw: false,
            sa: false,
            nsa: false,
            emec: false,
            s: false,
            t0sz: 0,
        };
    }

    pub fn aarch64_pa_max(&mut self) -> u64 {
        self.cpu.impdef.int(&"Maximum Physical Address Size")
    }

    pub fn aarch64_rls2_ttw_params(&mut self, s1aarch64: bool) -> S2TTWParams {
        // Realm stage 2 walk parameters are similar to Non-secure
        let mut walkparams = self.aarch64_nss2_ttw_params(s1aarch64);
        walkparams.emec = if self.is_feat_impl(Feat::MEC) && self.is_sctlr2_el2_enabled() {
            self.read::<sctlr2_el2::EMEC>() != 0
        } else {
            false
        };
        return walkparams;
    }

    pub fn aarch64_s1_dcache_enabled(&mut self, regime: Regime) -> bool {
        match regime {
            Regime::EL30 => panic!("Unreachable"),
            Regime::EL3 => return self.read::<sctlr_el3::C>() != 0,
            Regime::EL2 => return self.read::<sctlr_el2::C>() != 0,
            Regime::EL20 => return self.read::<sctlr_el2::C>() != 0,
            Regime::EL10 => return self.read::<sctlr_el1::C>() != 0,
        }
    }

    pub fn aarch64_s1_decode_tg0(&mut self, tg0: u64) -> TGx {
        let tg0 = if tg0 == 3 {
            self.cpu.impdef.int(&"TG0 encoded granule size")
        } else {
            tg0
        };

        let mut tgx = match tg0 {
            0 => TGx::TG4KB,
            1 => TGx::TG64KB,
            2 => TGx::TG16KB,
            _ => panic!("Unreachable"),
        };

        if !self.aarch64_have_s1_tg(tgx) {
            tgx = match self.cpu.impdef.int(&"TG0 encoded granule size") {
                0 => TGx::TG4KB,
                1 => TGx::TG64KB,
                2 => TGx::TG16KB,
                _ => panic!("Unreachable"),
            };
        }
        return tgx;
    }

    pub fn aarch64_s1_decode_tg1(&mut self, tg1: u64) -> TGx {
        let tg1 = if tg1 == 0 {
            self.cpu.impdef.int(&"TG1 encoded granule size")
        } else {
            tg1
        };

        let mut tgx = match tg1 {
            2 => TGx::TG4KB,
            3 => TGx::TG64KB,
            1 => TGx::TG16KB,
            _ => panic!("Unreachable"),
        };

        if !self.aarch64_have_s1_tg(tgx) {
            tgx = match self.cpu.impdef.int(&"TG1 encoded granule size") {
                2 => TGx::TG4KB,
                3 => TGx::TG64KB,
                1 => TGx::TG16KB,
                _ => panic!("Unreachable"),
            };
        }
        return tgx;
    }

    pub fn aarch64_s1_e0_po_enabled(&mut self, regime: Regime, nv1: bool) -> bool {
        if !self.is_feat_impl(Feat::S1POE) {
            return false;
        }

        match regime {
            Regime::EL10 => {
                self.is_tcr2_el1_enabled() && !nv1 && self.read::<tcr2_el1::E0POE>() != 0
            }
            Regime::EL20 => self.is_tcr2_el2_enabled() && self.read::<tcr2_el2::E0POE>() != 0,
            _ => panic!("Unreachable"),
        }
    }

    pub fn aarch64_s1_epd(&mut self, regime: Regime, va: u64) -> bool {
        let varange = VARange::aarch64_from_va(va);

        match regime {
            Regime::EL20 => {
                return if varange == VARange::Lower {
                    self.read::<tcr_el2::EPD0>() != 0
                } else {
                    self.read::<tcr_el2::EPD1>() != 0
                }
            }
            Regime::EL10 => {
                return if varange == VARange::Lower {
                    self.read::<tcr_el1::EPD0>() != 0
                } else {
                    self.read::<tcr_el1::EPD1>() != 0
                }
            }
            _ => panic!("Unreachable"),
        }
    }

    pub fn aarch64_s1_enabled(&mut self, regime: Regime) -> bool {
        match regime {
            Regime::EL10 => {
                (!self.el2_enabled()
                    || (self.read::<hcr_el2::DC>() == 0 && self.read::<hcr_el2::TGE>() == 0))
                    && self.read::<sctlr_el1::M>() != 0
            }
            Regime::EL20 => self.read::<sctlr_el2::M>() != 0,
            Regime::EL2 => self.read::<sctlr_el2::M>() != 0,
            Regime::EL3 => self.read::<sctlr_el3::M>() != 0,
            Regime::EL30 => panic!("Unreachable"),
        }
    }

    pub fn aarch64_s1_icache_enabled(&mut self, regime: Regime) -> bool {
        match regime {
            Regime::EL3 => self.read::<sctlr_el3::I>() != 0,
            Regime::EL2 => self.read::<sctlr_el2::I>() != 0,
            Regime::EL20 => self.read::<sctlr_el2::I>() != 0,
            Regime::EL10 => self.read::<sctlr_el1::I>() != 0,
            _ => panic!("Unreachable"),
        }
    }

    pub fn aarch64_s1_min_tx_sz(&mut self, regime: Regime, d128: bool, ds: bool, tgx: TGx) -> u64 {
        if self.is_feat_impl(Feat::LVA3) && d128 {
            if regime.has_unprivileged() {
                return 9;
            } else {
                return 8;
            }
        }

        if (self.is_feat_impl(Feat::LVA) && tgx == TGx::TG64KB) || ds {
            return 12;
        }

        return 16;
    }

    pub fn aarch64_s1_po_enabled(&mut self, regime: Regime) -> bool {
        if !self.is_feat_impl(Feat::S1POE) {
            return false;
        }

        match regime {
            Regime::EL10 => self.is_tcr2_el1_enabled() && self.read::<tcr2_el1::POE>() != 0,
            Regime::EL20 => self.is_tcr2_el2_enabled() && self.read::<tcr2_el2::POE>() != 0,
            Regime::EL2 => self.is_tcr2_el2_enabled() && self.read::<tcr2_el2::POE>() != 0,
            Regime::EL3 => self.read::<tcr_el3::POE>() != 0,
            Regime::EL30 => panic!("Unreachable"),
        }
    }

    pub fn aarch64_s1_por(&mut self, regime: Regime) -> u64 {
        match regime {
            Regime::EL3 => self.read::<POR_EL3>(),
            Regime::EL2 => self.read::<POR_EL2>(),
            Regime::EL20 => self.read::<POR_EL2>(),
            Regime::EL10 => self.read::<POR_EL1>(),
            _ => panic!("Unreachable"),
        }
    }

    pub fn aarch64_s1_ttbr(&mut self, regime: Regime, va: u64) -> u128 {
        let varange = VARange::aarch64_from_va(va);
        match regime {
            Regime::EL3 => self.read::<TTBR0_EL3>() as u128,
            Regime::EL2 => self.read::<TTBR0_EL2>() as u128,
            Regime::EL20 => {
                if varange == VARange::Lower {
                    self.read::<TTBR0_EL2>() as u128
                } else {
                    self.read::<TTBR1_EL2>() as u128
                }
            }
            Regime::EL10 => {
                if varange == VARange::Lower {
                    self.read::<TTBR0_EL1>() as u128
                } else {
                    self.read::<TTBR1_EL1>() as u128
                }
            }
            _ => panic!("Unreachable"),
        }
    }

    pub fn aarch64_s1_ttw_params_el10(&mut self, varange: VARange) -> S1TTWParams {
        let d128 = if self.is_feat_impl(Feat::D128) && self.is_tcr2_el1_enabled() {
            self.read::<tcr2_el1::D128>() != 0
        } else {
            false
        };
        let nvs = self.effective_hcr_el2_nvx();
        let nv1 = (nvs >> 1) & 1 != 0;

        let mair2 = if self.is_feat_impl(Feat::AIE) {
            self.read::<MAIR2_EL1>()
        } else {
            0
        };

        let aie = if self.is_feat_impl(Feat::AIE) && self.is_tcr2_el1_enabled() {
            self.read::<tcr2_el1::AIE>() != 0
        } else {
            false
        };

        let pie = if d128 {
            true
        } else {
            if self.is_feat_impl(Feat::S1PIE) && self.is_tcr2_el1_enabled() {
                self.read::<tcr2_el1::PIE>() != 0
            } else {
                false
            }
        };

        let pir = if self.is_feat_impl(Feat::S1PIE) {
            self.read::<PIR_EL1>()
        } else {
            0
        };

        let pire0 = if self.is_feat_impl(Feat::S1PIE) && !nv1 {
            self.read::<PIRE0_EL1>()
        } else {
            0
        };

        let (tgx, txsz, irgn, orgn, sh, tbi, nfd, tbid, e0_pd, hpd, mtx, skl, disch, fng) =
            match varange {
                VARange::Lower => {
                    let tg0 = self.read::<tcr_el1::TG0>();
                    let tgx = self.aarch64_s1_decode_tg0(tg0);
                    let txsz = self.read::<tcr_el1::T0SZ>();
                    let irgn = self.read::<tcr_el1::IRGN0>();
                    let orgn = self.read::<tcr_el1::ORGN0>();
                    let sh = self.read::<tcr_el1::SH0>();
                    let tbi = self.read::<tcr_el1::TBI0>() != 0;

                    let nfd = if self.is_feat_impl(Feat::SVE) || self.is_feat_impl(Feat::TME) {
                        self.read::<tcr_el1::NFD0>() != 0
                    } else {
                        false
                    };

                    let tbid = if self.is_feat_impl(Feat::PAuth) {
                        self.read::<tcr_el1::TBID0>() != 0
                    } else {
                        false
                    };

                    let e0_pd = if self.is_feat_impl(Feat::E0PD) {
                        self.read::<tcr_el1::E0PD0>() != 0
                    } else {
                        false
                    };

                    let mut hpd = if self.is_feat_impl(Feat::HPDS) {
                        self.read::<tcr_el1::HPD0>() != 0
                    } else {
                        false
                    };
                    if !hpd {
                        if aie {
                            hpd = true;
                        }
                        if pie {
                            hpd = true;
                        }
                        if self.aarch64_s1_po_enabled(Regime::EL10)
                            || self.aarch64_s1_e0_po_enabled(Regime::EL10, nv1)
                        {
                            hpd = true;
                        }
                    }

                    let mtx = if self.is_feat_impl(Feat::MTE4) {
                        self.read::<tcr_el1::MTX0>() != 0
                    } else {
                        false
                    };

                    let skl = if d128 {
                        self.read::<ttbr0_el1::SKL>()
                    } else {
                        0b00
                    };

                    let disch = if d128 {
                        self.read::<tcr2_el1::DisCH0>() != 0
                    } else {
                        false
                    };

                    let fng = if self.is_feat_impl(Feat::ASID2) && self.is_tcr2_el1_enabled() {
                        self.read::<tcr2_el1::FNG0>() != 0
                    } else {
                        false
                    };

                    (
                        tgx, txsz, irgn, orgn, sh, tbi, nfd, tbid, e0_pd, hpd, mtx, skl, disch, fng,
                    )
                }
                VARange::Upper => {
                    let tg1 = self.read::<tcr_el1::TG1>();
                    let tgx = self.aarch64_s1_decode_tg1(tg1);
                    let txsz = self.read::<tcr_el1::T1SZ>();
                    let irgn = self.read::<tcr_el1::IRGN1>();
                    let orgn = self.read::<tcr_el1::ORGN1>();
                    let sh = self.read::<tcr_el1::SH1>();
                    let tbi = self.read::<tcr_el1::TBI1>() != 0;

                    let nfd = if self.is_feat_impl(Feat::SVE) || self.is_feat_impl(Feat::TME) {
                        self.read::<tcr_el1::NFD1>() != 0
                    } else {
                        false
                    };

                    let tbid = if self.is_feat_impl(Feat::PAuth) {
                        self.read::<tcr_el1::TBID1>() != 0
                    } else {
                        false
                    };

                    let e0_pd = if self.is_feat_impl(Feat::E0PD) {
                        self.read::<tcr_el1::E0PD1>() != 0
                    } else {
                        false
                    };

                    let mut hpd = if self.is_feat_impl(Feat::HPDS) {
                        self.read::<tcr_el1::HPD1>() != 0
                    } else {
                        false
                    };
                    if !hpd {
                        if aie {
                            hpd = true;
                        }
                        if pie {
                            hpd = true;
                        }
                        if self.aarch64_s1_po_enabled(Regime::EL10)
                            || self.aarch64_s1_e0_po_enabled(Regime::EL10, nv1)
                        {
                            hpd = true;
                        }
                    }

                    let mtx = if self.is_feat_impl(Feat::MTE4) {
                        self.read::<tcr_el1::MTX1>() != 0
                    } else {
                        false
                    };

                    let skl = if d128 {
                        self.read::<ttbr1_el1::SKL>()
                    } else {
                        0b00
                    };

                    let disch = if d128 {
                        self.read::<tcr2_el1::DisCH1>() != 0
                    } else {
                        false
                    };

                    let fng = if self.is_feat_impl(Feat::ASID2) && self.is_tcr2_el1_enabled() {
                        self.read::<tcr2_el1::FNG1>() != 0
                    } else {
                        false
                    };

                    (
                        tgx, txsz, irgn, orgn, sh, tbi, nfd, tbid, e0_pd, hpd, mtx, skl, disch, fng,
                    )
                }
            };

        let mair = self.read::<MAIR_EL1>();
        let wxn = self.read::<sctlr_el1::WXN>() != 0;
        let ps = self.read::<tcr_el1::IPS>();
        let ee = self.read::<sctlr_el1::EE>() != 0;
        let sif = if self.have_el(EL::EL3)
            && (!self.is_feat_impl(Feat::RME) && self.is_feat_impl(Feat::SEL2))
        {
            self.read::<scr_el3::SIF>() != 0
        } else {
            false
        };

        let (dc, dct) = if self.el2_enabled() {
            (
                self.read::<hcr_el2::DC>() != 0,
                if self.is_feat_impl(Feat::MTE2) {
                    self.read::<hcr_el2::DCT>() != 0
                } else {
                    false
                },
            )
        } else {
            (false, false)
        };

        let ntlsmd = if self.is_feat_impl(Feat::LSMAOC) {
            self.read::<sctlr_el1::nTLSMD>() != 0
        } else {
            true
        };

        let cmow = if self.is_feat_impl(Feat::CMOW) {
            self.read::<sctlr_el1::CMOW>() != 0
        } else {
            false
        };

        let (ha, hd) = if self.is_feat_impl(Feat::HAFDBS) {
            (
                self.read::<tcr_el1::HA>() != 0,
                self.read::<tcr_el1::HD>() != 0,
            )
        } else {
            (false, false)
        };

        let ds = if (tgx == TGx::TG4KB || tgx == TGx::TG16KB) && self.is_feat_impl(Feat::LPA2) {
            self.read::<tcr_el1::DS>() != 0
        } else {
            false
        };

        let epan = if self.is_feat_impl(Feat::PAN3) {
            if !pie {
                self.read::<sctlr_el1::EPAN>() != 0
            } else {
                true
            }
        } else {
            false
        };

        let pnch = if self.is_feat_impl(Feat::THE) && !d128 && self.is_tcr2_el1_enabled() {
            self.read::<tcr2_el1::PnCH>() != 0
        } else {
            false
        };

        let haft = if self.is_feat_impl(Feat::HAFT) && ha && self.is_tcr2_el1_enabled() {
            self.read::<tcr2_el1::HAFT>() != 0
        } else {
            false
        };

        let emec = if self.is_feat_impl(Feat::MEC) && self.is_sctlr2_el2_enabled() {
            self.read::<sctlr2_el2::EMEC>() != 0
        } else {
            false
        };

        S1TTWParams {
            ha,
            hd,
            tbi,
            tbid,
            nfd,
            e0_pd,
            d128,
            aie,
            mair2,
            ds,
            ps,
            txsz,
            epan,
            dct,
            nv1,
            cmow,
            pnch,
            disch,
            haft,
            mtx,
            skl,
            pie,
            pir,
            pire0,
            emec,
            amec: false, // Untouched by the pseudocode
            fng,
            // A32-VMSA exclusive parameters
            t0sz: 0,
            t1sz: 0,
            uwxn: false,
            tgx,
            irgn,
            orgn,
            sh,
            hpd,
            ee,
            wxn,
            ntlsmd,
            dc,
            sif,
            mair,
        }
    }

    pub fn aarch64_s1_ttw_params_el2(&mut self, ss: SecurityState) -> S1TTWParams {
        let tg0 = self.read::<tcr_el2::TG0>();
        let tgx = self.aarch64_s1_decode_tg0(tg0);
        let txsz = self.read::<tcr_el2::T0SZ>();
        let ps = self.read::<tcr_el2::PS>();
        let irgn = self.read::<tcr_el2::IRGN0>();
        let orgn = self.read::<tcr_el2::ORGN0>();
        let sh = self.read::<tcr_el2::SH0>();
        let tbi = self.read::<tcr_el2::TBI>() != 0;
        let mair = self.read::<MAIR_EL2>();
        let pie = if self.is_feat_impl(Feat::S1PIE) && self.is_tcr2_el2_enabled() {
            self.read::<tcr2_el2::PIE>() != 0
        } else {
            false
        };

        let mut pir = 0;
        if self.is_feat_impl(Feat::S1PIE) {
            pir = self.read::<PIR_EL2>();
        }

        let mut mair2 = 0;
        if self.is_feat_impl(Feat::AIE) {
            mair2 = self.read::<MAIR2_EL2>();
        }
        let aie = if self.is_feat_impl(Feat::AIE) && self.is_tcr2_el2_enabled() {
            self.read::<tcr2_el2::AIE>() != 0
        } else {
            false
        };

        let wxn = self.read::<sctlr_el2::WXN>() != 0;
        let ee = self.read::<sctlr_el2::EE>() != 0;
        let sif;
        if self.have_el(EL::EL3) && (!self.is_feat_impl(Feat::RME) || self.is_feat_impl(Feat::SEL2))
        {
            sif = self.read::<scr_el3::SIF>() != 0;
        } else {
            sif = false;
        }

        let tbid = if self.is_feat_impl(Feat::PAuth) {
            self.read::<tcr_el2::TBI>() != 0
        } else {
            false
        };

        let mut hpd = if self.is_feat_impl(Feat::HPDS) {
            self.read::<tcr_el2::HPD>() != 0
        } else {
            false
        };
        if !hpd {
            if aie {
                hpd = true
            };
            if pie {
                hpd = true
            };
            if self.aarch64_s1_po_enabled(Regime::EL2) {
                hpd = true;
            }
        }

        let ha = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<tcr_el2::HA>() != 0
        } else {
            false
        };

        let hd = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<tcr_el2::HD>() != 0
        } else {
            false
        };

        let ds;
        if (tgx == TGx::TG4KB || tgx == TGx::TG16KB) && self.is_feat_impl(Feat::LPA2) {
            ds = self.read::<tcr_el2::DS>() != 0;
        } else {
            ds = false;
        }
        let mtx = if self.is_feat_impl(Feat::MTE4) {
            self.read::<tcr_el2::MTX>() != 0
        } else {
            false
        };
        let pnch = if self.is_feat_impl(Feat::THE) && self.is_tcr2_el2_enabled() {
            self.read::<tcr2_el2::PnCH>() != 0
        } else {
            false
        };

        let haft;
        if self.is_feat_impl(Feat::HAFT) && ha && self.is_tcr2_el2_enabled() {
            haft = self.read::<tcr2_el2::HAFT>() != 0;
        } else {
            haft = false;
        }

        let emec = if self.is_feat_impl(Feat::MEC) && self.is_tcr2_el2_enabled() {
            self.read::<sctlr2_el2::EMEC>() != 0
        } else {
            false
        };

        let amec;
        if self.is_feat_impl(Feat::MEC) && ss == SecurityState::Realm && self.is_tcr2_el2_enabled()
        {
            amec = self.read::<tcr2_el2::AMEC0>() != 0;
        } else {
            amec = false;
        }

        return S1TTWParams {
            ha,
            hd,
            tbi,
            tbid,
            nfd: false,
            e0_pd: false,
            d128: false,
            aie,
            mair2,
            ds,
            ps,
            txsz,
            epan: false,
            dct: false,
            nv1: false,
            cmow: false,
            pnch,
            disch: false,
            haft,
            mtx,
            skl: 0,
            pie,
            pir,
            pire0: 0,
            emec,
            amec,
            fng: false,
            // A32-VMSA exclusive parameters
            t0sz: 0,
            t1sz: 0,
            uwxn: false,
            tgx,
            irgn,
            orgn,
            sh,
            hpd,
            ee,
            wxn,
            ntlsmd: false,
            dc: false,
            sif,
            mair,
        };
    }

    pub fn aarch64_s1_ttw_params_el20(
        &mut self,
        ss: SecurityState,
        varange: VARange,
    ) -> S1TTWParams {
        let d128;
        if self.is_feat_impl(Feat::D128) && self.is_tcr2_el2_enabled() {
            d128 = self.read::<tcr2_el2::D128>() != 0;
        } else {
            d128 = false;
        }
        let pie;
        if d128 {
            pie = true;
        } else {
            pie = if self.is_feat_impl(Feat::S1PIE) && self.is_tcr2_el2_enabled() {
                self.read::<tcr2_el2::PIE>() != 0
            } else {
                false
            };
        }
        let pir;
        let pire0;
        if self.is_feat_impl(Feat::S1PIE) {
            pir = self.read::<PIR_EL2>();
            pire0 = self.read::<PIRE0_EL2>();
        } else {
            pir = 0;
            pire0 = 0;
        }
        let mair2;
        if self.is_feat_impl(Feat::AIE) {
            mair2 = self.read::<MAIR2_EL2>();
        } else {
            mair2 = 0;
        };
        let aie = if self.is_feat_impl(Feat::AIE) && self.is_tcr2_el2_enabled() {
            self.read::<tcr2_el2::AIE>() != 0
        } else {
            false
        };
        let (tgx, txsz, irgn, orgn, sh, tbi, nfd, tbid, e0_pd, hpd, mtx, skl, disch, fng) =
            if varange == VARange::Lower {
                let tg0 = self.read::<tcr_el2::TG0>();
                let tgx = self.aarch64_s1_decode_tg0(tg0);
                let txsz = self.read::<tcr_el2::T0SZ>();
                let irgn = self.read::<tcr_el2::IRGN0>();
                let orgn = self.read::<tcr_el2::ORGN0>();
                let sh = self.read::<tcr_el2::SH0>();
                let tbi = self.read::<tcr_el2::TBI>() != 0;

                let nfd = if self.is_feat_impl(Feat::SVE) || self.is_feat_impl(Feat::TME) {
                    self.read::<tcr_el2::NFD0>() != 0
                } else {
                    false
                };
                let tbid = if self.is_feat_impl(Feat::PAuth) {
                    self.read::<tcr_el2::TBI>() != 0
                } else {
                    false
                };
                let e0_pd = if self.is_feat_impl(Feat::E0PD) {
                    self.read::<tcr_el2::E0PD0>() != 0
                } else {
                    false
                };
                let mut hpd = if self.is_feat_impl(Feat::HPDS) {
                    self.read::<tcr_el2::HPD0>() != 0
                } else {
                    false
                };
                if !hpd {
                    if aie {
                        hpd = true;
                    }
                    if pie {
                        hpd = true;
                    }
                    if self.aarch64_s1_po_enabled(Regime::EL20)
                        || self.aarch64_s1_e0_po_enabled(Regime::EL20, false)
                    {
                        hpd = true;
                    }
                }
                let mtx = if self.is_feat_impl(Feat::MTE4) {
                    self.read::<tcr_el2::MTX0>() != 0
                } else {
                    false
                };
                let skl = if d128 {
                    self.read::<ttbr0_el2::SKL>()
                } else {
                    0b00
                };
                let disch = if d128 {
                    self.read::<tcr2_el2::DisCH0>() != 0
                } else {
                    false
                };
                let fng;
                if self.is_feat_impl(Feat::ASID2) && self.is_tcr2_el2_enabled() {
                    fng = self.read::<tcr2_el2::FNG0>() != 0;
                } else {
                    fng = false;
                }

                (
                    tgx, txsz, irgn, orgn, sh, tbi, nfd, tbid, e0_pd, hpd, mtx, skl, disch, fng,
                )
            } else {
                let tg1 = self.read::<tcr_el2::TG1>();
                let tgx = self.aarch64_s1_decode_tg1(tg1);
                let txsz = self.read::<tcr_el2::T1SZ>();
                let irgn = self.read::<tcr_el2::IRGN1>();
                let orgn = self.read::<tcr_el2::ORGN1>();
                let sh = self.read::<tcr_el2::SH1>();
                let tbi = self.read::<tcr_el2::TBI>() != 0;

                let nfd = if self.is_feat_impl(Feat::SVE) || self.is_feat_impl(Feat::TME) {
                    self.read::<tcr_el2::NFD1>() != 0
                } else {
                    false
                };
                let tbid = if self.is_feat_impl(Feat::PAuth) {
                    self.read::<tcr_el2::TBI>() != 0
                } else {
                    false
                };
                let e0_pd = if self.is_feat_impl(Feat::E0PD) {
                    self.read::<tcr_el2::E0PD1>() != 0
                } else {
                    false
                };
                let mut hpd = if self.is_feat_impl(Feat::HPDS) {
                    self.read::<tcr_el2::HPD1>() != 0
                } else {
                    false
                };
                if !hpd {
                    if aie {
                        hpd = true;
                    }
                    if pie {
                        hpd = true;
                    }
                    if self.aarch64_s1_po_enabled(Regime::EL20)
                        || self.aarch64_s1_e0_po_enabled(Regime::EL20, false)
                    {
                        hpd = true;
                    }
                }
                let mtx = if self.is_feat_impl(Feat::MTE4) {
                    self.read::<tcr_el2::MTX1>() != 0
                } else {
                    false
                };
                let skl = if d128 {
                    self.read::<ttbr1_el2::SKL>()
                } else {
                    0b00
                };
                let disch = if d128 {
                    self.read::<tcr2_el2::DisCH1>() != 0
                } else {
                    false
                };
                let fng;
                if self.is_feat_impl(Feat::ASID2) && self.is_tcr2_el2_enabled() {
                    fng = self.read::<tcr2_el2::FNG1>() != 0;
                } else {
                    fng = false;
                }

                (
                    tgx, txsz, irgn, orgn, sh, tbi, nfd, tbid, e0_pd, hpd, mtx, skl, disch, fng,
                )
            };

        let mair = self.read::<MAIR_EL2>();
        let wxn = self.read::<sctlr_el2::WXN>() != 0;
        let ps = self.read::<tcr_el2::IPS>();
        let ee = self.read::<sctlr_el2::EE>() != 0;
        let sif;
        if self.have_el(EL::EL3) && (!self.is_feat_impl(Feat::RME) || self.is_feat_impl(Feat::SEL2))
        {
            sif = self.read::<scr_el3::SIF>() != 0;
        } else {
            sif = false;
        }
        let ntlsmd;
        if self.is_feat_impl(Feat::LSMAOC) {
            ntlsmd = self.read::<sctlr_el2::nTLSMD>() != 0;
        } else {
            ntlsmd = true;
        }

        let cmow = if self.is_feat_impl(Feat::CMOW) {
            self.read::<sctlr_el2::CMOW>() != 0
        } else {
            false
        };
        let ha = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<tcr_el2::HA>() != 0
        } else {
            false
        };
        let hd = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<tcr_el2::HD>() != 0
        } else {
            false
        };
        let ds = if matches!(tgx, TGx::TG4KB | TGx::TG16KB) && self.is_feat_impl(Feat::LPA2) {
            self.read::<tcr_el2::DS>() != 0
        } else {
            false
        };
        let epan = if self.is_feat_impl(Feat::PAN3) {
            if !pie {
                self.read::<sctlr_el2::EPAN>() != 0
            } else {
                true
            }
        } else {
            false
        };
        let pnch = if self.is_feat_impl(Feat::THE) && !d128 && self.is_tcr2_el2_enabled() {
            self.read::<tcr2_el2::PnCH>() != 0
        } else {
            false
        };
        let haft = if self.is_feat_impl(Feat::HAFT) && ha && self.is_tcr2_el2_enabled() {
            self.read::<tcr2_el2::HAFT>() != 0
        } else {
            false
        };
        let emec = if self.is_feat_impl(Feat::MEC) && self.is_sctlr2_el2_enabled() {
            self.read::<sctlr2_el2::EMEC>() != 0
        } else {
            false
        };
        let amec = if self.is_feat_impl(Feat::MEC)
            && ss == SecurityState::Realm
            && self.is_tcr2_el2_enabled()
        {
            if varange == VARange::Lower {
                self.read::<tcr2_el2::AMEC0>() != 0
            } else {
                self.read::<tcr2_el2::AMEC1>() != 0
            }
        } else {
            false
        };

        return S1TTWParams {
            d128,
            pie,
            pir,
            pire0,
            tgx,
            txsz,
            irgn,
            orgn,
            sh,
            tbi,
            nfd,
            tbid,
            e0_pd,
            hpd,
            mtx,
            skl,
            disch,
            fng,
            t0sz: 0,
            t1sz: 0,
            haft,
            ha,
            hd,
            aie,
            amec,
            mair,
            mair2,
            ds,
            ps,
            epan,
            dct: false,
            nv1: false,
            cmow,
            pnch,
            emec,
            uwxn: false,
            ee,
            wxn,
            ntlsmd,
            dc: false,
            sif,
        };
    }

    pub fn aarch64_s1_ttw_params_el3(&mut self) -> S1TTWParams {
        let tg0 = self.read::<tcr_el3::TG0>();
        let tgx = self.aarch64_s1_decode_tg0(tg0);
        let txsz = self.read::<tcr_el3::T0SZ>();
        let ps = self.read::<tcr_el3::PS>();
        let irgn = self.read::<tcr_el3::IRGN0>();
        let orgn = self.read::<tcr_el3::ORGN0>();
        let sh = self.read::<tcr_el3::SH0>();
        let tbi = self.read::<tcr_el3::TBI>() != 0;
        let mair = self.read::<MAIR_EL3>();
        let d128 = if self.is_feat_impl(Feat::D128) {
            self.read::<tcr_el3::D128>() != 0
        } else {
            false
        };

        let skl = if d128 {
            self.read::<ttbr0_el3::SKL>()
        } else {
            0b00
        };
        let disch = if d128 {
            self.read::<tcr_el3::DisCH0>() != 0
        } else {
            false
        };
        let pie;
        if d128 {
            pie = true;
        } else {
            pie = if self.is_feat_impl(Feat::S1PIE) {
                self.read::<tcr_el3::PIE>() != 0
            } else {
                false
            };
        }
        let mut pir = 0;
        if self.is_feat_impl(Feat::S1PIE) {
            pir = self.read::<PIR_EL3>();
        }
        let mut mair2 = 0;
        if self.is_feat_impl(Feat::AIE) {
            mair2 = self.read::<MAIR2_EL3>();
        }
        let aie = if self.is_feat_impl(Feat::AIE) {
            self.read::<tcr_el3::AIE>() != 0
        } else {
            false
        };
        let wxn = self.read::<sctlr_el3::WXN>() != 0;
        let ee = self.read::<sctlr_el3::EE>() != 0;
        let sif = if !self.is_feat_impl(Feat::RME) || self.is_feat_impl(Feat::SEL2) {
            self.read::<scr_el3::SIF>() != 0
        } else {
            false
        };

        let tbid = if self.is_feat_impl(Feat::PAuth) {
            self.read::<tcr_el3::TBID>() != 0
        } else {
            false
        };
        let mut hpd = if self.is_feat_impl(Feat::HPDS) {
            self.read::<tcr_el3::HPD>() != 0
        } else {
            false
        };

        if !hpd {
            if aie {
                hpd = true;
            };
            if pie {
                hpd = true;
            };
            if self.aarch64_s1_po_enabled(Regime::EL3) {
                hpd = true;
            }
        }

        let ha = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<tcr_el3::HA>() != 0
        } else {
            false
        };

        let hd = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<tcr_el3::HD>() != 0
        } else {
            false
        };

        let ds;
        if (tgx == TGx::TG4KB || tgx == TGx::TG16KB) && self.is_feat_impl(Feat::LPA2) {
            ds = self.read::<tcr_el3::DS>() != 0;
        } else {
            ds = false;
        }

        let mtx = if self.is_feat_impl(Feat::MTE4) {
            self.read::<tcr_el3::MTX>() != 0
        } else {
            false
        };

        let pnch;
        if self.is_feat_impl(Feat::THE) && !d128 {
            pnch = self.read::<tcr_el3::PnCH>() != 0;
        } else {
            pnch = false;
        }

        let haft;
        if self.is_feat_impl(Feat::HAFT) && ha {
            haft = self.read::<tcr_el3::HAFT>() != 0;
        } else {
            haft = false;
        }

        let emec = if self.is_feat_impl(Feat::MEC) {
            self.read::<sctlr2_el3::EMEC>() != 0
        } else {
            false
        };

        return S1TTWParams {
            ha,
            hd,
            tbi,
            tbid,
            nfd: false,   // Untouched by the pseudocode,
            e0_pd: false, // Untouched by the pseudocode,
            d128,
            aie,
            mair2,
            ds,
            ps,
            txsz,
            epan: false, // Untouched by the pseudocode,
            dct: false,  // Untouched by the pseudocode,
            nv1: false,  // Untouched by the pseudocode,
            cmow: false, // Untouched by the pseudocode,
            pnch,
            disch,
            haft,
            mtx,
            skl,
            pie,
            pir,
            pire0: 0, // Untouched by the pseudocode,
            emec,
            amec: false, // Untouched by the pseudocode
            fng: false,  // Untouched by the pseudocode,
            // A32-VMSA exclusive parameters
            t0sz: 0,
            t1sz: 0,
            uwxn: false,
            tgx,
            irgn,
            orgn,
            sh,
            hpd,
            ee,
            wxn,
            ntlsmd: false, // Untouched by the pseudocode,
            dc: false,     // Untouched by the pseudocode,
            sif,
            mair,
        };
    }

    pub fn aarch64_s2_decode_tg0(&mut self, tg0: u64) -> TGx {
        let tg0 = if tg0 == 3 {
            self.cpu.impdef.int(&"TG0 encoded granule size")
        } else {
            tg0
        };

        let mut tgx = match tg0 {
            0 => TGx::TG4KB,
            1 => TGx::TG64KB,
            2 => TGx::TG16KB,
            _ => panic!("Unreachable"),
        };

        if !self.aarch64_have_s2_tg(tgx) {
            match self.cpu.impdef.int(&"TG0 encoded granule size") {
                0 => tgx = TGx::TG4KB,
                1 => tgx = TGx::TG64KB,
                2 => tgx = TGx::TG16KB,
                _ => panic!("Unreachable"),
            }
        }
        return tgx;
    }

    pub fn aarch64_s2_min_tx_sz(&mut self, d128: bool, ds: bool, tgx: TGx, s1aarch64: bool) -> u64 {
        let ips;
        if self.aarch64_pa_max() == 56 {
            if d128 {
                ips = 56;
            } else if tgx == TGx::TG64KB || ds {
                ips = 52;
            } else {
                ips = 48;
            }
        } else if self.aarch64_pa_max() == 52 {
            if tgx == TGx::TG64KB || ds {
                ips = 52;
            } else {
                ips = 48;
            }
        } else {
            ips = self.aarch64_pa_max();
        }

        let min_txsz = 64 - ips;
        if !s1aarch64 {
            return min(min_txsz, 24);
        }
        return min_txsz;
    }

    pub fn aarch64_ss2_ttw_params(&mut self, ipaspace: PASpace, _s1aarch64: bool) -> S2TTWParams {
        let d128 = if self.is_feat_impl(Feat::D128) {
            self.read::<vtcr_el2::D128>() != 0
        } else {
            false
        };

        let (tgx, txsz, skl, sl0, sl2);
        if ipaspace == PASpace::Secure {
            let tg0 = self.read::<vstcr_el2::TG0>();
            tgx = self.aarch64_s2_decode_tg0(tg0);
            txsz = self.read::<vstcr_el2::T0SZ>();
            if d128 {
                skl = self.read::<vsttbr_el2::SKL>();
                sl0 = 0;
            } else {
                sl0 = self.read::<vstcr_el2::SL0>();
                skl = 0;
            }
            if tgx == TGx::TG4KB && self.is_feat_impl(Feat::LPA2) {
                sl2 = self.read::<vstcr_el2::SL2>() != 0 && self.read::<vtcr_el2::DS>() != 0;
            } else {
                sl2 = false;
            }
        } else if ipaspace == PASpace::NonSecure {
            let tg0 = self.read::<vtcr_el2::TG0>();
            tgx = self.aarch64_s2_decode_tg0(tg0);
            txsz = self.read::<vtcr_el2::T0SZ>();
            if d128 {
                skl = self.read::<vttbr_el2::SKL>();
                sl0 = 0;
            } else {
                sl0 = self.read::<vtcr_el2::SL0>();
                skl = 0;
            }
            if tgx == TGx::TG4KB && self.is_feat_impl(Feat::LPA2) {
                sl2 = self.read::<vtcr_el2::SL2>() != 0 && self.read::<vtcr_el2::DS>() != 0;
            } else {
                sl2 = false;
            }
        } else {
            panic!("Unreachable");
        }

        let sw = self.read::<vstcr_el2::SW>() != 0;
        let nsw = self.read::<vtcr_el2::NSW>() != 0;
        let sa = self.read::<vstcr_el2::SA>() != 0;
        let nsa = self.read::<vtcr_el2::NSA>() != 0;
        let vm = (self.read::<hcr_el2::VM>() != 0) | (self.read::<hcr_el2::DC>() != 0);
        let ps = self.read::<vtcr_el2::PS>();
        let irgn = self.read::<vtcr_el2::IRGN0>();
        let orgn = self.read::<vtcr_el2::ORGN0>();
        let sh = self.read::<vtcr_el2::SH0>();
        let ee = self.read::<sctlr_el2::EE>() != 0;

        let ptw = if (self.read::<hcr_el2::TGE>() != 0) == false {
            self.read::<hcr_el2::PTW>() != 0
        } else {
            false
        };

        let fwb = if self.is_feat_impl(Feat::S2FWB) {
            self.read::<hcr_el2::FWB>() != 0
        } else {
            false
        };

        let ha = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<vtcr_el2::HA>() != 0
        } else {
            false
        };

        let hd = if self.is_feat_impl(Feat::HAFDBS) {
            self.read::<vtcr_el2::HD>() != 0
        } else {
            false
        };

        let ds;
        if matches!(tgx, TGx::TG4KB | TGx::TG16KB) && self.is_feat_impl(Feat::LPA2) {
            ds = self.read::<vtcr_el2::DS>() != 0;
        } else {
            ds = false;
        }

        let cmow = if self.is_feat_impl(Feat::CMOW) && self.is_hcrx_el2_enabled() {
            self.read::<hcrx_el2::CMOW>() != 0
        } else {
            false
        };

        let s2pie;
        if d128 {
            s2pie = true;
        } else {
            s2pie = if self.is_feat_impl(Feat::S2PIE) {
                self.read::<vtcr_el2::S2PIE>() != 0
            } else {
                false
            };
        }

        let s2pir = if self.is_feat_impl(Feat::S2PIE) {
            self.read::<S2PIR_EL2>()
        } else {
            0
        };

        let assuredonly;
        if self.is_feat_impl(Feat::THE) && !d128 {
            assuredonly = self.read::<vtcr_el2::AssuredOnly>() != 0;
        } else {
            assuredonly = false;
        }

        let tl0 = if self.is_feat_impl(Feat::THE) {
            self.read::<vtcr_el2::TL0>() != 0
        } else {
            false
        };

        let tl1 = if self.is_feat_impl(Feat::THE) {
            self.read::<vtcr_el2::TL1>() != 0
        } else {
            false
        };

        let haft;
        if self.is_feat_impl(Feat::HAFT) && ha {
            haft = self.read::<vtcr_el2::HAFT>() != 0;
        } else {
            haft = false;
        }

        let emec = false;
        let hdbss;
        if self.is_feat_impl(Feat::HDBSS)
            && ha
            && hd
            && (!self.have_el(EL::EL3) || self.read::<scr_el3::HDBSSEn>() != 0)
        {
            hdbss = self.read::<vtcr_el2::HDBSS>() != 0;
        } else {
            hdbss = false;
        }

        return S2TTWParams {
            sw,
            nsw,
            sa,
            nsa,
            vm,
            ps,
            irgn,
            orgn,
            sh,
            ee,
            ptw,
            fwb,
            ha,
            hd,
            ds,
            cmow,
            s2pie,
            s2pir,
            assuredonly,
            tl0,
            tl1,
            haft,
            emec,
            hdbss,
            tgx,
            txsz,
            skl,
            sl0,
            sl2,
            d128,
            s: false,
            t0sz: 0,
        };
    }

    pub fn s2_dcache_enabled(&mut self) -> bool {
        self.read::<hcr_el2::CD>() == 0
    }
}
