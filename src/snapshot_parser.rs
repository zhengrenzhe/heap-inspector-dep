use crate::snapshot::SnapshotData;
use crate::utils::Log;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SnapshotParser {
    buffer: Uint8Array,
}

#[wasm_bindgen]
impl SnapshotParser {
    #[wasm_bindgen(constructor)]
    pub fn new(byte_size: u32) -> Self {
        Self {
            buffer: Uint8Array::new_with_length(byte_size),
        }
    }

    #[wasm_bindgen]
    pub fn load(&self, b: &[u8]) {
        Log::str2("reading", format!("{} bytes", b.len()));
        self.buffer.copy_from(b);
        Log::str("read-done");

        Log::str("decoding");
        let result: SnapshotData = serde_json::from_slice(&self.buffer.to_vec()).unwrap();
        Log::str("decode-done");

        let nodes = result.get_all_nodes();
        assert_eq!(nodes.len() as u32, result.snapshot.node_count);
        Log::str2("get-nodes-done", format!("{}", nodes.len()));
    }
}
