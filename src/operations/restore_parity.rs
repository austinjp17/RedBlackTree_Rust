use crate::{
    DollarValue,  
    structures::{
        active_node::{Node, NodeColor},
        node_container::NodeType,
        tree::RedBlackTree,
    },
 };
use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};
use tracing::{info, trace, error, debug};


impl RedBlackTree {
    #[tracing::instrument]
    pub fn insertion_restore(&mut self, target_key: DollarValue) {
        
        info!("Checking Insertion: {:?}", &target_key);
        // ! Target Read Lock Taken
        let target_lock = self.search(target_key).unwrap();
        let read_target_type = target_lock.read().unwrap();
        let read_target_node = Node::from(read_target_type.clone());
        drop(read_target_type);

        // ! Parent Read Lock Taken
        let parent_lock = Arc::clone(&read_target_node.parent);
        let read_parent_node_type = parent_lock.read().unwrap();
        let read_parent_node = Node::from(read_parent_node_type.clone());

        // Read Parent Color Lock Taken
        

        // Case: Root
        if *read_parent_node_type == NodeType::Empty {
            // ! Parent Read Dropped
            drop(read_parent_node_type);

            
            match (&read_target_node).node_color {
                NodeColor::Black => {
                    return;
                }
                NodeColor::Red => {
                    let _ = Self::assign_color(
                        Arc::clone(&target_lock),
                        NodeColor::Black
                    );
                }
            };
        }
        
        else if *read_parent_node_type != NodeType::Empty {
            // ! Drop Parents Read
            drop(read_parent_node_type);
            // If parent red
            match (&read_parent_node).node_color {
                // Case 1
                NodeColor::Black => {
                    info!("Parent Node is Black");
                    return;
                }
                // Case 2
                NodeColor::Red => {
                    info!("Parent Node is Red");
                    // If uncle red
                    //
                    // ! GP Read Taken
                    let grandparent_lock = Arc::clone(&read_parent_node.parent);
                    let grandparent_node_type = grandparent_lock.read().unwrap();
                    let grandparent_node = Node::from(grandparent_node_type.clone());
                    drop(grandparent_node_type);
                    debug!("Grandparent matched: {:?}", grandparent_node.key);

                    // Init Uncle Node fields
                    let uncle_lock;
                    let uncle_node;
                    
                    // Find Uncle Node
                    // ! Uncle Read Taken && Dropped
                    if grandparent_node.key < read_parent_node.key {
                        uncle_lock = Arc::clone(&grandparent_node.left);
                        let uncle_node_type = uncle_lock.read().unwrap();
                        uncle_node = Node::from(uncle_node_type.clone());
                        
                    } else
                    /* Parent to Left of Grandparent */
                    {
                        uncle_lock = Arc::clone(&grandparent_node.right);
                        let uncle_node_type = uncle_lock.read().unwrap();
                        uncle_node = Node::from(uncle_node_type.clone());
                    }

                    debug!(
                        "Uncle Matched: {:?}\nParent Matched: {:?}",
                        uncle_node.key, read_parent_node.key
                    );

                    // ! 0 alive read/write locks
                    match (&uncle_node).node_color {
                        // Parent && Uncle Red
                        NodeColor::Red => {
                            
                            info!("Red Uncle -> Flip red uncle && parent to black");
                            // Change par color && uncle color to Black
                            let _ = Self::assign_color(uncle_lock.clone(), NodeColor::Black);
                            let _ = Self::assign_color(parent_lock.clone(), NodeColor::Black);
                            let _ = Self::assign_color(grandparent_lock.clone(), NodeColor::Red);
                            // Recursive statement: reruns from gp pov
                            self.insertion_restore(grandparent_node.key);
                            // TODO: Change x = x’s grandparent, repeat above for new x.
                        }
                        NodeColor::Black => {
                            // LL || LR || RR || RL Case handling
                            info!("Black uncle -> 4 cases");

                            // Case 2a: LL
                            if read_parent_node.key < grandparent_node.key
                                && read_target_node.key < read_parent_node.key
                            {
                                // Right Rotation
                                info!("Case A: Right Rotation");
                                self.right_rotation(target_lock.clone());
                            }
                            // Case 2b: LR
                            else if read_parent_node.key < grandparent_node.key
                                && read_target_node.key > read_parent_node.key
                            {
                                info!("Case B: Left Right Condition");
                                self.left_right_rotation(Arc::clone(&target_lock));
                            }
                            // Case 2c: RR
                            else if read_parent_node.key > grandparent_node.key
                                && read_target_node.key > read_parent_node.key
                            {
                                info!("Case C: Left Rotation");
                                // Left Rotation
                                self.left_rotation(target_lock.clone())
                            }
                            // Case 2d: RL
                            else if read_parent_node.key > grandparent_node.key
                                && read_target_node.key < read_parent_node.key
                            {
                                info!("Case D: Right Left Condition");
                                // self.right_left_rotation();
                                self.right_left_rotation(target_lock.clone());
                            }
                        }
                    }
                }
            }
        }
    }

}