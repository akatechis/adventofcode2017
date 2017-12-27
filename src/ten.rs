use std::io::{self, Write};

fn collect_indices_to_reverse_linear(from: usize, to: usize) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut from_ptr = from;

  while from_ptr <= to {
    indices.push(from_ptr);
    from_ptr += 1;
  }
  indices
}

fn collect_indices_to_reverse_circular(from: usize, to: usize, len: usize) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut from_ptr = from;

  while from_ptr != to + 1 {
    if from_ptr > 20 {
      break;
    }
    indices.push(from_ptr);
    from_ptr += 1;
    if from_ptr == len {
      from_ptr = 0;
    }
  }
  indices
}

fn reverse_slice_segment(slice: &mut [u16], from: usize, length: usize) {
  let slice_len = slice.len();
  let to = (from + length - 1) % slice_len;
  let indices = if from < to {
    collect_indices_to_reverse_linear(from, to)
  }
  else {
    collect_indices_to_reverse_circular(from, to, slice_len)
  };


  let end = indices.len();
  let mid = end / 2;
  let mut ptr = 0;

  while ptr < mid {
    let l_ptr = indices[ptr];
    let r_ptr = indices[end - ptr - 1];

    slice.swap(l_ptr, r_ptr);

    ptr += 1;
  }
}

fn hash_slice(input: &mut [u16], lengths: &[usize]) {
  let input_len = input.len();
  let lengths_len = lengths.len();

  let mut length_ptr = 0;
  let mut pos = 0;
  let mut skip = 0;

  while length_ptr < lengths_len {
    let current_length = lengths[length_ptr];

    println!("Reversing: {:?}, at {}, {} elements", input, pos, current_length);
    reverse_slice_segment(input, pos, current_length);

    pos += (current_length + skip) % input_len;
    length_ptr += 1;
    skip += 1;

  }
}

pub fn main() {
  let lengths: &[usize] = &[147, 37, 249, 1, 31, 2, 226, 0, 161, 71, 254, 243, 183, 255, 30, 70];
  let mut input_vec: Vec<u16> = (0..256u16).collect();
  let input: &mut [u16] = input_vec.as_mut_slice();

  hash_slice(input, lengths);

  println!("Result after hashing: {:?}", input);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn collect_indices_to_reverse_circular_works_for_span_that_covers_entire_slice() {
    let indices = collect_indices_to_reverse_circular(5, 4, 6);
    assert_eq!(vec![5, 0, 1, 2, 3, 4], indices);
  }

  #[test]
  fn collect_indices_to_reverse_circular_works_for_odd_span_length() {
    let indices = collect_indices_to_reverse_circular(5, 1, 6);
    assert_eq!(vec![5, 0, 1], indices);
  }

  #[test]
  fn collect_indices_to_reverse_circular_works_for_even_span_length() {
    let indices = collect_indices_to_reverse_circular(5, 2, 6);
    assert_eq!(vec![5, 0, 1, 2], indices);
  }

  #[test]
  fn collect_indices_to_reverse_linear_works_for_odd_span_length() {
    let indices = collect_indices_to_reverse_linear(2, 5);
    assert_eq!(vec![2, 3, 4, 5], indices);
  }

  #[test]
  fn collect_indices_to_reverse_linear_works_for_even_span_length() {
    let indices = collect_indices_to_reverse_linear(2, 6);
    assert_eq!(vec![2, 3, 4, 5, 6], indices);
  }

  #[test]
  fn reverse_slice_segment_works_for_linear_spans_with_odd_length() {
    let mut slice = [1, 2, 3, 4, 5, 6, 7, 8];
    reverse_slice_segment(&mut slice, 0, 3);
    assert_eq!([3, 2, 1, 4, 5, 6, 7, 8], slice);
  }

  #[test]
  fn reverse_slice_segment_works_for_circular_spans_with_odd_length() {
    let mut slice = [0, 1, 2, 3, 4, 5];
    reverse_slice_segment(&mut slice, 5, 3);
    assert_eq!([0, 5, 2, 3, 4, 1], slice);
  }

  #[test]
  fn reverse_slice_segment_works_for_linear_spans_with_even_length() {
    let mut slice = [1, 2, 3, 4, 5, 6, 7, 8];
    reverse_slice_segment(&mut slice, 0, 2);
    assert_eq!([2, 1, 3, 4, 5, 6, 7, 8], slice);
  }

  #[test]
  fn reverse_slice_segment_works_for_circular_spans_with_even_length() {
    let mut slice = [0, 1, 2, 3, 4, 5];
    reverse_slice_segment(&mut slice, 5, 4);
    assert_eq!([1, 0, 5, 3, 4, 2], slice);
  }

  #[test]
  fn hash_works_for_1_iteration() {
    let lengths = &[3];
    let mut input_vec: Vec<u16> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths);
    assert_eq!([2, 1, 0, 3, 4], input);
  }

  #[test]
  fn hash_works_for_2_iterations() {
    let lengths = &[3, 4];
    let mut input_vec: Vec<u16> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths);
    assert_eq!([4, 3, 0, 1, 2], input);
  }

  #[test]
  fn hash_works_for_3_iterations() {
    let lengths = &[3, 4, 1];
    let mut input_vec: Vec<u16> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths);
    assert_eq!([4, 3, 0, 1, 2], input);
  }

  #[test]
  fn hash_works_for_4_iterations() {
    let lengths = &[3, 4, 1, 5];
    let mut input_vec: Vec<u16> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths);
    assert_eq!([3, 4, 2, 1, 0], input);
  }
}
