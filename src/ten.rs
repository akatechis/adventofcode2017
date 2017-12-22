
fn swap_values_in_slice(slice: &mut [u16], a: usize, b: usize) {
  let tmp = slice[a];
  slice[a] = slice[b];
  slice[b] = tmp;
}

fn reverse_slice_segment(slice: &mut [u16], from: usize, to: usize) {
  let len = slice.len();
  let mut from_ptr = from;
  let mut to_ptr = to - 1;

  while from_ptr < to_ptr {
    swap_values_in_slice(slice, from_ptr, to_ptr);
    from_ptr += 1;
    to_ptr -= 1;

    if from_ptr == len {
      from_ptr = 0;
    }
    if to_ptr == 0 {
      to_ptr = len - 1;
    }
  }
}

fn hash_slice(input: &mut [u16], lengths: &[u16]) {
  let input_len = input.len();
  let lengths_len = lengths.len();

  let mut length_ptr = 0;
  let mut pos = 0;
  let mut skip = 0;

  while length_ptr < lengths_len {
    length_ptr += 1;

    let current_length = lengths[length_ptr] as usize;

    println!("Reversing from {} to {} ", pos, current_length);
    reverse_slice_segment(input, pos, current_length as usize);

    pos += (current_length + skip) % input_len;
    skip += 1;
  }
}

pub fn main() {
  let lengths: &[u16]  = &[147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70];
  let mut input_vec: Vec<u16> = (0..256u16).collect();
  let input: &mut [u16] = input_vec.as_mut_slice();

  hash_slice(input, lengths);

  println!("Result after hashing: {:?}", input);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn swap_values_works_correctly() {
    let mut slice = [1,2,3,4,5];

    swap_values_in_slice(&mut slice, 0, 4);
    assert_eq!([5,2,3,4,1], slice);

    swap_values_in_slice(&mut slice, 4, 1);
    assert_eq!([5,1,3,4,2], slice);
  }

  #[test]
  fn reverse_slice_segment_works_correctly() {
    let mut slice = [1, 2, 3, 4, 5, 6, 7, 8];

    reverse_slice_segment(&mut slice, 0, 3);
    assert_eq!([3,2,1,4,5,6,7,8], slice);
  }

  #[test]
  fn reverse_slice_segment_works_on_circular_spans() {
    let mut slice = [1, 2, 3, 4, 5, 6, 7, 8];

    reverse_slice_segment(&mut slice, 5, 1);
    assert_eq!([7, 6, 3, 4, 5, 2, 1, 8], slice);
  }

  #[test]
  fn hash_works_correctly() {
    let lengths = &[3,4,1,5];
    let mut input_vec: Vec<u16> = vec![0, 1, 2, 3, 4];
    let mut input = input_vec.as_mut_slice();

    hash_slice(&mut input, lengths);
    assert_eq!([3,4,2,1,0], input);
  }
}
