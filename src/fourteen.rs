use ten::knot_hash;

#[derive(Debug, PartialEq, Clone, Copy)]
enum DiskLocation {
  Free,
  Unmarked,
  Marked(u32)
}

type Disk = [[DiskLocation; 128]; 128];

fn create_disk_for_keystring(keystring: String) -> Disk {
  let mut disk = [[DiskLocation::Free; 128]; 128];

  for row_id in 0..128 {
    let hash = knot_hash(format!("{}-{}", keystring, row_id));
    for (h, ch) in hash.chars().enumerate() {
      let bits = format!("{:04b}", ch.to_digit(16).unwrap());
      for (b, bit) in bits.chars().enumerate() {
        if bit == '1' {
          let col_id = h * 4 + b;
          disk[row_id][col_id] = DiskLocation::Unmarked;
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
      DiskLocation::Free => b,
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
        DiskLocation::Unmarked => {
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

fn mark_region(mut disk: Disk, (row, col): (usize, usize), region_id: usize) {
  if row > 0 && row < disk.len() && col > 0 && col < disk[row].len() {
    if disk[row][col] == DiskLocation::Unmarked {
      disk[row][col] = DiskLocation::Marked(region_id as u32);
    }
    mark_region(disk, (row, col + 1), region_id);
    mark_region(disk, (row + 1, col), region_id);
  }
}

fn count_regions_in_disk(disk: Disk) -> usize {
  let mut regions = 0;

  while let Some(location) = unmarked_location(&disk) {
    regions += 1;
    mark_region(disk, location, regions);
  }

  regions
}

pub fn main() {
  let disk = create_disk_for_keystring("nbysizxe".to_string());

  let bits = count_used_bits_in_disk(&disk);
  println!("Used bits = {:?}", bits);

  let regions = count_regions_in_disk(disk);
  println!("Regions = {:?}", regions);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn unmarked_location_works() {
    let disk1 = create_disk_for_keystring("abc".to_string());
    let disk2 = create_disk_for_keystring("abc".to_string());

    assert_eq!(Some((2,2)), unmarked_location(&disk1));
    assert_eq!(Some((3,0)), unmarked_location(&disk2));
  }

  #[test]
  fn mark_region_works() {
    let mut disk1 = [[DiskLocation::Free; 128]; 128];
    disk1[0][0] = DiskLocation::Unmarked;
    disk1[0][1] = DiskLocation::Unmarked;
    disk1[1][0] = DiskLocation::Unmarked;
    disk1[1][1] = DiskLocation::Unmarked;
    disk1[2][2] = DiskLocation::Unmarked;

    mark_region(disk1, (0, 0), 3);

    assert_eq!(DiskLocation::Marked(3), disk1[0][0]);
    assert_eq!(DiskLocation::Marked(3), disk1[0][1]);
    assert_eq!(DiskLocation::Marked(3), disk1[1][0]);
    assert_eq!(DiskLocation::Marked(3), disk1[1][1]);
    assert_eq!(DiskLocation::Unmarked, disk1[2][2]);
  }

  #[test]
  fn create_disk_for_keystring_works() {
    let disk = create_disk_for_keystring("flqrgnkx".to_string());
    assert_eq!(128, disk.len());
    assert_eq!(DiskLocation::Unmarked, disk[0][0]);
    assert_eq!(DiskLocation::Free, disk[5][2]);
  }

  #[test]
  fn count_used_bits_in_disk_works() {
    let disk = create_disk_for_keystring("flqrgnkx".to_string());
    assert_eq!(8108, count_used_bits_in_disk(&disk));
  }

  #[test]
  #[ignore]
  fn count_regions_in_disk_works() {
    let disk = create_disk_for_keystring("flqrgnkx".to_string());
    assert_eq!(1242, count_regions_in_disk(disk));
  }
}
