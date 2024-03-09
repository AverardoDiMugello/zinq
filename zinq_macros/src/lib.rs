use proc_macro::TokenStream;

mod insn_set;
mod ir;
mod ir_block;
mod ir_fn;

#[proc_macro]
pub fn insn_set(input: TokenStream) -> TokenStream {
    insn_set::codegen(input)
}

#[proc_macro]
pub fn ir_fn(input: TokenStream) -> TokenStream {
    ir_fn::codegen(input)
}
