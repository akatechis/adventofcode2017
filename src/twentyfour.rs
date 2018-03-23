use rayon::prelude::*;

type Item = (usize, usize);
type Bridge = Vec<Item>;

fn bridge_strength(items: &Vec<Item>) -> usize {
  items.iter().fold(0, |s, &(a, b)| s + a + b)
}

fn parse(src: &str) -> Vec<Item> {
  src.lines().map(|ln| {
    let mut parts = ln.split('/');
    let a = parts.next().unwrap().parse().unwrap();
    let b = parts.next().unwrap().parse().unwrap();
    (a, b)
  }).collect()
}

/// returns `true` if `bridge` can be appended with `item` 
fn can_extend(bridge: &Bridge, item: &Item) -> bool {
  let last_item = bridge.iter().last().unwrap();
  last_item.1 == item.0
}

fn reverse_item(item: &Item) -> Item {
  (item.1, item.0)
}

/// extends the given `bridge`, using items from `items`
fn extend_bridge(bridge: &mut Bridge, items: &mut Vec<Item>) {
  // if there are no more items, return
  if items.is_empty() {
    return;
  }

  // get a list of all items from `items` that can extend the bridge
  let candidates = get_bridge_candidate_indices(&bridge, &items);

  if candidates.is_empty() {
    return;
  }
  else {
    let strongest_bridge = candidates.iter().map(|&candidate_i| {
      let mut items_local = items.clone();
      let candidate_root = {
        let bridge_last = bridge.last().unwrap();
        let mut candidate_item = items_local.remove(candidate_i);
        if candidate_item.0 != bridge_last.1 {
          candidate_item = reverse_item(&candidate_item);
        }
        candidate_item
      };
      let mut candidate_bridge = vec![candidate_root];
      extend_bridge(&mut candidate_bridge, &mut items_local);
      candidate_bridge
    })
    .max_by_key(bridge_strength).unwrap();
    bridge.extend(strongest_bridge);
  }
}

fn get_bridge_candidate_indices(bridge: &Bridge, items: &Vec<Item>) -> Vec<usize> {
  items.iter().enumerate()
  .filter(|&(_, item)|
    can_extend(&bridge, &item) || can_extend(&bridge, &reverse_item(&item)))
  .map(|(pos, _)| pos).collect()
}

fn get_root_indices(items: &Vec<Item>) -> Vec<usize> {
  items.iter().enumerate()
    .filter(|&(_, item)| item.0 == 0 || item.1 == 0)
    .map(|(pos, _)| pos)
    .collect()
}

fn construct_bridge(items: &Vec<Item>) -> Bridge {
  let roots = get_root_indices(items);

  let strongest = roots.par_iter().map(|&root_i| {
    let mut items_local = items.clone();
    let root = {
      let mut root = items_local.remove(root_i);
      if root.1 == 0 {
        root = reverse_item(&root);
      }
      root
    };
    let mut bridge = vec![root];
    extend_bridge(&mut bridge, &mut items_local);

    bridge
  })
  .max_by_key(bridge_strength).unwrap();

  strongest
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
  fn get_bridge_candidate_indices_works() {
    let bridge = vec![(1,2), (2,9), (9,5)];
    let items = vec![(10, 22), (5, 12), (1,31), (5,7), (5,10)];
    let expected = vec![1, 3, 4];
    assert_eq!(expected, get_bridge_candidate_indices(&bridge, &items));
  }

  #[test]
  fn reverse_item_works() {
    let item = (10, 33);
    let rev = reverse_item(&item);
    assert_eq!(item, (10, 33));
    assert_eq!(rev, (33, 10));
  }

  #[test]
  fn get_root_indices_works() {
    let items = vec![
      (0,2), (2,2), (2,3), (3,4), (3,5), (1,0), (10,1), (9,10)
    ];
    let roots = vec![0, 5];
    assert_eq!(roots, get_root_indices(&items));
  }

  #[test]
  fn construct_bridge_works() {
    let items = vec![
      (0,2), (2,2), (2,3), (3,4), (3,5), (0,1), (10,1), (9,10)
    ];

    let bridge = construct_bridge(&items);
    assert_eq!(bridge, vec![(0,1),(1,10),(10,9)]);
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
