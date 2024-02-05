use proc_macro::TokenStream;

mod insn_set;
mod ir;

#[proc_macro]
pub fn insn_set(input: TokenStream) -> TokenStream {
    insn_set::codegen(input)
}

#[proc_macro]
pub fn r#let(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn br(input: TokenStream) -> TokenStream {
    // br!(addr)
    // or
    // br!(addr1 if cond else addr2)
    // code.br_cond(Term::{Var(v)|Lit(l)}, Term::{Var(v)|Lit(l)})
    // Ident|LitInt
    // or
    // Ident|LitInt if Ident else Ident|LitInt
    ir::gen_branch(input)
}
