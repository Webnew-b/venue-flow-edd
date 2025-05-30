use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Get)]
pub fn custom_sturct_get(input:TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics,ty_generics,where_generics) = generics.split_for_impl();

    let field_descriptions : proc_macro2::TokenStream = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                syn::Fields::Named(field_named) => {
                    let field_names = field_named
                        .named
                        .iter()
                        .map(|i| 
                            i.ident.as_ref().unwrap()
                        );

                    let field_types = field_named
                        .named
                        .iter()
                        .map(|i| &i.ty);
                    
                    quote! {
                        #(
                            pub fn #field_names(&self) -> &#field_types {
                                &self.#field_names
                            }
                        )*
                    }
                },
                syn::Fields::Unnamed(field_unnamed) => {
                    let return_types_with_lifetime = field_unnamed.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote! { &'a #ty }
                    });

                    let field_ref_access = (0..field_unnamed.unnamed.len()).map(syn::Index::from).map(|i| quote! { &self.#i });
                    
                    quote! {
                        pub fn get_all<'a>(&'a self) -> (#(#return_types_with_lifetime,)*) {
                            (#(#field_ref_access,)*)
                        }
                    }
                },
                syn::Fields::Unit => {
                    return syn::Error::new_spanned(
                        &name, 
                        "Get can only be derived for structs with names,not unit."
                    )
                    .to_compile_error()
                    .into();
                },
            }
        }
        Data::Enum(_) => {
            return syn::Error::new_spanned(
                &name, 
                "Get can only be derived for structs with names,not enum."
            )
            .into_compile_error()
            .into();
        },
        Data::Union(_) => {
            return syn::Error::new_spanned(
                &name, 
                "Get can only be derived for structs with names,not union."
            )
            .to_compile_error()
            .into();
        },
    };

    let output = quote! {
        impl #impl_generics #name #ty_generics #where_generics {
            #field_descriptions
        }
    };

    output.into()
}
