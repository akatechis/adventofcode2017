
use std::cmp::{max, min};
use std::u32::{MAX, MIN};
use common::read_file_contents;

fn checksum(rows: Vec<(u32, u32)>) -> u32 {
  rows.iter()
  .map(|&(a, b)| b - a)
  .fold(0, |sum, diff| sum + diff)
}

fn parse_spreadsheet(raw: &str) -> Vec<(u32, u32)> {
  raw.lines()
  .map(|line| {

    let row = (MAX, MIN);

    line
    .split_whitespace()
    .map(|token| token.parse::<u32>().unwrap())
    .fold(row, | (n, x), num | (min(n, num), max(x, num)) )

  })
  .collect()
}

pub fn main(args: Vec<String>) {
  let file_contents = read_file_contents(&args[1]);
  let spreadsheet = parse_spreadsheet(&file_contents);
  let sum = checksum(spreadsheet);
  println!("{:?}", sum);
}

#[test]
fn it_can_parse_a_spreadsheet() {
  let sheet = parse_spreadsheet("5 1 9 5\n7 5 3\n2 4 6 8");

  assert_eq!(3, sheet.len());
  assert_eq!(1, sheet[0].0);
  assert_eq!(9, sheet[0].1);
  assert_eq!(3, sheet[1].0);
  assert_eq!(7, sheet[1].1);
  assert_eq!(2, sheet[2].0);
  assert_eq!(8, sheet[2].1);
}

#[test]
fn it_can_compute_a_checksum() {
  let rows = vec![(1,2), (5,10), (7, 22), (100, 500)];

  assert_eq!(421, checksum(rows));
}
