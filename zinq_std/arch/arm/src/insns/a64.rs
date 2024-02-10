// pub type InsnSize = BitArr!(for 32);
pub type InsnSize = u32;

pub mod branch {
    mod cond;
    pub use cond::Cond;
    mod uncond_imm;
    pub use uncond_imm::UncondImm;
}

pub mod data {
    mod arith_imm;
    pub use arith_imm::ArithImm;
    mod mov_imm;
    pub use mov_imm::MovImm;
}

// pub mod mem;

// Helpers

use bitvec::prelude::*;
use zinq::insn::semantics::*;

use crate::Arm;

/// A64 condition codes
#[derive(Debug, Clone, Copy)]
enum CondCode {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    NV,
}

impl From<u8> for CondCode {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => Self::EQ,
            0b0001 => Self::NE,
            0b0010 => Self::CS,
            0b0011 => Self::CC,
            0b0100 => Self::MI,
            0b0101 => Self::PL,
            0b0110 => Self::VS,
            0b0111 => Self::VC,
            0b1000 => Self::HI,
            0b1001 => Self::LS,
            0b1010 => Self::GE,
            0b1011 => Self::LT,
            0b1100 => Self::GT,
            0b1101 => Self::LE,
            0b1110 => Self::AL,
            0b1111 => Self::NV,
            _ => panic!(
                "Invalid A64 condition code. Arm instructions should never allow this to happen."
            ),
        }
    }
}

// Assembler

/// Return assembly symbol of a given a64 register
fn reg_symbol(sf: bool, reg: usize) -> String {
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

/// Return assembly symbol for a given a64 condition code
fn cond_symbol(code: CondCode) -> String {
    match code {
        CondCode::EQ => String::from("EQ"),
        CondCode::NE => String::from("NE"),
        CondCode::CS => String::from("CS"),
        CondCode::CC => String::from("CC"),
        CondCode::MI => String::from("MI"),
        CondCode::PL => String::from("PL"),
        CondCode::VS => String::from("VS"),
        CondCode::VC => String::from("VC"),
        CondCode::HI => String::from("HI"),
        CondCode::LS => String::from("LS"),
        CondCode::GE => String::from("GE"),
        CondCode::LT => String::from("LT"),
        CondCode::GT => String::from("GT"),
        CondCode::LE => String::from("LE"),
        CondCode::AL => String::from("AL"),
        CondCode::NV => String::from("NV"),
    }
}

// IR

/// Generate instruction increment
fn next_insn<'p>(proc: &'p Arm, code: &mut IrBlock<'p>) {
    let curr_pc = code.assign(Expr::ReadProc(&proc.pc));
    let next_pc = code.assign(Expr::Binary(
        BinOp::Add,
        Term::Var(curr_pc),
        Term::Lit(BitVec::from_element(4)),
        false,
    ));
    code.write_proc(&proc.pc, 0, Term::Var(next_pc));
}

/// Generate X register assignment. rd == 31 is a no-op
fn x_set<'p>(proc: &'p Arm, d: usize, val: Term, code: &mut IrBlock<'p>) {
    if d != 31 {
        code.write_proc(&proc.r[d], 0, val);
    }
}

/// Generate ConditionHolds
fn condition_holds<'p>(cond: CondCode, ctx: &'p Arm, code: &mut IrBlock<'p>) -> Var {
    let result = match cond {
        CondCode::EQ | CondCode::NE => {
            let z = code.assign(Expr::ReadProc(&ctx.pstate.z));
            let result = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(z),
                Term::Lit(bitvec!(1)),
                false,
            ));
            result
        }
        CondCode::CS | CondCode::CC => {
            let c = code.assign(Expr::ReadProc(&ctx.pstate.c));
            let result = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(c),
                Term::Lit(bitvec!(1)),
                false,
            ));
            result
        }
        CondCode::MI | CondCode::PL => {
            let n = code.assign(Expr::ReadProc(&ctx.pstate.n));
            let result = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(n),
                Term::Lit(bitvec!(1)),
                false,
            ));
            result
        }
        CondCode::VS | CondCode::VC => {
            let v = code.assign(Expr::ReadProc(&ctx.pstate.v));
            let result = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(v),
                Term::Lit(bitvec!(1)),
                false,
            ));
            result
        }
        CondCode::HI | CondCode::LS => {
            let c = code.assign(Expr::ReadProc(&ctx.pstate.c));
            let z = code.assign(Expr::ReadProc(&ctx.pstate.z));
            let res1 = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(c),
                Term::Lit(bitvec!(1)),
                false,
            ));
            let res2 = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(z),
                Term::Lit(bitvec!(0)),
                false,
            ));
            let result = code.assign(Expr::Binary(
                BinOp::And,
                Term::Var(res1),
                Term::Var(res2),
                false,
            ));
            result
        }
        CondCode::GE | CondCode::LT => {
            let n = code.assign(Expr::ReadProc(&ctx.pstate.n));
            let v = code.assign(Expr::ReadProc(&ctx.pstate.v));
            let result = code.assign(Expr::Binary(BinOp::Eq, Term::Var(n), Term::Var(v), false));
            result
        }
        CondCode::GT | CondCode::LE => {
            let n = code.assign(Expr::ReadProc(&ctx.pstate.n));
            let v = code.assign(Expr::ReadProc(&ctx.pstate.v));
            let res1 = code.assign(Expr::Binary(BinOp::Eq, Term::Var(n), Term::Var(v), false));

            let z = code.assign(Expr::ReadProc(&ctx.pstate.z));
            let res2 = code.assign(Expr::Binary(
                BinOp::Eq,
                Term::Var(z),
                Term::Lit(bitvec!(0)),
                false,
            ));

            let result = code.assign(Expr::Binary(
                BinOp::And,
                Term::Var(res1),
                Term::Var(res2),
                false,
            ));
            result
        }
        CondCode::AL | CondCode::NV => {
            let result = code.assign(Expr::Term(Term::Lit(bitvec!(1))));
            result
        }
    };

    let result = match cond {
        CondCode::NE
        | CondCode::CC
        | CondCode::PL
        | CondCode::VC
        | CondCode::LS
        | CondCode::LT
        | CondCode::LE => code.assign(Expr::Unary(UnaOp::Not, Term::Var(result))),
        _ => result,
    };

    result
}

/// Generate AddWithCarry for the given datasize and return the variables that point to the result and n, z, c, v
fn add_with_carry(
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
