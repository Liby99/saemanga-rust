use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};
use rocket::fairing::Fairing;
use rocket_contrib::templates::{handlebars, Template};

fn concatenate_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> HelperResult {
  for param in h.params() {
    out.write(&param.value().render())?;
  }
  Ok(())
}

pub fn template() -> impl Fairing {
  Template::custom(|engines| {
    engines
      .handlebars
      .register_helper("concat", Box::new(concatenate_helper));
  })
}
