use codegen::{Enum, Function, Impl, Scope};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Path, Result, Token};

struct InsnSet {
    isa_name: Ident,
    insns: Punctuated<Path, Token![,]>,
}

impl Parse for InsnSet {
    fn parse(input: ParseStream) -> Result<Self> {
        let isa_name = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let insns = input.parse_terminated(Path::parse, Token![,])?;
        Ok(InsnSet { isa_name, insns })
    }
}

pub fn codegen(input: TokenStream) -> TokenStream {
    let InsnSet { isa_name, insns } = parse_macro_input!(input);

    let mut scope = Scope::new();
    let enum_name = format!("{isa_name}Insn").to_case(Case::UpperCamel);
    let mut isa_enum = Enum::new(&enum_name);
    isa_enum.derive("Clone").derive("Debug").vis("pub");

    let mut isa_impl = Impl::new(&enum_name);
    isa_impl
        .impl_trait(format!("zinq::insn::Instruction<{isa_name}>"))
        .associate_type("InsnSize", "u32");

    let mut decode_fn = Function::new("decode");
    decode_fn
        .arg("raw", "&[u8]")
        .ret("zinq::Result<Self>")
        .line("let raw_32 = u32::from_le_bytes(raw.get(0..4).unwrap().try_into().unwrap());");
    // TODO: insn size is hard-coded here

    let mut name_fn = Function::new("name");
    name_fn.arg_ref_self().ret("String").line("match self {");

    let mut asm_fn = Function::new("assemble");
    asm_fn
        .arg_ref_self()
        .ret("&Self::InsnSize")
        .line("match self {");

    let mut disas_fn = Function::new("disassemble");
    disas_fn.arg_ref_self().ret("String").line("match self {");

    let mut size_fn = Function::new("size");
    size_fn.arg_ref_self().ret("usize").line("match self {");

    let mut sem_fn = Function::new("semantics");
    sem_fn
        .generic("\'p")
        .arg_ref_self()
        .arg("proc", format!("&'p {isa_name}"))
        .ret("zinq::insn::semantics::IrBlock<'p>")
        .line("match self {");

    for path_to_insn in insns {
        let (path_w_sep, path_no_sep) =
            path_to_insn
                .segments
                .pairs()
                .fold((String::new(), String::new()), |acc, n| {
                    (
                        acc.0 + &format!("{0}{1}", n.value().ident, n.punct().map_or("", |_| "::")),
                        acc.1 + &format!("{}", n.value().ident).to_case(Case::UpperCamel),
                    )
                });

        let variant_name = path_no_sep.to_case(Case::UpperCamel);
        isa_enum.new_variant(&variant_name).tuple(&path_w_sep);

        decode_fn.line(format!(
            "if raw_32 & <{path_w_sep} as zinq::insn::syntax::Decodable<u32>>::FIXEDMASK == <{path_w_sep} as zinq::insn::syntax::Decodable<u32>>::FIXEDBITS {{"
        ));
        decode_fn.line(format!("return <{path_w_sep} as zinq::insn::Instruction<{isa_name}>>::decode(&raw).and_then(|insn| Ok({enum_name}::{variant_name}(insn)));"));
        decode_fn.line(format!("}}"));

        name_fn.line(format!("{enum_name}::{variant_name}(i) => i.name(),"));
        asm_fn.line(format!("{enum_name}::{variant_name}(i) => i.assemble(),"));
        disas_fn.line(format!(
            "{enum_name}::{variant_name}(i) => i.disassemble(),"
        ));
        size_fn.line(format!("{enum_name}::{variant_name}(i) => i.size(),"));
        sem_fn.line(format!(
            "{enum_name}::{variant_name}(i) => i.semantics(&proc),"
        ));
    }

    decode_fn.line("return Err(zinq::Error(format!(\"Undecodable instruction!\")))");
    isa_impl.push_fn(decode_fn);
    name_fn.line("}");
    isa_impl.push_fn(name_fn);
    asm_fn.line("}");
    isa_impl.push_fn(asm_fn);
    disas_fn.line("}");
    isa_impl.push_fn(disas_fn);
    size_fn.line("}");
    isa_impl.push_fn(size_fn);
    sem_fn.line("}");
    isa_impl.push_fn(sem_fn);

    scope.push_enum(isa_enum);
    scope.push_impl(isa_impl);

    scope
        .new_impl(&enum_name)
        .impl_trait("zinq::insn::syntax::Decodable<u32>")
        .associate_const("FIXEDBITS", "u32", "0", "")
        .associate_const("FIXEDMASK", "u32", "0", "");
    // TODO: insn size is hard-coded here

    scope.to_string().parse().unwrap()
}
