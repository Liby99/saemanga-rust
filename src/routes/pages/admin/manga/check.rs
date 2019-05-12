use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;

use crate::util::database::Database;
use crate::app::manga_wrapper::MangaWrapper;

#[derive(FromForm)]
pub struct CheckForm {
  dmk_id: String
}

#[derive(Serialize)]
struct TemplateData {
  cartoonmad_url: String,
  manga: MangaWrapper,
}

#[post("/admin/manga/check", data="<data>")]
pub fn check(conn: Database, data: Form<CheckForm>) -> Result<Template, Redirect> {
  match MangaWrapper::get_by_dmk_id(&conn, &data.dmk_id) {
    Ok(wrapper) => Ok(Template::render("admin/check_manga", &TemplateData {
      cartoonmad_url: wrapper.manga().dmk_base_url(),
      manga: wrapper,
    })),
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err.code())))
  }
}