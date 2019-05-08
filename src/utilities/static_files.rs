use rocket_contrib::serve::StaticFiles;

pub fn static_files() -> StaticFiles {
  let public_path = concat!(env!("CARGO_MANIFEST_DIR"), "/public");
  StaticFiles::from(public_path)
}