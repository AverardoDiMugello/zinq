/// Only supports 64-bit assigns
#[macro_export]
macro_rules! assign {
    ($var_name:ident <= $e:expr, in $code:ident) => {
        let $var_name = $code.assign_64($e);
    };
}

/// Only supports boolean ctx writes
#[macro_export]
macro_rules! store {
    ($val:expr => $ctx_var:expr, in $code:ident) => {
        $code.write_ctx_bool($ctx_var, $val)
    };
}

/// Only supports 64-bit gotos
#[macro_export]
macro_rules! goto {
    ($e:expr, in $code:ident) => {
        $code.goto_64($e)
    };
}

/// Only supports 64-bit ctx reads
#[macro_export]
macro_rules! load {
    ($e:expr) => {
        Expr64::ReadCtx(ReadCtx::new($e))
    };
}

#[macro_export]
macro_rules! lit {
    ($e:expr) => {
        Term::Lit($e)
    };
}

#[macro_export]
macro_rules! var {
    ($name:ident) => {
        Term::Var($name)
    };
}
