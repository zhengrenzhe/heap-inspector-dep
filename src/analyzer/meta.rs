use serde_json::{json, Value};

use crate::analyzer::analyzer::Analyzer;

impl Analyzer {
    pub fn meta(&self) -> Value {
        json!({
            "edge_count": self.data_provider.edge_count,
            "node_count": self.data_provider.node_count,
            "file_size": self.file_size
        })
    }
}
