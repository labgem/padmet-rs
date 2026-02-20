
use std::collections::HashMap;

use crate::node;

#[derive(Debug, Clone)]
pub struct Relation {
    pub id_in: String,
    pub id_out: String,
    pub misc: HashMap<String, Vec<String>>
}

impl Relation {
    pub fn new(id_in: String, id_out: String, misc: HashMap<String, Vec<String>>) -> Self {
        Relation {
            id_in,
            id_out,
            misc
        }
    }
}