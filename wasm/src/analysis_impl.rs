use crate::analysis::SnapshotAnalysis;
use crate::result::{Result, ResultEdge, ResultNode};
use snapshot_parser::snapshot::{Edge, Node};
use wasm_bindgen::prelude::*;

impl SnapshotAnalysis {
    pub(crate) fn nodes_to_result_node(nodes: &[&Node]) -> Vec<ResultNode> {
        nodes
            .iter()
            .map(|node| ResultNode::from_node(node))
            .collect()
    }

    pub(crate) fn edges_to_result_edge(edges: &[&Edge]) -> Vec<ResultEdge> {
        edges
            .iter()
            .map(|edge| ResultEdge::from_edge(edge))
            .collect()
    }

    pub(crate) fn convert_graph_to_js(nodes: &[&Node], edges: &[&Edge]) -> JsValue {
        JsValue::from_serde(&Result::new(
            SnapshotAnalysis::nodes_to_result_node(nodes),
            SnapshotAnalysis::edges_to_result_edge(edges),
        ))
        .expect_throw("Failed parse SearchResult")
    }
}
