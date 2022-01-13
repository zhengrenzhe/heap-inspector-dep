use crate::graph::{EdgeGraphData, Graph, NodeDetailData, NodeGraphData, SearchResult};
use crate::snapshot::SnapshotData;
use crate::utils::Log;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SnapshotParser {
    raw_bytes: Uint8Array,
    snapshot: SnapshotData,
    graph: Graph,
}

#[wasm_bindgen]
impl SnapshotParser {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Self {
        Log::info2("reading", format!("{} bytes", bytes.len()));
        let raw_bytes = Uint8Array::from(bytes);
        Log::info("read-done");

        Log::info("decoding");
        let snapshot: SnapshotData = serde_json::from_slice(&raw_bytes.to_vec()).unwrap();
        Log::info("decode-done");

        Log::info("parsing");
        let graph = snapshot.get_graph();
        assert_eq!(graph.nodes.len() as u32, snapshot.snapshot.node_count);
        assert_eq!(graph.edges.len() as u32, snapshot.snapshot.edge_count);
        Log::info2(
            "parsing-done",
            format!("{} nodes {} edges", graph.nodes.len(), graph.edges.len()),
        );

        Self {
            raw_bytes,
            graph,
            snapshot,
        }
    }

    #[wasm_bindgen]
    pub fn get_graph(&self) -> JsValue {
        let nodes = self.graph.nodes[100..300]
            .iter()
            .map(|x| x.get_graph_data())
            .collect();
        let edges = self.graph.edges[100..300]
            .iter()
            .map(|x| x.get_graph_data())
            .collect();
        let r = SearchResult { nodes, edges };
        JsValue::from_serde(&r).expect_throw("Failed parse SearchResult")
    }

    #[wasm_bindgen]
    pub fn get_node_info_by_id(&self, id_string: String) -> JsValue {
        let id = id_string.parse::<u32>().unwrap();
        match &self.graph.nodes.iter().find(|node| node.id == id) {
            Some(node) => {
                let node_id = id_string;
                let node_type = String::from(
                    &self.snapshot.snapshot.meta.node_types[0]
                        .get_value(node.node_type_index as usize)
                        .clone(),
                );
                let node_name = String::from(&self.snapshot.strings[node.name_index as usize]);
                let self_size = node.self_size;
                let edge_count = node.edge_count;
                let detail = NodeDetailData {
                    node_id,
                    node_type,
                    node_name,
                    self_size,
                    edge_count,
                };
                JsValue::from_serde(&detail).expect("failed convert NodeDetailData")
            }
            None => JsValue::null(),
        }
    }
}
