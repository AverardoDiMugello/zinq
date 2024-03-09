use bitvec::prelude::*;
use zinq::insn::semantics::IrCtx;

use crate::Arm;

#[derive(Debug, Clone)]
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

impl<T: AsRef<BitSlice>> From<T> for ExtendType {
    fn from(value: T) -> Self {
        let bit2 = *value.as_ref().get(2).expect("Cannot build ExtendType");
        let bit1 = *value.as_ref().get(1).expect("Cannot build ExtendType");
        let bit0 = *value.as_ref().get(0).expect("Cannot build ExtendType");

        match (bit2, bit1, bit0) {
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

// pub fn extend_reg<'p>(
//     reg: usize,
//     extend_type: ExtendType,
//     shift: usize,
//     datasize: usize,
//     proc: &'p Arm,
//     code: &mut IrCtx<'p>,
// ) -> Var {
// }
