use crate::decode_snapshot::snapshot_define::{Edge, Node};
use crate::decode_snapshot::snapshot_provider::SnapshotProvider;
use crate::deserialization_snapshot::deserialization::deserialization;
use crate::utils::log::Log;

const NODE_FIELDS_LEN: usize = 7;
const EDGE_FIELDS_LEN: usize = 3;

pub fn decode(bytes: &[u8]) -> Option<SnapshotProvider> {
    if let Ok(r) = deserialization(bytes) {
        let (_, ((node_types, edge_types, node_count, edge_count), all_nodes, all_edges, strings)) =
            r;

        let mut nodes: Vec<Node> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();

        let all_nodes = all_nodes;
        let all_edges = all_edges;

        // parse nodes
        for node_base_idx in (0..all_nodes.len()).step_by(NODE_FIELDS_LEN) {
            nodes.push(Node {
                node_type_index: all_nodes[node_base_idx] as usize,
                name_index: all_nodes[node_base_idx + 1] as usize,
                id: all_nodes[node_base_idx + 2],
                self_size: all_nodes[node_base_idx + 3],
                edge_count: all_nodes[node_base_idx + 4],
                trace_node_id: all_nodes[node_base_idx + 5],
                detachedness: all_nodes[node_base_idx + 6],
                from_edge_index: vec![],
                to_edge_index: vec![],
            })
        }

        Log::info("parse-node-done");

        let mut edge_from_node_idx = 0;
        let mut edge_from_node_acc = 0;

        // parse edges
        for (edge_idx, edge_base_idx) in (0..all_edges.len()).step_by(EDGE_FIELDS_LEN).enumerate() {
            // edge base info
            let edge_to_node_idx = all_edges[edge_base_idx + 2] as usize / NODE_FIELDS_LEN;

            // ignore empty edges node
            while nodes[edge_from_node_idx].edge_count == 0 {
                edge_from_node_idx += 1;
                edge_from_node_acc = 0;
            }

            // set from/to node
            nodes[edge_from_node_idx].to_edge_index.push(edge_idx);
            nodes[edge_to_node_idx].from_edge_index.push(edge_idx);

            edges.push(Edge {
                edge_index: edge_idx,
                edge_type_index: all_edges[edge_base_idx] as usize,
                name_or_index_raw: all_edges[edge_base_idx + 1] as usize,
                to_node_index: edge_to_node_idx,
                to_node_id: nodes[edge_to_node_idx].id,
                from_node_index: edge_from_node_idx,
                from_node_id: nodes[edge_from_node_idx].id,
            });

            edge_from_node_acc += 1;

            // reset from node idx if needed
            if edge_from_node_acc >= nodes[edge_from_node_idx].edge_count as usize {
                edge_from_node_idx += 1;
                edge_from_node_acc = 0;
            }
        }

        return Some(SnapshotProvider {
            nodes,
            edges,
            strings,
            edge_count,
            node_count,
            edge_types,
            node_types,
        });
    }

    return None;
}
