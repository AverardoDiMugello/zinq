use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_data_mem_zero(
        &mut self,
        _regval: u64,
        vaddress: u64,
        mut accdesc: AccessDescriptor,
        size: u64,
    ) {
        // TODO: real tag handling
        if self.is_feat_impl(Feat::MTE2) && accdesc.tagaccess {
            accdesc.tagaccess = false;
        }

        if self.is_feat_impl(Feat::MTE2) && accdesc.tagchecked {
            accdesc.tagchecked = false;
        }

        let mut memaddrdesc =
            self.aarch64_translate_address(vaddress, accdesc.clone(), true, size as i64);
        for _ in 0..size {
            self.phys_mem_write(&memaddrdesc, 1, &accdesc, &[0]);
            memaddrdesc.paddress.as_mut().unwrap().address += 1;
        }
    }
}
