use crate::snapshot::{Edge, Node};

pub struct SnapshotProvider {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,

    pub strings: Vec<String>,

    pub edge_count: u32,
    pub node_count: u32,

    pub edge_types: Vec<String>,
    pub node_types: Vec<String>,
}
