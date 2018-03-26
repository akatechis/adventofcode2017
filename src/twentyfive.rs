use std::collections::VecDeque;
use std::collections::HashMap;
use std::str::Lines;
use self::TapeValue::*;
use self::Direction::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
  Left, Right
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TapeValue {
  One, Zero
}

struct StateBranch {
  write: TapeValue,
  move_dir: Direction,
  transition: String,
}

struct State {
  name: String,
  zero_branch: StateBranch,
  one_branch: StateBranch,
}

struct TuringMachine {
  tape: VecDeque<TapeValue>,
  cursor: usize,
  curr_state: String,
  max_steps: usize,
  curr_steps: usize,
  states: HashMap<String, State>,
}

fn parse_initial_state(lines: &mut Lines) -> String {
  let words = lines.next().unwrap().split_whitespace();
  let last = words.last().unwrap();
  last[..last.len() - 1].to_string()
}

fn parse_max_steps(lines: &mut Lines) -> usize {
  let mut words = lines.next().unwrap().split_whitespace();
  words.nth(5).unwrap().parse().unwrap()
}

fn parse_states(lines: &mut Lines) -> HashMap<String, State> {
  let mut states = HashMap::new();

  loop {
    let mut state_buf = Vec::with_capacity(9);

    loop {
      let ln = lines.next();
      match ln {
        None => break,
        Some(line) => if line == "" {
          break;
        }
        else {
          state_buf.push(line);
        },
      }
    }

    if state_buf.len() == 9 {
      let state = State::from(state_buf);
      states.insert(state.name.clone(), state);
    }
    else {
      break;
    }
  }

  states
}

fn parse_state_branch_from_iterator(iter: &mut Iterator<Item=&&str>) -> StateBranch {
  let write_raw = iter.next().unwrap().split_whitespace().nth(4).unwrap();
  let write = match write_raw {
    "1." => One,
    "0." => Zero,
    _ => panic!("Got invalid write for a state: {}", write_raw),
  };

  let move_dir_raw = iter.next().unwrap().split_whitespace().nth(6).unwrap();
  let move_dir = match move_dir_raw {
    "left." => Left,
    "right." => Right,
    _ => panic!("Got invalid move direction for a state, {}", move_dir_raw),
  };

  let transition_raw = iter.next().unwrap().split_whitespace().nth(4).unwrap();
  let transition = transition_raw[..transition_raw.len() - 1].to_string();

  StateBranch {
    write, move_dir, transition,
  }
}

impl State {
  fn from(spec: Vec<&str>) -> Self {
    let mut iter = spec.iter();
    let name_w = iter.next().unwrap().split_whitespace().nth(2).unwrap();
    let name = name_w[..name_w.len() - 1].to_string();

    iter.next(); // we assume we're in the zero section
    let zero_branch = parse_state_branch_from_iterator(&mut iter);

    iter.next(); // we assume we're in the one section
    let one_branch = parse_state_branch_from_iterator(&mut iter);

    State {
      name, zero_branch, one_branch
    }
  }
}

impl TuringMachine {
  fn from(spec: &str) -> Self {
    let mut lines = spec.lines();

    let curr_state = parse_initial_state(&mut lines);
    let max_steps = parse_max_steps(&mut lines);

    // discard the blank line between preamble and first state
    lines.next();
    let states = parse_states(&mut lines);

    let mut tape = VecDeque::new();
    tape.push_back(Zero);

    TuringMachine {
      tape,
      cursor: 0,
      curr_state,
      max_steps,
      curr_steps: 0,
      states,
    }
  }

  fn run_to_max_steps(&mut self) {
    while self.curr_steps < self.max_steps {
      self.curr_steps += 1;

      let state = self.states.get(&self.curr_state).unwrap();

      let tape_val = self.tape[self.cursor];

      let branch = match tape_val {
        Zero => &state.zero_branch, One => &state.one_branch,
      };

      // write the value to tape
      self.tape[self.cursor] = branch.write;

      // move left or right
      match branch.move_dir {
        Left => {
          // if cursor is 0, push_front
          if self.cursor == 0 {
            self.tape.push_front(Zero);
          }
          else {
            self.cursor -= 1;
          }
        },
        Right => {
          // if cursor is at the end, push_back
          if self.cursor == self.tape.len() - 1 {
            self.tape.push_back(Zero);
          }
          self.cursor += 1;
        },
      }

      // transition state
      self.curr_state = branch.transition.clone();
    }
  }

  fn checksum(&self) -> usize {
    self.tape.iter().fold(0, |sum, val| sum + match val {
      &Zero => 0, &One => 1
    })
  }
}

fn main_1() {
  let input = include_str!("../input/25");
  let mut machine = TuringMachine::from(&input);
  machine.run_to_max_steps();

  let checksum = machine.checksum();
  println!("Checksum: {}", checksum);
}

pub fn main() {
  main_1();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parsing_source_works() {
    let src = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";
    let machine = TuringMachine::from(&src);

    assert_eq!("A".to_string(), machine.curr_state);
    assert_eq!(6, machine.max_steps);
    assert_eq!(0, machine.curr_steps);
    assert_eq!(0, machine.cursor);

    assert_eq!(1, machine.tape.len());
    assert_eq!(Some(&Zero), machine.tape.get(0));

    assert_eq!(2, machine.states.len());
    assert_eq!(true, machine.states.contains_key("A"));
    assert_eq!(true, machine.states.contains_key("B"));

    let state_a = machine.states.get("A").unwrap();
    let a_zero = &state_a.zero_branch;
    let a_one = &state_a.one_branch;

    assert_eq!(One, a_zero.write);
    assert_eq!(Right, a_zero.move_dir);
    assert_eq!("B".to_string(), a_zero.transition);

    assert_eq!(Zero, a_one.write);
    assert_eq!(Left, a_one.move_dir);
    assert_eq!("B".to_string(), a_one.transition);

    let state_b = machine.states.get("B").unwrap();
    let b_zero = &state_b.zero_branch;
    let b_one = &state_b.one_branch;

    assert_eq!(One, b_zero.write);
    assert_eq!(Left, b_zero.move_dir);
    assert_eq!("A".to_string(), b_zero.transition);

    assert_eq!(One, b_one.write);
    assert_eq!(Right, b_one.move_dir);
    assert_eq!("A".to_string(), b_one.transition);
  }

  #[test]
  fn running_a_turing_machine_works() {
    let src = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";
    let mut machine = TuringMachine::from(&src);
    machine.run_to_max_steps();

    assert_eq!(3, machine.checksum());
  }
}
