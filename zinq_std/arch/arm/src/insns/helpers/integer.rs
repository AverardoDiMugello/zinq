use bitvec::prelude::*;
use zinq::insn::semantics::*;

/// Generate AddWithCarry for the given datasize and return the variables that point to the result and n, z, c, v
pub fn add_with_carry(
    x: Var,
    y: Var,
    carry: bool,
    datasize: usize,
    code: &mut IrBlock,
) -> (Var, (Var, Var, Var, Var)) {
    let carry = if carry { bitvec![1] } else { bitvec![0] };

    // let 'sum = x + y + carry_in;
    let sum = code.assign(Expr::Binary(BinOp::Add, Term::Var(x), Term::Var(y), false));
    let sum = code.assign(Expr::Binary(
        BinOp::Add,
        Term::Var(sum),
        Term::Lit(carry.clone()),
        false,
    ));

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
    let is_pos_x = code.assign(Expr::Unary(UnaOp::IsPos, Term::Var(x)));
    let is_pos_y = code.assign(Expr::Unary(UnaOp::IsPos, Term::Var(y)));
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
