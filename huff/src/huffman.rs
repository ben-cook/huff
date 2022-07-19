use std::collections::{BinaryHeap, HashMap};

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

pub fn save_tree(char_map: HashMap<char, i32>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for (k, v) in char_map.into_iter() {
        result.push(k as u8);
        result.push(v as u8);
    }
    result
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

#[cfg(test)]
mod tests {
    use super::*;


    // #[test]
    // fn sanity_check() {
    //     let mut map1 = HashMap::new();
    //     let mut map2 = HashMap::new();
    //
    //     map1.insert('t', 188);
    //     map1.insert('r', 110);
    //     map1.insert('g', 30);
    //     map1.insert('c', 94);
    //     map1.insert('p', 43);
    //     map1.insert(',', 40);
    //     map1.insert('Q', 1);
    //     map1.insert('i', 230);
    //     map1.insert('d', 59);
    //     map1.insert('S', 3);
    //     map1.insert('L', 3);
    //     map1.insert('u', 185);
    //     map1.insert('o', 90);
    //     map1.insert('q', 29);
    //     map1.insert('b', 24);
    //     map1.insert('F', 2);
    //     map1.insert('x', 3);
    //     map1.insert('e', 262);
    //     map1.insert('a', 177);
    //     map1.insert('N', 7);
    //     map1.insert('V', 4);
    //     map1.insert('n', 146);
    //     map1.insert('I', 4);
    //     map1.insert(' ', 385);
    //     map1.insert('C', 3);
    //     map1.insert('v', 27);
    //     map1.insert('.', 54);
    //     map1.insert('m', 104);
    //     map1.insert('U', 1);
    //     map1.insert('h', 11);
    //     map1.insert('E', 3);
    //     map1.insert('P', 8);
    //     map1.insert('s', 181);
    //     map1.insert('M', 8);
    //     map1.insert('\n', 8);
    //     map1.insert('D', 5);
    //     map1.insert('A', 2);
    //     map1.insert('f', 18);
    //     map1.insert('j', 1);
    //     map1.insert('l', 122);
    //
    //     map2.insert('x', 3);
    //     map2.insert('a', 177);
    //     map2.insert('f', 18);
    //     map2.insert('V', 4);
    //     map2.insert('s', 181);
    //     map2.insert('v', 27);
    //     map2.insert('.', 54);
    //     map2.insert('c', 94);
    //     map2.insert('C', 3);
    //     map2.insert('i', 230);
    //     map2.insert('h', 11);
    //     map2.insert('E', 3);
    //     map2.insert('o', 90);
    //     map2.insert('l', 122);
    //     map2.insert('S', 3);
    //     map2.insert('p', 43);
    //     map2.insert('P', 8);
    //     map2.insert('n', 146);
    //     map2.insert('A', 2);
    //     map2.insert('g', 30);
    //     map2.insert('D', 5);
    //     map2.insert('\n', 8);
    //     map2.insert('Q', 1);
    //     map2.insert('q', 29);
    //     map2.insert('N', 7);
    //     map2.insert('t', 188);
    //     map2.insert('e', 6);
    //     map2.insert(' ', 129);
    //     map2.insert('m', 104);
    //     map2.insert('M', 8);
    //     map2.insert('b', 24);
    //     map2.insert('F', 2);
    //     map2.insert('I', 4);
    //     map2.insert(',', 40);
    //     map2.insert('L', 3);
    //     map2.insert('j', 1);
    //     map2.insert('u', 185);
    //     map2.insert('d', 59);
    //     map2.insert('U', 1);
    //     map2.insert('r', 110);
    //
    //     assert_eq!(map1, map2);
    //
    //     let graph1 = generate_tree(&map1);
    //     let graph2 = generate_tree(&map2);
    //     assert_eq!(graph1, graph2);
    //
    //     let character_codes1 = generate_codes(&graph1);
    //     let character_codes2 = generate_codes(&graph2);
    //     assert_eq!(character_codes1, character_codes2);
    // }
}
