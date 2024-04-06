use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated};

#[derive(Clone)]
pub struct Input {
    pub package: PackageField,
    pub input: InputField,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Input {
            package: input.parse()?,
            input: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct PackageField {
    pub _package_token: kw::package,
    pub _column: syn::Token![:],
    pub package_name: syn::Ident,
    pub _comma: syn::Token![,],
}

impl Parse for PackageField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(PackageField {
            _package_token: input.parse()?,
            _column: input.parse()?,
            package_name: input.parse()?,
            _comma: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct InputField {
    pub _input_token: kw::input,
    pub _column: syn::Token![:],
    pub _brace: syn::token::Brace,
    pub consts: Vec<ItemConst>,
    pub _comma: syn::Token![,],
}

impl Parse for InputField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(InputField {
            _input_token: input.parse()?,
            _column: input.parse()?,
            _brace: syn::braced!(content in input),
            consts: {
                let mut consts = Vec::new();
                while !content.is_empty() {
                    consts.push(content.parse()?);
                }
                consts
            },
            _comma: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct ItemConst<E = Expr> {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub const_token: syn::token::Const,
    pub ident: syn::Ident,
    pub colon_token: syn::token::Colon,
    pub ty: syn::token::Underscore,
    pub eq_token: syn::token::Eq,
    pub expr: E,
    pub semi_token: syn::token::Semi,
}

impl<E> ItemConst<E> {
    pub fn map_expr<F, U>(self, f: F) -> ItemConst<U>
    where
        F: FnOnce(E) -> U,
    {
        let ItemConst {
            attrs,
            vis,
            const_token,
            ident,
            colon_token,
            ty,
            eq_token,
            expr,
            semi_token,
        } = self;
        ItemConst {
            attrs,
            vis,
            const_token,
            ident,
            colon_token,
            ty,
            eq_token,
            expr: f(expr),
            semi_token,
        }
    }
}

impl<Expr: Parse> Parse for ItemConst<Expr> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ItemConst {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            const_token: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            eq_token: input.parse()?,
            expr: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

#[derive(Clone)]
pub enum Expr {
    CommonPalette(CommonPaletteExpr),
    IncludeSprite(IncludeSpriteExpr),
}

impl Parse for Expr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::common_palette) {
            Ok(Expr::CommonPalette(input.parse()?))
        } else if lookahead.peek(kw::include_sprite) {
            Ok(Expr::IncludeSprite(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone)]
pub struct CommonPaletteExpr {
    pub name_token: kw::common_palette,
    pub _bang_token: syn::token::Bang,
    pub _paren_token: syn::token::Paren,
    pub colors: Punctuated<Rgb, syn::token::Comma>,
}

impl Parse for CommonPaletteExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(CommonPaletteExpr {
            name_token: input.parse()?,
            _bang_token: input.parse()?,
            _paren_token: syn::parenthesized!(content in input),
            colors: {
                let mut out = Punctuated::new();
                loop {
                    if content.is_empty() {
                        break;
                    }

                    if out.len() > 4 {
                        return Err(syn::Error::new(
                            content.span(),
                            "amount of specified palette colors should not be greater than 4",
                        ));
                    }

                    out.push_value(content.parse()?);
                    if let Some(comma) = content.parse()? {
                        out.push_punct(comma);
                    } else {
                        break;
                    }
                }
                content.parse::<syn::parse::Nothing>()?;
                out
            },
        })
    }
}

#[derive(Clone)]
pub struct IncludeSpriteExpr {
    pub name_token: kw::include_sprite,
    pub bang_token: syn::Token![!],
    pub paren_token: syn::token::Paren,
    pub path: syn::LitStr,
}

impl Parse for IncludeSpriteExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(IncludeSpriteExpr {
            name_token: input.parse()?,
            bang_token: input.parse()?,
            paren_token: {
                let out = syn::parenthesized!(content in input);
                out
            },
            path: {
                let out = content.parse()?;
                content.parse::<syn::parse::Nothing>()?;
                out
            },
        })
    }
}

impl ToTokens for IncludeSpriteExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name_token.to_tokens(tokens);
        self.bang_token.to_tokens(tokens);
        self.paren_token
            .surround(tokens, |tokens| self.path.to_tokens(tokens));
    }
}

#[derive(Copy, Clone)]
pub struct Rgb {
    pub value: u32,
}

impl Parse for Rgb {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit: syn::LitInt = input.parse()?;
        let span = lit.span();
        if !matches!(lit.suffix(), "" | "u32") {
            return Err(syn::Error::new(
                span,
                "wrong suffix (should be u32 or nothing)",
            ));
        }
        let value = lit.base10_parse()?;
        Ok(Rgb { value })
    }
}

pub mod kw {
    syn::custom_keyword!(package);
    syn::custom_keyword!(input);
    syn::custom_keyword!(common_palette);
    syn::custom_keyword!(include_sprite);
    syn::custom_keyword!(bg);
}
