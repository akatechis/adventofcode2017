
use common::read_file_lines;
use std::collections::HashSet;

fn passphrase_valid(passphrase: &str) -> bool {
  let words: Vec<&str> = passphrase.split(char::is_whitespace).collect();
  let mut seen = HashSet::new();

  for word in words {
    if seen.contains(word) {
      return false
    }
    seen.insert(word);
  }

  true
}

pub fn main(args: Vec<String>) {
  let contents = read_file_lines(&args[1]);
  let count = contents.iter().filter(|p| passphrase_valid(p)).count();
  print!("{:?}", count);
}

#[test]
fn it_correctly_validates_passphrases() {
  assert_eq!( true  , passphrase_valid("aa bb cc dd ee") );
  assert_eq!( true  , passphrase_valid("aa bb cc dd ee aaa") );
  assert_eq!( true  , passphrase_valid("gggggggggggg") );
  assert_eq!( false  , passphrase_valid("aaa aaa") );
  assert_eq!( false , passphrase_valid("aa bb cc dd ee aa") );
}
