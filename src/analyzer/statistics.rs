use std::collections::HashMap;

use serde_json::{json, Value};

use crate::analyzer::analyzer::Analyzer;

impl Analyzer {
    pub fn statistics(&self) -> Value {
        let node_types = &self.data_provider.node_types;
        let mut node_types_map: HashMap<&String, usize> = HashMap::new();
        let mut total_bytes: u64 = 0;

        for node in &self.data_provider.nodes {
            let node_type = &node_types[node.node_type_index];
            let new_val = match node_types_map.get(node_type) {
                Some(val) => val + 1,
                None => 1,
            };
            node_types_map.insert(node_type, new_val);
            total_bytes += node.self_size as u64
        }

        json!({ "percent": node_types_map, "total_bytes": total_bytes })
    }
}
