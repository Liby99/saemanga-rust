#![feature(proc_macro_span, proc_macro_diagnostic)]
#![feature(crate_visibility_modifier)]
#![recursion_limit="256"]

#[macro_use] extern crate quote;

extern crate proc_macro2;
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

  // First get the name of the enum and generate a snake case key for it
  let name = &ast.ident;
  let cookie_key = name.to_string().to_snake_case();

  // Then get the content of the enum while checking that it has to be an enum
  let data = &ast.data;
  let enum_data = match data {
    syn::Data::Enum(d) => d,
    _ => panic!("Data being derived from must be an enum")
  };

  // Get the variants of the enum
  let variants = enum_data.variants.iter().map(|v: &syn::Variant| {
    match &v.fields {
      syn::Fields::Unit => (),
      _ => panic!("Enum variants have to be unit")
    };
    (v.ident.clone(), v.ident.to_string().to_snake_case())
  }).collect::<Vec<_>>();

  // Check if there's at least one
  if variants.is_empty() {
    panic!("There has to be at least one enum variant");
  }

  // Get the default(first) element
  let default_variant = variants.first().unwrap().0.clone();

  // Get the cases used in from_str's match
  let mut from_str_case_tokens = proc_macro2::TokenStream::new();
  variants.iter().for_each(|(ident, snaked)| {
    from_str_case_tokens.extend(quote! { #snaked => Ok(Self::#ident), })
  });

  // Get the cases used in to_str's match
  let mut to_string_case_tokens = proc_macro2::TokenStream::new();
  variants.iter().for_each(|(ident, snaked)| {
    to_string_case_tokens.extend(quote! { Self::#ident => #snaked.to_string(), })
  });

  let gen = quote! {
    impl CookieValue for #name {
      type Data = Self;

      const KEY : &'static str = #cookie_key;

      fn from_str(s: &str) -> Result<Self::Data, Error> {
        match s {
          #from_str_case_tokens
          _ => Err(Error::UnknownHandMode),
        }
      }

      fn to_string(&self) -> String {
        match self {
          #to_string_case_tokens
        }
      }

      fn default() -> Self {
        Self::#default_variant
      }

      fn from_cookies(cookies: &Cookies) -> Self::Data {
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
