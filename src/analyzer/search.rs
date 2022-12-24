use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::analyzer::analyzer::Analyzer;

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchQuery {
    filter_from: Vec<String>,
    filter_name: String,
    self_size_mode: String,
    self_size: usize,
    retained_size_mode: String,
    retained_size: usize,
    depth: usize,
}

impl Analyzer {
    pub fn search(&self, query: &SearchQuery) -> Value {
        json!({"":""})
    }
}
