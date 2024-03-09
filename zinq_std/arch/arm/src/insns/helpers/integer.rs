// use super::{enums::*, x_read};
use crate::Arm;
use bitvec::prelude::*;
use zinq::insn::semantics::*;
use zinq_macros::ir_fn;

// /// Generate ShiftReg and return a Var that points to the result
// pub fn shift_reg<'p>(
//     reg: usize,
//     shift_type: ShiftType,
//     shift_amt: usize,
//     datasize: usize,
//     proc: &'p Arm,
//     code: &mut IrCtx<'p>,
// ) -> Var {
//     let result = x_read(reg, datasize, proc, code);
//     let result = match shift_type {
//         ShiftType::LSL => code.assign(Expr::Lsl {
//             val: Term::Var(result),
//             amt: shift_amt,
//         }),
//         ShiftType::LSR => code.assign(Expr::Lsr {
//             val: Term::Var(result),
//             amt: shift_amt,
//         }),
//         ShiftType::ASR => code.assign(Expr::Asr {
//             val: Term::Var(result),
//             amt: shift_amt,
//         }),
//         ShiftType::ROR => code.assign(Expr::Ror {
//             val: Term::Var(result),
//             amt: shift_amt,
//         }),
//     };
//     code.assign(Expr::Slice {
//         val: Term::Var(result),
//         start: 0,
//         len: datasize,
//     })
// }

ir_fn!(add_with_carry, Sz in {I32, I64}, (
    x: Sz,
    y: Sz,
    carry: Term<I1>
) -> (Sz, (Var<I1>, Var<I1>, Var<I1>, Var<I1>)) {
    // let sum = x + y + carry_in;
    let sum = add<Sz> x, y;
    let carry = zext<Sz> carry;
    let result = add<Sz> sum, carry;

    // bit n = result<N-1>;
    let n = msb<Sz> result;

    // bit z = if ISzero(result) then '1' else '0';
    let z = eq<Sz> result, 0;

    // bit c = if UInt(result) == unsigned_sum then '0' else '1';
    // ext(A op B) to iN is not equal to (ext(A) to iN) op (ext(B) to iN)
    let zext_res = zext_next<Sz> result;
    let zext_x = zext_next<Sz> x;
    let zext_y = zext_next<Sz> y;
    let sum_zext_xy = add<Sz::up1> zext_x, zext_y;
    let c = eq<Sz::up1> zext_res, sum_zext_xy;

    // bit v = if SInt(result) == signed_sum then '0' else '1';
    // msb(x) == msb(y)) & (msb(x) != msb(result))
    let msb_x = msb<Sz> x;
    let msb_y = msb<Sz> y;
    let eq_msb_xy = eq<I1> msb_x, msb_y;
    let msb_res = msb<Sz> result;
    let neq_msb_xres = neq<I1> msb_x, msb_res;
    let v = and<I1> eq_msb_xy, neq_msb_xres;

    return (result, (n, z, c, v));
});
