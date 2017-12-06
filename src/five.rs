
use common::read_file_lines;

fn jumps_needed(mut jumps: Vec<i32>) -> usize {
  let mut steps = 0;
  let mut ptr = 0;

  while ptr < jumps.len() as i32 {
    let offset = jumps[ptr as usize];

    steps += 1;

    // increment
    jumps[ptr as usize] += 1;

    // jump
    ptr += offset;
  }

  steps
}

pub fn main(args: Vec<String>) {
  let jumps: Vec<i32> = read_file_lines(&args[1])
  .iter()
  .map(|s| s.parse::<i32>().unwrap())
  .collect();

  let result = jumps_needed(jumps);
  println!("{:?}", result);
}

#[test]
fn it_computes_jumps_needed() {
  assert_eq!( 5 , jumps_needed(vec![0, 3, 0, 1, -3]) );
}
