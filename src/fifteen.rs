
struct Generator {
  seed: usize,
  factor: usize
}

impl Generator {
  fn new(seed: usize, factor: usize) -> Generator {
    Generator { seed, factor }
  }
}

fn compute_next_val(seed: usize, factor: usize) -> usize {
  seed * factor % 2147483647
}

impl Iterator for Generator {
  type Item=usize;
  fn next(&mut self) -> Option<usize> {
    let next_val = compute_next_val(self.seed, self.factor);
    self.seed = next_val;
    Some(next_val)
  }
}

fn count_matching_pairs<I1, I2>(a: I1, b: I2, sample: usize) -> usize
  where I1: Iterator<Item=usize>,
  I2: Iterator<Item=usize> {
  a.zip(b).take(sample).fold(0, |n, (x, y)| {
    let xs = x & 0xFFFF;
    let ys = y & 0xFFFF;
    if xs == ys {
      n + 1
    }
    else {
      n
    }
  })
}

fn main_1() {
  let mut gen_a = Generator::new(618, 16807);
  let mut gen_b = Generator::new(814, 48271);

  let pairs = count_matching_pairs(&mut gen_a, &mut gen_b, 40_000_000);
  println!("Number of matching pairs = {}", pairs);
}

fn main_2() {
  let mut gen_a = Generator::new(618, 16807).filter(|v| v % 4 == 0);
  let mut gen_b = Generator::new(814, 48271).filter(|v| v % 8 == 0);

  let pairs = count_matching_pairs(&mut gen_a, &mut gen_b, 5_000_000);
  println!("Number of filtered matching pairs = {}", pairs);
}

pub fn main () {
  main_1();
  main_2();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn generator_without_filter_works() {
    let gen_a = Generator::new(65, 16807);
    let gen_b = Generator::new(8921, 48271);

    let five_a: Vec<usize> = gen_a.take(5).collect();
    let five_b: Vec<usize> = gen_b.take(5).collect();

    let exp_five_a = vec![1092455, 1181022009, 245556042, 1744312007, 1352636452];
    let exp_five_b = vec![430625591, 1233683848, 1431495498, 137874439, 285222916];

    assert_eq!(exp_five_a, five_a);
    assert_eq!(exp_five_b, five_b);
  }

  #[test]
  fn generator_with_filter_works() {
    let gen_a = Generator::new(65, 16807).filter(|v| v % 4 == 0);
    let gen_b = Generator::new(8921, 48271).filter(|v| v % 8 == 0);

    let five_a: Vec<usize> = gen_a.take(5).collect();
    let five_b: Vec<usize> = gen_b.take(5).collect();

    let exp_five_a = vec![1352636452, 1992081072, 530830436, 1980017072, 740335192];
    let exp_five_b = vec![1233683848, 862516352, 1159784568, 1616057672, 412269392];

    assert_eq!(exp_five_a, five_a);
    assert_eq!(exp_five_b, five_b);
  }

  #[test]
  fn count_matching_pairs_works() {
    let mut gen_a = Generator::new(65, 16807);
    let mut gen_b = Generator::new(8921, 48271);

    assert_eq!(1, count_matching_pairs(&mut gen_a, &mut gen_b, 5));
  }

  #[test]
  fn count_matching_pairs_with_filter_works() {
    let mut gen_a = Generator::new(65, 16807).filter(|v| v % 4 == 0);
    let mut gen_b = Generator::new(8921, 48271).filter(|v| v % 8 == 0);

    assert_eq!(1, count_matching_pairs(&mut gen_a, &mut gen_b, 1056));
  }
}
