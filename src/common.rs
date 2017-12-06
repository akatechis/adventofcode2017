use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn read_file_contents(filename: &str) -> String {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut contents = String::new();
  for res in reader.lines() {
    match res {
      Ok(l) => {
        contents.push_str(&l);
        contents.push('\n');
      },
      Err(e) => {
        panic!("Couldn't read the input file: {:?}", e);
      }
    }
  }
  contents
}

pub fn read_file_lines(filename: &str) -> Vec<String> {
  read_file_contents(filename).lines().map(|s|String::from(s)).collect()
}
