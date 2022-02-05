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
        Log::info2("reading", vec![bytes.len().to_string()]);
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
            vec![nodes.len().to_string(), edges.len().to_string()],
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

        Log::info("searching");

        let nodes = self.get_nodes_by_cond(cond);

        let nodes_result: Vec<ResultNode> = nodes
            .iter()
            .map(|node| ResultNode::from_node(node))
            .collect();

        Log::info2("got-nodes", vec![nodes.len().to_string()]);

        let edges: Vec<ResultEdge> = vec![];

        JsValue::from_serde(&Result::new(nodes_result, edges))
            .expect_throw("Failed parse SearchResult")
    }

    #[wasm_bindgen]
    pub fn get_node_detail_info(&self, id: u32) -> JsValue {
        match self.nodes.iter().find(|node| node.id == id) {
            Some(node) => JsValue::from_serde(&NodeDetailInfo::from_node(node))
                .expect("failed convert NodeDetailInfo"),
            None => JsValue::null(),
        }
    }

    fn get_nodes_by_cond(&self, cond: FilterCondition) -> Vec<&Node> {
        let result: Vec<&Node> = self
            .nodes
            .iter()
            .filter(|node| {
                if !node.name.contains(&cond.constructor_name) {
                    return false;
                }

                true
            })
            .collect();

        if result.len() < cond.nodes_limit as usize {
            return result;
        }

        result[0..(cond.nodes_limit as usize)].to_vec()
    }
}
