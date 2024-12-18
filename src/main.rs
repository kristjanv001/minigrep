use std::env;
use minigrep::Config;

use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem with arguments: {err}");
    process::exit(1);
  });

  if let Err(e) = minigrep::run(config) {
    println!("App error: {e}");
    process::exit(1);
  }
}
