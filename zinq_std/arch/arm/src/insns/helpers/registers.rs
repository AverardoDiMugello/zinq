use bitvec::prelude::*;
use zinq::insn::semantics::*;

use crate::Arm;

// /// Generate X register non-assignment. rd == 31 is always zero
// pub fn x_read<'p>(n: usize, width: usize, proc: &'p Arm, code: &mut IrCtx<'p>) -> Var {
//     let x = if n != 31 {
//         code.assign(Expr::ReadProc(&proc.r[n], width))
//     } else {
//         let mut zeros = BitVec::with_capacity(64);
//         zeros.resize(width, false);
//         code.assign(Expr::Term(Term::Lit(zeros)))
//     };
//     return x;
// }

// /// Generate X register assignment. rd == 31 is a no-op
// pub fn x_set<'p>(d: usize, width: usize, val: Term, proc: &'p Arm, code: &mut IrCtx<'p>) {
//     if d != 31 {
//         code.write_proc(&proc.r[d].get(0..width).unwrap(), val);
//     }
// }

// /// Generate SP register non-assignment
// pub fn sp_read<'p>(width: usize, proc: &'p Arm, code: &mut IrCtx<'p>) -> Var {
//     code.assign(Expr::ReadProc(&proc.sp(), width))
// }

// /// Generate SP register assignment
// pub fn sp_set<'p>(val: Term, proc: &'p Arm, code: &mut IrCtx<'p>) {
//     code.write_proc(&proc.sp(), val);
// }

// /// Generate instruction increment
// pub fn inc_pc<'p>(proc: &'p Arm, code: &mut IrCtx<'p>) {
//     let curr_pc = code.assign(Expr::ReadProc(&proc.pc, 64));
//     let next_pc = code.assign(Expr::Add(
//         Term::Var(curr_pc),
//         Term::Lit(BitVec::from_element(4)),
//     ));
//     code.write_proc(&proc.pc, Term::Var(next_pc));
// }

// /// Generate variables for PC + offset and PC + 4
// pub fn pc_offset_and_next<'p>(offset: Term, proc: &'p Arm, code: &mut IrCtx<'p>) -> (Var, Var) {
//     let pc = code.assign(Expr::ReadProc(&proc.pc, 64));
//     let pc_plus_offset = code.assign(Expr::Add(Term::Var(pc), offset));
//     let pc_plus_4 = code.assign(Expr::Add(Term::Var(pc), Term::Lit(BitVec::from_element(4))));
//     (pc_plus_offset, pc_plus_4)
// }
