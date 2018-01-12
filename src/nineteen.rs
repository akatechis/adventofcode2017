use self::Dir::*;

#[derive(Debug, PartialEq)]
enum Dir {
  Up, Down, Left, Right
}

type Map = Vec<Vec<char>>;

fn initial_column(map: &Map) -> usize {
  map[0].iter().position(|&x|x == '|').unwrap()
}

fn directions() -> Vec<Dir> {
  vec![Up, Down, Left, Right]
}

fn check_bounds(map: &Map, row: usize, col: usize, dir: &Dir) -> bool {
  match *dir {
    Up => row > 0,
    Down => row < map.len()-1,
    Left => col > 0,
    Right => col < map[0].len()-1
  }
}

fn rotate(map: &Map, direction: Dir, row: usize, col: usize) -> Dir {
  let excluded = match direction {
    Up => Down,
    Down => Up,
    Left => Right,
    Right => Left
  };
  for d in directions() {
    let d_leads_to_pipe_or_alpha = match d {
      Up => if check_bounds(map, row, col, &Up) {
          map[row-1][col].is_alphabetic() || map[row-1][col] == '|'
        }
        else {
          false
        },
      Down => if check_bounds(map, row, col, &Down) {
          map[row+1][col].is_alphabetic() || map[row+1][col] == '|'
        }
        else {
          false
        },
      Left => if check_bounds(map, row, col, &Left) {
          map[row][col-1].is_alphabetic() || map[row][col-1] == '-'
        }
        else {
          false
        },
      Right => if check_bounds(map, row, col, &Right) {
          map[row][col+1].is_alphabetic() || map[row][col+1] == '-'
        }
        else {
          false
        },
    };
    if d != excluded && d_leads_to_pipe_or_alpha {
      return d;
    }
  }
  panic!(format!("Couldn't find a rotation at ({},{}), coming from {:?}", row, col, direction));
}

fn navigate(map: &Map) -> (String, usize) {
  let mut symbols = String::new();
  let mut d = Down;
  let mut row = 0;
  let mut col = initial_column(map);
  let mut steps = 1;

  loop {
    if check_bounds(map, row, col, &d) {
      // take a step in the current direction
      match d {
        Up => row -= 1,
        Down => row += 1,
        Left => col -= 1,
        Right => col += 1,
      }

      // if we landed on a tile w/ a letter, collect it.
      let tile = map[row][col];
      if tile.is_alphabetic() {
        symbols.push(tile);
      }

      // if we landed on a junction, determine the next direaction
      if tile == '+' {
        d = rotate(map, d, row, col);
      }

      // if we landed on blank, we're done
      if tile == ' ' {
        break;
      }
      else {
        steps += 1;
      }
    }
    // current dir leads to out of bounds
    else {
      break;
    }
  }

  (symbols, steps)
}

pub fn main () {
  let input: Vec<Vec<char>> = include_str!("../input/nineteen").lines().map(|ln|ln.chars().collect()).collect();
  let path = navigate(&input);
  println!("Symbols = {}", path.0);
  println!("Steps = {}", path.1);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_bounds_works() {
    let map = vec![
      "     |          ".chars().collect(),
      "     |  +--+    ".chars().collect(),
      "     A  |  C    ".chars().collect(),
      " F---|----E|--+ ".chars().collect(),
      "     |  |  |  D ".chars().collect(),
      "     +B-+  +--+ ".chars().collect()
    ];

    assert_eq!(false, check_bounds(&map, 0,0, &Up));
    assert_eq!(true, check_bounds(&map, 0,0, &Right));
    assert_eq!(true, check_bounds(&map, 0,0, &Down));
    assert_eq!(false, check_bounds(&map, 0,8, &Up));
    assert_eq!(false, check_bounds(&map, 0,15, &Up));

    assert_eq!(false, check_bounds(&map, 0,0, &Left));
    assert_eq!(false, check_bounds(&map, 3,0, &Left));
    assert_eq!(false, check_bounds(&map, 5,0, &Left));

    assert_eq!(false, check_bounds(&map, 5,0, &Down));
    assert_eq!(false, check_bounds(&map, 5,8, &Down));
    assert_eq!(false, check_bounds(&map, 5,15, &Down));

    assert_eq!(false, check_bounds(&map, 0,15, &Right));
    assert_eq!(false, check_bounds(&map, 3,15, &Right));
    assert_eq!(false, check_bounds(&map, 5,15, &Right));
  }

  #[test]
  fn navigate_works() {
    let input = vec![
      "     |          ".chars().collect(),
      "     |  +--+    ".chars().collect(),
      "     A  |  C    ".chars().collect(),
      " F---|----E|--+ ".chars().collect(),
      "     |  |  |  D ".chars().collect(),
      "     +B-+  +--+ ".chars().collect()
    ];
    assert_eq!(("ABCDEF".to_string(), 38), navigate(&input));
  }

  #[test]
  fn rotate_works() {
    let m1 = vec![
      "         |      ".chars().collect(),
      "       --+      ".chars().collect(),
      "                ".chars().collect(),
    ];

    // came to the junction from the left
    assert_eq!(Up, rotate(&m1, Right, 1, 9));

    // came to the junction from the top
    assert_eq!(Left, rotate(&m1, Down, 1, 9));

    let m2 = vec![
      "                ".chars().collect(),
      "       +--      ".chars().collect(),
      "       |        ".chars().collect(),
    ];

    // came to the junction from the bottom
    assert_eq!(Right, rotate(&m2, Up, 1, 7));

    // came to the junction from the right
    assert_eq!(Down, rotate(&m2, Left, 1, 7));
  }
}
