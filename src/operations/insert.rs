use crate::{
    DollarValue,  
    structures::{
        active_node::{Node, NodeColor},
        node_container::NodeType,
        tree::RedBlackTree,
    }
 };
use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};
use tracing::{info, trace, error, debug};

#[derive(Clone, Debug)]
pub struct InsertForm {
    pub key: DollarValue,
    pub val: u32,
}
impl InsertForm {
    pub fn new(key: DollarValue, val: u32) -> Self {
        InsertForm { key, val }
    }
}


impl RedBlackTree {
    pub fn handle_insert(&mut self, params: InsertForm) {
        let _ = self.b_tree_insert(params.key.clone(), params.val);
        self.insertion_restore(params.key);
    }
    pub fn handle_multi_insert(&mut self, data: Vec<InsertForm>) {
        for form in data {
            self.handle_insert(form);
        }
    }
    pub fn b_tree_insert(&mut self, key: DollarValue, val: u32) -> Result<()> {
        info!("Inserting {:?}...", key);
        let mut queue: VecDeque<Arc<RwLock<NodeType>>> = VecDeque::new();
        queue.push_back(Arc::clone(&self.head));
        let mut new_node_data: Node;
        while !queue.is_empty() {
            // ! Read taken && dropped for every node until insert location found
            let pointer_lock = queue.pop_front().unwrap();
            let pointer = pointer_lock.read().unwrap();
            new_node_data = Node::new(key.clone(), val, pointer_lock.clone(), NodeColor::Red);

            match pointer.clone() {
                // Reached when 0 element tree
                NodeType::Empty => {
                    self.head = Arc::new(RwLock::new(NodeType::from(new_node_data)));
                    break;
                }
                // Else traverse pointer closer to correct leaf node
                NodeType::Pointer(current_node) => {
                    // Collision Handling
                    if &new_node_data.key == &current_node.key {
                        return Err(anyhow!("Key already exists."));
                    }

                    // Send pointer left || right
                    // least -> greatest from left -> right
                    // Send Right
                    if &new_node_data.key > &current_node.key {
                        let right_lock = current_node.right.clone();
                        let right_child = right_lock.read().unwrap();
                        match *right_child {
                            NodeType::Empty => {
                                drop(right_child);
                                current_node.set_right_child(NodeType::from(new_node_data));
                                break;
                            }
                            NodeType::Pointer(_) => queue.push_back(current_node.right),
                        }
                    } else
                    /* Send Left */
                    {
                        let left_lock = current_node.left.clone();
                        let left_child = left_lock.read().unwrap();
                        match *left_child {
                            NodeType::Empty => {
                                drop(left_child);
                                current_node.set_left_child(NodeType::from(new_node_data));
                                break;
                            }
                            NodeType::Pointer(_) => queue.push_back(current_node.left),
                        }
                    }
                }
            }
        }
        Ok(())
    }

}