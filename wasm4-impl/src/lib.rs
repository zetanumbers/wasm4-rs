mod include_sprites;

use proc_macro::TokenStream;

#[proc_macro]
pub fn include_sprites_impl(item: TokenStream) -> TokenStream {
    syn::parse(item)
        .and_then(include_sprites::implementation)
        .map_or_else(
            syn::Error::into_compile_error,
            quote::ToTokens::into_token_stream,
        )
        .into()
}
