use crate::core::model::{ir::*, *};
use crate::std::arm::cpu::config::Feat;
use crate::std::arm::cpu::registers::{aarch64::*, *};
use crate::std::arm::cpu::{pseudoc::*, ArmCtx};
use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
use bitvec::prelude::*;

pub fn emulate_sys<'a>(cpu: &ArmCpu, ctx: &mut ProcCtx<'a, ArmCpu>, args: Vec<CallArg>) {
    let mut ctx = cpu.ctx(ctx);

    let op0 = args
        .get(0)
        .and_then(|arg| {
            if let CallArg::Bv8(val) = arg {
                Some(val.clone())
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding");
    let op1 = args
        .get(1)
        .and_then(|arg| {
            if let CallArg::Bv8(val) = arg {
                Some(val.clone())
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding");
    let crn = args
        .get(2)
        .and_then(|arg| {
            if let CallArg::Bv8(val) = arg {
                Some(val.clone())
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding");
    let crm = args
        .get(3)
        .and_then(|arg| {
            if let CallArg::Bv8(val) = arg {
                Some(val.clone())
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding");
    let op2 = args
        .get(4)
        .and_then(|arg| {
            if let CallArg::Bv8(val) = arg {
                Some(val.clone())
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding");

    let data = args
        .get(5)
        .and_then(|arg| {
            if let CallArg::Bv64(val) = arg {
                Some(val.clone())
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding");

    match (op0, op1, crn, crm, op2) {
        (0b01, 0b000, 0b0111, 0b0110, 0b001) => {
            // DC IVAC
            // Cache ops not supported yet
        }
        (0b01, 0b011, 0b0111, 0b0100, 0b001) => {
            // DC ZVA, Xt
            ctx.aarch64_mem_zero(data, CacheType::Data);
        }
        (0b01, 0b011, 0b0111, 0b1110, 0b001) => {
            // DC CIVAC
            // Cache ops not supported yet
        }
        (0b01, 0b000, 0b1000, 0b0111, 0b000) => {
            // TLBI VMALLE1
            let curr_el = ctx.curr_el();

            let ss = ctx.security_state_at_el(EL::EL1);
            let vmid = ctx.vmid();
            match curr_el {
                EL::EL0 => panic!("Undefined"),
                EL::EL1 => {
                    if ctx.is_feat_impl(Feat::XS)
                        && ctx.is_feat_impl(Feat::HCX)
                        && ctx.is_hcrx_el2_enabled()
                        && ctx.read::<hcrx_el2::FnXS>() != 0
                    {
                        ctx.aarch64_tlbi_vmall(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::ISH,
                            TLBIMemAttr::ExcludeXS,
                        );
                    } else {
                        ctx.aarch64_tlbi_vmall(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::ISH,
                            TLBIMemAttr::AllAttr,
                        );
                    }
                }
                EL::EL2 => {
                    if ctx.el_is_in_host(EL::EL0) {
                        let ss = ctx.security_state_at_el(EL::EL2);
                        ctx.aarch64_tlbi_vmall(
                            ss,
                            Regime::EL20,
                            0,
                            Shareability::NSH,
                            TLBIMemAttr::AllAttr,
                        );
                    } else {
                        let ss = ctx.security_state_at_el(EL::EL1);
                        ctx.aarch64_tlbi_vmall(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::NSH,
                            TLBIMemAttr::AllAttr,
                        );
                    }
                }
                EL::EL3 => panic!("Unsupported"),
            }
        }
        (0b01, 0b000, 0b1000, 0b0011, 0b001) => {
            // TLBI VAE1IS
            let curr_el = ctx.curr_el();
            let vmid = ctx.vmid();
            let xt = data;
            match curr_el {
                EL::EL0 => panic!("Undefined"),
                EL::EL1 => {
                    let ss = ctx.security_state_at_el(EL::EL1);
                    if ctx.is_feat_impl(Feat::XS)
                        && ctx.is_feat_impl(Feat::HCX)
                        && ctx.is_hcrx_el2_enabled()
                        && ctx.read::<hcrx_el2::FnXS>() != 0
                    {
                        ctx.aarch64_tlbi_va(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::ISH,
                            TLBILevel::Last,
                            TLBIMemAttr::ExcludeXS,
                            xt,
                        );
                    } else {
                        ctx.aarch64_tlbi_va(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::ISH,
                            TLBILevel::Last,
                            TLBIMemAttr::AllAttr,
                            xt,
                        );
                    }
                }
                EL::EL2 => {
                    if ctx.el_is_in_host(EL::EL0) {
                        let ss = ctx.security_state_at_el(EL::EL2);
                        ctx.aarch64_tlbi_va(
                            ss,
                            Regime::EL20,
                            0,
                            Shareability::ISH,
                            TLBILevel::Last,
                            TLBIMemAttr::AllAttr,
                            xt,
                        );
                    } else {
                        let ss = ctx.security_state_at_el(EL::EL1);
                        ctx.aarch64_tlbi_va(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::ISH,
                            TLBILevel::Last,
                            TLBIMemAttr::AllAttr,
                            xt,
                        );
                    }
                }
                EL::EL3 => panic!("Unsupported"),
            }
        }
        (0b01, 0b000, 0b1000, 0b0011, 0b111) => {
            // TLBI VAALE1IS
            let curr_el = ctx.curr_el();
            let vmid = ctx.vmid();
            let xt = data;
            match curr_el {
                EL::EL0 => panic!("Undefined"),
                EL::EL1 => {
                    let ss = ctx.security_state_at_el(EL::EL1);
                    if ctx.is_feat_impl(Feat::XS)
                        && ctx.is_feat_impl(Feat::HCX)
                        && ctx.is_hcrx_el2_enabled()
                        && ctx.read::<hcrx_el2::FnXS>() != 0
                    {
                        ctx.aarch64_tlbi_vaa(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::ISH,
                            TLBILevel::Last,
                            TLBIMemAttr::ExcludeXS,
                            xt,
                        );
                    } else {
                        ctx.aarch64_tlbi_vaa(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::ISH,
                            TLBILevel::Last,
                            TLBIMemAttr::AllAttr,
                            xt,
                        );
                    }
                }
                EL::EL2 => {
                    if ctx.el_is_in_host(EL::EL0) {
                        let ss = ctx.security_state_at_el(EL::EL2);
                        ctx.aarch64_tlbi_vaa(
                            ss,
                            Regime::EL20,
                            0,
                            Shareability::ISH,
                            TLBILevel::Last,
                            TLBIMemAttr::AllAttr,
                            xt,
                        );
                    } else {
                        let ss = ctx.security_state_at_el(EL::EL1);
                        ctx.aarch64_tlbi_vaa(
                            ss,
                            Regime::EL10,
                            vmid,
                            Shareability::ISH,
                            TLBILevel::Last,
                            TLBIMemAttr::AllAttr,
                            xt,
                        );
                    }
                }
                EL::EL3 => panic!("Unsupported"),
            }
        }
        (0b01, 0b000, 0b1000, 0b0010, 0b001) => {
            // TODO: TLBI RVAE1IS (apparently doesn't matter for Sail Linux boot)
        }
        (0b01, 0b000, 0b0111, 0b0101, 0b000) => {
            // IC IALLU
            // Cache ops not supported yet
        }
        (0b01, 0b000, 0b0111, 0b0001, 0b000) => {
            // IC IALLUIS
            // Cache ops not supported yet
        }
        _ => {
            eprintln!("Sysinsn {0} {1} {2} {3} {4}", op0, op1, crn, crm, op2)
            // panic!("Sysinsn {0} {1} {2} {3} {4}", op0, op1, crn, crm, op2);
        }
    }
}
