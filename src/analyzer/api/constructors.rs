use std::collections::HashSet;

use crate::analyzer::consts::{NODE_TYPE_NATIVE, NODE_TYPE_OBJECT};
use crate::analyzer::Analyzer;

impl Analyzer {
    pub fn constructors(&self) -> HashSet<&String> {
        let mut hash: HashSet<&String> = HashSet::new();

        for node in &self.data_provider.nodes {
            let node_type = self.data_provider.get_node_type(node);
            if [NODE_TYPE_OBJECT, NODE_TYPE_NATIVE].contains(&&**node_type) {
                let node_name = self.data_provider.get_node_name(node);
                hash.insert(node_name);
            }
        }

        hash
    }
}
