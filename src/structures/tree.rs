use std::sync::{Arc, RwLock};
use crate::structures::node_container::NodeType;

#[derive(Debug)]
pub struct RedBlackTree <T: std::cmp::Ord> {
    pub head: Arc<RwLock<NodeType<T>>>,
}
impl <T: std::cmp::Ord> RedBlackTree <T> {
    pub fn new() -> Self {
        RedBlackTree {
            head: Arc::new(RwLock::new(NodeType::Empty)),
        }
    }
}