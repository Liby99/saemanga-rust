use rocket::response::Redirect;
use rocket::request::Form;

use crate::util::Database;
use crate::app::dmk;
use crate::app::manga::Manga;
use super::super::AdminUser;

#[derive(FromForm)]
pub struct UpsertMangaFormData {
  dmk_id: String,
}

#[post("/admin/manga/upsert", data="<data>")]
pub fn upsert(_user: AdminUser, conn: Database, data: Form<UpsertMangaFormData>) -> Redirect {
  match dmk::fetch_manga_data(&data.dmk_id).and_then(|manga| Manga::upsert(&conn, &manga)) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => err.redirect_to_admin()
  }
}