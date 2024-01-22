/*
 * Terminals
 */

/// Build a literal. Type does not need to be specified
#[macro_export]
macro_rules! lit {
    ($e:expr) => {
        Term::Lit($e)
    };
}

/// Build a variable. Type does not need to be specified
#[macro_export]
macro_rules! var {
    ($name:ident) => {
        Term::Var($name)
    };
}

/// Bool terminal
#[macro_export]
macro_rules! term_bool {
    ($e:expr) => {
        ExprBool::Term($e)
    };
}

/// 32-bit terminal
#[macro_export]
macro_rules! term_32 {
    ($e:expr) => {
        Expr32::Term($e)
    };
}

/// 64-bit terminal
#[macro_export]
macro_rules! term_64 {
    ($e:expr) => {
        Expr64::Term($e)
    };
}

/// 128-bit terminal
#[macro_export]
macro_rules! term_128 {
    ($e:expr) => {
        Expr128::Term($e)
    };
}

/*
 * Logic exprs
 */

// Bools

/// Bool And expression
#[macro_export]
macro_rules! and_bool {
    ($a:expr, $b:expr) => {
        ExprBool::Logic(Logic::And { a: $a, b: $b })
    };
}

/// Bool Not expression
#[macro_export]
macro_rules! not_bool {
    ($e:expr) => {
        ExprBool::Logic(Logic::Not($e))
    };
}

/// Bool Or expression
#[macro_export]
macro_rules! or_bool {
    ($a:expr, $b:expr) => {
        ExprBool::Logic(Logic::Or { a: $a, b: $b })
    };
}

/// Bool Xor expression
#[macro_export]
macro_rules! xor_bool {
    ($a:expr, $b:expr) => {
        ExprBool::Logic(Logic::Xor { a: $a, b: $b })
    };
}

// 32-bit

/// 32-bit And expression
#[macro_export]
macro_rules! and_32 {
    ($a:expr, $b:expr) => {
        Expr32::Logic(Logic::And { a: $a, b: $b })
    };
}

/// 32-bit Not expression
#[macro_export]
macro_rules! not_32 {
    ($e:expr) => {
        Expr32::Logic(Logic::Not($e))
    };
}

/// 32-bit Or expression
#[macro_export]
macro_rules! or_32 {
    ($a:expr, $b:expr) => {
        Expr32::Logic(Logic::Or { a: $a, b: $b })
    };
}

/// 32-bit Xor expression
#[macro_export]
macro_rules! xor_32 {
    ($a:expr, $b:expr) => {
        Expr32::Logic(Logic::Xor { a: $a, b: $b })
    };
}

// 64-bit

/// 64-bit And expression
#[macro_export]
macro_rules! and_64 {
    ($a:expr, $b:expr) => {
        Expr64::Logic(Logic::And { a: $a, b: $b })
    };
}

/// 64-bit Not expression
#[macro_export]
macro_rules! not_64 {
    ($e:expr) => {
        Expr64::Logic(Logic::Not($e))
    };
}

/// 64-bit Or expression
#[macro_export]
macro_rules! or_64 {
    ($a:expr, $b:expr) => {
        Expr64::Logic(Logic::Or { a: $a, b: $b })
    };
}

/// 64-bit Xor expression
#[macro_export]
macro_rules! xor_64 {
    ($a:expr, $b:expr) => {
        Expr64::Logic(Logic::Xor { a: $a, b: $b })
    };
}

// 128-bit

/// 128-bit And expression
#[macro_export]
macro_rules! and_128 {
    ($a:expr, $b:expr) => {
        Expr128::Logic(Logic::And { a: $a, b: $b })
    };
}

/// 128-bit Not expression
#[macro_export]
macro_rules! not_128 {
    ($e:expr) => {
        Expr128::Logic(Logic::Not($e))
    };
}

/// 128-bit Or expression
#[macro_export]
macro_rules! or_128 {
    ($a:expr, $b:expr) => {
        Expr128::Logic(Logic::Or { a: $a, b: $b })
    };
}

/// 128-bit Xor expression
#[macro_export]
macro_rules! xor_128 {
    ($a:expr, $b:expr) => {
        Expr128::Logic(Logic::Xor { a: $a, b: $b })
    };
}

/*
 * Arith exprs
 */

// 32-bit

/// 32-bit Add expression
#[macro_export]
macro_rules! add_32 {
    ($a:expr, $b:expr) => {
        Expr32::Arith(Arith::Add { a: $a, b: $b })
    };
}

/// 32-bit Sub expression
#[macro_export]
macro_rules! sub_32 {
    ($a:expr, $b:expr) => {
        Expr32::Arith(Arith::Sub { a: $a, b: $b })
    };
}

// 64-bit

/// 64-bit Add expression
#[macro_export]
macro_rules! add_64 {
    ($a:expr, $b:expr) => {
        Expr64::Arith(Arith::Add { a: $a, b: $b })
    };
}

/// 64-bit Sub expression
#[macro_export]
macro_rules! sub_64 {
    ($a:expr, $b:expr) => {
        Expr64::Arith(Arith::Sub { a: $a, b: $b })
    };
}

// 128-bit

/// 128-bit Add expression
#[macro_export]
macro_rules! add_128 {
    ($a:expr, $b:expr) => {
        Expr128::Arith(Arith::Add { a: $a, b: $b })
    };
}

/// 128-bit Sub expression
#[macro_export]
macro_rules! sub_128 {
    ($a:expr, $b:expr) => {
        Expr128::Arith(Arith::Sub { a: $a, b: $b })
    };
}

/*
 * Bitwise
 */

// 32-bit

/// 32-bit ShiftL expression
#[macro_export]
macro_rules! shl_32 {
    ($a:expr, $b:expr) => {
        Expr32::Bitwise(Bitwise::ShiftL { val: $a, amt: $b })
    };
}

/// 32-bit ShiftR expression
#[macro_export]
macro_rules! shr_32 {
    ($a:expr, $b:expr) => {
        Expr32::Bitwise(Bitwise::ShiftR { val: $a, amt: $b })
    };
}

// 64-bit

/// 64-bit ShiftL expression
#[macro_export]
macro_rules! shl_64 {
    ($a:expr, $b:expr) => {
        Expr64::Bitwise(Bitwise::ShiftL { val: $a, amt: $b })
    };
}

/// 64-bit ShiftR expression
#[macro_export]
macro_rules! shr_64 {
    ($a:expr, $b:expr) => {
        Expr64::Bitwise(Bitwise::ShiftR { val: $a, amt: $b })
    };
}

// 128-bit

/// 128-bit ShiftL expression
#[macro_export]
macro_rules! shl_128 {
    ($a:expr, $b:expr) => {
        Expr128::Bitwise(Bitwise::ShiftL { a: $a, b: $b })
    };
}

/// 128-bit ShiftR expression
#[macro_export]
macro_rules! shr_128 {
    ($a:expr, $b:expr) => {
        Expr128::Bitwise(Bitwise::ShiftR { a: $a, b: $b })
    };
}

/*
 * Read processor context
 */

/// Boolean read processor context expression
#[macro_export]
macro_rules! read_proc_bool {
    ($e:expr) => {
        ExprBool::ReadProc(ReadProc::new($e))
    };
}

/// 32-bit read processor context expression
#[macro_export]
macro_rules! read_proc_32 {
    ($e:expr) => {
        Expr32::ReadProc(ReadProc::new($e))
    };
}

/// 64-bit read processor context expression
#[macro_export]
macro_rules! read_proc_64 {
    ($e:expr) => {
        Expr64::ReadProc(ReadProc::new($e))
    };
}

/// 128-bit read processor context expression
#[macro_export]
macro_rules! read_proc_128 {
    ($e:expr) => {
        Expr128::ReadProc(ReadProc::new($e))
    };
}
