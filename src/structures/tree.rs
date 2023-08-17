use std::sync::{Arc, RwLock};
use crate::structures::node_container::NodeType;

#[derive(Debug)]
pub struct RedBlackTree {
    pub head: Arc<RwLock<NodeType>>,
}
impl RedBlackTree {
    pub fn new() -> Self {
        RedBlackTree {
            head: Arc::new(RwLock::new(NodeType::Empty)),
        }
    }
}