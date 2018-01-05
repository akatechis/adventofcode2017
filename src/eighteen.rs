use std::collections::HashMap;
use self::Instr::*;

#[derive(Debug)]
enum Instr {
  Sound(String),
  Set(String, String),
  Add(String, String),
  Mul(String, String),
  Mod(String, String),
  Recover(String),
  Jump(String, String)
}

type Registers = HashMap<char, i64>;

fn compile_instruction(instruction: &str) -> Instr {
  let mut instr = instruction.split_whitespace();
  match instr.next().unwrap() {
    "snd" => {
      let x = instr.next().unwrap();
      Sound(x.to_string())
    },
    "set" => {
      let x = instr.next().unwrap();
      let y = instr.next().unwrap();
      Set(x.to_string(), y.to_string())
    },
    "add" => {
      let x = instr.next().unwrap();
      let y = instr.next().unwrap();
      Add(x.to_string(), y.to_string())
    },
    "mul" => {
      let x = instr.next().unwrap();
      let y = instr.next().unwrap();
      Mul(x.to_string(), y.to_string())
    },
    "mod" => {
      let x = instr.next().unwrap();
      let y = instr.next().unwrap();
      Mod(x.to_string(), y.to_string())
    },
    "rcv" => {
      let x = instr.next().unwrap();
      Recover(x.to_string())
    },
    "jgz" => {
      let x = instr.next().unwrap();
      let y = instr.next().unwrap();
      Jump(x.to_string(), y.to_string())
    },
    x => {
      panic!("Invalid instruction: {}", x);
    }
  }
}

fn init_register(registers: &mut Registers, reg: char) {
  if !registers.contains_key(&reg) {
    registers.insert(reg, 0);
  }
}

fn register_val(registers: &mut Registers, identifier: &str) -> i64 {
  let reg = identifier.chars().nth(0).unwrap();
  if reg.is_alphabetic() {
    init_register(registers, reg);
    *registers.get(&reg).unwrap()
  }
  else {
    identifier.parse().unwrap()
  }
}

fn recover_sound(registers: &mut Registers, program: Vec<Instr>) -> i64 {
  let mut pc = 0;
  let mut last_sound = 0;

  loop {
    match program[pc] {
      Recover(ref id) => {
        if register_val(registers, id) != 0 {
          return last_sound;
        }
      },
      Sound(ref id) => {
        last_sound = register_val(registers, id);
      },
      Set(ref reg, ref val_id) => {
        let register = reg.chars().nth(0).unwrap();
        let value = register_val(registers, val_id);
        registers.insert(register, value);
      },
      Jump(ref reg, ref val) => {
        let reg_val = register_val(registers, reg);
        let val_val = register_val(registers, val);
        if reg_val > 0 {
          if val_val > 0 {
            pc += val_val as usize;
          }
          if val_val < 0 {
            pc -= -val_val as usize;
          }
          // skip the increment phase of this instr
          continue;
        }
      },
      Mod(ref reg, ref val) => {
        let register = reg.chars().nth(0).unwrap();
        let reg_val = register_val(registers, reg);
        let val_val = register_val(registers, val);
        let prod = reg_val % val_val;
        registers.insert(register, prod);
      },
      Mul(ref reg, ref val) => {
        let register = reg.chars().nth(0).unwrap();
        let reg_val = register_val(registers, reg);
        let val_val = register_val(registers, val);
        let prod = reg_val * val_val;
        registers.insert(register, prod);
      },
      Add(ref reg, ref val) => {
        let register = reg.chars().nth(0).unwrap();
        let reg_val = register_val(registers, reg);
        let val_val = register_val(registers, val);
        let sum = reg_val + val_val;
        registers.insert(register, sum);
      }
    }

    // increment program counter
    pc += 1;
  }
}

pub fn main () {
  let mut registers = HashMap::new();
  let program: Vec<Instr> = include_str!("../input/eighteen").lines().map(compile_instruction).collect();

  let sound = recover_sound(&mut registers, program);
  println!("Sound recovered = {}", sound);
}
