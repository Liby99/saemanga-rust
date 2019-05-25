use rocket_contrib::templates::Template;
use rocket::http::Cookies;

use crate::util::Database;
use crate::app::user::User;
use crate::app::manga::Manga;

#[derive(Serialize)]
struct NeighborEpisodeData {
  is_book: bool,
  url: String,
}

#[derive(Serialize)]
struct EpisodeData {

  // Basic information about the current episode
  episode: i32,
  is_book: bool,
  pages: Vec<String>,

  // Neighbor episodes
  prev: Option<NeighborEpisodeData>,
  next: Option<NeighborEpisodeData>,
}

#[derive(Serialize)]
struct PageData {
  manga: Manga,
  episode: EpisodeData,
}

#[get("/manga/<dmk_id>")]
pub fn manga(
  user: Option<&User>,
  conn: Database,
  cookies: Cookies,
  dmk_id: String
) -> Template {
  Template::render("manga", PageData {
    manga: Manga::get_by_dmk_id(&conn, &dmk_id).unwrap().unwrap(),
    episode: EpisodeData {
      episode: 1,
      is_book: false,
      pages: vec![
        format!("http://www.cartoonmad.com/home75458/6037/011/001.jpg")
      ],
      next: None,
      prev: None,
    }
  })
}