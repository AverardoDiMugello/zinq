use crate::core::model::{ir::*, *};
use crate::std::arm::cpu::registers::*;
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
            // TLB not supported yet
        }
        (0b01, 0b000, 0b1000, 0b0011, 0b111) => {
            // TLBI VAALE1IS
            // TLB not supported yet
        }
        (0b01, 0b000, 0b0111, 0b0101, 0b000) => {
            // IC IALLU
            // Cache ops not supported yet
        }
        _ => eprintln!("Sysinsn {0} {1} {2} {3} {4}", op0, op1, crn, crm, op2),
    }
}
