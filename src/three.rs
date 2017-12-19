use common::read_file_contents;
use std::collections::HashMap;

struct StepIterator {
  side_len: u32,
  current_len: u32,
  dir: Dir,
  inc: bool
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
  Left, Right, Up, Down
}

// produces an infinite sequence of steps
// eg: R, U, L, L, D, D, R, R, R,...
impl StepIterator {

  fn new() -> StepIterator {
    StepIterator {
      side_len: 1,
      current_len: 0,
      inc: false,
      dir: Dir::Right, 
    }
  }

  fn next(&mut self) -> Dir {
    let mut newdirection = self.dir;

    if self.current_len == self.side_len {
      self.current_len = 0;
      if self.inc {
        self.side_len += 1;
      }
      self.inc = !self.inc;
      newdirection = rotate(newdirection);
    }

    self.current_len += 1;
    self.dir = newdirection;

    newdirection
  }
}

fn rotate(d: Dir) -> Dir {
  match d {
    Dir::Left => Dir::Down,
    Dir::Right => Dir::Up,
    Dir::Up => Dir::Left,
    Dir::Down => Dir::Right
  }
}

fn location_of_cell(cell: u32) -> (i32, i32) {
  let mut location = (0, 0);
  let mut i = 1;
  let mut step_iter = StepIterator::new();

  while i < cell {
    i += 1;

    // take a step
    match step_iter.next() {
      Dir::Left => {
        location.0 -= 1;
      },
      Dir::Right => {
        location.0 += 1;
      },
      Dir::Up => {
        location.1 += 1;
      },
      Dir::Down => {
        location.1 -= 1;
      }
    }
  }
  location
}

fn shift_location(location: &mut (i32, i32), dir: Dir) {
  match dir {
    Dir::Left => {
      location.0 -= 1;
    },
    Dir::Right => {
      location.0 += 1;
    },
    Dir::Up => {
      location.1 += 1;
    },
    Dir::Down => {
      location.1 -= 1;
    }
  }
}

fn neighbor_locations(location: &(i32, i32)) -> Vec<(i32, i32)> {
  let directions = vec![
    (Dir::Left, None),
    (Dir::Right, None),
    (Dir::Up, None),
    (Dir::Down, None),
    (Dir::Left, Some(Dir::Up)),
    (Dir::Left, Some(Dir::Down)),
    (Dir::Right, Some(Dir::Up)),
    (Dir::Right, Some(Dir::Down))
  ];

  let neighbors = directions.iter().map(|&(dir1, dir2)| {
    let mut other = location.clone();
    shift_location(&mut other, dir1);
    if let Some(diagonal) = dir2 {
      shift_location(&mut other, diagonal);
    }
    other
  })
  .collect();

  neighbors
}

fn value_of_cell(cell: u32, memo: &mut HashMap<(i32,i32), u32>) -> u32 {
  if memo.len() == 0 {
    memo.insert((0,0), 1);
    // base case
    1
  }
  else {
    let loc = location_of_cell(cell);

    if memo.contains_key(&loc) {
      // cell has been computed before
      let value = memo.get(&loc).unwrap();
      *value
    }
    else {
      // let's compute the cell's value
      // let's get all the adjacent cell locations
      let cell_value = neighbor_locations(&loc).iter()
      .map(|neighbor| memo.get(&neighbor).unwrap_or(&0))
      .fold(0, |sum, n| sum + n);

      memo.insert(loc, cell_value);

      cell_value
    }
  }
}

pub fn main(args: Vec<String>) {
  let input: u32 = read_file_contents(&args[1]).trim().parse().unwrap();
  let (x, y) = location_of_cell(input);
  let distance = x.abs() + y.abs();
  println!("distance = {}", distance);
}

pub fn main_plus(args: Vec<String>) {
  let input: u32 = read_file_contents(&args[1]).trim().parse().unwrap();
  let mut cell = 0;
  let mut value: u32 = 0;
  let mut memo = HashMap::new();

  while value <= input {
    value = value_of_cell(cell, &mut memo);

    cell += 1;
  }

  println!("Smallest value greater than input = {}", value);
}

#[cfg(test)]
mod tests {

  #[test]
  fn it_computes_location() {
    assert_eq!( (0,0),   location_of_cell(1) );
    assert_eq!( (1,0),   location_of_cell(2) );
    assert_eq!( (1,1),   location_of_cell(3) );
    assert_eq!( (0,1),   location_of_cell(4) );
    assert_eq!( (-1,1),  location_of_cell(5) );
    assert_eq!( (1,2),   location_of_cell(14) );
    assert_eq!( (-2,-1), location_of_cell(20) );
    assert_eq!( (0,-2),  location_of_cell(23) );
  }

  #[test]
  fn it_computes_value() {
    let mut memo = HashMap::new();
    assert_eq!(1, value_of_cell(1, &mut memo));
    assert_eq!(1, value_of_cell(2, &mut memo));
    assert_eq!(2, value_of_cell(3, &mut memo));
    assert_eq!(4, value_of_cell(4, &mut memo));
    assert_eq!(5, value_of_cell(5, &mut memo));
    assert_eq!(10, value_of_cell(6, &mut memo));
    assert_eq!(11, value_of_cell(7, &mut memo));
    assert_eq!(23, value_of_cell(8, &mut memo));
    assert_eq!(25, value_of_cell(9, &mut memo));
    assert_eq!(26, value_of_cell(10, &mut memo));
    assert_eq!(54, value_of_cell(11, &mut memo));
  }

  #[test]
  fn it_computes_neighbor_locations() {
    let loc = (12,-4);
    let neighbors = neighbor_locations(&loc);
    assert_eq!(8, neighbors.len());
    assert_eq!(true, neighbors.contains(&(11, -4)));
    assert_eq!(true, neighbors.contains(&(13, -4)));
    assert_eq!(true, neighbors.contains(&(12, -3)));
    assert_eq!(true, neighbors.contains(&(12, -5)));

    assert_eq!(true, neighbors.contains(&(11, -3)));
    assert_eq!(true, neighbors.contains(&(11, -5)));
    assert_eq!(true, neighbors.contains(&(13, -3)));
    assert_eq!(true, neighbors.contains(&(13, -5)));
  }

  #[test]
  fn it_produces_correct_steps() {
    let mut iter = StepIterator::new();

    assert_eq!(  Dir::Right,  iter.next() );
    assert_eq!(  Dir::Up,     iter.next() );
    assert_eq!(  Dir::Left,   iter.next() );
    assert_eq!(  Dir::Left,   iter.next() );
    assert_eq!(  Dir::Down,   iter.next() );
    assert_eq!(  Dir::Down,   iter.next() );
    assert_eq!(  Dir::Right,  iter.next() );
    assert_eq!(  Dir::Right,  iter.next() );
    assert_eq!(  Dir::Right,  iter.next() );
    assert_eq!(  Dir::Up,     iter.next() );
    assert_eq!(  Dir::Up,     iter.next() );
    assert_eq!(  Dir::Up,     iter.next() );
    assert_eq!(  Dir::Left,   iter.next() );
    assert_eq!(  Dir::Left,   iter.next() );
    assert_eq!(  Dir::Left,   iter.next() );
    assert_eq!(  Dir::Left,   iter.next() );
    assert_eq!(  Dir::Down,   iter.next() );
    assert_eq!(  Dir::Down,   iter.next() );
    assert_eq!(  Dir::Down,   iter.next() );
    assert_eq!(  Dir::Down,   iter.next() );
  }

}
