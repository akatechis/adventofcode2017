use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use self::Instr::*;

#[derive(Debug, Clone)]
enum Instr {
  Snd(String),
  Set(String, String),
  Add(String, String),
  Mul(String, String),
  Mod(String, String),
  Rcv(String),
  Jump(String, String)
}

struct Process {
  self_id: usize,
  other_id: usize,
  program_counter: usize,
  sends_executed: usize,
  registers: Registers
}

enum Message {
  Working,
  Waiting,
  Send(usize, i64),
  Recv
}

impl Process {
  fn new(id: usize) -> Self {
    let mut registers = Registers::new();
    registers.insert('p', id as i64);
    Self {
      self_id: id,
      other_id: if id == 0 { 1 } else { 0 },
      registers,
      program_counter: 0,
      sends_executed: 0,
    }
  }
}

type Registers = HashMap<char, i64>;

fn compile_instruction(instruction: &str) -> Instr {
  let mut instr = instruction.split_whitespace();
  match instr.next().unwrap() {
    "snd" => {
      let x = instr.next().unwrap();
      Snd(x.to_string())
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
      Rcv(x.to_string())
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

fn recover_sound(registers: &mut Registers, program: &Vec<Instr>) -> i64 {
  let mut pc = 0;
  let mut last_sound = 0;

  loop {
    match program[pc] {
      Rcv(ref id) => {
        if register_val(registers, id) != 0 {
          return last_sound;
        }
      },
      Snd(ref id) => {
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

fn run_progam(program_ptr: Box<(usize, Vec<Instr>, Arc<Mutex<Vec<(usize, i64)>>>)>) -> Process {
  let (pid, program, msg_queue) = *program_ptr;
  let process = Process::new(pid);
  process
}

fn run_parallel(program: &Vec<Instr>, processes: usize) {
  let msg_queue = Arc::new(Mutex::new(vec![]));
  let mut receivers = vec![];
  let mut threads = vec![];

  for pid in 0..processes {
    let (tx_p, rx_m) = mpsc::channel();
    receivers.push(rx_m);
    // let (tx_m, rx_p) = mpsc::channel();

    let t_program = program.clone();
    let t_queue = Arc::clone(&msg_queue);
    let program_ptr = Box::new((pid, t_program, t_queue));

    let t = thread::spawn(move || {
      let process = run_progam(program_ptr);
      tx_p.send(process).unwrap();
    });

    threads.push(t);
  }
}

pub fn main () {
  let mut registers = HashMap::new();
  let program: Vec<Instr> = include_str!("../input/eighteen").lines().map(compile_instruction).collect();
  let sound = recover_sound(&mut registers, &program);
  println!("Sound recovered = {}", sound);

  run_parallel(&program, 2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_program_runs_parallel() {
    let program = vec![
      Instr::Set("a".to_string(), "22".to_string()),
      Instr::Add("a".to_string(), "20".to_string())
    ];
    let processes = run_parallel(&program);

    assert_eq!(42, *processes[0].registers.get(&'a').unwrap());
    assert_eq!(42, *processes[1].registers.get(&'a').unwrap());
  }
}
