
#[derive(Debug,PartialEq)]
enum Dir {
  NW, N, NE,
  SW, S, SE
}

fn parse_direction(dir_string: &str) -> Dir {
  match dir_string {
    "nw" => Dir::NW,
    "n" => Dir::N,
    "ne" => Dir::NE,
    "sw" => Dir::SW,
    "s" => Dir::S,
    "se" => Dir::SE,
    other => panic!("Unknown direction: {:?}", other)
  }
}

fn parse_directions(dirs: &str) -> Vec<Dir> {
  dirs.split(',').map(parse_direction).collect()
}

fn take_step(location: (i32, i32, i32), step: &Dir) -> (i32, i32, i32) {
  let mut loc = location.clone();
  match *step {
    Dir::NW => {
      loc.0 -= 1;
      loc.1 += 1;      
    },
    Dir::SE => {
      loc.0 += 1;
      loc.1 -= 1;
    },
    Dir::N => {
      loc.1 += 1;
      loc.2 -= 1;
    },
    Dir::S => {
      loc.1 -= 1;
      loc.2 += 1;
    },
    Dir::NE => {
      loc.0 += 1;
      loc.2 -= 1;
    },
    Dir::SW => {
      loc.0 -= 1;
      loc.2 += 1;
    }
  }
  loc
}

fn location(steps: &Vec<Dir>) -> (i32, i32, i32) {
  steps.iter().fold((0,0,0), take_step)
}

fn location_distance(loc: (i32, i32, i32)) -> usize {
  let (x, y, z) = loc;
  (x.abs() + y.abs() + z.abs()) as usize / 2
}

fn distance(steps: &Vec<Dir>) -> usize {
  location_distance(location(steps))
}

fn max_distance(steps: &Vec<Dir>) -> usize {
  let mut max_distance = 0;
  steps.iter().fold((0,0,0), |loc, step| {
    let new_loc = take_step(loc, step);
    let new_distance = location_distance(new_loc);
    if new_distance > max_distance {
      max_distance = new_distance;
    }
    new_loc
  });

  max_distance
}

pub fn main() {
  let input: Vec<Dir> = parse_directions(include_str!("../input/eleven"));
  let distance = distance(&input);
  let max_distance = max_distance(&input);

  println!("Distance = {}", distance);
  println!("Max distance = {}", max_distance);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn max_distance_works() {
    let steps = vec![Dir::N, Dir::N, Dir::N, Dir::N, Dir::N, Dir::S, Dir::S, Dir::S, Dir::S];
    let actual = max_distance(&steps);
    assert_eq!(5, actual);
  }

  #[test]
  fn parse_directions_works() {
    let expected = vec![Dir::NW, Dir::N, Dir::NE, Dir::SW, Dir::S, Dir::SE];
    assert_eq!(expected, parse_directions("nw,n,ne,sw,s,se"));
  }

  #[test]
  fn location_works() {
    let actual = location(&parse_directions("nw,nw,nw,se"));
    assert_eq!((-2,2,0), actual);
  }

  #[test]
  fn take_step_works() {
    // x axis
    assert_eq!((-1,1,0), take_step((0,0,0), &Dir::NW));
    assert_eq!((1,-1,0), take_step((0,0,0), &Dir::SE));

    // z axis
    assert_eq!((1,0,-1), take_step((0,0,0), &Dir::NE));
    assert_eq!((-1,0,1), take_step((0,0,0), &Dir::SW));

    // y axis
    assert_eq!((0,1,-1), take_step((0,0,0), &Dir::N));
    assert_eq!((0,-1,1), take_step((0,0,0), &Dir::S));
  }

  #[test]
  fn distance_works() {
    assert_eq!(3, distance(&parse_directions("ne,ne,ne")));
    assert_eq!(0, distance(&parse_directions("ne,ne,sw,sw")));
    assert_eq!(2, distance(&parse_directions("ne,ne,s,s")));
    assert_eq!(3, distance(&parse_directions("se,sw,se,sw,sw")));
  }
}
