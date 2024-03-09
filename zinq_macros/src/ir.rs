use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::{braced, Ident, Lit, Result, Stmt};

pub mod kw {
    use syn::custom_keyword;

    // Types
    custom_keyword!(I1);
    custom_keyword!(I32);
    custom_keyword!(I64);
    custom_keyword!(I128);

    // Typed expressions
    // Logic
    custom_keyword!(and);
    custom_keyword!(not);
    // Arith
    custom_keyword!(add);

    // Conversion expressions
    // Bool
    custom_keyword!(eq);
    custom_keyword!(neq);
    custom_keyword!(msb);
    // Number
    custom_keyword!(sext_next);
    custom_keyword!(sext_max);
    custom_keyword!(zext_next);
    custom_keyword!(zext_max);
    custom_keyword!(strunc_next);
    custom_keyword!(strunc_min);
    custom_keyword!(ztrunc_next);
    custom_keyword!(ztrunc_min);
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

pub enum ExprType {
    Explicit(IrType),
    HyperP(Ident),
}

impl std::fmt::Display for ExprType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprType::Explicit(ir_type) => write!(f, "{ir_type}"),
            ExprType::HyperP(ident) => write!(f, "{ident}"),
        }
    }
}

pub enum IrTerm {
    Lit(Lit),
    Var(Ident),
}

impl std::fmt::Display for IrTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrTerm::Lit(l) => write!(f, "{}", l.to_token_stream().to_string()),
            IrTerm::Var(ident) => write!(f, "{ident}"),
        }
    }
}

pub enum IrExpr {
    // Typed expressions
    // Logic
    And(ExprType, IrTerm, IrTerm),
    Not(ExprType, IrTerm),
    // Arith
    Add(ExprType, IrTerm, IrTerm),
    // Conversion expressions
    // Bool
    Eq(ExprType, IrTerm, IrTerm),
    Neq(ExprType, IrTerm, IrTerm),
    Msb(ExprType, IrTerm),
    // Number
    SextNext(ExprType, IrTerm),
    SextMax(ExprType, IrTerm),
    ZextNext(ExprType, IrTerm),
    ZextMax(ExprType, IrTerm),
    StruncNext(ExprType, IrTerm),
    StruncMin(ExprType, IrTerm),
    ZtruncNext(ExprType, IrTerm),
    ZtruncMin(ExprType, IrTerm),
}

impl std::fmt::Display for IrExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrExpr::And(expr_type, a, b) => write!(f, "And<{expr_type} {a}, {b}"),
            IrExpr::Not(expr_type, a) => write!(f, "Not<{expr_type} {a}"),
            IrExpr::Add(expr_type, a, b) => write!(f, "Add<{expr_type} {a}, {b}"),
            IrExpr::Eq(expr_type, a, b) => write!(f, "Eq<{expr_type} {a}, {b}"),
            IrExpr::Neq(expr_type, a, b) => write!(f, "Neq<{expr_type} {a}, {b}"),
            IrExpr::Msb(expr_type, a) => write!(f, "Msb<{expr_type} {a}"),
            IrExpr::SextNext(expr_type, a) => write!(f, "SextNext<{expr_type} {a}"),
            IrExpr::SextMax(expr_type, a) => write!(f, "SextMax<{expr_type} {a}"),
            IrExpr::ZextNext(expr_type, a) => write!(f, "ZextNext<{expr_type} {a}"),
            IrExpr::ZextMax(expr_type, a) => write!(f, "ZextMax<{expr_type} {a}"),
            IrExpr::StruncNext(expr_type, a) => write!(f, "StruncNext<{expr_type} {a}"),
            IrExpr::StruncMin(expr_type, a) => write!(f, "StruncMin<{expr_type} {a}"),
            IrExpr::ZtruncNext(expr_type, a) => write!(f, "ZtruncNext<{expr_type} {a}"),
            IrExpr::ZtruncMin(expr_type, a) => write!(f, "ZtruncMin<{expr_type} {a}"),
        }
    }
}

pub enum IrStmt {
    Rust(Stmt),
    Assignment(IrExpr),
}

fn parse_ir_stmt(input: ParseStream) -> Option<IrStmt> {}

pub struct IrBlock {
    brace_token: Brace,
    stmts: Vec<IrStmt>,
}

impl IrBlock {
    fn parse_within(input: ParseStream) -> Result<Vec<IrStmt>> {
        let mut stmts = Vec::new();
        loop {
            while let semi @ Some(_) = input.parse()? {
                stmts.push(Stmt::Expr(Expr::Verbatim(TokenStream::new()), semi));
            }
            if input.is_empty() {
                break;
            }
            let stmt = if let Some(ir_stmt) = parse_ir_stmt(input) {
                ir_stmt
            } else {
                let stmt = parse_stmt(input, AllowNoSemi(true))?;
                let requires_semicolon = match &stmt {
                    Stmt::Expr(stmt, None) => expr::requires_terminator(stmt),
                    Stmt::Macro(stmt) => {
                        stmt.semi_token.is_none() && !stmt.mac.delimiter.is_brace()
                    }
                    Stmt::Local(_) | Stmt::Item(_) | Stmt::Expr(_, Some(_)) => false,
                };
                IrStmt::Rust(stmt)
            };
            stmts.push(stmt);
            if input.is_empty() {
                break;
            } else if requires_semicolon {
                return Err(input.error("unexpected token, expected `;`"));
            }
        }
        Ok(stmts)
    }
}

impl Parse for IrBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(IrBlock {
            brace_token: braced!(content in input),
            stmts: content.call(IrBlock::parse_within)?,
        })
    }
}

/*
 * Taken from the syn crate
 */

struct AllowNoSemi(bool);
