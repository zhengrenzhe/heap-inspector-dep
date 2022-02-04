use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::filter::FilterCondition;
use crate::log::Log;
use crate::result::{NodeDetailInfo, Result, ResultEdge, ResultNode};
use snapshot_parser::reader::Reader;
use snapshot_parser::snapshot::{Edge, Node, Snapshot};

#[wasm_bindgen]
pub struct SnapshotAnalysis {
    snapshot: Snapshot,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[wasm_bindgen]
impl SnapshotAnalysis {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Self {
        Log::info2("reading", format!("{} bytes", bytes.len()));
        let raw_bytes = Uint8Array::from(bytes).to_vec();
        Log::info("read-done");

        Log::info("decoding");
        let snapshot = Reader::from_bytes(&raw_bytes);
        Log::info("decode-done");

        Log::info("parsing");
        let (nodes, edges) = snapshot.get_graph();
        assert_eq!(nodes.len() as u32, snapshot.snapshot.node_count);
        assert_eq!(edges.len() as u32, snapshot.snapshot.edge_count);
        Log::info2(
            "parsing-done",
            format!("{} nodes {} edges", nodes.len(), edges.len()),
        );

        SnapshotAnalysis {
            snapshot,
            nodes,
            edges,
        }
    }

    #[wasm_bindgen]
    pub fn get_graph_info(&self, cond: &JsValue) -> JsValue {
        let cond = cond
            .into_serde::<FilterCondition>()
            .expect("failed to decode condition");

        let nodes: Vec<ResultNode> = self
            .get_nodes_by_name(&cond.constructor_name)
            .iter()
            .map(|node| ResultNode::from_node(node))
            .collect();

        Log::info(&format!("{}", nodes.len()));

        let edges: Vec<ResultEdge> = vec![];

        JsValue::from_serde(&Result::new(nodes, edges)).expect_throw("Failed parse SearchResult")
    }

    #[wasm_bindgen]
    pub fn get_node_detail_info(&self, id: &str) -> JsValue {
        let id = id.parse::<u32>().unwrap();
        match self.nodes.iter().find(|node| node.id == id) {
            Some(node) => JsValue::from_serde(&NodeDetailInfo::from_node(node))
                .expect("failed convert NodeDetailInfo"),
            None => JsValue::null(),
        }
    }

    #[wasm_bindgen]
    pub fn get_snapshot_info(&self) {}

    fn get_nodes_by_name(&self, name: &str) -> Vec<&Node> {
        Log::info(name);
        self.nodes
            .iter()
            .filter(|node| node.name.contains(name))
            .collect()
    }
}
