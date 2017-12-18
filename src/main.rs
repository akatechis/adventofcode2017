use std::env;

mod common;
mod two;

fn main() {
  let args: Vec<String> = env::args().collect();
  two::main_plus(args);
}
