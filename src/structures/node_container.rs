use crate::structures::active_node::Node;

#[derive(Debug, Clone)]
pub enum NodeType {
    Pointer(Node),
    Empty,
}


impl From<Node> for NodeType {
    fn from(value: Node) -> Self {
        NodeType::Pointer(value)
    }
}

impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        let mut empty = 0;
        let mut pointers = 0;
        for node_t in [self, other].into_iter() {
            match node_t {
                Self::Pointer(_) => pointers += 1,
                Self::Empty => empty += 1,
            }
        }
        if empty == 2 || pointers == 2 {
            return true;
        }
        false
    }
}
