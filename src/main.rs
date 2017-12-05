
use std::env;

mod one;

fn main() {
  let args: Vec<String> = env::args().collect();
  one::main(args);
}
