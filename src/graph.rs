use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Node {
    pub node_type_index: u32,
    pub name_index: u32,
    pub id: u32,
    pub self_size: u32,
    pub edge_count: u32,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source_node_id: u32,
    pub target_node_id: u32,
    pub edge_type_index: u32,
}

pub struct Graph {
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn get_search_result(self) -> JsValue {
        let nodes = self.nodes[0..10].to_vec();
        let edges = self.edges[0..10].to_vec();
        let r = SearchResult { nodes, edges };
        JsValue::from_serde(&r).expect_throw("Failed parse SearchResult")
    }
}
