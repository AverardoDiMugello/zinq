use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::registers::pstate;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn in_streaming_mode(&mut self) -> bool {
        self.is_feat_impl(Feat::SME) && self.read::<pstate::SM>() != 0
    }
}
