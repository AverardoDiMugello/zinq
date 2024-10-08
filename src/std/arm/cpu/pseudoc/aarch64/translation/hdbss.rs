use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn append_to_hdbss(
        &mut self,
        _fault: &mut FaultRecord,
        _ipa: &FullAddress,
        _accdesc: &AccessDescriptor,
        _walkparams: &S2TTWParams,
        _level: i64,
    ) {
        todo!("AppendToHDBSS");
    }
}
