use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};


/*
 * 修改此宏时需注意以下情况：
 * 1. 生命周期：由于生成的函数返回引用，生命周期问题至关重要。
 * 2. 引用类型的字段：当前宏对于本身已经是引用类型的字段（例如 `field: &'a T`）处理尚不理想，
 * 可能会生成返回“引用的引用”（例如 `fn field(&self) -> &&'a T`）的 getter。理想情况下，应直接返回 `&'a T`。
 * 3. 支持范围：目前仅支持带命名字段的结构体和元组结构体。
 * 枚举（Enum）、联合（Union）以及单元结构体（Unit struct）明确不支持（会导致编译错误）。
 * 4. 未来增强：计划改进对 `Rc<T>` 和 `Arc<T>` 类型字段的处理方式
 * （例如，考虑提供克隆 `Rc/Arc` 的选项）。
 */
/*
 * Key considerations when modifying this macro:
 * 1. Lifetimes: These are crucial because the generated functions return by reference.
 * 2. Fields that are references: The macro currently doesn't optimally handle fields
 * that are themselves references (e.g., `field: &'a T`). It might generate getters
 * returning a reference to a reference (e.g., `fn get_field(&self) -> &&'a T`),
 * whereas returning `&'a T` directly would often be more idiomatic.
 * 3. Scope: It only supports structs with named fields and tuple structs.
 * Enums, Unions, and Unit structs are explicitly not supported (and will cause a compile error).
 * 4. Future Enhancements: Plan to improve handling for `Rc<T>` and `Arc<T>` fields
 * (e.g., consider options for cloning them).
 */
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
