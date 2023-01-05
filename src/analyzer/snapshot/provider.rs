use serde_json::from_slice;

use crate::analyzer::snapshot::definition::{Edge, Node};
use crate::analyzer::snapshot::deserialization::deserialization;
use crate::analyzer::snapshot::serde_mapping::Snapshot;

pub struct SnapshotProvider {
    pub nodes: Vec<Node>,
    pub node_types: Vec<String>,
    pub node_count: u64,

    pub edges: Vec<Edge>,
    pub edge_types: Vec<String>,
    pub edge_count: u64,

    pub strings: Vec<String>,
}

impl SnapshotProvider {
    pub fn from_slice(bytes: &[u8]) -> SnapshotProvider {
        let snapshot: Snapshot = match from_slice(bytes) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        };

        let (nodes, node_types, node_count, edges, edge_types, edge_count, strings) =
            deserialization(snapshot);

        SnapshotProvider {
            nodes,
            node_types,
            node_count,
            edges,
            edge_types,
            edge_count,
            strings,
        }
    }

    pub fn get_node_name(&self, node: &Node) -> &String {
        &self.strings[node.name_index as usize]
    }

    pub fn get_node_type(&self, node: &Node) -> &String {
        &self.node_types[node.node_type_index as usize]
    }
}
