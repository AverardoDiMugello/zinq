use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate};
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn get_current_exlocken(&mut self) -> bool {
        if self.halted() || self.restarting() {
            return false;
        }

        match self.curr_el() {
            EL::EL0 => panic!("Unreachable"),
            EL::EL1 => self.read::<gcscr_el1::EXLOCKEN>() != 0,
            EL::EL2 => self.read::<gcscr_el2::EXLOCKEN>() != 0,
            EL::EL3 => self.read::<gcscr_el3::EXLOCKEN>() != 0,
        }
    }
}
