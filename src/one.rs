use common::read_file_contents;

fn base_captcha(input: &str, lookahead: usize) -> u32 {
  let chars: Vec<char> = input.chars().collect();
  let len = input.len();

  if input.len() == 0 {
    0
  }
  else if input.len() == 1 {
    chars[0].to_digit(10).unwrap_or(0)
  }
  else {
    let mut sum = 0_u32;
    let correlated = lookahead..len+lookahead;
    let indices = 0..len;

    for (i, j) in indices.zip(correlated) {
      let (a, b) = (chars[i], chars[j % len]);
      if a.is_numeric() && a == b {
        sum += a.to_digit(10).unwrap_or(0);
      }
    }

    sum
  }
}

fn captcha_sum(input: &str) -> u32 {
  base_captcha(input, 1)
}

fn captcha_sum_rot(input: &str) -> u32 {
  let len = input.len();
  let half = len / 2 as usize;
  base_captcha(input, half)
}

pub fn main(args: Vec<String>) {
  let input = read_file_contents(&args[1]);
  let seq = input.trim();
  let sum = captcha_sum(&seq);
  print!("Captcha is {:?}", sum);
}

pub fn main_plus(args: Vec<String>) {
  let input = read_file_contents(&args[1]);
  let seq = input.trim();
  let sum = captcha_sum_rot(&seq);
  println!("Captcha is {:?}", sum);
}

// some simple tests given in the writeup
#[test]
fn captcha_sum_works() {
  assert_eq!(0, captcha_sum(""));
  assert_eq!(7, captcha_sum("7"));
  assert_eq!(3, captcha_sum("1122"));
  assert_eq!(4, captcha_sum("1111"));
  assert_eq!(0, captcha_sum("1234"));
  assert_eq!(9, captcha_sum("91212129"));
}

#[test]
fn captcha_sum_rot_works() {
  assert_eq!(0, captcha_sum_rot(""));
  assert_eq!(7, captcha_sum_rot("7"));
  assert_eq!(6, captcha_sum_rot("1212"));
  assert_eq!(0, captcha_sum_rot("1221"));
  assert_eq!(4, captcha_sum_rot("123425"));
  assert_eq!(12, captcha_sum_rot("123123"));
  assert_eq!(4, captcha_sum_rot("12131415"));
}
