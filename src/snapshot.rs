use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SnapshotData {
    pub edges: Vec<u32>,
    pub locations: Vec<u32>,
    pub nodes: Vec<u32>,
    pub samples: Vec<u32>,
    pub snapshot: SnapshotSnapshot,
    pub strings: Vec<String>,
    pub trace_function_infos: Vec<u32>,
    pub trace_tree: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotSnapshot {
    pub edge_count: u32,
    pub meta: SnapshotMeta,
    pub node_count: u32,
    pub trace_function_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotMeta {
    pub edge_fields: Vec<String>,
    pub edge_types: Vec<Type>,
    pub location_fields: Vec<String>,
    pub node_fields: Vec<String>,
    pub node_types: Vec<Type>,
    pub sample_fields: Vec<String>,
    pub trace_function_info_fields: Vec<String>,
    pub trace_node_fields: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Type {
    MultiType(Vec<String>),
    SingleType(String),
}
