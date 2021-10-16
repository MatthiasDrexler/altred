extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

// #[proc_macro_derive(FullAutowire)]
// pub fn full_autowiring_derive(input: TokenStream) -> TokenStream {
//      // ...
// }

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
        // impl Default for #name {
        //     fn default() -> Self {
        //         <#name>::new()
        //     }
        // }

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
    fn it_works() {
        let input: proc_macro2::TokenStream = "\
            pub(crate) struct ToAutowire {
                dep: Box<dyn TDep>,
                dep2: Box<dyn TDep2>
            }"
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
