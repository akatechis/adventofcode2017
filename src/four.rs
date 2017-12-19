
use common::read_file_lines;
use std::collections::HashSet;

fn base_validation(passphrase: &str, map: fn(&str) -> String) -> bool {
  let words: Vec<&str> = passphrase.split(char::is_whitespace).collect();
  let mut seen = HashSet::new();

  for word in words {
    let key = map(word);
    if seen.contains(&key) {
      return false;
    }
    seen.insert(key);
  }

  true
}

fn passphrase_valid(passphrase: &str) -> bool {
  base_validation(passphrase, |word| String::from(word))
}

fn passphrase_valid_anagram(passphrase: &str) -> bool {
  base_validation(passphrase, |word| {
    let mut characters: Vec<char> = word.chars().collect();
    characters.sort();
    let sorted: String = characters.into_iter().collect::<>();
    sorted
  })
}

pub fn main(args: Vec<String>) {
  let contents = read_file_lines(&args[1]);
  let count = contents.iter().filter(|p| passphrase_valid(p)).count();
  print!("Valid passwords = {}", count);
}

pub fn main_plus(args: Vec<String>) {
  let contents = read_file_lines(&args[1]);
  let count = contents.iter().filter(|p| passphrase_valid_anagram(p)).count();
  println!("Valid passwords = {}", count);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_correctly_validates_passphrases_for_uniqueness() {
    assert_eq!(true, passphrase_valid("aa bb cc dd ee") );
    assert_eq!(true, passphrase_valid("aa bb cc dd ee aaa") );
    assert_eq!(true, passphrase_valid("gggggggggggg") );
    assert_eq!(false, passphrase_valid("aaa aaa") );
    assert_eq!(false, passphrase_valid("aa bb cc dd ee aa") );
  }

  #[test]
  fn it_correctly_validates_passphrases_for_anagram() {
    assert_eq!(true, passphrase_valid_anagram("abcde fghij"));
    assert_eq!(true, passphrase_valid_anagram("a ab abc abd abf abj"));
    assert_eq!(true, passphrase_valid_anagram("iiii oiii ooii oooi oooo"));
    assert_eq!(false, passphrase_valid_anagram("abcde xyz ecdab"));
    assert_eq!(false, passphrase_valid_anagram("oiii ioii iioi iiio"));
  }
}
