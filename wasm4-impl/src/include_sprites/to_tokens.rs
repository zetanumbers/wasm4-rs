use super::{parse, Sprite};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use std::path::PathBuf;
use wasm4_common::draw::BitsPerPixel;

pub struct Output {
    pub package_name: syn::Ident,
    pub palette: (
        wasm4_common::draw::Palette,
        parse::ItemConst<parse::CommonPaletteExpr>,
    ),
    pub sprites: Vec<(Sprite, parse::ItemConst<parse::IncludeSpriteExpr>)>,
}

impl ToTokens for Output {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Output {
            package_name,
            palette: (palette, palette_item),
            sprites,
        } = self;

        {
            let parse::ItemConst::<parse::CommonPaletteExpr> {
                attrs,
                vis,
                const_token,
                ident,
                colon_token,
                ty,
                eq_token,
                expr,
                semi_token,
            } = palette_item;

            let ty = quote_spanned! { ty.spans[0] => #package_name::draw::Palette };
            let color_ty = quote! { #package_name::draw::Color };
            let palette = palette.map(|c| c.0);
            let expr = quote_spanned! { expr.name_token.span => [ #( #color_ty(#palette) ),* ] };

            attrs.iter().for_each(|attr| attr.to_tokens(tokens));
            vis.to_tokens(tokens);
            const_token.to_tokens(tokens);
            ident.to_tokens(tokens);
            colon_token.to_tokens(tokens);
            ty.to_tokens(tokens);
            eq_token.to_tokens(tokens);
            expr.to_tokens(tokens);
            semi_token.to_tokens(tokens);
        }

        let pkg_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        for (sprite, sprite_item) in sprites {
            let parse::ItemConst::<parse::IncludeSpriteExpr> {
                attrs,
                vis,
                const_token,
                ident,
                colon_token,
                ty,
                eq_token,
                expr,
                semi_token,
            } = sprite_item;

            let bytes = sprite.bytes();
            let shape = sprite.shape();
            let bpp = match sprite.bpp() {
                BitsPerPixel::One => quote! { #package_name::draw::BitsPerPixel::One },
                BitsPerPixel::Two => quote! { #package_name::draw::BitsPerPixel::Two },
            };
            let indices = sprite.indices.into_u16();
            let byte_len = bytes.len();
            let indices_ty = quote! { #package_name::draw::DrawIndices };
            let ty = quote_spanned! { ty.spans[0] => #package_name::draw::Sprite<[u8; #byte_len]> };
            let path = format!("{}", pkg_root.join(expr.path.value()).display());
            let path = quote_spanned!(expr.path.span() => #path);
            let expr = quote_spanned! { expr.name_token.span =>
                match #package_name::draw::Sprite::from_byte_array(
                    [ #( #bytes ),* ],
                    [ #( #shape ),* ],
                    #bpp,
                    unsafe { <#indices_ty>::from_u16_unchecked(#indices) },
                ) {
                    Some(s) => s,
                    None => panic!(),
                }
            };

            attrs.iter().for_each(|attr| attr.to_tokens(tokens));
            vis.to_tokens(tokens);
            const_token.to_tokens(tokens);
            ident.to_tokens(tokens);
            colon_token.to_tokens(tokens);
            ty.to_tokens(tokens);
            eq_token.to_tokens(tokens);
            expr.to_tokens(tokens);
            semi_token.to_tokens(tokens);
            quote! { const _: &[u8] = include_bytes!(#path); }.to_tokens(tokens);
        }
    }
}
