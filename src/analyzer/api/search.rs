use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::analyzer::Analyzer;

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
        println!("{:?}", query);
        json!({"nodes":[], "edges": []})
    }
}
