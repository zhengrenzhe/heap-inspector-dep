use crate::snapshot::{Edge, Node, Snapshot};

impl Snapshot {
    fn get_from_strings(&self, index: usize) -> String {
        self.strings.get(index).expect("index unbound").clone()
    }

    pub fn get_graph(&self) -> (Vec<Node>, Vec<Edge>) {
        let meta = &self.snapshot.meta;

        // get nodes
        let mut nodes: Vec<Node> = vec![];
        let node_field_size = meta.node_fields.len();

        for node_index in 0..(self.snapshot.node_count as usize) {
            let base = node_index * node_field_size;
            nodes.push(Node {
                node_type: meta.node_types[0].get_value(self.nodes[base] as usize),
                name: self.get_from_strings(self.nodes[base + 1] as usize),
                id: self.nodes[base + 2],
                self_size: self.nodes[base + 3],
                edge_count: self.nodes[base + 4],
                trace_node_id: self.nodes[base + 5],
                detachedness: self.nodes[base + 6],
                from_edge_index: vec![],
                to_edge_index: vec![],
            })
        }

        // get edges
        let mut edges: Vec<Edge> = vec![];
        let edge_field_size = meta.edge_fields.len();
        let mut from_node_index: usize = 0;
        let mut current_edge_count: u32 = 0;

        for edge_index in 0..(self.snapshot.edge_count as usize) {
            let base = edge_index * edge_field_size;

            let edge_type = meta.edge_types[0].get_value(self.edges[base] as usize);
            let edge_name_or_index = self.get_from_strings(self.edges[base + 1] as usize);
            let name = edge_name_or_index.clone();

            let to_node_index = (self.edges[base + 2] as usize) / meta.node_fields.len();
            let to_node_id = nodes[to_node_index].id;

            nodes[to_node_index].from_edge_index.push(edge_index as u32);
            nodes[from_node_index].to_edge_index.push(edge_index as u32);

            let from_node_id = nodes[from_node_index].id;

            edges.push(Edge {
                edge_type,
                name_or_index: edge_name_or_index,
                name,
                to_node_index,
                to_node_id,
                from_node_id,
                from_node_index,
            });

            current_edge_count += 1;
            if current_edge_count >= nodes[from_node_index].edge_count {
                current_edge_count = 0;
                from_node_index += 1;
            }
        }

        (nodes, edges)
    }
}
