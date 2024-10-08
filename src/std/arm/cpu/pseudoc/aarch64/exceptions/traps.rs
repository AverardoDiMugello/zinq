use bitvec::prelude::*;

use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, pstate, PC};
use crate::std::arm::cpu::{ArmCtx, Feat};

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn aarch64_check_for_eret_trap(&mut self, eret_with_pac: bool, pac_uses_key_a: bool) {
        use EL::*;
        let route_to_el2;
        // Non-secure EL1 execution of ERET, ERETAA, ERETAB when either HCR_EL2.NV or
        // HFGITR_EL2.ERET is set, is trapped to EL2
        route_to_el2 = self.curr_el() == EL1
            && self.el2_enabled()
            && (self.effective_hcr_el2_nvx() & 1 != 0
                || (self.is_feat_impl(Feat::FGT)
                    && (!self.have_el(EL3) || self.read::<scr_el3::FGTEn>() != 0)
                    && self.read::<hfgitr_el2::ERET>() != 0));
        if route_to_el2 {
            let preferred_exception_return = self.read::<PC>(); // ThisInstrAddr(64);
            let vect_offset = 0x0;
            let mut except = ExceptionRecord::exception_syndrome(Exception::ERetTrap);
            if !eret_with_pac {
                // ERET
                except.syndrome &= !0b10;
                except.syndrome &= !0b1; // RES0
            } else {
                except.syndrome |= 1 << 1;
                if pac_uses_key_a {
                    // ERETAA
                    except.syndrome &= !0b1;
                } else {
                    // ERETAB
                    except.syndrome |= 1;
                }
            }
            self.aarch64_take_exception(EL2, except, preferred_exception_return, vect_offset);
        }
    }

    pub fn aarch64_check_for_smc_undef_or_trap(&mut self, imm: u16) {
        use EL::*;
        if self.curr_el() == EL0
            || (!(self.curr_el() == EL1 && self.el2_enabled() && self.read::<hcr_el2::TSC>() != 0)
                && self.have_el(EL3)
                && self.read::<scr_el3::SMD>() != 0)
        {
            panic!("Undefined HVC use! Should this UNDEFINED instruction be handled differently?");
        }
        let route_to_el2;
        if !self.have_el(EL3) {
            if self.curr_el() == EL1
                && self.el2_enabled()
                && self.read::<hcr_el2::TSC>() != 0
                && (self.effective_hcr_el2_nvx() & 1 != 0
                    || (self.cpu.impdef.bool(&"Trap SMC execution at EL1 to EL2")))
            {
                route_to_el2 = true;
            } else {
                panic!(
                    "Undefined HVC use! Should this UNDEFINED instruction be handled differently?"
                );
            }
        } else {
            route_to_el2 =
                self.curr_el() == EL1 && self.el2_enabled() && self.read::<hcr_el2::TSC>() != 0;
        }

        if route_to_el2 {
            let preferred_exception_return = self.read::<PC>(); // ThisInstrAddr(64);
            let vect_offset = 0x0;
            let mut except = ExceptionRecord::exception_syndrome(Exception::MonitorCall);
            except.syndrome |= imm as u64;
            except.trappedsyscallinst = true;
            self.aarch64_take_exception(EL2, except, preferred_exception_return, vect_offset);
        }
    }

    pub fn aarch64_check_for_svc_trap(&mut self, immediate: u16) {
        if self.is_feat_impl(Feat::FGT) {
            let route_to_el2;
            if self.curr_el() == EL::EL0 {
                route_to_el2 = !self.using_aarch32()
                    && !self.el_using_aarch32(EL::EL1)
                    && self.el2_enabled()
                    && self.read::<hfgitr_el2::SVC_EL0>() != 0
                    && (!self.is_in_host()
                        && (!self.have_el(EL::EL3) || self.read::<scr_el3::FGTEn>() != 0));
            } else if self.curr_el() == EL::EL1 {
                route_to_el2 = self.el2_enabled()
                    && self.read::<hfgitr_el2::SVC_EL1>() != 0
                    && (!self.have_el(EL::EL3) || self.read::<scr_el3::FGTEn>() != 0);
            } else {
                route_to_el2 = false;
            }

            if route_to_el2 {
                let mut except = ExceptionRecord::exception_syndrome(Exception::SupervisorCall);
                except.syndrome |= immediate as u64;
                except.trappedsyscallinst = true;
                let pref_excp_ret = self.read::<PC>(); // ThisInstrAddr(64);
                let vect_offset = 0;
                self.aarch64_take_exception(EL::EL2, except, pref_excp_ret, vect_offset);
            }
        }
    }

    pub fn aarch64_system_access_trap(&mut self, target_el: EL, ec: u8, this_instr: u32) {
        assert!(self.have_el(target_el));
        assert!(target_el != EL::EL0);
        assert!(target_el.as_num() >= self.curr_el().as_num());

        let pref_excp_ret = self.read::<PC>(); // ThisInstrAddr(64);
        let vect_offset = 0x0;

        let except = self.aarch64_system_access_trap_syndrome(this_instr, ec);
        self.aarch64_take_exception(target_el, except, pref_excp_ret, vect_offset);
    }

    pub fn aarch64_system_access_trap_syndrome(
        &mut self,
        instr_in: u32,
        ec: u8,
    ) -> ExceptionRecord {
        let mut except: ExceptionRecord;
        let instr = instr_in;
        match ec {
            0x0 => {
                // Trapped access due to unknown reason.
                except = ExceptionRecord::exception_syndrome(Exception::Uncategorized);
            }
            0x7 => {
                // Trapped access to SVE, Advance SIMD&FP System register.
                except = ExceptionRecord::exception_syndrome(Exception::AdvSIMDFPAccessTrap);
                except.syndrome |= self.condition_syndrome() << 20;
            }
            0x14 => {
                // Trapped access to 128-bit System register or
                // 128-bit System instruction.
                except = ExceptionRecord::exception_syndrome(Exception::SystemRegister128Trap);
                let instr = instr as u64;
                except.syndrome.view_bits_mut::<Lsb0>()[20..21 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[19..20 + 1]); // Op0
                except.syndrome.view_bits_mut::<Lsb0>()[17..19 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[5..7 + 1]); // Op2
                except.syndrome.view_bits_mut::<Lsb0>()[14..16 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[16..18 + 1]); // Op1
                except.syndrome.view_bits_mut::<Lsb0>()[10..13 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[12..15 + 1]); // CRn
                except.syndrome.view_bits_mut::<Lsb0>()[6..9 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[1..4 + 1]); // Rt
                except.syndrome.view_bits_mut::<Lsb0>()[1..4 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[8..11 + 1]); // CRm
            }
            0x18 => {
                // Trapped access to System register or system instruction.
                except = ExceptionRecord::exception_syndrome(Exception::SystemRegisterTrap);
                let instr = instr as u64;
                except.syndrome.view_bits_mut::<Lsb0>()[20..21 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[19..20 + 1]); // Op0
                except.syndrome.view_bits_mut::<Lsb0>()[17..19 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[5..7 + 1]); // Op2
                except.syndrome.view_bits_mut::<Lsb0>()[14..16 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[16..18 + 1]); // Op1
                except.syndrome.view_bits_mut::<Lsb0>()[10..13 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[12..15 + 1]); // CRn
                except.syndrome.view_bits_mut::<Lsb0>()[5..9 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[0..4 + 1]); // Rt
                except.syndrome.view_bits_mut::<Lsb0>()[1..4 + 1]
                    .copy_from_bitslice(&instr.view_bits::<Lsb0>()[8..11 + 1]); // CRm
                except
                    .syndrome
                    .view_bits_mut::<Lsb0>()
                    .set(0, instr.view_bits::<Lsb0>()[21]); // Direction
            }
            0x19 => {
                // Trapped access to SVE System register
                except = ExceptionRecord::exception_syndrome(Exception::SVEAccessTrap);
            }
            0x1D => {
                // Trapped access to SME System register
                except = ExceptionRecord::exception_syndrome(Exception::SMEAccessTrap);
            }
            _ => panic!("Unreachable"),
        }
        return except;
    }
}
