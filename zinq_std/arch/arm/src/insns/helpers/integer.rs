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

// /// Generate AddWithCarry for the given datasize and return the variables that point to the result and n, z, c, v
// pub fn add_with_carry(
//     x: &Term,
//     y: &Term,
//     carry: &Term,
//     datasize: usize,
//     code: &mut IrCtx,
// ) -> (Var, (Var, Var, Var, Var)) {
//     // let sum = x + y + carry_in;
//     let sum = code.assign(Expr::Add(x.clone(), y.clone()));
//     let sum = code.assign(Expr::Add(Term::Var(sum), carry.clone()));

//     // let result : bits('N) = sum['N - 1 .. 0];
//     let result = sum;

//     // let n : bits(1) = [result['N - 1]];
//     let n = code.assign(Expr::Msb(Term::Var(result)));

//     // let z : bits(1) = if IsZero(result) then 0b1 else 0b0;
//     let z = code.assign(Expr::IsZero(Term::Var(result)));

//     // let c : bits(1) = for N > len(A) && len(A) == len(B), ext(A op B) to iN is not equal to (ext(A) to iN) op (ext(B) to iN)
//     let zext_result = code.assign(Expr::Zext {
//         val: Term::Var(result),
//         size: Term::Lit(BitVec::from_element(datasize + 1)),
//     });
//     let zext_x = code.assign(Expr::Zext {
//         val: x.clone(),
//         size: Term::Lit(BitVec::from_element(datasize + 1)),
//     });
//     let zext_y = code.assign(Expr::Zext {
//         val: y.clone(),
//         size: Term::Lit(BitVec::from_element(datasize + 1)),
//     });
//     let zext_x_plus_y = code.assign(Expr::Add(Term::Var(zext_x), Term::Var(zext_y)));
//     let c = code.assign(Expr::Neq(Term::Var(zext_result), Term::Var(zext_x_plus_y)));

//     // let v : bits(1) = (Msb(x) == Msb(y)) & (Msb(x) != Msb(result))
//     let msb_x = code.assign(Expr::Msb(x.clone()));
//     let msb_y = code.assign(Expr::Msb(y.clone()));
//     let msb_eq_x_y = code.assign(Expr::Eq(Term::Var(msb_x), Term::Var(msb_y)));

//     let msb_res = code.assign(Expr::Msb(Term::Var(result)));
//     let msb_neq_x_res = code.assign(Expr::Neq(Term::Var(msb_x), Term::Var(msb_res)));

//     let v = code.assign(Expr::And(Term::Var(msb_eq_x_y), Term::Var(msb_neq_x_res)));

//     (result, (n, z, c, v))
// }

ir_fn!(add_with_carry, SZ in {I32, I64}, (
    x: SZ,
    y: SZ,
    carry: Term<I1>
) -> (SZ, (Var<bool>, Var<bool>, Var<bool>, Var<bool>)) {
    // comment...
});
