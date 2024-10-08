use crate::core::model::proc::{ProcDecl, Register};
use crate::std::arm::cpu::ArmCpu;

/// Negative condition flag
pub struct N;
impl Register for N {
    const NAME: &'static str = "N";
    const LEN: usize = 1;
}

/// Zero condition flag
pub struct Z;
impl Register for Z {
    const NAME: &'static str = "Z";
    const LEN: usize = 1;
}

/// Carry condition flag
pub struct C;
impl Register for C {
    const NAME: &'static str = "C";
    const LEN: usize = 1;
}

/// Overflow condition flag
pub struct V;
impl Register for V {
    const NAME: &'static str = "V";
    const LEN: usize = 1;
}

/// Debug mask bit \[AArch64 only\]
pub struct D;
impl Register for D {
    const NAME: &'static str = "D";
    const LEN: usize = 1;
}

/// SError interrupt mask bit
pub struct A;
impl Register for A {
    const NAME: &'static str = "A";
    const LEN: usize = 1;
}

/// IRQ mask bit
pub struct I;
impl Register for I {
    const NAME: &'static str = "I";
    const LEN: usize = 1;
}

/// FIQ mask bit
pub struct F;
impl Register for F {
    const NAME: &'static str = "F";
    const LEN: usize = 1;
}

/// Lock exception return state
pub struct EXLOCK;
impl Register for EXLOCK {
    const NAME: &'static str = "EXLOCK";
    const LEN: usize = 1;
}

/// Privileged Access Never Bit \[v8.1\]
pub struct PAN;
impl Register for PAN {
    const NAME: &'static str = "PAN";
    const LEN: usize = 1;
}

/// User Access Override \[v8.2\]
pub struct UAO;
impl Register for UAO {
    const NAME: &'static str = "UAO";
    const LEN: usize = 1;
}

/// Data Independent Timing \[v8.4\]
pub struct DIT;
impl Register for DIT {
    const NAME: &'static str = "DIT";
    const LEN: usize = 1;
}

/// Tag Check Override \[v8.5, AArch64 only\]
pub struct TCO;
impl Register for TCO {
    const NAME: &'static str = "TCO";
    const LEN: usize = 1;
}

/// PMU exception Mask
pub struct PM;
impl Register for PM {
    const NAME: &'static str = "PM";
    const LEN: usize = 1;
}

/// synchronous PMU exception to be observed
pub struct PPEND;
impl Register for PPEND {
    const NAME: &'static str = "PPEND";
    const LEN: usize = 1;
}

/// Branch Type \[v8.5\]
pub struct BTYPE;
impl Register for BTYPE {
    const NAME: &'static str = "BTYPE";
    const LEN: usize = 2;
}

/// PAC instruction modifier
pub struct PACM;
impl Register for PACM {
    const NAME: &'static str = "PACM";
    const LEN: usize = 1;
}

/// Accumulation array enabled \[SME\]
pub struct ZA;
impl Register for ZA {
    const NAME: &'static str = "ZA";
    const LEN: usize = 1;
}

/// Streaming SVE mode enabled \[SME\]
pub struct SM;
impl Register for SM {
    const NAME: &'static str = "SM";
    const LEN: usize = 1;
}

/// Interrupt mask bit
pub struct ALLINT;
impl Register for ALLINT {
    const NAME: &'static str = "ALLINT";
    const LEN: usize = 1;
}

/// Software step bit
pub struct SS;
impl Register for SS {
    const NAME: &'static str = "SS";
    const LEN: usize = 1;
}

/// Illegal Execution state bit
pub struct IL;
impl Register for IL {
    const NAME: &'static str = "IL";
    const LEN: usize = 1;
}

/// Exception level
pub struct EL;
impl Register for EL {
    const NAME: &'static str = "EL";
    const LEN: usize = 2;
}

/// Execution state: 0=AArch64, 1=AArch32
#[allow(non_camel_case_types)]
pub struct nRW;
impl Register for nRW {
    const NAME: &'static str = "nRW";
    const LEN: usize = 1;
}

/// Stack pointer select: 0=SP0, 1=SPx \[AArch64 only\]
pub struct SP;
impl Register for SP {
    const NAME: &'static str = "SP";
    const LEN: usize = 1;
}

/// Cumulative saturation flag \[AArch32 only\]
pub struct Q;
impl Register for Q {
    const NAME: &'static str = "Q";
    const LEN: usize = 1;
}

/// Greater than or Equal flags \[AArch32 only\]
pub struct GE;
impl Register for GE {
    const NAME: &'static str = "GE";
    const LEN: usize = 4;
}

/// Speculative Store Bypass Safe
pub struct SSBS;
impl Register for SSBS {
    const NAME: &'static str = "SSBS";
    const LEN: usize = 1;
}

/// If-then bits, RES0 in CPSR \[AArch32 only\]
pub struct IT;
impl Register for IT {
    const NAME: &'static str = "IT";
    const LEN: usize = 8;
}

/// J bit, RES0 \[AArch32 only, RES0 in SPSR and CPSR\]
pub struct J;
impl Register for J {
    const NAME: &'static str = "J";
    const LEN: usize = 1;
}

/// T32 bit, RES0 in CPSR \[AArch32 only\]
pub struct T;
impl Register for T {
    const NAME: &'static str = "T";
    const LEN: usize = 1;
}

/// Endianness bit \[AArch32 only\]
pub struct E;
impl Register for E {
    const NAME: &'static str = "E";
    const LEN: usize = 1;
}

/// Mode field \[AArch32 only\]
pub struct M;
impl Register for M {
    const NAME: &'static str = "M";
    const LEN: usize = 5;
}

impl ArmCpu {
    pub fn decl_pstate(&self, decl: &mut ProcDecl<ArmCpu>) {
        decl.reg::<N>()
            .reg::<Z>()
            .reg::<C>()
            .reg::<V>()
            .reg::<D>()
            .reg::<A>()
            .reg::<I>()
            .reg::<F>()
            .reg::<EXLOCK>()
            .reg::<PAN>()
            .reg::<UAO>()
            .reg::<DIT>()
            .reg::<TCO>()
            .reg::<PM>()
            .reg::<PPEND>()
            .reg::<BTYPE>()
            .reg::<PACM>()
            .reg::<ZA>()
            .reg::<SM>()
            .reg::<ALLINT>()
            .reg::<SS>()
            .reg::<IL>()
            .reg::<EL>()
            .reg::<nRW>()
            .reg::<SP>()
            .reg::<Q>()
            .reg::<GE>()
            .reg::<SSBS>()
            .reg::<IT>()
            .reg::<J>()
            .reg::<T>()
            .reg::<E>()
            .reg::<M>();
    }
}
