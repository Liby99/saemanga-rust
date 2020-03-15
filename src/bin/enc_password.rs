use std::env;

use saemanga::app::user::encrypt_password;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    println!("Expecting argument: the password you want to encode");
  } else {
    let password = &args[1];
    println!("{}", encrypt_password(password));
  }
}
