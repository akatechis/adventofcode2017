use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Scanner {
  layer: u32,
  depth: u32
}

type Scanners = HashMap<u32, Scanner>;

fn parse_scanners(input: &str) -> Scanners {
  let mut scanners = Scanners::new();

  input.lines().for_each(|line| {
    let mut parts = line.split_whitespace();
    let layer: u32 = parts.next().unwrap().replace(':', "").parse().unwrap();
    let depth: u32 = parts.next().unwrap().parse().unwrap();

    scanners.insert(layer, Scanner { layer, depth });
  });

  scanners
}

fn severity(severity: usize, scanner: &Scanner) -> usize {
  if scanner.layer % (scanner.depth * 2 - 2) == 0 {
    severity + scanner.layer as usize * scanner.depth as usize
  }
  else {
    severity + 0
  }
}

fn delay_scanners(scanners: &mut Scanners, delay: u32) {
  scanners.values_mut().for_each(|scanner| {
    scanner.layer += delay;
  });
}

fn reset_scanners(scanners: &mut Scanners, delay: u32) {
  scanners.values_mut().for_each(|scanner| {
    scanner.layer -= delay;
  });
}

fn calculate_severity_for_trip(scanners: &mut Scanners, delay: u32) -> usize {
  delay_scanners(scanners, delay);
  let severity = scanners.values().fold(0, severity);
  reset_scanners(scanners, delay);
  severity
}

fn calculate_delay_for_trip(scanners: &mut Scanners) -> u32 {
  let mut delay = 0;
  loop {
    let severity = calculate_severity_for_trip(scanners, delay);
    if severity == 0 {
      break;
    }
    delay += 1;
  }
  delay
}

pub fn main() {
  let input = include_str!("../input/thirteen");
  let mut scanners = parse_scanners(input);
  let severity = calculate_severity_for_trip(&mut scanners, 0);
  println!("Severity = {}", severity);

  let delay = calculate_delay_for_trip(&mut scanners);
  println!("Delay = {}", delay);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_severity_for_trip_works() {
    let mut scanners = parse_scanners("0: 3\n1: 2\n4: 4\n6: 4");
    assert_eq!(24, calculate_severity_for_trip(&mut scanners, 0));
  }

  #[test]
  fn calculate_delay_for_trip_works() {
    let mut scanners = parse_scanners("0: 3\n1: 2\n4: 4\n6: 4");
    assert_eq!(10, calculate_delay_for_trip(&mut scanners));
  }
}
