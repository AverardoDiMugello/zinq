use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::pstate;
use crate::std::arm::cpu::ArmCtx;

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_is_unpriv_access_priv(&mut self) -> bool {
        let privileged = match self.curr_el() {
            EL::EL0 => false,
            EL::EL1 => self.effective_hcr_el2_nvx() & 0b11 == 0b11,
            EL::EL2 => !self.el_is_in_host(EL::EL0),
            EL::EL3 => true,
        };

        if self.is_feat_impl(Feat::UAO) && self.read::<pstate::UAO>() != 0 {
            self.curr_el() != EL::EL1
        } else {
            privileged
        }
    }

    // TODO: SCTLR_ELx doesn't match Sail!
    // pub fn check_spa_alignment(&mut self) {
    //     let sp = self.sp();
    //     let stack_align_check = *if self.curr_el() == EL::EL0 {
    //         match target_el {
    //             EL::EL0 => panic!("Unreachable"),
    //             EL::EL1 => self.read::<sctlr_el1::SA0>() != 0,
    //             EL::EL2 => self.read::<sctlr_el2::SA0>() != 0,
    //             EL::EL3 => self.read::<sctlr_el3::SA0>() != 0,
    //         }
    //     } else {
    //         match target_el {
    //             EL::EL0 => panic!("Unreachable"),
    //             EL::EL1 => self.read::<sctlr_el1::SA>() != 0,
    //             EL::EL2 => self.read::<sctlr_el2::SA>() != 0,
    //             EL::EL3 => self.read::<sctlr_el3::SA>() != 0,
    //         }
    //     };
    // }

    pub fn mem_read(
        &mut self,
        address: u64,
        size: usize,
        accdesc: AccessDescriptor,
    ) -> Option<[u8; 16]> {
        let memaddrdesc =
            self.aarch64_translate_address(address, accdesc.clone(), true, size as i64);
        if memaddrdesc.is_fault() {
            panic!("translation fault!: {0:?}", memaddrdesc.fault.statuscode);
        } else {
            // println!(
            //     "paddr = 0x{0:x}",
            //     memaddrdesc.paddress.as_ref().unwrap().address
            // );
        }
        let (_memstatus, value) = self.phys_mem_read(&memaddrdesc, size, &accdesc);
        value
    }

    pub fn mem_write(
        &mut self,
        address: u64,
        size: usize,
        value: &[u8],
        accdesc: AccessDescriptor,
    ) {
        let memaddrdesc =
            self.aarch64_translate_address(address, accdesc.clone(), true, size as i64);
        // println!(
        //     "paddr = 0x{0:x}",
        //     memaddrdesc.paddress.as_ref().unwrap().address
        // );
        let _memstatus = self.phys_mem_write(&memaddrdesc, size, &accdesc, value);
    }
}
