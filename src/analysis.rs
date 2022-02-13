use wasm_bindgen::prelude::*;

use crate::consts::STRING_NODE_TYPE;
use crate::filter::{FilterCondition, SameStringCondition};
use crate::log::Log;
use crate::reader::Reader;
use crate::result::{EdgeDetailInfo, NodeDetailInfo};
use crate::search::count_same_string;
use crate::snapshot::Node;
use crate::snapshot_provider::SnapshotProvider;
use crate::utils::decode_js_value;

#[wasm_bindgen]
pub struct SnapshotAnalysis {
    provider: SnapshotProvider,
}

#[wasm_bindgen]
impl SnapshotAnalysis {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Self {
        Log::info("parsing");
        let provider = Reader::from_bytes(bytes);
        Log::info2_usize("parsing-done", provider.nodes.len(), provider.edges.len());

        SnapshotAnalysis { provider }
    }

    #[wasm_bindgen]
    pub fn get_graph(&self, cond: &JsValue) -> JsValue {
        let cond: FilterCondition = decode_js_value(cond);

        Log::info("searching");

        let nodes = SnapshotAnalysis::filter_nodes(&self.provider, cond);

        let (result_node, result_edge) =
            SnapshotAnalysis::get_children_graph(&self.provider, &nodes);

        Log::info2_usize("got-nodes", result_node.len(), result_node.len());

        SnapshotAnalysis::convert_graph_to_js(&result_node, &result_edge)
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
    pub fn get_edge_detail(&self, edge_index: usize) -> JsValue {
        let (strings, strings_len) = self.provider.get_strings();
        let (edge_types, edge_types_len) = self.provider.get_edge_types();

        match self
            .provider
            .edges
            .iter()
            .find(|edge| edge.edge_index == edge_index)
        {
            None => JsValue::null(),
            Some(edge) => JsValue::from_serde(&EdgeDetailInfo::from_edge(
                edge,
                strings,
                strings_len,
                edge_types,
                edge_types_len,
            ))
            .expect("failed convert EdgeDetailInfo"),
        }
    }

    #[wasm_bindgen]
    pub fn get_same_string_value_nodes(&self, cond: &JsValue) -> JsValue {
        let cond: SameStringCondition = decode_js_value(cond);

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
                if STRING_NODE_TYPE.contains(&node_type) {
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

        let (result_node, result_edge) =
            SnapshotAnalysis::get_children_graph(&self.provider, &nodes);

        Log::info2_usize("got-nodes", result_node.len(), result_edge.len());

        SnapshotAnalysis::convert_graph_to_js(&result_node, &result_edge)
    }
}
