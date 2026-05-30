//! The main object to access the data of a PADMet file

/* std use */
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::path::Path;

/* crate use */

/* project use */
use crate::io::read_lines;
use crate::node::Node;
use crate::policy::Policy;
use crate::relation::Relation;

#[derive(Debug, Clone)]
pub struct PadmetSpec {
    /// A dictionnary from node identifier to the list of relations coming from this node
    pub dic_of_relations_in: HashMap<String, Vec<Relation>>,
    /// A dictionnaru from node identifier to the list of relations coming to this node
    pub dic_of_relations_out: HashMap<String, Vec<Relation>>,
    /// A dictionnary from node identifier to the node struct
    pub dic_of_nodes: HashMap<String, Node>,
    /// The policy
    pub policy: Policy,
    /// Metadata of the PADMet object
    pub info: HashMap<String, HashMap<String, String>>,
}

/// Flag the section of the PADMet file
#[derive(PartialEq)]
pub enum PadmetSection {
    Default,
    Policy,
    DatabaseInformation,
    Nodes,
    Relations,
}

impl PadmetSpec {
    /// Create a PadmetSpec
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

    /// Load a PadmetSpec from a file
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
        for line in lines.map_while(Result::ok) {
            if !line.is_empty() {
                // Check in which section of the PADMet file we are.
                if line == "Data Base informations" {
                    padmet_section = PadmetSection::DatabaseInformation;
                } else if line == "Policy" {
                    padmet_section = PadmetSection::Policy;
                } else if line == "Nodes" {
                    padmet_section = PadmetSection::Nodes;
                } else if line == "Relations" {
                    padmet_section = PadmetSection::Relations;
                // Behave differently depending on the section
                } else if padmet_section == PadmetSection::DatabaseInformation {
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
                            let hashmap = info
                                .get_mut(data_key)
                                .expect("Expect key to be already present in the info hashmap.");
                            hashmap.insert(key, value);
                        }
                    }
                } else if padmet_section == PadmetSection::Policy {
                    // TODO
                } else if padmet_section == PadmetSection::Nodes {
                    let mut parts = line.split("\t");
                    let node_type: String = parts.next().expect("Expect node_type").to_owned();
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
                    let node = Node::new(node_type.clone(), node_id.clone(), node_misc);
                    dic_of_nodes.insert(node_id.clone(), node);
                } else if padmet_section == PadmetSection::Relations {
                    let mut parts = line.split("\t");
                    let relation_id_in: String =
                        parts.next().expect("Expect relation in node").to_owned();
                    let relation_type: String =
                        parts.next().expect("Expect relation type").to_owned();
                    let relation_id_out: String =
                        parts.next().expect("Expect relation out node").to_owned();
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
                    let relation = Relation::new(
                        relation_id_in.clone(),
                        relation_id_out.clone(),
                        relation_type,
                        relation_misc,
                    );

                    if !dic_of_relations_in.contains_key(&relation_id_in) {
                        dic_of_relations_in.insert(relation_id_in.clone(), Vec::new());
                    }
                    dic_of_relations_in
                        .get_mut(&relation_id_in)
                        .expect("Key should be already be created.")
                        .push(relation.clone());
                    if !dic_of_relations_out.contains_key(&relation_id_out) {
                        dic_of_relations_out.insert(relation_id_out.clone(), Vec::new());
                    }
                    dic_of_relations_out
                        .get_mut(&relation_id_out)
                        .expect("Key should be already be created.")
                        .push(relation.clone());
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

    /// Get a set of all relations in the padmet object
    pub fn get_all_relations(&self) -> Vec<Relation> {
        let mut all_relations: Vec<Relation> = Vec::new();
        for relations in self.dic_of_relations_in.values() {
            for relation in relations {
                all_relations.push(relation.clone());
            }
        }
        for relations in self.dic_of_relations_out.values() {
            for relation in relations {
                all_relations.push(relation.clone());
            }
        }
        // Deduplicate ?
        all_relations
    }

    /// Get the list of relations of a certain type, coming from a certain node
    pub fn get_relations_type_id_in(
        &self,
        relation_type: &String,
        relation_id_in: &String,
    ) -> Vec<Relation> {
        if let Some(relations) = self.dic_of_relations_in.get(relation_id_in) {
            let relations_of_type: Vec<Relation> = relations
                .iter()
                .filter(|&r| r.relation_type == *relation_type)
                .cloned()
                .collect();
            return relations_of_type;
        }
        Vec::new()
    }

    /// Get the list of relations of a certain type, leading to a certain node
    pub fn get_relations_type_id_out(
        &self,
        relation_type: &String,
        relation_id_out: &String,
    ) -> Vec<Relation> {
        if let Some(relations) = self.dic_of_relations_out.get(relation_id_out) {
            let relations_of_type: Vec<Relation> = relations
                .iter()
                .filter(|&r| r.relation_type == *relation_type)
                .cloned()
                .collect();
            return relations_of_type;
        }
        Vec::new()
    }

    /// Get relation from relation_id_in to relation_id_out nodes and with a specific relation_type
    pub fn get_relations(
        self,
        relation_id_in: &String,
        relation_type: &String,
        relation_id_out: &String,
    ) -> Vec<Relation> {
        if let Some(relations) = self.dic_of_relations_in.get(relation_id_in) {
            let relations_filtered = relations
                .iter()
                .filter(|&r| r.relation_type == *relation_type && r.id_out == *relation_id_out)
                .cloned()
                .collect();
            return relations_filtered;
        }
        Vec::new()
    }

    pub fn get_nodes_of_type(&self, node_type: &str) -> Vec<Node> {
        self.dic_of_nodes
            .values()
            .filter(|node| node.node_type == node_type)
            .cloned()
            .collect()
    }

    /// Get all the compounds from a padmet instance.
    pub fn get_compounds(&self) -> Vec<Node> {
        self.get_nodes_of_type("compound")
    }

    /// Get all the reactions from a padmet instance
    pub fn get_reactions(&self) -> Vec<Node> {
        self.get_nodes_of_type("reaction")
    }

    /// Get all the genes from a padmet instance
    pub fn get_genes(&self) -> Vec<Node> {
        self.get_nodes_of_type("gene")
    }

    /// Get all the pathways from a padmet instance
    pub fn get_pathways(&self) -> Vec<Node> {
        self.get_nodes_of_type("pathway")
    }

    /// Get all the pathways from a padmet instance with their set of reactions.
    pub fn get_pathways_reactions(&self) -> HashMap<String, HashSet<String>> {
        let pathways = self.get_pathways();
        let mut pathway_to_reactions: HashMap<String, HashSet<String>> = HashMap::new();
        for pathway in pathways {
            let reactions_relations =
                self.get_relations_type_id_out(&"is_in_pathway".to_owned(), &(pathway.node_id));
            let reactions_id: Vec<String> = reactions_relations
                .into_iter()
                .map(|relation| relation.id_in)
                .collect();
            let reaction_set: HashSet<String> = HashSet::from_iter(reactions_id.iter().cloned());
            pathway_to_reactions.insert(pathway.node_id, reaction_set);
        }
        pathway_to_reactions
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_open_padmet() {
        let padmet_test_file_1: PathBuf =
            PathBuf::from("vendor/padmet/tests/test_data/padmet/padmet_1.padmet");
        let padmet_object: PadmetSpec = PadmetSpec::from_file(padmet_test_file_1).unwrap();
        let nodes = &padmet_object.dic_of_nodes;
        dbg!(nodes);
        let reactions = padmet_object.get_reactions();
        assert!(reactions.len() > 0);

        let pathways = padmet_object.get_pathways();
        assert!(pathways.len() > 0);
    }
    
    #[test]
    fn test_get_reactions_of_pathway() {
                let padmet_test_file_1: PathBuf =
            PathBuf::from("vendor/padmet/tests/test_data/padmet/padmet_1.padmet");
        let padmet_object: PadmetSpec = PadmetSpec::from_file(padmet_test_file_1).unwrap();
        let pathways = padmet_object.get_pathways();
        let pathway_reactions = padmet_object.get_pathways_reactions();
        assert_eq!(pathways.len(), pathway_reactions.len());
        assert!(pathway_reactions.get("FAO-PWY").expect("pathway FAO-PWY should exist in the test padmet file").len() > 0);
    }

}
