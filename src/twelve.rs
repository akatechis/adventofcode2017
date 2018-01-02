use std::collections::{HashMap, HashSet};

type Visited = HashSet<u32>;
type ProgramList = HashMap<u32, Program>;

#[derive(Debug)]
struct Program {
  id: u32,
  peers: Vec<u32>
}

fn count_reachable_peers_memo(program_list: &ProgramList, root_id: u32, visited: &mut Visited) -> usize {
  if visited.contains(&root_id) {
    0
  }
  else {
    visited.insert(root_id);
    let p = program_list.get(&root_id).unwrap();
    p.peers.iter().fold(0, |count, peer_id| count + count_reachable_peers_memo(program_list, *peer_id, visited)) + 1
  }
}

fn count_reachable_peers(program_list: &ProgramList, root_id: u32) -> usize {
  let mut visited = Visited::new();
  count_reachable_peers_memo(program_list, root_id, &mut visited)
}

fn parse_programs(input: &str) -> ProgramList {
  let mut programs = ProgramList::new();

  input.lines().for_each(|line| {
    let mut parts = line.split_whitespace();
    let id: u32 = parts.next().unwrap().parse().unwrap();
    parts.next();
    let peers: Vec<u32> = parts.map(|p| p.replace(',', "").parse().unwrap()).collect();
    programs.insert(id, Program { id, peers });
  });

  programs
}

fn select_next_group_root(program_list: &ProgramList, visited: &Visited) -> Option<u32> {
  program_list.keys().find(|id| !visited.contains(&id)).map(|id| *id)
}

fn count_program_groups(program_list: &ProgramList) -> usize {
  let mut visited = Visited::new();
  let mut program_groups = 0;
  let mut root_id = 0;

  loop {
    count_reachable_peers_memo(program_list, root_id, &mut visited);
    program_groups += 1;
    match select_next_group_root(program_list, &visited) {
      Some(program_id) => {
        root_id = program_id;
      }
      None => {
        break;
      }
    }
  }

  program_groups
}

pub fn main() {
  let input = include_str!("../input/twelve");
  let programs = parse_programs(input);
  let reachable_peers = count_reachable_peers(&programs, 0);
  println!("peers reachable from program_id 0 = {:?}", reachable_peers);

  let program_groups = count_program_groups(&programs);
  println!("program groups = {}", program_groups);
}

#[cfg(test)]
mod tests {
  use super::*;
}
