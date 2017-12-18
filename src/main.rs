use std::env;

mod common;
mod seven;

fn main() {
  let args: Vec<String> = env::args().collect();
  seven::main_plus(args);
}
