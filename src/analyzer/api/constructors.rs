use std::collections::HashMap;

use serde::Serialize;

use crate::analyzer::consts::{NODE_TYPE_NATIVE, NODE_TYPE_OBJECT};
use crate::analyzer::Analyzer;

#[derive(Serialize)]
pub struct ConstructorItem {
    pub total_self_size: u64,
    pub count: u64,
}

impl Analyzer {
    pub fn constructors(&self) -> HashMap<&String, ConstructorItem> {
        let mut map: HashMap<&String, ConstructorItem> = HashMap::new();

        for node in &self.data_provider.nodes {
            let node_type = self.data_provider.get_node_type(node);
            if [NODE_TYPE_OBJECT, NODE_TYPE_NATIVE].contains(&&**node_type) {
                let node_name = self.data_provider.get_node_name(node);
                match map.get_mut(node_name) {
                    Some(c) => {
                        c.total_self_size += node.self_size;
                        c.count += 1;
                    }
                    None => {
                        map.insert(
                            node_name,
                            ConstructorItem {
                                total_self_size: node.self_size,
                                count: 1,
                            },
                        );
                    }
                }
            }
        }

        map
    }
}
