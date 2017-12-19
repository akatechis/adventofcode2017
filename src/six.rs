
use std::cmp::Ordering;
use std::collections::HashSet;
use common::read_file_contents;

fn choose_bank<'a>(banks: &'a Vec<u32>) -> (usize, &u32) {
  banks.iter().enumerate().max_by(|&(i_1, n_1), &(i_2, n_2)| {
    let ord = n_1.cmp(n_2);
    match ord {
      Ordering::Equal => i_2.cmp(&i_1),
      _ => ord
    }
  })
  .unwrap()
}

fn redistribute_bank(banks: &mut Vec<u32>, bank: usize) {
  // take blocks from the bank that was chosen
  let mut blocks = banks[bank];
  let mut ptr = bank;

  banks[ptr] = 0;

  // start at the bank after the one we started at
  ptr = (ptr + 1) % banks.len();

  while blocks > 0 {
    blocks -= 1;
    banks[ptr] += 1;
    ptr = (ptr + 1) % banks.len();
  }
}

fn serialize_banks(banks: &Vec<u32>) -> String {
  format!("{:?}", banks)
}

fn count_steps_for_loop(banks: &mut Vec<u32>) -> usize {
  let mut mem = HashSet::new();
  let mut steps = 0;

  mem.insert(serialize_banks(banks));

  loop {
    let (bank, _) = choose_bank(banks);
    steps += 1;
    redistribute_bank(banks, bank);
    let s = serialize_banks(banks);
    if mem.contains(&s) {
      break;
    }
    else {
      mem.insert(s);
    }
  }
  steps
}

pub fn main(args: Vec<String>) {
  let input = read_file_contents(&args[1]);
  let mut banks: Vec<u32> = input
  .split_whitespace()
  .map(|s| s.parse::<u32>().unwrap())
  .collect();

  let result_1 = count_steps_for_loop(&mut banks);
  println!("Steps needed for first loop = {}", result_1);

  let result_2 = count_steps_for_loop(&mut banks);
  println!("Steps needed for second loop = {}", result_2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn choose_bank_chooses_the_largest_bank() {
    assert_eq!((2, &7), choose_bank(&vec![0, 2, 7, 0]));
    assert_eq!((2, &8), choose_bank(&vec![0, 2, 8, 0]));
    assert_eq!((2, &8), choose_bank(&vec![0, 5, 8, 0]));

    // handles equal banks by choosing the first one
    assert_eq!((0, &22), choose_bank(&vec![22, 22, 22, 22]));
  }

  #[test]
  fn redistribute_bank_correctly_distributes() {
    {
      let mut banks = vec![0, 2, 7, 0];
      redistribute_bank(&mut banks, 2);
      assert_eq!([2,4,1,2].to_vec(), banks);
    }
    {
      let mut banks = vec![2, 4, 1, 2];
      redistribute_bank(&mut banks, 1);
      assert_eq!([3, 1, 2, 3].to_vec(), banks);
    }
    {
      let mut banks = vec![3, 1, 2, 3];
      redistribute_bank(&mut banks, 0);
      assert_eq!([0, 2, 3, 4].to_vec(), banks);
    }
  }

  #[test]
  fn count_steps_for_loop_counts_correctly() {
    let mut banks = vec![0, 2, 7, 0];
    let steps_one = count_steps_for_loop(&mut banks);
    assert_eq!(5, steps_one);

    let steps_two = count_steps_for_loop(&mut banks);
    assert_eq!(4, steps_two);
  }
}
