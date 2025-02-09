extern crate proc_macro;

use darling::FromDeriveInput;
use derive_lerpable::FieldTokensLerpable;
use parser::{GenFinal, LivecodeReceiver};
use proc_macro::TokenStream;

mod derive_lerpable;
mod parser;

#[proc_macro_derive(Lerpable, attributes(lerpable))]
pub fn murrelet_livecode_derive_lerpable(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ast_receiver = LivecodeReceiver::from_derive_input(&ast).unwrap();
    FieldTokensLerpable::from_ast(ast_receiver).into()
}
