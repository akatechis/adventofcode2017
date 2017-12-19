
enum StreamItem {
  Group(String, u32),
  Garbage(String, u32)
}

struct StreamReader {
  raw: String,
  ptr: i32
}

impl StreamReader {
  fn next() -> Option<StreamItem> {

  }
}

pub fn main (args: Vec<String>) {

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_garbage() {

  }

}
