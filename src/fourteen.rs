use ten::knot_hash;
use self::Location::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Location {
  Free, Unmarked, Marked(usize)
}

type Disk = Vec<Vec<Location>>;

fn create_disk_for_keystring(keystring: String) -> Disk {
  let mut disk = vec![];

  for row_id in 0..128 {
    let mut row_vec = vec![];
    let hash = knot_hash(format!("{}-{}", keystring, row_id));
    for ch in hash.chars() {
      let bits = format!("{:04b}", ch.to_digit(16).unwrap());
      for bit in bits.chars() {
        if bit == '1' {
          row_vec.push(Unmarked);
        }
        else {
          row_vec.push(Free);
        }
      }
    }
    disk.push(row_vec);
  }

  disk
}

fn count_used_bits_in_disk(disk: &Disk) -> usize {
  disk.iter()
  .map(|row|
    row.iter()
    .fold(0, |b, l| match *l {
      Free => b,
      _ => b + 1
    }))
  .sum()
}

fn unmarked_location(disk: &Disk) -> Option<(usize, usize)> {
  let mut row = 0;

  while row < disk.len() {
    let mut col = 0;

    while col < disk[row].len() {

      match disk[row][col] {
        Unmarked => {
          return Some((row, col));
        }
        _ => {}
      }

      col += 1;
    }

    row += 1;
  }

  None
}

fn mark_region(disk: &mut Disk, (row, col): (usize, usize), region_id: usize) {
  if row < disk.len() && col < disk[row].len() && disk[row][col] == Unmarked {
    disk[row][col] = Marked(region_id);

    mark_region(disk, (row, col + 1), region_id);
    mark_region(disk, (row + 1, col), region_id);

    if col > 0 {
      mark_region(disk, (row, col - 1), region_id);
    }
    if row > 0 {
      mark_region(disk, (row - 1, col), region_id);
    }
  }
}

fn count_regions_in_disk(disk: &mut Disk) -> usize {
  let mut regions = 0;

  while let Some(location) = unmarked_location(&disk) {
    regions += 1;
    mark_region(disk, location, 2 + regions);
  }

  regions
}

pub fn main() {
  let mut disk = create_disk_for_keystring("nbysizxe".to_string());

  let bits = count_used_bits_in_disk(&disk);
  println!("Used bits = {:?}", bits);

  let regions = count_regions_in_disk(&mut disk);
  println!("Regions = {:?}", regions);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn unmarked_location_works() {
    let mut disk1 = vec![vec![Free; 128]; 128];
    disk1[3][3] = Unmarked;
    disk1[4][4] = Unmarked;

    let mut disk2 = vec![vec![Free; 128]; 128];
    disk2[3][3] = Marked(2);
    disk2[4][4] = Unmarked;

    assert_eq!(Some((3,3)), unmarked_location(&disk1));
    assert_eq!(Some((4,4)), unmarked_location(&disk2));
  }

  #[test]
  fn mark_region_works() {
    use super::Location::*;
    let mut disk = vec![vec![Free; 128]; 128];
    disk[0][0] = Unmarked;
    disk[0][1] = Unmarked;
    disk[1][0] = Unmarked;
    disk[1][1] = Unmarked;
    disk[2][2] = Unmarked;
    disk[2][3] = Unmarked;

    disk[5][6] = Unmarked;
    disk[5][7] = Unmarked;
    disk[5][8] = Unmarked;
    disk[5][9] = Unmarked;
    disk[6][8] = Unmarked;
    disk[7][8] = Unmarked;
    disk[7][7] = Unmarked;
    disk[7][6] = Unmarked;
    disk[7][5] = Unmarked;

    mark_region(&mut disk, (0, 0), 3);

    assert_eq!(Marked(3), disk[0][0]);
    assert_eq!(Marked(3), disk[0][1]);
    assert_eq!(Marked(3), disk[1][0]);
    assert_eq!(Marked(3), disk[1][1]);
    assert_eq!(Unmarked, disk[2][2]);
    assert_eq!(Unmarked, disk[2][3]);
    assert_eq!(Unmarked, disk[5][6]);
    assert_eq!(Unmarked, disk[5][7]);
    assert_eq!(Unmarked, disk[5][8]);
    assert_eq!(Unmarked, disk[5][9]);
    assert_eq!(Unmarked, disk[6][8]);
    assert_eq!(Unmarked, disk[7][8]);
    assert_eq!(Unmarked, disk[7][7]);
    assert_eq!(Unmarked, disk[7][6]);
    assert_eq!(Unmarked, disk[7][5]);

    mark_region(&mut disk, (2, 2), 4);

    assert_eq!(Marked(3), disk[0][0]);
    assert_eq!(Marked(3), disk[0][1]);
    assert_eq!(Marked(3), disk[1][0]);
    assert_eq!(Marked(3), disk[1][1]);
    assert_eq!(Marked(4), disk[2][2]);
    assert_eq!(Marked(4), disk[2][3]);
    assert_eq!(Unmarked, disk[5][6]);
    assert_eq!(Unmarked, disk[5][7]);
    assert_eq!(Unmarked, disk[5][8]);
    assert_eq!(Unmarked, disk[5][9]);
    assert_eq!(Unmarked, disk[6][8]);
    assert_eq!(Unmarked, disk[7][8]);
    assert_eq!(Unmarked, disk[7][7]);
    assert_eq!(Unmarked, disk[7][6]);
    assert_eq!(Unmarked, disk[7][5]);

    mark_region(&mut disk, (5,6), 22);

    assert_eq!(Marked(3), disk[0][0]);
    assert_eq!(Marked(3), disk[0][1]);
    assert_eq!(Marked(3), disk[1][0]);
    assert_eq!(Marked(3), disk[1][1]);
    assert_eq!(Marked(4), disk[2][2]);
    assert_eq!(Marked(4), disk[2][3]);
    assert_eq!(Marked(22), disk[5][6]);
    assert_eq!(Marked(22), disk[5][7]);
    assert_eq!(Marked(22), disk[5][8]);
    assert_eq!(Marked(22), disk[5][9]);
    assert_eq!(Marked(22), disk[6][8]);
    assert_eq!(Marked(22), disk[7][8]);
    assert_eq!(Marked(22), disk[7][7]);
    assert_eq!(Marked(22), disk[7][6]);
    assert_eq!(Marked(22), disk[7][5]);
  }

  #[test]
  fn create_disk_for_keystring_works() {
    let disk = create_disk_for_keystring("flqrgnkx".to_string());
    assert_eq!(128, disk.len());
    assert_eq!(128, disk[0].len());
    assert_eq!(Unmarked, disk[0][0]);
    assert_eq!(Free, disk[5][2]);
  }

  #[test]
  fn count_used_bits_in_disk_works() {
    let disk = create_disk_for_keystring("flqrgnkx".to_string());
    assert_eq!(8108, count_used_bits_in_disk(&disk));
  }

  #[test]
  fn count_regions_in_disk_works() {
    let mut disk = create_disk_for_keystring("flqrgnkx".to_string());
    assert_eq!(1242, count_regions_in_disk(&mut disk));

    let mut disk2 = vec![vec![Free; 128]; 128];
    disk2[0][0] = Unmarked;
    disk2[0][1] = Unmarked;
    disk2[1][0] = Unmarked;
    disk2[1][1] = Unmarked;
    disk2[2][2] = Unmarked;
    disk2[2][3] = Unmarked;

    assert_eq!(2, count_regions_in_disk(&mut disk2));
  }
}
