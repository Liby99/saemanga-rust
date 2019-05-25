use rocket::Route;
use rocket_contrib::json::Json;

use crate::util::Database;
use crate::app::genre::Genre;
use crate::app::manga::Manga;

pub fn routes() -> Vec<Route> {
  routes![
    get_latest_manga,
  ]
}

#[derive(Serialize)]
struct LatestMangaData {
  title: String,
  dmk_id: String,
  cover_url: String,
  saemanga_url: String,
}

#[derive(Serialize)]
struct GetLatestMangaResult<'a> {
  success: bool,
  message: &'a str,
  data: Vec<LatestMangaData>,
}

#[get("/ajax/get_latest_manga?<genre>")]
fn get_latest_manga<'a>(conn: Database, genre: Option<String>) -> Json<GetLatestMangaResult<'a>> {

  // First verify the genre information
  let genre = match genre {
    Some(id) => match Genre::for_id(id.as_str()) {
      Some(genre) => Some(genre),
      None => {
        return Json(GetLatestMangaResult {
          success: false,
          message: "Unknown genre",
          data: vec![]
        });
      }
    },
    None => None
  };

  // Then get the latest manga from the database and turn it into the data of front-end
  let data = match Manga::get_latest_10(&conn, genre) {
    Ok(mangas) => mangas.into_iter().map(|manga| {
      let data = manga.data();
      LatestMangaData {
        title: data.title().clone(),
        dmk_id: data.dmk_id().clone(),
        cover_url: data.dmk_cover_url(),
        saemanga_url: data.saemanga_url(),
      }
    }).collect::<Vec<_>>(),
    Err(err) => {
      return Json(GetLatestMangaResult {
        success: false,
        message: err.msg(),
        data: vec![]
      })
    }
  };

  // Finally return the success result
  Json(GetLatestMangaResult {
    success: true,
    message: "",
    data: data,
  })
}