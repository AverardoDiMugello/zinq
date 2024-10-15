use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_mair_attr(&mut self, index: u64, mair2: u64, mair: u64) -> u64 {
        assert!(index < 8 || (self.is_feat_impl(Feat::AIE) && (index < 16)));
        if index > 7 {
            (mair2 >> ((index - 8) * 8)) & 0b1111_1111
        } else {
            (mair >> (index * 8)) & 0b1111_1111
        }
    }

    pub fn aarch64_s1_initial_ttw_state(
        &mut self,
        walkparams: &S1TTWParams,
        va: u64,
        regime: Regime,
        ss: SecurityState,
    ) -> TTWState {
        let ttbr = self.aarch64_s1_ttbr(regime, va);

        let tablebase = FullAddress {
            address: self.aarch64_s1_tt_base_address(&walkparams, regime, ttbr),
            paspace: match ss {
                SecurityState::Secure => PASpace::Secure,
                SecurityState::NonSecure => PASpace::NonSecure,
                SecurityState::Root => PASpace::Root,
                SecurityState::Realm => PASpace::Realm,
            },
        };

        // permissions.ap_table = '00';
        // if HasUnprivileged(regime) then
        //     permissions.uxn_table = '0';
        //     permissions.pxn_table = '0';
        // else
        //     permissions.xn_table  = '0';
        // this is just the default in both cases...
        let permissions = Permissions::default();
        let s1assured;
        if regime == Regime::EL10
            && self.el2_enabled()
            && (self.read::<hcr_el2::VM>() != 0 || self.read::<hcr_el2::DC>() != 0)
        {
            let varange = VARange::aarch64_from_va(va);
            if (varange == VARange::Lower && self.read::<vtcr_el2::TL0>() != 0)
                || (varange == VARange::Upper && self.read::<vtcr_el2::TL1>() != 0)
            {
                s1assured = true;
            } else {
                s1assured = false;
            }
        } else {
            s1assured = false;
        };

        return TTWState {
            istable: true,
            level: walkparams.aarch64_s1_start_level(),
            baseaddress: tablebase,
            s1assured,
            disch: walkparams.disch,
            ng: regime.has_unprivileged(),
            memattrs: MemoryAttributes::walk_mem_attrs(
                walkparams.sh,
                walkparams.irgn,
                walkparams.orgn,
            ),
            permissions,
            ..Default::default()
        };
    }

    pub fn aarch64_s1_next_walk_state_leaf(
        &mut self,
        currentstate: TTWState,
        s2fs1mro: bool,
        regime: Regime,
        ss: SecurityState,
        walkparams: &S1TTWParams,
        descriptor: u128,
    ) -> TTWState {
        let address = self.aarch64_leaf_base(
            descriptor,
            walkparams.d128,
            walkparams.ds,
            walkparams.tgx,
            currentstate.level,
        );
        let mut paspace;
        if currentstate.baseaddress.paspace == PASpace::Secure {
            // Determine PA space of the next table from NS bit
            let ns = if walkparams.d128 {
                (descriptor >> 127) & 1 != 0
            } else {
                (descriptor >> 5) & 1 != 0
            };

            if !ns {
                paspace = PASpace::Secure;
            } else {
                paspace = PASpace::NonSecure;
            }
        } else if currentstate.baseaddress.paspace == PASpace::Root {
            // Determine PA space of the block from NSE and NS bits
            let (nse, ns) = if walkparams.d128 {
                ((descriptor >> 11) & 1 != 0, (descriptor >> 127) & 1 != 0)
            } else {
                ((descriptor >> 11) & 1 != 0, (descriptor >> 5) & 1 != 0)
            };
            paspace = PASpace::decode(nse, ns);

            // If Secure state is not implemented, but RME is,
            // force Secure space accesses to Non-secure space
            if paspace == PASpace::Secure && !self.have_secure_state() {
                paspace = PASpace::NonSecure;
            }
        } else if currentstate.baseaddress.paspace == PASpace::Realm
            && (regime == Regime::EL2 || regime == Regime::EL20)
        {
            // Realm EL2 and EL2&0 regimes have a stage 1 NS bit
            let ns = if walkparams.d128 {
                (descriptor >> 127) & 1 != 0
            } else {
                (descriptor >> 5) & 1 != 0
            };

            if !ns {
                paspace = PASpace::Realm;
            } else {
                paspace = PASpace::NonSecure;
            }
        } else if currentstate.baseaddress.paspace == PASpace::Realm {
            paspace = PASpace::Realm;
        } else {
            paspace = PASpace::NonSecure;
        };

        let baseaddress = FullAddress { paspace, address };

        let attrindex = if walkparams.aie {
            if walkparams.d128 {
                (descriptor >> 2) & ((1 << (5 - 2 + 1)) - 1)
            } else {
                let desc_4_2 = (descriptor >> 2) & ((1 << (4 - 2 + 1)) - 1);
                let desc_59 = (descriptor >> 59) & 1;
                (desc_59 << 3) | desc_4_2
            }
        } else {
            (descriptor >> 2) & ((1 << (4 - 2 + 1)) - 1)
        } as u64;

        let sh = if walkparams.d128 {
            ((descriptor >> 8) & ((1 << (9 - 8 + 1)) - 1)) as u64
        } else if walkparams.ds {
            walkparams.sh
        } else {
            ((descriptor >> 8) & ((1 << (9 - 8 + 1)) - 1)) as u64
        };

        let attr = self.aarch64_mair_attr(attrindex, walkparams.mair2, walkparams.mair);

        let protectedbit = if walkparams.d128 {
            (descriptor >> 114) & 1 != 0
        } else {
            if walkparams.pnch {
                (descriptor >> 52) & 1 != 0
            } else {
                false
            }
        };

        TTWState {
            istable: false,
            level: currentstate.level,
            baseaddress,
            contiguous: if walkparams.pnch || currentstate.disch {
                Some(false)
            } else {
                Some(walkparams.tgx.aarch64_contiguous_bit(
                    walkparams.d128,
                    currentstate.level,
                    descriptor,
                ))
            },
            s1assured: currentstate.s1assured && s2fs1mro && protectedbit,
            ng: if regime.has_unprivileged() {
                false
            } else if ss == SecurityState::Secure
                && currentstate.baseaddress.paspace == PASpace::NonSecure
            {
                // In Secure state, a translation must be treated as non-global,
                // regardless of the value of the nG bit,
                // if NSTable is set to 1 at any level of the translation table walk
                true
            } else if walkparams.fng {
                // Translations are treated as non-global regardless of the value of the nG bit.
                true
            } else {
                (descriptor >> 11) & 1 != 0
            },
            guardedpage: if walkparams.d128 {
                (descriptor >> 113) & 1 != 0
            } else {
                (descriptor >> 50) & 1 != 0
            },
            memattrs: MemoryAttributes::s1_decode_mem_attrs(attr, sh, true, walkparams.mtx),
            permissions: self.aarch64_s1_apply_output_perms(
                currentstate.permissions,
                descriptor,
                regime,
                walkparams,
            ),
            ..Default::default()
        }
    }

    pub fn aarch64_s1_next_walk_state_table(
        &mut self,
        currentstate: TTWState,
        s2fs1mro: bool,
        regime: Regime,
        walkparams: &S1TTWParams,
        descriptor: u128,
    ) -> TTWState {
        let skl = if walkparams.d128 {
            (descriptor >> 109) & 0b11
        } else {
            0b00
        } as i64;

        let address = self.aarch64_next_table_base(
            descriptor,
            walkparams.d128,
            skl,
            walkparams.ds,
            walkparams.tgx,
        );
        let paspace = if currentstate.baseaddress.paspace == PASpace::Secure {
            // Determine PA space of the next table from NSTable bit
            let nstable = if walkparams.d128 {
                (descriptor >> 127) & 1 != 0
            } else {
                (descriptor >> 63) & 1 != 0
            };

            if !nstable {
                PASpace::Secure
            } else {
                PASpace::NonSecure
            }
        } else {
            // Otherwise bit 63 is RES0 and there is no NSTable bit
            currentstate.baseaddress.paspace
        };
        let tablebase = FullAddress { paspace, address };

        let protectedbit = if walkparams.d128 {
            (descriptor >> 114) & 1 != 0
        } else {
            if walkparams.pnch {
                (descriptor >> 52) & 1 != 0
            } else {
                false
            }
        };

        TTWState {
            istable: true,
            level: if walkparams.d128 {
                currentstate.level + skl + 1
            } else {
                currentstate.level + 1
            },
            baseaddress: tablebase,
            s1assured: currentstate.s1assured && s2fs1mro && protectedbit,
            disch: if walkparams.d128 {
                (descriptor >> 112) & 1 != 0
            } else {
                false
            },
            ng: currentstate.ng,
            memattrs: currentstate.memattrs,
            permissions: if !walkparams.hpd && !walkparams.pie {
                self.aarch64_s1_apply_table_perms(
                    currentstate.permissions,
                    descriptor,
                    regime,
                    walkparams,
                )
            } else {
                currentstate.permissions
            },
            ..Default::default()
        }
    }

    pub fn aarch64_s1_walk(
        &mut self,
        walkparams: &S1TTWParams,
        va: u64,
        regime: Regime,
        n: usize,
        fault: &mut FaultRecord,
    ) -> Option<(Option<AddressDescriptor>, TTWState, u128)> {
        let mut tlbcontext = None;
        let mut tlbrecord = None;
        if self.cpu.tlb_enabled {
            tlbcontext = Some(self.aarch64_get_s1_tlbcontext(
                regime,
                fault.accessdesc.as_ref().unwrap().ss,
                va,
                walkparams.tgx,
            ));
            if let Some(tlbentry) = self.s1_tlb_lookup(&tlbcontext.as_ref().unwrap()) {
                let valid_name = tlbentry.valid_name;
                tlbrecord = Some(tlbentry.tlbrecord.clone()); // Making this an option prevents having to initialize a default plus the tlb entry each time

                let tlbrecord = tlbrecord.as_ref().unwrap();
                let oa = self.stage_oa(va, walkparams.d128, walkparams.tgx, &tlbrecord.walkstate);

                if valid_name
                    && !(self.aarch64_oa_out_of_range(
                        oa.address,
                        walkparams.d128,
                        walkparams.ps,
                        walkparams.tgx,
                    ))
                    && !(((((walkparams.ha && walkparams.hd)
                        && (walkparams.pie || ((tlbrecord.s1descriptor >> 51) & 1 != 0)))
                        & ((tlbrecord.s1descriptor >> 7) & 1 != 0))
                        & fault.accessdesc.as_ref().unwrap().write)
                        & !(fault.accessdesc.as_ref().unwrap().acctype == AccessType::AT
                            || fault.accessdesc.as_ref().unwrap().acctype == AccessType::IC
                            || fault.accessdesc.as_ref().unwrap().acctype == AccessType::DC))
                {
                    // println!("TLB hit! 0x{0:x}.", oa.address);
                    fault.level = tlbrecord.walkstate.level;
                    // assert(constraint((0 <= 'N - 1 & 'N - 1 < 128)));
                    // return((fault, __UNKNOWN_AddressDescriptor(), tlbrecord.walkstate, tlbrecord.s1descriptor[N - 1 .. 0]))
                    let mut descriptor = tlbrecord.s1descriptor;
                    if n == 64 {
                        descriptor &= u64::MAX as u128;
                    }
                    return Some((None, tlbrecord.walkstate.clone(), descriptor));
                }
            }
        }

        if regime.has_unprivileged() && self.aarch64_s1_epd(regime, va) {
            panic!("regime.has_unprivileged() && self.aarch64_s1_epd(regime, va)");
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        let mut walkstate = self.aarch64_s1_initial_ttw_state(
            &walkparams,
            va,
            regime,
            fault.accessdesc.as_ref().unwrap().ss,
        );
        // println!(
        //     "aarch64_s1_initial_ttw_state walkstate.baseaddress = 0x{0:x}",
        //     walkstate.baseaddress.address
        // );

        let startlevel = walkstate.level;

        if startlevel > 3 {
            panic!("startlevel > 3");
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        let mut descriptor;
        let wa_vaddress = va;
        let wa_mecid = self.aarch64_tt_walk_mecid(
            walkparams.emec,
            regime,
            fault.accessdesc.as_ref().unwrap().ss,
        );

        let wa_memattrs = if !self.aarch64_s1_dcache_enabled(regime) {
            MemoryAttributes::normal_nc(walkstate.memattrs.xs)
        } else {
            walkstate.memattrs.clone()
        };

        let mut walkaddress = AddressDescriptor::create(wa_vaddress, None, wa_memattrs);
        walkaddress.mecid = Some(wa_mecid);

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
            walkaddress.memattrs.as_mut().unwrap().shareability = walkstate.memattrs.shareability;
        } else {
            walkaddress.memattrs.as_mut().unwrap().shareability = walkaddress
                .memattrs
                .as_ref()
                .unwrap()
                .effective_shareability();
        };
        walkaddress.tlbcontext = tlbcontext.clone(); // From Sail for __tlb_enabled
        let mut s2fs1mro = false;
        let mut desctype;
        let mut descaddress = self.aarch64_s1_sl_tt_entry_address(
            walkstate.level,
            walkparams,
            va,
            &walkstate.baseaddress,
        );
        // println!(
        //     "aarch64_s1_sl_tt_entry_address descaddress.address = 0x{0:x}",
        //     descaddress.address
        // );

        // Detect Address Size Fault by Descriptor Address
        if self.aarch64_oa_out_of_range(
            descaddress.address,
            walkparams.d128,
            walkparams.ps,
            walkparams.tgx,
        ) {
            fault.statuscode = Fault::AddressSize;
            fault.level = 0;
            return None;
        }

        loop {
            fault.level = walkstate.level;
            walkaddress.paddress = Some(descaddress.clone());
            walkaddress.s1assured = walkstate.s1assured;

            let toplevel = walkstate.level == startlevel;
            let varange = VARange::aarch64_from_va(va);
            let walkaccess = AccessDescriptor::create_s1_ttw(
                toplevel,
                varange,
                fault.accessdesc.as_ref().unwrap(),
            );
            let mut s2fault = FaultRecord::no_fault();

            let fetch_descriptor_result;
            if regime == Regime::EL10 && self.el2_enabled() {
                let s2walkaddress =
                    self.aarch64_s2_translate(&mut s2fault, &walkaddress, true, true, &walkaccess);

                if s2fault.statuscode != Fault::None {
                    *fault = s2fault;
                    return None;
                }

                let s2walkaddress = s2walkaddress.unwrap();
                s2fs1mro = s2walkaddress.s2fs1mro.unwrap();
                fetch_descriptor_result =
                    self.fetch_descriptor(walkparams.ee, &s2walkaddress, &walkaccess, fault, n);
            } else {
                fetch_descriptor_result =
                    self.fetch_descriptor(walkparams.ee, &walkaddress, &walkaccess, fault, n);
            }

            if fault.statuscode != Fault::None {
                return None;
            }
            descriptor = fetch_descriptor_result.unwrap();

            let mut new_descriptor;
            loop {
                new_descriptor = descriptor;
                desctype = self.aarch64_decode_descriptor_type(
                    descriptor,
                    walkparams.d128,
                    walkparams.ds,
                    walkparams.tgx,
                    walkstate.level,
                );

                match desctype {
                    DescriptorType::Table => {
                        walkstate = self.aarch64_s1_next_walk_state_table(
                            walkstate, s2fs1mro, regime, walkparams, descriptor,
                        );

                        let skl = if walkparams.d128 {
                            (descriptor >> 109) & 0b11
                        } else {
                            0b00
                        } as i64;

                        descaddress = self.aarch64_tt_entry_address(
                            walkstate.level,
                            walkparams.d128,
                            skl,
                            walkparams.tgx,
                            walkparams.txsz,
                            va,
                            &walkstate.baseaddress,
                        );

                        // Detect Address Size Fault by Descriptor Address
                        if self.aarch64_oa_out_of_range(
                            descaddress.address,
                            walkparams.d128,
                            walkparams.ps,
                            walkparams.tgx,
                        ) {
                            fault.statuscode = Fault::AddressSize;
                            return None;
                        }

                        if walkparams.haft {
                            // new_descriptor<10> = '1';
                            new_descriptor |= 1 << 10;
                        }

                        if walkparams.d128
                            && skl != 0b00
                            && self.aarch64_block_nt_faults(walkparams.d128, descriptor)
                        {
                            panic!(
                                "walkparams.d128
                            && skl != 0b00
                            && self.aarch64_block_nt_faults(walkparams.d128, descriptor)"
                            );
                            fault.statuscode = Fault::Translation;
                            return None;
                        }
                    }
                    DescriptorType::Leaf => {
                        walkstate = self.aarch64_s1_next_walk_state_leaf(
                            walkstate,
                            s2fs1mro,
                            regime,
                            fault.accessdesc.as_ref().unwrap().ss,
                            walkparams,
                            descriptor,
                        );
                    }
                    DescriptorType::Invalid => {
                        panic!("Invalid descriptor!");
                        fault.statuscode = Fault::Translation;
                        return None;
                    }
                };

                if new_descriptor != descriptor {
                    let descaccess =
                        AccessDescriptor::create_tte_update(fault.accessdesc.as_ref().unwrap());

                    let descpaddr;
                    if regime == Regime::EL10 && self.el2_enabled() {
                        descpaddr =
                            self.aarch64_s2_translate(fault, &walkaddress, true, true, &descaccess);

                        if fault.statuscode != Fault::None {
                            return None;
                        }
                    } else {
                        descpaddr = Some(walkaddress.clone());
                    }

                    let descriptor_result = self.aarch64_mem_swap_table_desc(
                        fault,
                        descriptor,
                        new_descriptor,
                        walkparams.ee,
                        &descaccess,
                        descpaddr.as_ref().unwrap(),
                        n,
                    );

                    if fault.statuscode != Fault::None {
                        return None;
                    }

                    descriptor = descriptor_result.unwrap();
                }

                if new_descriptor == descriptor {
                    break;
                }
            }

            if desctype == DescriptorType::Leaf {
                break;
            }
        }

        let oa = self.stage_oa(va, walkparams.d128, walkparams.tgx, &walkstate);

        if walkstate.contiguous.is_some_and(|val| val == true)
            && self.aarch64_contiguous_bit_faults(
                walkparams.d128,
                walkparams.txsz,
                walkparams.tgx,
                walkstate.level,
            )
        {
            panic!(
                "walkstate.contiguous.is_some_and(|val| val == true)
            && self.aarch64_contiguous_bit_faults(
                walkparams.d128,
                walkparams.txsz,
                walkparams.tgx,
                walkstate.level,
            )"
            );
            fault.statuscode = Fault::Translation;
        } else if desctype == DescriptorType::Leaf
            && walkstate.level < 3
            && self.aarch64_block_nt_faults(walkparams.d128, descriptor)
        {
            panic!(
                "desctype == DescriptorType::Leaf
            && walkstate.level < 3
            && self.aarch64_block_nt_faults(walkparams.d128, descriptor)"
            );
            fault.statuscode = Fault::Translation;
        } else if self.aarch64_s1_amec_fault(
            walkparams,
            walkstate.baseaddress.paspace,
            regime,
            descriptor,
        ) {
            panic!(
                "self.aarch64_s1_amec_fault(
            walkparams,
            walkstate.baseaddress.paspace,
            regime,
            descriptor,
        )"
            );
            fault.statuscode = Fault::Translation;
        }
        // Detect Address Size Fault by final output
        else if self.aarch64_oa_out_of_range(
            oa.address,
            walkparams.d128,
            walkparams.ps,
            walkparams.tgx,
        ) {
            fault.statuscode = Fault::AddressSize;
        }
        // Check descriptor AF bit
        else if (descriptor >> 10) & 1 == 0
            && !walkparams.ha
            && !(matches!(
                fault.accessdesc.as_ref().unwrap().acctype,
                AccessType::DC | AccessType::IC
            ) && !self
                .cpu
                .impdef
                .bool(&"Generate access flag fault on IC/DC operations"))
        {
            fault.statuscode = Fault::AccessFlag;
        }

        if fault.statuscode != Fault::None {
            return None;
        }

        if self.cpu.tlb_enabled
            && fault.statuscode == Fault::None
            && ((descriptor >> 10) & 0b1 != 0)
        {
            let mut tlbcontext = tlbcontext.unwrap_or_default();
            let mut tlbrecord = tlbrecord.unwrap_or_default();

            tlbcontext.xs = walkstate.memattrs.xs;
            tlbcontext.level = walkstate.level;
            tlbcontext.ng = walkstate.ng;
            tlbcontext.isd128 = walkparams.d128;
            tlbrecord.context = tlbcontext;
            tlbrecord.walkstate = walkstate.clone();
            tlbrecord.blocksize = walkparams
                .tgx
                .translation_size(walkparams.d128, walkstate.level);
            if walkparams.d128 {
                // assert(constraint(127 < 'N));
                // tlbrecord.s1descriptor = descriptor[127..0]
                tlbrecord.s1descriptor = descriptor;
            } else {
                // assert(constraint(63 < 'N));
                // tlbrecord.s1descriptor[63..0] = descriptor[63..0];
                // tlbrecord.s1descriptor[127..64] = Zeros(64)
                tlbrecord.s1descriptor = descriptor & !((u64::MAX as u128) << 64);
            };
            if walkstate.contiguous.is_some_and(|val| val) {
                tlbrecord.contigsize = walkparams
                    .tgx
                    .contiguous_size(walkparams.d128, walkstate.level);
            } else {
                tlbrecord.contigsize = 0;
            };

            // println!("TLB S1Walk fill");
            self.s1_tlb_cache(tlbrecord);
        }

        if n == 64 {
            descriptor &= u64::MAX as u128;
        }

        return Some((Some(walkaddress), walkstate, descriptor));
    }

    pub fn aarch64_s2_initial_ttw_state(
        &mut self,
        ss: SecurityState,
        walkparams: &S2TTWParams,
    ) -> TTWState {
        let ttbr = self.read::<VTTBR_EL2>() as u128;
        let paspace = match ss {
            SecurityState::NonSecure => PASpace::NonSecure,
            SecurityState::Realm => PASpace::Realm,
            _ => panic!("Unreachable"),
        };
        let tablebase = FullAddress {
            paspace,
            address: self.aarch64_s2_tt_base_address(walkparams, paspace, ttbr),
        };

        return TTWState {
            baseaddress: tablebase,
            level: walkparams.aarch64_s2_start_level(),
            istable: true,
            memattrs: MemoryAttributes::walk_mem_attrs(
                walkparams.sh,
                walkparams.irgn,
                walkparams.orgn,
            ),
            ..Default::default()
        };
    }

    pub fn aarch64_s2_next_walk_state_leaf(
        &mut self,
        currentstate: TTWState,
        ss: SecurityState,
        walkparams: &S2TTWParams,
        ipa: &AddressDescriptor,
        descriptor: u128,
    ) -> TTWState {
        let baseaddress = FullAddress {
            paspace: if ss == SecurityState::Secure {
                self.aarch64_ss2_output_paspace(walkparams, ipa.paddress.unwrap().paspace)
            } else if ss == SecurityState::Realm {
                let ns = if walkparams.d128 {
                    (descriptor >> 127) & 1 != 0
                } else {
                    (descriptor >> 55) & 1 != 0
                };
                if ns {
                    PASpace::NonSecure
                } else {
                    PASpace::Realm
                }
            } else {
                PASpace::NonSecure
            },
            address: self.aarch64_leaf_base(
                descriptor,
                walkparams.d128,
                walkparams.ds,
                walkparams.tgx,
                currentstate.level,
            ),
        };

        let istable = false;
        let level = currentstate.level;
        let baseaddress = baseaddress;
        let mut permissions = self.aarch64_s2_apply_output_perms(descriptor, walkparams);

        let mut memattrs;
        let s2_attr = ((descriptor >> 2) & 0b1111) as u64;
        let s2_sh = if walkparams.ds {
            walkparams.sh
        } else {
            ((descriptor >> 8) & 0b11) as u64
        };
        let s2_fnxs = (descriptor >> 11) & 1 != 0;
        if walkparams.fwb {
            memattrs = MemoryAttributes::aarch64_s2_apply_fwb_mem_attrs(
                ipa.memattrs.as_ref().unwrap(),
                walkparams,
                descriptor,
                self.is_feat_impl(Feat::MTE2),
            );
            if (s2_attr >> 1) & 0b111 == 0b111 {
                permissions.s2tag_na = true;
            } else {
                permissions.s2tag_na = false;
            }
        } else {
            let s2aarch64 = true;
            memattrs = MemoryAttributes::s2_decode_mem_attrs(
                s2_attr,
                s2_sh,
                s2aarch64,
                self.is_feat_impl(Feat::MtePerm),
            );
            // FnXS is used later to mask the XS value from stage 1
            memattrs.xs = !s2_fnxs;
            if s2_attr == 0b0100 {
                permissions.s2tag_na = true;
            } else {
                permissions.s2tag_na = false;
            }
        }
        let contiguous = Some(walkparams.tgx.aarch64_contiguous_bit(
            walkparams.d128,
            currentstate.level,
            descriptor,
        ));

        let s2assuredonly;
        if walkparams.d128 {
            s2assuredonly = (descriptor >> 114) & 1 != 0;
        } else {
            s2assuredonly = if walkparams.assuredonly {
                (descriptor >> 58) & 1 != 0
            } else {
                false
            };
        }

        return TTWState {
            istable,
            level,
            baseaddress,
            permissions,
            memattrs,
            s2assuredonly,
            contiguous,
            ..Default::default()
        };
    }

    pub fn aarch64_s2_next_walk_state_table(
        &mut self,
        currentstate: TTWState,
        walkparams: &S2TTWParams,
        descriptor: u128,
    ) -> TTWState {
        let skl = if walkparams.d128 {
            (descriptor >> 109) & 0b11
        } else {
            0b00
        };

        let tablebase = FullAddress {
            paspace: currentstate.baseaddress.paspace,
            address: self.aarch64_next_table_base(
                descriptor,
                walkparams.d128,
                skl as i64,
                walkparams.ds,
                walkparams.tgx,
            ),
        };

        return TTWState {
            istable: true,
            level: if walkparams.d128 {
                currentstate.level + (skl as i64) + 1
            } else {
                currentstate.level + 1
            },
            baseaddress: tablebase,
            memattrs: currentstate.memattrs,
            ..Default::default()
        };
    }

    pub fn aarch64_s2_walk(
        &mut self,
        fault: &mut FaultRecord,
        ipa: &AddressDescriptor,
        walkparams: &S2TTWParams,
        accdesc: &AccessDescriptor,
        n: usize,
    ) -> Option<(AddressDescriptor, TTWState, u128)> {
        let ipa_64 = ipa.paddress.unwrap().address;

        if self.cpu.tlb_enabled {
            todo!("S2Walk TLB lookup");
        }
        // tlbcontext : TLBContext = undefined;
        // tlbrecord : TLBRecord = undefined;
        // if __tlb_enabled then {
        //     tlbcontext = AArch64_GetS2TLBContext(accdesc.ss, ipa.paddress, walkparams.tgx);
        //     let tlbentry : TLBLine = S2TLBLookup(tlbcontext);
        //     tlbrecord = tlbentry.tlbrecord;
        //     let oa : FullAddress = StageOA(ipa_64, walkparams.d128, walkparams.tgx, tlbrecord.walkstate);
        //     if (tlbentry.valid_name & not_bool(AArch64_OAOutOfRange(oa.address, walkparams.d128, walkparams.ps, walkparams.tgx))) & not_bool(((((walkparams.ha @ walkparams.hd) == 0b11 & (walkparams.s2pie == 0b1 | [tlbrecord.s2descriptor[51]] == 0b1)) & [tlbrecord.s2descriptor[7]] == 0b0) & accdesc.write) & not_bool(accdesc.acctype == AccessType_AT | accdesc.acctype == AccessType_IC | accdesc.acctype == AccessType_DC)) then {
        //         if walkparams.fwb == 0b1 then {
        //             tlbrecord.walkstate.memattrs = AArch64_S2ApplyFWBMemAttrs(ipa.memattrs, walkparams, tlbrecord.s2descriptor);
        //             if tlbrecord.s2descriptor[5 .. 3] == 0b111 then {
        //                 tlbrecord.walkstate.permissions.s2tag_na = 0b1
        //             } else {
        //                 tlbrecord.walkstate.permissions.s2tag_na = 0b0
        //             }
        //         };
        //         fault.level = tlbrecord.walkstate.level;
        //         return((fault, __UNKNOWN_AddressDescriptor(), tlbrecord.walkstate, tlbrecord.s2descriptor[N - 1 .. 0]))
        //     };
        //     ()
        // };

        let mut walkstate;
        if accdesc.ss == SecurityState::Secure {
            walkstate = self
                .aarch64_ss2_initial_ttw_state(walkparams, ipa.paddress.as_ref().unwrap().paspace);
        } else {
            walkstate = self.aarch64_s2_initial_ttw_state(accdesc.ss, walkparams);
        }
        let startlevel = walkstate.level;

        if startlevel > 3 {
            panic!("startlevel > 3");
            fault.statuscode = Fault::Translation;
            fault.level = 0;
            return None;
        }

        let mut descriptor: u128;
        let walkaccess = AccessDescriptor::create_s2_ttw(accdesc);
        let mut walkaddress: AddressDescriptor;
        let mut skl;

        walkaddress = AddressDescriptor {
            vaddress: ipa.vaddress,
            mecid: Some(self.aarch64_tt_walk_mecid(walkparams.emec, Regime::EL10, accdesc.ss)),

            memattrs: if !self.s2_dcache_enabled() {
                Some(MemoryAttributes::normal_nc(walkstate.memattrs.xs))
            } else {
                Some(walkstate.memattrs)
            },
            paddress: None,
            fault: FaultRecord::no_fault(),
            s1assured: false,
            s2fs1mro: None,
            tlbcontext: None,
        };

        walkaddress.memattrs.as_mut().unwrap().shareability =
            walkaddress.memattrs.unwrap().effective_shareability();

        let mut desctype: DescriptorType;

        // Initial lookup might index into concatenated tables
        let mut descaddress = self.arch64_s2_sl_tt_entry_address(
            walkparams,
            ipa.paddress.unwrap().address,
            &walkstate.baseaddress,
        );

        // Detect Address Size Fault by Descriptor Address
        if self.aarch64_oa_out_of_range(
            descaddress.address,
            walkparams.d128,
            walkparams.ps,
            walkparams.tgx,
        ) {
            fault.statuscode = Fault::AddressSize;
            fault.level = 0;
            return None;
        }

        loop {
            fault.level = walkstate.level;
            walkaddress.paddress = Some(descaddress);
            let descriptor_result =
                self.fetch_descriptor(walkparams.ee, &walkaddress, &walkaccess, fault, n);

            if fault.statuscode != Fault::None {
                return None;
            }
            descriptor = descriptor_result.unwrap();

            let mut new_descriptor;
            loop {
                new_descriptor = descriptor;
                desctype = self.aarch64_decode_descriptor_type(
                    descriptor,
                    walkparams.d128,
                    walkparams.ds,
                    walkparams.tgx,
                    walkstate.level,
                );

                match desctype {
                    DescriptorType::Table => {
                        walkstate = self
                            .aarch64_s2_next_walk_state_table(walkstate, walkparams, descriptor);

                        skl = if walkparams.d128 {
                            (descriptor >> 109) & 0b11
                        } else {
                            0b00
                        } as u64;

                        descaddress = self.aarch64_tt_entry_address(
                            walkstate.level,
                            walkparams.d128,
                            skl as i64,
                            walkparams.tgx,
                            walkparams.txsz,
                            ipa_64,
                            &walkstate.baseaddress,
                        );

                        // Detect Address Size Fault by table descriptor
                        if self.aarch64_oa_out_of_range(
                            descaddress.address,
                            walkparams.d128,
                            walkparams.ps,
                            walkparams.tgx,
                        ) {
                            fault.statuscode = Fault::AddressSize;
                            return None;
                        }

                        if walkparams.haft {
                            new_descriptor |= 1 << 10;
                        }

                        if walkparams.d128
                            && skl != 0b00
                            && self.aarch64_block_nt_faults(walkparams.d128, descriptor)
                        {
                            panic!(
                                "walkparams.d128
                            && skl != 0b00
                            && self.aarch64_block_nt_faults(walkparams.d128, descriptor)"
                            );
                            fault.statuscode = Fault::Translation;
                            return None;
                        }
                    }
                    DescriptorType::Leaf => {
                        walkstate = self.aarch64_s2_next_walk_state_leaf(
                            walkstate, accdesc.ss, walkparams, &ipa, descriptor,
                        );
                    }
                    DescriptorType::Invalid => {
                        panic!("Invalid S2descriptor");
                        fault.statuscode = Fault::Translation;
                        return None;
                    }
                }

                if new_descriptor != descriptor {
                    let descaccess: AccessDescriptor =
                        AccessDescriptor::create_tte_update(&accdesc);
                    let descriptor_result = self.aarch64_mem_swap_table_desc(
                        fault,
                        descriptor,
                        new_descriptor,
                        walkparams.ee,
                        &descaccess,
                        &walkaddress,
                        n,
                    );

                    if fault.statuscode != Fault::None {
                        return None;
                    }
                    descriptor = descriptor_result.unwrap();
                }

                if new_descriptor == descriptor {
                    break;
                }
            }

            if desctype == DescriptorType::Leaf {
                break;
            }
        }
        let oa = self.stage_oa(ipa_64, walkparams.d128, walkparams.tgx, &walkstate);

        if walkstate.contiguous.is_some_and(|val| val == true)
            && self.aarch64_contiguous_bit_faults(
                walkparams.d128,
                walkparams.txsz,
                walkparams.tgx,
                walkstate.level,
            )
        {
            panic!(
                "walkstate.contiguous.is_some_and(|val| val == true)
            && self.aarch64_contiguous_bit_faults(
                walkparams.d128,
                walkparams.txsz,
                walkparams.tgx,
                walkstate.level,
            )"
            );
            fault.statuscode = Fault::Translation;
        } else if desctype == DescriptorType::Leaf
            && walkstate.level < 3
            && self.aarch64_block_nt_faults(walkparams.d128, descriptor)
        {
            panic!(
                "desctype == DescriptorType::Leaf
            && walkstate.level < 3
            && self.aarch64_block_nt_faults(walkparams.d128, descriptor)"
            );
            fault.statuscode = Fault::Translation;
        }
        // Detect Address Size Fault by final output
        else if self.aarch64_oa_out_of_range(
            oa.address,
            walkparams.d128,
            walkparams.ps,
            walkparams.tgx,
        ) {
            fault.statuscode = Fault::AddressSize;
        }
        // Check descriptor AF bit
        else if (descriptor >> 10) & 1 == 0
            && !walkparams.ha
            && !(matches!(accdesc.acctype, AccessType::DC | AccessType::IC)
                && !self
                    .cpu
                    .impdef
                    .bool(&"Generate access flag fault on IC/DC operations"))
        {
            fault.statuscode = Fault::AccessFlag;
        }

        if self.cpu.tlb_enabled && fault.statuscode == Fault::None && ((descriptor >> 10) & 1 != 0)
        {
            todo!("S2Walk TLB cache");
        }

        // if (__tlb_enabled & fault.statuscode == Fault_None) & [descriptor[10]] == 0b1 then {
        //     tlbcontext.xs = walkstate.memattrs.xs;
        //     tlbcontext.level = walkstate.level;
        //     tlbcontext.isd128 = walkparams.d128 == 0b1;
        //     tlbrecord.context = tlbcontext;
        //     tlbrecord.walkstate = walkstate;
        //     tlbrecord.blocksize = TranslationSize(walkparams.d128, walkparams.tgx, walkstate.level);
        //     if walkparams.d128 == 0b1 then {
        //         assert(constraint(127 < 'N));
        //         tlbrecord.s2descriptor = descriptor[127 .. 0]
        //     } else {
        //         assert(constraint(63 < 'N));
        //         tlbrecord.s2descriptor[63 .. 0] = descriptor[63 .. 0];
        //         tlbrecord.s2descriptor[127 .. 64] = Zeros(64)
        //     };
        //     if walkstate.contiguous == 0b1 then {
        //         tlbrecord.contigsize = ContiguousSize(walkparams.d128, walkparams.tgx, walkstate.level)
        //     } else {
        //         tlbrecord.contigsize = 0
        //     };
        //     S2TLBCache(tlbrecord)
        // };

        return Some((walkaddress, walkstate, descriptor));
    }

    pub fn aarch64_ss2_initial_ttw_state(
        &mut self,
        walkparams: &S2TTWParams,
        ipaspace: PASpace,
    ) -> TTWState {
        let ttbr;
        if ipaspace == PASpace::Secure {
            ttbr = self.read::<VSTTBR_EL2>() as u128;
        } else {
            ttbr = self.read::<VTTBR_EL2>() as u128;
        }

        let paspace = if ipaspace == PASpace::Secure {
            if !walkparams.sw {
                PASpace::Secure
            } else {
                PASpace::NonSecure
            }
        } else {
            if !walkparams.nsw {
                PASpace::Secure
            } else {
                PASpace::NonSecure
            }
        };
        let tablebase = FullAddress {
            paspace,
            address: self.aarch64_s2_tt_base_address(walkparams, paspace, ttbr),
        };

        return TTWState {
            baseaddress: tablebase,
            level: walkparams.aarch64_s2_start_level(),
            istable: true,
            memattrs: MemoryAttributes::walk_mem_attrs(
                walkparams.sh,
                walkparams.irgn,
                walkparams.orgn,
            ),
            ..Default::default()
        };
    }

    pub fn aarch64_ss2_output_paspace(
        &mut self,
        walkparams: &S2TTWParams,
        ipaspace: PASpace,
    ) -> PASpace {
        if ipaspace == PASpace::Secure {
            if !walkparams.sw && !walkparams.sa {
                return PASpace::Secure;
            } else {
                return PASpace::NonSecure;
            }
        } else {
            if !walkparams.sw && !walkparams.sa && !walkparams.nsw && !walkparams.nsa {
                return PASpace::Secure;
            } else {
                return PASpace::NonSecure;
            }
        }
    }
}
