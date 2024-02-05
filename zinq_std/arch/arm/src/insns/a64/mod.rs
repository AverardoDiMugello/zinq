use zinq::insn::semantics::{expr::*, IrBlock, Var};
use zinq::*;

use crate::Arm;

pub mod branch;
pub mod data;
pub mod mem;

// Helpers

/// A64 condition codes
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
    assign_64!(pc <= read_proc_64!(&proc.pc), in code);
    assign_64!(next_insn <= add_64!(var!(pc), lit!(4)), in code);
    proc_write_64!(var!(next_insn) => &proc.pc, in code);
}

/// Generate ConditionHolds
fn condition_holds<'p>(cond: CondCode, ctx: &'p Arm, code: &mut IrBlock<'p>) -> Var<bool> {
    let result = match cond {
        CondCode::EQ | CondCode::NE => {
            assign_bool!(z <= read_proc_bool!(&ctx.pstate.z), in code);
            assign_bool!(result <= eq_bool!(var!(z), lit!(true)), in code);
            result
        }
        CondCode::CS | CondCode::CC => {
            assign_bool!(c <= read_proc_bool!(&ctx.pstate.c), in code);
            assign_bool!(result <= eq_bool!(var!(c), lit!(true)), in code);
            result
        }
        CondCode::MI | CondCode::PL => {
            assign_bool!(n <= read_proc_bool!(&ctx.pstate.n), in code);
            assign_bool!(result <= eq_bool!(var!(n), lit!(true)), in code);
            result
        }
        CondCode::VS | CondCode::VC => {
            assign_bool!(v <= read_proc_bool!(&ctx.pstate.v), in code);
            assign_bool!(result <= eq_bool!(var!(v), lit!(true)), in code);
            result
        }
        CondCode::HI | CondCode::LS => {
            assign_bool!(c <= read_proc_bool!(&ctx.pstate.c), in code);
            assign_bool!(z <= read_proc_bool!(&ctx.pstate.z), in code);
            assign_bool!(res1 <= eq_bool!(var!(c), lit!(true)), in code);
            assign_bool!(res2 <= eq_bool!(var!(z), lit!(false)), in code);
            assign_bool!(result <= and_bool!(var!(res1), var!(res2)), in code);
            result
        }
        CondCode::GE | CondCode::LT => {
            assign_bool!(n <= read_proc_bool!(&ctx.pstate.n), in code);
            assign_bool!(v <= read_proc_bool!(&ctx.pstate.v), in code);
            assign_bool!(result <= eq_bool!(var!(n), var!(v)), in code);
            result
        }
        CondCode::GT | CondCode::LE => {
            assign_bool!(n <= read_proc_bool!(&ctx.pstate.n), in code);
            assign_bool!(v <= read_proc_bool!(&ctx.pstate.v), in code);
            assign_bool!(res1 <= eq_bool!(var!(n), var!(v)), in code);

            assign_bool!(z <= read_proc_bool!(&ctx.pstate.z), in code);
            assign_bool!(res2 <= eq_bool!(var!(z), lit!(false)), in code);

            assign_bool!(result <= and_bool!(var!(res1), var!(res2)), in code);
            result
        }
        CondCode::AL | CondCode::NV => {
            assign_bool!(result <= term_bool!(lit!(true)), in code);
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
        | CondCode::LE => {
            assign_bool!(result <= not_bool!(var!(result)), in code);
            result
        }
        _ => result,
    };

    result
}

/// Generate AddWithCarry for 32-bit values
fn add_with_carry_32(
    x: Var<u32>,
    y: Var<u32>,
    carry: u64,
    code: &mut IrBlock,
) -> (Var<u32>, Var<bool>, Var<bool>, Var<bool>, Var<bool>) {
    assign_64_from!(x_ext <= zext_32_to_64!(var!(x)), in code);
    assign_64_from!(y_ext <= zext_32_to_64!(var!(y)), in code);

    assign_64!(result_64_no_carry <= add_64!(var!(x_ext), var!(y_ext)), in code);
    assign_64!(result_64 <= add_64!(var!(result_64_no_carry), lit!(carry)), in code);
    assign_32_from!(result <= trunc_64_to_32!(var!(result_64)), in code);

    assign_32!(result_n_minus_1 <= shr_32!(var!(result), lit!(31)), in code);
    assign_bool_from!(n <= neq_32!(var!(result_n_minus_1), lit!(0)), in code);

    assign_bool_from!(z <= eq_32!(var!(result), lit!(0)), in code);

    assign_64_from!(result_uint <= zext_32_to_64!(var!(result)), in code);
    assign_bool_from!(c <= eq_64!(var!(result_uint), var!(result_64)), in code);

    assign_64_from!(result_sint <= sext_32_to_64!(var!(result)), in code);
    assign_bool_from!(v <= eq_64!(var!(result_sint), var!(result_64)), in code);

    (result, n, z, c, v)
}

/// Generate AddWithCarry for 64-bit values
fn add_with_carry_64(
    x: Var<u64>,
    y: Var<u64>,
    carry: u128,
    code: &mut IrBlock,
) -> (Var<u64>, Var<bool>, Var<bool>, Var<bool>, Var<bool>) {
    assign_128_from!(x_ext <= zext_64_to_128!(var!(x)), in code);
    assign_128_from!(y_ext <= zext_64_to_128!(var!(y)), in code);

    assign_128!(result_128_no_carry <= add_128!(var!(x_ext), var!(y_ext)), in code);
    assign_128!(result_128 <= add_128!(var!(result_128_no_carry), lit!(carry)), in code);
    assign_64_from!(result <= trunc_128_to_64!(var!(result_128)), in code);

    assign_64!(result_n_minus_1 <= shr_64!(var!(result), lit!(63)), in code);
    assign_bool_from!(n <= neq_64!(var!(result_n_minus_1), lit!(0)), in code);

    assign_bool_from!(z <= eq_64!(var!(result), lit!(0)), in code);

    assign_128_from!(result_uint <= zext_64_to_128!(var!(result)), in code);
    assign_bool_from!(c <= eq_128!(var!(result_uint), var!(result_128)), in code);

    assign_128_from!(result_sint <= sext_64_to_128!(var!(result)), in code);
    assign_bool_from!(v <= eq_128!(var!(result_sint), var!(result_128)), in code);

    (result, n, z, c, v)
}
