use saemanga::api::dmk_api;
use std::thread;

#[test]
fn fetch_async() {
  let ids : Vec<&str> = vec!["3628", "5844", "7796"];
  let handles : Vec<thread::JoinHandle<()>> = ids.into_iter().map(|id: &str| {
    thread::spawn(move || {
      let res = dmk_api::fetch_manga_data(&id.to_string());
      match res {
        Ok(manga) => println!("Fetched Manga {} has title {}", id, manga.title()),
        Err(_) => println!("Fetch manga {} failed", id)
      }
    })
  }).collect::<Vec<_>>();
  for handle in handles {
    handle.join().unwrap();
  }
}