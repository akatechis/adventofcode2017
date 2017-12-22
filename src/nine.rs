use std::str::Chars;

fn compute_total_score(stream: Chars) -> (u32, u32, u32) {
  let mut total = 0;
  let mut total_chars = 0;
  let mut garbage_chars = 0;

  let mut score = 0;
  let mut parsing_garbage = false;
  let mut skip = false;

  for ch in stream {
    match ch {

      '{' => {
        total_chars += 1;
        if !skip && parsing_garbage {
          garbage_chars += 1;
        }

        if !skip && !parsing_garbage {
          score += 1;
          total += score;
        }
        skip = false;
      }

      '}' => {
        total_chars += 1;
        if !skip && parsing_garbage {
          garbage_chars += 1;
        }

        if !skip && !parsing_garbage {
          score -= 1;
        }
        skip = false;
      }

      '<' => {
        total_chars += 1;
        if !skip && parsing_garbage {
          garbage_chars += 1;
        }

        if !skip {
          parsing_garbage = true;
        }
        skip = false;
      }

      '>' => {
        total_chars += 1;

        if !skip {
          parsing_garbage = false;
        }
        skip = false;
      }

      '!' => {
        total_chars += 1;

        skip = !skip;
      }

      _ => {
        total_chars += 1;
        if !skip && parsing_garbage {
          garbage_chars += 1;
        }

        skip = false;
      }

    }
  }

  (total, total_chars, garbage_chars)
}

pub fn main() {
  let input = include_str!("../input/nine");
  let score = compute_total_score(input.chars());

  println!("Score for stream = {}", score.0);

  println!("Total characters = {}", score.1);
  println!("Garbage characters = {}", score.2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn compute_total_score_counts_single_group() {
    assert_eq!((1, 2, 0), compute_total_score("{}".chars()));
  }

  #[test]
  fn compute_total_score_counts_nested_groups() {
    assert_eq!((6, 6, 0), compute_total_score("{{{}}}".chars()));
    assert_eq!((5, 7, 0), compute_total_score("{{},{}}".chars()));
    assert_eq!((16, 14, 0), compute_total_score("{{{},{},{{}}}}".chars()));
  }

  #[test]
  fn compute_total_score_counts_ignores_garbage() {
    assert_eq!((1, 17, 4), compute_total_score("{<a>,<a>,<a>,<a>}".chars()));
    assert_eq!((9, 29, 8), compute_total_score("{{<ab>},{<ab>},{<ab>},{<ab>}}".chars()));
  }

  #[test]
  fn compute_total_score_skips_bang_character() {
    assert_eq!((10, 14, 0), compute_total_score("{{},{},{!}{}}}".chars()));
    assert_eq!((9, 29, 0), compute_total_score("{{<!!>},{<!!>},{<!!>},{<!!>}}".chars()));
    assert_eq!((3, 29, 17), compute_total_score("{{<a!>},{<a!>},{<a!>},{<ab>}}".chars()));
  }

}
