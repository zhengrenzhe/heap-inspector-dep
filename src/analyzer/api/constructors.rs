use std::collections::HashMap;

use petgraph::Direction;
use serde::{Deserialize, Serialize};

use crate::analyzer::snapshot::consts::NodeType;
use crate::analyzer::snapshot::definition::Node;
use crate::analyzer::Analyzer;

#[derive(Serialize)]
pub struct ConstructorResponse {
    pub total_self_size: u64,
    pub count: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConstructorQuery {
    pub q: Option<Vec<String>>,
}

impl Analyzer {
    pub fn get_neighbors(&self, node: &Node) -> Vec<&Node> {
        let mut arr: Vec<&Node> = Vec::with_capacity(10000);
        for nei in self
            .data_provider
            .graph
            .neighbors_directed(node.graph_node, Direction::Outgoing)
        {
            let val = self.data_provider.graph.node_weight(nei).unwrap();
            let nei_node = &self.data_provider.nodes[*val];
            arr.push(nei_node);
        }
        arr
    }

    pub fn constructors(&self, query: &ConstructorQuery) -> HashMap<&String, ConstructorResponse> {
        match &query.q {
            Some(q) => {
                let q = &q[0];
                for node in &self.data_provider.nodes {
                    if &node.name == q {
                        let neighbors = self.get_neighbors(node);
                        let mut size = 0;
                        for neighbor in neighbors {
                            size += neighbor.self_size;
                            println!("{:?}", neighbor);
                        }
                        println!("total size: {}", size);
                    }
                }
            }
            None => {}
        };

        let mut map: HashMap<&String, ConstructorResponse> = HashMap::new();

        for node in &self.data_provider.nodes {
            let node_type = &node.node_type;
            if [NodeType::Object, NodeType::Native].contains(node_type) {
                let node_name = &node.name;
                match map.get_mut(node_name) {
                    Some(c) => {
                        c.total_self_size += node.self_size;
                        c.count += 1;
                    }
                    None => {
                        map.insert(
                            node_name,
                            ConstructorResponse {
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
