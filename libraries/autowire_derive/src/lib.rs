extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(FullAutowire)]
pub fn full_autowiring_derive(input: TokenStream) -> TokenStream {
    let syntax_tree: DeriveInput = syn::parse(input).unwrap();
    let generated_output = impl_full_autowiring_derive(&syntax_tree);
    generated_output.into()
}

#[cfg(test)]
pub(crate) fn full_autowiring_derive_test_adapter(
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let syntax_tree: DeriveInput = syn::parse2(input).unwrap();
    impl_full_autowiring_derive(&syntax_tree)
}

fn impl_full_autowiring_derive(syntax_tree: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &syntax_tree.ident;
    let visibility = &syntax_tree.vis;
    let fields = match &syntax_tree.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let fields_in_function_signature = fields.iter().map(|field| &field.ident);
    let fields_in_constructor = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);

    quote! {
        impl Default for #name {
            fn default() -> Self {
                #name::new()
            }
        }

        impl #name {
            #visibility fn new() -> Self {
                let mut container = di_container::get::<profiles::Default>();
                Provider::<#name>::create(&mut container)
            }

            #[cfg(test)]
            #visibility fn construct(#(#fields_in_function_signature: #field_type),*) -> Self {
                #name {
                    #(#fields_in_constructor),*
                }
            }
        }
    }
}

#[proc_macro_derive(Autowire)]
pub fn autowiring_derive(input: TokenStream) -> TokenStream {
    let syntax_tree: DeriveInput = syn::parse(input).unwrap();
    let generated_output = impl_autowiring_derive(&syntax_tree);
    generated_output.into()
}

#[cfg(test)]
pub(crate) fn autowiring_derive_test_adapter(
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let syntax_tree: DeriveInput = syn::parse2(input).unwrap();
    impl_autowiring_derive(&syntax_tree)
}

fn impl_autowiring_derive(syntax_tree: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &syntax_tree.ident;
    quote! {
        impl #name {
            pub(crate) fn autowire() -> Self {
                let mut container = di_container::get::<profiles::Default>();
                Provider::<#name>::create(&mut container)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn impl_full_autowiring_derive_should_create_default_constructor() {
        let input: proc_macro2::TokenStream = "\
            pub(crate) struct ToAutowire {
                dep: Box<dyn TDep>,
                dep2: Box<dyn TDep2>
            }"
        .parse()
        .unwrap();
        let expected_output: proc_macro2::TokenStream = "\
            impl Default for ToAutowire {
                fn default() -> Self {
                    ToAutowire :: new()
                }
            }

            impl ToAutowire {
                pub(crate) fn new() -> Self {
                    let mut container = di_container :: get :: < profiles::Default > ();
                    Provider :: < ToAutowire > :: create(&mut container)
                }

                #[cfg(test)]
                pub(crate) fn construct(dep: Box<dyn TDep> , dep2: Box<dyn TDep2>) -> Self {
                    ToAutowire {
                        dep ,
                        dep2
                    }
                }
            }"
        .parse()
        .unwrap();

        let generated_token_stream = full_autowiring_derive_test_adapter(input);

        let normalized_generated_token_stream = generated_token_stream.to_string();
        let normalized_expected_token_stream = expected_output.to_string();
        assert_that(&normalized_generated_token_stream)
            .is_equal_to(String::from(normalized_expected_token_stream));
    }

    #[test]
    fn impl_autowiring_derive_should_create_autowire_function() {
        let input: proc_macro2::TokenStream = "\
            pub(crate) struct ToAutowire {}"
            .parse()
            .unwrap();
        let expected_output: proc_macro2::TokenStream = "\
            impl ToAutowire {
                pub(crate) fn autowire() -> Self {
                    let mut container = di_container :: get :: < profiles::Default > ();
                    Provider :: < ToAutowire > :: create(&mut container)
                }
            }"
        .parse()
        .unwrap();

        let generated_token_stream = autowiring_derive_test_adapter(input);

        let normalized_generated_token_stream = generated_token_stream.to_string();
        let normalized_expected_token_stream = expected_output.to_string();
        assert_that(&normalized_generated_token_stream)
            .is_equal_to(String::from(normalized_expected_token_stream));
    }
}
