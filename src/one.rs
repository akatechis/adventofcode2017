
fn captcha_sum(input: &str) -> u32 {
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
    let indices = (0..len).zip(1..len+1);
    for (i, j) in indices {
      let (a, b) = (chars[i], chars[j % len]);
      if a.is_numeric() && a == b {
        sum += a.to_digit(10).unwrap_or(0);
      }
    }

    sum
  }
}

// entry point to the module from main.rs
pub fn main(args: Vec<String>) {
  let sum = captcha_sum(&args[1]);
  print!("{:?}", sum);
}

// some simple tests given in the writeup
#[test]
pub fn it_works() {
  assert_eq!(0, captcha_sum(""));
  assert_eq!(7, captcha_sum("7"));
  assert_eq!(3, captcha_sum("1122"));
  assert_eq!(4, captcha_sum("1111"));
  assert_eq!(0, captcha_sum("1234"));
  assert_eq!(9, captcha_sum("91212129"));
}
