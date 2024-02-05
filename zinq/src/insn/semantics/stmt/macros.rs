/// Assign the bool expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_bool {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_bool($e);
    };
}

/// Assign the convert-to-bool expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_bool_from {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_bool_from($e);
    };
}

/// Assign the 32-bit expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_32 {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_32($e);
    };
}

/// Assign the convert-to-32-bit expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_32_from {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_32_from($e);
    };
}

/// Assign the 64-bit expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_64 {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_64($e);
    };
}

/// Assign the convert-to-64-bit expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_64_from {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_64_from($e);
    };
}

/// Assign the 128-bit expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_128 {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_128($e);
    };
}

/// Assign the convert-to-128-bit expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_128_from {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_128_from($e);
    };
}

/// Assign the convert-to-addr expression to a variable in the given IR code-block
#[macro_export]
macro_rules! assign_addr_from {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_addr_from($e);
    };
}

/// Write the address terminal into the Processor context
#[macro_export]
macro_rules! proc_write_addr {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.proc_write_addr($ctx_var, $val)
    };
}

/// Write the boolean terminal into the Processor context
#[macro_export]
macro_rules! proc_write_bool {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.proc_write_bool($ctx_var, $val)
    };
}

/// Write the 32-bit terminal into the Processor context
#[macro_export]
macro_rules! proc_write_32 {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.proc_write_32($ctx_var, $val)
    };
}

/// Write the 64-bit terminal into the Processor context
#[macro_export]
macro_rules! proc_write_64 {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.proc_write_64($ctx_var, $val)
    };
}

/// Write the 128-bit terminal into the Processor context
#[macro_export]
macro_rules! proc_write_128 {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.proc_write_128($ctx_var, $val)
    };
}

/// Unconditionally branch to the given address
#[macro_export]
macro_rules! br_uncond {
    ($addr:expr, in $code:ident) => {
        $code.br_uncond($addr)
    };
}

/// Conditionally branch to one of the given addresses
#[macro_export]
macro_rules! br_cond {
    ($cond:expr, $true_case:expr, $false_case:expr, in $code:ident) => {
        $code.br_cond($cond, $true_case, $false_case)
    };
}
