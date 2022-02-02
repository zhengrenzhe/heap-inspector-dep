use crate::snapshot::{Edge, Node, Snapshot};

impl Snapshot {
    fn get_string(&self, index: usize) -> String {
        self.strings.get(index).unwrap_or(&String::from("")).clone()
    }

    pub fn get_graph(&self) -> (Vec<Node>, Vec<Edge>) {
        let mut nodes: Vec<Node> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();

        let meta = &self.snapshot.meta;
        let all_nodes = &self.nodes;
        let all_edges = &self.edges;
        let nodes_fields_count = self.snapshot.meta.node_fields.len();

        // parse nodes
        for node_base_idx in (0..all_nodes.len()).step_by(meta.node_fields.len()) {
            nodes.push(Node {
                node_type: meta.node_types[0].get_value(all_nodes[node_base_idx] as usize),
                name: self.get_string(all_nodes[node_base_idx + 1] as usize),
                id: all_nodes[node_base_idx + 2],
                self_size: all_nodes[node_base_idx + 3],
                edge_count: all_nodes[node_base_idx + 4],
                trace_node_id: all_nodes[node_base_idx + 5],
                detachedness: all_nodes[node_base_idx + 6],
                from_edge_index: vec![],
                to_edge_index: vec![],
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
            let edge_type = meta.edge_types[0].get_value(all_edges[edge_base_idx] as usize);
            let edge_name_or_index = self.get_string(all_edges[edge_base_idx + 1] as usize);
            let edge_to_node_idx = all_edges[edge_base_idx + 2] as usize / nodes_fields_count;

            // ignore empty edges node
            while nodes[edge_from_node_idx].edge_count == 0 {
                edge_from_node_idx += 1;
                edge_from_node_acc = 0;
            }

            // set from/to node
            let edge_idx = edge_idx as u64;
            nodes[edge_from_node_idx].to_edge_index.push(edge_idx);
            nodes[edge_to_node_idx].from_edge_index.push(edge_idx);

            let from_node_id = nodes[edge_from_node_idx].id;
            let to_node_id = nodes[edge_to_node_idx].id;

            edges.push(Edge {
                edge_type,
                name_or_index: edge_name_or_index,
                to_node_index: edge_to_node_idx,
                to_node_id,
                from_node_index: edge_from_node_idx,
                from_node_id,
            });

            edge_from_node_acc += 1;

            // reset from node idx if needed
            if edge_from_node_acc >= nodes[edge_from_node_idx].edge_count as usize {
                edge_from_node_idx += 1;
                edge_from_node_acc = 0;
            }
        }

        (nodes, edges)
    }
}
