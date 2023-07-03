mod structable;

use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructTable, attributes(structable))]
/// Derive macro to implementing `VecTable` traits
pub fn openstack_result_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let receiver = structable::TableStructInputReceiver::from_derive_input(&input).unwrap();
    let tokens = quote!(#receiver);
    tokens.into()
}
