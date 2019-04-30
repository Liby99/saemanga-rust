use rocket_contrib::templates::{Template};

use crate::app::genre::*;

#[derive(Serialize)]
struct TemplateData {
  logged_in: bool,
  latests: Vec<MangaData>,
  user: Option<UserData>,
  genres: &'static [&'static Genre; 14],
}

#[derive(Serialize)]
struct UserData {
  username: String,
  has_follows: bool,
  follows: Vec<FollowData>,
}

#[derive(Serialize)]
struct FollowData {
  liked: bool,
  has_update: bool,
  max_read_episode: u32,
  manga: MangaData,
}

#[derive(Serialize)]
struct MangaData {
  title: String,
  dmk_id: String,
  cover_url: String,
  saemanga_url: String,
  last_episode: u32,
  last_episode_is_book: bool,
  ended: bool,
}

#[get("/index")]
pub fn index() -> Template {

  // Create temporary data
  let data = TemplateData {
    logged_in: true,
    latests: vec![
      MangaData {
        title: String::from("可憐可愛元氣君"),
        dmk_id: String::from("8193"),
        cover_url: String::from("http://cartoonmad.com/cartoonimg/coimg/8193.jpg"),
        saemanga_url: String::from("http://saemanga.com/manga/8193"),
        last_episode: 7,
        last_episode_is_book: false,
        ended: false,
      },
      MangaData {
        title: String::from("家庭教師"),
        dmk_id: String::from("1254"),
        cover_url: String::from("http://cartoonmad.com/cartoonimg/coimg/1254.jpg"),
        saemanga_url: String::from("http://saemanga.com/manga/1254"),
        last_episode: 409,
        last_episode_is_book: false,
        ended: true,
      },
    ],
    user: Some(UserData {
      username: String::from("Liby99"),
      has_follows: true,
      follows: vec![
        FollowData {
          liked: true,
          has_update: false,
          max_read_episode: 408,
          manga: MangaData {
            title: String::from("家庭教師"),
            dmk_id: String::from("1254"),
            cover_url: String::from("http://cartoonmad.com/cartoonimg/coimg/1254.jpg"),
            saemanga_url: String::from("http://saemanga.com/manga/1254"),
            last_episode: 409,
            last_episode_is_book: false,
            ended: true
          },
        },
      ],
    }),
    genres: &ALL_GENRES,
  };

  // Render the data
  Template::render("index", &data)
}