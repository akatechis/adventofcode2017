use std::env;

mod common;
mod nine;

fn main() {
  let args: Vec<String> = env::args().collect();
  nine::main(args);
}
