#![feature(proc_macro_span, proc_macro_diagnostic)]
#![feature(crate_visibility_modifier)]
#![recursion_limit="256"]

extern crate devise;
extern crate proc_macro;

#[allow(unused_imports)]
#[macro_use] extern crate quote;

#[allow(unused_imports)]
crate use devise::{syn, proc_macro2};

mod collection;

#[allow(unused_imports)]
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn collection(attr: TokenStream, input: TokenStream) -> TokenStream {
  collection::collection_attr(attr, input).unwrap_or_else(|diag| {
    diag.emit();
    TokenStream::new()
  })
}
