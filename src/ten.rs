
fn compute_extended_lengths(lengths: &[usize]) -> Vec<usize> {
  let mut extended_lengths = Vec::new();

  for length in lengths {
    let str_length = length.to_string();

    for b in str_length.as_bytes().iter() {
      extended_lengths.push(*b as usize);
    }

    extended_lengths.push(44);
  }

  extended_lengths.pop();
  extended_lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
  extended_lengths
}

fn reverse_slice_segment(slice: &[u8], from: usize, length: usize) -> Vec<u8> {
  let mut cycle: Vec<u8> = slice.iter().cycle().skip(from).take(length)
  .map(|n| *n).collect();
  cycle.reverse();

  cycle
}

fn hash_slice(input: &mut [u8], lengths: &[usize], iterations: usize) {
  let input_len = input.len();

  let mut pos = 0;
  let mut skip = 0;

  for _ in 0..iterations {
    for length in lengths {
      let reversed = reverse_slice_segment(input, pos, *length);
      let mut i = pos;
      for n in reversed {
        input[i] = n;
        i = (i + 1) % input_len;
      }

      pos = (pos + length + skip) % input_len;
      skip += 1;
    }
  }
}

fn knot_hash(input: &mut [u8], lengths: &[usize]) -> String {
  hash_slice(input, compute_extended_lengths(lengths).as_slice(), 64);

  let hash: Vec<String> = input.chunks(16)
  .map(|block| {
    let mut block_iter = block.iter();
    let initial = *block_iter.next().unwrap();
    format!("{:x}", block_iter.fold(initial, |h, i| h ^ i))
  })
  .collect();

  hash.as_slice().join("")
}

pub fn main() {
  let lengths: &[usize] = &[147, 37, 249, 1, 31, 2, 226, 0, 161, 71, 254, 243, 183, 255, 30, 70];
  // let mut input_vec: Vec<u8> = (0..255u8).collect();
  // input_vec.push(255);
  // let input: &mut [u8] = input_vec.as_mut_slice();
  // hash_slice(input, lengths, 1);
  // println!("First two numbers are {} and {}. Their product is {}", input[0], input[1], input[0] as u16 * input[1] as u16);

  let str_input = String::from("147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70");
  let mut bytes_input = str_input.into_bytes();
  let hash_input = bytes_input.as_mut_slice();
  let hash = knot_hash(hash_input, lengths);
  println!("Hash = {}", hash);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn compute_extended_lengths_works() {
    let lengths = &[1, 2, 3];
    assert_eq!(vec![49,44,50,44,51,17,31,73,47,23], compute_extended_lengths(lengths));
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
  fn hash_works_for_1_iteration() {
    let lengths = &[3];
    let mut input_vec: Vec<u8> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths, 1);
    assert_eq!([2, 1, 0, 3, 4], input);
  }

  #[test]
  fn hash_works_for_2_iterations() {
    let lengths = &[3, 4];
    let mut input_vec: Vec<u8> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths, 1);
    assert_eq!([4, 3, 0, 1, 2], input);
  }

  #[test]
  fn hash_works_for_3_iterations() {
    let lengths = &[3, 4, 1];
    let mut input_vec: Vec<u8> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths, 1);
    assert_eq!([4, 3, 0, 1, 2], input);
  }

  #[test]
  fn hash_works_for_4_iterations() {
    let lengths = &[3, 4, 1, 5];
    let mut input_vec: Vec<u8> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths, 1);
    assert_eq!([3, 4, 2, 1, 0], input);
  }

  #[test]
  fn knot_hash_works() {
    let lengths = &[147, 37, 249, 1, 31, 2, 226, 0, 161, 71, 254, 243, 183, 255, 30, 70];

    let str_1 = String::from("AoC 2017");
    let mut bytes_1 = str_1.into_bytes();
    let input_1 = bytes_1.as_mut_slice();

    let str_2 = String::from("1,2,3");
    let mut bytes_2 = str_2.into_bytes();
    let input_2 = bytes_2.as_mut_slice();

    let str_3 = String::from("1,2,4");
    let mut bytes_3 = str_3.into_bytes();
    let input_3 = bytes_3.as_mut_slice();

    let str_empty = String::from("");
    let mut bytes_empty = str_empty.into_bytes();
    let input_empty = bytes_empty.as_mut_slice();

    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", knot_hash(input_empty, lengths));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", knot_hash(input_1, lengths));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", knot_hash(input_2, lengths));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", knot_hash(input_3, lengths));
  }
}
