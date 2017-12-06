use common::read_file_contents;

struct StepIterator {
  len: u32,
  _len: u32,
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
      len: 1,
      _len: 0,
      inc: false,
      dir: Dir::Right, 
    }
  }

  fn next(&mut self) -> Dir {
    let mut newdirection = self.dir;

    if self._len == self.len {
      self._len = 0;
      if self.inc {
        self.len += 1;
      }
      self.inc = !self.inc;
      newdirection = rotate(newdirection);
    }

    self._len += 1;
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

pub fn main(args: Vec<String>) {
  let input: u32 = read_file_contents(&args[1]).trim().parse().unwrap();
  let (x, y) = location_of_cell(input);
  let distance = x.abs() + y.abs();
  println!("distance = {}", distance);
}

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
