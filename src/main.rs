
use std::env;

mod common;
mod four;

fn main() {
  let args: Vec<String> = env::args().collect();
  four::main(args);
}
