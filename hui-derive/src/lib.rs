extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Implements `Signal` trait for the given type
#[proc_macro_derive(Signal)]
pub fn derive_signal(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;
  quote!(impl ::hui::signal::Signal for #name {}).into()
}

/// Implements `State` trait for the given type
#[proc_macro_derive(State)]
pub fn derive_state(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;
  quote!(impl ::hui::state::State for #name {}).into()
}
