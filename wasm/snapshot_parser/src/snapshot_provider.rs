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

impl SnapshotProvider {
    pub fn get_strings(&self) -> (&Vec<String>, usize) {
        (&self.strings, self.strings.len())
    }

    pub fn get_node_types(&self) -> (&Vec<String>, usize) {
        (&self.node_types, self.node_types.len())
    }

    pub fn get_edge_types(&self) -> (&Vec<String>, usize) {
        (&self.edge_types, self.edge_types.len())
    }
}
