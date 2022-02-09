use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::snapshot::{Edge, Node};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResultNode {
    id: String,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
export interface IResultNode {
    [key: string]: any;
    id: string;
}
"#;

impl ResultNode {
    pub fn from_node(node: &Node) -> Self {
        Self {
            id: node.id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResultEdge {
    source: String,
    target: String,
    edge_index: usize,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE2: &'static str = r#"
export interface IResultEdge {
    [key: string]: any;
    source: string;
    target: string;
    edge_index: number;
}
"#;

impl ResultEdge {
    pub fn from_edge(edge: &Edge) -> Self {
        Self {
            source: edge.from_node_id.to_string(),
            target: edge.to_node_id.to_string(),
            edge_index: edge.edge_index,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    nodes: Vec<ResultNode>,
    edges: Vec<ResultEdge>,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE3: &'static str = r#"
export interface IResult {
    nodes: IResultNode[];
    edges: IResultEdge[];
}
"#;

impl Result {
    pub fn new(nodes: Vec<ResultNode>, edges: Vec<ResultEdge>) -> Self {
        Self { nodes, edges }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeDetailInfo {
    id: String,
    node_type: String,
    node_name: String,
    self_size: u32,
    edge_count: u32,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE4: &'static str = r#"
export interface INodeDetailInfo {
    id: string;
    node_type: string;
    node_name: string;
    self_size: number;
    edge_count: number;
}
"#;

impl NodeDetailInfo {
    pub fn from_node(
        node: &Node,
        strings: &[String],
        strings_len: usize,
        node_types: &[String],
        node_types_len: usize,
    ) -> Self {
        Self {
            id: node.id.to_string(),
            node_type: String::from(node.get_node_type(node_types, node_types_len)),
            node_name: String::from(node.get_node_name(strings, strings_len)),
            self_size: node.self_size,
            edge_count: node.edge_count,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EdgeDetailInfo {
    edge_index: usize,
    edge_type: String,
    edge_name: String,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE5: &'static str = r#"
export interface IEdgeDetailInfo {
    edge_index: number;
    edge_type: string;
    edge_name: string;
}
"#;

impl EdgeDetailInfo {
    pub fn from_edge(
        edge: &Edge,
        strings: &[String],
        strings_len: usize,
        edge_types: &[String],
        edge_types_len: usize,
    ) -> Self {
        Self {
            edge_index: edge.edge_index,
            edge_type: String::from(edge.get_edge_type(edge_types, edge_types_len)),
            edge_name: String::from(edge.get_edge_name(strings, strings_len)),
        }
    }
}
