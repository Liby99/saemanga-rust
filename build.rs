use std::process::Command;

/// Use `npx webpack` to build front-end components prior to run
fn main() {
  Command::new("npm").args(&["run", "build"]).status().unwrap();
}