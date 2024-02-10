use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Error, Ident, LitBool, LitInt, Result, Token};

enum TermBool {
    Lit(LitBool),
    Var(Ident),
}

impl Parse for TermBool {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitBool) {
            let lit = input.parse()?;
            Ok(TermBool::Lit(lit))
        } else if lookahead.peek(Ident) {
            let ident = input.parse()?;
            Ok(TermBool::Var(ident))
        } else {
            Err(lookahead.error())
        }
    }
}

enum TermInt {
    Lit(LitInt),
    Var(Ident),
}

impl Parse for TermInt {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitInt) {
            let lit = input.parse()?;
            Ok(TermInt::Lit(lit))
        } else if lookahead.peek(Ident) {
            let ident = input.parse()?;
            Ok(TermInt::Var(ident))
        } else {
            Err(lookahead.error())
        }
    }
}

enum Branch {
    Uncond {
        addr: TermInt,
    },
    Cond {
        cond: TermBool,
        if_true: TermInt,
        if_false: TermInt,
    },
}

impl Parse for Branch {
    fn parse(input: ParseStream) -> Result<Self> {
        let if_true = input.parse::<TermInt>()?;
        if input.is_empty() {
            Ok(Branch::Uncond { addr: if_true })
        } else {
            input.parse::<Token![if]>()?;
            let cond = input.parse::<TermBool>()?;
            input.parse::<Token![else]>()?;
            let if_false = input.parse::<TermInt>()?;
            Ok(Branch::Cond {
                cond,
                if_true,
                if_false,
            })
        }
    }
}

pub fn gen_branch(input: TokenStream) -> TokenStream {
    // let br = parse_macro_input!(input as Branch);
    // match br {
    //     Branch::Uncond { addr } =>
    // }
    todo!("this.. :|")
}
