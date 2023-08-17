use crate::structures::{
    active_node::{Node, NodeColor},
    node_container::NodeType,
    tree::RedBlackTree,
};
use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};
use tracing::{info, trace, error, debug};


impl <T: std::cmp::Ord> RedBlackTree <T> {
    pub fn assign_color (
        target_lock: Arc<RwLock<NodeType<T>>>,
        color: NodeColor,
    ) -> Result<()> {
        let mut target_type = target_lock.write().unwrap();
        let mut new_node = Node::from(target_type.clone());
        new_node.node_color = color;
        *target_type = NodeType::from(new_node);
        Ok(())
    }
    
    pub fn assign_left_child(
        target_node_lock: Arc<RwLock<NodeType<T>>>,
        new_left_child: Arc<RwLock<NodeType<T>>>,
    ) -> Result<()> {
        let mut target_node_type = target_node_lock.write().unwrap();
        let res = match &*target_node_type {
            NodeType::Pointer(node) => {
                let mut new_node = node.clone();
                new_node.left = Arc::clone(&new_left_child);
                Ok(NodeType::from(new_node))
            }
            _ => Err(()),
        };
        if let Ok(new_node) = res {
            *target_node_type = new_node;
            return Ok(());
        }

        Err(anyhow!("Error assigning left child"))
    }
    
    pub fn assign_right_child(
        target_node_lock: Arc<RwLock<NodeType<T>>>,
        new_right_child: Arc<RwLock<NodeType<T>>>,
    ) -> Result<()> {
        let mut target_node_type = target_node_lock.write().unwrap();
        let res = match &*target_node_type {
            NodeType::Pointer(node) => {
                let mut new_node = node.clone();
                new_node.right = Arc::clone(&new_right_child);
                Ok(NodeType::from(new_node))
            }
            _ => Err(()),
        };
        if let Ok(new_node) = res {
            *target_node_type = new_node;
            return Ok(());
        }
        Err(anyhow!("Error assigning right child"))
    }
    
    pub fn assign_parent(
        target_node_lock: Arc<RwLock<NodeType<T>>>,
        new_parent: Arc<RwLock<NodeType<T>>>,
    ) -> Result<()> {
        let mut target_node_type = target_node_lock.write().unwrap();
        let res = match &*target_node_type {
            NodeType::Pointer(node) => {
                let mut new_node = node.clone();
                new_node.parent = Arc::clone(&new_parent);
                Ok(NodeType::from(new_node))
            }
            _ => Err(()),
        };
        if let Ok(new_node) = res {
            *target_node_type = new_node;
            return Ok(());
        }
        Err(anyhow!("Error assigning parent"))
    }
    
    pub fn assign_root(&mut self, new_head: Arc<RwLock<NodeType<T>>>) {
        self.head = new_head;
    }
}