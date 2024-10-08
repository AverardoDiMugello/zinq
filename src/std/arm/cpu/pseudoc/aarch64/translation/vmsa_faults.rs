use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate};
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_addr_top(&mut self, tbid: bool, acctype: AccessType, tbi: bool) -> u64 {
        if tbid && acctype == AccessType::IFETCH {
            return 63;
        }

        if tbi {
            return 55;
        } else {
            return 63;
        }
    }

    pub fn aarch64_contiguous_bit_faults(
        &mut self,
        d128: bool,
        txsz: u64,
        tgx: TGx,
        level: i64,
    ) -> bool {
        // Input Address size
        let iasize = 64 - txsz;
        // Translation size
        let tsize = tgx.translation_size(d128, level) + tgx.contiguous_size(d128, level);

        return tsize > iasize
            && self
                .cpu
                .impdef
                .bool(&"Translation fault on misprogrammed contiguous bit");
    }

    pub fn aarch64_va_is_out_of_range(
        &mut self,
        va: u64,
        acctype: AccessType,
        regime: Regime,
        walkparams: &S1TTWParams,
    ) -> bool {
        let addrtop = self.aarch64_addr_top(walkparams.tbid, acctype, walkparams.tbi);

        // If the VA has a Logical Address Tag then the bits holding the Logical Address Tag are
        // ignored when checking if the address is out of range.
        let va = if walkparams.mtx && acctype != AccessType::IFETCH {
            if VARange::aarch64_from_va(va) == VARange::Upper {
                va | (0b1111 << 56)
            } else {
                va & !(0b1111 << 56)
            }
        } else {
            va
        };

        // Input Address size
        let iasize = 64 - walkparams.txsz;

        if addrtop < iasize {
            return false;
        }

        if regime.has_unprivileged() {
            if VARange::aarch64_from_va(va) == VARange::Lower {
                (va >> iasize) & ((1 << (addrtop - iasize + 1)) - 1) != 0
            } else {
                (va >> iasize) & ((1 << (addrtop - iasize + 1)) - 1)
                    != ((1 << (addrtop - iasize + 1)) - 1)
            }
        } else {
            (va >> iasize) & ((1 << (addrtop - iasize + 1)) - 1) != 0
        }
    }

    pub fn aarch64_ipa_is_out_of_range(&mut self, ipa: u64, walkparams: &S2TTWParams) -> bool {
        let iasize = 64 - walkparams.txsz;

        if iasize < 56 {
            return (ipa >> iasize) & ((1 << (55 - iasize + 1)) - 1) != 0;
        } else {
            return false;
        }
    }

    pub fn aarch64_oa_out_of_range(&mut self, address: u64, d128: bool, ps: u64, tgx: TGx) -> bool {
        // Output Address size
        let oasize = self.aarch64_physical_address_size(d128, ps, tgx);

        if oasize < 56 {
            return (address >> oasize) & ((1 << (55 - oasize + 1)) - 1) != 0;
        } else {
            return false;
        }
    }

    pub fn aarch64_s1_check_permissions(
        &mut self,
        fault: &mut FaultRecord,
        regime: Regime,
        walkstate: &TTWState,
        walkparams: &S1TTWParams,
        accdesc: &AccessDescriptor,
    ) {
        let s1perms = self.aarch64_s1_compute_permissions(regime, walkstate, walkparams, accdesc);
        let permissions = &walkstate.permissions;

        if accdesc.acctype == AccessType::IFETCH {
            if s1perms.overlay
                && s1perms
                    .overlay_perms
                    .as_ref()
                    .is_some_and(|perms| !perms.ox)
            {
                fault.statuscode = Fault::Permission;
                fault.overlay = true;
            } else if matches!(walkstate.memattrs.memtype, MemType::Device(_)) {
                fault.statuscode = Fault::Permission;
            } else if !s1perms.x {
                // fault.statuscode = Fault::Permission;
            }
        } else if accdesc.acctype == AccessType::DC {
            if accdesc.cacheop == CacheOp::Invalidate {
                if s1perms.overlay
                    && s1perms
                        .overlay_perms
                        .as_ref()
                        .is_some_and(|perms| !perms.ow)
                {
                    fault.statuscode = Fault::Permission;
                    fault.overlay = true;
                } else if !s1perms.w {
                    fault.statuscode = Fault::Permission;
                }
                // DC from privileged context which clean cannot generate a Permission fault
                else if accdesc.el == EL::EL0 {
                    if s1perms.overlay
                        && s1perms
                            .overlay_perms
                            .as_ref()
                            .is_some_and(|perms| !perms.or)
                    {
                        fault.statuscode = Fault::Permission;
                        fault.overlay = true;
                    }
                } else if walkparams.cmow
                    && accdesc.opscope == CacheOpScope::PoC
                    && accdesc.cacheop == CacheOp::CleanInvalidate
                    && s1perms.overlay
                    && s1perms
                        .overlay_perms
                        .as_ref()
                        .is_some_and(|perms| !perms.ow)
                {
                    fault.statuscode = Fault::Permission;
                    fault.overlay = true;
                } else if !s1perms.r {
                    fault.statuscode = Fault::Permission;
                } else if walkparams.cmow
                    && accdesc.opscope == CacheOpScope::PoC
                    && accdesc.cacheop == CacheOp::CleanInvalidate
                    && !s1perms.w
                {
                    fault.statuscode = Fault::Permission;
                }
            }
        } else if accdesc.acctype == AccessType::IC {
            // IC from privileged context cannot generate Permission fault
            if accdesc.el == EL::EL0 {
                if s1perms.overlay
                    && s1perms
                        .overlay_perms
                        .as_ref()
                        .is_some_and(|perms| !perms.or)
                    && self
                        .cpu
                        .impdef
                        .bool(&"Permission fault on EL0 IC_IVAU execution")
                {
                    fault.statuscode = Fault::Permission;
                    fault.overlay = true;
                } else if walkparams.cmow
                    && s1perms.overlay
                    && s1perms
                        .overlay_perms
                        .as_ref()
                        .is_some_and(|perms| !perms.ow)
                {
                    fault.statuscode = Fault::Permission;
                    fault.overlay = true;
                } else if !s1perms.r
                    && self
                        .cpu
                        .impdef
                        .bool(&"Permission fault on EL0 IC_IVAU execution")
                {
                    fault.statuscode = Fault::Permission;
                } else if walkparams.cmow && !s1perms.w {
                    fault.statuscode = Fault::Permission;
                }
            }
        } else if self.is_feat_impl(Feat::GCS) && accdesc.acctype == AccessType::GCS {
            if !s1perms.gcs {
                fault.statuscode = Fault::Permission;
            } else if accdesc.write && (!walkparams.ha && !walkparams.hd) && permissions.ndirty {
                fault.statuscode = Fault::Permission;
                fault.dirtybit = true;
                fault.write = Some(true);
            }
        } else if accdesc.read
            && s1perms.overlay
            && s1perms
                .overlay_perms
                .as_ref()
                .is_some_and(|perms| !perms.or)
        {
            fault.statuscode = Fault::Permission;
            fault.overlay = true;
            fault.write = Some(false);
        } else if accdesc.write
            && s1perms.overlay
            && s1perms
                .overlay_perms
                .as_ref()
                .is_some_and(|perms| !perms.ow)
        {
            fault.statuscode = Fault::Permission;
            fault.overlay = true;
            fault.write = Some(true);
        } else if accdesc.read && !s1perms.r {
            fault.statuscode = Fault::Permission;
            fault.write = Some(false);
        } else if accdesc.write && !s1perms.w {
            fault.statuscode = Fault::Permission;
            fault.write = Some(true);
        } else if accdesc.write
            && accdesc.tagaccess
            && walkstate.memattrs.tags == MemTagType::CanonicallyTagged
        {
            fault.statuscode = Fault::Permission;
            fault.write = Some(true);
            fault.s1tagnotdata = true;
        } else if accdesc.write
            && !(walkparams.ha && walkparams.hd)
            && walkparams.pie
            && permissions.ndirty
        {
            fault.statuscode = Fault::Permission;
            fault.dirtybit = true;
            fault.write = Some(true);
        }
    }

    pub fn aarch64_s1_compute_permissions(
        &mut self,
        regime: Regime,
        walkstate: &TTWState,
        walkparams: &S1TTWParams,
        accdesc: &AccessDescriptor,
    ) -> S1AccessControls {
        let mut s1perms;

        if walkparams.pie {
            s1perms =
                self.aarch64_s1_indirect_base_permissions(regime, walkstate, walkparams, accdesc);
        } else {
            s1perms =
                self.aarch64_s1_direct_base_permissions(regime, walkstate, walkparams, accdesc);
        }

        if accdesc.el == EL::EL0 && !self.aarch64_s1_e0_po_enabled(regime, walkparams.nv1) {
            s1perms.overlay = false;
        } else if accdesc.el != EL::EL0 && !self.aarch64_s1_po_enabled(regime) {
            s1perms.overlay = false;
        }

        if s1perms.overlay {
            let s1overlay_perms = self.aarch64_s1_overlay_permissions(regime, walkstate, accdesc);
            s1perms.overlay_perms = Some(s1overlay_perms);
        }

        if s1perms.overlay
            && s1perms.wxn
            && s1perms.overlay_perms.as_ref().is_some_and(|perms| perms.ox)
        {
            s1perms.overlay_perms.as_mut().unwrap().ow = false;
        } else if s1perms.wxn {
            s1perms.x = false;
        }

        return s1perms;
    }

    pub fn aarch64_s1_direct_base_permissions(
        &mut self,
        regime: Regime,
        walkstate: &TTWState,
        walkparams: &S1TTWParams,
        accdesc: &AccessDescriptor,
    ) -> S1AccessControls {
        let permissions = &walkstate.permissions;

        let (r, w, mut x);
        let (mut pr, mut pw, px);
        let (mut ur, mut uw, mut ux);

        if regime.has_unprivileged() {
            // Apply leaf permissions
            match (permissions.ap >> 1) & 0b11 {
                0b00 => (pr, pw, ur, uw) = (true, true, false, false), // Privileged access
                0b01 => (pr, pw, ur, uw) = (true, true, true, true),   // No effect
                0b10 => (pr, pw, ur, uw) = (true, false, false, false), // Read-only, privileged access
                0b11 => (pr, pw, ur, uw) = (true, false, true, false),  // Read-only
                _ => panic!("Unreachable"),
            };

            // Apply hierarchical permissions
            match permissions.ap_table {
                0b00 => (pr, pw, ur, uw) = (pr, pw, ur, uw), // No effect
                0b01 => (pr, pw, ur, uw) = (pr, pw, false, false), // Privileged access
                0b10 => (pr, pw, ur, uw) = (pr, false, ur, false), // Read-only
                0b11 => (pr, pw, ur, uw) = (pr, false, false, false), // Read-only, privileged access
                _ => panic!("Unreachable"),
            };

            // Locations writable by unprivileged cannot be executed by privileged
            px = !(permissions.pxn | permissions.pxn_table | uw);
            ux = !(permissions.uxn | permissions.uxn_table);

            if self.is_feat_impl(Feat::PAN)
                && accdesc.pan
                && !(regime == Regime::EL10 && walkparams.nv1)
            {
                let pan;
                if self.cpu.impdef.bool(&"SCR_EL3.SIF affects EPAN")
                    && accdesc.ss == SecurityState::Secure
                    && walkstate.baseaddress.paspace == PASpace::NonSecure
                    && walkparams.sif
                {
                    ux = false;
                }

                if self.cpu.impdef.bool(&"Realm EL2&0 regime affects EPAN")
                    && accdesc.ss == SecurityState::Realm
                    && regime == Regime::EL20
                    && walkstate.baseaddress.paspace != PASpace::Realm
                {
                    ux = false;
                }

                pan = self.read::<pstate::PAN>() != 0 && (ur | uw | (walkparams.epan && ux));
                pr = pr && !pan;
                pw = pw && !pan;
            }

            (r, w, x) = if accdesc.el == EL::EL0 {
                (ur, uw, ux)
            } else {
                (pr, pw, px)
            };
        } else {
            // Apply leaf permissions
            match (permissions.ap >> 2) & 1 != 0 {
                false => (pr, pw) = (true, true), // No effect
                true => (pr, pw) = (true, false), // Read-only
            }
            // Apply hierarchical permissions
            match (permissions.ap_table >> 1) & 1 != 0 {
                false => (pr, pw) = (pr, pw),   // No effect
                true => (pr, pw) = (pr, false), // Read-only
            }
            px = !(permissions.xn | permissions.xn_table);

            (r, w, x) = (pr, pw, px);
        }

        // Compute WXN value
        let wxn = walkparams.wxn && w && x;

        // Prevent execution from Non-secure space by PE in secure state if SIF is set
        if accdesc.ss == SecurityState::Secure
            && walkstate.baseaddress.paspace == PASpace::NonSecure
        {
            x = x && !(walkparams.sif);
        }
        // Prevent execution from non-Root space by Root
        if accdesc.ss == SecurityState::Root && walkstate.baseaddress.paspace != PASpace::Root {
            x = false;
        }
        // Prevent execution from non-Realm space by Realm EL2 and Realm EL2&0
        if accdesc.ss == SecurityState::Realm
            && matches!(regime, Regime::EL2 | Regime::EL20)
            && walkstate.baseaddress.paspace != PASpace::Realm
        {
            x = false;
        }

        return S1AccessControls {
            r: r,
            w: w,
            x: x,
            gcs: false,
            wxn: wxn,
            overlay: true,
            overlay_perms: None,
        };
    }

    pub fn aarch64_s1_has_alignment_fault(
        &mut self,
        accdesc: &AccessDescriptor,
        aligned: bool,
        ntlsmd: bool,
        memattrs: &MemoryAttributes,
    ) -> bool {
        if accdesc.acctype == AccessType::IFETCH {
            false
        } else if self.is_feat_impl(Feat::MTE) && accdesc.write && accdesc.devstoreunpred {
            matches!(memattrs.memtype, MemType::Device(_))
        } else if accdesc.a32lsmd && !ntlsmd {
            if let MemType::Device(device) = memattrs.memtype {
                return !matches!(device, DeviceType::GRE);
            }
            false
        } else if accdesc.acctype == AccessType::DCZero {
            matches!(memattrs.memtype, MemType::Device(_))
        } else {
            matches!(memattrs.memtype, MemType::Device(_)) && !aligned
        }
    }

    pub fn aarch64_s1_indirect_base_permissions(
        &mut self,
        regime: Regime,
        walkstate: &TTWState,
        walkparams: &S1TTWParams,
        accdesc: &AccessDescriptor,
    ) -> S1AccessControls {
        let (mut r, mut w, mut x, mut gcs, mut wxn, mut overlay);
        let (mut pr, mut pw, mut px, mut pgcs, pwxn, p_overlay);
        let (mut ur, mut uw, mut ux, mut ugcs, uwxn, u_overlay);
        let permissions = &walkstate.permissions;

        // Apply privileged indirect permissions
        match permissions.ppi {
            0b0000 => (pr, pw, px, pgcs) = (false, false, false, false), // No access
            0b0001 => (pr, pw, px, pgcs) = (true, false, false, false),  // Privileged read
            0b0010 => (pr, pw, px, pgcs) = (false, false, true, false),  // Privileged execute
            0b0011 => (pr, pw, px, pgcs) = (true, false, true, false), // Privileged read and execute
            0b0100 => (pr, pw, px, pgcs) = (false, false, false, false), // Reserved
            0b0101 => (pr, pw, px, pgcs) = (true, true, false, false), // Privileged read and write
            0b0110 => (pr, pw, px, pgcs) = (true, true, true, false), // Privileged read, write and execute
            0b0111 => (pr, pw, px, pgcs) = (true, true, true, false), // Privileged read, write and execute
            0b1000 => (pr, pw, px, pgcs) = (true, false, false, false), // Privileged read
            0b1001 => (pr, pw, px, pgcs) = (true, false, false, true), // Privileged read and gcs
            0b1010 => (pr, pw, px, pgcs) = (true, false, true, false), // Privileged read and execute
            0b1011 => (pr, pw, px, pgcs) = (false, false, false, false), // Reserved
            0b1100 => (pr, pw, px, pgcs) = (true, true, false, false), // Privileged read and write
            0b1101 => (pr, pw, px, pgcs) = (false, false, false, false), // Reserved
            0b1110 => (pr, pw, px, pgcs) = (true, true, true, false), // Privileged read, write and execute
            0b1111 => (pr, pw, px, pgcs) = (false, false, false, false), // Reserved
            _ => panic!("Unreachable"),
        };

        p_overlay = !((permissions.ppi >> 3) & 1 != 0);
        pwxn = permissions.ppi == 0b0110;

        (r, w, x, gcs, wxn, overlay) = (pr, pw, px, pgcs, pwxn, p_overlay);

        if regime.has_unprivileged() {
            // Apply unprivileged indirect permissions
            match permissions.upi {
                0b0000 => (ur, uw, ux, ugcs) = (false, false, false, false), // No access
                0b0001 => (ur, uw, ux, ugcs) = (true, false, false, false),  // Unprivileged read
                0b0010 => (ur, uw, ux, ugcs) = (false, false, true, false),  // Unprivileged execute
                0b0011 => (ur, uw, ux, ugcs) = (true, false, true, false), // Unprivileged read and execute
                0b0100 => (ur, uw, ux, ugcs) = (false, false, false, false), // Reserved
                0b0101 => (ur, uw, ux, ugcs) = (true, true, false, false), // Unprivileged read and write
                0b0110 => (ur, uw, ux, ugcs) = (true, true, true, false), // Unprivileged read, write and execute
                0b0111 => (ur, uw, ux, ugcs) = (true, true, true, false), // Unprivileged read, write and execute
                0b1000 => (ur, uw, ux, ugcs) = (true, false, false, false), // Unprivileged read
                0b1001 => (ur, uw, ux, ugcs) = (true, false, false, true), // Unprivileged read and gcs
                0b1010 => (ur, uw, ux, ugcs) = (true, false, true, false), // Unprivileged read and execute
                0b1011 => (ur, uw, ux, ugcs) = (false, false, false, false), // Reserved
                0b1100 => (ur, uw, ux, ugcs) = (true, true, false, false), // Unprivileged read and write
                0b1101 => (ur, uw, ux, ugcs) = (false, false, false, false), // Reserved
                0b1110 => (ur, uw, ux, ugcs) = (true, true, true, false), // Unprivileged read,write and execute
                0b1111 => (ur, uw, ux, ugcs) = (false, false, false, false), // Reserved
                _ => panic!("Unreachable"),
            }
            u_overlay = !((permissions.upi >> 3) & 1 != 0);
            uwxn = permissions.upi == 0b0110;

            // If the decoded permissions has either px or pgcs along with either uw or ugcs,
            // then all effective Stage 1 Base Permissions are set to 0
            if (px || pgcs) && (uw || ugcs) {
                (pr, pw, px, pgcs) = (false, false, false, false);
                (ur, uw, ux, ugcs) = (false, false, false, false);
            }

            if self.is_feat_impl(Feat::PAN)
                && accdesc.pan
                && !(regime == Regime::EL10 && walkparams.nv1)
            {
                if self.read::<pstate::PAN>() != 0 && (permissions.upi != 0b0000) {
                    (pr, pw) = (false, false);
                }
            }

            if accdesc.el == EL::EL0 {
                (r, w, x, gcs, wxn, overlay) = (ur, uw, ux, ugcs, uwxn, u_overlay);
            } else {
                (r, w, x, gcs, wxn, overlay) = (pr, pw, px, pgcs, pwxn, p_overlay);
            }
        }

        // Prevent execution from Non-secure space by PE in secure state if SIF is set
        if accdesc.ss == SecurityState::Secure
            && walkstate.baseaddress.paspace == PASpace::NonSecure
        {
            x = x && !(walkparams.sif);
            gcs = false;
        }
        // Prevent execution from non-Root space by Root
        if accdesc.ss == SecurityState::Root && walkstate.baseaddress.paspace != PASpace::Root {
            x = false;
            gcs = false;
        }
        // Prevent execution from non-Realm space by Realm EL2 and Realm EL2&0
        if accdesc.ss == SecurityState::Realm
            && matches!(regime, Regime::EL2 | Regime::EL20)
            && walkstate.baseaddress.paspace != PASpace::Realm
        {
            x = false;
            gcs = false;
        }

        return S1AccessControls {
            r,
            w,
            x,
            gcs,
            wxn,
            overlay,
            overlay_perms: None,
        };
    }

    pub fn aarch64_s1_overlay_permissions(
        &mut self,
        regime: Regime,
        walkstate: &TTWState,
        accdesc: &AccessDescriptor,
    ) -> S1OverlayAccessControls {
        let (mut r, mut w, mut x);
        let (pr, pw, px);
        let (ur, uw, ux);
        let permissions = &walkstate.permissions;

        let por = self.aarch64_s1_por(regime);
        let bit_index = 4 * permissions.po_index;
        let ppo = (por >> bit_index) & ((1 << ((bit_index + 3) - bit_index + 1)) - 1);

        // Apply privileged overlay permissions
        match ppo {
            0b0000 => (pr, pw, px) = (false, false, false), // No access
            0b0001 => (pr, pw, px) = (true, false, false),  // Privileged read
            0b0010 => (pr, pw, px) = (false, false, true),  // Privileged execute
            0b0011 => (pr, pw, px) = (true, false, true),   // Privileged read and execute
            0b0100 => (pr, pw, px) = (false, true, false),  // Privileged write
            0b0101 => (pr, pw, px) = (true, true, false),   // Privileged read and write
            0b0110 => (pr, pw, px) = (false, true, true),   // Privileged write and execute
            0b0111 => (pr, pw, px) = (true, true, true),    // Privileged read, write and execute
            _ => (pr, pw, px) = (false, false, false),      // Reserved
        };

        (r, w, x) = (pr, pw, px);

        if regime.has_unprivileged() {
            let upo = (self.read::<POR_EL0>() >> bit_index)
                & ((1 << ((bit_index + 3) - bit_index + 1)) - 1);

            // Apply unprivileged overlay permissions
            match upo {
                0b0000 => (ur, uw, ux) = (false, false, false), // No access
                0b0001 => (ur, uw, ux) = (true, false, false),  // Unprivileged read
                0b0010 => (ur, uw, ux) = (false, false, true),  // Unprivileged execute
                0b0011 => (ur, uw, ux) = (true, false, true),   // Unprivileged read and execute
                0b0100 => (ur, uw, ux) = (false, true, false),  // Unprivileged write
                0b0101 => (ur, uw, ux) = (true, true, false),   // Unprivileged read and write
                0b0110 => (ur, uw, ux) = (false, true, true),   // Unprivileged write and execute
                0b0111 => (ur, uw, ux) = (true, true, true), // Unprivileged read, write and execute
                _ => (ur, uw, ux) = (false, false, false),   // Reserved
            };

            (r, w, x) = if accdesc.el == EL::EL0 {
                (ur, uw, ux)
            } else {
                (pr, pw, px)
            };
        }

        return S1OverlayAccessControls {
            or: r,
            ow: w,
            ox: x,
        };
    }

    pub fn aarch64_s1_tx_sz_faults(&mut self, regime: Regime, walkparams: &S1TTWParams) -> bool {
        let mintxsz =
            self.aarch64_s1_min_tx_sz(regime, walkparams.d128, walkparams.ds, walkparams.tgx);
        let maxtxsz = self.aarch64_max_tx_sz(walkparams.tgx);

        if walkparams.txsz < mintxsz {
            return self.is_feat_impl(Feat::LVA)
                || self.cpu.impdef.bool(&"Fault on TxSZ value below minimum");
        }
        if walkparams.txsz > maxtxsz {
            return self.cpu.impdef.bool(&"Fault on TxSZ value above maximum");
        }

        return false;
    }

    pub fn aarch64_s2_check_permissions(
        &mut self,
        fault: &mut FaultRecord,
        walkstate: &TTWState,
        walkparams: &S2TTWParams,
        ipa: &AddressDescriptor,
        accdesc: &AccessDescriptor,
    ) -> bool {
        let memtype = walkstate.memattrs.memtype;
        let permissions = &walkstate.permissions;
        let s2perms = self.aarch64_s2_compute_permissions(permissions, walkparams, accdesc);

        let (r, w);
        let (or, ow);

        if accdesc.acctype == AccessType::TTW {
            r = s2perms.r_mmu;
            w = s2perms.w_mmu;
            or = s2perms
                .overlay_perms
                .as_ref()
                .is_some_and(|o_perms| o_perms.or_mmu);
            ow = s2perms
                .overlay_perms
                .as_ref()
                .is_some_and(|o_perms| o_perms.ow_mmu);
        } else if accdesc.rcw {
            r = s2perms.r_rcw;
            w = s2perms.w_rcw;
            or = s2perms
                .overlay_perms
                .as_ref()
                .is_some_and(|o_perms| o_perms.or_rcw);
            ow = s2perms
                .overlay_perms
                .as_ref()
                .is_some_and(|o_perms| o_perms.ow_rcw);
        } else {
            r = s2perms.r;
            w = s2perms.w;
            or = s2perms
                .overlay_perms
                .as_ref()
                .is_some_and(|o_perms| o_perms.or);
            ow = s2perms
                .overlay_perms
                .as_ref()
                .is_some_and(|o_perms| o_perms.ow);
        }

        if accdesc.acctype == AccessType::TTW {
            if accdesc.toplevel
                && accdesc.varange == VARange::Lower
                && ((walkparams.tl0 && !s2perms.toplevel0)
                    || (walkparams.tl1 && (s2perms.toplevel1 && !s2perms.toplevel0)))
            {
                fault.statuscode = Fault::Permission;
                fault.toplevel = true;
            } else if accdesc.toplevel
                && accdesc.varange == VARange::Upper
                && ((walkparams.tl1 && !s2perms.toplevel1)
                    || (walkparams.tl0 && (!s2perms.toplevel1 && s2perms.toplevel0)))
            {
                fault.statuscode = Fault::Permission;
                fault.toplevel = true;
            }
            // Stage 2 Permission fault due to AssuredOnly check
            else if walkstate.s2assuredonly && !ipa.s1assured {
                fault.statuscode = Fault::Permission;
                fault.assuredonly = true;
            } else if s2perms.overlay && !or {
                fault.statuscode = Fault::Permission;
                fault.overlay = true;
            } else if accdesc.write && s2perms.overlay && !ow {
                fault.statuscode = Fault::Permission;
                fault.overlay = true;
            } else if walkparams.ptw && matches!(memtype, MemType::Device(_)) {
                fault.statuscode = Fault::Permission;
            }
            // Prevent translation table walks in Non-secure space by Realm state
            else if accdesc.ss == SecurityState::Realm
                && walkstate.baseaddress.paspace != PASpace::Realm
            {
                fault.statuscode = Fault::Permission;
            } else if !r {
                fault.statuscode = Fault::Permission;
            } else if accdesc.write && !w {
                fault.statuscode = Fault::Permission;
            } else if accdesc.write
                && !(walkparams.ha && walkparams.hd)
                && walkparams.s2pie
                && !permissions.s2dirty
            {
                fault.statuscode = Fault::Permission;
                fault.dirtybit = true;
            }
        }
        // Stage 2 Permission fault due to AssuredOnly check
        else if (walkstate.s2assuredonly && !ipa.s1assured)
            || (!walkstate.s2assuredonly
                && self.is_feat_impl(Feat::GCS)
                && self.read::<vtcr_el2::GCSH>() != 0
                && accdesc.acctype == AccessType::GCS
                && accdesc.el != EL::EL0)
        {
            fault.statuscode = Fault::Permission;
            fault.assuredonly = true;
        } else if accdesc.acctype == AccessType::IFETCH {
            if s2perms.overlay
                && s2perms
                    .overlay_perms
                    .as_ref()
                    .is_some_and(|o_perms| !o_perms.ox)
            {
                fault.statuscode = Fault::Permission;
                fault.overlay = true;
            } else if matches!(memtype, MemType::Device(_)) {
                fault.statuscode = Fault::Permission;
            }
            // Prevent execution from Non-secure space by Realm state
            else if accdesc.ss == SecurityState::Realm
                && walkstate.baseaddress.paspace != PASpace::Realm
            {
                fault.statuscode = Fault::Permission;
            } else if !s2perms.x {
                fault.statuscode = Fault::Permission;
            }
        } else if accdesc.acctype == AccessType::DC {
            if accdesc.cacheop == CacheOp::Invalidate {
                if !self.el_using_aarch32(EL::EL1) && s2perms.overlay && !ow {
                    fault.statuscode = Fault::Permission;
                    fault.overlay = true;
                }
                if !self.el_using_aarch32(EL::EL1) && !w {
                    fault.statuscode = Fault::Permission;
                }
            } else if !self.el_using_aarch32(EL::EL1)
                && accdesc.el == EL::EL0
                && s2perms.overlay
                && !or
            {
                fault.statuscode = Fault::Permission;
                fault.overlay = true;
            } else if walkparams.cmow
                && accdesc.opscope == CacheOpScope::PoC
                && accdesc.cacheop == CacheOp::CleanInvalidate
                && s2perms.overlay
                && !ow
            {
                fault.statuscode = Fault::Permission;
                fault.overlay = true;
            } else if !self.el_using_aarch32(EL::EL1) && accdesc.el == EL::EL0 && !r {
                fault.statuscode = Fault::Permission;
            } else if walkparams.cmow
                && accdesc.opscope == CacheOpScope::PoC
                && accdesc.cacheop == CacheOp::CleanInvalidate
                && !w
            {
                fault.statuscode = Fault::Permission;
            }
        } else if accdesc.acctype == AccessType::IC {
            if !self.el_using_aarch32(EL::EL1)
                && accdesc.el == EL::EL0
                && s2perms.overlay
                && !or
                && self
                    .cpu
                    .impdef
                    .bool(&"Permission fault on EL0 IC_IVAU execution")
            {
                fault.statuscode = Fault::Permission;
                fault.overlay = true;
            } else if walkparams.cmow && s2perms.overlay && !ow {
                fault.statuscode = Fault::Permission;
                fault.overlay = true;
            } else if !self.el_using_aarch32(EL::EL1)
                && accdesc.el == EL::EL0
                && !r
                && self
                    .cpu
                    .impdef
                    .bool(&"Permission fault on EL0 IC_IVAU execution")
            {
                fault.statuscode = Fault::Permission;
            } else if walkparams.cmow && !w {
                fault.statuscode = Fault::Permission;
            }
        } else if accdesc.read && s2perms.overlay && !or {
            fault.statuscode = Fault::Permission;
            fault.overlay = true;
            fault.write = Some(false);
        } else if accdesc.write && s2perms.overlay && !ow {
            fault.statuscode = Fault::Permission;
            fault.overlay = true;
            fault.write = Some(true);
        } else if accdesc.read && !r {
            fault.statuscode = Fault::Permission;
            fault.write = Some(false);
        } else if accdesc.write && !w {
            fault.statuscode = Fault::Permission;
            fault.write = Some(true);
        } else if self.is_feat_impl(Feat::MtePerm)
            && ((accdesc.tagchecked
                && self.aarch64_effective_tcf(accdesc.el, accdesc.read) != TCFType::Ignore)
                || accdesc.tagaccess)
            && ipa
                .memattrs
                .as_ref()
                .is_some_and(|memattrs| memattrs.tags == MemTagType::AllocationTagged)
            && permissions.s2tag_na
            && self.s2_dcache_enabled()
        {
            fault.statuscode = Fault::Permission;
            fault.tagaccess = true;
            fault.write = Some(accdesc.tagaccess && accdesc.write);
        } else if accdesc.write
            && !(walkparams.ha && walkparams.hd)
            && walkparams.s2pie
            && !permissions.s2dirty
        {
            fault.statuscode = Fault::Permission;
            fault.dirtybit = true;
            fault.write = Some(true);
        }

        let mro;
        if s2perms.overlay {
            mro = !(s2perms.w
                && s2perms
                    .overlay_perms
                    .as_ref()
                    .is_some_and(|o_perms| o_perms.ow))
                && (s2perms.w_rcw
                    && s2perms
                        .overlay_perms
                        .as_ref()
                        .is_some_and(|o_perms| o_perms.ow_rcw))
                && (s2perms.w_mmu
                    && s2perms
                        .overlay_perms
                        .as_ref()
                        .is_some_and(|o_perms| o_perms.ow_mmu))
        } else {
            mro = !s2perms.w && s2perms.w_rcw && s2perms.w_mmu;
        }
        return mro;
    }

    pub fn aarch64_s2_compute_permissions(
        &mut self,
        permissions: &Permissions,
        walkparams: &S2TTWParams,
        accdesc: &AccessDescriptor,
    ) -> S2AccessControls {
        let mut s2perms;
        if walkparams.s2pie {
            s2perms = self.aarch64_s2_indirect_base_permissions(permissions, accdesc);
            s2perms.overlay = self.is_feat_impl(Feat::S2POE) && self.read::<vtcr_el2::S2POE>() != 0;
            if s2perms.overlay {
                let s2overlay_perms = self.aarch64_s2_overlay_permissions(permissions, accdesc);

                // Toplevel is applicable only when the effective S2 permissions is MRO
                if !(s2perms.w && s2overlay_perms.ow)
                    && (s2perms.w_rcw && s2overlay_perms.ow_rcw)
                    && (s2perms.w_mmu && s2overlay_perms.ow_mmu)
                {
                    s2perms.toplevel0 = s2perms.toplevel0 | s2overlay_perms.toplevel0;
                    s2perms.toplevel1 = s2perms.toplevel1 | s2overlay_perms.toplevel1;
                } else {
                    s2perms.toplevel0 = false;
                    s2perms.toplevel1 = false;
                }
                s2perms.overlay_perms = Some(s2overlay_perms);
            }
        } else {
            s2perms = self.aarch64_s2_direct_base_permissions(permissions, accdesc);
        }

        return s2perms;
    }

    pub fn aarch64_s2_direct_base_permissions(
        &mut self,
        permissions: &Permissions,
        accdesc: &AccessDescriptor,
    ) -> S2AccessControls {
        let r = permissions.s2ap & 1 != 0;
        let w = (permissions.s2ap >> 1) & 1 != 0;
        let (px, ux);
        match (permissions.s2xn, permissions.s2xnx) {
            (false, false) => (px, ux) = (true, true),
            (false, true) => (px, ux) = (false, true),
            (true, false) => (px, ux) = (false, false),
            (true, true) => (px, ux) = (true, false),
        };

        let x = if accdesc.el == EL::EL0 { ux } else { px };

        S2AccessControls {
            r: r,
            w: w,
            x: x,
            r_rcw: r,
            w_rcw: w,
            r_mmu: r,
            w_mmu: w,
            toplevel0: false,
            toplevel1: false,
            overlay: false,
            overlay_perms: None,
        }
    }

    pub fn aarch64_s2_has_alignment_fault(
        &mut self,
        accdesc: &AccessDescriptor,
        aligned: bool,
        memattrs: &MemoryAttributes,
    ) -> bool {
        if accdesc.acctype == AccessType::IFETCH {
            return false;
        } else if self.is_feat_impl(Feat::MTE) && accdesc.write && accdesc.devstoreunpred {
            return matches!(memattrs.memtype, MemType::Device(_));
        } else if accdesc.acctype == AccessType::DCZero {
            return matches!(memattrs.memtype, MemType::Device(_));
        } else {
            return matches!(memattrs.memtype, MemType::Device(_)) && !aligned;
        }
    }

    pub fn aarch64_s2_inconsistent_sl(&mut self, walkparams: &S2TTWParams) -> bool {
        let startlevel = walkparams.aarch64_s2_start_level();
        let levels = 3 - startlevel;
        let granulebits = walkparams.tgx.granule_bits() as i64;
        let descsizelog2 = 3;
        let stride = granulebits - descsizelog2;

        let sl_min_iasize = levels * stride + granulebits + 1;

        let sl_max_iasize = sl_min_iasize + (stride - 1) + 4;
        let iasize = 64 - walkparams.txsz as i64;

        return iasize < sl_min_iasize || iasize > sl_max_iasize;
    }

    pub fn aarch64_s2_indirect_base_permissions(
        &mut self,
        permissions: &Permissions,
        accdesc: &AccessDescriptor,
    ) -> S2AccessControls {
        let (r, w);
        let (r_rcw, w_rcw);
        let (r_mmu, w_mmu);
        let (px, ux);
        let (toplevel0, toplevel1);

        let s2pi = permissions.s2pi;
        match s2pi {
            0b0000 => (r, w, px, ux, w_rcw, w_mmu) = (false, false, false, false, false, false), // No Access
            0b0001 => (r, w, px, ux, w_rcw, w_mmu) = (false, false, false, false, false, false), // Reserved
            0b0010 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, true, true), // MRO
            0b0011 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, true, true), // MRO-TL1
            0b0100 => (r, w, px, ux, w_rcw, w_mmu) = (false, true, false, false, false, false), // Write Only
            0b0101 => (r, w, px, ux, w_rcw, w_mmu) = (false, false, false, false, false, false), // Reserved
            0b0110 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, true, true), // MRO-TL0
            0b0111 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, true, true), // MRO-TL01
            0b1000 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, false, false), // Read Only
            0b1001 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, true, false, false), // Read, Unpriv Execute
            0b1010 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, true, false, false, false), // Read, Priv Execute
            0b1011 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, true, true, false, false), // Read, All Execute
            0b1100 => (r, w, px, ux, w_rcw, w_mmu) = (true, true, false, false, true, true),  // RW
            0b1101 => (r, w, px, ux, w_rcw, w_mmu) = (true, true, false, true, true, true), // RW, Unpriv Execute
            0b1110 => (r, w, px, ux, w_rcw, w_mmu) = (true, true, true, false, true, true), // RW, Priv Execute
            0b1111 => (r, w, px, ux, w_rcw, w_mmu) = (true, true, true, true, true, true), // RW, All Execute
            _ => panic!("Unreachable"),
        };

        let x = if accdesc.el == EL::EL0 { ux } else { px };

        // RCW and MMU read permissions.
        (r_rcw, r_mmu) = (r, r);

        // Stage 2 Top Level Permission Attributes.
        match s2pi {
            0b0110 => (toplevel0, toplevel1) = (true, false),
            0b0011 => (toplevel0, toplevel1) = (false, true),
            0b0111 => (toplevel0, toplevel1) = (true, true),
            _ => (toplevel0, toplevel1) = (false, false),
        };

        S2AccessControls {
            r: r,
            w: w,
            x: x,
            r_rcw: r_rcw,
            r_mmu: r_mmu,
            w_rcw: w_rcw,
            w_mmu: w_mmu,
            toplevel0: toplevel0,
            toplevel1: toplevel1,
            overlay_perms: None,
            overlay: false,
        }
    }

    pub fn aarch64_s2_invalid_sl(&mut self, walkparams: &S2TTWParams) -> bool {
        match walkparams.tgx {
            TGx::TG4KB => match (walkparams.sl2, walkparams.sl0) {
                (true, 0b01 | 0b11) => return true,
                (true, 0b10) => return true,
                (true, 0b00) => return self.aarch64_pa_max() < 52,
                (false, 0b10) => return self.aarch64_pa_max() < 44,
                (false, 0b11) => return !self.is_feat_impl(Feat::TTST),
                _ => return false,
            },
            TGx::TG16KB => match walkparams.sl0 {
                0b11 => return !walkparams.ds || self.aarch64_pa_max() < 52,
                0b10 => return self.aarch64_pa_max() < 42,
                _ => return false,
            },
            TGx::TG64KB => match walkparams.sl0 {
                0b11 => return true,
                0b10 => return self.aarch64_pa_max() < 44,
                _ => return false,
            },
        }
    }

    pub fn aarch64_s2_overlay_permissions(
        &mut self,
        permissions: &Permissions,
        accdesc: &AccessDescriptor,
    ) -> S2OverlayAccessControls {
        let (r, w);
        let (r_rcw, w_rcw);
        let (r_mmu, w_mmu);
        let (px, ux);
        let (toplevel0, toplevel1);

        let index = 4 * permissions.s2po_index;
        let s2po = (self.read::<S2POR_EL1>() >> index) & ((1 << ((index + 3) - index + 1)) - 1);

        match s2po {
            0b0000 => (r, w, px, ux, w_rcw, w_mmu) = (false, false, false, false, false, false), // No Access
            0b0001 => (r, w, px, ux, w_rcw, w_mmu) = (false, false, false, false, false, false), // Reserved
            0b0010 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, true, true), // MRO
            0b0011 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, true, true), // MRO-TL1
            0b0100 => (r, w, px, ux, w_rcw, w_mmu) = (false, true, false, false, false, false), // Write Only
            0b0101 => (r, w, px, ux, w_rcw, w_mmu) = (false, false, false, false, false, false), // Reserved
            0b0110 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, true, true), // MRO-TL0
            0b0111 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, true, true), // MRO-TL01
            0b1000 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, false, false, false), // Read Only
            0b1001 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, false, true, false, false), // Read, Unpriv Execute
            0b1010 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, true, false, false, false), // Read, Priv Execute
            0b1011 => (r, w, px, ux, w_rcw, w_mmu) = (true, false, true, true, false, false), // Read, All Execute
            0b1100 => (r, w, px, ux, w_rcw, w_mmu) = (true, true, false, false, true, true),  // RW
            0b1101 => (r, w, px, ux, w_rcw, w_mmu) = (true, true, false, true, true, true), // RW, Unpriv Execute
            0b1110 => (r, w, px, ux, w_rcw, w_mmu) = (true, true, true, false, true, true), // RW, Priv Execute
            0b1111 => (r, w, px, ux, w_rcw, w_mmu) = (true, true, true, true, true, true), // RW, All Execute
            _ => panic!("Unreachable"),
        };
        let x = if accdesc.el == EL::EL0 { ux } else { px };

        // RCW and MMU read permissions.
        (r_rcw, r_mmu) = (r, r);

        // Stage 2 Top Level Permission Attributes.
        match s2po {
            0b0110 => (toplevel0, toplevel1) = (true, false),
            0b0011 => (toplevel0, toplevel1) = (false, true),
            0b0111 => (toplevel0, toplevel1) = (true, true),
            _ => (toplevel0, toplevel1) = (false, false),
        };

        S2OverlayAccessControls {
            or: r,
            ow: w,
            ox: x,
            or_rcw: r_rcw,
            ow_rcw: w_rcw,
            or_mmu: r_mmu,
            ow_mmu: w_mmu,
            toplevel0: toplevel0,
            toplevel1: toplevel1,
        }
    }

    pub fn aarch64_s2_tx_sz_faults(&mut self, walkparams: &S2TTWParams, s1aarch64: bool) -> bool {
        let mintxsz =
            self.aarch64_s2_min_tx_sz(walkparams.d128, walkparams.ds, walkparams.tgx, s1aarch64);
        let maxtxsz = self.aarch64_max_tx_sz(walkparams.tgx);

        if walkparams.txsz < mintxsz {
            return self.is_feat_impl(Feat::LPA)
                || self.cpu.impdef.bool(&"Fault on TxSZ value below minimum");
        }

        if walkparams.txsz > maxtxsz {
            return self.cpu.impdef.bool(&"Fault on TxSZ value above maximum");
        }

        return false;
    }
}
