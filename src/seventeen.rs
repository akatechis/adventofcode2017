
fn value_after_n_inserts(buffer: &mut Vec<usize>, step: usize, inserts: usize, value_after: usize) -> usize {
  let mut ptr = 0;
  for value in 1..inserts + 1 {
    ptr = (ptr + step) % buffer.len();
    buffer.insert(ptr + 1, value);
    ptr += 1;
  }

  let pos = buffer.iter().position(|&v|v==value_after).unwrap();
  buffer[pos + 1]
}

fn spinlock(step: usize) -> usize {
  let mut current = 0;
  let mut one_slot = 0;
  for v in 1..50_000_000+1 {
    current = ((current + step) % v) + 1;
    if current == 1 {
      one_slot = v;
    }
  }
  one_slot
}

pub fn main () {
  {
    let step = 377;
    let inserts = 2017;
    let mut buf = vec![0];
    let answer = value_after_n_inserts(&mut buf, step, inserts, inserts);
    println!("Position after 2017: {}", answer);
  }
  {
    let answer = spinlock(377);
    println!("Position after 0: {}", answer);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn value_after_n_inserts_works() {
    assert_eq!(1, value_after_n_inserts(&mut vec![0], 3, 3, 3));
    assert_eq!(638, value_after_n_inserts(&mut vec![0], 3, 2017, 2017));
    assert_eq!(151, value_after_n_inserts(&mut vec![0], 3, 2017, 1134));
  }
}
