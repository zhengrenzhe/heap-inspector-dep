use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub node_type_index: u32,
    pub name_index: u32,
    pub id: u32,
    pub self_size: u32,
    pub edge_count: u32,
    pub trace_node_id: u32,
    pub detachedness: u32,
}

impl Node {
    pub fn get_graph_data(&self) -> NodeGraphData {
        NodeGraphData {
            id: self.id.to_string(),
        }
    }
}

pub struct Edge {
    pub source_node_id: u32,
    pub target_node_id: u32,
    pub edge_type_index: u32,
}

impl Edge {
    pub fn get_graph_data(&self) -> EdgeGraphData {
        EdgeGraphData {
            source: self.source_node_id.to_string(),
            target: self.target_node_id.to_string(),
        }
    }
}

pub struct Graph {
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeGraphData {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct NodeDetailData {
    pub node_id: String,
    pub node_type: String,
    pub node_name: String,
    pub self_size: u32,
    pub edge_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct EdgeGraphData {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub nodes: Vec<NodeGraphData>,
    pub edges: Vec<EdgeGraphData>,
}
