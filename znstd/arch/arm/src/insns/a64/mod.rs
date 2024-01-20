use std::marker::PhantomData;

use zinq::insn::semantics::{expr::*, IrBlock, Var};

pub mod branch;
pub mod data;
pub mod mem;

// Helpers

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

/// Generate AddWithCarry for 64-bit values
fn add_with_carry_64(
    operand1: Var<u64>,
    operand2: Var<u64>,
    carry: u128,
    code: &mut IrBlock,
) -> (Var<u64>, Var<bool>, Var<bool>, Var<bool>, Var<bool>) {
    // let 'unsigned_sum = UInt(x) + UInt(y) + UInt(carry_in);
    let op1_ext = code.assign_128_conv(To128::From64(ExtConv::Zero(
        Term::Var(operand1),
        PhantomData,
    )));
    let op2_ext = code.assign_128_conv(To128::From64(ExtConv::Zero(
        Term::Var(operand2),
        PhantomData,
    )));
    let unsigned_sum_no_carry = code.assign_128(Expr128::Arith(Arith::Add {
        a: Term::Var(op1_ext),
        b: Term::Var(op2_ext),
    }));
    let unsigned_sum = code.assign_128(Expr128::Arith(Arith::Add {
        a: Term::Var(unsigned_sum_no_carry),
        b: Term::Lit(carry),
    }));

    // let 'signed_sum = SInt(x) + SInt(y) + UInt(carry_in);
    let op1_sext = code.assign_128_conv(To128::From64(ExtConv::Signed(
        Term::Var(operand1),
        PhantomData,
    )));
    let op2_sext = code.assign_128_conv(To128::From64(ExtConv::Signed(
        Term::Var(operand2),
        PhantomData,
    )));
    let signed_sum_no_carry = code.assign_128(Expr128::Arith(Arith::Add {
        a: Term::Var(op1_sext),
        b: Term::Var(op2_sext),
    }));
    let signed_sum = code.assign_128(Expr128::Arith(Arith::Add {
        a: Term::Var(signed_sum_no_carry),
        b: Term::Lit(carry),
    }));

    // let result : bits('N) = unsigned_sum['N - 1 .. 0];
    let result = code.assign_64_conv(To64::From128(TruncConv::Zero(
        Term::Var(unsigned_sum),
        PhantomData,
    )));

    // let n : bits(1) = [result['N - 1]];
    let result_n_minus_1 = code.assign_64(Expr64::Bitwise(Bitwise::ShiftR {
        val: Term::Var(result),
        amt: Term::Lit(63),
    }));
    let n = code.assign_bool_conv(ToBool::From64(Term::Var(result_n_minus_1)));
    // let z : bits(1) = if IsZero(result) then 0b1 else 0b0;
    let z = code.assign_bool_conv(ToBool::From64(Term::Var(result)));
    // let c : bits(1) = if UInt(result) == unsigned_sum then 0b0 else 0b1;
    let result_zext =
        code.assign_128_conv(To128::From64(ExtConv::Zero(Term::Var(result), PhantomData)));
    let c = code.assign_bool_conv(ToBool::Cmp128(Cmp::Eq {
        lhs: Term::Var(result_zext),
        rhs: Term::Var(unsigned_sum),
    }));
    // let v : bits(1) = if SInt(result) == signed_sum then 0b0 else 0b1;
    let result_sext = code.assign_128_conv(To128::From64(ExtConv::Signed(
        Term::Var(result),
        PhantomData,
    )));
    let v = code.assign_bool_conv(ToBool::Cmp128(Cmp::Eq {
        lhs: Term::Var(result_sext),
        rhs: Term::Var(signed_sum),
    }));

    (result, n, z, c, v)
}

/// Generate AddWithCarry for 32-bit values
fn add_with_carry_32(
    operand1: Var<u32>,
    operand2: Var<u32>,
    carry: u64,
    code: &mut IrBlock,
) -> (Var<u32>, Var<bool>, Var<bool>, Var<bool>, Var<bool>) {
    // let 'unsigned_sum = UInt(x) + UInt(y) + UInt(carry_in);
    let op1_ext = code.assign_64_conv(To64::From32(ExtConv::Zero(
        Term::Var(operand1),
        PhantomData,
    )));
    let op2_ext = code.assign_64_conv(To64::From32(ExtConv::Zero(
        Term::Var(operand2),
        PhantomData,
    )));
    let unsigned_sum_no_carry = code.assign_64(Expr64::Arith(Arith::Add {
        a: Term::Var(op1_ext),
        b: Term::Var(op2_ext),
    }));
    let unsigned_sum = code.assign_64(Expr64::Arith(Arith::Add {
        a: Term::Var(unsigned_sum_no_carry),
        b: Term::Lit(carry),
    }));

    // let 'signed_sum = SInt(x) + SInt(y) + UInt(carry_in);
    let op1_sext = code.assign_64_conv(To64::From32(ExtConv::Signed(
        Term::Var(operand1),
        PhantomData,
    )));
    let op2_sext = code.assign_64_conv(To64::From32(ExtConv::Signed(
        Term::Var(operand2),
        PhantomData,
    )));
    let signed_sum_no_carry = code.assign_64(Expr64::Arith(Arith::Add {
        a: Term::Var(op1_sext),
        b: Term::Var(op2_sext),
    }));
    let signed_sum = code.assign_64(Expr64::Arith(Arith::Add {
        a: Term::Var(signed_sum_no_carry),
        b: Term::Lit(carry),
    }));

    // let result : bits('N) = unsigned_sum['N - 1 .. 0];
    let result = code.assign_32_conv(To32::From64(TruncConv::Zero(
        Term::Var(unsigned_sum),
        PhantomData,
    )));

    // let n : bits(1) = [result['N - 1]];
    let result_n_minus_1 = code.assign_32(Expr32::Bitwise(Bitwise::ShiftR {
        val: Term::Var(result),
        amt: Term::Lit(31),
    }));
    let n = code.assign_bool_conv(ToBool::From32(Term::Var(result_n_minus_1)));
    // let z : bits(1) = if IsZero(result) then 0b1 else 0b0;
    let z = code.assign_bool_conv(ToBool::From32(Term::Var(result)));
    // let c : bits(1) = if UInt(result) == unsigned_sum then 0b0 else 0b1;
    let result_zext =
        code.assign_64_conv(To64::From32(ExtConv::Zero(Term::Var(result), PhantomData)));
    let c = code.assign_bool_conv(ToBool::Cmp64(Cmp::Eq {
        lhs: Term::Var(result_zext),
        rhs: Term::Var(unsigned_sum),
    }));
    // let v : bits(1) = if SInt(result) == signed_sum then 0b0 else 0b1;
    let result_sext = code.assign_64_conv(To64::From32(ExtConv::Signed(
        Term::Var(result),
        PhantomData,
    )));
    let v = code.assign_bool_conv(ToBool::Cmp64(Cmp::Eq {
        lhs: Term::Var(result_sext),
        rhs: Term::Var(signed_sum),
    }));

    (result, n, z, c, v)
}
