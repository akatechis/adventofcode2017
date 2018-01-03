use std::collections::HashMap;

type Programs = Vec<char>;

#[derive(Debug, PartialEq)]
enum Step {
  Spin(usize),
  Exchange(usize,usize),
  Partner(char, char)
}

fn create_programs(programs: usize) -> Programs {
  let mut p = vec![];
  for c in 0..programs as u8 {
    let ch = 'a' as u8 + c;
    p.push(ch as char);
  }
  p
}

fn format_programs(programs: &Programs) -> String {
  let mut fmt = String::new();

  for ch in programs {
    fmt.push(*ch);
  }

  fmt
}

fn perform_step(programs: &mut Programs, step: Step) {
  match step {
    Step::Spin(amount) => {
      for _ in 0..amount {
        let pr = programs.pop().unwrap();
        programs.insert(0, pr);
      }
    },
    Step::Exchange(left, right) => {
      programs.swap(left, right);
    },
    Step::Partner(a, b) => {
      let left = programs.iter().position(|&p|p==a).unwrap();
      let right = programs.iter().position(|&p|p==b).unwrap();
      programs.swap(left, right);
    }
  }
}

fn do_the_dance<I>(programs: &mut Programs, steps: &mut I) where I:Iterator<Item=Step> {
  for step in steps {
    perform_step(programs, step);
  }
}

fn decode_step(step_str: &str) -> Step {
  match step_str.chars().nth(0).unwrap() {
    's' => {
      let amt: usize = step_str[1..].parse().unwrap();
      Step::Spin(amt)
    },
    'x' => {
      let mut indices = step_str[1..].split('/');
      let a: usize = indices.next().unwrap().parse().unwrap();
      let b: usize = indices.next().unwrap().parse().unwrap();
      Step::Exchange(a, b)
    },
    'p' => {
      let mut names = step_str[1..].split('/');
      let x: char = names.next().unwrap().chars().nth(0).unwrap();
      let y: char = names.next().unwrap().chars().nth(0).unwrap();
      Step::Partner(x, y)
    },
    step => panic!("Invalid step encoding: {}", step)
  }
}

fn main_1<'a,I>(input: &mut I) where I:Iterator<Item=&'a str> {
  let mut p = create_programs(16);
  do_the_dance(&mut p, &mut input.map(decode_step));
  let program_output = format_programs(&p);
  println!("Program output = {}", program_output);
}

fn main_2() {
  let mut p = create_programs(16);
  let mut dances = 0;
  let mut configurations = HashMap::new();

  configurations.insert(format_programs(&p), 0);

  let input = &mut include_str!("../input/sixteen").split(',').cycle().map(decode_step);

  loop {
    do_the_dance(&mut p, &mut input.take(10_000));
    let cfg = format_programs(&p);
    dances += 1;

    if configurations.contains_key(&cfg) {
      let first_seen = *configurations.get(&cfg).unwrap();
      let offset = (1_000_000_000 % (dances - first_seen)) - first_seen;
      for (hash, o) in configurations.clone() {
        if offset == o {
          println!("Configuration after a billion: {}", hash);
          return;
        }
      }
    }
    else {
      configurations.insert(cfg, dances);
    }
  }
}

pub fn main() {
  // let mut input1 = include_str!("../input/sixteen").split(',');
  // main_1(&mut input1);

  main_2();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn perform_spin_works() {
    let mut p = vec!['a','b','c','d','e'];

    perform_step(&mut p, Step::Spin(1));

    assert_eq!(vec!['e','a','b','c','d'], p);
  }

  #[test]
  fn perform_exchange_works() {
    let mut p = vec!['e','a','b','c','d'];

    perform_step(&mut p, Step::Exchange(3,4));

    assert_eq!(vec!['e','a','b','d','c'], p);
  }

  #[test]
  fn perform_partner_works() {
    let mut p = vec!['e','a','b','d','c'];

    perform_step(&mut p, Step::Partner('e', 'b'));

    assert_eq!(vec!['b','a','e','d','c'], p);
  }

  #[test]
  fn format_programs_works() {
    let p = vec!['a','z','q','x','b','n'];
    assert_eq!("azqxbn".to_string(), format_programs(&p));
  }

  #[test]
  fn decode_step_works() {
    assert_eq!(Step::Spin(7), decode_step("s7"));
    assert_eq!(Step::Exchange(7, 12), decode_step("x7/12"));
    assert_eq!(Step::Partner('n', 'b'), decode_step("pn/b"));
  }

  #[test]
  fn create_programs_works() {
    assert_eq!(vec!['a','b','c','d','e'], create_programs(5));
    assert_eq!('p', *create_programs(16).iter().last().unwrap());
  }

  #[test]
  fn do_the_dance_works() {
    let mut p = create_programs(5);
    let mut steps = "s1,x3/4,pe/b".split(",").map(decode_step);

    do_the_dance(&mut p, &mut steps);

    assert_eq!("baedc".to_string(), format_programs(&p));
  }
}
