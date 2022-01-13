use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub struct Node {
    pub node_type_index: u32,
    pub name_index: u32,
    pub id: u32,
    pub self_size: u32,
    pub edge_count: u32,
}

impl Node {
    pub fn dump(&self) -> NodeDump {
        NodeDump {
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
    pub fn dump(&self) -> EdgeDump {
        EdgeDump {
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
pub struct NodeDump {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct EdgeDump {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    nodes: Vec<NodeDump>,
    edges: Vec<EdgeDump>,
}

impl Graph {
    pub fn get_search_result(self) -> JsValue {
        let nodes = self.nodes[0..10].iter().map(|x| x.dump()).collect();
        let edges = self.edges[0..10].iter().map(|x| x.dump()).collect();
        let r = SearchResult { nodes, edges };
        JsValue::from_serde(&r).expect_throw("Failed parse SearchResult")
    }
}
