use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn restarting(&mut self) -> bool {
        // self.sysreg.edscr.status() == 1
        // TODO: external registers
        false
    }

    pub fn halted(&mut self) -> bool {
        // !(self.sysreg.edscr.status() == 1 || self.sysreg.edscr.status() == 2)
        // TODO: external registers
        false
    }
}
