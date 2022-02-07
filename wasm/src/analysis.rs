use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::filter::FilterCondition;
use crate::log::Log;
use crate::result::{NodeDetailInfo, Result, ResultEdge, ResultNode};
use snapshot_parser::reader::Reader;
use snapshot_parser::snapshot::Node;
use snapshot_parser::snapshot_provider::SnapshotProvider;

#[wasm_bindgen]
pub struct SnapshotAnalysis {
    provider: SnapshotProvider,
}

#[wasm_bindgen]
impl SnapshotAnalysis {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Self {
        Log::info1_usize("reading", bytes.len());
        let raw_bytes = Uint8Array::from(bytes).to_vec();
        Log::info("read-done");

        Log::info("parsing");
        let provider = Reader::from_bytes(&raw_bytes);
        Log::info2_usize("parsing-done", provider.nodes.len(), provider.edges.len());

        SnapshotAnalysis { provider }
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

        Log::info1_usize("got-nodes", nodes.len());

        let edges: Vec<ResultEdge> = vec![];

        JsValue::from_serde(&Result::new(nodes_result, edges))
            .expect_throw("Failed parse SearchResult")
    }

    #[wasm_bindgen]
    pub fn get_node_detail_info(&self, id: u32) -> JsValue {
        match self.provider.nodes.iter().find(|node| node.id == id) {
            Some(node) => JsValue::from_serde(&NodeDetailInfo::from_node(
                node,
                &self.provider.strings,
                self.provider.strings.len(),
                &self.provider.node_types,
                self.provider.node_types.len(),
            ))
            .expect("failed convert NodeDetailInfo"),
            None => JsValue::null(),
        }
    }

    fn get_nodes_by_cond(&self, cond: FilterCondition) -> Vec<&Node> {
        let strings = &self.provider.strings;
        let strings_len = strings.len();

        let result: Vec<&Node> = self
            .provider
            .nodes
            .iter()
            .filter(|node| {
                if !node
                    .get_node_name(strings, strings_len)
                    .contains(&cond.filter_name)
                {
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
