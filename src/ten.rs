
fn compute_extended_lengths(input: String) -> Vec<usize> {
  let mut extended_lengths = Vec::new();

  for ch in input.chars() {
    extended_lengths.push(ch as usize);
  }

  extended_lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
  extended_lengths
}

fn reverse_slice_segment(slice: &[u8], from: usize, length: usize) -> Vec<u8> {
  let mut cycle: Vec<u8> = slice.iter().cycle().skip(from).take(length)
  .map(|n| *n).collect();
  cycle.reverse();

  cycle
}

fn hash_slice(lengths: &[usize], iterations: usize) -> Vec<u8> {
  let mut input_vec: Vec<u8> = (0..255u8).collect();
  input_vec.push(255);

  let input_len = input_vec.len();

  let mut pos = 0;
  let mut skip = 0;

  for _ in 0..iterations {
    for length in lengths {
      let reversed = reverse_slice_segment(&input_vec, pos, *length);
      let mut i = pos;
      for n in reversed {
        input_vec[i] = n;
        i = (i + 1) % input_len;
      }

      pos = (pos + length + skip) % input_len;
      skip += 1;
    }
  }

  input_vec
}

pub fn knot_hash(input: String) -> String {
  let ext_lengths = compute_extended_lengths(input);
  let sparse_hash = hash_slice(ext_lengths.as_slice(), 64);

  let hash: Vec<String> = sparse_hash.chunks(16)
  .map(|block| {
    let mut block_iter = block.iter();
    let initial = *block_iter.next().unwrap();
    format!("{:02x}", block_iter.fold(initial, |h, i| h ^ i))
  })
  .collect();

  hash.as_slice().join("")
}

pub fn main() {
  let lengths: &[usize] = &[147, 37, 249, 1, 31, 2, 226, 0, 161, 71, 254, 243, 183, 255, 30, 70];
  let hash_bytes = hash_slice(lengths, 1);
  println!("First two numbers are {} and {}. Their product is {}", hash_bytes[0], hash_bytes[1], hash_bytes[0] as u16 * hash_bytes[1] as u16);

  let str_input = String::from("147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70");
  let hash = knot_hash(str_input);
  println!("Hash = {}", hash);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn compute_extended_lengths_works() {
    assert_eq!(vec![49,44,50,44,51,17,31,73,47,23], compute_extended_lengths("1,2,3".to_string()));
  }

  #[test]
  fn reverse_slice_segment_works_for_linear_spans_with_odd_length() {
    let slice = &[1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(vec![3, 2, 1], reverse_slice_segment(slice, 0, 3));
  }

  #[test]
  fn reverse_slice_segment_works_for_circular_spans_with_odd_length() {
    let slice = &[0, 1, 2, 3, 4, 5];
    assert_eq!(vec![1,0,5], reverse_slice_segment(slice, 5, 3));
  }

  #[test]
  fn reverse_slice_segment_works_for_linear_spans_with_even_length() {
    let slice = &[1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(vec![2, 1], reverse_slice_segment(slice, 0, 2));
  }

  #[test]
  fn reverse_slice_segment_works_for_circular_spans_with_even_length() {
    let slice = &[0, 1, 2, 3, 4, 5];
    assert_eq!(vec![2, 1, 0, 5], reverse_slice_segment(slice, 5, 4));
  }

  #[test]
  fn knot_hash_works() {
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", knot_hash(String::from("")));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", knot_hash(String::from("1,2,3")));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", knot_hash(String::from("1,2,4")));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", knot_hash(String::from("AoC 2017")));
  }
}
