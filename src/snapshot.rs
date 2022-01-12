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
    pub edge_types: Vec<EdgeOrNodeType>,
    pub location_fields: Vec<String>,
    pub node_fields: Vec<String>,
    pub node_types: Vec<EdgeOrNodeType>,
    pub sample_fields: Vec<String>,
    pub trace_function_info_fields: Vec<String>,
    pub trace_node_fields: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeOrNodeType {
    MultiType(Vec<String>),
    SingleType(String),
}

impl EdgeOrNodeType {
    pub fn get_value(&self, index: usize) -> &String {
        match self {
            EdgeOrNodeType::MultiType(values) => values.get(index).unwrap(),
            EdgeOrNodeType::SingleType(value) => Some(value).unwrap(),
        }
    }
}

pub struct Node {
    pub node_type_index: u32,
    pub name_index: u32,
    pub id: u32,
    pub self_size: u32,
    pub edge_count: u32,
}

pub struct Edge {
    pub source_node_id: u32,
    pub target_node_id: u32,
    pub edge_type_index: u32,
}

pub struct Graph {
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
}

impl SnapshotData {
    pub fn get_graph(&self) -> Graph {
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
                node_type_index: self.nodes[base as usize],
                name_index: self.nodes[(base + 1) as usize],
                id: self.nodes[(base + 2) as usize],
                self_size: self.nodes[(base + 3) as usize],
                edge_count: self.nodes[(base + 4) as usize],
            });

            index += 1;
        }

        let edge_field_size = self.snapshot.meta.edge_fields.len() as u32;
        let mut edges: Vec<Edge> = vec![];

        for node in nodes.iter() {
            let base = node.edge_count * edge_field_size;
            let mut index = 0;
            loop {
                if index >= node.edge_count {
                    break;
                }

                let edge_base = base + index * edge_field_size;

                edges.push(Edge {
                    source_node_id: self.nodes[self.edges[(edge_base + 2) as usize] as usize],
                    target_node_id: self.nodes[self.edges[(edge_base + 3) as usize] as usize],
                    edge_type_index: self.edges[edge_base as usize],
                });

                index += 1
            }
        }

        Graph { edges, nodes }
    }
}
