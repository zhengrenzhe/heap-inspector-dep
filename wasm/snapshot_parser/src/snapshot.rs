use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EdgeOrNodeType {
    MultiType(Vec<String>),
    SingleType(String),
}

impl EdgeOrNodeType {
    pub fn get_value(&self, index: usize) -> String {
        match self {
            EdgeOrNodeType::MultiType(values) => values.get(index).unwrap().to_string(),
            EdgeOrNodeType::SingleType(value) => value.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnapshotMeta {
    pub edge_fields: Vec<String>,
    pub edge_types: Vec<EdgeOrNodeType>,
    pub location_fields: Vec<String>,
    pub node_fields: Vec<String>,
    pub node_types: Vec<EdgeOrNodeType>,
    pub sample_fields: Vec<String>,
    pub trace_function_info_fields: Vec<String>,
    pub trace_node_fields: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnapshotInfo {
    pub edge_count: u64,
    pub meta: SnapshotMeta,
    pub node_count: u64,
    pub trace_function_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub edges: Vec<u64>,
    pub locations: Vec<u64>,
    pub nodes: Vec<u64>,
    pub samples: Vec<u64>,
    pub snapshot: SnapshotInfo,
    pub strings: Vec<String>,
    pub trace_function_infos: Vec<u64>,
    pub trace_tree: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub node_type: String,
    pub name: String,
    pub id: u64,
    pub self_size: u64,
    pub edge_count: u64,
    pub trace_node_id: u64,
    pub detachedness: u64,
    pub from_edge_index: Vec<u64>,
    pub to_edge_index: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub edge_type: String,
    pub name_or_index: String,
    pub to_node_index: usize,
    pub to_node_id: u64,
    pub from_node_index: usize,
    pub from_node_id: u64,
}
