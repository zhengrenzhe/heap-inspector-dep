use crate::snapshot::{Edge, Node, Snapshot};
use std::collections::HashMap;

type ParseTMP<'a> = HashMap<&'a str, u32>;

impl Snapshot {
    fn get_string(&self, index: usize) -> String {
        self.strings.get(index).unwrap_or(&String::from("")).clone()
    }

    fn parse_object<T>(
        &self,
        values: &[u32],
        fields: &[String],
        creator: &dyn Fn(ParseTMP, usize) -> T,
    ) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        let mut value_index = 0;
        let mut item_index: usize = 0;
        let values_len = values.len();
        let fields_len = fields.len();

        while value_index < values_len {
            let mut field_index = 0;
            let mut object: ParseTMP = HashMap::new();

            while field_index < fields_len {
                object.insert(&fields[field_index], *values.get(value_index).unwrap());

                field_index += 1;
                value_index += 1;
            }

            result.push(creator(object, item_index));
            item_index += 1;
        }

        result
    }

    fn get_obj_value(obj: &ParseTMP, key: &str) -> u32 {
        *obj.get(key)
            .unwrap_or_else(|| panic!("get_obj_value {} unbound", key))
    }

    fn create_node(&self, obj: ParseTMP) -> Node {
        let id = Snapshot::get_obj_value(&obj, "id");
        let edge_count = Snapshot::get_obj_value(&obj, "edge_count");
        let self_size = Snapshot::get_obj_value(&obj, "self_size");
        let trace_node_id = Snapshot::get_obj_value(&obj, "trace_node_id");
        let detachedness = Snapshot::get_obj_value(&obj, "detachedness");
        let name = self.get_string(Snapshot::get_obj_value(&obj, "name") as usize);
        let node_type = self.snapshot.meta.node_types[0]
            .get_value(Snapshot::get_obj_value(&obj, "type") as usize);
        Node {
            node_type,
            name,
            id,
            self_size,
            edge_count,
            trace_node_id,
            detachedness,
            from_edge_index: vec![],
            to_edge_index: vec![],
        }
    }

    fn create_edge(&self, obj: ParseTMP, nodes: &[Node]) -> Edge {
        let to_node = Snapshot::get_obj_value(&obj, "to_node");
        let type_index = Snapshot::get_obj_value(&obj, "type") as usize;
        let name_or_index_val = Snapshot::get_obj_value(&obj, "name_or_index") as usize;

        let edge_type = self.snapshot.meta.edge_types[0].get_value(type_index);
        let name_or_index = self.get_string(name_or_index_val);
        let name = name_or_index.clone();

        let to_node_index = (to_node as usize) / self.snapshot.meta.node_fields.len();
        let to_node = &nodes[to_node_index];

        Edge {
            edge_type,
            name_or_index,
            name,
            to_node_index,
            to_node_id: to_node.id,
            from_node_index: 0,
            from_node_id: 0,
        }
    }

    pub fn get_graph(&self) -> (Vec<Node>, Vec<Edge>) {
        // get nodes
        let mut nodes: Vec<Node> =
            self.parse_object(&self.nodes, &self.snapshot.meta.node_fields, &|obj, _| {
                self.create_node(obj)
            });

        // get edges
        let mut edges = self.parse_object(
            &self.edges,
            &self.snapshot.meta.edge_fields,
            &|obj: ParseTMP, _| self.create_edge(obj, &nodes),
        );

        // set edge data
        let mut edge_index: usize = 0;
        let edges_len = edges.len();
        for (node_index, node) in nodes.iter_mut().enumerate() {
            let mut node_edge_index: usize = 0;

            while node_edge_index < (node.edge_count as usize) {
                if edge_index > edges_len {
                    panic!("index unbound edges length");
                }

                let edge = &mut edges[edge_index];
                edge.from_node_id = node.id;
                edge.from_node_index = node_index;

                node.to_edge_index.push(edge_index as u32);

                edge_index += 1;
                node_edge_index += 1;
            }
        }

        for (edge_index, edge) in edges.iter_mut().enumerate() {
            nodes[edge.to_node_index as usize]
                .from_edge_index
                .push(edge_index as u32);
        }

        (nodes, edges)
    }
}
