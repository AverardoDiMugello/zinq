use std::marker::PhantomData;

use zinq::insn::semantics::{expr::*, IrBlock, Var};
use zinq::*;

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

/// Generate AddWithCarry for 32-bit values
fn add_with_carry_32(
    operand1: Var<u32>,
    operand2: Var<u32>,
    carry: u64,
    code: &mut IrBlock,
) -> (Var<u32>, Var<bool>, Var<bool>, Var<bool>, Var<bool>) {
    // let 'unsigned_sum = UInt(x) + UInt(y) + UInt(carry_in);
    assign_64_conv!(op1_ext <= zext_32_to_64!(var!(operand1)), in code);
    assign_64_conv!(op2_ext <= zext_32_to_64!(var!(operand2)), in code);
    assign_64!(unsigned_sum_no_carry <= add_64!(var!(op1_ext), var!(op2_ext)), in code);
    assign_64!(unsigned_sum <= add_64!(var!(unsigned_sum_no_carry), lit!(carry)), in code);

    // let 'signed_sum = SInt(x) + SInt(y) + UInt(carry_in);
    assign_64_conv!(op1_sext <= sext_32_to_64!(var!(operand1)), in code);
    assign_64_conv!(op2_sext <= sext_32_to_64!(var!(operand2)), in code);
    assign_64!(signed_sum_no_carry <= add_64!(var!(op1_sext), var!(op2_sext)), in code);
    assign_64!(signed_sum <= add_64!(var!(signed_sum_no_carry), lit!(carry)), in code);

    // let result : bits('N) = unsigned_sum['N - 1 .. 0];
    assign_32_conv!(result <= trunc_64_to_32!(var!(unsigned_sum)), in code);

    // let n : bits(1) = [result['N - 1]];
    assign_32!(result_n_minus_1 <= shr_32!(var!(result), lit!(63)), in code);
    assign_bool_conv!(n <= neq_32!(var!(result_n_minus_1), lit!(0)), in code);
    // let z : bits(1) = if IsZero(result) then 0b1 else 0b0;
    assign_bool_conv!(z <= eq_32!(var!(result), lit!(0)), in code);
    // let c : bits(1) = if UInt(result) == unsigned_sum then 0b0 else 0b1;
    assign_64_conv!(result_zext <= zext_32_to_64!(var!(result)), in code);
    assign_bool_conv!(c <= eq_64!(var!(result_zext), var!(unsigned_sum)), in code);
    // let v : bits(1) = if SInt(result) == signed_sum then 0b0 else 0b1;
    assign_64_conv!(result_sext <= sext_32_to_64!(var!(result)), in code);
    assign_bool_conv!(v <= eq_64!(var!(result_sext), var!(signed_sum)), in code);

    (result, n, z, c, v)
}

/// Generate AddWithCarry for 64-bit values
fn add_with_carry_64(
    operand1: Var<u64>,
    operand2: Var<u64>,
    carry: u128,
    code: &mut IrBlock,
) -> (Var<u64>, Var<bool>, Var<bool>, Var<bool>, Var<bool>) {
    // let 'unsigned_sum = UInt(x) + UInt(y) + UInt(carry_in);
    assign_128_conv!(op1_ext <= zext_64_to_128!(var!(operand1)), in code);
    assign_128_conv!(op2_ext <= zext_64_to_128!(var!(operand2)), in code);
    assign_128!(unsigned_sum_no_carry <= add_128!(var!(op1_ext), var!(op2_ext)), in code);
    assign_128!(unsigned_sum <= add_128!(var!(unsigned_sum_no_carry), lit!(carry)), in code);

    // let 'signed_sum = SInt(x) + SInt(y) + UInt(carry_in);
    assign_128_conv!(op1_sext <= sext_64_to_128!(var!(operand1)), in code);
    assign_128_conv!(op2_sext <= sext_64_to_128!(var!(operand2)), in code);
    assign_128!(signed_sum_no_carry <= add_128!(var!(op1_sext), var!(op2_sext)), in code);
    assign_128!(signed_sum <= add_128!(var!(signed_sum_no_carry), lit!(carry)), in code);

    // let result : bits('N) = unsigned_sum['N - 1 .. 0];
    assign_64_conv!(result <= trunc_128_to_64!(var!(unsigned_sum)), in code);

    // let n : bits(1) = [result['N - 1]];
    assign_64!(result_n_minus_1 <= shr_64!(var!(result), lit!(63)), in code);
    assign_bool_conv!(n <= neq_64!(var!(result_n_minus_1), lit!(0)), in code);
    // let z : bits(1) = if IsZero(result) then 0b1 else 0b0;
    assign_bool_conv!(z <= eq_64!(var!(result), lit!(0)), in code);
    // let c : bits(1) = if UInt(result) == unsigned_sum then 0b0 else 0b1;
    assign_128_conv!(result_zext <= zext_64_to_128!(var!(result)), in code);
    assign_bool_conv!(c <= eq_128!(var!(result_zext), var!(unsigned_sum)), in code);
    // let v : bits(1) = if SInt(result) == signed_sum then 0b0 else 0b1;
    assign_128_conv!(result_sext <= sext_64_to_128!(var!(result)), in code);
    assign_bool_conv!(v <= eq_128!(var!(result_sext), var!(signed_sum)), in code);

    (result, n, z, c, v)
}
