use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_full_translate(
        &mut self,
        va: u64,
        accdesc: AccessDescriptor,
        aligned: bool,
    ) -> AddressDescriptor {
        let el = self.curr_el();
        let regime = self.translation_regime(el);
        let mut fault = FaultRecord::no_fault_for_access(accdesc.clone());

        let ipa = self.aarch64_s1_translate(regime, va, aligned, &mut fault);

        if fault.statuscode != Fault::None {
            return AddressDescriptor::create_faulty(va, fault);
        }
        let ipa = ipa.unwrap();

        if regime == Regime::EL10 && self.el2_enabled() {
            let pa = self.aarch64_s2_translate(&mut fault, &ipa, true, aligned, &accdesc);
            if fault.statuscode != Fault::None {
                return AddressDescriptor::create_faulty(va, fault);
            } else {
                return pa.unwrap();
            }
        } else {
            return ipa;
        }
    }

    pub fn aarch64_mem_swap_table_desc(
        &mut self,
        fault: &mut FaultRecord,
        prev_desc: u128,
        new_desc: u128,
        ee: bool,
        descaccess: &AccessDescriptor,
        descpaddr: &AddressDescriptor,
        n: usize,
    ) -> Option<u128> {
        if self.is_feat_impl(Feat::RME) {
            fault.gpcf = self.granule_protection_check(descpaddr, descaccess);
            if fault.gpcf.gpf != GPCF::None {
                fault.statuscode = Fault::GPCFOnWalk;
                fault.paddress = descpaddr.paddress;
                fault.gpcfs2walk = fault.secondstage;
                return None;
            }
        }

        // All observers in the shareability domain observe the
        // following memory read and write accesses atomically.
        let (memstatus, mem_desc) = self.phys_mem_read(descpaddr, n / 8, descaccess);

        if ee {
            todo!("mem_desc = BigEndianReverse(mem_desc);");
        }

        if memstatus.is_fault() {
            todo!("memstatus.is_fault");
        }
        let mem_desc = mem_desc.unwrap();
        let mut mem_desc = match n / 8 {
            1 => {
                let mut out = [0; 1];
                out.copy_from_slice(&mem_desc[0..1]);
                u8::from_le_bytes(out) as u128
            }
            2 => {
                let mut out = [0; 2];
                out.copy_from_slice(&mem_desc[0..2]);
                u16::from_le_bytes(out) as u128
            }
            4 => {
                let mut out = [0; 4];
                out.copy_from_slice(&mem_desc[0..4]);
                u32::from_le_bytes(out) as u128
            }
            8 => {
                let mut out = [0; 8];
                out.copy_from_slice(&mem_desc[0..8]);
                u64::from_le_bytes(out) as u128
            }
            16 => u128::from_le_bytes(mem_desc),
            _ => panic!("Unsupported phys_mem_read size {0:?}", n / 8),
        };

        if mem_desc == prev_desc {
            let ordered_new_desc = if ee {
                todo!("BigEndianReverse(new_desc)")
            } else {
                new_desc
            };
            let memstatus = self.phys_mem_write(
                descpaddr,
                n / 8,
                descaccess,
                &ordered_new_desc.to_le_bytes(),
            );

            if memstatus.is_fault() {
                todo!("memstatus.is_fault");
            }

            // Reflect what is now in memory (in little endian format)
            mem_desc = new_desc;
        }

        if n == 64 {
            mem_desc &= (1 << 64) - 1;
        }

        return Some(mem_desc);
    }

    fn aarch64_s1_disabled_output(
        &mut self,
        regime: Regime,
        va: u64,
        aligned: bool,
        fault: &mut FaultRecord,
    ) -> Option<AddressDescriptor> {
        let accdesc = fault.accessdesc.as_ref().unwrap();
        let walkparams = self.aarch64_get_s1_ttw_params(regime, accdesc.ss, va);

        // No memory page is guarded when stage 1 address translation is disabled
        //     SetInGuardedPage(FALSE);

        let oa = FullAddress {
            address: va & ((1 << 56) - 1),
            paspace: match accdesc.ss {
                SecurityState::Secure => PASpace::Secure,
                SecurityState::NonSecure => PASpace::NonSecure,
                SecurityState::Root => PASpace::Root,
                SecurityState::Realm => PASpace::Realm,
            },
        };

        let memattrs;
        if regime == Regime::EL10 && self.el2_enabled() && walkparams.dc {
            let default_cacheability = MemAttrHints {
                attrs: MemAttr::WB,
                hints: Some(MemHint::RWA),
                transient: None,
            };

            memattrs = MemoryAttributes {
                memtype: MemType::Normal,
                outer: default_cacheability,
                inner: default_cacheability,
                shareability: Shareability::NSH,
                tags: if walkparams.dct {
                    MemTagType::AllocationTagged
                } else if walkparams.mtx {
                    MemTagType::CanonicallyTagged
                } else {
                    MemTagType::Untagged
                },
                notagaccess: false,
                xs: false,
            };
        } else if accdesc.acctype == AccessType::IFETCH {
            let i_cache_attr;
            if self.aarch64_s1_icache_enabled(regime) {
                i_cache_attr = MemAttrHints {
                    attrs: MemAttr::WT,
                    hints: Some(MemHint::RA),
                    transient: Some(false),
                };
            } else {
                i_cache_attr = MemAttrHints {
                    attrs: MemAttr::NC,
                    hints: None,
                    transient: None,
                };
            }

            memattrs = MemoryAttributes {
                memtype: MemType::Normal,
                outer: i_cache_attr,
                inner: i_cache_attr,
                shareability: Shareability::OSH,
                tags: MemTagType::Untagged,
                notagaccess: false,
                xs: true,
            };
        } else {
            let uninit_attrs = MemAttrHints {
                attrs: MemAttr::NC,
                hints: None,
                transient: None,
            };

            memattrs = MemoryAttributes {
                memtype: MemType::Device(DeviceType::NoGNoRNoE),
                outer: uninit_attrs,
                inner: uninit_attrs,
                shareability: Shareability::OSH,
                tags: if walkparams.mtx {
                    MemTagType::CanonicallyTagged
                } else {
                    MemTagType::Untagged
                },
                notagaccess: false,
                xs: true,
            };
        }

        let va = if walkparams.mtx && !walkparams.tbi && accdesc.acctype != AccessType::IFETCH {
            if regime.has_unprivileged() && va >> 55 & 1 != 0 {
                va | (0b1111 << 56)
            } else {
                va & !(0b1111 << 56)
            }
        } else {
            va
        };

        fault.level = 0;
        let addrtop = self.aarch64_addr_top(walkparams.tbid, accdesc.acctype, walkparams.tbi);
        let pamax = self.aarch64_pa_max();

        let va_addrtop_to_pamax = if addrtop > pamax {
            (va >> pamax) & ((1 << (addrtop - pamax + 1)) - 1)
        } else {
            (va >> addrtop) & ((1 << (pamax - addrtop + 1)) - 1)
        };
        if va_addrtop_to_pamax != 0 {
            fault.statuscode = Fault::AddressSize;
        } else if self.aarch64_s1_has_alignment_fault(
            accdesc,
            aligned,
            walkparams.ntlsmd,
            &memattrs,
        ) {
            fault.statuscode = Fault::Alignment;
        }

        if fault.statuscode != Fault::None {
            return None;
        } else {
            let paspace = oa.paspace.clone();
            Some(AddressDescriptor {
                paddress: Some(oa),
                vaddress: va,
                memattrs: Some(memattrs),
                fault: FaultRecord::no_fault(),
                s1assured: false,
                mecid: Some(self.aarch64_s1_disabled_output_mecid(&walkparams, regime, paspace)),
                s2fs1mro: None,
                tlbcontext: None, // Uninitialized here
            })
        }
    }

    fn aarch64_s1_translate(
        &mut self,
        regime: Regime,
        va: u64,
        aligned: bool,
        fault: &mut FaultRecord,
    ) -> Option<AddressDescriptor> {
        // Prepare fault fields in case a fault is detected
        fault.secondstage = false;
        fault.s2fs1walk = false;

        if !self.aarch64_s1_enabled(regime) {
            // println!("S1 not enabled.");
            return self.aarch64_s1_disabled_output(regime, va, aligned, fault);
        }
        // println!("S1 enabled.");

        let accdesc = fault.accessdesc.as_ref().unwrap().clone();
        let mut walkparams = self.aarch64_get_s1_ttw_params(regime, accdesc.ss, va);

        let s1mintxsz =
            self.aarch64_s1_min_tx_sz(regime, walkparams.d128, walkparams.ds, walkparams.tgx);
        let s1maxtxsz = self.aarch64_max_tx_sz(walkparams.tgx);

        if self.aarch64_s1_tx_sz_faults(regime, &walkparams) {
            panic!("self.aarch64_s1_tx_sz_faults(regime, &walkparams)");
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        } else if walkparams.txsz < s1mintxsz {
            walkparams.txsz = s1mintxsz & 0b111111;
        } else if walkparams.txsz > s1maxtxsz {
            walkparams.txsz = s1maxtxsz & 0b111111;
        }
        let walkparams = walkparams;

        if self.aarch64_va_is_out_of_range(va, accdesc.acctype, regime, &walkparams) {
            panic!("self.aarch64_va_is_out_of_range(va, accdesc.acctype, regime, &walkparams)");
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        if accdesc.el == EL::EL0 && walkparams.e0_pd {
            panic!("accdesc.el == EL::EL0 && walkparams.e0_pd");
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        if self.is_feat_impl(Feat::TME)
            && accdesc.el == EL::EL0
            && walkparams.nfd
            && accdesc.transactional
        {
            panic!(
                "self.is_feat_impl(Feat::TME)
            && accdesc.el == EL::EL0
            && walkparams.nfd
            && accdesc.transactional"
            );
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        if self.is_feat_impl(Feat::SVE)
            && accdesc.el == EL::EL0
            && walkparams.nfd
            && ((accdesc.nonfault && accdesc.contiguous)
                || (accdesc.firstfault && !accdesc.first && !accdesc.contiguous))
        {
            panic!(
                "self.is_feat_impl(Feat::SVE)
            && accdesc.el == EL::EL0
            && walkparams.nfd
            && ((accdesc.nonfault && accdesc.contiguous)
                || (accdesc.firstfault && !accdesc.first && !accdesc.contiguous))"
            );
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        let mut descipaddr: Option<AddressDescriptor>;
        let mut walkstate: TTWState;
        let mut descriptor: u128;
        let mut new_desc: u128;
        let mut mem_desc: Option<u128> = Some(0);

        loop {
            let s1_walk_res = self.aarch64_s1_walk(
                &walkparams,
                va,
                regime,
                if walkparams.d128 { 128 } else { 64 },
                fault,
            );

            if s1_walk_res.is_none() {
                return None;
            }

            (descipaddr, walkstate, descriptor) = s1_walk_res.unwrap();

            // if accdesc.acctype == AccessType_IFETCH then
            // // Flag the fetched instruction is from a guarded page
            // SetInGuardedPage(walkstate.guardedpage == '1');

            if self.aarch64_s1_has_alignment_fault(
                &accdesc,
                aligned,
                walkparams.ntlsmd,
                &walkstate.memattrs,
            ) {
                fault.statuscode = Fault::Alignment;
            }

            if fault.statuscode == Fault::None {
                // self.aarch64_s1_check_permissions(fault, regime, &walkstate, &walkparams, &accdesc);
            }

            new_desc = descriptor;
            if walkparams.ha && self.aarch64_setting_access_flag_permitted(fault) {
                // Set descriptor AF bit
                // new_desc<10> = '1';
                new_desc |= 1 << 10;
            }

            // If HW update of dirty bit is enabled, the walk state permissions
            // will already reflect a configuration permitting writes.
            // The update of the descriptor occurs only if the descriptor bits in
            // memory do not reflect that and the access instigates a write.
            if self.aarch64_setting_dirty_state_permitted(fault)
                && walkparams.ha
                && walkparams.hd
                && (walkparams.pie || (descriptor >> 51) & 1 != 0)
                && accdesc.write
                && !matches!(
                    accdesc.acctype,
                    AccessType::AT | AccessType::IC | AccessType::DC
                )
            {
                // Clear descriptor AP[2]/nDirty bit permitting stage 1 writes
                // new_desc<7> = '0';
                new_desc &= !(1 << 7);
            }

            // Either the access flag was clear or AP[2]/nDirty is set
            if new_desc != descriptor {
                let descpaddr;
                let descaccess = AccessDescriptor::create_tte_update(&accdesc);
                if regime == Regime::EL10 && self.el2_enabled() {
                    let mut s2fault = FaultRecord::no_fault();

                    let descpaddr_result = self.aarch64_s2_translate(
                        &mut s2fault,
                        descipaddr.as_ref().unwrap(),
                        true,
                        true,
                        &descaccess,
                    );

                    if s2fault.statuscode != Fault::None {
                        *fault = s2fault;
                        return None;
                    }

                    descpaddr = descpaddr_result.unwrap();
                } else {
                    descpaddr = descipaddr.as_ref().unwrap().clone();
                }

                mem_desc = self.aarch64_mem_swap_table_desc(
                    fault,
                    descriptor,
                    new_desc,
                    walkparams.ee,
                    &descaccess,
                    &descpaddr,
                    if walkparams.d128 { 128 } else { 64 },
                );
            }

            if new_desc == descriptor
                || mem_desc
                    .as_ref()
                    .is_some_and(|mem_desc| *mem_desc == new_desc)
            {
                break;
            }
        }

        if self.cpu.tlb_enabled
            && new_desc != descriptor
            && ((mem_desc.as_ref().unwrap().clone() >> 10) & 1) != 0
        {
            let mut tlbcontext = descipaddr
                .as_ref()
                .unwrap()
                .tlbcontext
                .as_ref()
                .unwrap()
                .clone();
            let mut tlbrecord;
            if (descriptor >> 10) & 1 != 0 {
                // let tlbentry = self.s1_tlb_lookup(descipaddr.tlbcontext.as_ref().unwrap());
                let tlbentry = self.s1_tlb_lookup(&tlbcontext).unwrap();
                // assert(tlbentry.valid_name);
                tlbrecord = tlbentry.tlbrecord.clone();
                // println!("TLB S1Translate edit.");
            } else {
                // tlbcontext = descipaddr.tlbcontext.as_ref().unwrap().clone();
                tlbcontext.xs = walkstate.memattrs.xs;
                tlbcontext.level = walkstate.level;
                tlbcontext.ng = walkstate.ng;
                tlbcontext.isd128 = walkparams.d128;
                tlbrecord = TLBRecord {
                    context: tlbcontext,
                    walkstate: walkstate.clone(),
                    blocksize: walkparams
                        .tgx
                        .translation_size(walkparams.d128, walkstate.level),
                    contigsize: if walkstate.contiguous.is_some_and(|val| val) {
                        walkparams
                            .tgx
                            .contiguous_size(walkparams.d128, walkstate.level)
                    } else {
                        0
                    },
                    s1descriptor: 0,
                    s2descriptor: 0,
                };

                // println!("TLB S1Translate fill.");
            };
            if walkparams.d128 {
                tlbrecord.s1descriptor = mem_desc.as_ref().unwrap().clone();
            } else {
                // tlbrecord.s1descriptor[63..0] = mem_desc[63..0];
                // tlbrecord.s1descriptor[127..64] = Zeros(64)
                tlbrecord.s1descriptor = descriptor & !((u64::MAX as u128) << 64);
            };
            self.s1_tlb_cache(tlbrecord);
        }

        if fault.statuscode != Fault::None {
            return None;
        }

        // Output Address
        let oa = self.stage_oa(va, walkparams.d128, walkparams.tgx, &walkstate);
        let mut memattrs = if accdesc.acctype == AccessType::IFETCH
            && (matches!(walkstate.memattrs.memtype, MemType::Device(_))
                || !self.aarch64_s1_icache_enabled(regime))
        {
            // Treat memory attributes as Normal Non-Cacheable
            MemoryAttributes::normal_nc(walkstate.memattrs.xs)
        } else if accdesc.acctype != AccessType::IFETCH
            && !self.aarch64_s1_dcache_enabled(regime)
            && walkstate.memattrs.memtype == MemType::Normal
        {
            // Treat memory attributes as Normal Non-Cacheable
            let mut memattrs = MemoryAttributes::normal_nc(walkstate.memattrs.xs);
            // The effect of SCTLR_ELx.C when '0' is Constrained UNPREDICTABLE
            // on the Tagged attribute
            if self.is_feat_impl(Feat::MTE2)
                && walkstate.memattrs.tags == MemTagType::AllocationTagged
            {
                memattrs.tags = MemTagType::Untagged;
            }
            memattrs
        } else {
            walkstate.memattrs.clone()
        };

        // Shareability value of stage 1 translation subject to stage 2 is IMPLEMENTATION DEFINED
        // to be either effective value or descriptor value
        if regime == Regime::EL10
            && self.el2_enabled()
            && self.read::<hcr_el2::VM>() != 0
            && !self
                .cpu
                .impdef
                .bool(&"Apply effective shareability at stage 1")
        {
            memattrs.shareability = walkstate.memattrs.shareability;
        } else {
            memattrs.shareability = memattrs.effective_shareability();
        }

        if accdesc.ls64 && memattrs.memtype == MemType::Normal {
            if memattrs.inner.attrs != MemAttr::NC || memattrs.outer.attrs != MemAttr::NC {
                fault.statuscode = Fault::Exclusive;
                return None;
            }
        }

        let mut ipa = AddressDescriptor::create(va, Some(oa), memattrs);
        ipa.tlbcontext = descipaddr.and_then(|descipaddr| descipaddr.tlbcontext); // From Sail for __tlb_enabled
        ipa.s1assured = walkstate.s1assured;
        ipa.mecid = Some(self.s1_output_mecid(
            &walkparams,
            regime,
            VARange::aarch64_from_va(va),
            ipa.paddress.as_ref().unwrap().paspace,
            descriptor,
        ));

        return Some(ipa);
    }

    pub fn aarch64_s2_translate(
        &mut self,
        fault: &mut FaultRecord,
        ipa: &AddressDescriptor,
        s1aarch64: bool,
        aligned: bool,
        accdesc: &AccessDescriptor,
    ) -> Option<AddressDescriptor> {
        let mut walkparams =
            self.aarch64_get_s2_ttw_params(accdesc.ss, ipa.paddress.unwrap().paspace, s1aarch64);
        let mut s2fs1mro = false;
        // Prepare fault fields in case a fault is detected
        fault.statuscode = Fault::None; // Ignore any faults from stage 1
        fault.dirtybit = false;
        fault.overlay = false;
        fault.tagaccess = false;
        fault.s1tagnotdata = false;
        fault.secondstage = true;
        fault.s2fs1walk = accdesc.acctype == AccessType::TTW;
        fault.ipaddress = ipa.paddress;

        if !walkparams.vm {
            // Stage 2 translation is disabled
            return Some(ipa.clone());
        }

        let s2mintxsz =
            self.aarch64_s2_min_tx_sz(walkparams.d128, walkparams.ds, walkparams.tgx, s1aarch64);
        let s2maxtxsz = self.aarch64_max_tx_sz(walkparams.tgx);
        if self.aarch64_s2_tx_sz_faults(&walkparams, s1aarch64) {
            panic!("self.aarch64_s2_tx_sz_faults(&walkparams, s1aarch64)");
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        } else if walkparams.txsz < s2mintxsz {
            walkparams.txsz = s2mintxsz & 0b111111;
        } else if walkparams.txsz > s2maxtxsz {
            walkparams.txsz = s2maxtxsz & 0b111111;
        }
        let walkparams = walkparams;

        if !walkparams.d128
            && (self.aarch64_s2_invalid_sl(&walkparams)
                || self.aarch64_s2_inconsistent_sl(&walkparams))
        {
            panic!(
                "!walkparams.d128
            && (self.aarch64_s2_invalid_sl(&walkparams)
                || self.aarch64_s2_inconsistent_sl(&walkparams))"
            );
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        if self.aarch64_ipa_is_out_of_range(ipa.paddress.unwrap().address, &walkparams) {
            panic!("self.aarch64_ipa_is_out_of_range(ipa.paddress.unwrap().address, &walkparams)");
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        let mut descipaddr: AddressDescriptor;
        let mut walkstate: TTWState;
        let mut descriptor: u128;
        let mut new_desc: u128;
        let mut mem_desc = None;

        loop {
            let s2_walk_res = self.aarch64_s2_walk(fault, &ipa, &walkparams, accdesc, 128);

            if fault.statuscode != Fault::None {
                return None;
            }
            (descipaddr, walkstate, descriptor) = s2_walk_res.unwrap();

            if self.aarch64_s2_has_alignment_fault(&accdesc, aligned, &walkstate.memattrs) {
                fault.statuscode = Fault::Alignment;
            }

            if fault.statuscode == Fault::None {
                s2fs1mro = self.aarch64_s2_check_permissions(
                    fault,
                    &walkstate,
                    &walkparams,
                    &ipa,
                    &accdesc,
                );
            }
            new_desc = descriptor;
            if walkparams.ha && self.aarch64_setting_access_flag_permitted(fault) {
                new_desc |= 1 << 10;
            }

            if self.aarch64_setting_dirty_state_permitted(fault)
                && walkparams.ha
                && walkparams.hd
                && (walkparams.s2pie || ((descriptor >> 51) & 1 != 0))
                && accdesc.write
                && !matches!(
                    accdesc.acctype,
                    AccessType::AT | AccessType::IC | AccessType::DC
                )
            {
                new_desc |= 1 << 7;
            }

            // Either the access flag was clear or S2AP[1]/Dirty is clear
            if new_desc != descriptor {
                if walkparams.hdbss && ((descriptor >> 7) & 1 == 0) && ((new_desc >> 7) & 1 != 0) {
                    self.append_to_hdbss(
                        fault,
                        ipa.paddress.as_ref().unwrap(),
                        &accdesc,
                        &walkparams,
                        walkstate.level,
                    );
                }

                if self.read::<hdbssprod_el2::FSC>() != 0b101000
                    && (!fault.hdbssf || fault.statuscode.is_external_abort())
                {
                    let descaccess = AccessDescriptor::create_tte_update(&accdesc);
                    mem_desc = self.aarch64_mem_swap_table_desc(
                        fault,
                        descriptor,
                        new_desc,
                        walkparams.ee,
                        &descaccess,
                        &descipaddr,
                        if walkparams.d128 { 128 } else { 64 },
                    );
                }
                if fault.statuscode != Fault::None {
                    return None;
                }
            }

            if new_desc == descriptor
                || mem_desc
                    .as_ref()
                    .is_some_and(|mem_desc| *mem_desc == new_desc)
            {
                break;
            }
        }

        if self.cpu.tlb_enabled
            && new_desc != descriptor
            && ((mem_desc.as_ref().unwrap() >> 10) & 1 != 0)
        {
            todo!("S2Translate TLB cache");
        }
        // if (__tlb_enabled & new_desc != descriptor) & [mem_desc[10]] == 0b1 then {
        //     tlbrecord : TLBRecord = undefined;
        //     if [descriptor[10]] == 0b1 then {
        //         let tlbentry : TLBLine = S2TLBLookup(descpaddr.tlbcontext);
        //         assert(tlbentry.valid_name);
        //         tlbrecord = tlbentry.tlbrecord
        //     } else {
        //         tlbrecord.context = descpaddr.tlbcontext;
        //         tlbrecord.context.xs = walkstate.memattrs.xs;
        //         tlbrecord.context.level = walkstate.level;
        //         tlbrecord.context.includes_s2_name = true;
        //         tlbrecord.context.isd128 = walkparams.d128 == 0b1;
        //         tlbrecord.walkstate = walkstate;
        //         tlbrecord.blocksize = TranslationSize(walkparams.d128, walkparams.tgx, walkstate.level);
        //         if walkstate.contiguous == 0b1 then {
        //             tlbrecord.contigsize = ContiguousSize(walkparams.d128, walkparams.tgx, walkstate.level)
        //         } else {
        //             tlbrecord.contigsize = 0
        //         }
        //     };
        //     if walkparams.d128 == 0b1 then {
        //         tlbrecord.s2descriptor = mem_desc
        //     } else {
        //         tlbrecord.s2descriptor[63 .. 0] = mem_desc[63 .. 0];
        //         tlbrecord.s2descriptor[127 .. 64] = Zeros(64)
        //     };
        //     S2TLBCache(tlbrecord)
        // };

        if fault.statuscode != Fault::None {
            return None;
        }

        let ipa_64 = ipa.paddress.as_ref().unwrap().address;
        // Output Address
        let oa = self.stage_oa(ipa_64, walkparams.d128, walkparams.tgx, &walkstate);
        let s2_memattrs;
        if (accdesc.acctype == AccessType::TTW
            && matches!(walkstate.memattrs.memtype, MemType::Device(_))
            && !walkparams.ptw)
            || (accdesc.acctype == AccessType::IFETCH
                && (matches!(walkstate.memattrs.memtype, MemType::Device(_))
                    || self.read::<hcr_el2::ID>() != 0))
            || (accdesc.acctype != AccessType::IFETCH
                && walkstate.memattrs.memtype == MemType::Normal
                && !self.s2_dcache_enabled())
        {
            // Treat memory attributes as Normal Non-Cacheable
            s2_memattrs = MemoryAttributes::normal_nc(walkstate.memattrs.xs);
        } else {
            s2_memattrs = walkstate.memattrs;
        }
        if accdesc.ls64 && s2_memattrs.memtype == MemType::Normal {
            if s2_memattrs.inner.attrs != MemAttr::NC || s2_memattrs.outer.attrs != MemAttr::NC {
                fault.statuscode = Fault::Exclusive;
                return None;
            }
        }

        let s2aarch64 = true;
        let memattrs;
        if !walkparams.fwb {
            memattrs = MemoryAttributes::s2_combine_s1_mem_attrs(
                ipa.memattrs.unwrap(),
                s2_memattrs,
                s2aarch64,
                self.is_feat_impl(Feat::MTE2),
                self.is_feat_impl(Feat::MtePerm),
            );
        } else {
            memattrs = s2_memattrs;
        }

        let mut pa = AddressDescriptor::create(ipa.vaddress, Some(oa), memattrs);
        pa.s2fs1mro = Some(s2fs1mro);
        pa.mecid = Some(self.aarch64_s2_output_mecid(
            &walkparams,
            pa.paddress.unwrap().paspace,
            descriptor,
        ));
        return Some(pa);
    }

    fn aarch64_setting_access_flag_permitted(&mut self, fault: &FaultRecord) -> bool {
        if fault.statuscode == Fault::None {
            return true;
        } else if fault.statuscode == Fault::Alignment || fault.statuscode == Fault::Permission {
            return true;
        } else {
            return false;
        }
    }

    fn aarch64_setting_dirty_state_permitted(&mut self, fault: &FaultRecord) -> bool {
        if fault.statuscode == Fault::None {
            return true;
        } else if fault.statuscode == Fault::Alignment {
            return true;
        } else {
            return false;
        }
    }

    pub fn aarch64_translate_address(
        &mut self,
        va: u64,
        accdesc: AccessDescriptor,
        aligned: bool,
        _size: i64,
    ) -> AddressDescriptor {
        //    if (SPESampleInFlight && !(accdesc.acctype IN {AccessType_IFETCH,
        //                                                    AccessType_SPE})) then
        //         SPEStartCounter(SPECounterPosTranslationLatency);

        let result = self.aarch64_full_translate(va, accdesc, aligned);

        //     if !IsFault(result) && accdesc.acctype != AccessType_IFETCH then
        //         result.fault = AArch64.CheckDebug(va, accdesc, size);

        //     if (IsFeatureImplemented(FEAT_RME) && !IsFault(result) &&
        //           (accdesc.acctype != AccessType_DC ||
        //            boolean IMPLEMENTATION_DEFINED "GPC Fault on DC operations")) then
        //         result.fault.gpcf = GranuleProtectionCheck(result, accdesc);
        //
        //         if result.fault.gpcf.gpf != GPCF_None then
        //             result.fault.statuscode = Fault_GPCFOnOutput;
        //             result.fault.paddress   = result.paddress;
        //
        //     if !IsFault(result) && accdesc.acctype == AccessType_IFETCH then
        //         result.fault = AArch64.CheckDebug(va, accdesc, size);
        //
        //     if (SPESampleInFlight && !(accdesc.acctype IN {AccessType_IFETCH,
        //                                                    AccessType_SPE})) then
        //         SPEStopCounter(SPECounterPosTranslationLatency);
        //
        //     // Update virtual address for abort functions
        //     result.vaddress = ZeroExtend(va, 64);

        return result;
    }
}
