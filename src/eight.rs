
use common::read_file_lines;
use std::collections::HashMap;
use std::i32::MIN;

#[derive(Debug)]
struct Performance {
  reg_name: String,
  reg_value: i32
}

#[derive(PartialEq, Debug)]
enum Cond {
  EQ(String, i32),
  NE(String, i32),
  LT(String, i32),
  GT(String, i32),
  LTE(String, i32),
  GTE(String, i32)
}

#[derive(PartialEq, Debug)]
enum Instruction {
  INC(String, i32),
  DEC(String, i32)
}

#[derive(Debug)]
struct CPU {
  registers: HashMap<String, i32>,
  perf: Performance
}

impl CPU {
  fn new() -> CPU {
    CPU {
      registers: HashMap::new(),
      perf: Performance {
        reg_name: String::from(""),
        reg_value: MIN
      }
    }
  }

  fn init(reg_name: String, reg_value: i32) -> CPU {
    let mut registers = HashMap::new();
    registers.insert(reg_name, reg_value);
    CPU {
      registers,
      perf: Performance {
        reg_name: String::from(""), reg_value: MIN
      }
    }
  }

  fn evaluate_condition(&self, condition: &Cond) -> bool {
    match *condition {
      Cond::EQ(ref register_name, value) => {
        let reg_value = self.registers.get(register_name).unwrap_or(&0);
        reg_value == &value
      },

      Cond::NE(ref register_name, value) => {
        let reg_value = self.registers.get(register_name).unwrap_or(&0);
        reg_value != &value
      },

      Cond::LT(ref register_name, value) => {
        let reg_value = self.registers.get(register_name).unwrap_or(&0);
        reg_value < &value
      },

      Cond::LTE(ref register_name, value) => {
        let reg_value = self.registers.get(register_name).unwrap_or(&0);
        reg_value <= &value
      },

      Cond::GT(ref register_name, value) => {
        let reg_value = self.registers.get(register_name).unwrap_or(&0);
        reg_value > &value
      },

      Cond::GTE(ref register_name, value) => {
        let reg_value = self.registers.get(register_name).unwrap_or(&0);
        reg_value >= &value
      }

    }
  }

  fn execute(&mut self, instruction: &Instruction) {
    match *instruction {
      Instruction::INC(ref register_name, delta) => {
        let register = self.registers.entry(register_name.clone()).or_insert(0);
        *register += delta;

        if register > &mut self.perf.reg_value {
          self.perf.reg_value = *register;
          self.perf.reg_name = register_name.clone();
        }
      },

      Instruction::DEC(ref register_name, delta) => {
        let register = self.registers.entry(register_name.clone()).or_insert(0);
        *register -= delta;

        if register > &mut self.perf.reg_value {
          self.perf.reg_value = *register;
          self.perf.reg_name = register_name.clone();
        }
      }
    }
  }

  fn max_register(&self) -> (String, i32) {
    let mut max_v = MIN;
    let mut max_n = "".to_string();
    for (name, &value) in &self.registers {
      if value > max_v {
        max_v = value;
        max_n = name.clone();
      }
    }
    (max_n, max_v)
  }

  fn peak_register(&self) -> &Performance {
    &self.perf
  }
}

fn decode_instr(text: &str) -> Instruction {
  let mut tokens = text.split_whitespace();
  let register_name = tokens.next().unwrap();
  let op = tokens.next().unwrap();
  let diff: i32 = tokens.next().unwrap().parse().unwrap();

  match op {
    "inc" => Instruction::INC(register_name.to_string(), diff),
    "dec" => Instruction::DEC(register_name.to_string(), diff),
    e => panic!("Unknown op: {}", e)
  }
}

fn decode_cond(text: &str) -> Cond {
  let mut tokens = text.split_whitespace();
  let register_name = tokens.next().unwrap();
  let comparator = tokens.next().unwrap();
  let ref_value: i32 = tokens.next().unwrap().parse().unwrap();

  match comparator {
    "==" => Cond::EQ(register_name.to_string(), ref_value),
    "!=" => Cond::NE(register_name.to_string(), ref_value),
    ">" => Cond::GT(register_name.to_string(), ref_value),
    ">=" => Cond::GTE(register_name.to_string(), ref_value),
    "<" => Cond::LT(register_name.to_string(), ref_value),
    "<=" => Cond::LTE(register_name.to_string(), ref_value),
    e => panic!("Unknown comparator: {}", e)
  }
}

fn decode(text: &str) -> (Instruction, Cond) {
  let mut segments = text.split("if");

  let instr = decode_instr(segments.next().unwrap());
  let cnd = decode_cond(segments.next().unwrap());

  (instr, cnd)
}

pub fn main(args: Vec<String>) {
  let mut cpu = CPU::new();

  read_file_lines(&args[1]).iter()
  .for_each(|raw_instruction| {
    let (instr, cnd) = decode(&raw_instruction);

    if cpu.evaluate_condition(&cnd) {
      cpu.execute(&instr);
    }

  });

  let (max_name, max_value) = cpu.max_register();
  println!("Max register at end is {}, whose value is {}", max_name, max_value);

  let &Performance{ ref reg_name, ref reg_value } = cpu.peak_register();
  println!("Peak register was {}, whose value was {}", reg_name, reg_value);
}

#[cfg(test)]
mod tests {

  mod cpu {
    use eight::*;

    #[test]
    fn cpu_correctly_reports_the_maximum_register() {
      let mut cpu = CPU::init("a".to_string(), 33);
      cpu.execute(&Instruction::INC("b".to_string(), 55));
      cpu.execute(&Instruction::INC("c".to_string(), 12));
      cpu.execute(&Instruction::DEC("qwe".to_string(), -5000));
      cpu.execute(&Instruction::INC("b".to_string(), 1000));

      let (max_name, max_value) = cpu.max_register();

      assert_eq!("qwe", max_name);
      assert_eq!(5000, max_value);
    }
  }

  mod decode {
    use eight::*;

    #[test]
    fn decode_inc_instruction() {
      let expected_instr = Instruction::INC("b".to_string(), 5);
      let expected_cnd = Cond::GT("a".to_string(), 1);
      let (instr, cnd) = decode("b inc 5 if a > 1");

      assert_eq!(expected_instr, instr);
      assert_eq!(expected_cnd, cnd);
    }

    #[test]
    fn decode_dec_instruction() {
      let expected_instr = Instruction::DEC("c".to_string(), -10);
      let expected_cnd = Cond::GTE("a".to_string(), 1);
      let (instr, cnd) = decode("c dec -10 if a >= 1");

      assert_eq!(expected_instr, instr);
      assert_eq!(expected_cnd, cnd);
    }
  }

  mod conditions {
    use eight::*;
    #[test]
    fn equality_condition_works_for_register_that_exists () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(true, cpu.evaluate_condition(&Cond::EQ("a".to_string(), 12)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::EQ("a".to_string(), 11)) );
    }

    #[test]
    fn equality_condition_works_for_register_that_does_not_exist () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(false, cpu.evaluate_condition(&Cond::EQ("b".to_string(), 12)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::EQ("b".to_string(), 99)));
    }

    #[test]
    fn inequality_condition_works_for_register_that_exists () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(false, cpu.evaluate_condition(&Cond::NE("a".to_string(), 12)));
      assert_eq!(true, cpu.evaluate_condition(&Cond::NE("a".to_string(), 99)));
    }

    #[test]
    fn inequality_condition_works_for_register_that_does_not_exist () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(true, cpu.evaluate_condition(&Cond::NE("b".to_string(), 12)));
      assert_eq!(true, cpu.evaluate_condition(&Cond::NE("b".to_string(), 55)));
    }

    #[test]
    fn less_than_condition_works_for_register_that_exists () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(true, cpu.evaluate_condition(&Cond::LT("a".to_string(), 13)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::LT("a".to_string(), 12)));
    }

    #[test]
    fn less_than_condition_works_for_register_that_does_not_exist () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(true, cpu.evaluate_condition(&Cond::LT("b".to_string(), 13)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::LT("b".to_string(), -12)));
    }

    #[test]
    fn less_than_equal_condition_works_for_register_that_exists () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(true, cpu.evaluate_condition(&Cond::LTE("a".to_string(), 13)));
      assert_eq!(true, cpu.evaluate_condition(&Cond::LTE("a".to_string(), 12)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::LTE("a".to_string(), 11)));
    }

    #[test]
    fn less_than_equal_condition_works_for_register_that_does_not_exist () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(true, cpu.evaluate_condition(&Cond::LTE("b".to_string(), 13)));
      assert_eq!(true, cpu.evaluate_condition(&Cond::LTE("b".to_string(), 12)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::LTE("b".to_string(), -11)));
    }

    #[test]
    fn greater_than_condition_works_for_register_that_exists () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(false, cpu.evaluate_condition(&Cond::GT("a".to_string(), 13)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::GT("a".to_string(), 12)));
      assert_eq!(true, cpu.evaluate_condition(&Cond::GT("a".to_string(), 11)));
    }

    #[test]
    fn greater_than_condition_works_for_register_that_does_not_exist () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(false, cpu.evaluate_condition(&Cond::GT("b".to_string(), 13)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::GT("b".to_string(), 12)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::GT("b".to_string(), 11)));
    }

    #[test]
    fn greater_than_equal_condition_works_for_register_that_exists () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(false, cpu.evaluate_condition(&Cond::GTE("a".to_string(), 13)));
      assert_eq!(true, cpu.evaluate_condition(&Cond::GTE("a".to_string(), 12)));
      assert_eq!(true, cpu.evaluate_condition(&Cond::GTE("a".to_string(), 11)));
    }

    #[test]
    fn greater_than_equal_condition_works_for_register_that_does_not_exist () {
      let cpu = CPU::init("a".to_string(), 12);

      assert_eq!(false, cpu.evaluate_condition(&Cond::GTE("b".to_string(), 13)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::GTE("b".to_string(), 12)));
      assert_eq!(false, cpu.evaluate_condition(&Cond::GTE("b".to_string(), 11)));
    }

  }

  mod instructions {
    use eight::*;

    #[test]
    fn inc_instruction_increments_by_positive_amount_when_register_exists() {
      let mut cpu = CPU::init("a".to_string(), 42);
      cpu.execute(&Instruction::INC("a".to_string(), 5));

      assert_eq!(47, *cpu.registers.get(&"a".to_string()).unwrap());
    }

    #[test]
    fn inc_instruction_increments_by_negative_amount_when_register_exists() {
      let mut cpu = CPU::init("a".to_string(), 42);
      cpu.execute(&Instruction::INC("a".to_string(), -5));

      assert_eq!(37, *cpu.registers.get(&"a".to_string()).unwrap());
    }

    #[test]
    fn inc_instruction_increments_by_positive_amount_when_register_does_not_exist() {
      let mut cpu = CPU::init("a".to_string(), 42);
      cpu.execute(&Instruction::INC("b".to_string(), 5));

      assert_eq!(5, *cpu.registers.get(&"b".to_string()).unwrap());
    }

    #[test]
    fn inc_instruction_increments_by_negative_amount_when_register_does_not_exist() {
      let mut cpu = CPU::init("a".to_string(), 42);
      cpu.execute(&Instruction::INC("b".to_string(), -5));

      assert_eq!(-5, *cpu.registers.get(&"b".to_string()).unwrap());
    }

    #[test]
    fn dec_instruction_decreases_by_positive_amount_when_register_exists() {
      let mut cpu = CPU::init("a".to_string(), 42);
      cpu.execute(&Instruction::DEC("a".to_string(), 5));

      assert_eq!(37, *cpu.registers.get(&"a".to_string()).unwrap());
    }

    #[test]
    fn dec_instruction_decreases_by_negative_amount_when_register_exists() {
      let mut cpu = CPU::init("a".to_string(), 42);
      cpu.execute(&Instruction::DEC("a".to_string(), -5));

      assert_eq!(47, *cpu.registers.get(&"a".to_string()).unwrap());
    }

    #[test]
    fn dec_instruction_decrements_by_positive_amount_when_register_does_not_exist() {
      let mut cpu = CPU::init("a".to_string(), 42);
      cpu.execute(&Instruction::DEC("b".to_string(), 5));

      assert_eq!(-5, *cpu.registers.get(&"b".to_string()).unwrap());
    }

    #[test]
    fn dec_instruction_decrements_by_negative_amount_when_register_does_not_exist() {
      let mut cpu = CPU::init("a".to_string(), 42);
      cpu.execute(&Instruction::DEC("b".to_string(), -5));

      assert_eq!(5, *cpu.registers.get(&"b".to_string()).unwrap());
    }

  }
}
