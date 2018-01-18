use self::Instruction::*;
use std::collections::HashMap;

type Program = Vec<Instruction>;
type Registers = HashMap<char, i32>;

#[derive(Debug, PartialEq)]
enum Instruction {
  SET(char, String),
  SUB(char, String),
  MUL(char, String),
  JNZ(char, String)
}

#[derive(Debug, PartialEq)]
struct InstructionCounter {
  set: usize,
  sub: usize,
  mul: usize,
  jnz: usize
}

fn init_registers() -> Registers {
  let mut reg = HashMap::new();
  reg.insert('a', 0);
  reg.insert('b', 0);
  reg.insert('c', 0);
  reg.insert('d', 0);
  reg.insert('e', 0);
  reg.insert('f', 0);
  reg.insert('g', 0);
  reg.insert('h', 0);
  reg
}

fn register_value(registers: &Registers, register_name: char) -> i32 {
  *registers.get(&register_name).unwrap_or(&0)
}

fn literal_or_register(registers: &Registers, literal: &str) -> i32 {
  literal.parse().unwrap_or(
    register_value(registers, literal.chars().nth(0).unwrap()))
}

fn execute(registers: &mut Registers, program: &Program) -> InstructionCounter {
  let mut counter = InstructionCounter { set: 0, sub: 0, mul: 0, jnz: 0 };
  let mut i_pointer = 0;

  while i_pointer < program.len() {
    match program[i_pointer] {
      SET(reg, ref val) => {
        counter.set += 1;

        let value = literal_or_register(registers, val);
        registers.insert(reg, value);
        i_pointer += 1;
      },
      SUB(reg, ref val) => {
        counter.sub += 1;

        let v1 = register_value(registers, reg);
        let value = literal_or_register(registers, val);
        registers.insert(reg, v1 - value);
        i_pointer += 1;
      },
      MUL(reg, ref val) => {
        counter.mul += 1;

        let v1 = register_value(registers, reg);
        let value = literal_or_register(registers, val);
        registers.insert(reg, v1 * value);
        i_pointer += 1;
      },
      JNZ(reg, ref val) => {
        counter.jnz += 1;

        let reg_val = literal_or_register(registers, &reg.to_string());
        let value = literal_or_register(registers, val);
        if reg_val != 0 {
          if value > 0 {
            i_pointer += value as usize;
          }
          if value < 0 {
            i_pointer -= -value as usize;
          }
        }
        else {
          i_pointer += 1;
        }
      }
    }
  }

  counter
}

fn compile(src: &str) -> Program {
  src.lines().map(|ln| {
    let mut parts = ln.split_whitespace();
    match parts.next().unwrap() {
      "set" => {
        let reg = parts.next().unwrap().chars().nth(0).unwrap();
        let val = parts.next().unwrap().to_string();
        SET(reg, val)
      },
      "sub" => {
        let reg = parts.next().unwrap().chars().nth(0).unwrap();
        let val = parts.next().unwrap().to_string();
        SUB(reg, val)
      },
      "mul" => {
        let reg = parts.next().unwrap().chars().nth(0).unwrap();
        let val = parts.next().unwrap().to_string();
        MUL(reg, val)
      },
      "jnz" => {
        let reg = parts.next().unwrap().chars().nth(0).unwrap();
        let val = parts.next().unwrap().to_string();
        JNZ(reg, val)
      },
      instr => panic!("Invalid instruction encountered: {}", instr)
    }
  }).collect()
}

fn is_prime(candidate: usize) -> bool {
  for n in 2..candidate {
    if candidate % n == 0 {
      return false;
    }
  }
  return true;
}

fn main_2() {
  let mut b = 57 * 100 + 100000;
  let max = b + 17000;
  let mut non_primes = 0;
  loop {
    non_primes += if is_prime(b) { 0 } else { 1 };
    b += 17;

    if b > max {
      break;
    }
  }

  println!("Number of non primes = {}", non_primes);
}

fn main_1() {
  let program = compile(include_str!("../input/twentythree"));
  let mut registers = init_registers();

  let counter = execute(&mut registers, &program);
  println!("Instructions executed: {:?}", counter);
}

pub fn main() {
  main_1();
  main_2();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn execute_works() {
    let program = compile("set a 20\nsub b 5\nmul c 10\n mul c 3");
    let mut registers = init_registers();
    let counter = execute(&mut registers, &program);

    let expected = InstructionCounter {
      set: 1, sub: 1, mul: 2, jnz: 0
    };

    assert_eq!(expected, counter);
  }

  #[test]
  fn compile_works() {
    let program = compile("set a 12\nsub b 5\nmul c 10\njnz e -3");

    let expected = vec![
      SET('a', "12".to_string()),
      SUB('b', "5".to_string()),
      MUL('c', "10".to_string()),
      JNZ('e', "-3".to_string()),
    ];

    assert_eq!(expected, program);
  }
}
