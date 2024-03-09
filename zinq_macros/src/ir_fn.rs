use std::ops::Deref;

use codegen::{Function, Scope};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    braced, parenthesized, parse_macro_input, Block, FnArg, Ident, Result, ReturnType, Stmt, Token,
};

use crate::ir::*;

/*
 * ir_fn!(fn_name<h_param in {i32, i64, ...}>(param_1: typ_1, param_2: typ_2, ...) -> ret_type { code... })
 */
struct IrFn {
    name: Ident,
    hparam_name: Ident,
    hparams: Punctuated<IrType, Token![,]>,
    params: Punctuated<FnArg, Token![,]>,
    ret_type: ReturnType,
    stmts: Vec<Stmt>,
}

impl Parse for IrFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;

        // h_param in {I32, I64, ...},
        let hparam_name = input.parse()?;
        input.parse::<Token![in]>()?;
        let content;
        braced!(content in input);
        let hparams = content.parse_terminated(IrType::parse, Token![,])?;
        input.parse::<Token![,]>()?;

        // (param_1: typ_1, param_2: typ_2, ...) -> ret_type
        let content;
        parenthesized!(content in input);
        let params = content.parse_terminated(FnArg::parse, Token![,])?;
        let ret_type = input.parse()?;

        // { code... }
        let content;
        braced!(content in input);
        let stmts = content.call(Block::parse_within)?;

        Ok(IrFn {
            name,
            hparam_name,
            hparams,
            params,
            ret_type,
            stmts,
        })
    }
}

pub fn codegen(input: TokenStream) -> TokenStream {
    let IrFn {
        name,
        hparam_name,
        hparams,
        params,
        ret_type,
        stmts,
    } = parse_macro_input!(input);

    let mut scope = Scope::new();

    for hparam in hparams {
        let fn_name = format!("{name}_{hparam}");
        let func = scope.new_fn(&fn_name).vis("pub"); // TODO: figure out how to handle visibility

        // Add fn args, mapping hyper-parameterized args to Term<hparam>
        params.iter().for_each(|param| {
            if let FnArg::Typed(pat_type) = param {
                // Replace fn args of type "hparam_name" with "Term<hparam>"
                let arg_name = pat_type.pat.to_token_stream().to_string();
                if pat_type.ty.to_token_stream().to_string() == hparam_name.to_string() {
                    func.arg(arg_name.as_str(), format!("Term<{hparam}>"));
                } else {
                    func.arg(arg_name.as_str(), pat_type.ty.to_token_stream().to_string());
                }
            } else {
                panic!("No support for functions with receiver params, e.g. \"self\" yet");
            }
        });
        // The IrCtx where the resulting code will be inserted
        func.arg("code", "&mut IrCtx");

        // Add return value, mapping hyper-parameterized types to Var<hparam>
        if let ReturnType::Type(_, _) = ret_type {
            // TODO: this is super hack-y! We are just replacing all instances of the hparam_name
            // with Var<hparam>. Probably fine in practice for now but needs a real solution.
            func.ret(
                ret_type
                    .to_token_stream()
                    .to_string()
                    .replace("->", "")
                    .replace(
                        hparam_name.to_string().as_str(),
                        format!("Var<{hparam}>").as_str(),
                    ),
            );
        }
    }

    scope.to_string().parse().unwrap()
}
