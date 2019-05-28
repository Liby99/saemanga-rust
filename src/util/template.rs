use rocket::fairing::Fairing;
use rocket_contrib::templates::{Template, handlebars};
use rocket_contrib::templates::handlebars::Renderable;
use handlebars::{Helper, Handlebars, Context, RenderError, RenderContext, Output, HelperResult, JsonRender};

fn concatenate_helper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut Output) -> HelperResult {
  for param in h.params() {
    out.write(&param.value().render())?;
  }
  Ok(())
}

pub fn template() -> impl Fairing {
  Template::custom(|engines| {
    engines.handlebars.register_helper("concat", Box::new(concatenate_helper));
  })
}