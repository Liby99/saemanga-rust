use rocket::response::Redirect;
use rocket::request::Form;

use crate::util::database::Database;
use crate::app::dmk;

#[derive(FromForm)]
pub struct AddMangaFormData {
  dmk_id: String,
}

#[post("/admin/manga/add", data = "<data>")]
pub fn add(conn: Database, data: Form<AddMangaFormData>) -> Redirect {
  // match dmk::fetch_manga_data(&data.dmk_id) {
  //   Some(manga) => {

  //   },
  //   None => Redirect::to("/admin/error")
  // }
  Redirect::to("/admin/error?code=0")
}