use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn granule_protection_check(
        &mut self,
        _addrdesc: &AddressDescriptor,
        _accdesc: &AccessDescriptor,
    ) -> GPCFRecord {
        // TODO
        return GPCFRecord::no_fault();
    }
}
