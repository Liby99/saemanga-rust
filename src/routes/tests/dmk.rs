use rocket::Route;
use rocket_contrib::templates::Template;
use lazy_static::lazy_static;
use scraper::{Selector, Html};

pub fn routes() -> Vec<Route> {
  routes![
    fetch_google,
    fetch_dmk_home,
    foster_parenting,
  ]
}

#[derive(Serialize)]
struct FetchGoogleTemplateData<'a> {
  text: &'a str
}

#[get("/tests/dmk/fetch_google")]
fn fetch_google() -> Template {

  // First fetch google and get html text string
  let url = "https://google.com";
  let mut response = reqwest::get(url).unwrap();
  let html_text = response.text().unwrap();

  // Then parse it to dom tree
  let document = Html::parse_document(&html_text);

  // Use selector to get some element
  lazy_static! {
    static ref BODY_SEL : Selector = Selector::parse("body").unwrap();
  }
  let body = document.select(&BODY_SEL).next().unwrap();

  lazy_static! {
    static ref LOGO_SEL : Selector = Selector::parse("#hplogo").unwrap();
  }
  let logo = body.select(&LOGO_SEL).next().unwrap();
  let logo_src = logo.value().attr("src").unwrap();

  // Render the template
  Template::render("tests/dmk/fetch_google", FetchGoogleTemplateData {
    text: logo_src
  })
}

#[derive(Serialize)]
struct FetchDmkHomeTemplateData<'a> {
  text: &'a String
}

#[get("/tests/dmk/fetch_dmk_home")]
fn fetch_dmk_home() -> Template {
  let url = "https://cartoonmad.com";
  let mut response = reqwest::get(url).unwrap();
  let html_text = response.text_with_charset("big5").unwrap();

  let complete_html_text = format!("<!DOCTYPE html>{}", html_text);

  // Then parse it to dom tree
  let _document = Html::parse_document(&complete_html_text);

  // Render the template
  Template::render("tests/dmk/fetch_dmk_home", FetchDmkHomeTemplateData {
    text: &complete_html_text
  })
}

#[get("/tests/dmk/foster_parenting")]
fn foster_parenting() -> Template {
  let html_text = "
    <table>
      <form>
        <tr></tr>
      </form>
    </table>
  ";

  let complete_html_text = format!("<!DOCTYPE html>{}", html_text);

  // Then parse it to dom tree
  let _document = Html::parse_document(&complete_html_text);

  // Render the template
  Template::render("tests/dmk/foster_parenting", ())
}