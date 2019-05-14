use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;

use crate::util::database::Database;
use crate::util::Error;
use crate::app::manga::Manga;
use super::super::AdminUser;

#[derive(FromForm)]
pub struct CheckForm {
  dmk_id: String
}

#[derive(Serialize)]
struct TemplateData {
  cartoonmad_url: String,
  add_date_time: String,
  update_date_time: String,
  refresh_date_time: String,
  manga: Manga,
}

#[post("/admin/manga/check", data="<data>")]
pub fn check(_user: AdminUser, conn: Database, data: Form<CheckForm>) -> Result<Template, Redirect> {
  match Manga::get_by_dmk_id(&conn, &data.dmk_id).and_then(|maybe_manga| maybe_manga.ok_or(Error::MangaNotFoundError)) {
    Ok(manga) => Ok(Template::render("admin/check_manga", &TemplateData {
      cartoonmad_url: manga.data().dmk_base_url(),
      add_date_time: manga.add_date_time().to_rfc3339(),
      update_date_time: manga.update_date_time().to_rfc3339(),
      refresh_date_time: manga.refresh_date_time().to_rfc3339(),
      manga: manga,
    })),
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err.code())))
  }
}

#[post("/admin/manga/check", data="<_data>", rank=2)]
pub fn check_fail(_data: Form<CheckForm>) -> Redirect {
  Redirect::to("/admin/login")
}