use anyhow::{anyhow, Result};
use std::{
    convert::From,
    sync::{Arc, RwLock}
};
use crate::structures::node_container::NodeType;


use crate::DollarValue; // TEMP

#[derive(Debug, Clone)]
pub enum NodeColor {
    Red,
    Black,
}
#[derive(Debug, Clone)]
pub struct Node <T: std::cmp::Ord> {
    pub key: DollarValue,
    pub val: u32,
    pub node_color: NodeColor,
    pub left: Arc<RwLock<NodeType<T>>>,
    pub right: Arc<RwLock<NodeType<T>>>,
    pub parent: Arc<RwLock<NodeType<T>>>,
}

impl <T: std::cmp::Ord> From<NodeType<T>> for Node <T> {
    fn from(value: NodeType<T>) -> Self {
        
        let parent = Arc::new(RwLock::new(NodeType::Empty));
        
        let result = match value {
            NodeType::Empty => Node::new(
                DollarValue::new(999999.1234, 3),
                999,
                parent,
                NodeColor::Black,
            ),
            NodeType::Pointer(node) => node.clone(),
        };
        result
    
    }
}

impl <T: std::cmp::Ord> Node <T> {
    pub fn new(
        key: DollarValue,
        val: u32,
        parent: Arc<RwLock<NodeType<T>>>,
        node_color: NodeColor,
    ) -> Self {
        Node {
            key,
            val,
            parent,
            left: Arc::new(RwLock::new(NodeType::Empty)),
            right: Arc::new(RwLock::new(NodeType::Empty)),
            node_color,
        }
    }

    // SETTERS
    

    
    
    pub fn set_left_child(self, left_child: NodeType<T>) {
        let mut unlocked_left_child = self.left.write().unwrap();
        *unlocked_left_child = left_child;
        drop(unlocked_left_child)
    }

    pub fn set_right_child(self, right_child: NodeType<T>) {
        let mut unlocked_right_child = self.right.write().unwrap();
        *unlocked_right_child = right_child;
        drop(unlocked_right_child)
    }

    pub fn set_parent(self, new_par: NodeType<T>) {
        let mut unlocked_parent = self.parent.write().unwrap();
        *unlocked_parent = new_par;
    }
}

