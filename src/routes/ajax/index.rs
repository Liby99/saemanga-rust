use rocket::Route;
use rocket_contrib::json::Json;

use crate::util::Database;
use crate::app::genre::Genre;
use crate::app::manga::Manga;
use crate::app::manga_data::{dmk_cover_url, saemanga_url};
use crate::app::dmk;

pub fn routes() -> Vec<Route> {
  routes![
    get_latest_manga,
    search,
  ]
}

#[derive(Clone, Serialize)]
struct LatestMangaData {
  title: String,
  dmk_id: String,
  cover_url: String,
  saemanga_url: String,
}

#[derive(Serialize)]
struct GetMangaResult<'a> {
  success: bool,
  message: &'a str,
  data: Vec<LatestMangaData>,
}

#[get("/ajax/get_latest_manga?<genre>")]
fn get_latest_manga<'a>(conn: Database, genre: Option<String>) -> Json<GetMangaResult<'a>> {

  // First verify the genre information
  let genre = match genre {
    Some(id) => match Genre::for_id(id.as_str()) {
      Some(genre) => Some(genre),
      None => {
        return Json(GetMangaResult {
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
      return Json(GetMangaResult {
        success: false,
        message: err.msg(),
        data: vec![]
      })
    }
  };

  // Finally return the success result
  Json(GetMangaResult {
    success: true,
    message: "",
    data: data,
  })
}

#[get("/ajax/search?<text>")]
fn search<'a>(conn: Database, text: String) -> Json<GetMangaResult<'a>> {

  // First check the text validity
  if text.len() == 0 {
    return Json(GetMangaResult {
      success: false,
      message: "Search text cannot be empty",
      data: vec![],
    });
  }

  let db_result = match Manga::search(&conn, &text) {
    Ok(result) => result.into_iter().map(|manga| {
      let data = manga.data();
      LatestMangaData {
        title: data.title().clone(),
        dmk_id: data.dmk_id().clone(),
        cover_url: data.dmk_cover_url(),
        saemanga_url: data.saemanga_url(),
      }
    }).collect::<Vec<_>>(),
    Err(err) => {
      return Json(GetMangaResult {
        success: false,
        message: err.msg(),
        data: vec![]
      })
    }
  };

  let dmk_result = match dmk::search(&text) {
    Ok(result) => result.into_iter().filter_map(|(dmk_id, title)| {
      match db_result.iter().find(|&data| data.dmk_id == dmk_id) {
        Some(_) => None,
        None => Some(LatestMangaData {
          title: title.clone(),
          dmk_id: dmk_id.clone(),
          cover_url: dmk_cover_url(&dmk_id),
          saemanga_url: saemanga_url(&dmk_id),
        })
      }
    }).collect::<Vec<_>>(),
    Err(err) => {
      return Json(GetMangaResult {
        success: false,
        message: err.msg(),
        data: vec![]
      })
    }
  };

  Json(GetMangaResult {
    success: true,
    message: "",
    data: [db_result, dmk_result].concat(),
  })
}