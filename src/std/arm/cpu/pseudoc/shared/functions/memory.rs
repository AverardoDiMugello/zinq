use crate::std::arm::cpu::{config::Feat, pseudoc::*, registers::aarch64::*, ArmCtx};

use bitvec::prelude::*;

#[derive(Clone, Copy)]
pub struct FullAddress {
    pub paspace: PASpace,
    pub address: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PASpace {
    NonSecure,
    Secure,
    Root,
    Realm,
}

impl PASpace {
    pub fn decode(nse: bool, ns: bool) -> Self {
        match (nse, ns) {
            (false, false) => PASpace::Secure,
            (false, true) => PASpace::NonSecure,
            (true, false) => PASpace::Root,
            (true, true) => PASpace::Realm,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VARange {
    Lower,
    Upper,
}

impl VARange {
    pub fn aarch64_from_va(va: u64) -> Self {
        if (va >> 55) & 1 == 0 {
            Self::Lower
        } else {
            Self::Upper
        }
    }
}

#[derive(Clone)]
pub struct Permissions {
    pub ap_table: u64,   // Stage 1 hierarchical access permissions
    pub xn_table: bool,  // Stage 1 hierarchical execute-never for single EL regimes
    pub pxn_table: bool, // Stage 1 hierarchical privileged execute-never
    pub uxn_table: bool, // Stage 1 hierarchical unprivileged execute-never
    pub ap: u64,         // Stage 1 access permissions
    pub xn: bool,        // Stage 1 execute-never for single EL regimes
    pub uxn: bool,       // Stage 1 unprivileged execute-never
    pub pxn: bool,       // Stage 1 privileged execute-never
    pub ppi: u64,        // Stage 1 privileged indirect permissions
    pub upi: u64,        // Stage 1 unprivileged indirect permissions
    pub ndirty: bool,    // Stage 1 dirty state for indirect permissions scheme
    pub s2pi: u64,       // Stage 2 indirect permissions
    pub s2dirty: bool,   // Stage 2 dirty state
    pub po_index: u64,   // Stage 1 overlay permissions index
    pub s2po_index: u64, // Stage 2 overlay permissions index
    pub s2ap: u64,       // Stage 2 access permissions
    pub s2tag_na: bool,  // Stage 2 tag access
    pub s2xnx: bool,     // Stage 2 extended execute-never
    pub s2xn: bool,      // Stage 2 execute-never
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            ap_table: 0,
            xn_table: false,
            pxn_table: false,
            uxn_table: false,
            ap: 0,
            xn: false,
            uxn: false,
            pxn: false,
            ppi: 0,
            upi: 0,
            ndirty: false,
            s2pi: 0,
            s2dirty: false,
            po_index: 0,
            s2po_index: 0,
            s2ap: 0,
            s2tag_na: false,
            s2xnx: false,
            s2xn: false,
        }
    }
}

pub struct S1AccessControls {
    pub r: bool,       // Stage 1 base read permission
    pub w: bool,       // Stage 1 base write permission
    pub x: bool,       // Stage 1 base execute permission
    pub gcs: bool,     // Stage 1 GCS permission
    pub wxn: bool,     // Stage 1 write permission implies execute-never
    pub overlay: bool, // Stage 1 overlay feature enabled
    pub overlay_perms: Option<S1OverlayAccessControls>,
}

pub struct S1OverlayAccessControls {
    pub or: bool, // Stage 1 overlay read permission
    pub ow: bool, // Stage 1 overlay write permission
    pub ox: bool, // Stage 1 overlay execute permission
}

pub struct S2AccessControls {
    pub r: bool,         // Stage 2 read permission.
    pub w: bool,         // Stage 2 write permission.
    pub x: bool,         // Stage 2 execute permission.
    pub r_rcw: bool,     // Stage 2 Read perms for RCW instruction.
    pub w_rcw: bool,     // Stage 2 Write perms for RCW instruction.
    pub r_mmu: bool,     // Stage 2 Read perms for TTW data.
    pub w_mmu: bool,     // Stage 2 Write perms for TTW data.
    pub toplevel0: bool, // IPA as top level table for TTBR0_EL1.
    pub toplevel1: bool, // IPA as top level table for TTBR1_EL1.
    pub overlay: bool,   // Overlay enable
    pub overlay_perms: Option<S2OverlayAccessControls>,
}

pub struct S2OverlayAccessControls {
    pub or: bool,        // Stage 2 overlay read permission.
    pub ow: bool,        // Stage 2 overlay write permission.
    pub ox: bool,        // Stage 2 overlay execute permission.
    pub or_rcw: bool,    // Stage 2 overlay Read perms for RCW instruction.
    pub ow_rcw: bool,    // Stage 2 overlay Write perms for RCW instruction.
    pub or_mmu: bool,    // Stage 2 overlay Read perms for TTW data.
    pub ow_mmu: bool,    // Stage 2 overlay Write perms for TTW data.
    pub toplevel0: bool, // IPA as top level table for TTBR0_EL1.
    pub toplevel1: bool, // IPA as top level table for TTBR1_EL1.
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemHint {
    No,  // No Read-Allocate, No Write-Allocate
    WA,  // No Read-Allocate, Write-Allocate
    RA,  // Read-Allocate, No Write-Allocate
    RWA, // Read-Allocate, Write-Allocate
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemAttr {
    NC, // Non-cacheable
    WT, // Write-through
    WB, // Write-back
}

#[derive(Clone, Copy)]
pub struct MemAttrHints {
    pub attrs: MemAttr,
    pub hints: Option<MemHint>,
    pub transient: Option<bool>,
}

impl MemAttrHints {
    fn decode_sdf_attr(rgn: u64) -> Self {
        let attrs;
        let hints;
        match rgn {
            0b00 => {
                // Non-cacheable (no allocate)
                attrs = MemAttr::NC;
                hints = None;
            }
            0b01 => {
                // Write-back, Read and Write allocate
                attrs = MemAttr::WB;
                hints = Some(MemHint::RWA);
            }
            0b10 => {
                // Write-through, Read allocate
                attrs = MemAttr::WT;
                hints = Some(MemHint::RA);
            }
            0b11 => {
                // Write-back, Read allocate
                attrs = MemAttr::WB;
                hints = Some(MemHint::RA);
            }
            _ => panic!("Unreachable"),
        }

        Self {
            attrs,
            hints,
            transient: Some(false),
        }
    }

    fn decode_ldf_attr(attr: u64) -> Self {
        let attrs;
        if (attr >> 2) & 1 == 0 {
            attrs = MemAttr::WT;
        }
        // Write-through
        else if attr == 0b0100 {
            attrs = MemAttr::NC;
        }
        // Non-cacheable
        else if (attr >> 2) & 1 == 1 {
            attrs = MemAttr::WB;
        }
        // Write-back
        else {
            panic!("Unreachable");
        }

        // Allocation hints are applicable only to cacheable memory.
        let mut hints = None;
        if attrs != MemAttr::NC {
            match attr & 0b11 {
                0b00 => {
                    hints = Some(MemHint::No);
                } // No allocation hints
                0b01 => {
                    hints = Some(MemHint::WA);
                } // Write-allocate
                0b10 => {
                    hints = Some(MemHint::RA);
                } // Read-allocate
                0b11 => {
                    hints = Some(MemHint::RWA);
                } // Read/Write allocate
                _ => panic!("Unreachable"),
            }
        }
        // The Transient hint applies only to cacheable memory with some allocation hints.
        let transient;
        if attrs != MemAttr::NC && hints != Some(MemHint::No) {
            transient = Some((attr >> 3) & 1 == 0);
        } else {
            transient = None;
        }

        Self {
            attrs,
            hints,
            transient,
        }
    }

    fn s2_combine_s1_attr_hints(s1_attrhints: &Self, s2_attrhints: &Self) -> Self {
        let attrs;
        let mut hints = None;
        let mut transient = None;

        if s1_attrhints.attrs == MemAttr::NC || s2_attrhints.attrs == MemAttr::NC {
            attrs = MemAttr::NC;
        } else if s1_attrhints.attrs == MemAttr::WT || s2_attrhints.attrs == MemAttr::WT {
            attrs = MemAttr::WT;
        } else {
            attrs = MemAttr::WB;
        }

        if attrs != MemAttr::NC {
            hints = s1_attrhints.hints;
            transient = s1_attrhints.transient;
        }

        Self {
            attrs,
            hints,
            transient,
        }
    }

    fn s2_decode_cacheability(attr: u64) -> Self {
        return Self {
            attrs: match attr {
                0b10 => MemAttr::WT, // Write-through
                0b11 => MemAttr::WB, // Write-back
                _ => MemAttr::NC,    // Non-cacheable
            },
            hints: None,
            transient: None,
        };
    }
}

#[derive(Clone, Copy)]
pub struct MemoryAttributes {
    pub memtype: MemType,
    pub inner: MemAttrHints,        // Inner hints and attributes
    pub outer: MemAttrHints,        // Outer hints and attributes
    pub shareability: Shareability, // Shareability attribute
    pub tags: MemTagType,           // MTE tag type for this memory.
    pub notagaccess: bool,          // Allocation Tag access permission
    pub xs: bool,                   // XS attribute
}

impl MemoryAttributes {
    pub fn normal_nc(xs: bool) -> Self {
        Self {
            memtype: MemType::Normal,
            inner: MemAttrHints {
                attrs: MemAttr::NC,
                hints: None,
                transient: None,
            },
            outer: MemAttrHints {
                attrs: MemAttr::NC,
                hints: None,
                transient: None,
            },
            shareability: Shareability::OSH,
            tags: MemTagType::Untagged,
            notagaccess: false,
            xs,
        }
    }

    pub fn effective_shareability(&self) -> Shareability {
        if matches!(self.memtype, MemType::Device(_))
            || (self.inner.attrs == MemAttr::NC && self.outer.attrs == MemAttr::NC)
        {
            Shareability::OSH
        } else {
            self.shareability
        }
    }

    pub fn walk_mem_attrs(sh: u64, irgn: u64, orgn: u64) -> Self {
        Self {
            memtype: MemType::Normal,
            shareability: Shareability::decode(sh),
            inner: MemAttrHints::decode_sdf_attr(irgn),
            outer: MemAttrHints::decode_sdf_attr(orgn),
            tags: MemTagType::Untagged,
            notagaccess: false,
            xs: true,
        }
    }

    pub fn s1_decode_mem_attrs(attr: u64, sh: u64, s1aarch64: bool, mtx: bool) -> Self {
        let mut memattrs = Self::normal_nc(false);

        if attr >> 4 == 0 {
            // Device memory
            let attr_0 = (attr >> 0) & 1 != 0;
            memattrs.memtype = MemType::Device(DeviceType::decode((attr >> 2) & 0b11));
            memattrs.xs = if s1aarch64 { !attr_0 } else { true };
        } else if attr == 0b01000000 {
            memattrs.memtype = MemType::Normal;
            memattrs.outer.attrs = MemAttr::NC;
            memattrs.inner.attrs = MemAttr::NC;
            memattrs.xs = false;
        } else if attr == 0b10100000 {
            memattrs.memtype = MemType::Normal;
            memattrs.outer.attrs = MemAttr::WT;
            memattrs.outer.hints = Some(MemHint::RA);
            memattrs.outer.transient = Some(false);
            memattrs.inner.attrs = MemAttr::WT;
            memattrs.inner.hints = Some(MemHint::RA);
            memattrs.inner.transient = Some(false);
            memattrs.xs = false;
        } else if attr == 0b11110000 {
            // Tagged memory
            memattrs.memtype = MemType::Normal;
            memattrs.outer.attrs = MemAttr::WB;
            memattrs.outer.hints = Some(MemHint::RWA);
            memattrs.outer.transient = Some(false);
            memattrs.inner.attrs = MemAttr::WB;
            memattrs.inner.hints = Some(MemHint::RWA);
            memattrs.inner.transient = Some(false);
            memattrs.xs = false;
        } else {
            memattrs.memtype = MemType::Normal;
            memattrs.outer = MemAttrHints::decode_ldf_attr((attr >> 4) & 0b1111);
            memattrs.inner = MemAttrHints::decode_ldf_attr(attr & 0b1111);

            if memattrs.inner.attrs == MemAttr::WB && memattrs.outer.attrs == MemAttr::WB {
                memattrs.xs = false;
            } else {
                memattrs.xs = true;
            }
            memattrs.xs = true;
        }

        if s1aarch64 && attr == 0b11110000 {
            memattrs.tags = MemTagType::AllocationTagged;
        } else if s1aarch64 && mtx {
            memattrs.tags = MemTagType::CanonicallyTagged;
        } else {
            memattrs.tags = MemTagType::Untagged;
        }

        memattrs.notagaccess = false;

        memattrs.shareability = Shareability::decode(sh);

        return memattrs;
    }

    pub fn s2_combine_s1_mem_attrs(
        s1_memattrs: Self,
        s2_memattrs: Self,
        s2aarch64: bool,
        impl_feat_mte2: bool,
        impl_feat_mte_perm: bool,
    ) -> Self {
        let mut memattrs = MemoryAttributes::normal_nc(false);
        if let MemType::Device(s1_dev) = s1_memattrs.memtype {
            if let MemType::Device(s2_dev) = s2_memattrs.memtype {
                memattrs.memtype =
                    MemType::Device(DeviceType::s2_combine_s1_device(s1_dev, s2_dev));
            }
        } else if matches!(s1_memattrs.memtype, MemType::Device(_)) {
            memattrs = s1_memattrs;
        } else if matches!(s2_memattrs.memtype, MemType::Device(_)) {
            memattrs = s2_memattrs;
        } else {
            memattrs.memtype = MemType::Normal;
            memattrs.inner =
                MemAttrHints::s2_combine_s1_attr_hints(&s1_memattrs.inner, &s2_memattrs.inner);
            memattrs.outer =
                MemAttrHints::s2_combine_s1_attr_hints(&s1_memattrs.outer, &s2_memattrs.outer);
        }

        memattrs.tags = MemTagType::s2_mem_tag_type(&memattrs, s1_memattrs.tags, impl_feat_mte2);

        if !impl_feat_mte_perm {
            memattrs.notagaccess = false;
        } else {
            memattrs.notagaccess =
                s2_memattrs.notagaccess && s1_memattrs.tags == MemTagType::AllocationTagged;
        }
        memattrs.shareability = Shareability::s2_combine_s1_shareability(
            s1_memattrs.shareability,
            s2_memattrs.shareability,
        );

        if memattrs.memtype == MemType::Normal
            && memattrs.inner.attrs == MemAttr::WB
            && memattrs.outer.attrs == MemAttr::WB
        {
            memattrs.xs = false;
        } else if s2aarch64 {
            memattrs.xs = s2_memattrs.xs && s1_memattrs.xs;
        } else {
            memattrs.xs = s1_memattrs.xs;
        }
        memattrs.shareability = memattrs.effective_shareability();
        return memattrs;
    }

    pub fn s2_decode_mem_attrs(
        attr: u64,
        sh: u64,
        s2aarch64: bool,
        impl_feat_mte_perm: bool,
    ) -> Self {
        let mut memattrs = Self::normal_nc(false);

        if attr >> 2 == 0b00 {
            memattrs.memtype = MemType::Device(DeviceType::decode(attr & 0b11));
        } else if attr == 0b0100 {
            if s2aarch64 && impl_feat_mte_perm {
                memattrs.memtype = MemType::Normal;
                memattrs.outer = MemAttrHints::s2_decode_cacheability(0b11); // Write-back
                memattrs.inner = MemAttrHints::s2_decode_cacheability(0b11); // Write-back
            } else {
                memattrs.memtype = MemType::Normal;
                memattrs.outer = MemAttrHints::s2_decode_cacheability((attr >> 2) & 0b11);
                memattrs.inner = MemAttrHints::s2_decode_cacheability(attr & 0b11);
            }
        } else {
            memattrs.memtype = MemType::Normal;
            memattrs.outer = MemAttrHints::s2_decode_cacheability((attr >> 2) & 0b11);
            memattrs.inner = MemAttrHints::s2_decode_cacheability(attr & 0b11);
        }
        memattrs.shareability = Shareability::decode(sh);

        if s2aarch64 && impl_feat_mte_perm {
            memattrs.notagaccess = attr == 0b0100;
        } else {
            memattrs.notagaccess = false;
        }
        return memattrs;
    }

    pub fn aarch64_s2_apply_fwb_mem_attrs(
        s1_memattrs: &Self,
        walkparams: &S2TTWParams,
        descriptor: u128,
        impl_feat_mte2: bool,
    ) -> Self {
        let mut memattrs: MemoryAttributes;
        let s2_attr = ((descriptor >> 2) & 0b1111) as u64;
        let s2_sh = if walkparams.ds {
            walkparams.sh
        } else {
            ((descriptor >> 8) & 0b11) as u64
        };
        let s2_fnxs = (descriptor >> 11) & 1 != 0;

        if (s2_attr >> 2 & 1) == 0 {
            let s2_device = DeviceType::decode(s2_attr & 0b11);
            memattrs = MemoryAttributes {
                memtype: if let MemType::Device(s1_memattrs_device) = s1_memattrs.memtype {
                    MemType::Device(DeviceType::s2_combine_s1_device(
                        s1_memattrs_device,
                        s2_device,
                    ))
                } else {
                    MemType::Device(s2_device)
                },
                xs: s1_memattrs.xs,
                ..*s1_memattrs
            };
        } else if s2_attr & 0b11 == 0b11 {
            memattrs = s1_memattrs.clone();
        } else if s2_attr & 0b11 == 0b10 {
            memattrs = MemoryAttributes {
                memtype: MemType::Normal,
                inner: if s1_memattrs.memtype == MemType::Normal
                    && s1_memattrs.inner.attrs != MemAttr::NC
                {
                    MemAttrHints {
                        hints: s1_memattrs.inner.hints,
                        transient: s1_memattrs.inner.transient,
                        attrs: MemAttr::WB,
                    }
                } else {
                    MemAttrHints {
                        hints: Some(MemHint::RWA),
                        transient: Some(false),
                        attrs: MemAttr::WB,
                    }
                },
                outer: if s1_memattrs.memtype == MemType::Normal
                    && s1_memattrs.outer.attrs != MemAttr::NC
                {
                    MemAttrHints {
                        hints: s1_memattrs.outer.hints,
                        transient: s1_memattrs.outer.transient,
                        attrs: MemAttr::WB,
                    }
                } else {
                    MemAttrHints {
                        hints: Some(MemHint::RWA),
                        transient: Some(false),
                        attrs: MemAttr::WB,
                    }
                },
                xs: false,
                ..*s1_memattrs
            };
        } else {
            if matches!(s1_memattrs.memtype, MemType::Device(_)) {
                memattrs = s1_memattrs.clone();
            } else {
                let cacheability_attr = MemAttrHints {
                    attrs: MemAttr::NC,
                    hints: None,
                    transient: None,
                };

                memattrs = MemoryAttributes {
                    memtype: MemType::Normal,
                    inner: cacheability_attr,
                    outer: cacheability_attr,
                    xs: s1_memattrs.xs,
                    ..*s1_memattrs
                };
            }
        }
        let s2_shareability = Shareability::decode(s2_sh);
        memattrs.shareability =
            Shareability::s2_combine_s1_shareability(s1_memattrs.shareability, s2_shareability);
        memattrs.tags = MemTagType::s2_mem_tag_type(&memattrs, s1_memattrs.tags, impl_feat_mte2);
        memattrs.notagaccess =
            (s2_attr >> 1) & 0b111 == 0b111 && memattrs.tags == MemTagType::AllocationTagged;

        if s2_fnxs {
            memattrs.xs = false;
        }
        memattrs.shareability = memattrs.effective_shareability();
        return memattrs;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemType {
    Normal,
    Device(DeviceType),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    GRE,
    NoGRE,
    NoGNoRE,
    NoGNoRNoE,
}

impl DeviceType {
    pub fn decode(device: u64) -> Self {
        match device {
            0b00 => DeviceType::NoGNoRNoE,
            0b01 => DeviceType::NoGNoRE,
            0b10 => DeviceType::NoGRE,
            0b11 => DeviceType::GRE,
            _ => panic!("Unreachable"),
        }
    }

    pub fn s2_combine_s1_device(s1_device: Self, s2_device: Self) -> Self {
        if s1_device == DeviceType::NoGNoRNoE || s2_device == DeviceType::NoGNoRNoE {
            return DeviceType::NoGNoRNoE;
        } else if s1_device == DeviceType::NoGNoRE || s2_device == DeviceType::NoGNoRE {
            return DeviceType::NoGNoRE;
        } else if s1_device == DeviceType::NoGRE || s2_device == DeviceType::NoGRE {
            return DeviceType::NoGRE;
        } else {
            return DeviceType::GRE;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shareability {
    NSH,
    ISH,
    OSH,
}

impl Shareability {
    pub fn decode(sh: u64) -> Self {
        match sh {
            0b10 => Shareability::OSH,
            0b11 => Shareability::ISH,
            0b00 => Shareability::NSH,
            _ => panic!("Unreachable"),
        }
    }

    pub fn s2_combine_s1_shareability(s1_shareability: Self, s2_shareability: Self) -> Self {
        if s1_shareability == Shareability::OSH || s2_shareability == Shareability::OSH {
            return Shareability::OSH;
        } else if s1_shareability == Shareability::ISH || s2_shareability == Shareability::ISH {
            return Shareability::ISH;
        } else {
            return Shareability::NSH;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemOp {
    LOAD,
    STORE,
    PREFETCH,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemTagType {
    Untagged,
    AllocationTagged,
    CanonicallyTagged,
}

impl MemTagType {
    fn s2_mem_tag_type(
        s2_memattrs: &MemoryAttributes,
        s1_tagtype: MemTagType,
        impl_feat_mte2: bool,
    ) -> MemTagType {
        if !impl_feat_mte2 {
            return MemTagType::Untagged;
        }

        if (s1_tagtype == MemTagType::AllocationTagged)
            && (s2_memattrs.memtype == MemType::Normal)
            && (s2_memattrs.inner.attrs == MemAttr::WB)
            && (s2_memattrs.inner.hints == Some(MemHint::RWA))
            && (s2_memattrs.inner.transient.is_some_and(|val| val == false))
            && (s2_memattrs.outer.attrs == MemAttr::WB)
            && (s2_memattrs.outer.hints == Some(MemHint::RWA))
            && (s2_memattrs.outer.transient.is_some_and(|val| val == false))
        {
            return MemTagType::AllocationTagged;
        }

        if s1_tagtype != MemTagType::AllocationTagged {
            return s1_tagtype;
        }

        return MemTagType::Untagged;
    }
}

#[derive(Clone)]
pub struct AccessDescriptor {
    pub acctype: AccessType,
    pub el: EL,                    // Acting EL for the access
    pub ss: SecurityState,         // Acting Security State for the access
    pub acqsc: bool,               // Acquire with Sequential Consistency
    pub acqpc: bool,               // FEAT_LRCPC: Acquire with Processor Consistency
    pub relsc: bool,               // Release with Sequential Consistency
    pub limitedordered: bool,      // FEAT_LOR: Acquire/Release with limited ordering
    pub exclusive: bool,           // Access has Exclusive semantics
    pub atomicop: bool,            // FEAT_LSE: Atomic read-modify-write access
    pub modop: MemAtomicOp,        // FEAT_LSE: The modification operation in the 'atomicop' access
    pub nontemporal: bool,         // Hints the access is non-temporal
    pub read: bool,                // Read from memory or only require read permissions
    pub write: bool,               // Write to memory or only require write permissions
    pub cacheop: CacheOp,          // DC/IC: Cache operation
    pub opscope: CacheOpScope,     // DC/IC: Scope of cache operation
    pub cachetype: CacheType,      // DC/IC: Type of target cache
    pub pan: bool,                 // FEAT_PAN: The access is subject to PSTATE.PAN
    pub transactional: bool,       // FEAT_TME: Access is part of a transaction
    pub nonfault: bool,            // SVE: Non-faulting load
    pub firstfault: bool,          // SVE: First-fault load
    pub first: bool,               // SVE: First-fault load for the first active element
    pub contiguous: bool,          // SVE: Contiguous load/store not gather load/scatter store
    pub streamingsve: bool,        // SME: Access made by PE while in streaming SVE mode
    pub ls64: bool,                // FEAT_LS64: Accesses by accelerator support loads/stores
    pub mops: bool,                // FEAT_MOPS: Memory operation (CPY/SET) accesses
    pub rcw: bool,                 // FEAT_THE: Read-Check-Write access
    pub rcws: bool,                // FEAT_THE: Read-Check-Write Software access
    pub toplevel: bool,            // FEAT_THE: Translation table walk access for TTB address
    pub varange: VARange,          // FEAT_THE: The corresponding TTBR supplying the TTB
    pub a32lsmd: bool,             // A32 Load/Store Multiple Data access
    pub tagchecked: bool,          // FEAT_MTE2: Access is tag checked
    pub tagaccess: bool,           // FEAT_MTE: Access targets the tag bits
    pub devstoreunpred: bool, // FEAT_MTE: Accesses that store Allocation tags to Device memory. Access represents a Load/Store pair access
    pub highestaddressfirst: bool, // FEAT_LRCPC3: Highest address is accessed first
    pub mpam: MPAMInfo,       // FEAT_MPAM: MPAM information
}

impl Default for AccessDescriptor {
    fn default() -> Self {
        Self {
            acctype: AccessType::GPR,
            el: EL::EL0,
            ss: SecurityState::NonSecure,
            acqsc: false,
            acqpc: false,
            relsc: false,
            limitedordered: false,
            exclusive: false,
            atomicop: false,
            modop: MemAtomicOp::ADD,
            nontemporal: false,
            read: false,
            write: false,
            cacheop: CacheOp::Clean,
            opscope: CacheOpScope::PoC,
            cachetype: CacheType::Data,
            pan: false,
            transactional: false,
            nonfault: false,
            firstfault: false,
            first: false,
            contiguous: false,
            streamingsve: false,
            ls64: false,
            mops: false,
            rcw: false,
            rcws: false,
            toplevel: false,
            varange: VARange::Lower,
            a32lsmd: false,
            tagchecked: false,
            tagaccess: false,
            devstoreunpred: false,
            highestaddressfirst: false,
            mpam: MPAMInfo {
                mpam_sp: PARTIDSpaceType::NonSecure,
                partid: 0,
                pmg: 0,
            },
        }
    }
}

impl AccessDescriptor {
    pub fn create_s1_ttw(toplevel: bool, varange: VARange, accdesc_in: &Self) -> Self {
        Self {
            acctype: AccessType::TTW,
            el: accdesc_in.el,
            ss: accdesc_in.ss,
            read: true,
            toplevel,
            varange,
            mpam: accdesc_in.mpam,
            ..Default::default()
        }
    }

    pub fn create_s2_ttw(accdesc_in: &Self) -> Self {
        Self {
            acctype: AccessType::TTW,
            el: accdesc_in.el,
            ss: accdesc_in.ss,
            read: true,
            mpam: accdesc_in.mpam,
            ..Default::default()
        }
    }

    pub fn create_tte_update(accdesc_in: &Self) -> Self {
        Self {
            acctype: AccessType::TTW,
            el: accdesc_in.el,
            ss: accdesc_in.ss,
            atomicop: true,
            modop: MemAtomicOp::CAS,
            read: true,
            write: true,
            mpam: accdesc_in.mpam,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    IFETCH, // Instruction FETCH
    GPR,    // Software load/store to a General Purpose Register
    ASIMD,  // Software ASIMD extension load/store instructions
    SVE,    // Software SVE load/store instructions
    SME,    // Software SME load/store instructions
    IC,     // Sysop IC
    DC,     // Sysop DC (not DC {Z,G,GZ}VA)
    DCZero, // Sysop DC {Z,G,GZ}VA
    AT,     // Sysop AT
    NV2,    // NV2 memory redirected access
    SPE,    // Statistical Profiling buffer access
    GCS,    // Guarded Control Stack access
    TRBE,   // Trace Buffer access
    GPTW,   // Granule Protection Table Walk
    TTW,    // Translation Table Walk
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemAtomicOp {
    GCSSS1,
    ADD,
    BIC,
    EOR,
    ORR,
    SMAX,
    SMIN,
    UMAX,
    UMIN,
    SWP,
    CAS,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CacheOp {
    Clean,
    Invalidate,
    CleanInvalidate,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CacheOpScope {
    SetWay,
    PoU,
    PoC,
    PoE,
    PoP,
    PoDP,
    PoPA,
    ALLU,
    ALLUIS,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CacheType {
    Data,
    Tag,
    DataTag,
    Instruction,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CachePASpace {
    NonSecure,
    Any,
    RealmNonSecure,
    Realm,
    Root,
    SecureNonSecure,
    Secure,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Fault {
    None,
    AccessFlag,
    Alignment,
    Background,
    Domain,
    Permission,
    Translation,
    AddressSize,
    SyncExternal,
    SyncExternalOnWalk,
    SyncParity,
    SyncParityOnWalk,
    GPCFOnWalk,
    GPCFOnOutput,
    AsyncParity,
    AsyncExternal,
    TagCheck,
    Debug,
    TLBConflict,
    BranchTarget,
    HWUpdateAccessFlag,
    Lockdown,
    Exclusive,
    ICacheMaint,
}

impl Fault {
    pub fn is_external_abort(&mut self) -> bool {
        matches!(
            self,
            Fault::SyncExternal
                | Fault::SyncParity
                | Fault::SyncExternalOnWalk
                | Fault::SyncParityOnWalk
                | Fault::AsyncExternal
                | Fault::AsyncParity
        )
    }
}

#[derive(Clone)]
pub struct FaultRecord {
    pub statuscode: Fault,                    // Fault Status
    pub accessdesc: Option<AccessDescriptor>, // Details of the faulting access
    pub ipaddress: Option<FullAddress>,       // Intermediate physical address
    pub gpcf: GPCFRecord,                     // Granule Protection Check Fault record
    pub paddress: Option<FullAddress>,        // Physical address
    pub gpcfs2walk: bool,                     // GPC for a stage 2 translation table walk
    pub s2fs1walk: bool,                      // Is on a Stage 1 translation table walk
    pub write: Option<bool>,                  // TRUE for a write, FALSE for a read
    pub s1tagnotdata: bool, // TRUE for a fault due to tag not accessible at stage 1.
    pub tagaccess: bool,    // TRUE for a fault due to NoTagAccess permission.
    pub level: i64,         // For translation, access flag and Permission faults
    pub extflag: Option<bool>, // IMPLEMENTATION DEFINED syndrome for External aborts
    pub secondstage: bool,  // Is a Stage 2 abort
    pub assuredonly: bool,  // Stage 2 Permission fault due to AssuredOnly attribute
    pub toplevel: bool,     // Stage 2 Permission fault due to TopLevel
    pub overlay: bool,      // Fault due to overlay permissions
    pub dirtybit: bool,     // Fault due to dirty state
    pub domain: Option<u64>, // Domain number, AArch32 only
    pub merrorstate: Option<ErrorState>, // Incoming error state from memory
    pub maybe_false_match: Option<bool>, // Watchpoint matches rounded range
    pub watchpt_num: Option<i64>, // Matching watchpoint number
    pub hdbssf: bool,       // Fault caused by HDBSS
    pub debugmoe: Option<u64>, // Debug method of entry, from AArch32 only
}

impl FaultRecord {
    pub fn no_fault() -> Self {
        Self {
            statuscode: Fault::None,
            accessdesc: None,
            ipaddress: None,
            gpcf: GPCFRecord::no_fault(),
            paddress: None,
            gpcfs2walk: false,
            s2fs1walk: false,
            write: None,
            s1tagnotdata: false,
            tagaccess: false,
            level: 0,
            extflag: None,
            secondstage: false,
            assuredonly: false,
            toplevel: false,
            overlay: false,
            dirtybit: false,
            domain: None,
            merrorstate: None,
            maybe_false_match: None,
            watchpt_num: None,
            hdbssf: false,
            debugmoe: None,
        }
    }

    pub fn no_fault_for_access(accdesc: AccessDescriptor) -> Self {
        let write = !accdesc.read && accdesc.write;
        Self {
            statuscode: Fault::None,
            accessdesc: Some(accdesc),
            ipaddress: None,
            gpcf: GPCFRecord::no_fault(),
            paddress: None,
            gpcfs2walk: false,
            s2fs1walk: false,
            write: Some(write),
            s1tagnotdata: false,
            tagaccess: false,
            level: 0,
            extflag: None,
            secondstage: false,
            assuredonly: false,
            toplevel: false,
            overlay: false,
            dirtybit: false,
            domain: None,
            merrorstate: None,
            maybe_false_match: None,
            watchpt_num: None,
            hdbssf: false,
            debugmoe: None,
        }
    }

    pub fn is_external_abort(&mut self) -> bool {
        self.statuscode.is_external_abort() || self.gpcf.gpf == GPCF::EABT
    }
}

#[derive(Clone)]
pub struct GPCFRecord {
    pub gpf: GPCF,
    pub level: Option<i64>,
}

impl GPCFRecord {
    pub fn no_fault() -> Self {
        Self {
            gpf: GPCF::None,
            level: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GPCF {
    None,
    AddressSize,
    Walk,
    EABT,
    Fail,
}

#[derive(Clone, Copy)]
pub enum ErrorState {
    UC,
    UEU,
    UEO,
    UER,
    CE,
}

pub struct PhysMemRetStatus {
    pub statuscode: Fault,
    pub extflag: bool,
    pub merrorstate: ErrorState,
    pub store64bstatus: u64,
}

impl PhysMemRetStatus {
    pub fn is_fault(&self) -> bool {
        self.statuscode != Fault::None
    }
}

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub const TAG_GRANULE: u64 = 1 << 4;

    pub fn new_acc_desc(&mut self, acctype: AccessType) -> AccessDescriptor {
        let el = self.curr_el();
        AccessDescriptor {
            acctype,
            el,
            ss: self.security_state_at_el(el),
            acqsc: false,
            acqpc: false,
            relsc: false,
            limitedordered: false,
            exclusive: false,
            atomicop: false,
            modop: MemAtomicOp::ADD, // Architecture leaves uninitialized
            nontemporal: false,
            read: false,
            write: false,
            cacheop: CacheOp::Clean,     // Arch leaves uninit
            opscope: CacheOpScope::ALLU, // Arch leaves uninit
            cachetype: CacheType::Data,  // Arch leaves uninit
            pan: false,
            transactional: false,
            nonfault: false,
            firstfault: false,
            first: false,
            contiguous: false,
            streamingsve: false,
            ls64: false,
            mops: false,
            rcw: false,
            rcws: false,
            toplevel: false,         // Arch leaves uninit
            varange: VARange::Lower, // Arch leaves uninit
            a32lsmd: false,
            tagchecked: false,
            tagaccess: false,
            devstoreunpred: false,
            highestaddressfirst: false,
            mpam: MPAMInfo {
                mpam_sp: PARTIDSpaceType::NonSecure,
                partid: 0,
                pmg: 0,
            }, // TODO: GenMPAMCurEl(acctype)
        }
    }

    pub fn create_acc_desc_asimd(
        &mut self,
        memop: MemOp,
        nontemporal: bool,
        tagchecked: bool,
    ) -> AccessDescriptor {
        AccessDescriptor {
            nontemporal,
            read: memop == MemOp::LOAD,
            write: memop == MemOp::STORE,
            pan: true,
            streamingsve: self.in_streaming_mode(),
            tagchecked: if self.in_streaming_mode()
                && self
                    .cpu
                    .impdef
                    .bool("No tag checking of SIMD&FP loads and stores in Streaming SVE mode")
            {
                false
            } else {
                tagchecked
            },
            transactional: self.is_feat_impl(Feat::TME) && false, // TSTATE.depth > 0
            ..self.new_acc_desc(AccessType::ASIMD)
        }
    }

    pub fn create_acc_desc_acq_rel(&mut self, memop: MemOp, tagchecked: bool) -> AccessDescriptor {
        AccessDescriptor {
            acqsc: memop == MemOp::LOAD,
            relsc: memop == MemOp::STORE,
            read: memop == MemOp::LOAD,
            write: memop == MemOp::STORE,
            pan: true,
            tagchecked,
            transactional: self.is_feat_impl(Feat::TME) && false, // TSTATE.depth > 0
            ..self.new_acc_desc(AccessType::GPR)
        }
    }

    pub fn create_acc_desc_atomic_op(
        &mut self,
        modop: MemAtomicOp,
        acquire: bool,
        release: bool,
        tagchecked: bool,
    ) -> AccessDescriptor {
        AccessDescriptor {
            acqsc: acquire,
            relsc: release,
            atomicop: true,
            modop,
            read: true,
            write: true,
            pan: true,
            tagchecked,
            transactional: self.is_feat_impl(Feat::TME) && false, // TSTATE.depth > 0
            ..self.new_acc_desc(AccessType::GPR)
        }
    }

    pub fn create_acc_desc_dc_zero(&mut self, cachetype: CacheType) -> AccessDescriptor {
        AccessDescriptor {
            write: true,
            pan: true,
            tagchecked: cachetype == CacheType::Data,
            tagaccess: matches!(cachetype, CacheType::Tag | CacheType::DataTag),
            devstoreunpred: cachetype == CacheType::Tag,
            transactional: self.is_feat_impl(Feat::TME) && false, // TSTATE.depth > 0
            ..self.new_acc_desc(AccessType::DCZero)
        }
    }

    pub fn create_acc_desc_ex_ldst(
        &mut self,
        memop: MemOp,
        acqrel: bool,
        tagchecked: bool,
    ) -> AccessDescriptor {
        AccessDescriptor {
            acqsc: acqrel && memop == MemOp::LOAD,
            relsc: acqrel && memop == MemOp::STORE,
            exclusive: true,
            read: memop == MemOp::LOAD,
            write: memop == MemOp::STORE,
            pan: true,
            tagchecked,
            transactional: self.is_feat_impl(Feat::TME) && false, // TSTATE.depth > 0
            ..self.new_acc_desc(AccessType::GPR)
        }
    }

    pub fn create_acc_desc_gpr(
        &mut self,
        memop: MemOp,
        nontemporal: bool,
        privileged: bool,
        tagchecked: bool,
    ) -> AccessDescriptor {
        AccessDescriptor {
            el: if !privileged {
                EL::EL0
            } else {
                self.curr_el().clone()
            },
            nontemporal,
            read: memop == MemOp::LOAD,
            write: memop == MemOp::STORE,
            pan: true,
            tagchecked,
            transactional: self.is_feat_impl(Feat::TME) && false, // TSTATE.depth > 0
            ..self.new_acc_desc(AccessType::GPR)
        }
    }

    pub fn create_acc_desc_ifetch(&mut self) -> AccessDescriptor {
        AccessDescriptor {
            ..self.new_acc_desc(AccessType::IFETCH)
        }
    }

    pub fn phys_mem_read(
        &mut self,
        desc: &AddressDescriptor,
        size: usize,
        _accdesc: &AccessDescriptor,
    ) -> (PhysMemRetStatus, Option<[u8; 16]>) {
        self.ctx
            .mem_read(desc.paddress.unwrap().address as usize, size)
            .0
            .map_or(
                (
                    PhysMemRetStatus {
                        statuscode: Fault::Translation,
                        extflag: false,
                        merrorstate: ErrorState::UEU,
                        store64bstatus: 1,
                    },
                    None,
                ),
                move |data| {
                    (
                        PhysMemRetStatus {
                            statuscode: Fault::None,
                            extflag: false,
                            merrorstate: ErrorState::CE,
                            store64bstatus: 1,
                        },
                        Some(data),
                    )
                },
            )
    }

    pub fn phys_mem_write(
        &mut self,
        desc: &AddressDescriptor,
        _size: usize,
        _accdesc: &AccessDescriptor,
        value: &[u8],
    ) -> PhysMemRetStatus {
        let write_result = self
            .ctx
            .mem_write(desc.paddress.unwrap().address as usize, value);

        match write_result.0 {
            Some(_) => {
                return PhysMemRetStatus {
                    statuscode: Fault::None,
                    extflag: false,
                    merrorstate: ErrorState::CE,
                    store64bstatus: 1,
                };
            }
            None => {
                // println!("{e}");
                return PhysMemRetStatus {
                    statuscode: Fault::Translation,
                    extflag: false,
                    merrorstate: ErrorState::UEU,
                    store64bstatus: 1,
                };
            }
        }
    }

    fn effective_tbi(
        &mut self,
        address: u64,
        is_instr: bool,
        el: EL,
        s1_translation_regime: EL,
    ) -> bool {
        assert!(self.have_el(el));
        let regime = s1_translation_regime;
        assert!(!self.el_using_aarch32(regime));

        let tbi;
        let tbid;
        match regime {
            EL::EL0 => panic!("Unreachable"),
            EL::EL1 => {
                tbi = if address.view_bits::<Lsb0>()[55] {
                    self.read::<tcr_el1::TBI1>() != 0
                } else {
                    self.read::<tcr_el1::TBI0>() != 0
                };
                if self.is_feat_impl(Feat::PAuth) {
                    tbid = if address.view_bits::<Lsb0>()[55] {
                        self.read::<tcr_el1::TBID1>() != 0
                    } else {
                        self.read::<tcr_el1::TBID0>() != 0
                    };
                } else {
                    tbid = false;
                }
            }
            EL::EL2 => {
                if self.is_feat_impl(Feat::VHE) && self.el_is_in_host(el) {
                    tbi = if address.view_bits::<Lsb0>()[55] {
                        self.read::<tcr_el2::TBI1>() != 0
                    } else {
                        self.read::<tcr_el2::TBI0>() != 0
                    };
                    if self.is_feat_impl(Feat::PAuth) {
                        tbid = self.read::<tcr_el2::TBID1>() != 0;
                    } else {
                        tbid = self.read::<tcr_el2::TBID0>() != 0;
                    }
                } else {
                    tbi = self.read::<tcr_el2::TBI>() != 0;
                    if self.is_feat_impl(Feat::PAuth) {
                        tbid = self.read::<tcr_el2::TBID>() != 0;
                    } else {
                        tbid = false;
                    }
                }
            }
            EL::EL3 => {
                tbi = self.read::<tcr_el3::TBI>() != 0;
                if self.is_feat_impl(Feat::PAuth) {
                    tbid = self.read::<tcr_el3::TBID>() != 0;
                } else {
                    tbid = false;
                }
            }
        }

        tbi && (!tbid || !is_instr)
    }

    fn addr_top(&mut self, address: u64, is_instr: bool, el: EL) -> u64 {
        assert!(self.have_el(el));
        let regime = self.s1_translation_regime(el);
        if self.el_using_aarch32(regime) {
            return 31;
        } else {
            if self.effective_tbi(address, is_instr, el, regime) {
                return 55;
            } else {
                return 63;
            }
        }
    }

    pub fn aarch64_branch_addr(&mut self, vaddress: u64, el: EL) -> u64 {
        assert!(!self.using_aarch32());
        let msbit = self.addr_top(vaddress, true, el);
        if msbit == 63 {
            return vaddress;
        } else if (matches!(el, EL::EL0 | EL::EL1) || self.is_in_host())
            && (vaddress >> msbit) & 1 != 0
        {
            return vaddress & ((1 << msbit) - 1) | (((1 << (64 - msbit)) - 1) << msbit);
        } else {
            return vaddress & ((1 << msbit) - 1);
        }
    }
}
