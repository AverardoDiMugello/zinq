use syn::parse::{Parse, ParseStream};
use syn::Result;

pub mod kw {
    use syn::custom_keyword;

    custom_keyword!(I1);
    custom_keyword!(I32);
    custom_keyword!(I64);
    custom_keyword!(I128);
}

pub enum IrType {
    I1,
    I32,
    I64,
    I128,
}

impl std::fmt::Display for IrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I1 => write!(f, "I1"),
            Self::I32 => write!(f, "I32"),
            Self::I64 => write!(f, "I64"),
            Self::I128 => write!(f, "I128"),
        }
    }
}

impl Parse for IrType {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::I1) {
            input.parse::<kw::I1>()?;
            Ok(Self::I1)
        } else if lookahead.peek(kw::I32) {
            input.parse::<kw::I32>()?;
            Ok(Self::I32)
        } else if lookahead.peek(kw::I64) {
            input.parse::<kw::I64>()?;
            Ok(Self::I64)
        } else if lookahead.peek(kw::I128) {
            input.parse::<kw::I128>()?;
            Ok(Self::I128)
        } else {
            Err(lookahead.error())
        }
    }
}
