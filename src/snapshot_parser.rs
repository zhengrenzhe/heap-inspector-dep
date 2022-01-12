use crate::snapshot::{Edge, Graph, SnapshotData};
use crate::utils::Log;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SnapshotParser {
    raw_bytes: Uint8Array,
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
        let snapshot_data: SnapshotData = serde_json::from_slice(&raw_bytes.to_vec()).unwrap();
        Log::info("decode-done");

        Log::info("parsing");
        let graph = snapshot_data.get_graph();
        assert_eq!(graph.nodes.len() as u32, snapshot_data.snapshot.node_count);
        assert_eq!(graph.edges.len() as u32, snapshot_data.snapshot.edge_count);
        Log::info2(
            "parsing-done",
            format!("{} nodes {} edges", graph.nodes.len(), graph.edges.len()),
        );

        Self { raw_bytes, graph }
    }

    pub fn get_graph(&self) {}
}
