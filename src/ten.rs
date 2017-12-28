
fn reverse_slice_segment(slice: &[u8], from: usize, length: usize) -> Vec<u8> {
  let mut cycle: Vec<u8> = slice.iter().cycle().skip(from).take(length)
  .map(|n| *n).collect();
  cycle.reverse();

  cycle
}

fn hash_slice(input: &mut [u8], lengths: &[usize]) {
  let input_len = input.len();
  let lengths_len = lengths.len();

  let mut length_ptr = 0;
  let mut pos = 0;
  let mut skip = 0;

  while length_ptr < lengths_len {
    let current_length = lengths[length_ptr];

    let reversed = reverse_slice_segment(input, pos, current_length);
    let mut i = pos;
    for n in reversed {
      input[i] = n;
      i = (i + 1) % input_len;
    }

    pos = (pos + current_length + skip) % input_len;
    length_ptr += 1;
    skip += 1;

  }
}

pub fn main() {
  let lengths: &[usize] = &[147, 37, 249, 1, 31, 2, 226, 0, 161, 71, 254, 243, 183, 255, 30, 70];
  let mut input_vec: Vec<u8> = (0..255u8).collect();
  input_vec.push(255);
  let input: &mut [u8] = input_vec.as_mut_slice();

  hash_slice(input, lengths);

  println!("First two numbers are {} and {}. Their product is {}", input[0], input[1], input[0] as u16 * input[1] as u16);
}

#[cfg(test)]
mod test {
  use super::*;

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

    hash_slice(&mut input, lengths);
    assert_eq!([2, 1, 0, 3, 4], input);
  }

  #[test]
  fn hash_works_for_2_iterations() {
    let lengths = &[3, 4];
    let mut input_vec: Vec<u8> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths);
    assert_eq!([4, 3, 0, 1, 2], input);
  }

  #[test]
  fn hash_works_for_3_iterations() {
    let lengths = &[3, 4, 1];
    let mut input_vec: Vec<u8> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths);
    assert_eq!([4, 3, 0, 1, 2], input);
  }

  #[test]
  fn hash_works_for_4_iterations() {
    let lengths = &[3, 4, 1, 5];
    let mut input_vec: Vec<u8> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths);
    assert_eq!([3, 4, 2, 1, 0], input);
  }
}
