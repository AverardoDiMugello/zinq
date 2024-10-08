use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_block_desc_supported(
        &mut self,
        d128: bool,
        ds: bool,
        tgx: TGx,
        level: i64,
    ) -> bool {
        match tgx {
            TGx::TG4KB => (level == 0 && (ds || d128)) || level == 1 || level == 2,
            TGx::TG16KB => (level == 1 && (ds || d128)) || level == 2,
            TGx::TG64KB => (level == 1 && (d128 || self.aarch64_pa_max() >= 52)) || level == 2,
        }
    }

    pub fn aarch64_block_nt_faults(&mut self, d128: bool, descriptor: u128) -> bool {
        if !self.is_feat_impl(Feat::BBM) {
            return false;
        }
        let nt = if d128 {
            (descriptor >> 6) & 1 != 0
        } else {
            (descriptor >> 16) & 1 != 0
        };
        let bbm_level = self.aarch64_block_bbm_support_level();
        if bbm_level.is_none() {
            return false;
        }
        let bbm_level = bbm_level.unwrap();

        let nt_faults = self
            .cpu
            .impdef
            .bool(&"BBM level 1 or 2 support nT bit causes Translation Fault");

        return (bbm_level == 1 || bbm_level == 2) && nt && nt_faults;
    }

    pub fn aarch64_decode_descriptor_type(
        &mut self,
        descriptor: u128,
        d128: bool,
        ds: bool,
        tgx: TGx,
        level: i64,
    ) -> DescriptorType {
        if descriptor & 1 == 0 {
            panic!("descriptor & 1 == 0");
            return DescriptorType::Invalid;
        } else if d128 {
            // bits(2) skl = descriptor<110:109>;
            let skl = (descriptor >> 109) & 0b11;
            if (tgx == TGx::TG16KB || tgx == TGx::TG64KB) && skl == 3 {
                panic!("(tgx == TGx::TG16KB || tgx == TGx::TG64KB) && skl == 3");
                return DescriptorType::Invalid;
            }

            let effective_level = level + skl as i64;
            if effective_level > 3 {
                panic!("effective_level > 3");
                return DescriptorType::Invalid;
            } else if effective_level == 3 {
                return DescriptorType::Leaf;
            } else {
                return DescriptorType::Table;
            }
        } else {
            if (descriptor >> 1) & 1 != 0 {
                if level == 3 {
                    return DescriptorType::Leaf;
                } else {
                    return DescriptorType::Table;
                }
            } else {
                if self.aarch64_block_desc_supported(d128, ds, tgx, level) {
                    return DescriptorType::Leaf;
                } else {
                    panic!("!self.aarch64_block_desc_supported(d128, ds, tgx, level)");
                    return DescriptorType::Invalid;
                }
            }
        }
    }

    pub fn aarch64_s1_apply_output_perms(
        &mut self,
        permissions: Permissions,
        descriptor: u128,
        regime: Regime,
        walkparams: &S1TTWParams,
    ) -> Permissions {
        let mut permissions = permissions;

        if walkparams.pie {
            let pi_index = if walkparams.d128 {
                // pi_index = descriptor<118:115>;
                (descriptor >> 115) & ((1 << (118 - 115 + 1)) - 1)
            } else {
                // pi_index = descriptor<54:53>:descriptor<51>:descriptor<6>;
                let desc_6 = (descriptor >> 6) & 1;
                let desc_51 = (descriptor >> 51) & 1;
                let desc_54_53 = (descriptor >> 53) & ((1 << (54 - 53 + 1)) - 1);
                (desc_54_53 << 2) | (desc_51 << 1) | desc_6
            };
            // permissions.ppi    = Elem[walkparams.pir, UInt(pi_index), 4];
            permissions.ppi = (walkparams.pir >> (pi_index * 4)) & 0b1111;
            // permissions.upi    = Elem[walkparams.pire0, UInt(pi_index), 4];
            permissions.upi = (walkparams.pire0 >> (pi_index * 4)) & 0b1111;
            // permissions.ndirty = descriptor<7>;
            permissions.ndirty = (descriptor >> 7) & 1 != 0;
        } else {
            if regime == Regime::EL10 && self.el2_enabled() && walkparams.nv1 {
                // permissions.ap<2:1> = descriptor<7>:'0';
                permissions.ap |= (((descriptor >> 7) & 1) << 2) as u64;
                permissions.ap &= !(1 << 1);
                // permissions.pxn     = descriptor<54>;
                permissions.pxn = ((descriptor >> 54) & 1) != 0;
            } else if regime.has_unprivileged() {
                // permissions.ap<2:1> = descriptor<7:6>;
                permissions.ap |= (((descriptor >> 7) & 1) << 2) as u64;
                permissions.ap |= (((descriptor >> 6) & 1) << 1) as u64;
                // permissions.uxn     = descriptor<54>;
                permissions.uxn = ((descriptor >> 54) & 1) != 0;
                // permissions.pxn     = descriptor<53>;
                permissions.pxn = ((descriptor >> 53) & 1) != 0;
            } else {
                // permissions.ap<2:1> = descriptor<7>:'1';
                permissions.ap |= (((descriptor >> 7) & 1) << 2) as u64;
                permissions.ap |= 1 << 1;
                // permissions.xn      = descriptor<54>;
                permissions.xn = ((descriptor >> 54) & 1) != 0;
            }
            // Descriptors marked with DBM set have the effective value of AP[2] cleared.
            // This implies no Permission faults caused by lack of write permissions are
            // reported, and the Dirty bit can be set.
            if walkparams.ha && walkparams.hd && (descriptor >> 51) & 1 != 0 {
                // permissions.ap<2> = '0';
                permissions.ap &= !(1 << 2);
            }
        }

        if self.is_feat_impl(Feat::S1POE) {
            if walkparams.d128 {
                // permissions.po_index = descriptor<124:121>;
                permissions.po_index = ((descriptor >> 121) & ((1 << (124 - 121 + 1)) - 1)) as u64;
            } else {
                // permissions.po_index = '0':descriptor<62:60>;
                permissions.po_index = ((descriptor >> 60) & ((1 << (62 - 60 + 1)) - 1)) as u64;
            }
        }
        return permissions;
    }

    pub fn aarch64_s1_apply_table_perms(
        &mut self,
        permissions: Permissions,
        descriptor: u128,
        regime: Regime,
        walkparams: &S1TTWParams,
    ) -> Permissions {
        let mut permissions = permissions;
        let ap_table;
        if regime == Regime::EL10 && self.el2_enabled() && walkparams.nv1 {
            let pxn_table;
            if walkparams.d128 {
                // ap_table  = descriptor<126>:'0';
                ap_table = ((descriptor >> 126) & 1) << 1;
                // pxn_table = descriptor<124>;
                pxn_table = (descriptor >> 124) & 1 != 0;
            } else {
                // ap_table  = descriptor<62>:'0';
                ap_table = ((descriptor >> 62) & 1) << 1;
                // pxn_table = descriptor<60>;
                pxn_table = (descriptor >> 60) & 1 != 0;
            }
            permissions.ap_table = permissions.ap_table | ap_table as u64;
            permissions.pxn_table = permissions.pxn_table | pxn_table;
        } else if regime.has_unprivileged() {
            let pxn_table;
            let uxn_table;
            if walkparams.d128 {
                // ap_table  = descriptor<126:125>;
                ap_table = (descriptor >> 125) & 0b11;
                // uxn_table = descriptor<124>;
                uxn_table = (descriptor >> 124) & 1 != 0;
                // pxn_table = descriptor<123>;
                pxn_table = (descriptor >> 123) & 1 != 0;
            } else {
                // ap_table  = descriptor<62:61>;
                ap_table = (descriptor >> 61) & 0b11;
                // uxn_table = descriptor<60>;
                uxn_table = (descriptor >> 60) & 1 != 0;
                // pxn_table = descriptor<59>;
                pxn_table = (descriptor >> 59) & 1 != 0;
            }
            permissions.ap_table = permissions.ap_table | ap_table as u64;
            permissions.uxn_table = permissions.uxn_table | uxn_table;
            permissions.pxn_table = permissions.pxn_table | pxn_table;
        } else {
            let xn_table;
            if walkparams.d128 {
                // ap_table = descriptor<126>:'0';
                ap_table = ((descriptor >> 126) & 1) << 1;
                // xn_table = descriptor<124>;
                xn_table = (descriptor >> 124) & 1 != 0;
            } else {
                // ap_table = descriptor<62>:'0';
                ap_table = ((descriptor >> 62) & 1) << 1;
                // xn_table = descriptor<60>;
                xn_table = (descriptor >> 60) & 1 != 0;
            }
            permissions.ap_table = permissions.ap_table | ap_table as u64;
            permissions.xn_table = permissions.xn_table | xn_table;
        }
        return permissions;
    }

    pub fn aarch64_s2_apply_output_perms(
        &mut self,
        descriptor: u128,
        walkparams: &S2TTWParams,
    ) -> Permissions {
        let mut permissions = Permissions::default();
        let s2pi_index;
        if walkparams.s2pie {
            if walkparams.d128 {
                // s2pi_index = descriptor<118:115>;
                s2pi_index = (descriptor >> 115) & 0b1111;
            } else {
                // s2pi_index = descriptor<54:53,51,6>;
                s2pi_index = (((descriptor >> 53) & 0b11) << 1)
                    | (((descriptor >> 51) & 1) << 1)
                    | ((descriptor >> 6) & 1)
            }
            // permissions.s2pi = Elem[walkparams.s2pir, UInt(s2pi_index), 4];
            permissions.s2pi = (walkparams.s2pir >> (s2pi_index * 4)) & 0b1111;
            // permissions.s2dirty = descriptor<7>;
            permissions.s2dirty = (descriptor >> 7) & 1 != 0;
        } else {
            // permissions.s2ap = descriptor<7:6>;
            permissions.s2ap = ((descriptor >> 6) & 0b11) as u64;
            if walkparams.d128 {
                // permissions.s2xn = descriptor<118>;
                permissions.s2xn = (descriptor >> 118) & 1 != 0;
            } else {
                // permissions.s2xn = descriptor<54>;
                permissions.s2xn = (descriptor >> 54) & 1 != 0;
            }
            if self.is_feat_impl(Feat::XNX) {
                if walkparams.d128 {
                    // permissions.s2xnx = descriptor<117>;
                    permissions.s2xnx = (descriptor >> 117) & 1 != 0;
                } else {
                    // permissions.s2xnx = descriptor<53>;
                    permissions.s2xnx = (descriptor >> 53) & 1 != 0;
                }
            } else {
                permissions.s2xnx = false;
            }
            // Descriptors marked with DBM set have the effective value of S2AP[1] set.
            // This implies no Permission faults caused by lack of write permissions are
            // reported, and the Dirty bit can be set.
            let desc_dbm;
            if walkparams.d128 {
                // desc_dbm = descriptor<115>;
                desc_dbm = (descriptor >> 115) & 1 != 0;
            } else {
                // desc_dbm = descriptor<51>;
                desc_dbm = (descriptor >> 51) & 1 != 0;
            }
            if walkparams.ha && walkparams.hd && desc_dbm {
                // permissions.s2ap<1> = '1';
                permissions.s2ap |= 0b10;
            }
        }

        if self.is_feat_impl(Feat::S2POE) {
            if walkparams.d128 {
                // permissions.s2po_index = descriptor<124:121>;
                permissions.s2po_index = ((descriptor >> 121) & 0b1111) as u64;
            } else {
                // permissions.s2po_index = descriptor<62:59>;
                permissions.s2po_index = ((descriptor >> 59) & 0b1111) as u64;
            }
        }
        return permissions;
    }
}
