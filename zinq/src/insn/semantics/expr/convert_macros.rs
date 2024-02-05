/*
 * Convert to boolean via comparison
 */

// 32-bit

/// 32-bit equal expression
#[macro_export]
macro_rules! eq_32 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp32(Cmp::Eq {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 32-bit greater-than expression
#[macro_export]
macro_rules! gt_32 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp32(Cmp::Gt {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 32-bit greater-than-or-equal-to expression
#[macro_export]
macro_rules! gte_32 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp32(Cmp::Gte {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 32-bit less-than expression
#[macro_export]
macro_rules! lt_32 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp32(Cmp::Lt {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 32-bit less-than-or-equal-to expression
#[macro_export]
macro_rules! lte_32 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp32(Cmp::Lte {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 32-bit not-equal expression
#[macro_export]
macro_rules! neq_32 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp32(Cmp::Neq {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

// 64-bit

/// 64-bit equal expression
#[macro_export]
macro_rules! eq_64 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp64(Cmp::Eq {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 64-bit greater-than expression
#[macro_export]
macro_rules! gt_64 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp64(Cmp::Gt {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 64-bit greater-than-or-equal-to expression
#[macro_export]
macro_rules! gte_64 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp64(Cmp::Gte {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 64-bit less-than expression
#[macro_export]
macro_rules! lt_64 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp64(Cmp::Lt {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 64-bit less-than-or-equal-to expression
#[macro_export]
macro_rules! lte_64 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp64(Cmp::Lte {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 64-bit not-equal expression
#[macro_export]
macro_rules! neq_64 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp64(Cmp::Neq {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

// 128-bit

/// 128-bit equal expression
#[macro_export]
macro_rules! eq_128 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp128(Cmp::Eq {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 128-bit greater-than expression
#[macro_export]
macro_rules! gt_128 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp128(Cmp::Gt {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 128-bit greater-than-or-equal-to expression
#[macro_export]
macro_rules! gte_128 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp128(Cmp::Gte {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 128-bit less-than expression
#[macro_export]
macro_rules! lt_128 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp128(Cmp::Lt {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 128-bit less-than-or-equal-to expression
#[macro_export]
macro_rules! lte_128 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp128(Cmp::Lte {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/// 128-bit not-equal expression
#[macro_export]
macro_rules! neq_128 {
    ($lhs:expr, $rhs:expr) => {
        ToBool::Cmp128(Cmp::Neq {
            lhs: $lhs,
            rhs: $rhs,
        })
    };
}

/*
 * Convert to 32-bit
 */

/// Truncate 64-bit value into 32-bit value
#[macro_export]
macro_rules! trunc_64_to_32 {
    ($e:expr) => {
        To32::From64(TruncConv::Zero($e, std::marker::PhantomData))
    };
}

/// Sign-preserving truncate 64-bit value into 32-bit value
#[macro_export]
macro_rules! strunc_64_to_32 {
    ($e:expr) => {
        To32::From64(TruncConv::Signed($e, std::marker::PhantomData))
    };
}

/// Truncate 128-bit value into 32-bit value
#[macro_export]
macro_rules! trunc_128_to_32 {
    ($e:expr) => {
        To32::From128(TruncConv::Zero($e, std::marker::PhantomData))
    };
}

/// Sign-preserving truncate 128-bit value into 32-bit value
#[macro_export]
macro_rules! strunc_128_to_32 {
    ($e:expr) => {
        To32::From128(TruncConv::Signed($e, std::marker::PhantomData))
    };
}

/*
 * Convert to 64-bit
 */

/// Extend 32-bit value into 64-bit value
#[macro_export]
macro_rules! zext_32_to_64 {
    ($e:expr) => {
        To64::From32(ExtConv::Zero($e, std::marker::PhantomData))
    };
}

/// Sign-preserving extend 32-bit value into 64-bit value
#[macro_export]
macro_rules! sext_32_to_64 {
    ($e:expr) => {
        To64::From32(ExtConv::Signed($e, std::marker::PhantomData))
    };
}

/// Truncate 128-bit value into 64-bit value
#[macro_export]
macro_rules! trunc_128_to_64 {
    ($e:expr) => {
        To64::From128(TruncConv::Zero($e, std::marker::PhantomData))
    };
}

/// Sign-preserving truncate 128-bit value into 64-bit value
#[macro_export]
macro_rules! strunc_128_to_64 {
    ($e:expr) => {
        To64::From128(TruncConv::Signed($e, std::marker::PhantomData))
    };
}

/*
 * Convert to 128-bit
 */

/// Extend 32-bit value into 128-bit value
#[macro_export]
macro_rules! zext_32_to_128 {
    ($e:expr) => {
        To128::From32(ExtConv::Zero($expr, std::marker::PhantomData))
    };
}

/// Sign-preserving extend 32-bit value into 128-bit value
#[macro_export]
macro_rules! sext_32_to_128 {
    ($e:expr) => {
        To128::From32(ExtConv::Signed($expr, std::marker::PhantomData))
    };
}
/// Extend 64-bit value into 128-bit value
#[macro_export]
macro_rules! zext_64_to_128 {
    ($e:expr) => {
        To128::From64(ExtConv::Zero($e, std::marker::PhantomData))
    };
}

/// Sign-preserving extend 64-bit value into 128-bit value
#[macro_export]
macro_rules! sext_64_to_128 {
    ($e:expr) => {
        To128::From64(ExtConv::Signed($e, std::marker::PhantomData))
    };
}

/*
 * Convert to address
 */

/// Convert 32-bit value to address
#[macro_export]
macro_rules! to_addr_from_32 {
    ($e:expr) => {
        ToAddr::From32($e)
    };
}

/// Convert 64-bit value to address
#[macro_export]
macro_rules! to_addr_from_64 {
    ($e:expr) => {
        ToAddr::From64($e)
    };
}
