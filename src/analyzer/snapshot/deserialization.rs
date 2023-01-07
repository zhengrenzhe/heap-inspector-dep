use crate::analyzer::snapshot::consts::NodeType;
use crate::analyzer::snapshot::definition::{Edge, Node};
use crate::analyzer::snapshot::serde_mapping::{EdgeOrNodeType, Snapshot};

fn dump0(data: &[EdgeOrNodeType]) -> Vec<String> {
    match data.get(0).unwrap() {
        EdgeOrNodeType::MultiType(values) => values.clone(),
        EdgeOrNodeType::SingleType(_) => vec![],
    }
}

pub fn deserialization(s: &Snapshot) -> (Vec<Node>, Vec<String>, u64, Vec<Edge>, Vec<String>, u64) {
    let mut nodes: Vec<Node> = Vec::with_capacity(s.snapshot.node_count as usize);
    let mut edges: Vec<Edge> = Vec::with_capacity(s.snapshot.edge_count as usize);

    let meta = &s.snapshot.meta;
    let all_nodes = &s.nodes;
    let all_edges = &s.edges;

    let edge_types = dump0(&meta.edge_types);
    let node_types = dump0(&meta.node_types);

    // parse nodes
    for node_base_idx in (0..all_nodes.len()).step_by(meta.node_fields.len()) {
        // node type
        let node_type_index = all_nodes[node_base_idx];
        let node_type = NodeType::from(&node_types[node_type_index as usize]);

        // name index
        let name_index = all_nodes[node_base_idx + 1];

        // id
        let id = all_nodes[node_base_idx + 2];

        // self size
        let self_size = all_nodes[node_base_idx + 3];

        // edge count
        let edge_count = all_nodes[node_base_idx + 4];

        // trace node id
        let trace_node_id = all_nodes[node_base_idx + 5];

        // detachedness
        let detachedness = all_nodes[node_base_idx + 6];

        nodes.push(Node {
            node_type,
            name_index,
            id,
            self_size,
            edge_count,
            trace_node_id,
            detachedness,
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

    (
        nodes,
        node_types,
        s.snapshot.node_count,
        edges,
        edge_types,
        s.snapshot.edge_count,
    )
}
