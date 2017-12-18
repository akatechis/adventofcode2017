use common::read_file_lines;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct Item {
  name: String,
  weight: u32,
  children: Vec<String>
}

#[derive(Debug, PartialEq)]
struct BalanceInfo {
  name: String,
  weight: u32,
  ideal_weight: u32
}

fn calc_full_weight(item: &Item, index: &HashMap<String, Item>) -> u32 {
  let mut total_weight = item.weight;

  for child in &item.children {
    let child_item = index.get(child).unwrap();
    let child_weight = calc_full_weight(child_item, index);

    total_weight += child_weight;
  }

  total_weight
}

fn append_entry_to_key(map: &mut HashMap<u32, Vec<String>>, key: u32, entry: String) {
  if map.contains_key(&key) {
    let entries = map.get_mut(&key).unwrap();
    entries.push(entry);
  }
  else {
    map.insert(key, vec![entry]);
  }
}

fn name_of_unbalanced_child(parent: &Item, index: &HashMap<String, Item>, expected_weight: u32) -> Option<BalanceInfo> {
  let mut full_weights = HashMap::new();

  // calculate the full weight and child weight of each child
  parent.children.iter().for_each(|child_name| {
    let child = index.get(child_name).unwrap();
    let full_weight = calc_full_weight(child, index);

    append_entry_to_key(&mut full_weights, full_weight, child_name.clone());
  });

  // if all children of children have the same weight, then the oddball is the parent
  if full_weights.len() == 1 {
    Some(BalanceInfo {
      name: parent.name.clone(),
      weight: parent.weight,
      ideal_weight: expected_weight
    })
  }
  else {
    // find the child that is distinct in full_weight
    let mut oddball_name = String::from("");
    let mut oddball_weight = 0;
    let mut normal_weight = 0;

    for (weight, child_names) in full_weights.iter() {
      if child_names.len() == 1 {
        oddball_name = child_names.get(0).unwrap().clone();
        oddball_weight = *weight;
      }
      else {
        normal_weight = *weight;
      }
    }

    let oddball = index.get(&oddball_name).unwrap();
    let expected = oddball.weight - (((oddball_weight - normal_weight) as i32).abs() as u32);
    name_of_unbalanced_child(oddball, index, expected)
  }
}

fn parse_item(input: &str) -> Item {
  let mut parts = input.split(char::is_whitespace);
  let name = String::from(parts.next().unwrap());
  let w_str = parts.next().unwrap();
  let weight = w_str[1..w_str.len()-1].parse::<u32>().unwrap();

  let mut children = vec![];

  if parts.next() != None {
    for p in parts {
      let child_s = if p.chars().last().unwrap() == ',' {
        &p[..p.len()-1]
      }
      else {
        &p
      };
      let child = String::from(child_s);
      children.push(child);
    }
  }

  Item {
    name, weight, children
  }
}

pub fn main(args: Vec<String>) {
  let items: Vec<Item> = read_file_lines(&args[1])
  .iter()
  .map(|line| parse_item(&line))
  .collect();

  let mut children = HashSet::new();

  items.iter().for_each(|it| {
    it.children.iter().for_each(|child| {
      children.insert(child);
    });
  });

  items.iter().for_each(|it| {
    if !children.contains(&it.name) {
      println!("Item is the root: {:?}", it);
    }
  });
}

pub fn main_plus(args: Vec<String>) {
  let items: Vec<Item> = read_file_lines(&args[1])
  .iter()
  .map(|line| parse_item(&line))
  .collect();

  let mut index: HashMap<String, Item> = HashMap::new();

  for item in items {
    index.insert(item.name.clone(), item);
  }

  let root = index.get("hlhomy").unwrap();

  let unbalanced = name_of_unbalanced_child(root, &index, 0);

  println!("balance info: {:?}", unbalanced);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_item_works() {
    let i1 = Item {
      name: String::from("pbga"),
      weight: 66,
      children: vec![]
    };
    let i2 = Item {
      name: String::from("xhth"),
      weight: 57,
      children: vec![]
    };
    let i3 = Item {
      name: String::from("foo"),
      weight: 42,
      children: vec![String::from("aaa"), String::from("bbb"), String::from("ccc")]
    };
    let i4 = Item {
      name: String::from("bar"),
      weight: 99,
      children: vec![String::from("qqqq")]
    };

    assert_eq!(i1, parse_item("pbga (66)"));
    assert_eq!(i2, parse_item("xhth (57)"));
    assert_eq!(i3, parse_item("foo (42) -> aaa, bbb, ccc"));
    assert_eq!(i4, parse_item("bar (99) -> qqqq"));
  }

  #[test]
  fn calc_full_weight_works() {
    let index = make_item_map();

    let w1 = calc_full_weight(&index.get(&String::from("ugml")).unwrap(), &index);
    assert_eq!(251, w1);

    let w2 = calc_full_weight(&index.get(&String::from("padx")).unwrap(), &index);
    assert_eq!(243, w2);

    let w3 = calc_full_weight(&index.get(&String::from("fwft")).unwrap(), &index);
    assert_eq!(243, w3);
  }

  #[test]
  fn name_of_unbalanced_child_works() {
    let index = make_item_map();
    let root = index.get(&String::from("tknk")).unwrap();

    let name = name_of_unbalanced_child(&root, &index, 0);
    let expected = Some(BalanceInfo {
      name: String::from("ugml"),
      weight: 68,
      ideal_weight: 60
    });
    assert_eq!(expected, name);
  }

  fn make_item_map() -> HashMap<String, Item> {
    let item0 = parse_item("pbga (66)");
    let item1 = parse_item("xhth (57)");
    let item2 = parse_item("ebii (61)");
    let item3 = parse_item("havc (66)");
    let item4 = parse_item("ktlj (57)");
    let item5 = parse_item("fwft (72) -> ktlj, cntj, xhth");
    let item6 = parse_item("qoyq (66)");
    let item7 = parse_item("padx (45) -> pbga, havc, qoyq");
    let item8 = parse_item("tknk (41) -> ugml, padx, fwft");
    let item9 = parse_item("jptl (61)");
    let item10 = parse_item("ugml (68) -> gyxo, ebii, jptl");
    let item11 = parse_item("gyxo (61)");
    let item12 = parse_item("cntj (57)");

    let mut index = HashMap::new();
    index.insert(String::from("pbga"), item0);
    index.insert(String::from("xhth"), item1);
    index.insert(String::from("ebii"), item2);
    index.insert(String::from("havc"), item3);
    index.insert(String::from("ktlj"), item4);
    index.insert(String::from("fwft"), item5);
    index.insert(String::from("qoyq"), item6);
    index.insert(String::from("padx"), item7);
    index.insert(String::from("tknk"), item8);
    index.insert(String::from("jptl"), item9);
    index.insert(String::from("ugml"), item10);
    index.insert(String::from("gyxo"), item11);
    index.insert(String::from("cntj"), item12);

    return index;
  }

}
