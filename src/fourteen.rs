use ten::knot_hash;

type Disk = Vec<Vec<usize>>;
// 0 = free
// 1 = unmarked
// 2.. = marked

fn create_disk_for_keystring(keystring: String) -> Disk {
  let mut disk = vec![vec![0; 128]; 128];

  for row_id in 0..128 {
    let hash = knot_hash(format!("{}-{}", keystring, row_id));
    for (h, ch) in hash.chars().enumerate() {
      let bits = format!("{:04b}", ch.to_digit(16).unwrap());
      for (b, bit) in bits.chars().enumerate() {
        if bit == '1' {
          let col_id = h * 4 + b;
          disk[row_id][col_id] = 1;
        }
      }
    }
  }

  disk
}

fn count_used_bits_in_disk(disk: &Disk) -> usize {
  disk.iter()
  .map(|row|
    row.iter()
    .fold(0, |b, l| match *l {
      0 => b,
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
        1 => {
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
  if row >= disk.len() || col >= disk[row].len() {
    return;
  }
  if disk[row][col] != 1 {
    return;
  }

  // mark the rest of the row
  let mut col_ptr = col;
  while col_ptr < disk[row].len() {
    if disk[row][col_ptr] == 1 {
      disk[row][col_ptr] = region_id;
    }
    else {
      break;
    }
    col_ptr += 1;
  }

  // mark the rest of the column
  let mut row_ptr = row + 1;
  while row_ptr < disk.len() {
    if disk[row_ptr][col] == 1 {
      disk[row_ptr][col] = region_id;
    }
    else {
      break;
    }
    row_ptr += 1;
  }

  // if we marked down or to the right, we should try to expand to bottom-right
  let marked_right =
    col + 1 < disk[row].len() &&
    disk[row][col + 1] == region_id;

  let marked_bottom =
    row + 1 < disk.len() &&
    disk[row + 1][col] == region_id;

  if marked_right || marked_bottom {
    mark_region(disk, (row + 1, col + 1), region_id);
  }
}

fn count_regions_in_disk(disk: &mut Disk) -> usize {
  let mut regions = 0;

  while let Some(location) = unmarked_location(&disk) {
    regions += 1;
    println!("Location of region = {:?}", location);
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

fn print_disk(disk: &Disk) {
  for row in disk {
    for cell in row {
      match *cell {
        0 => print!("."),
        1 => print!("#"),
        _ => print!("o")
      }
    }
    print!("\n");
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn unmarked_location_works() {
    let mut disk1 = vec![vec![0; 128]; 128];
    disk1[3][3] = 1;
    disk1[4][4] = 1;

    let mut disk2 = vec![vec![0; 128]; 128];
    disk2[3][3] = 2;
    disk2[4][4] = 1;

    assert_eq!(Some((3,3)), unmarked_location(&disk1));
    assert_eq!(Some((4,4)), unmarked_location(&disk2));
  }

  #[test]
  fn mark_region_works() {
    let mut disk = vec![vec![0; 128]; 128];
    disk[0][0] = 1;
    disk[0][1] = 1;
    disk[1][0] = 1;
    disk[1][1] = 1;
    disk[2][2] = 1;
    disk[2][3] = 1;

    mark_region(&mut disk, (0, 0), 3);

    assert_eq!(3, disk[0][0]);
    assert_eq!(3, disk[0][1]);
    assert_eq!(3, disk[1][0]);
    assert_eq!(3, disk[1][1]);
    assert_eq!(1, disk[2][2]);
    assert_eq!(1, disk[2][3]);

    mark_region(&mut disk, (2, 2), 4);

    assert_eq!(3, disk[0][0]);
    assert_eq!(3, disk[0][1]);
    assert_eq!(3, disk[1][0]);
    assert_eq!(3, disk[1][1]);
    assert_eq!(4, disk[2][2]);
    assert_eq!(4, disk[2][3]);
  }

  #[test]
  fn create_disk_for_keystring_works() {
    let disk = create_disk_for_keystring("flqrgnkx".to_string());
    assert_eq!(128, disk.len());
    assert_eq!(128, disk[0].len());
    assert_eq!(1, disk[0][0]);
    assert_eq!(0, disk[5][2]);
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

    let mut disk2 = vec![vec![0; 128]; 128];
    disk2[0][0] = 1;
    disk2[0][1] = 1;
    disk2[1][0] = 1;
    disk2[1][1] = 1;
    disk2[2][2] = 1;
    disk2[2][3] = 1;

    assert_eq!(2, count_regions_in_disk(&mut disk2));
  }
}
