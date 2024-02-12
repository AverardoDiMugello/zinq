use bitvec::prelude::*;
use zinq::insn::semantics::*;

use crate::Arm;

// Assembly helpers

/// Return assembly symbol of a given a64 register
pub fn reg_symbol(sf: bool, reg: usize) -> String {
    // 64-bit variant
    if sf {
        // Special symbol for stack pointer
        if reg == 31 {
            String::from("SP")
        } else {
            format!("X{reg}")
        }
    } else {
        // 32-bit variant
        if reg == 31 {
            String::from("WSP")
        } else {
            format!("W{reg}")
        }
    }
}

// Semantics helpers

/// Generate X register non-assignment. rd == 31 is always zero
pub fn x_read<'p>(n: usize, width: usize, proc: &'p Arm, code: &mut IrBlock<'p>) -> Var {
    let x = if n != 31 {
        code.assign(Expr::ReadProc(&proc.r[n]))
    } else {
        code.assign(Expr::Term(Term::Lit(bitvec![0; 64])))
    };
    code.assign(Expr::Slice {
        val: Term::Var(x),
        start: 0,
        len: width,
    })
}

/// Generate X register assignment. rd == 31 is a no-op
pub fn x_set<'p>(d: usize, val: Term, proc: &'p Arm, code: &mut IrBlock<'p>) {
    if d != 31 {
        code.write_proc(&proc.r[d], 0, val);
    }
}

/// Generate SP register non-assignment
pub fn sp_read<'p>(width: usize, proc: &'p Arm, code: &mut IrBlock<'p>) -> Var {
    let sp = code.assign(Expr::ReadProc(&proc.sp()));
    code.assign(Expr::Slice {
        val: Term::Var(sp),
        start: 0,
        len: width,
    })
}

/// Generate SP register assignment
pub fn sp_set<'p>(val: Term, proc: &'p Arm, code: &mut IrBlock<'p>) {
    code.write_proc(&proc.sp(), 0, val);
}

/// Generate instruction increment
pub fn inc_pc<'p>(proc: &'p Arm, code: &mut IrBlock<'p>) {
    let curr_pc = code.assign(Expr::ReadProc(&proc.pc));
    let next_pc = code.assign(Expr::Binary(
        BinOp::Add,
        Term::Var(curr_pc),
        Term::Lit(BitVec::from_element(4)),
        false,
    ));
    code.write_proc(&proc.pc, 0, Term::Var(next_pc));
}

/// Generate variables for PC + offset and PC + 4
pub fn pc_offset_and_next<'p>(offset: Term, proc: &'p Arm, code: &mut IrBlock<'p>) -> (Var, Var) {
    let pc = code.assign(Expr::ReadProc(&proc.pc));
    let pc_plus_offset = code.assign(Expr::Binary(BinOp::Add, Term::Var(pc), offset, true));
    let pc_plus_4 = code.assign(Expr::Binary(
        BinOp::Add,
        Term::Var(pc),
        Term::Lit(BitVec::from_element(4)),
        false,
    ));
    (pc_plus_offset, pc_plus_4)
}
