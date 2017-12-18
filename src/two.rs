use std::cmp::{max, min};
use std::u32::{MAX, MIN};
use common::read_file_contents;

fn min_max_checksum(rows: Vec<(u32, u32)>) -> u32 {
  rows.iter()
  .map(|&(a, b)| b - a)
  .fold(0, |sum, diff| sum + diff)
}

fn div_checksum(rows: Vec<(u32, u32)>) -> u32 {
  rows.iter()
  .map(|&(a, b)| a / b)
  .fold(0, |sum, n| sum + n)
}

fn min_max_parse_spreadsheet(raw: &str) -> Vec<(u32, u32)> {
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

fn div_parse_spreadsheet(raw: &str) -> Vec<(u32, u32)> {
  raw.lines()
  .map(|line| {
    let mut numbers: Vec<u32> = line
    .split_whitespace()
    .map(|num_s| num_s.parse::<u32>().unwrap())
    .collect();

    numbers.sort_by(|a, b| b.cmp(a));

    for (i, num) in numbers.iter().enumerate() {
      let mut j = i+1;
      while j < numbers.len() {
        let div = numbers.get(j).unwrap();
        if num % div == 0 {
          return (*num, *div);
        }
        j += 1;
      }
    }

    return (0, 0);
  })
  .collect()
}

pub fn main(args: Vec<String>) {
  let file_contents = read_file_contents(&args[1]);
  let spreadsheet = min_max_parse_spreadsheet(&file_contents);
  let sum = min_max_checksum(spreadsheet);

  println!("{:?}", sum);
}

pub fn main_plus(args: Vec<String>) {
  let file_contents = read_file_contents(&args[1]);
  let spreadsheet = div_parse_spreadsheet(&file_contents);
  let sum = div_checksum(spreadsheet);

  println!("{:?}", sum);
}

#[test]
fn it_can_parse_a_spreadsheet_min_max() {
  let sheet = min_max_parse_spreadsheet("5 1 9 5\n7 5 3\n2 4 6 8");

  assert_eq!(3, sheet.len());
  assert_eq!(1, sheet[0].0);
  assert_eq!(9, sheet[0].1);
  assert_eq!(3, sheet[1].0);
  assert_eq!(7, sheet[1].1);
  assert_eq!(2, sheet[2].0);
  assert_eq!(8, sheet[2].1);
}

#[test]
fn it_can_compute_a_min_max_checksum() {
  let rows = vec![(1,2), (5,10), (7, 22), (100, 500)];

  assert_eq!(421, min_max_checksum(rows));
}

#[test]
fn it_can_parse_a_spreadsheet_div() {
  let sheet = div_parse_spreadsheet("5 9 2 8\n9 4 7 3\n3 8 6 5");

  assert_eq!(3, sheet.len());
  assert_eq!(8, sheet[0].0);
  assert_eq!(2, sheet[0].1);
  assert_eq!(9, sheet[1].0);
  assert_eq!(3, sheet[1].1);
  assert_eq!(6, sheet[2].0);
  assert_eq!(3, sheet[2].1);
}

#[test]
fn it_can_compute_a_div_checksum() {
  let rows = vec![(8,2), (9, 3), (6, 3)];

  assert_eq!(9, div_checksum(rows));
}
