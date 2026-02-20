use std::collections::HashMap;

use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

use crate::node::Node;
use crate::policy::{self, Policy};
use crate::relation::{self, Relation};

#[derive(Debug)]
pub struct PadmetSpec {
    pub dic_of_relations_in: HashMap<String, Vec<Relation>>,
    pub dic_of_relations_out: HashMap<String, Vec<Relation>>,
    pub dic_of_nodes: HashMap<String, Node>,
    pub policy: Policy,
    pub info: HashMap<String, HashMap<String, String>>,
}

#[derive(PartialEq)]
pub enum PadmetSection {
    Default,
    Policy,
    DatabaseInformation,
    Nodes,
    Relations,
}

impl PadmetSpec {
    pub fn new(
        dic_of_relations_in: HashMap<String, Vec<Relation>>,
        dic_of_relations_out: HashMap<String, Vec<Relation>>,
        dic_of_nodes: HashMap<String, Node>,
        policy: Policy,
        info: HashMap<String, HashMap<String, String>>,
    ) -> Self {
        PadmetSpec {
            dic_of_relations_in,
            dic_of_relations_out,
            dic_of_nodes,
            policy,
            info,
        }
    }

    pub fn from_file<P>(filename: P) -> io::Result<PadmetSpec>
    where
        P: AsRef<Path>,
    {
        let mut dic_of_relations_in: HashMap<String, Vec<Relation>> = HashMap::new();
        let mut dic_of_relations_out: HashMap<String, Vec<Relation>> = HashMap::new();
        let mut dic_of_nodes: HashMap<String, Node> = HashMap::new();
        let mut policy: Policy = Policy::new();
        let mut info: HashMap<String, HashMap<String, String>> = HashMap::new();

        let mut padmet_section: PadmetSection = PadmetSection::Default;
        let lines = read_lines(filename)?;
        let mut current_data: Option<String> = None;
        for line_res in lines {
            if let Ok(line) = line_res {
                if line != "" {
                    if line == "Data Base informations" {
                        padmet_section = PadmetSection::DatabaseInformation;
                    } else if line == "Policy" {
                        padmet_section = PadmetSection::Policy;
                    } else if line == "Nodes" {
                        padmet_section = PadmetSection::Nodes;
                    } else if line == "Relations" {
                        padmet_section = PadmetSection::Relations
                    } else {
                        if padmet_section == PadmetSection::DatabaseInformation {
                            if !line.contains("\t") {
                                let line = line.replace(":", "");
                                current_data = Some(line.clone());
                                info.insert(line.clone(), HashMap::new());
                            } else {
                                let line = line.replace("\t", "");
                                let mut parts = line.split(":");
                                let key = parts.next().expect("Error parsing PADMet on Data Base informations section. Key not found.").to_owned();
                                let value = parts.next().expect("Error parsing PADMet on Data Base informations section. Value not found.").to_owned();
                                if let Some(ref data_key) = current_data {
                                    let hashmap = info.get_mut(data_key).expect(
                                        "Expect key to be already present in the info hashmap.",
                                    );
                                    hashmap.insert(key, value);
                                }
                            }
                        } else if padmet_section == PadmetSection::Policy {
                            // TODO
                        } else if padmet_section == PadmetSection::Nodes {
                            let mut parts = line.split("\t");
                            let node_type: String =
                                parts.next().expect("Expect node_type").to_owned();
                            let node_id: String = parts.next().expect("Expect node_id").to_owned();
                            let mut node_misc: HashMap<String, Vec<String>> = HashMap::new();
                            let misc_items: Vec<String> = parts.map(|e| e.to_owned()).collect();
                            let mut i = 0;
                            while i + 1 < misc_items.len() {
                                let key = misc_items[i].clone();
                                let value = misc_items[i + 1].clone();
                                if let Some(misc_data) = node_misc.get_mut(&key) {
                                    misc_data.push(value);
                                } else {
                                    node_misc.insert(key, vec![value]);
                                }
                                i += 2;
                            }
                            // Insert node
                            let node = Node::new(node_type, node_id.clone(), node_misc);
                            dic_of_nodes.insert(node_id.clone(), node);
                        } else if padmet_section == PadmetSection::Relations {
                            let mut parts = line.split("\t");
                            let relation_id_in: String =
                                parts.next().expect("Expect relation in node").to_owned();
                            let relation_id_type: String =
                                parts.next().expect("Expect relation in node").to_owned();
                            let relation_id_out: String =
                                parts.next().expect("Expect relation in node").to_owned();
                            let mut relation_misc: HashMap<String, Vec<String>> = HashMap::new();
                            let misc_items: Vec<String> = parts.map(|e| e.to_owned()).collect();
                            let mut i = 0;
                            while i + 1 < misc_items.len() {
                                let key = &misc_items[i];
                                let value = &misc_items[i + 1];
                                if let Some(misc_data) = relation_misc.get_mut(key) {
                                    misc_data.push(value.clone());
                                } else {
                                    relation_misc.insert(key.clone(), vec![value.clone()]);
                                }
                                i += 2;
                            }
                            // Insert relations
                            let relation =
                                Relation::new(relation_id_in.clone(), relation_id_out.clone(), relation_misc);
                            
                            if !dic_of_relations_in.contains_key(&relation_id_in) {
                                dic_of_relations_in.insert(relation_id_in.clone(), Vec::new());
                            } 
                            dic_of_relations_in.get_mut(&relation_id_in).expect("Key should be already be created.").push(relation.clone());
                            if !dic_of_relations_out.contains_key(&relation_id_out) {
                                dic_of_relations_out.insert(relation_id_out.clone(), Vec::new());
                            } 
                            dic_of_relations_out.get_mut(&relation_id_out).expect("Key should be already be created.").push(relation.clone());

                        }
                    }
                }
            }
        }

        let padmet_object = PadmetSpec::new(
            dic_of_relations_in,
            dic_of_relations_out,
            dic_of_nodes,
            policy,
            info,
        );
        Ok(padmet_object)
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
