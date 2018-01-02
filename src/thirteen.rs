use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum Dir { Up, Down }

#[derive(Debug, Clone)]
struct Scanner {
  layer: u32,
  depth: u32,
  current_depth: u32,
  scan_dir: Dir
}

type Scanners = HashMap<u32, Scanner>;

fn advance_scanners(scanners: &mut Scanners) {
  scanners.values_mut().for_each(|scanner| {
    match scanner.scan_dir {
      Dir::Up => {
        scanner.current_depth -= 1;
      },
      Dir::Down => {
        scanner.current_depth += 1;
      }
    }
    if scanner.current_depth == 0 || scanner.current_depth == scanner.depth - 1 {
      scanner.scan_dir = match scanner.scan_dir {
        Dir::Up => Dir::Down,
        Dir::Down => Dir::Up
      }
    }
  });
}

fn reset_scanners(scanners: &mut Scanners) {
  scanners.values_mut().for_each(|scanner| {
    scanner.scan_dir = Dir::Down;
    scanner.current_depth = 0;
  });
}

fn parse_scanners(input: &str) -> Scanners {
  let mut scanners = Scanners::new();

  input.lines().for_each(|line| {
    let mut parts = line.split_whitespace();
    let layer: u32 = parts.next().unwrap().replace(':', "").parse().unwrap();
    let depth: u32 = parts.next().unwrap().parse().unwrap();

    scanners.insert(layer, Scanner {
      layer, depth,
      current_depth: 0,
      scan_dir: Dir::Down
    });
  });

  scanners
}

fn max_layer(scanners: &Scanners) -> u32 {
  let mut layers: Vec<&u32> = scanners.keys().collect();
  layers.sort();
  **layers.last().unwrap()
}

fn calculate_severity_for_trip(scanners: &mut Scanners, first_move: u32, max_severity: Option<usize>) -> usize {
  let mut current_time = 0;
  let mut position: i32 = -1;
  let mut severity: usize = 0;
  let max_layer = max_layer(&scanners);

  while position < max_layer as i32 {
    if current_time >= first_move {
      position += 1;
    }

    if position > -1 {
      match scanners.get(&(position as u32)) {
        Some(scanner) if scanner.current_depth == 0 => {
          severity += (scanner.layer * scanner.depth) as usize;

          // shortcircuit if max was specified, and we exceeded it
          if max_severity.is_some() && severity > max_severity.unwrap() {
            return severity;
          }
        },
        _ => ()
      }
    }

    advance_scanners(scanners);

    current_time += 1;
  }

  severity
}

fn calculate_delay_for_trip(scanners: &mut Scanners) -> u32 {
  let mut delay = 0; //15000
  reset_scanners(scanners);
  loop {
    let tasks: Vec<u32> = (delay..delay+8).collect();
    let results: Vec<Option<u32>> = tasks.par_iter()
    .map(|delay| {
      let mut local_scanners = scanners.clone();
      let severity = calculate_severity_for_trip(&mut local_scanners, *delay, Some(0));
      if severity == 0 {
        Some(*delay)
      }
      else {
        None
      }
    })
    .collect();

    match results.iter().find(|res| res.is_some()) {
      Some(result) => {
        delay = result.unwrap();
        break;
      },
      None => {
        delay += 8;
      }
    }
  }

  delay
}

pub fn main() {
  let input = include_str!("../input/thirteen");
  let mut scanners = parse_scanners(input);
  let severity = calculate_severity_for_trip(&mut scanners, 0, None);
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
    assert_eq!(24, calculate_severity_for_trip(&mut scanners, 0, None));
  }

  #[test]
  fn calculate_delay_for_trip_works() {
    let mut scanners = parse_scanners("0: 3\n1: 2\n4: 4\n6: 4");
    assert_eq!(9, calculate_delay_for_trip(&mut scanners));
  }
}
