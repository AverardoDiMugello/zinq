use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn reset_sve_state(&mut self) {
        todo!("reset_sve_state");
    }
}
