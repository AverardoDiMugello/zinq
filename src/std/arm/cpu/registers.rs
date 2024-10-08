use crate::core::model::proc::*;

pub mod aarch32;
pub mod aarch64;
pub mod pstate;

/// Program counter
pub struct PC;
impl Register for PC {
    const NAME: &'static str = "PC";
    const LEN: usize = 64;
}

/// General-purpose registers
pub struct R;
impl RegisterArray for R {
    const NAME: &'static str = "R";
    const ELEMS: usize = 31;
    const ELEM_LEN: usize = 64;
}

/// 64-bit views of the R registers
pub struct X;
impl RegisterArrayView for X {
    type RegisterArray = R;
    const NAME: &'static str = "X";
    const OFFSET: usize = 0;
    const LEN: usize = 64;
}

/// 32-bit views of the R registers
pub struct W;
impl RegisterArrayView for W {
    type RegisterArray = R;
    const NAME: &'static str = "W";
    const OFFSET: usize = 0;
    const LEN: usize = 32;
}

/// 16-bit views of the R registers
pub struct X16;
impl RegisterArrayView for X16 {
    type RegisterArray = R;
    const NAME: &'static str = "X16";
    const OFFSET: usize = 0;
    const LEN: usize = 16;
}

/// 8-bit views of the R registers
pub struct X8;
impl RegisterArrayView for X8 {
    type RegisterArray = R;
    const NAME: &'static str = "X8";
    const OFFSET: usize = 0;
    const LEN: usize = 8;
}

/// SIMD&FP registers
pub struct Z;
impl RegisterArray for Z {
    const NAME: &'static str = "Z";
    const ELEMS: usize = 32;
    const ELEM_LEN: usize = 128;
}

/// 128-bit views of the Z registers
pub struct Q;
impl RegisterArrayView for Q {
    type RegisterArray = Z;
    const NAME: &'static str = "Q";
    const OFFSET: usize = 0;
    const LEN: usize = 128;
}

/// 64-bit views of the Z registers
pub struct D;
impl RegisterArrayView for D {
    type RegisterArray = Z;
    const NAME: &'static str = "D";
    const OFFSET: usize = 0;
    const LEN: usize = 64;
}

/// 32-bit views of the Z registers
pub struct S;
impl RegisterArrayView for S {
    type RegisterArray = Z;
    const NAME: &'static str = "S";
    const OFFSET: usize = 0;
    const LEN: usize = 32;
}

/// 16-bit views of the Z registers
pub struct H;
impl RegisterArrayView for H {
    type RegisterArray = Z;
    const NAME: &'static str = "H";
    const OFFSET: usize = 0;
    const LEN: usize = 16;
}

/// 8-bit views of the Z registers
pub struct B;
impl RegisterArrayView for B {
    type RegisterArray = Z;
    const NAME: &'static str = "B";
    const OFFSET: usize = 0;
    const LEN: usize = 8;
}

/// 32-bit view of SP_EL0
#[allow(non_camel_case_types)]
pub struct WSP_EL0;
impl RegisterView for WSP_EL0 {
    type Register = aarch64::SP_EL0;
    const NAME: &'static str = "WSP_EL0";
    const OFFSET: usize = 0;
    const LEN: usize = 32;
}

/// 32-bit view of SP_EL1
#[allow(non_camel_case_types)]
pub struct WSP_EL1;
impl RegisterView for WSP_EL1 {
    type Register = aarch64::SP_EL1;
    const NAME: &'static str = "WSP_EL1";
    const OFFSET: usize = 0;
    const LEN: usize = 32;
}

/// 32-bit view of SP_EL2
#[allow(non_camel_case_types)]
pub struct WSP_EL2;
impl RegisterView for WSP_EL2 {
    type Register = aarch64::SP_EL2;
    const NAME: &'static str = "WSP_EL2";
    const OFFSET: usize = 0;
    const LEN: usize = 32;
}

/// 32-bit view of SP_EL3
#[allow(non_camel_case_types)]
pub struct WSP_EL3;
impl RegisterView for WSP_EL3 {
    type Register = aarch64::SP_EL3;
    const NAME: &'static str = "WSP_EL3";
    const OFFSET: usize = 0;
    const LEN: usize = 32;
}
