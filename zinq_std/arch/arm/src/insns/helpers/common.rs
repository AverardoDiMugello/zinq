use bitvec::prelude::*;
use zinq::insn::semantics::*;

/// Generate a vector of zeros of the given length
pub fn zeros(width: usize, code: &mut IrBlock) -> Var {
    let z = code.assign(Expr::Term(Term::Lit(bitvec![0; 64])));
    code.assign(Expr::Slice {
        val: Term::Var(z),
        start: 0,
        len: width,
    })
}
