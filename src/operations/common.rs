use crate::{
    DollarValue,
    structures::{
        tree::RedBlackTree,
        node_container::NodeType,
    }
};
use std::{
    sync::{Arc, RwLock},
    collections::VecDeque,
};

impl RedBlackTree {
    pub fn search(&mut self, target_key: DollarValue) -> Option<Arc<RwLock<NodeType>>> {
        let mut queue: VecDeque<Arc<RwLock<NodeType>>> = VecDeque::new();

        queue.push_back(Arc::clone(&self.head));
        while !queue.is_empty() {
            // Get NodeType lock
            let pointer_lock = queue.pop_front().unwrap();

            // Get Read Access to single NodeType
            let pointer = pointer_lock.read().unwrap();

            // If 'alive' node, access pointer struct contained in NodeType
            if let NodeType::Pointer(node_pointer) = pointer.clone() {
                // Check Self
                if node_pointer.key == target_key {
                    return Some(pointer_lock.clone());
                }

                // Add Left Child to queue
                let left_child = node_pointer.left.read().unwrap();
                match left_child.clone() {
                    // If 'alive' node, push additional owner to Arc<> into queue
                    NodeType::Pointer(_) => queue.push_back(node_pointer.left.clone()),
                    NodeType::Empty => {}
                }
                drop(left_child);

                // Add Right Child to queue
                let right_child = node_pointer.right.read().unwrap();
                match *right_child {
                    NodeType::Pointer(_) => queue.push_back(node_pointer.right.clone()),
                    NodeType::Empty => {}
                }
                drop(right_child)
            }
        }
        println!("Didn't find: {:?}", target_key);
        None
    }
    
}