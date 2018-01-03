
fn value_after_n_inserts(buffer: &mut Vec<usize>, step: usize, target: usize) -> usize {
  let mut ptr = 0;
  let mut value = 1;
  while value <= target {
    // step forward
    ptr = (ptr + step) % buffer.len();

    // insert
    buffer.insert(ptr + 1, value);

    // go to [value]
    ptr = value;

    println!("ptr = {}", ptr);
    value += 1;
  }

  buffer[ptr + 1]
}

pub fn main () {
  let step = 377;
  let target = 2017;
  let mut buf = vec![0];
  let answer = value_after_n_inserts(&mut buf, step, target);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn value_after_n_inserts_works() {
    let mut buf = vec![0];
    assert_eq!(1, value_after_n_inserts(&mut buf, 3, 3));
  }
}
