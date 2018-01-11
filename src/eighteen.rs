use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
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

#[derive(Debug)]
struct InstructionCounter {
  snd: usize,
  set: usize,
  add: usize,
  mul: usize,
  modulus: usize,
  rcv: usize,
  jump: usize
}

impl InstructionCounter {
  fn new() -> Self {
    Self {snd: 0, set: 0, add: 0, mul: 0, modulus: 0, rcv: 0, jump: 0}
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

fn run_parallel(program: &Vec<Instr>, processes: usize) -> Arc<Mutex<Vec<InstructionCounter>>> {
  let counters = Arc::new(Mutex::new(vec![
    InstructionCounter::new(),
    InstructionCounter::new()
  ]));
  let msg_queue: Arc<Mutex<Vec<(usize, i64)>>> = Arc::new(Mutex::new(vec![]));
  let proc_state = Arc::new(Mutex::new(vec!["working".to_string(), "working".to_string()]));

  for pid in 0..processes {
    let thread_program = program.clone();
    let thread_proc_state = proc_state.clone();
    let thread_msg_queue = msg_queue.clone();
    let thread_counters = counters.clone();

    thread::spawn(move || {
      let registers = &mut Registers::new();
      registers.insert('p', pid as i64);
      let mut program_counter = 0;

      while program_counter < thread_program.len() {
        match thread_program[program_counter] {
          Rcv(ref reg) => {
            thread_counters.lock().unwrap()[pid].rcv += 1;
            let mut thread_states = thread_proc_state.lock().unwrap();
            thread_states[pid] = "waiting".to_string();
            println!("Thread {} waiting for a value", pid);
            loop {
              let mut queue = thread_msg_queue.lock().unwrap();
              match queue.iter().position(|msg| msg.0 == pid) {
                Some(msg_index) => {
                  println!("Thread {} received a value", pid);
                  let (_, value) = queue[msg_index];
                  let register = reg.chars().nth(0).unwrap();
                  registers.insert(register, value);
                  queue.remove(msg_index);
                  thread_states[pid] = "working".to_string();
                  break;
                },
                None => {
                  // release the lock before going to sleep, so other thread has a chance to snd
                  drop(queue);
                  thread::sleep(Duration::from_millis(200));
                }
              }
            }
          },
          Snd(ref id) => {
            thread_counters.lock().unwrap()[pid].snd += 1;
            let other_pid = (pid + 1) % processes;
            let value = register_val(registers, id);
            thread_msg_queue.lock().unwrap().push((other_pid, value));
            println!("Thread {} sent value to {}", pid, other_pid);
          },
          Set(ref reg, ref val_id) => {
            thread_counters.lock().unwrap()[pid].set += 1;
            let register = reg.chars().nth(0).unwrap();
            let value = register_val(registers, val_id);
            registers.insert(register, value);
          },
          Jump(ref reg, ref val) => {
            thread_counters.lock().unwrap()[pid].jump += 1;
            let reg_val = register_val(registers, reg);
            let val_val = register_val(registers, val);
            if reg_val > 0 {
              if val_val > 0 {
                program_counter += val_val as usize;
              }
              if val_val < 0 {
                program_counter -= -val_val as usize;
              }
              // skip the increment phase of this instr
              continue;
            }
          },
          Mod(ref reg, ref val) => {
            thread_counters.lock().unwrap()[pid].modulus += 1;
            let register = reg.chars().nth(0).unwrap();
            let reg_val = register_val(registers, reg);
            let val_val = register_val(registers, val);
            let prod = reg_val % val_val;
            registers.insert(register, prod);
          },
          Mul(ref reg, ref val) => {
            thread_counters.lock().unwrap()[pid].mul += 1;
            let register = reg.chars().nth(0).unwrap();
            let reg_val = register_val(registers, reg);
            let val_val = register_val(registers, val);
            let prod = reg_val * val_val;
            registers.insert(register, prod);
          },
          Add(ref reg, ref val) => {
            thread_counters.lock().unwrap()[pid].add += 1;
            let register = reg.chars().nth(0).unwrap();
            let reg_val = register_val(registers, reg);
            let val_val = register_val(registers, val);
            let sum = reg_val + val_val;
            registers.insert(register, sum);
          }
        }
        program_counter += 1;
        thread::sleep(Duration::from_millis(50));
      }

      // all done. update thread_state
      thread_proc_state.lock().unwrap()[pid] = "done".to_string();
    });
  }

  loop {
    let p_states = proc_state.lock().unwrap();
    match (p_states[0].as_str(), p_states[1].as_str()) {
      // terminal conditions:
      // both threads done
      ("done", "done") => break,

      // a thread is waiting for a thread that will never send
      ("done", "waiting") => break,
      ("waiting", "done") => break,

      // both threads waiting on each other
      ("waiting", "waiting") => break,

      // everything else, keep waiting
      (_, _) => {
        drop(p_states);
        thread::sleep(Duration::from_millis(10));
      }
    }
  }

  counters
}

pub fn main () {
  let program: Vec<Instr> = include_str!("../input/eighteen").lines().map(compile_instruction).collect();
  // let mut registers = HashMap::new();
  // let sound = recover_sound(&mut registers, &program);
  // println!("Sound recovered = {}", sound);

  let counters = run_parallel(&program, 2);
  println!("Counter for second thread = {:?}", counters.lock().unwrap()[1]);
}
