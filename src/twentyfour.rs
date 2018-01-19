use std::collections::HashSet;
use rayon::prelude::*;

type Item = (usize, usize);
type Bridge = Vec<Item>;

fn item_strength(item: &Item) -> usize {
  item.0 + item.1
}

fn bridge_strength(items: &Vec<Item>) -> usize {
  items.iter().fold(0, |s, item| s + item_strength(&item))
}

fn parse(src: &str) -> Vec<Item> {
  src.lines().map(|ln| {
    let mut parts = ln.split('/');
    let a = parts.next().unwrap().parse().unwrap();
    let b = parts.next().unwrap().parse().unwrap();
    (a, b)
  }).collect()
}

fn can_extend(bridge: &Bridge, item: &Item) -> bool {
  let last_item = bridge.iter().last().unwrap();
  last_item.1 == item.0
}

fn build_bridge(root: &Item, items: &mut Vec<Item>) -> Bridge {
  let mut bridge = vec![*root];
  let mut ptr = 0;
  let mut used_items = HashSet::new();

  loop {
    let next_item = items[ptr];
    let mut fit = false;

    // if the next_item "fits"
    if !used_items.contains(&ptr) && can_extend(&bridge, &next_item) {
      used_items.insert(ptr);
      bridge.push();
      fit = true;
    }
    else if !used_items.contains(&ptr) && can_extend(&bridge, &(next_item.1, next_item.0)) {
      used_items.insert(ptr);
      bridge.push();
      fit = true;
    }

    if fit {
      ptr = 0; // new piece might have made some previous piece fit
    }
    else {
      ptr += 1;
    }

    if ptr == items.len() {
      break;
    }
  }

  bridge
}

fn construct_bridge(items: &Vec<Item>) -> Bridge {
  let mut initial_pieces = vec![];

  // select initial pieces
  for n in 0..items.len() {
    if items[n].0 == 0 || items[n].1 == 0 {
      initial_pieces.push(n);
    }
  }

  let bridges: Vec<Bridge> = initial_pieces.par_iter().map(|piece|{
    let mut items_c = items.clone();
    let root = items_c.swap_remove(*piece);
    build_bridge(&root, &mut items_c)
  })
  .collect();

  bridges.iter().max_by_key(|b| bridge_strength(&b)).unwrap().clone()
}

fn main_1() {
  let bridge_items = parse(include_str!("../input/24"));
  let bridge = construct_bridge(&bridge_items);
  println!("Strength of bridge = {}", bridge_strength(&bridge));
}

pub fn main() {
  main_1();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn construct_bridge_works() {
    let items = vec![
      (0,2), (2,2), (2,3), (3,4), (3,5), (0,1), (10,1), (9,10)
    ];

    let bridge = construct_bridge(&items);
    assert_eq!(bridge, vec![(0,1),(10,1),(9,10)]);
  }

  #[test]
  fn input_works() {
    let items = parse("10/20\n5/10\n3/17");
    let expected = vec![(10,20),(5,10),(3,17)];
    assert_eq!(items, expected);
  }

  #[test]
  fn bridge_strength_works() {
    let bridge = vec![(10,20),(5,10),(3,17)];
    assert_eq!(bridge_strength(&bridge), 65);
  }
}
