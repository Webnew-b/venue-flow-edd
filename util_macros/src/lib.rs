use proc_macro::TokenStream;

mod iteral_display;
mod get;


#[proc_macro_derive(Get)]
pub fn derive_get(input: TokenStream) -> TokenStream {
    get::derive_impl(input)
}


#[proc_macro_derive(IteralDisplay)]
pub fn derive_iteral_display(input: TokenStream) -> TokenStream {
    iteral_display::derive_impl(input)
}
