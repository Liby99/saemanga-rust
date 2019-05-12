use proc_macro::TokenStream;
use devise::{Spanned, Result};
use crate::syn::{LitStr, DeriveInput};

#[allow(non_snake_case)]
pub fn collection_attr(attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
  let attr_stream2 = crate::proc_macro2::TokenStream::from(attr);
  let attr_span = attr_stream2.span();
  let string_lit = crate::syn::parse2::<LitStr>(attr_stream2).map_err(|_| attr_span.error("expected string literal"))?;

  let input : devise::syn::DeriveInput = crate::syn::parse::<DeriveInput>(input).unwrap();

  let name = string_lit.value();
  let guard_type = &input.ident;

  Ok(quote! {
    #input

    impl #guard_type {
      pub fn coll(conn: &mongodb::db::Database) -> mongodb::coll::Collection {
        conn.collection(#name)
      }
    }
  }.into())
}