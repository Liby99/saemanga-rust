use rocket::response::Redirect;
use rocket::Route;
use rocket_contrib::templates::Template;

use crate::util::Database;

use crate::app::follow::{AggregateFollow, Follow};
use crate::app::genre::Genre;
use crate::app::manga::Manga;
use crate::app::user::User;
use crate::app::user_setting::*;

pub fn routes() -> Vec<Route> {
  routes![index, old_index,]
}

#[derive(Serialize)]
struct TemplateData {
  latests: Vec<MangaData>,
  user: Option<UserData>,
  genres: Vec<GenreData>,
  setting: SettingData,
}

#[derive(Serialize)]
struct SettingData {
  is_left_hand_mode: bool,
  is_night_mode: bool,
  is_liked_only: bool,
  scale_percentage: f32,
  main_width: f32,
}

impl From<UserSetting> for SettingData {
  fn from(setting: UserSetting) -> Self {
    let s: f32 = setting.scale.get();
    Self {
      is_left_hand_mode: setting.hand_mode == HandMode::Left,
      is_night_mode: setting.light_mode == LightMode::Night,
      is_liked_only: setting.liked_only_mode == LikedOnlyMode::On,
      scale_percentage: (s * 100.0).round(),
      main_width: 768.0 * s,
    }
  }
}

#[derive(Serialize)]
struct UserData {
  display_name: String,
  follows: Vec<FollowData>,
}

#[derive(Serialize)]
struct FollowData {
  is_liked: bool,
  has_update: bool,
  is_up_to_date: bool,
  max_read_episode: i32,
  manga: MangaData,
}

#[derive(Serialize)]
struct MangaData {
  title: String,
  dmk_id: String,
  cover_url: String,
  saemanga_url: String,
  last_episode: i32,
  last_episode_is_book: bool,
  ended: bool,
}

#[derive(Serialize)]
struct GenreData {
  id: &'static str,
  name: &'static str,
}

impl From<&&Genre> for GenreData {
  fn from(genre: &&Genre) -> Self {
    Self {
      id: genre.id,
      name: genre.name,
    }
  }
}

#[get("/index")]
pub fn index(
  user: Option<&User>,
  conn: Database,
  setting: UserSetting,
) -> Result<Template, Redirect> {
  // Create temporary data
  let data = TemplateData {
    latests: Manga::get_latest_10(&conn, None)
      .unwrap()
      .into_iter()
      .map(|m| {
        let data = m.data();
        MangaData {
          title: data.title().clone(),
          dmk_id: data.dmk_id().clone(),
          cover_url: data.dmk_cover_url(),
          saemanga_url: data.saemanga_url(),
          last_episode: data.latest_episode().episode(),
          last_episode_is_book: data.latest_episode().is_book(),
          ended: data.ended(),
        }
      })
      .collect(),
    user: match user {
      Some(user) => Some(UserData {
        display_name: user.display_name().clone(),
        follows: Follow::get_by_user(&conn, &user)
          .map_err(|err| err.redirect(None))?
          .into_iter()
          .map(|agg_follow| {
            let AggregateFollow { follow, manga } = agg_follow;
            let data = manga.data();
            let latest_episode = data.latest_episode();
            FollowData {
              is_liked: follow.is_liked(),
              has_update: follow.max_episode() < data.latest_episode().episode(),
              is_up_to_date: follow.is_up_to_date(),
              max_read_episode: follow.max_episode(),
              manga: MangaData {
                title: data.title().clone(),
                dmk_id: data.dmk_id().clone(),
                cover_url: data.dmk_cover_url(),
                saemanga_url: data.saemanga_url(),
                last_episode: latest_episode.episode(),
                last_episode_is_book: latest_episode.is_book(),
                ended: data.ended(),
              },
            }
          })
          .collect::<Vec<_>>(),
      }),
      None => None,
    },
    genres: Genre::all_genres().iter().map(GenreData::from).collect(),
    setting: SettingData::from(setting),
  };

  // Render the data
  Ok(Template::render("index", &data))
}

#[get("/index.html")]
pub fn old_index() -> Redirect {
  Redirect::to("/index")
}
