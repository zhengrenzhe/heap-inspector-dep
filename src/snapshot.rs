// use crate::utils::log::Log;
// use serde::{Deserialize, Serialize};
//
// const EMPTY_STR: &str = "";
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// pub enum EdgeOrNodeType {
//     MultiType(Vec<String>),
//     SingleType(String),
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct SnapshotMeta {
//     pub edge_fields: Vec<String>,
//     pub edge_types: Vec<EdgeOrNodeType>,
//     pub location_fields: Vec<String>,
//     pub node_fields: Vec<String>,
//     pub node_types: Vec<EdgeOrNodeType>,
//     pub sample_fields: Vec<String>,
//     pub trace_function_info_fields: Vec<String>,
//     pub trace_node_fields: Vec<String>,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct SnapshotInfo {
//     pub edge_count: u32,
//     pub meta: SnapshotMeta,
//     pub node_count: u32,
//     pub trace_function_count: u32,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Snapshot {
//     pub edges: Vec<u32>,
//     pub locations: Vec<u32>,
//     pub nodes: Vec<u32>,
//     pub samples: Vec<u32>,
//     pub snapshot: SnapshotInfo,
//     pub strings: Vec<String>,
//     pub trace_function_infos: Vec<u32>,
//     pub trace_tree: Vec<u32>,
// }
//
// pub struct Node {
//     pub node_type_index: usize,
//     pub name_index: usize,
//     pub id: u32,
//     pub self_size: u32,
//     pub edge_count: u32,
//     pub trace_node_id: u32,
//     pub detachedness: u32,
//     pub from_edge_index: Vec<usize>,
//     pub to_edge_index: Vec<usize>,
// }
//
// impl Node {
//     pub fn get_node_name<'a>(&self, strings: &'a [String], strings_len: usize) -> &'a str {
//         if self.name_index <= strings_len {
//             return strings.get(self.name_index).unwrap();
//         }
//         EMPTY_STR
//     }
//
//     pub fn get_node_type<'a>(&self, node_types: &'a [String], node_types_len: usize) -> &'a str {
//         if self.node_type_index <= node_types_len {
//             return node_types.get(self.node_type_index).unwrap();
//         }
//         EMPTY_STR
//     }
// }
//
// pub struct Edge {
//     pub edge_index: usize,
//     pub edge_type_index: usize,
//     pub name_or_index_raw: usize,
//     pub to_node_index: usize,
//     pub to_node_id: u32,
//     pub from_node_index: usize,
//     pub from_node_id: u32,
// }
//
// impl Edge {
//     pub fn get_edge_name<'a>(&self, strings: &'a [String], strings_len: usize) -> &'a str {
//         if self.name_or_index_raw <= strings_len {
//             return strings.get(self.name_or_index_raw).unwrap();
//         }
//         EMPTY_STR
//     }
//
//     pub fn get_edge_type<'a>(&self, edge_types: &'a [String], edge_types_len: usize) -> &'a str {
//         if self.edge_type_index <= edge_types_len {
//             return edge_types.get(self.edge_type_index).unwrap();
//         }
//         EMPTY_STR
//     }
// }
//
// fn dump0(data: &[EdgeOrNodeType]) -> Vec<String> {
//     match data.get(0).unwrap() {
//         EdgeOrNodeType::MultiType(values) => values.clone(),
//         EdgeOrNodeType::SingleType(_) => vec![],
//     }
// }
//
// impl Snapshot {
//     pub fn decode(&self) -> (Vec<Node>, Vec<Edge>, Vec<String>, Vec<String>) {
//         let mut nodes: Vec<Node> = Vec::new();
//         let mut edges: Vec<Edge> = Vec::new();
//
//         let meta = &self.snapshot.meta;
//         let all_nodes = &self.nodes;
//         let all_edges = &self.edges;
//
//         // parse nodes
//         for node_base_idx in (0..all_nodes.len()).step_by(meta.node_fields.len()) {
//             nodes.push(Node {
//                 node_type_index: all_nodes[node_base_idx] as usize,
//                 name_index: all_nodes[node_base_idx + 1] as usize,
//                 id: all_nodes[node_base_idx + 2],
//                 self_size: all_nodes[node_base_idx + 3],
//                 edge_count: all_nodes[node_base_idx + 4],
//                 trace_node_id: all_nodes[node_base_idx + 5],
//                 detachedness: all_nodes[node_base_idx + 6],
//                 from_edge_index: vec![],
//                 to_edge_index: vec![],
//             })
//         }
//
//         Log::info("parse-node-done");
//
//         let mut edge_from_node_idx = 0;
//         let mut edge_from_node_acc = 0;
//
//         // parse edges
//         for (edge_idx, edge_base_idx) in (0..all_edges.len())
//             .step_by(meta.edge_fields.len())
//             .enumerate()
//         {
//             // edge base info
//             let edge_to_node_idx =
//                 all_edges[edge_base_idx + 2] as usize / self.snapshot.meta.node_fields.len();
//
//             // ignore empty edges node
//             while nodes[edge_from_node_idx].edge_count == 0 {
//                 edge_from_node_idx += 1;
//                 edge_from_node_acc = 0;
//             }
//
//             // set from/to node
//             nodes[edge_from_node_idx].to_edge_index.push(edge_idx);
//             nodes[edge_to_node_idx].from_edge_index.push(edge_idx);
//
//             edges.push(Edge {
//                 edge_index: edge_idx,
//                 edge_type_index: all_edges[edge_base_idx] as usize,
//                 name_or_index_raw: all_edges[edge_base_idx + 1] as usize,
//                 to_node_index: edge_to_node_idx,
//                 to_node_id: nodes[edge_to_node_idx].id,
//                 from_node_index: edge_from_node_idx,
//                 from_node_id: nodes[edge_from_node_idx].id,
//             });
//
//             edge_from_node_acc += 1;
//
//             // reset from node idx if needed
//             if edge_from_node_acc >= nodes[edge_from_node_idx].edge_count as usize {
//                 edge_from_node_idx += 1;
//                 edge_from_node_acc = 0;
//             }
//         }
//
//         Log::info("parse-edge-done");
//
//         let node_types = dump0(&self.snapshot.meta.node_types);
//         let edges_types = dump0(&self.snapshot.meta.edge_types);
//
//         (nodes, edges, node_types, edges_types)
//     }
// }
