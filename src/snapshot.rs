use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SnapshotData {
    pub edges: Vec<u32>,
    pub locations: Vec<u32>,
    pub nodes: Vec<u32>,
    pub samples: Vec<u32>,
    pub snapshot: SnapshotSnapshot,
    pub strings: Vec<String>,
    pub trace_function_infos: Vec<u32>,
    pub trace_tree: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotSnapshot {
    pub edge_count: u32,
    pub meta: SnapshotMeta,
    pub node_count: u32,
    pub trace_function_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotMeta {
    pub edge_fields: Vec<String>,
    pub edge_types: Vec<Type>,
    pub location_fields: Vec<String>,
    pub node_fields: Vec<String>,
    pub node_types: Vec<Type>,
    pub sample_fields: Vec<String>,
    pub trace_function_info_fields: Vec<String>,
    pub trace_node_fields: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Type {
    MultiType(Vec<String>),
    SingleType(String),
}

impl Type {
    pub fn get_multi_with_index(&self, index: usize) -> Option<&String> {
        match self {
            Type::MultiType(types) => types.get(index),
            Type::SingleType(v) => Some(v),
        }
    }

    pub fn get_single(&self) -> Option<&String> {
        match self {
            Type::MultiType(_) => None,
            Type::SingleType(s) => Some(s),
        }
    }
}

pub struct Node {
    pub node_type: String,
    pub name: String,
    pub id: u32,
    pub self_size: u32,
    pub edge_count: u32,
}

impl SnapshotData {
    pub fn get_all_nodes(&self) -> Vec<Node> {
        let node_count = self.snapshot.node_count;
        let node_field_size = self.snapshot.meta.node_fields.len() as u32;

        let mut nodes: Vec<Node> = vec![];
        let mut index: u32 = 0;

        loop {
            if index >= node_count {
                break;
            }

            let base = index * node_field_size;

            nodes.push(Node {
                node_type: self.snapshot.meta.node_types[0]
                    .get_multi_with_index(self.nodes[base as usize] as usize)
                    .unwrap()
                    .clone(),
                name: self.strings[self.nodes[(base + 1) as usize] as usize].clone(),
                id: self.nodes[(base + 2) as usize],
                self_size: self.nodes[(base + 3) as usize],
                edge_count: self.nodes[(base + 4) as usize],
            });

            index += 1;
        }

        nodes
    }

    pub fn get_edges() {}
}
