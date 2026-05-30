
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Relation {
    pub id_in: String,
    pub id_out: String,
    pub relation_type: String,
    pub misc: HashMap<String, Vec<String>>
}

impl Relation {
    pub fn new(id_in: String, id_out: String, relation_type: String, misc: HashMap<String, Vec<String>>) -> Self {
        Relation {
            id_in,
            id_out,
            relation_type,
            misc
        }
    }
}