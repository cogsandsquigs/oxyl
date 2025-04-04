use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NodeType)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl crate::ast::NodeType for #ident {}
    };
    output.into()
}
