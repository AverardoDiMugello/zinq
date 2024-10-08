use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::aarch64::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    const MAX_ZERO_BLOCK_SIZE: u64 = 2048;

    pub fn aarch64_mem_zero(&mut self, regval: u64, cachetype: CacheType) {
        let size = 4 * (2u64.pow(self.read::<dczid_el0::BS>() as u32));
        assert!(size <= ArmCtx::MAX_ZERO_BLOCK_SIZE);
        if self.is_feat_impl(Feat::MTE2) {
            assert!(size >= ArmCtx::TAG_GRANULE);
        }

        // Align(regval, size)
        let vaddress = size * (regval / size);

        let accdesc = self.create_acc_desc_dc_zero(CacheType::Data);

        if matches!(cachetype, CacheType::Tag | CacheType::DataTag) {
            todo!("AArch64.WriteTagMem(regval, vaddress, accdesc, size)");
        }

        if matches!(cachetype, CacheType::Data | CacheType::DataTag) {
            self.aarch64_data_mem_zero(regval, vaddress, accdesc, size);
        }

        // let size = size as usize;
        // let mut zeros = Vec::with_capacity(size);
        // zeros.resize(size, 0);
        // self.mem_write(vaddress, ByteSz(size), &zeros, accdesc);
    }
}
