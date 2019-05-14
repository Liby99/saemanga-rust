use std::thread;

use crate::util::Database;
use crate::util::error::Error;
use super::manga_data::MangaData;
use super::manga::Manga;
use super::genre::Genre;
use super::dmk;

pub struct Latest;

type FetchGenreHandle = thread::JoinHandle<Result<Vec<MangaData>, Error>>;

type FetchMangaHandle = thread::JoinHandle<Result<MangaData, Error>>;

impl Latest {
  pub fn fetch_latest(conn: &Database, genres: Vec<&'static Genre>) -> Result<Vec<Manga>, Error> {

    // Ver.3 Double Layer Parallel
    let genre_handles : Vec<FetchGenreHandle> = genres.into_iter().map(|genre| {
      thread::spawn(move || -> Result<Vec<MangaData>, Error> {
        let ids = dmk::fetch_latest_manga_with_genre(genre)?;
        let manga_handles : Vec<FetchMangaHandle> = ids.into_iter().map(|id| {
          thread::spawn(move || dmk::fetch_manga_data(&id))
        }).collect();
        Ok(manga_handles.into_iter().filter_map(|handle| {
          handle.join().ok().and_then(|res| match res {
            Ok(manga) => Some(manga),
            Err(err) => { println!("Error {}: {}", err.code(), err.msg()); None }
          })
        }).collect())
      })
    }).collect();

    let genre_mangas : Vec<Manga> = genre_handles.into_iter().filter_map(|handle| -> Option<Vec<Manga>> {
      handle.join().ok().and_then(|res| match res {
        Ok(genre_mangas) => Some(genre_mangas.into_iter().filter_map(|data: MangaData| -> Option<Manga> {
          // println!("Upserting manga {}: {}", data.dmk_id(), data.title());
          Manga::upsert(conn, &data).ok()
        }).collect()),
        Err(err) => { println!("Error {}: {}", err.code(), err.msg()); None }
      })
    }).flatten().collect();

    Ok(genre_mangas)

    // Ver.1, Async on fetching part
    // let latest_manga_ids : Vec<String> = dmk::fetch_latest_manga()?;
    // let _ = Self::insert(conn, genre, &latest_manga_ids)?;
    // let handles : Vec<FetchMangaHandle> = latest_manga_ids.into_iter().map(|id| {
    //   thread::spawn(move || dmk::fetch_manga_data(&id))
    // }).collect();
    // let manga_datas : Vec<MangaData> = handles.into_iter().filter_map(|handle| {
    //   handle.join().ok().and_then(|res| res.ok())
    // }).collect();
    // let mangas : Vec<Manga> = manga_datas.into_iter().filter_map(|data| {
    //   Manga::upsert(conn, &data).ok()
    // }).collect();
    // Ok(mangas)

    // Ver.2, Non-async way
    // let latest_manga_ids : Vec<String> = dmk::fetch_latest_manga()?;
    // let _ = Self::insert(conn, genre, &latest_manga_ids)?;
    // let mangas : Vec<Manga> = latest_manga_ids.into_iter().filter_map(|id| {
    //   let data = dmk::fetch_manga_data(&id).ok()?;
    //   Manga::upsert(conn, &data).ok()
    // }).collect();
    // Ok(mangas)
  }

  pub fn fetch_all_genres(conn: &Database) -> Result<Vec<Manga>, Error> {
    Self::fetch_latest(conn, Genre::all_genres())
  }

  pub fn fetch_overall(conn: &Database) -> Result<Vec<Manga>, Error> {
    Self::fetch_latest(conn, vec![Genre::all()])
  }
}