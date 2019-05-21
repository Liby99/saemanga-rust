#![feature(proc_macro_span, proc_macro_diagnostic)]
#![feature(crate_visibility_modifier)]
#![recursion_limit="256"]

#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate syn;
extern crate heck;

use proc_macro::TokenStream;
use heck::SnakeCase;

#[proc_macro_derive(CookieValue)]
pub fn cookie_value_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_cookie_value(&ast)
}

fn impl_cookie_value(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let cookie_key = name.to_string().to_snake_case();
  let gen = quote! {
    impl CookieValue for #name {
      const KEY : &'static str = #cookie_key;

      fn from_str(s: &str) -> Result<Self, Error> {
        match s {
          "left" => Ok(Self::Left),
          "right" => Ok(Self::Right),
          _ => Err(Error::UnknownHandMode),
        }
      }

      fn to_string(&self) -> String {
        match self {
          Self::Left => "left".to_string(),
          Self::Right => "right".to_string(),
        }
      }

      fn default() -> Self {
        Self::Right
      }

      fn from_cookies(cookies: &Cookies) -> Self {
        cookies.get(Self::KEY).map_or(Self::default(), |cookie| {
          Self::from_str(cookie.value()).map_or_else(|_| Self::default(), |v| v)
        })
      }

      fn into_cookies(&self, cookies: &mut Cookies) {
        let value = self.to_string();
        cookies.add(Cookie::build(Self::KEY, value).path("/").finish());
      }
    }
  };
  gen.into()
}
