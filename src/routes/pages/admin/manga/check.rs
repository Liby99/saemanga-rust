use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::util::database::Database;
use crate::util::Error;
use crate::app::manga::Manga;
use super::super::AdminUser;

#[derive(Serialize)]
struct TemplateData {
  cartoonmad_url: String,
  add_date_time: String,
  update_date_time: String,
  refresh_date_time: String,
  manga: Manga,
}

#[get("/admin/manga/check?<dmk_id>")]
pub fn check(_user: AdminUser, conn: Database, dmk_id: String) -> Result<Template, Redirect> {
  match Manga::get_by_dmk_id(&conn, &dmk_id).and_then(|maybe_manga| maybe_manga.ok_or(Error::MangaNotFoundError)) {
    Ok(manga) => Ok(Template::render("admin/manga/check", &TemplateData {
      cartoonmad_url: manga.data().dmk_base_url(),
      add_date_time: manga.add_date_time().to_rfc3339(),
      update_date_time: manga.update_date_time().to_rfc3339(),
      refresh_date_time: manga.refresh_date_time().to_rfc3339(),
      manga: manga,
    })),
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err.code())))
  }
}

#[allow(non_snake_case)]
#[get("/admin/manga/check?<_id>", rank=2)]
pub fn check_fail(_id: String) -> Redirect {
  Redirect::to("/admin/login")
}