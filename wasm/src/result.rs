use serde::{Deserialize, Serialize};
use snapshot_parser::snapshot::{Edge, Node};

#[derive(Serialize, Deserialize)]
pub struct SearchedNode {
    pub id: String,
}

impl SearchedNode {
    pub fn from_node(node: &Node) -> Self {
        Self {
            id: node.id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchedEdge {
    pub source: String,
    pub target: String,
}

impl SearchedEdge {
    pub fn from_edge(edge: &Edge) -> Self {
        Self {
            source: edge.from_node_id.to_string(),
            target: edge.to_node_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub nodes: Vec<SearchedNode>,
    pub edges: Vec<SearchedEdge>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeDetailInfo {
    id: String,
    node_type: String,
    node_name: String,
    self_size: u32,
    edge_count: u32,
}

impl NodeDetailInfo {
    pub fn from_node(node: &Node) -> Self {
        Self {
            id: node.id.to_string(),
            node_type: node.node_type.clone(),
            node_name: node.name.clone(),
            self_size: node.self_size,
            edge_count: node.edge_count,
        }
    }
}
