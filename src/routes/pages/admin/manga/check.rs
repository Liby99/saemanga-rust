use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;

use crate::util::database::Database;
use crate::app::manga::Manga;

#[derive(FromForm)]
pub struct CheckForm {
  dmk_id: String
}

#[derive(Serialize)]
struct TemplateData {
  cartoonmad_url: String,
  manga: Manga,
}

#[post("/admin/manga/check", data="<data>")]
pub fn check(conn: Database, data: Form<CheckForm>) -> Result<Template, Redirect> {
  match Manga::get_by_dmk_id(&conn, &data.dmk_id) {
    Ok(manga) => Ok(Template::render("admin/check_manga", &TemplateData {
      cartoonmad_url: manga.manga().dmk_base_url(),
      manga: manga,
    })),
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err.code())))
  }
}