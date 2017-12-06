
use std::env;

// mod one;
mod two;

fn main() {
  let args: Vec<String> = env::args().collect();
  // one::main(args);
  two::main(args);
}
