use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive_impl(input:TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let data = match &ast.data {
        Data::Struct(_) => {
            return syn::Error::new_spanned(
                &name, 
                "IteralDisplay can only be derived for enums with names,not sturct."
            )
            .into_compile_error()
            .into();
        }
        Data::Enum(data) => {
            data
        },
        Data::Union(_) => {
            return syn::Error::new_spanned(
                &name, 
                "IteralDisplay can only be derived for structs with names,not union."
            )
            .to_compile_error()
            .into();
        },
    };


    let match_arms:Vec<_> = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let event_type_str = variant_name.to_string()
            .chars()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                if i > 0 && c.is_uppercase() {
                    acc.push('_');
                }
                acc.push(c.to_ascii_lowercase());
                acc
            });
        
        match &variant.fields {
            Fields::Named(_) => {
                quote! { Self::#variant_name { .. } => #event_type_str, }
            }
            Fields::Unnamed(_) => {
                quote! { Self::#variant_name(..) => #event_type_str, }
            }
            Fields::Unit => {
                quote! { Self::#variant_name => #event_type_str, }
            }
        }
    }).collect();
    

    let expanded = quote! {
        impl #name {
            pub fn iteral_display(&self) -> &'static str {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    expanded.into()
}
