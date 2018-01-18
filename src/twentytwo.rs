use self::Dir::*;
use self::Node::*;

type Grid = Vec<Vec<Node>>;

fn parse(src: &str) -> Grid {
  src.lines().map(|ln|ln.chars().map(|ch|
    match ch {
      '#' => Infected,
      '.' => Clean,
      _ => panic!("Unknown character")
    })
  .collect()).collect()
}

fn visualize(grid: &Grid) -> String {
  let mut buf = String::new();
  for row in grid {
    for node in row {
      let ch = match *node {
        Clean => '.',
        Infected => '#'
      };
      buf.push(ch);
    }
    buf.push('\n');
  }
  buf
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
  Clean,
  Infected
}

enum Dir {
  Up, Down, Left, Right
}

impl Dir {
  fn turn_left(self) -> Dir {
    match self {
      Up => Left,
      Down => Right,
      Left => Down,
      Right => Up
    }
  }

  fn turn_right(self) -> Dir {
    match self {
      Up => Right,
      Down => Left,
      Left => Up,
      Right => Down
    }
  }
}

fn expand_grid_upwards(grid: &mut Grid) {
  let height = grid.len();
  let length = grid[0].len();
  for _ in 0..height {
    let new_row = vec![Clean; length];
    grid.insert(0, new_row);
  }
}

fn expand_grid_downwards(grid: &mut Grid) {
  let height = grid.len();
  let length = grid[0].len();
  let new_row = vec![Clean; length];
  grid.resize(height * 2, new_row);
}

fn expand_grid_leftwards(grid: &mut Grid) {
  let height = grid.len();
  let length = grid[0].len();
  for r in 0..height {
    for _ in 0..length {
      grid[r].insert(0, Clean);
    }
  }
}

fn expand_grid_rightwards(grid: &mut Grid) {
  let height = grid.len();
  let length = grid[0].len();
  for r in 0..height {
    grid[r].resize(length * 2, Clean);
  }
}

fn count_infections_simulation(grid: &mut Grid, iterations: usize) -> usize {
  let mut infections = 0;
  let mut direction = Up;
  let mut row = grid.len() / 2;
  let mut col = grid.len() / 2;

  for _ in 0..iterations {
    // turning phase
    direction = if grid[row][col] == Infected {
      direction.turn_right()
    }
    else {
      direction.turn_left()
    };

    // infection phase
    if grid[row][col] == Infected {
      grid[row][col] = Clean;
    }
    else {
      grid[row][col] = Infected;
      infections += 1;
    };

    // move phase
    match direction {
      Up => {
        if row == 0 {
          expand_grid_upwards(grid);
          row = grid.len() / 2;
        }
        row -= 1;
      },
      Down => {
        if row == grid.len() - 1 {
          expand_grid_downwards(grid);
        }
        row += 1;
      },
      Left => {
        if col == 0 {
          expand_grid_leftwards(grid);
          col = grid[0].len() / 2;
        }
        col -= 1;
      },
      Right => {
        if col == grid[row].len() - 1 {
          expand_grid_rightwards(grid);
        }
        col += 1;
      }
    }
  }

  infections
}

fn main_1() {
  let mut input = parse(include_str!("../input/twentytwo"));
  let infections = count_infections_simulation(&mut input, 10_000);

  println!("No. of Infections = {}", infections);
}

pub fn main () {
  main_1();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn count_infections_simulation_works() {
    fn run_test(iterations: usize, expected: usize) {
      let mut input = parse("..#\n#..\n...");
      let actual = count_infections_simulation(&mut input, iterations);
      assert_eq!(expected, actual);
    }

    run_test(7, 5);
    run_test(70, 41);
    run_test(10_000, 5587);
  }

  #[test]
  fn parse_works() {
    let input = ".....\n..#.#\n###.#\n#..#.\n##.#.";
    let expected = vec![
      vec![Clean;5],
      vec![Clean,Clean,Infected,Clean,Infected],
      vec![Infected,Infected,Infected,Clean,Infected],
      vec![Infected,Clean,Clean,Infected,Clean],
      vec![Infected,Infected,Clean,Infected,Clean],
    ];

    assert_eq!(expected, parse(input));
  }

  #[test]
  fn expand_grid_upwards_works() {
    let mut grid = vec![
      vec![Clean; 10],
      vec![Infected; 10],
      vec![Clean; 10]
    ];
    let actual = vec![
      vec![Clean; 10],
      vec![Clean; 10],
      vec![Clean; 10],

      vec![Clean; 10],
      vec![Infected; 10],
      vec![Clean; 10]
    ];
    expand_grid_upwards(&mut grid);

    assert_eq!(actual, grid);
  }

  #[test]
  fn expand_grid_downwards_works() {
    let mut grid = vec![
      vec![Clean; 10],
      vec![Infected; 10],
      vec![Clean; 10]
    ];
    let actual = vec![
      vec![Clean; 10],
      vec![Infected; 10],
      vec![Clean; 10],

      vec![Clean; 10],
      vec![Clean; 10],
      vec![Clean; 10]
    ];
    expand_grid_downwards(&mut grid);

    assert_eq!(actual, grid);
  }

  #[test]
  fn expand_grid_leftwards_works() {
    let mut grid = vec![
      vec![Clean; 3],
      vec![Infected; 3],
      vec![Clean; 3]
    ];
    let actual = vec![
      vec![Clean,Clean,Clean,  Clean,Clean,Clean],
      vec![Clean,Clean,Clean,  Infected,Infected,Infected],
      vec![Clean,Clean,Clean,  Clean,Clean,Clean],
    ];
    expand_grid_leftwards(&mut grid);

    assert_eq!(actual, grid);
  }

  #[test]
  fn expand_grid_rightwards_works() {
    let mut grid = vec![
      vec![Clean; 3],
      vec![Infected; 3],
      vec![Clean; 3]
    ];
    let actual = vec![
      vec![Clean,Clean,Clean,          Clean,Clean,Clean],
      vec![Infected,Infected,Infected, Clean,Clean,Clean],
      vec![Clean,Clean,Clean,          Clean,Clean,Clean],
    ];
    expand_grid_rightwards(&mut grid);

    assert_eq!(actual, grid);
  }
}
