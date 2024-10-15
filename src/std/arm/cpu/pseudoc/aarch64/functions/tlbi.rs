use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch32::*, aarch64::*, *};
use crate::std::arm::cpu::ArmCtx;

#[derive(Debug)]
pub enum TLBIOp {
    DALL, // AArch32 Data TLBI operations - deprecated
    DASID,
    DVA,
    IALL, // AArch32 Instruction TLBI operations - deprecated
    IASID,
    IVA,
    ALL,
    ASID,
    IPAS2,
    VAA,
    VA,
    VMALL,
    VMALLS12,
    RIPAS2,
    RVAA,
    RVA,
    RPA,
    PAALL,
    VMALLWS2,
    TLBIPOp(TLBIPOp),
}

#[derive(Debug)]
pub enum TLBIPOp {
    IPAS2,
    VAA,
    VA,
    RIPAS2,
    RVAA,
    RVA,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TLBILevel {
    Any,
    Last,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TLBIMemAttr {
    AllAttr,   // All TLB entries within the scope of the invalidation
    ExcludeXS, // Only TLB entries with XS=0 within the scope of the invalidation
}

#[derive(Debug)]
pub struct TLBIRecord {
    op: TLBIOp,
    from_aarch64: bool, // originated as an AArch64 operation
    security: SecurityState,
    regime: Regime,
    vmid: u16,
    asid: u16,
    level: TLBILevel,
    attr: TLBIMemAttr,
    ipaspace: PASpace, // For operations that take IPA as input address
    address: u64,      // input address, for range operations, start address
    end_address: u64,  // for range operations, end address
    d64: bool,         // For operations that evict VMSAv8-64 based TLB entries
    d128: bool,        // For operations that evict VMSAv9-128 based TLB entries
    // translation table walk level holding the leaf entry
    // for the address being invalidated
    // For Non-Range Invalidations:
    //   When the ttl is
    //     '00xx'    : this applies to all TLB entries
    //     Otherwise : TLBIP instructions invalidates D128 TLB
    //                 entries only
    //                 TLBI instructions invalidates D64 TLB
    //                 entries only
    // For Range Invalidations:
    //   When the ttl is
    //     '00'      : this applies to all TLB entries
    //     Otherwise : TLBIP instructions invalidates D128 TLB
    //                 entries only
    //                 TLBI instructions invalidates D64 TLB
    //                 entries only
    ttl: u64, // bits{4}
    tg: TGx,  // for range operations, translation granule
}

impl TLBIRecord {
    fn tlbi_match(
        &self,
        tlb_entry: &TLBRecord,
        use_vmid: bool,
        use_asid: bool,
        res_tlbi_ttl: bool,
    ) -> bool {
        let mut is_match;

        let entry_block_mask = (1 << tlb_entry.blocksize) - 1;
        let _entry_end_address = tlb_entry.context.ia | entry_block_mask;
        let _entry_start_address = tlb_entry.context.ia & (!entry_block_mask);

        match self.op {
            TLBIOp::VAA => {
                let addr_lsb = tlb_entry.blocksize;
                let tlbi_address_55_addr_lsb =
                    (self.address >> addr_lsb) & ((1 << (55 - addr_lsb + 1)) - 1);
                let tlb_entry_context_ia_55_addr_lsb =
                    (tlb_entry.context.ia >> addr_lsb) & ((1 << (55 - addr_lsb + 1)) - 1);
                is_match = tlb_entry.context.includes_s1
                    && self.security == tlb_entry.context.ss
                    && self.regime == tlb_entry.context.regime
                    && (!use_vmid || self.vmid == tlb_entry.context.vmid)
                    && tlbi_address_55_addr_lsb == tlb_entry_context_ia_55_addr_lsb
                    && (!self.from_aarch64
                        || res_tlbi_ttl
                        || (TGx::from_bits(
                            (self.ttl >> 3) & 0b1 != 0,
                            (self.ttl >> 2) & 0b1 != 0,
                        ) == tlb_entry.context.tg
                            && (self.ttl & 0b11) as i64 == tlb_entry.walkstate.level))
                    && ((self.d128 && tlb_entry.context.isd128)
                        || (self.d64 && !tlb_entry.context.isd128)
                        || (self.d64 && self.d128))
                    && (self.level == TLBILevel::Any || !tlb_entry.walkstate.istable);
            }
            TLBIOp::VA => {
                let addr_lsb = tlb_entry.blocksize;
                let tlbi_address_55_addr_lsb =
                    (self.address >> addr_lsb) & ((1 << (55 - addr_lsb + 1)) - 1);
                let tlb_entry_context_ia_55_addr_lsb =
                    (tlb_entry.context.ia >> addr_lsb) & ((1 << (55 - addr_lsb + 1)) - 1);

                is_match = tlb_entry.context.includes_s1
                    && self.security == tlb_entry.context.ss
                    && self.regime == tlb_entry.context.regime
                    && (!use_vmid || self.vmid == tlb_entry.context.vmid)
                    && (!use_asid || self.asid == tlb_entry.context.asid || !tlb_entry.context.ng)
                    && tlbi_address_55_addr_lsb == tlb_entry_context_ia_55_addr_lsb
                    && (!self.from_aarch64
                        || res_tlbi_ttl
                        || (TGx::from_bits(
                            (self.ttl >> 3) & 0b1 != 0,
                            (self.ttl >> 2) & 0b1 != 0,
                        ) == tlb_entry.context.tg
                            && (self.ttl & 0b11) as i64 == tlb_entry.walkstate.level))
                    && ((self.d128 && tlb_entry.context.isd128)
                        || (self.d64 && !tlb_entry.context.isd128)
                        || (self.d64 && self.d128))
                    && (self.level == TLBILevel::Any || !tlb_entry.walkstate.istable);
            }
            TLBIOp::VMALL => {
                is_match = tlb_entry.context.includes_s1
                    && self.security == tlb_entry.context.ss
                    && self.regime == tlb_entry.context.regime
                    && (!use_vmid || self.vmid == tlb_entry.context.vmid);
            }
            _ => todo!("{0:?}", self.op),
        }

        if self.attr == TLBIMemAttr::ExcludeXS && tlb_entry.context.xs {
            is_match = false;
        }

        return is_match;
    }
}

impl Default for TLBIRecord {
    fn default() -> Self {
        Self {
            op: TLBIOp::ALL,
            from_aarch64: false,
            security: SecurityState::NonSecure,
            regime: Regime::EL10,
            vmid: 0,
            asid: 0,
            level: TLBILevel::Any,
            attr: TLBIMemAttr::AllAttr,
            ipaspace: PASpace::NonSecure,
            address: 0,
            end_address: 0,
            d64: false,
            d128: false,
            ttl: 0,
            tg: TGx::TG4KB,
        }
    }
}

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn vmid(&mut self) -> u16 {
        if self.el2_enabled() {
            if !self.el_using_aarch32(EL::EL2) {
                if self.is_feat_impl(Feat::VMID16) && self.read::<vtcr_el2::VS>() != 0 {
                    return self.read::<vttbr_el2::VMID>() as u16;
                } else {
                    return (self.read::<vttbr_el2::VMID>() & 0b1111_1111) as u16;
                }
            } else {
                return self.read::<vttbr::VMID>() as u16;
            }
        } else if self.have_el(EL::EL2) && self.is_feat_impl(Feat::SEL2) {
            return 0;
        } else {
            return 0; // VMID_NONE is architecturally zero;
        }
    }

    pub fn res_tlbi_ttl(&mut self, ttl: u64) -> bool {
        match ttl {
            0b0100 => !self.is_feat_impl(Feat::LPA2),
            0b1000 => true,
            0b1001 => !self.is_feat_impl(Feat::LPA2),
            0b1100 => true,
            _ => ttl >> 2 == 0,
        }
    }

    pub fn tlbi(&mut self, r: TLBIRecord) {
        // println!("TLBI: {r:?}");
        let el2_enabled = self.el2_enabled();
        let res_tlbi_ttl = self.res_tlbi_ttl(r.ttl);
        self.ctx.tlb_flush(|page, entry| {
            let invalidate = r.tlbi_match(
                &entry.tlbrecord,
                entry.tlbrecord.context.use_vmid(el2_enabled),
                entry.tlbrecord.context.use_asid(),
                res_tlbi_ttl,
            );

            // if invalidate {
            //     println!(
            //         "Invalidated: 0x{page:x} -> 0x{0:x}",
            //         entry.tlbrecord.walkstate.baseaddress.address
            //     );
            // } else {
            //     println!(
            //         "Retained: 0x{page:x} -> 0x{0:x}",
            //         entry.tlbrecord.walkstate.baseaddress.address
            //     );
            // }
            invalidate
        });
    }

    pub fn aarch64_tlbi_vmall(
        &mut self,
        security: SecurityState,
        regime: Regime,
        vmid: u16,
        shareability: Shareability,
        attr: TLBIMemAttr,
    ) {
        assert_ne!(self.read::<pstate::EL>(), EL::EL0.as_num());

        let r = TLBIRecord {
            op: TLBIOp::VMALL,
            from_aarch64: true,
            security,
            regime,
            level: TLBILevel::Any,
            vmid,
            attr,
            ..Default::default()
        };

        self.tlbi(r);
        if shareability != Shareability::NSH {
            // TODO: Broadcast(shareability, r);
        }
        return;
    }

    pub fn aarch64_tlbi_vaa(
        &mut self,
        security: SecurityState,
        regime: Regime,
        vmid: u16,
        shareability: Shareability,
        level: TLBILevel,
        attr: TLBIMemAttr,
        xt: u64,
    ) {
        assert_ne!(self.read::<pstate::EL>(), EL::EL0.as_num());

        let r = TLBIRecord {
            op: TLBIOp::VAA,
            from_aarch64: true,
            security,
            regime,
            vmid,
            level,
            attr,
            ttl: (xt >> 44) & 0b1111,
            // ZeroExtend(Xt<43:0> : Zeros(12), 64)
            address: (xt & ((1 << 44) - 1)) << 12,
            d64: true,
            d128: ((xt >> 44) & 0b1111) >> 2 == 0,
            ..Default::default()
        };
        self.tlbi(r);
        if shareability != Shareability::NSH {
            // TODO: Broadcast(shareability, r);
        }
        return;
    }

    pub fn aarch64_tlbi_va(
        &mut self,
        security: SecurityState,
        regime: Regime,
        vmid: u16,
        shareability: Shareability,
        level: TLBILevel,
        attr: TLBIMemAttr,
        xt: u64,
    ) {
        assert_ne!(self.read::<pstate::EL>(), EL::EL0.as_num());

        let r = TLBIRecord {
            op: TLBIOp::VA,
            from_aarch64: true,
            security: security,
            regime: regime,
            vmid: vmid,
            level: level,
            attr: attr,
            asid: ((xt >> 48) & 0xffff) as u16,
            ttl: (xt >> 44) & 0b1111,
            // ZeroExtend(Xt<43:0> : Zeros(12), 64)
            address: (xt & ((1 << 44) - 1)) << 12,
            d64: true,
            d128: ((xt >> 44) & 0b1111) >> 2 == 0,
            ..Default::default()
        };

        self.tlbi(r);
        if shareability != Shareability::NSH {
            // Broadcast(shareability, r);
        }
        return;
    }
}
