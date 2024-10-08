use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::ArmCtx;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Regime {
    EL3,
    EL30,
    EL2,
    EL20,
    EL10,
}

impl Regime {
    pub fn has_unprivileged(&self) -> bool {
        matches!(self, Regime::EL10 | Regime::EL20 | Regime::EL30)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum TGx {
    TG4KB,
    TG16KB,
    TG64KB,
}

impl TGx {
    pub fn granule_bits(&self) -> u64 {
        match self {
            TGx::TG4KB => 12,
            TGx::TG16KB => 14,
            TGx::TG64KB => 16,
        }
    }

    pub fn translation_size(&self, d128: bool, level: i64) -> u64 {
        let granulebits = self.granule_bits() as i64;
        let descsizelog2 = if d128 { 4 } else { 3 } as i64;
        let blockbits = (3 - level) * (granulebits - descsizelog2);

        return (granulebits + blockbits) as u64;
    }

    pub fn contiguous_size(&self, d128: bool, level: i64) -> u64 {
        if d128 {
            match self {
                TGx::TG4KB => {
                    return if level == 1 { 2 } else { 4 };
                }
                TGx::TG16KB => {
                    if level == 1 {
                        return 2;
                    } else if level == 2 {
                        return 4;
                    } else {
                        return 6;
                    }
                }
                TGx::TG64KB => {
                    return if level == 2 { 6 } else { 4 };
                }
            }
        } else {
            match self {
                TGx::TG4KB => {
                    return 4;
                }
                TGx::TG16KB => {
                    return if level == 2 { 5 } else { 7 };
                }
                TGx::TG64KB => {
                    return 5;
                }
            }
        }
    }

    pub fn aarch64_contiguous_bit(&self, d128: bool, level: i64, descriptor: u128) -> bool {
        if d128 {
            if (*self == TGx::TG64KB && level == 1) || (*self == TGx::TG4KB && level == 0) {
                return false; // RES0
            } else {
                return (descriptor >> 111) & 1 != 0;
            }
        }

        // When using TGx 64KB and FEAT_LPA is implememted,
        // the Contiguous bit is RES0 for Block descriptors at level 1
        if *self == TGx::TG64KB && level == 1 {
            return false; // RES0
        }

        // When the effective value of TCR_ELx.DS is '1',
        // the Contiguous bit is RES0 for all the following:
        //      * For TGx 4KB, Block descriptors at level 0
        //      * For TGx 16KB, Block descriptors at level 1
        if *self == TGx::TG16KB && level == 1 {
            return false; // RES0
        }

        if *self == TGx::TG4KB && level == 0 {
            return false; // RES0
        }

        return (descriptor >> 52) & 1 != 0;
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TLBContext {
    pub ss: SecurityState,
    pub regime: Regime,
    pub vmid: u16,
    pub asid: u16,
    pub ng: bool,
    pub ipaspace: PASpace, // Used in stage 2 lookups & invalidations only
    pub includes_s1: bool,
    pub includes_s2: bool,
    pub includes_gpt: bool,
    pub ia: u64, // Input Address
    pub tg: TGx,
    pub cnp: bool,
    pub level: i64, // Assist TLBI level hints (FEAT_TTL)
    pub isd128: bool,
    pub xs: bool, // XS attribute (FEAT_XS)
}

impl Default for TLBContext {
    fn default() -> Self {
        TLBContext {
            ss: SecurityState::NonSecure,
            regime: Regime::EL10,
            vmid: 0,
            asid: 0,
            ng: false,
            ipaspace: PASpace::NonSecure,
            includes_s1: false,
            includes_s2: false,
            includes_gpt: false,
            ia: 0,
            tg: TGx::TG4KB,
            cnp: false,
            level: -1,
            isd128: false,
            xs: false,
        }
    }
}

#[derive(Clone)]
pub struct TLBRecord {
    // pub context: TLBContext, // Kept separate, unlike in Sail
    pub walkstate: TTWState,
    pub blocksize: u64,     // Number of bits directly mapped from IA to OA
    pub contigsize: u64,    // Number of entries log 2 marking a contiguous output range
    pub s1descriptor: u128, // Stage 1 leaf descriptor in memory (valid if the TLB caches stage 1)
    pub s2descriptor: u128, // Stage 2 leaf descriptor in memory (valid if the TLB caches stage 2)
}

impl Default for TLBRecord {
    fn default() -> Self {
        Self {
            walkstate: TTWState::default(),
            blocksize: 0,
            contigsize: 0,
            s1descriptor: 0,
            s2descriptor: 0,
        }
    }
}

pub struct TLBLine {
    pub valid_name: bool,
    pub tlbrecord: TLBRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorType {
    Table,
    Leaf,
    Invalid,
}

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn translation_regime(&mut self, el: EL) -> Regime {
        match el {
            EL::EL3 => {
                return if self.el_using_aarch32(EL::EL3) {
                    Regime::EL30
                } else {
                    Regime::EL3
                };
            }
            EL::EL2 => {
                return if self.el_is_in_host(EL::EL2) {
                    Regime::EL20
                } else {
                    Regime::EL2
                };
            }
            EL::EL1 => {
                return Regime::EL10;
            }
            EL::EL0 => {
                if self.current_security_state() == SecurityState::Secure
                    && self.el_using_aarch32(EL::EL3)
                {
                    return Regime::EL30;
                } else if self.el_is_in_host(EL::EL0) {
                    return Regime::EL20;
                } else {
                    return Regime::EL10;
                }
            }
        }
    }

    pub fn fetch_descriptor(
        &mut self,
        ee: bool,
        walkaddress: &AddressDescriptor,
        walkaccess: &AccessDescriptor,
        fault: &mut FaultRecord,
        n: usize,
    ) -> Option<u128> {
        if self.is_feat_impl(Feat::RME) {
            fault.gpcf = self.granule_protection_check(walkaddress, walkaccess);
            if fault.gpcf.gpf != GPCF::None {
                fault.statuscode = Fault::GPCFOnWalk;
                fault.paddress = walkaddress.paddress;
                fault.gpcfs2walk = fault.secondstage;
                return None;
            }
        }

        let (memstatus, descriptor) = self.phys_mem_read(walkaddress, n / 8, walkaccess);
        if memstatus.is_fault() {
            todo!("memstatus.is_fault()")
        }
        let descriptor = descriptor.unwrap();
        let descriptor = match n / 8 {
            1 => {
                let mut out = [0; 1];
                out.copy_from_slice(&descriptor[0..1]);
                u8::from_le_bytes(out) as u128
            }
            2 => {
                let mut out = [0; 2];
                out.copy_from_slice(&descriptor[0..2]);
                u16::from_le_bytes(out) as u128
            }
            4 => {
                let mut out = [0; 4];
                out.copy_from_slice(&descriptor[0..4]);
                u32::from_le_bytes(out) as u128
            }
            8 => {
                let mut out = [0; 8];
                out.copy_from_slice(&descriptor[0..8]);
                u64::from_le_bytes(out) as u128
            }
            16 => u128::from_le_bytes(descriptor),
            _ => panic!("Unsupported phys_mem_read size {0:?}", n / 8),
        };

        if ee {
            todo!("descriptor = BigEndianReverse(descriptor)");
        }

        return Some(descriptor);
    }

    pub fn stage_oa(&mut self, ia: u64, d128: bool, tgx: TGx, walkstate: &TTWState) -> FullAddress {
        let tsize = tgx.translation_size(d128, walkstate.level);
        let csize = if walkstate.contiguous.is_some_and(|c| c) {
            tgx.contiguous_size(d128, walkstate.level)
        } else {
            0
        };

        let ia_msb = tsize + csize;

        // oa.address = walkstate.baseaddress.address<55:ia_msb>:ia<ia_msb-1:0>;
        let mut oa = ia & ((1 << ((ia_msb - 1) - 0 + 1)) - 1);
        oa |=
            ((walkstate.baseaddress.address >> ia_msb) & ((1 << (55 - ia_msb + 1)) - 1)) << ia_msb;

        FullAddress {
            paspace: walkstate.baseaddress.paspace,
            address: oa,
        }
    }
}

#[derive(Clone)]
pub struct TTWState {
    pub istable: bool,
    pub level: i64,
    pub baseaddress: FullAddress,
    pub contiguous: Option<bool>,
    pub s1assured: bool,     // Stage 1 Assured Translation Property
    pub s2assuredonly: bool, // Stage 2 AssuredOnly attribute
    pub disch: bool,         // Stage 1 Disable Contiguous Hint
    pub ng: bool,
    pub guardedpage: bool,
    pub sdftype: Option<SDFType>, // AArch32 Short-descriptor format walk only
    pub domain: Option<u64>,      // AArch32 Short-descriptor format walk only
    pub memattrs: MemoryAttributes,
    pub permissions: Permissions,
}

impl Default for TTWState {
    fn default() -> Self {
        Self {
            istable: false,
            level: -1,
            baseaddress: FullAddress {
                paspace: PASpace::NonSecure,
                address: 0,
            },
            contiguous: None,
            s1assured: false,
            s2assuredonly: false,
            disch: false,
            ng: false,
            guardedpage: false,
            sdftype: None,
            domain: None,
            memattrs: MemoryAttributes::normal_nc(false),
            permissions: Permissions::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum SDFType {
    Table,
    Invalid,
    Supersection,
    Section,
    LargePage,
    SmallPage,
}

#[derive(Clone)]
pub struct AddressDescriptor {
    pub fault: FaultRecord, // fault.statuscode indicates whether the address is valid
    pub memattrs: Option<MemoryAttributes>,
    pub paddress: Option<FullAddress>,
    pub s1assured: bool,        // Stage 1 Assured Translation Property
    pub s2fs1mro: Option<bool>, // Stage 2 MRO permission for Stage 1
    pub mecid: Option<u64>,     // FEAT_MEC: Memory Encryption Context ID
    pub vaddress: u64,
    pub tlbcontext: Option<TLBContext>, // From the Sail model for __tlb_enabled
}

impl AddressDescriptor {
    pub fn is_fault(&self) -> bool {
        Fault::None != self.fault.statuscode
    }

    pub fn create(va: u64, pa: Option<FullAddress>, memattrs: MemoryAttributes) -> Self {
        Self {
            fault: FaultRecord::no_fault(),
            memattrs: Some(memattrs),
            paddress: pa,
            s1assured: false,
            s2fs1mro: Some(false),
            mecid: None,
            vaddress: va,
            tlbcontext: None,
        }
    }

    pub fn create_faulty(va: u64, fault: FaultRecord) -> Self {
        Self {
            fault,
            memattrs: None,
            paddress: None,
            s1assured: false,
            s2fs1mro: None,
            mecid: None,
            vaddress: va,
            tlbcontext: None,
        }
    }
}

pub struct S1TTWParams {
    // A64-VMSA exclusive parameters
    pub ha: bool,    // TCR_ELx.HA
    pub hd: bool,    // TCR_ELx.HD
    pub tbi: bool,   // TCR_ELx.TBI{x}
    pub tbid: bool,  // TCR_ELx.TBID{x}
    pub nfd: bool,   // TCR_EL1.NFDx or TCR_EL2.NFDx when HCR_EL2.E2H == '1'
    pub e0_pd: bool, // TCR_EL1.E0PDx or TCR_EL2.E0PDx when HCR_EL2.E2H == '1'
    pub d128: bool,  // TCR_ELx.D128
    pub aie: bool,   // (TCR2_ELx/TCR_EL3).AIE
    pub mair2: u64,  // MAIR2_ELx
    pub ds: bool,    // TCR_ELx.DS
    pub ps: u64,     // TCR_ELx.{I}PS
    pub txsz: u64,   // TCR_ELx.TxSZ
    pub epan: bool,  // SCTLR_EL1.EPAN or SCTLR_EL2.EPAN when HCR_EL2.E2H == '1'
    pub dct: bool,   // HCR_EL2.DCT
    pub nv1: bool,   // HCR_EL2.NV1
    pub cmow: bool,  // SCTLR_EL1.CMOW or SCTLR_EL2.CMOW when HCR_EL2.E2H == '1'
    pub pnch: bool,  // TCR{2}_ELx.PnCH
    pub disch: bool, // TCR{2}_ELx.DisCH
    pub haft: bool,  // TCR{2}_ELx.HAFT
    pub mtx: bool,   // TCR_ELx.MTX{y}
    pub skl: u64,    // TCR_ELx.SKL
    pub pie: bool,   // TCR2_ELx.PIE or TCR_EL3.PIE
    pub pir: u64,    // PIR_ELx
    pub pire0: u64,  // PIRE0_EL1 or PIRE0_EL2 when HCR_EL2.E2H == '1'
    pub emec: bool,  // SCTLR2_EL2.EMEC or SCTLR2_EL3.EMEC
    pub amec: bool,  // TCR2_EL2.AMEC0 or TCR2_EL2.AMEC1 when HCR_EL2.E2H == '1'
    pub fng: bool,   // TCR2_EL1.FNGx or TCR2_EL2.FNGx when HCR_EL2.E2H == '1'

    // A32-VMSA exclusive parameters
    pub t0sz: u64,  // TTBCR.T0SZ
    pub t1sz: u64,  // TTBCR.T1SZ
    pub uwxn: bool, // SCTLR.UWXN

    // Parameters common to both A64-VMSA & A32-VMSA (A64/A32)
    pub tgx: TGx,     // TCR_ELx.TGx      / Always TGx_4KB
    pub irgn: u64,    // TCR_ELx.IRGNx    / TTBCR.IRGNx or HTCR.IRGN0
    pub orgn: u64,    // TCR_ELx.ORGNx    / TTBCR.ORGNx or HTCR.ORGN0
    pub sh: u64,      // TCR_ELx.SHx      / TTBCR.SHx or HTCR.SH0
    pub hpd: bool,    // TCR_ELx.HPD{x}   / TTBCR2.HPDx or HTCR.HPD
    pub ee: bool,     // SCTLR_ELx.EE     / SCTLR.EE or HSCTLR.EE
    pub wxn: bool,    // SCTLR_ELx.WXN    / SCTLR.WXN or HSCTLR.WXN
    pub ntlsmd: bool, // SCTLR_ELx.nTLSMD / SCTLR.nTLSMD or HSCTLR.nTLSMD
    pub dc: bool,     // HCR_EL2.DC       / HCR.DC
    pub sif: bool,    // SCR_EL3.SIF      / SCR.SIF
    pub mair: u64,    // MAIR_ELx         / MAIR1:MAIR0 or HMAIR1:HMAIR0
}

impl S1TTWParams {
    pub fn aarch64_s1_start_level(&self) -> i64 {
        // Input Address size
        let iasize = 64 - self.txsz as i64;
        let granulebits = self.tgx.granule_bits() as i64;
        let descsizelog2 = if self.d128 { 4 } else { 3 };
        let stride = granulebits - descsizelog2;
        let s1startlevel = 3 - (((iasize - 1) - granulebits) / stride);
        if self.d128 {
            return s1startlevel + self.skl as i64;
        }
        s1startlevel
    }
}

pub struct S2TTWParams {
    // A64-VMSA exclusive parameters
    pub ha: bool,          // VTCR_EL2.HA
    pub hd: bool,          // VTCR_EL2.HD
    pub sl2: bool,         // V{S}TCR_EL2.SL2
    pub ds: bool,          // VTCR_EL2.DS
    pub d128: bool,        // VTCR_ELx.D128
    pub sw: bool,          // VSTCR_EL2.SW
    pub nsw: bool,         // VTCR_EL2.NSW
    pub sa: bool,          // VSTCR_EL2.SA
    pub nsa: bool,         // VTCR_EL2.NSA
    pub ps: u64,           // VTCR_EL2.PS
    pub txsz: u64,         // V{S}TCR_EL2.T0SZ
    pub fwb: bool,         // HCR_EL2.FWB
    pub cmow: bool,        // HCRX_EL2.CMOW
    pub skl: u64,          // VTCR_EL2.SKL
    pub s2pie: bool,       // VTCR_EL2.S2PIE
    pub s2pir: u64,        // S2PIR_EL2
    pub tl0: bool,         // VTCR_EL2.TL0
    pub tl1: bool,         // VTCR_EL2.TL1
    pub assuredonly: bool, // VTCR_EL2.AssuredOnly
    pub haft: bool,        // VTCR_EL2.HAFT
    pub emec: bool,        // SCTLR2_EL2.EMEC
    pub hdbss: bool,       // VTCR_EL2.HDBSS

    // A32-VMSA exclusive parameters
    pub s: bool,   // VTCR.S
    pub t0sz: u64, // VTCR.T0SZ

    // Parameters common to both A64-VMSA & A32-VMSA if implemented (A64/A32)
    pub tgx: TGx,  // V{S}TCR_EL2.TG0  / Always TGx_4KB
    pub sl0: u64,  // V{S}TCR_EL2.SL0  / VTCR.SL0
    pub irgn: u64, // VTCR_EL2.IRGN0   / VTCR.IRGN0
    pub orgn: u64, // VTCR_EL2.ORGN0   / VTCR.ORGN0
    pub sh: u64,   // VTCR_EL2.SH0     / VTCR.SH0
    pub ee: bool,  // SCTLR_EL2.EE     / HSCTLR.EE
    pub ptw: bool, // HCR_EL2.PTW      / HCR.PTW
    pub vm: bool,  // HCR_EL2.VM       / HCR.VM
}

impl S2TTWParams {
    pub fn aarch64_s2_start_level(&self) -> i64 {
        if self.d128 {
            let iasize = 64 - self.txsz as i64;
            let granulebits = self.tgx.granule_bits() as i64;
            let descsizelog2 = 4;
            let stride = granulebits - descsizelog2;
            let s2startlevel = 3 - (((iasize - 1) - granulebits) / stride);
            let s2startlevel = s2startlevel + self.skl as i64;

            return s2startlevel;
        }

        match self.tgx {
            TGx::TG4KB => match (self.sl2, self.sl0) {
                (false, 0b00) => return 2,
                (false, 0b01) => return 1,
                (false, 0b10) => return 0,
                (false, 0b11) => return 3,
                (true, 0b00) => return -1,
                _ => panic!("Unreachable"),
            },
            TGx::TG16KB => match self.sl0 {
                0b00 => 3,
                0b01 => 2,
                0b10 => 1,
                0b11 => 0,
                _ => panic!("Unreachable"),
            },
            TGx::TG64KB => match self.sl0 {
                0b00 => 3,
                0b01 => 2,
                0b10 => 1,
                _ => panic!("Unreachable"),
            },
        }
    }
}
