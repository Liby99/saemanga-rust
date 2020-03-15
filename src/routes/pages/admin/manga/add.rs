use rocket::request::Form;
use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::dmk;
use crate::app::manga::Manga;
use crate::util::Database;

#[derive(FromForm)]
pub struct AddMangaFormData {
  dmk_id: String,
}

#[post("/admin/manga/add", data = "<data>")]
pub fn add(_user: AdminUser, conn: Database, data: Form<AddMangaFormData>) -> Redirect {
  match dmk::fetch_manga_data(&data.dmk_id) {
    Ok(manga) => match Manga::insert(&conn, &manga) {
      Ok(_) => Redirect::to("/admin/index"),
      Err(err) => err.redirect_to_admin(),
    },
    Err(err) => err.redirect_to_admin(),
  }
}
