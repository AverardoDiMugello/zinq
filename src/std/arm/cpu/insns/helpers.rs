use crate::core::model::{ir::*, proc::*};
use crate::std::arm::cpu::pseudoc::*;
use crate::std::arm::cpu::registers::{aarch64::*, *};
use crate::std::arm::cpu::ArmCtx;
use crate::std::arm::cpu::{ArmCpu, ArmIRCtx};
use bitvec::prelude::*;

mod mrs;
pub use mrs::emulate_mrs;
mod msr;
pub use msr::emulate_msr_reg;
mod sys;
pub use sys::emulate_sys;

pub fn x8<'a>(n: u32, ctx: &ArmIRCtx, block: &mut IrBlock<ArmCpu>) -> Bv8 {
    if n != 31 {
        block.read_reg8(ctx.arr8::<X8>(n))
    } else {
        Bv8::zero()
    }
}

pub fn x16<'a>(n: u32, ctx: &ArmIRCtx, block: &mut IrBlock<ArmCpu>) -> Bv16 {
    if n != 31 {
        block.read_reg16(ctx.arr16::<X16>(n))
    } else {
        Bv16::zero()
    }
}

pub fn x32<'a>(n: u32, ctx: &ArmIRCtx, block: &mut IrBlock<ArmCpu>) -> Bv32 {
    if n != 31 {
        block.read_reg32(ctx.arr32::<W>(n))
    } else {
        Bv32::zero()
    }
}

pub fn x64<'a>(n: u32, ctx: &ArmIRCtx, block: &mut IrBlock<ArmCpu>) -> Bv64 {
    if n != 31 {
        block.read_reg64(ctx.arr64::<X>(n))
    } else {
        Bv64::zero()
    }
}

pub fn wsp<'a>(el: EL, pstate_sp: bool, ctx: &ArmIRCtx, block: &mut IrBlock<ArmCpu>) -> Bv32 {
    if !pstate_sp {
        block.read_reg32(ctx.wsp(EL::EL0))
    } else {
        block.read_reg32(ctx.wsp(el))
    }
}

pub fn sp<'a>(el: EL, pstate_sp: bool, ctx: &ArmIRCtx, block: &mut IrBlock<ArmCpu>) -> Bv64 {
    if !pstate_sp {
        block.read_reg64(ctx.sp(EL::EL0))
    } else {
        block.read_reg64(ctx.sp(el))
    }
}

pub fn inc_pc<'a>(ctx: &ArmIRCtx, block: &mut IrBlock<ArmCpu>) {
    let pc = block.read_reg64(ctx.reg64::<PC>());
    let (next_pc, _) = block.uadd_i64(pc, Bv64::Lit(4));
    block.write_reg64(ctx.reg64::<PC>(), next_pc);
}

pub enum PSTATEField {
    DAIFSet,
    DAIFClr,
    PAN, // Armv8.1
    UAO, // Armv8.2
    DIT, // Armv8.4
    SSBS,
    TCO, // Armv8.5
    SVCRSM,
    SVCRZA,
    SVCRSMZA,
    ALLINT,
    PM,
    SP,
}

pub enum ExtendType {
    SXTB,
    SXTH,
    SXTW,
    SXTX,
    UXTB,
    UXTH,
    UXTW,
    UXTX,
}

impl ExtendType {
    pub fn decode<'a>(hi: bool, mi: bool, lo: bool) -> Self {
        match (hi, mi, lo) {
            (false, false, false) => Self::UXTB,
            (false, false, true) => Self::UXTH,
            (false, true, false) => Self::UXTW,
            (false, true, true) => Self::UXTX,
            (true, false, false) => Self::SXTB,
            (true, false, true) => Self::SXTH,
            (true, true, false) => Self::SXTW,
            (true, true, true) => Self::SXTX,
        }
    }
}

// Adds up to 6 IROps
pub fn extend_reg_i32<'a>(
    reg: u32,
    exttype: ExtendType,
    shift: u32,
    ctx: &ArmIRCtx,
    block: &mut IrBlock<ArmCpu>,
) -> Bv32 {
    let val = x32(reg, ctx, block);
    let (unsigned, len) = match exttype {
        ExtendType::SXTB => (false, 8),
        ExtendType::SXTH => (false, 16),
        ExtendType::SXTW => (false, 32),
        ExtendType::SXTX => (false, 64),
        ExtendType::UXTB => (true, 8),
        ExtendType::UXTH => (true, 16),
        ExtendType::UXTW => (true, 32),
        ExtendType::UXTX => (true, 64),
    };

    let nbits = std::cmp::min(len, 32 - shift);
    let to_extend = block.and_i32(
        val,
        Bv32::Lit(
            1u32.checked_shl(nbits)
                .and_then(|v| Some(v - 1))
                .unwrap_or(u32::MAX),
        ),
    );
    let to_extend = block.lsl_i32(to_extend, Bv32::Lit(shift));
    if unsigned {
        to_extend
    } else {
        let sign_bit = block.nth_bit_i32(to_extend, Bv32::Lit(nbits + shift - 1));
        let sext_mask = block.repl_b_in_i32(sign_bit, Bv32::Lit(32 - (nbits + shift)));
        let sext_mask = block.lsl_i32(sext_mask, Bv32::Lit(nbits + shift));
        block.or_i32(to_extend, sext_mask)
    }
}

// Adds up to 6 IROps
pub fn extend_reg_i64<'a>(
    reg: u32,
    exttype: ExtendType,
    shift: u64,
    ctx: &ArmIRCtx,
    block: &mut IrBlock<ArmCpu>,
) -> Bv64 {
    let val = x64(reg, ctx, block);
    let (unsigned, len) = match exttype {
        ExtendType::SXTB => (false, 8),
        ExtendType::SXTH => (false, 16),
        ExtendType::SXTW => (false, 32),
        ExtendType::SXTX => (false, 64),
        ExtendType::UXTB => (true, 8),
        ExtendType::UXTH => (true, 16),
        ExtendType::UXTW => (true, 32),
        ExtendType::UXTX => (true, 64),
    };

    let nbits = std::cmp::min(len, 64 - shift);
    let to_extend = block.and_i64(
        val,
        Bv64::Lit(
            1u64.checked_shl(nbits as u32)
                .and_then(|v| Some(v - 1))
                .unwrap_or(u64::MAX),
        ),
    );
    let to_extend = block.lsl_i64(to_extend, Bv64::Lit(shift));
    if unsigned {
        to_extend
    } else {
        let sign_bit = block.nth_bit_i64(to_extend, Bv64::Lit(nbits + shift - 1));
        let sext_mask = block.repl_b_in_i64(sign_bit, Bv64::Lit(64 - (nbits + shift)));
        let sext_mask = block.lsl_i64(sext_mask, Bv64::Lit(nbits + shift));
        block.or_i64(to_extend, sext_mask)
    }
}

// Note: adds max 7 IROps
pub fn condition_holds<'a>(
    cond: &BitSlice<u32>,
    ctx: &ArmIRCtx,
    block: &mut IrBlock<ArmCpu>,
) -> Bit {
    let result = match (cond[3], cond[2], cond[1]) {
        // EQ or NE
        (false, false, false) => block.read_reg1(ctx.reg1::<pstate::Z>()),
        // CS or CC
        (false, false, true) => block.read_reg1(ctx.reg1::<pstate::C>()),
        // MI or PL
        (false, true, false) => block.read_reg1(ctx.reg1::<pstate::N>()),
        // VS or VC
        (false, true, true) => block.read_reg1(ctx.reg1::<pstate::V>()),
        // HI or LS
        (true, false, false) => {
            let c = block.read_reg1(ctx.reg1::<pstate::C>());
            let lhs = block.eq_b(c, Bit::one());
            let z = block.read_reg1(ctx.reg1::<pstate::Z>());
            let rhs = block.eq_b(z, Bit::zero());
            block.and_b(lhs, rhs)
        }
        // GE or LT
        (true, false, true) => {
            let n = block.read_reg1(ctx.reg1::<pstate::N>());
            let v = block.read_reg1(ctx.reg1::<pstate::V>());
            block.eq_b(n, v)
        }
        // GT or LE
        (true, true, false) => {
            let n = block.read_reg1(ctx.reg1::<pstate::N>());
            let v = block.read_reg1(ctx.reg1::<pstate::V>());
            let lhs = block.eq_b(n, v);
            let z = block.read_reg1(ctx.reg1::<pstate::Z>());
            let rhs = block.eq_b(z, Bit::zero());
            block.and_b(lhs, rhs)
        }
        // AL
        (true, true, true) => Bit::one(),
    };

    if cond[0] && (cond[3], cond[2], cond[1], cond[0]) != (true, true, true, true) {
        block.not_b(result)
    } else {
        result
    }
}

// Note: adds 2 IROps
pub fn address_add<'a>(
    base: Bv64,
    offset: Bv64,
    _cpa2: bool,
    block: IrBlockIdx,
    graph: &mut IRGraph<ArmCpu>,
) -> Addr {
    let block = graph.block_mut(block);
    let (result, _) = block.uadd_i64(base, offset);
    result
    // TODO: PointerAddCheckAtEL
    // block.i64_as_addr(result)
}

pub fn emulate_hvc<'a>(cpu: &ArmCpu, ctx: &mut ProcCtx<'a, ArmCpu>, args: Vec<CallArg>) {
    let imm = args
        .first()
        .and_then(|arg| {
            if let CallArg::Bv16(immediate) = arg {
                Some(immediate)
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding")
        .clone();

    let mut ctx = ArmCtx { cpu, ctx };

    if ctx.curr_el() == EL::EL0
        || (ctx.curr_el() == EL::EL1 && !ctx.el2_enabled())
        || (!ctx.have_el(EL::EL3) && ctx.read::<hcr_el2::HCD>() != 0)
        || (ctx.have_el(EL::EL3) && ctx.read::<scr_el3::HCE>() == 0)
    {
        panic!("Undefined HVC use! Should this UNDEFINED instruction be handled differently?");
    }
    ctx.aarch64_call_hypervisor(imm);
}

pub fn emulate_smc<'a>(cpu: &ArmCpu, ctx: &mut ProcCtx<'a, ArmCpu>, args: Vec<CallArg>) {
    let imm = args
        .first()
        .and_then(|arg| {
            if let CallArg::Bv16(immediate) = arg {
                Some(immediate)
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding")
        .clone();

    let mut ctx = ArmCtx { cpu, ctx };

    ctx.aarch64_check_for_smc_undef_or_trap(imm);
    ctx.aarch64_call_secure_monitor(imm);
}

pub fn emulate_svc<'a>(cpu: &ArmCpu, ctx: &mut ProcCtx<'a, ArmCpu>, args: Vec<CallArg>) {
    let imm = args
        .first()
        .and_then(|arg| {
            if let CallArg::Bv16(immediate) = arg {
                Some(immediate)
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding")
        .clone();

    let mut ctx = ArmCtx { cpu, ctx };

    ctx.aarch64_check_for_svc_trap(imm);
    ctx.aarch64_call_supervisor(imm);
}

pub fn emulate_msr_imm_daifset<'a>(
    cpu: &ArmCpu,
    ctx: &mut ProcCtx<'a, ArmCpu>,
    args: Vec<CallArg>,
) {
    let this_instr = args
        .first()
        .and_then(|arg| {
            if let CallArg::Bv32(this_instr) = arg {
                Some(this_instr)
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding")
        .clone();

    let mut ctx = ArmCtx { cpu, ctx };

    ctx.aarch64_check_daif_access(this_instr);
    // operand = this_instr[8:12]
    let new_d = ctx.read::<pstate::D>() != 0 || (this_instr >> 11) & 1 != 0;
    ctx.write::<pstate::D>(if new_d { 1 } else { 0 });
    let new_a = ctx.read::<pstate::A>() != 0 || (this_instr >> 10) & 1 != 0;
    ctx.write::<pstate::A>(if new_a { 1 } else { 0 });
    let new_i = ctx.read::<pstate::I>() != 0 || (this_instr >> 9) & 1 != 0;
    ctx.write::<pstate::I>(if new_i { 1 } else { 0 });
    let new_f = ctx.read::<pstate::F>() != 0 || (this_instr >> 8) & 1 != 0;
    ctx.write::<pstate::F>(if new_f { 1 } else { 0 });
}

pub fn emulate_msr_imm_daifclr<'a>(
    cpu: &ArmCpu,
    ctx: &mut ProcCtx<'a, ArmCpu>,
    args: Vec<CallArg>,
) {
    let this_instr = args
        .first()
        .and_then(|arg| {
            if let CallArg::Bv32(this_instr) = arg {
                Some(this_instr)
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding")
        .clone();

    let mut ctx = ArmCtx { cpu, ctx };

    ctx.aarch64_check_daif_access(this_instr);
    // operand = this_instr[8:12]
    let new_d = ctx.read::<pstate::D>() != 0 && (this_instr >> 11) & 1 == 0;
    ctx.write::<pstate::D>(if new_d { 1 } else { 0 });
    let new_a = ctx.read::<pstate::A>() != 0 && (this_instr >> 10) & 1 == 0;
    ctx.write::<pstate::A>(if new_a { 1 } else { 0 });
    let new_i = ctx.read::<pstate::I>() != 0 && (this_instr >> 9) & 1 == 0;
    ctx.write::<pstate::I>(if new_i { 1 } else { 0 });
    let new_f = ctx.read::<pstate::F>() != 0 && (this_instr >> 8) & 1 == 0;
    ctx.write::<pstate::F>(if new_f { 1 } else { 0 });
}

pub fn emulate_msr_imm_allint<'a>(cpu: &ArmCpu, ctx: &mut ProcCtx<'a, ArmCpu>, args: Vec<CallArg>) {
    let this_instr = args
        .first()
        .and_then(|arg| {
            if let CallArg::Bv32(this_instr) = arg {
                Some(this_instr)
            } else {
                None
            }
        })
        .expect("Only called from valid instruction decoding")
        .clone();

    let mut ctx = ArmCtx { cpu, ctx };

    let op0 = (this_instr >> 8) != 0;
    if ctx.curr_el() == EL::EL1
        && ctx.is_hcrx_el2_enabled()
        && ctx.read::<hcrx_el2::TALLINT>() != 0
        && op0
    {
        ctx.aarch64_system_access_trap(EL::EL2, 0x18, this_instr);
    }
    ctx.write::<pstate::A>(if op0 { 1 } else { 0 });
}
