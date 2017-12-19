use common::read_file_lines;

struct JumpCounter {
  jumps: Vec<i32>,
  next_offset: fn(i32) -> i32
}

fn compute_jumps_needed(counter: &mut JumpCounter) -> usize {
  let mut steps = 0;
  let mut ptr = 0;

  while ptr < counter.jumps.len() as i32 {
    let offset = counter.jumps[ptr as usize];

    steps += 1;

    // modify the jump instruction
    counter.jumps[ptr as usize] = (counter.next_offset)(offset);

    // jump
    ptr += offset;
  }

  steps
}

fn next_offset_simple(offset: i32) -> i32 {
  offset + 1
}

fn next_offset_complex(offset: i32) -> i32 {
  if offset >= 3 {
    offset - 1
  }
  else {
    offset + 1
  }
}

pub fn main(args: Vec<String>) {
  let jumps: Vec<i32> = read_file_lines(&args[1])
  .iter().map(|s| s.parse::<i32>().unwrap())
  .collect();

  let mut jump_counter = JumpCounter {
    jumps, next_offset: next_offset_simple
  };

  let result = compute_jumps_needed(&mut jump_counter);
  println!("Jumps needed to exit = {:?}", result);
}

pub fn main_plus(args: Vec<String>) {
  let jumps: Vec<i32> = read_file_lines(&args[1])
  .iter().map(|s| s.parse::<i32>().unwrap())
  .collect();

  let mut jump_counter = JumpCounter {
    jumps, next_offset: next_offset_complex
  };

  let result = compute_jumps_needed(&mut jump_counter);
  println!("Jumps needed to exit = {:?}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_computes_jumps_needed_for_simple_case() {
    let mut counter = JumpCounter {
      jumps: vec![0, 3, 0, 1, -3],
      next_offset: next_offset_simple
    };
    assert_eq!(5, compute_jumps_needed(&mut counter));
  }

  #[test]
  fn it_computes_jumps_needed_for_complex_case() {
    let mut counter = JumpCounter {
      jumps: vec![0, 3, 0, 1, -3],
      next_offset: next_offset_complex
    };
    assert_eq!(10, compute_jumps_needed(&mut counter));
  }
}
