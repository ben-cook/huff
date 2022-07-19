use std::collections::{BinaryHeap, HashMap};

use anyhow::Result;

use crate::binary_tree::Node;

pub fn generate_tree(map: &HashMap<char, i32>) -> Node<(i32, Option<char>)> {
    let mut heap = BinaryHeap::new();

    // Create the nodes of the tree
    for (k, v) in map.iter() {
        let new_node = Node::<(i32, Option<char>)> {
            left: None,
            right: None,
            value: (-1 * v, Some(*k)),
        };
        heap.push(new_node);
    }

    // Insert the nodes into the tree in the optimal order
    while heap.len() > 1 {
        let first_popped_node = heap
            .pop()
            .expect("Binary Heap is empty. This should be unreachable.");
        let second_popped_node = heap
            .pop()
            .expect("Binary Heap is empty. This should be unreachable.");

        let combined_node = Node::<(i32, Option<char>)> {
            value: (first_popped_node.value.0 + second_popped_node.value.0, None),
            left: Some(Box::new(first_popped_node)),
            right: Some(Box::new(second_popped_node)),
        };

        heap.push(combined_node);
    }

    heap.pop().expect("Heap is empty after algorithm ran")
}

pub fn save_tree(char_map: HashMap<char, i32>) -> Result<Vec<u8>> {
    let mut result: Vec<u8> = Vec::new();
    for (k, v) in char_map.into_iter() {
        // u8 restricts this to only ascii
        // TODO: support utf-8
        result.push(k as u8);

        leb128::write::unsigned(&mut result, v.try_into()?)?;
    }
    Ok(result)
}

pub fn generate_codes(root: &Node<(i32, Option<char>)>) -> HashMap<char, String> {
    let mut map: HashMap<char, String> = HashMap::new();
    let mut arr: Vec<i32> = Vec::new();

    recursive_generate_codes(root, &mut arr, &mut map);

    map
}

fn recursive_generate_codes(
    root: &Node<(i32, Option<char>)>,
    current_array: &mut Vec<i32>,
    map: &mut HashMap<char, String>,
) {
    if let Some(node) = &root.left {
        current_array.push(0);
        recursive_generate_codes(node, current_array, map);
        current_array.pop();
    }

    if let Some(node) = &root.right {
        current_array.push(1);
        recursive_generate_codes(node, current_array, map);
        current_array.pop();
    }

    if let Some(char) = root.value.1 {
        // This is a leaf node
        map.insert(
            char,
            current_array
                .iter()
                .map(|s| s.to_string())
                .reduce(|cur, next| format!("{}{}", cur, next))
                .unwrap(),
        );
    }
}
