/// Assign the bool expression to a variable the given IR code-block
#[macro_export]
macro_rules! assign_bool {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_bool($e);
    };
}

/// Assign the convert-to-bool expression to a variable the given IR code-block
#[macro_export]
macro_rules! assign_bool_conv {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_bool_conv($e);
    };
}

/// Assign the 32-bit expression to a variable the given IR code-block
#[macro_export]
macro_rules! assign_32 {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_32($e);
    };
}

/// Assign the convert-to-32-bit expression to a variable the given IR code-block
#[macro_export]
macro_rules! assign_32_conv {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_32_conv($e);
    };
}

/// Assign the 64-bit expression to a variable the given IR code-block
#[macro_export]
macro_rules! assign_64 {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_64($e);
    };
}

/// Assign the convert-to-64-bit expression to a variable the given IR code-block
#[macro_export]
macro_rules! assign_64_conv {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_64_conv($e);
    };
}

/// Assign the 128-bit expression to a variable the given IR code-block
#[macro_export]
macro_rules! assign_128 {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_128($e);
    };
}

/// Assign the convert-to-128-bit expression to a variable the given IR code-block
#[macro_export]
macro_rules! assign_128_conv {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_128_conv($e);
    };
}

/// Write the boolean terminal into the Processor context
#[macro_export]
macro_rules! write_proc_bool {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.write_proc_bool($ctx_var, $val)
    };
}

/// Write the 32-bit terminal into the Processor context
#[macro_export]
macro_rules! write_proc_32 {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.write_proc_32($ctx_var, $val)
    };
}

/// Write the 64-bit terminal into the Processor context
#[macro_export]
macro_rules! write_proc_64 {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.write_proc_64($ctx_var, $val)
    };
}

/// Write the 128-bit terminal into the Processor context
#[macro_export]
macro_rules! write_proc_128 {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.write_proc_128($ctx_var, $val)
    };
}

#[macro_export]
macro_rules! goto_32 {
    ($e:expr, in $code:ident) => {
        $code.goto_32($e)
    };
}

#[macro_export]
macro_rules! goto_64 {
    ($e:expr, in $code:ident) => {
        $code.goto_64($e)
    };
}
