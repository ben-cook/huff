use std::{collections::{BinaryHeap, HashMap}};

use structopt::StructOpt;
use anyhow::{Context, Result};

mod binary_tree;
use binary_tree::Node;

#[derive(StructOpt)]
pub struct Cli {
  #[structopt(parse(from_os_str))]
  pub input_path: std::path::PathBuf,
}

pub fn run(args: Cli) -> Result<()> {
  let content = std::fs::read_to_string(&args.input_path)
      .with_context(|| format!("could not read file {:?}", &args.input_path))?;

  let char_vec: Vec<char> = content.chars().collect();

  let character_counts = get_character_counts(&char_vec);

  let min_heap = node_heap(character_counts);
  
  let root_node = generate_graph(min_heap);

  let codes = generate_codes(root_node);

  let mut encoding = String::new();
  for char in char_vec {
    encoding.push_str(codes.get(&char).unwrap());
  }

  print!("{}", encoding);

  Ok(())
}

fn get_character_counts(string: &Vec<char>) -> HashMap<char, i32> {
  let mut character_counts: HashMap<char, i32> = HashMap::new();

  for char in string {
    *character_counts.entry(*char).or_insert(0) += 1;
  }

  character_counts
}

fn node_heap<K: Ord>(map: HashMap<K, i32>) -> BinaryHeap<Node<(i32, Option<K>)>> {
  let mut heap = BinaryHeap::new();

  for (k, v) in map.into_iter() {
    let new_node = Node::<(i32, Option<K>)> {
      left: None,
      right: None,
      value: (-1 * v, Some(k)),
    };
    heap.push(new_node);
  }

  heap
}

fn generate_graph(mut heap: BinaryHeap<Node<(i32, Option<char>)>>) -> Node<(i32, Option<char>)> {
  while heap.len() > 1 {
    let first_popped_node = heap.pop().expect("Binary Heap is empty for some reason.");
    let second_popped_node = heap.pop().expect("Binary Heap is empty for some reason.");

    let combined_node = Node::<(i32, Option<char>)> {
      value: (first_popped_node.value.0 + second_popped_node.value.0, None),
      left: Some(Box::new(first_popped_node)),
      right: Some(Box::new(second_popped_node)),
    };

    heap.push(combined_node);
  }

  let root_node = heap.pop().expect("Heap is empty after algorithm ran");
  root_node
}

fn generate_codes(root: Node<(i32, Option<char>)>) -> HashMap<char, String> {
  let mut map: HashMap<char, String> = HashMap::new();
  let mut arr: Vec<i32> = Vec::new();

  recursive_codes(root, &mut arr, &mut map);
  
  map
}

fn recursive_codes(root: Node<(i32, Option<char>)>, current_array: &mut Vec<i32>, map: &mut HashMap<char, String>) {
  if let Some(node) = root.left {
    current_array.push(0);
    recursive_codes(*node, current_array, map);
    current_array.pop();
  }

  if let Some(node) = root.right {
    current_array.push(1);
    recursive_codes(*node, current_array, map);
    current_array.pop();
  }

  if let Some(char) = root.value.1 {
    // This is a leaf node
    map.insert(char, current_array.iter().map(|s| s.to_string())
                                              .reduce(|cur, next| cur + &next)
                                              .unwrap());
  }
}