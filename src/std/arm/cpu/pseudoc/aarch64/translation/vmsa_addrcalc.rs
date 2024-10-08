use std::cmp::{max, min};

use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_leaf_base(
        &mut self,
        descriptor: u128,
        d128: bool,
        ds: bool,
        tgx: TGx,
        level: i64,
    ) -> u64 {
        let mut leafbase = 0;

        let granulebits = tgx.granule_bits() as i64;
        let descsizelog2 = if d128 { 4 } else { 3 } as i64;
        let stride = granulebits - descsizelog2;
        let leafsize = granulebits + stride * (3 - level);

        // leafbase<47:0> = Align(descriptor<47:0>, 1 << leafsize);
        let descriptor_47_0 = descriptor & ((1 << (47 - 0 + 1)) - 1);
        leafbase |= (1 << leafsize) * (descriptor_47_0 / (1 << leafsize));

        if d128 {
            // leafbase<55:48> = descriptor<55:48>;
            leafbase |= ((descriptor >> 48) & ((1 << (55 - 48 + 1)) - 1)) << 48;
        } else if tgx == TGx::TG64KB
            && (self.aarch64_pa_max() >= 52
                || self
                    .cpu
                    .impdef
                    .bool(&"descriptor[15:12] for 64KB granule are OA[51:48]"))
        {
            // leafbase<51:48> = descriptor<15:12>;
            leafbase |= ((descriptor >> 12) & ((1 << (15 - 12 + 1)) - 1)) << 48;
        } else if ds {
            // leafbase<51:48> = descriptor<9:8>:descriptor<49:48>;
            leafbase |= ((descriptor >> 8) & ((1 << (9 - 8 + 1)) - 1)) << 50;
            leafbase |= ((descriptor >> 48) & ((1 << (49 - 48 + 1)) - 1)) << 48;
        }

        return leafbase as u64;
    }

    pub fn aarch64_next_table_base(
        &mut self,
        descriptor: u128,
        d128: bool,
        skl: i64,
        ds: bool,
        tgx: TGx,
    ) -> u64 {
        let mut tablebase = 0;
        let granulebits = tgx.granule_bits() as i64;

        let tablesize = if d128 {
            let descsizelog2 = 4;
            let stride = granulebits - descsizelog2;
            stride * (1 + skl) + descsizelog2
        } else {
            granulebits
        };

        match tgx {
            TGx::TG4KB => {
                // tablebase<47:12> = descriptor<47:12>;
                tablebase |= ((descriptor >> 12) & ((1 << (47 - 12 + 1)) - 1)) << 12;
            }
            TGx::TG16KB => {
                // tablebase<47:14> = descriptor<47:14>;
                tablebase |= ((descriptor >> 14) & ((1 << (47 - 14 + 1)) - 1)) << 14;
            }
            TGx::TG64KB => {
                // tablebase<47:16> = descriptor<47:16>;
                tablebase |= ((descriptor >> 16) & ((1 << (47 - 16 + 1)) - 1)) << 16;
            }
        };

        // tablebase = Align(tablebase, 1 << tablesize);
        tablebase = (1 << tablesize) * (tablebase / (1 << tablesize));

        if d128 {
            // tablebase<55:48> = descriptor<55:48>;
            tablebase |= ((descriptor >> 48) & ((1 << (55 - 48 + 1)) - 1)) << 48;
        } else if tgx == TGx::TG64KB
            && (self.aarch64_pa_max() >= 52
                || self
                    .cpu
                    .impdef
                    .bool(&"descriptor[15:12] for 64KB granule are OA[51:48]"))
        {
            // tablebase<51:48> = descriptor<15:12>;
            tablebase |= ((descriptor >> 12) & ((1 << (15 - 12 + 1)) - 1)) << 48;
        } else if ds {
            // tablebase<51:48> = descriptor<9:8>:descriptor<49:48>;
            tablebase |= ((descriptor >> 8) & ((1 << (9 - 8 + 1)) - 1)) << 50;
            tablebase |= ((descriptor >> 48) & ((1 << (49 - 48 + 1)) - 1)) << 48;
        }

        return tablebase as u64;
    }

    pub fn aarch64_physical_address_size(&mut self, d128: bool, ps: u64, tgx: TGx) -> u64 {
        let ps = match ps {
            0b000 => 32,
            0b001 => 36,
            0b010 => 40,
            0b011 => 42,
            0b100 => 44,
            0b101 => 48,
            0b110 => 52,
            0b111 => 56,
            _ => panic!("Unreachable"),
        };

        let max_ps;
        if d128 {
            max_ps = self.aarch64_pa_max();
        } else if self.is_feat_impl(Feat::LPA)
            && (tgx == TGx::TG64KB || self.is_feat_impl(Feat::LPA2))
        {
            max_ps = min(52, self.aarch64_pa_max());
        } else {
            max_ps = min(48, self.aarch64_pa_max());
        }

        return min(ps, max_ps);
    }

    pub fn aarch64_s1_sl_tt_entry_address(
        &mut self,
        level: i64,
        walkparams: &S1TTWParams,
        ia: u64,
        tablebase: &FullAddress,
    ) -> FullAddress {
        // Input Address size
        let descsizelog2 = if walkparams.d128 { 4 } else { 3 };
        let iasize = 64 - walkparams.txsz as i64;
        let granulebits = walkparams.tgx.granule_bits() as i64;
        let stride = granulebits - descsizelog2;
        let levels = 3 - level;

        let lsb = levels * stride + granulebits;
        let msb = iasize - 1;
        // index = ZeroExtend(ia<msb:lsb>:Zeros(descsizelog2), 56);
        let index = ((ia >> lsb) & ((1 << (msb - lsb + 1)) - 1)) << descsizelog2;

        FullAddress {
            paspace: tablebase.paspace,
            address: tablebase.address | index,
        }
    }

    pub fn aarch64_s1_tt_base_address(
        &mut self,
        walkparams: &S1TTWParams,
        regime: Regime,
        ttbr: u128,
    ) -> u64 {
        let mut tablebase = 0;

        // Input Address size
        let iasize = 64 - walkparams.txsz as i64;
        let granulebits = walkparams.tgx.granule_bits() as i64;
        let descsizelog2 = if walkparams.d128 { 4 } else { 3 };
        let stride = granulebits - descsizelog2;
        let startlevel = walkparams.aarch64_s1_start_level();
        let levels = 3 - startlevel;

        // Base address is aligned to size of the initial translation table in bytes
        let mut tsize = (iasize - (levels * stride + granulebits)) + descsizelog2;

        if walkparams.d128 {
            tsize = max(tsize, 5);
            if regime == Regime::EL3 {
                // tablebase<55:5> = ttbr<55:5>;
                tablebase |= ((ttbr >> 5) & ((1 << (55 - 5 + 1)) - 1)) << 5;
            } else {
                // tablebase<55:5> = ttbr<87:80>:ttbr<47:5>;
                tablebase |= ((ttbr >> 80) & ((1 << (87 - 80 + 1)) - 1)) << 49;
                tablebase |= ((ttbr >> 5) & ((1 << (47 - 5 + 1)) - 1)) << 5;
            }
        } else if walkparams.ds
            || (walkparams.tgx == TGx::TG64KB
                && walkparams.ps == 0b110
                && (self.is_feat_impl(Feat::LPA)
                    || self
                        .cpu
                        .impdef
                        .bool(&"BADDR expresses 52 bits for 64KB granule")))
        {
            tsize = max(tsize, 6);
            // tablebase<51:6> = ttbr<5:2>:ttbr<47:6>;
            tablebase |= ((ttbr >> 2) & ((1 << (5 - 2 + 1)) - 1)) << 48;
            tablebase |= ((ttbr >> 6) & ((1 << (47 - 6 + 1)) - 1)) << 6;
        } else {
            // tablebase<47:1> = ttbr<47:1>;
            tablebase |= ((ttbr >> 1) & ((1 << (47 - 1 + 1)) - 1)) << 1;
        }

        // tablebase = Align(tablebase, 1 << tsize);
        tablebase = (1 << tsize) * (tablebase / (1 << tsize));
        return tablebase as u64;
    }

    pub fn arch64_s2_sl_tt_entry_address(
        &mut self,
        walkparams: &S2TTWParams,
        ipa: u64,
        tablebase: &FullAddress,
    ) -> FullAddress {
        let startlevel = walkparams.aarch64_s2_start_level();
        let iasize = 64 - walkparams.txsz as i64;
        let granulebits = walkparams.tgx.granule_bits() as i64;
        let descsizelog2 = if walkparams.d128 { 4 } else { 3 };
        let stride = granulebits - descsizelog2;
        let levels = 3 - startlevel;

        let lsb = levels * stride + granulebits;
        let msb = iasize - 1;
        // index = ZeroExtend(ipa<msb:lsb>:Zeros(descsizelog2), 56);
        let index = ((ipa >> lsb) & ((1 << (msb - lsb + 1)) - 1)) << descsizelog2;

        FullAddress {
            address: tablebase.address | index,
            paspace: tablebase.paspace,
        }
    }

    pub fn aarch64_s2_tt_base_address(
        &mut self,
        walkparams: &S2TTWParams,
        paspace: PASpace,
        ttbr: u128,
    ) -> u64 {
        let mut tablebase = 0;

        // Input Address size
        let iasize = 64 - walkparams.txsz as i64;
        let granulebits = walkparams.tgx.granule_bits() as i64;
        let descsizelog2 = if walkparams.d128 { 4 } else { 3 };
        let stride = granulebits - descsizelog2;
        let startlevel = walkparams.aarch64_s2_start_level();
        let levels = 3 - startlevel;

        // Base address is aligned to size of the initial translation table in bytes
        let mut tsize = (iasize - (levels * stride + granulebits)) + descsizelog2;

        if walkparams.d128 {
            tsize = max(tsize, 5);
            if paspace == PASpace::Secure {
                // tablebase<55:5> = ttbr<55:5>;
                tablebase |= ((ttbr >> 5) & ((1 << (55 - 5 + 1)) - 1)) << 5;
            } else {
                // tablebase<55:5> = ttbr<87:80>:ttbr<47:5>;
                tablebase |= ((ttbr >> 80) & ((1 << (87 - 80 + 1)) - 1)) << 49;
                tablebase |= ((ttbr >> 5) & ((1 << (47 - 5 + 1)) - 1)) << 5;
            }
        } else if walkparams.ds
            || (walkparams.tgx == TGx::TG64KB
                && walkparams.ps == 0b110
                && (self.is_feat_impl(Feat::LPA)
                    || self
                        .cpu
                        .impdef
                        .bool(&"BADDR expresses 52 bits for 64KB granule")))
        {
            tsize = max(tsize, 6);
            // tablebase<51:6> = ttbr<5:2>:ttbr<47:6>;
            tablebase |= ((ttbr >> 2) & ((1 << (5 - 2 + 1)) - 1)) << 48;
            tablebase |= ((ttbr >> 6) & ((1 << (47 - 6 + 1)) - 1)) << 6;
        } else {
            // tablebase<47:1> = ttbr<47:1>;
            tablebase |= ((ttbr >> 1) & ((1 << (47 - 1 + 1)) - 1)) << 1;
        }
        // tablebase = Align(tablebase, 1 << tsize);
        tablebase = (1 << tsize) * (tablebase / (1 << tsize));

        return tablebase as u64;
    }

    pub fn aarch64_tt_entry_address(
        &mut self,
        level: i64,
        d128: bool,
        skl: i64,
        tgx: TGx,
        txsz: u64,
        ia: u64,
        tablebase: &FullAddress,
    ) -> FullAddress {
        let _iasize = 64 - txsz as i64;
        let granulebits = tgx.granule_bits() as i64;
        let descsizelog2 = if d128 { 4 } else { 3 };
        let stride = granulebits - descsizelog2;
        let levels = 3 - level;

        let lsb = levels * stride + granulebits;
        let nstride = if d128 { skl + 1 } else { 1 };
        let msb = (lsb + (stride * nstride)) - 1;
        // index = ZeroExtend(ia<msb:lsb>:Zeros(descsizelog2), 56);
        let index = ((ia >> lsb) & ((1 << (msb - lsb + 1)) - 1)) << descsizelog2;

        FullAddress {
            paspace: tablebase.paspace,
            address: tablebase.address | index,
        }
    }
}
