use rocket::response::Redirect;
use rocket::request::Form;

use crate::util::database::Database;
use crate::app::error::Error;
use crate::app::dmk;

#[derive(FromForm)]
pub struct AddMangaFormData {
  dmk_id: String,
}

#[post("/admin/manga/add", data="<data>")]
pub fn add(conn: Database, data: Form<AddMangaFormData>) -> Redirect {
  match dmk::fetch_manga_data(&data.dmk_id) {
    Ok(manga) => {
      Redirect::to(format!("/admin/error?code={}", Error::NotImplementedError.code()))
    },
    Err(err) => Redirect::to(format!("/admin/error?code={}", err.code()))
  }
}