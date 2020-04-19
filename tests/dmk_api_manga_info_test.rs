use saemanga::app::dmk::*;
use saemanga::app::manga_data::*;

#[test]
fn test_fetch_manga_5893() {
  let dmk_id = String::from("5893");
  match fetch_manga_data(&dmk_id) {
    Ok(manga) => {
      assert_eq!(
        manga.genre().name(),
        "校園",
        "Gotoubun no hanayome should have genre gakuen"
      );
      assert_eq!(
        manga.first_episode().episode(),
        0,
        "Gotoubun no hanayome should start with episode 0"
      );
    }
    Err(err) => assert!(false, format!("Fetch manga failed: {:?}", err)),
  };
}

/// Deprecated warning: DMK no longer stores ended manga
#[test]
#[ignore]
fn test_fetch_manga_kaikan_shirei() {
  let dmk_id = String::from("1237");
  match fetch_manga_data(&dmk_id) {
    Ok(manga) => {
      assert_eq!(
        manga.status(),
        &MangaStatus::Ended,
        "Manga 1237 should be ended"
      );
      assert_eq!(manga.dmk_id(), "1237", "Manga 1237 should have dmk_id 1237");
      assert_eq!(
        manga.latest_episode().episode(),
        17,
        "Manga 1237 Lastest episode should be 17"
      );
    }
    Err(err) => assert!(false, format!("Fetch manga failed: {:?}", err)),
  };
}

#[test]
fn test_fetch_manga_shingeki_no_kyoujin() {
  let dmk_id = String::from("1221");
  match fetch_manga_data(&dmk_id) {
    Ok(manga) => {
      assert_eq!(
        manga.first_episode().episode(),
        0,
        "Shingeki No Kyoujin starts with episode 0"
      );
    }
    Err(err) => assert!(false, format!("Fetch manga failed: {:?}", err)),
  };
}

#[test]
fn test_fetch_manga_one_piece() {
  let dmk_id = String::from("1152");
  match fetch_manga_data(&dmk_id) {
    Ok(manga) => {
      assert!(
        manga.latest_episode().episode() > 900,
        "One piece should have at least 900 episodes"
      );
    }
    Err(err) => assert!(false, format!("Fetch manga failed: {:?}", err)),
  };
}
