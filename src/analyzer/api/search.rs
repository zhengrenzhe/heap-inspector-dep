use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::analyzer::analyzer::Analyzer;

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchQuery {
    filter_from: Option<Vec<String>>,
    filter_name: Option<String>,
    self_size_mode: Option<String>,
    self_size: Option<usize>,
    retained_size_mode: Option<String>,
    retained_size: Option<usize>,
    depth: Option<usize>,
    depth_mode: Option<String>,
    node_types: Option<Vec<String>>,
}

impl Analyzer {
    pub fn search(&self, query: &SearchQuery) -> Value {
        for node in &self.data_provider.nodes[0..10] {
            println!("{:?}", node);
        }
        let nodes = &self.data_provider.nodes[0..1000];
        let edges = &self.data_provider.edges[0..1000];
        json!({"nodes":nodes, "edges": edges})
    }
}
