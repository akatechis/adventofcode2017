use std::env;

mod common;
mod six;

fn main() {
  let args: Vec<String> = env::args().collect();
  six::main(args);
}
