use js_sys::Uint8Array;
use snapshot_parser::consts::{
    NODE_TYPE_CONCATENATED_STRING, NODE_TYPE_SLICED_STRING, NODE_TYPE_STRING,
};
use wasm_bindgen::prelude::*;

use crate::filter::{FilterCondition, SameStringCondition};
use crate::log::Log;
use crate::result::NodeDetailInfo;
use crate::search::count_same_string;
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
    pub fn get_graph(&self, cond: &JsValue) -> JsValue {
        let cond = cond
            .into_serde::<FilterCondition>()
            .expect("failed to decode condition");

        Log::info("searching");

        let nodes = self.get_nodes_by_cond(cond);

        Log::info1_usize("got-nodes", nodes.len());

        SnapshotAnalysis::convert_graph_to_js(&nodes, &[])
    }

    #[wasm_bindgen]
    pub fn get_node_detail(&self, id: u32) -> JsValue {
        let (strings, strings_len) = self.provider.get_strings();
        let (node_types, node_types_len) = self.provider.get_node_types();

        match self.provider.nodes.iter().find(|node| node.id == id) {
            Some(node) => JsValue::from_serde(&NodeDetailInfo::from_node(
                node,
                strings,
                strings_len,
                node_types,
                node_types_len,
            ))
            .expect("failed convert NodeDetailInfo"),
            None => JsValue::null(),
        }
    }

    #[wasm_bindgen]
    pub fn get_same_string_value_nodes(&self, cond: &JsValue) -> JsValue {
        let cond = cond
            .into_serde::<SameStringCondition>()
            .expect("failed to decode condition");

        Log::info("searching");

        let (node_types, node_types_len) = self.provider.get_node_types();
        let (strings, strings_len) = self.provider.get_strings();

        let use_excludes = !cond.excludes.is_empty();
        let use_includes = !cond.includes.is_empty();

        // node name, node index
        let string_nodes: Vec<(&str, usize)> = self
            .provider
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(node_index, node)| {
                let node_type = node.get_node_type(node_types, node_types_len);
                if node_type == NODE_TYPE_STRING
                    || node_type == NODE_TYPE_CONCATENATED_STRING
                    || node_type == NODE_TYPE_SLICED_STRING
                {
                    let node_name = node.get_node_name(strings, strings_len);
                    if node_name.chars().count() < cond.minimum_string_len {
                        return None;
                    }
                    if use_excludes && cond.excludes.iter().any(|ex| node_name.contains(ex)) {
                        return None;
                    }
                    if use_includes && !cond.includes.iter().any(|inc| node_name.contains(inc)) {
                        return None;
                    }
                    Some((node_name, node_index))
                } else {
                    None
                }
            })
            .collect();

        let nodes: Vec<&Node> = count_same_string(&string_nodes, cond.more_than_same_times)
            .iter()
            .map(|node_index| &self.provider.nodes[*node_index])
            .collect();

        Log::info1_usize("got-nodes", nodes.len());

        SnapshotAnalysis::convert_graph_to_js(&nodes, &[])
    }

    fn get_nodes_by_cond(&self, cond: FilterCondition) -> Vec<&Node> {
        let (strings, strings_len) = self.provider.get_strings();

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
