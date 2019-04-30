use std::process::Command;

/// Use `npx webpack` to build front-end components prior to run
fn main() {
  Command::new("npx").args(&["webpack"]).status().unwrap();
}