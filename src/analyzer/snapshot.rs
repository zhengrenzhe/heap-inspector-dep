use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::from_slice;

use crate::analyzer::consts::{NODE_TYPE_NATIVE, NODE_TYPE_OBJECT};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EdgeOrNodeType {
    MultiType(Vec<String>),
    SingleType(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnapshotMeta {
    pub edge_fields: Vec<String>,
    pub edge_types: Vec<EdgeOrNodeType>,
    pub location_fields: Vec<String>,
    pub node_fields: Vec<String>,
    pub node_types: Vec<EdgeOrNodeType>,
    pub sample_fields: Vec<String>,
    pub trace_function_info_fields: Vec<String>,
    pub trace_node_fields: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnapshotInfo {
    pub edge_count: u64,
    pub meta: SnapshotMeta,
    pub node_count: u64,
    pub trace_function_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub edges: Vec<u64>,
    pub locations: Vec<u64>,
    pub nodes: Vec<u64>,
    pub samples: Vec<u64>,
    pub snapshot: SnapshotInfo,
    pub strings: Vec<String>,
    pub trace_function_infos: Vec<u64>,
    pub trace_tree: Vec<u64>,
}

fn dump0(data: &Vec<EdgeOrNodeType>) -> Vec<String> {
    match data.get(0).unwrap() {
        EdgeOrNodeType::MultiType(values) => values.clone(),
        EdgeOrNodeType::SingleType(_) => vec![],
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub node_type_index: u64,
    pub name_index: u64,
    pub id: u64,
    pub self_size: u64,
    pub edge_count: u64,
    pub trace_node_id: u64,
    pub detachedness: u64,
    pub from_edge_index: Vec<u64>,
    pub to_edge_index: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub edge_index: u64,
    pub edge_type_index: u64,
    pub name_or_index_raw: u64,
    pub to_node_index: u64,
    pub to_node_id: u64,
    pub from_node_index: u64,
    pub from_node_id: u64,
    pub source: u64,
    pub target: u64,
}

pub struct SnapshotDataProvider {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,

    pub strings: Vec<String>,

    pub edge_count: u64,
    pub node_count: u64,

    pub edge_types: Vec<String>,
    pub node_types: Vec<String>,
}

impl SnapshotDataProvider {
    pub fn from_slice(bytes: &[u8]) -> SnapshotDataProvider {
        let snapshot: Snapshot = match from_slice(bytes) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        };

        SnapshotDataProvider::parse_snapshot(snapshot)
    }

    pub fn get_node_name(&self, node: &Node) -> &String {
        &self.strings[node.name_index as usize]
    }

    pub fn get_node_type(&self, node: &Node) -> &String {
        &self.node_types[node.node_type_index as usize]
    }

    fn parse_snapshot(s: Snapshot) -> SnapshotDataProvider {
        let mut nodes: Vec<Node> = Vec::with_capacity(s.snapshot.node_count as usize);
        let mut edges: Vec<Edge> = Vec::with_capacity(s.snapshot.edge_count as usize);

        let meta = s.snapshot.meta;
        let all_nodes = s.nodes;
        let all_edges = s.edges;

        // parse nodes
        for node_base_idx in (0..all_nodes.len()).step_by(meta.node_fields.len()) {
            let edge_count = all_nodes[node_base_idx + 4];
            nodes.push(Node {
                node_type_index: all_nodes[node_base_idx],
                name_index: all_nodes[node_base_idx + 1],
                id: all_nodes[node_base_idx + 2],
                self_size: all_nodes[node_base_idx + 3],
                edge_count,
                trace_node_id: all_nodes[node_base_idx + 5],
                detachedness: all_nodes[node_base_idx + 6],
                from_edge_index: Vec::with_capacity(edge_count as usize),
                to_edge_index: Vec::with_capacity(edge_count as usize),
            })
        }

        let mut edge_from_node_idx = 0;
        let mut edge_from_node_acc = 0;

        // parse edges
        for (edge_idx, edge_base_idx) in (0..all_edges.len())
            .step_by(meta.edge_fields.len())
            .enumerate()
        {
            // edge base info
            let edge_to_node_idx = all_edges[edge_base_idx + 2] as usize / meta.node_fields.len();

            // ignore empty edges node
            while nodes[edge_from_node_idx].edge_count == 0 {
                edge_from_node_idx += 1;
                edge_from_node_acc = 0;
            }

            // set from/to node
            nodes[edge_from_node_idx]
                .to_edge_index
                .push(edge_idx as u64);
            nodes[edge_to_node_idx]
                .from_edge_index
                .push(edge_idx as u64);

            edges.push(Edge {
                edge_index: edge_idx as u64,
                edge_type_index: all_edges[edge_base_idx],
                name_or_index_raw: all_edges[edge_base_idx + 1],
                to_node_index: edge_to_node_idx as u64,
                to_node_id: nodes[edge_to_node_idx].id,
                target: nodes[edge_to_node_idx].id,
                from_node_index: edge_from_node_idx as u64,
                from_node_id: nodes[edge_from_node_idx].id,
                source: nodes[edge_from_node_idx].id,
            });

            edge_from_node_acc += 1;

            // reset from node idx if needed
            if edge_from_node_acc >= nodes[edge_from_node_idx].edge_count as usize {
                edge_from_node_idx += 1;
                edge_from_node_acc = 0;
            }
        }

        let edge_types = dump0(&meta.edge_types);
        let node_types = dump0(&meta.node_types);

        return SnapshotDataProvider {
            nodes,
            edges,
            strings: s.strings,
            edge_count: s.snapshot.edge_count,
            node_count: s.snapshot.node_count,
            edge_types,
            node_types,
        };
    }
}
