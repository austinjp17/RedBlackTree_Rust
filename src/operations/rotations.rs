use crate::{
    DollarValue,  
    structures::{
        active_node::{Node, NodeColor},
        node_container::NodeType,
        tree::RedBlackTree,
    }
 };

use std::sync::{Arc, RwLock};

impl RedBlackTree {

    pub fn right_left_rotation(&mut self, pusher_lock: Arc<RwLock<NodeType>>) {
        // ! Pusher Read Taken && Dropped
        let pusher_node_type = pusher_lock.read().unwrap();
        let pusher_node = Node::from(pusher_node_type.clone());
        drop(pusher_node_type);

        // ! Par Read Taken && Dropped
        let par_lock = pusher_node.parent;
        let par_type = par_lock.read().unwrap();
        let par_node = Node::from(par_type.clone());
        drop(par_type);

        // ! GP Read Taken && Dropped
        let gp_lock = par_node.parent;
        let gp_type = gp_lock.read().unwrap();
        let gp_node = Node::from(gp_type.clone());
        drop(gp_type);

        let _ = Self::assign_left_child(par_lock.clone(), Arc::new(RwLock::new(NodeType::Empty)));

        let _ = Self::assign_right_child(gp_lock.clone(), pusher_lock.clone());

        let _ = Self::assign_right_child(pusher_lock.clone(), par_lock.clone());

        let _ = Self::assign_parent(pusher_lock.clone(), gp_lock.clone());

        let _ = Self::assign_parent(par_lock.clone(), pusher_lock.clone());

        // Rotation 2: Left
        // self.left_rotation(pusher_lock);

        let _ = Self::assign_parent(pusher_lock.clone(), gp_node.parent.clone());

        let _ = Self::assign_parent(gp_lock.clone(), pusher_lock.clone());

        let _ = Self::assign_left_child(pusher_lock.clone(), gp_lock.clone());

        let _ = Self::assign_right_child(gp_lock.clone(), Arc::new(RwLock::new(NodeType::Empty)));
        
        // Recolor Parent Black and Children Red
        let _ = Self::assign_color(
            Arc::clone(&pusher_lock), 
            NodeColor::Black
        );
        let _ = Self::assign_color(
            Arc::clone(&par_lock),
            NodeColor::Red
        );
        let _ = Self::assign_color(
            Arc::clone(&gp_lock),
            NodeColor::Red
        );

        let new_par_lock = Arc::clone(&gp_node.parent);
        let new_par_type = new_par_lock.read().unwrap();
        match &*new_par_type {
            NodeType::Pointer(_) => {}
            NodeType::Empty => {
                drop(new_par_type);
                self.assign_root(Arc::clone(&pusher_lock));
            }
        } 



    }

    pub fn left_right_rotation(&mut self, pusher_lock: Arc<RwLock<NodeType>>) {
        

        // ! Pusher Read Taken && Dropped
        let pusher_node_type = pusher_lock.read().unwrap();
        let pusher_node = Node::from(pusher_node_type.clone());
        drop(pusher_node_type);

        // ! Par Read Taken && Dropped
        let par_lock = pusher_node.parent;
        let par_type = par_lock.read().unwrap();
        let par_node = Node::from(par_type.clone());
        drop(par_type);

        // ! GP Read Taken && Dropped
        let gp_lock = par_node.parent;
        let gp_type = gp_lock.read().unwrap();
        let gp_node = Node::from(gp_type.clone());
        drop(gp_type);

        // Rotation 1: Left

        // 'parent' becomes left child of pusher
        let _ = Self::assign_left_child(pusher_lock.clone(), par_lock.clone());

        // change child parent to 'parent' parent
        let _ = Self::assign_parent(pusher_lock.clone(), gp_lock.clone());

        // 'parent' becomes child to pusher
        let _ = Self::assign_parent(par_lock.clone(), pusher_lock.clone());

        // remove child pointer on 'parent'
        let _ = Self::assign_right_child(par_lock.clone(), Arc::new(RwLock::new(NodeType::Empty)));

        // set gp left child to 'child'
        let _ = Self::assign_left_child(gp_lock.clone(), pusher_lock.clone());

        // Rotation 2: Right
        // self.right_rotation(pusher_lock.clone());

        // ! Pusher Read Taken && Dropped
        // let pusher_lock = par_lock;
        // let par_lock = pusher_lock;
        // let gp_lock = gp_lock;

        let _ = Self::assign_right_child(pusher_lock.clone(), gp_lock.clone());

        let _ = Self::assign_left_child(
            Arc::clone(&gp_lock),
            Arc::new(RwLock::new(NodeType::Empty))
        );

        let _ = Self::assign_parent(
            Arc::clone(&gp_lock), 
            Arc::clone(&pusher_lock)
        );
        let _ = Self::assign_parent(
            Arc::clone(&pusher_lock),
            Arc::clone(&gp_node.parent)
        );

        // Recolor Parent Black and Children Red
        let _ = Self::assign_color(
            Arc::clone(&pusher_lock), 
            NodeColor::Black
        );
        let _ = Self::assign_color(
            Arc::clone(&par_lock),
            NodeColor::Red
        );
        let _ = Self::assign_color(
            Arc::clone(&gp_lock),
            NodeColor::Red
        );

        
        // If Promoted Node's parent is Empty, node is new head
        // ! GP & GGP Par Read Taken && Dropped
        let new_par_lock = Arc::clone(&gp_node.parent);
        let new_par_type = new_par_lock.read().unwrap();
        let ggp_lock = gp_node.parent;
        let ggp_type = ggp_lock.read().unwrap();
        let ggp_node = Node::from(ggp_type.clone());
        drop(ggp_type);
        match &*new_par_type {
            NodeType::Pointer(_) => {
                drop(new_par_type);
                if gp_node.key > ggp_node.key {
                    let _ = Self::assign_right_child(ggp_lock.clone(), pusher_lock.clone());
                } else {
                    let _ = Self::assign_left_child(ggp_lock.clone(),pusher_lock.clone());
                }
            }
            NodeType::Empty => {
                drop(new_par_type);
                self.assign_root(Arc::clone(&pusher_lock));
            }
        } 



    }

    pub fn right_rotation(&mut self, pusher_lock: Arc<RwLock<NodeType>>) {
        
        // ! Pusher Read Taken && Dropped
        let pusher_node_type = pusher_lock.read().unwrap();
        let pusher_node = Node::from(pusher_node_type.clone());
        drop(pusher_node_type);

        // ! Par Read Taken && Dropped
        let par_lock = pusher_node.parent;
        let par_type = par_lock.read().unwrap();
        let par_node = Node::from(par_type.clone());
        drop(par_type);
        
        // ! GP Read Taken && Dropped
        let gp_lock = par_node.parent;
        let gp_type = gp_lock.read().unwrap();
        let gp_node = Node::from(gp_type.clone());
        drop(gp_type);
        
        // Perform Rotation: MISSING ONE?
        let _ = Self::assign_right_child(
            Arc::clone(&par_lock), 
            Arc::clone(&gp_lock)
        );

        let _ = Self::assign_left_child(
            Arc::clone(&gp_lock),
            Arc::new(RwLock::new(NodeType::Empty))
        );

        let _ = Self::assign_parent(
            Arc::clone(&gp_lock), 
            Arc::clone(&par_lock)
        );
        let _ = Self::assign_parent(
            Arc::clone(&par_lock),
            Arc::clone(&gp_node.parent)
        );

        
        
        // Recolor Parent Black and Children Red

        let _ = Self::assign_color(
            Arc::clone(&par_lock), 
            NodeColor::Black
        );
        let _ = Self::assign_color(
            Arc::clone(&gp_lock),
            NodeColor::Red
        );
        let _ = Self::assign_color(
            Arc::clone(&pusher_lock),
            NodeColor::Red
        );
        

        // If Promoted Node's parent is Empty, node is new head
        // ! GP Par Read Taken && Dropped
        let new_par_lock = Arc::clone(&gp_node.parent);
        let new_par_type = new_par_lock.read().unwrap();
        match &*new_par_type {
            NodeType::Pointer(_) => {}
            NodeType::Empty => {
                drop(new_par_type);
                self.assign_root(Arc::clone(&par_lock));
            }
        }    
        // self.assign_root();
    }

    pub fn left_rotation(&mut self, pusher_lock: Arc<RwLock<NodeType>>) {
        
        // Q: NODES ARE MEMORY CLONES SO CHANGES WON'T AFFECT TREE?
        // is dereference copying?
        // Get Rotation initiator
        // ! Pusher Read Lock Taken && Dropped
        let pusher_node_type = pusher_lock.read().unwrap();
        let pusher_node = Node::from(pusher_node_type.clone());
        drop(pusher_node_type);

        // ! Parent Read Lock Taken && Dropped
        let parent_lock = Arc::clone(&pusher_node.parent);
        let parent_node_type = parent_lock.read().unwrap();
        let parent_node = Node::from(parent_node_type.clone());
        drop(parent_node_type);

        // Must drop parent read before getting gp write or will deadlock
        // ! GP Read lock taken && dropped
        let gp_lock = Arc::clone(&parent_node.parent);
        let gp_node_type = gp_lock.read().unwrap();
        let gp_node = Node::from(gp_node_type.clone());
        drop(gp_node_type);

        let ggp_lock = gp_node.parent.clone();
        let ggp_type = ggp_lock.read().unwrap();
        let ggp_node = Node::from(ggp_type.clone());
        drop(ggp_type);

        // Can't use methods? 'cannot move out of dereference, NodeType doesn't implement Copy
        // gp_node_type.assign_left_child(Arc::clone(&parent_lock));

        // Perform Rotation
        // GP becomes left child of pusher parent
        let _ = Self::assign_left_child(
            parent_lock.clone(),
            gp_lock.clone()
        );

        // GP Right Child becomes Empty
        let _ = Self::assign_right_child(
            gp_lock.clone(),
            Arc::new(RwLock::new(NodeType::Empty))
        );

        // // Make target par = gp par
        let _ = Self::assign_parent(
            Arc::clone(&gp_lock), 
            Arc::clone(&parent_lock)
        );

        // // Make target par = to gp par
        let _ = Self::assign_parent(
            parent_lock.clone(), 
            gp_node.parent.clone()
        );

        // Re-color
        let _ = Self::assign_color(
            Arc::clone(&parent_lock), 
            NodeColor::Black
        );
        let _ = Self::assign_color(
            Arc::clone(&gp_lock),
            NodeColor::Red
        );
        let _ = Self::assign_color(
            Arc::clone(&pusher_lock),
            NodeColor::Red
        );


        // Check if promoted node is now head
        let new_par_lock = Arc::clone(&gp_node.parent);
        let new_par_type = new_par_lock.read().unwrap();
        
        match &*new_par_type {
            NodeType::Pointer(_) => {
                drop(new_par_type);
                // If new parent not root, then ggp must now point to old parent instead of gp
                // need to find whether parent should be left || right child of ggp

                // GP greater than (right of) GGP
                
                if gp_node.key > ggp_node.key {
                    let _ = Self::assign_right_child(ggp_lock.clone(), parent_lock.clone());
                } else {
                    let _ = Self::assign_left_child(ggp_lock.clone(),parent_lock.clone());
                }
            }
            NodeType::Empty => {
                drop(new_par_type);
                self.assign_root(Arc::clone(&parent_lock));
            }
        }
        
    }

}