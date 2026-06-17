use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: String,
    pub node_id: String,
    pub node_misc: HashMap<String, Vec<String>>,
}

impl Node {
    pub fn new(
        node_type: String,
        node_id: String,
        node_misc: HashMap<String, Vec<String>>,
    ) -> Self {
        Node {
            node_type,
            node_id,
            node_misc,
        }
    }
}
