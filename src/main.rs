use std::env;

mod common;
mod five;

fn main() {
  let args: Vec<String> = env::args().collect();
  five::main(args);
}
