use std::fmt;

use super::x_read;
use crate::Arm;
use zinq::insn::semantics::*;

#[derive(Debug, Clone, Copy)]
pub enum ShiftType {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

impl From<(bool, bool)> for ShiftType {
    fn from(value: (bool, bool)) -> Self {
        let (high, low) = value;
        match (high, low) {
            (false, false) => ShiftType::Lsl,
            (false, true) => ShiftType::Lsr,
            (true, false) => ShiftType::Asr,
            (true, true) => ShiftType::Ror,
        }
    }
}

impl fmt::Display for ShiftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lsl => write!(f, "LSL"),
            Self::Lsr => write!(f, "LSR"),
            Self::Asr => write!(f, "ASR"),
            Self::Ror => write!(f, "ROR"),
        }
    }
}

/// Generate ShiftReg and return a Var that points to the result
pub fn shift_reg<'p>(
    reg: usize,
    shift_type: ShiftType,
    shift_amt: usize,
    datasize: usize,
    proc: &'p Arm,
    code: &mut IrBlock<'p>,
) -> Var {
    let result = x_read(reg, datasize, proc, code);
    let result = match shift_type {
        ShiftType::Lsl => code.assign(Expr::Lsl {
            val: Term::Var(result),
            amt: shift_amt,
        }),
        ShiftType::Lsr => code.assign(Expr::Lsr {
            val: Term::Var(result),
            amt: shift_amt,
        }),
        ShiftType::Asr => code.assign(Expr::Asr {
            val: Term::Var(result),
            amt: shift_amt,
        }),
        ShiftType::Ror => code.assign(Expr::Ror {
            val: Term::Var(result),
            amt: shift_amt,
        }),
    };
    code.assign(Expr::Slice {
        val: Term::Var(result),
        start: 0,
        len: datasize,
    })
}

/// Generate AddWithCarry for the given datasize and return the variables that point to the result and n, z, c, v
pub fn add_with_carry(
    x: Term,
    y: Term,
    carry: Term,
    datasize: usize,
    code: &mut IrBlock,
) -> (Var, (Var, Var, Var, Var)) {
    // let 'sum = x + y + carry_in;
    let sum = code.assign(Expr::Binary(BinOp::Add, x.clone(), y.clone(), false));
    let sum = code.assign(Expr::Binary(BinOp::Add, Term::Var(sum), carry, false));

    // let result : bits('N) = sum['N - 1 .. 0];
    let result = code.assign(Expr::Slice {
        val: Term::Var(sum),
        start: 0,
        len: datasize,
    });

    // let n : bits(1) = [result['N - 1]];
    let n = code.assign(Expr::Slice {
        val: Term::Var(result),
        start: datasize - 1,
        len: 1,
    });
    // let z : bits(1) = if IsZero(result) then 0b1 else 0b0;
    let z = code.assign(Expr::Unary(UnaOp::IsZero, Term::Var(result)));
    // let c : bits(1) = if UInt(result) == unsigned_sum then 0b0 else 0b1;
    let not_c = code.assign(Expr::Binary(
        BinOp::Eq,
        Term::Var(result),
        Term::Var(sum),
        false,
    ));
    let c = code.assign(Expr::Unary(UnaOp::Not, Term::Var(not_c)));
    // let v : bits(1) = if ((IsPos(x) == IsPos(y)) & (IsPos(x) != IsPos(result)))
    let is_pos_x = code.assign(Expr::Unary(UnaOp::IsPos, x));
    let is_pos_y = code.assign(Expr::Unary(UnaOp::IsPos, y));
    let is_pos_res = code.assign(Expr::Unary(UnaOp::IsPos, Term::Var(result)));
    let pos_x_eq_y = code.assign(Expr::Binary(
        BinOp::Eq,
        Term::Var(is_pos_x),
        Term::Var(is_pos_y),
        false,
    ));
    let pos_x_neq_res = code.assign(Expr::Binary(
        BinOp::Neq,
        Term::Var(is_pos_x),
        Term::Var(is_pos_res),
        false,
    ));
    let v = code.assign(Expr::Binary(
        BinOp::And,
        Term::Var(pos_x_eq_y),
        Term::Var(pos_x_neq_res),
        false,
    ));

    (result, (n, z, c, v))
}
