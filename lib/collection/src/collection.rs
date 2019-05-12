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
      pub fn coll(conn: &Database) -> mongodb::coll::Collection {
        conn.collection(#name)
      }

      pub fn from_bson(bs: mongodb::Bson) -> Result<Self, Error> {
        match bson::from_bson::<#guard_type>(bs) {
          Ok(s) => Ok(s),
          Err(_) => Err(Error::DeserializeError)
        }
      }

      pub fn from_doc(doc: mongodb::ordered::OrderedDocument) -> Result<Self, Error> {
        Self::from_bson(mongodb::Bson::Document(doc))
      }

      pub fn to_bson(&self) -> Result<mongodb::Bson, Error> {
        match bson::to_bson(&self) {
          Ok(bs) => Ok(bs),
          Err(_) => Err(Error::SerializeError),
        }
      }

      pub fn to_doc(&self) -> Result<mongodb::ordered::OrderedDocument, Error> {
        self.to_bson().and_then(|bs| match bs {
          mongodb::Bson::Document(doc) => Ok(doc),
          _ => Err(Error::SerializeError),
        })
      }

      pub fn get(conn: &Database, filter: Option<mongodb::ordered::OrderedDocument>, option: Option<mongodb::coll::options::FindOptions>) -> Result<Vec<Self>, Error> {
        let coll = Self::coll(&conn);
        let cursor = coll.find(filter, option).map_err(|_| Error::DatabaseError)?;
        Ok(cursor.map(|result| match result {
          Ok(doc) => Self::from_doc(doc),
          Err(_) => Err(Error::DatabaseError)
        }).filter_map(Result::ok).collect::<Vec<_>>())
      }

      pub fn get_all(conn: &Database) -> Result<Vec<Self>, Error> {
        Self::get(conn, None, None)
      }

      pub fn get_one(conn: &Database, filter: Option<mongodb::ordered::OrderedDocument>, option: Option<mongodb::coll::options::FindOptions>) -> Result<Option<Self>, Error> {
        let coll = Self::coll(&conn);
        let option_doc = coll.find_one(filter, option).map_err(|_| Error::DatabaseError)?;
        match option_doc {
          Some(doc) => Ok(Some(Self::from_doc(doc)?)),
          None => Ok(None),
        }
      }
    }
  }.into())
}