use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EdgeOrNodeType {
    MultiType(Vec<String>),
    SingleType(String),
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
