use std::collections::HashMap;
use std::fs;

use petgraph::graph::NodeIndex;
use petgraph::visit::NodeRef;
use petgraph::{Direction, Graph};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::analytics::consts::NodeType;
use crate::analytics::mapping::{EdgeOrNodeType, Snapshot};

pub struct MetaInfo {
    pub edge_count: u64,
    pub node_count: u64,
    pub node_types: Vec<String>,
    pub edge_types: Vec<String>,
}

pub struct DeserializationResult {
    nodes: Vec<Node>,
    node_types: Vec<String>,
    edges: Vec<Edge>,
    edge_types: Vec<String>,
    graph: Graph<usize, usize>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchQuery {
    filter_from: Option<Vec<String>>,
    filter_name: Option<String>,
    self_size_mode: Option<String>,
    self_size: Option<usize>,
    retained_size_mode: Option<String>,
    retained_size: Option<usize>,
    depth: Option<usize>,
    depth_mode: Option<String>,
    node_types: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct ConstructorResponse {
    pub total_self_size: u64,
    pub count: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConstructorQuery {
    pub q: Option<Vec<String>>,
}

pub struct LocalConfig {
    pub file_path: String,
}

pub enum ProviderMode {
    Local(LocalConfig),
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub name: String,
    pub id: u64,
    pub self_size: u64,
    pub edge_count: u64,
    pub trace_node_id: u64,
    pub detachedness: u64,
    pub from_edge_index: Vec<u64>,
    pub to_edge_index: Vec<u64>,
    pub graph_node: NodeIndex,
}

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

pub struct SnapshotSample {
    pub nodes: Vec<Node>,
    pub node_types: Vec<String>,
    pub node_count: u64,

    pub edges: Vec<Edge>,
    pub edge_types: Vec<String>,
    pub edge_count: u64,

    pub graph: Graph<usize, usize>,
}

pub struct Provider {
    pub sample: SnapshotSample,
    pub mode: ProviderMode,
}

impl Provider {
    pub fn new(mode: ProviderMode) -> Provider {
        match &mode {
            ProviderMode::Local(local_config) => Provider {
                sample: Provider::new_from_local(&local_config.file_path),
                mode,
            },
        }
    }

    pub fn new_from_local(file_path: &str) -> SnapshotSample {
        let bytes = match fs::read(file_path) {
            Ok(bytes) => bytes,
            Err(_) => panic!("read {} error", &file_path),
        };

        let snapshot = Snapshot::from_bytes(&bytes);

        let result = Provider::deserialization(&snapshot);

        SnapshotSample {
            nodes: result.nodes,
            node_types: result.node_types,
            node_count: snapshot.snapshot.node_count,
            edges: result.edges,
            edge_types: result.edge_types,
            edge_count: snapshot.snapshot.edge_count,
            graph: result.graph,
        }
    }

    pub fn deserialization(snapshot: &Snapshot) -> DeserializationResult {
        let mut nodes: Vec<Node> = Vec::with_capacity(snapshot.snapshot.node_count as usize);
        let mut edges: Vec<Edge> = Vec::with_capacity(snapshot.snapshot.edge_count as usize);

        let meta = &snapshot.snapshot.meta;
        let all_nodes = &snapshot.nodes;
        let all_edges = &snapshot.edges;

        let edge_types = Provider::dump0(&meta.edge_types);
        let node_types = Provider::dump0(&meta.node_types);

        let mut graph = Graph::<usize, usize>::new();

        // parse nodes
        for (node_idx, node_base_idx) in (0..all_nodes.len())
            .step_by(meta.node_fields.len())
            .enumerate()
        {
            // add node to graph
            let graph_node = graph.add_node(node_idx);

            // node type
            let node_type_index = all_nodes[node_base_idx];
            let node_type = NodeType::from(&node_types[node_type_index as usize]);

            // name index
            let name_index = all_nodes[node_base_idx + 1];
            let name = snapshot.strings[name_index as usize].clone();

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
                name,
                node_type,
                id,
                self_size,
                edge_count,
                trace_node_id,
                detachedness,
                from_edge_index: Vec::with_capacity(edge_count as usize),
                to_edge_index: Vec::with_capacity(edge_count as usize),
                graph_node,
            });
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

            // from node
            let from_node = &mut nodes[edge_from_node_idx];
            from_node.to_edge_index.push(edge_idx as u64);
            let from_node_id = from_node.id;
            let from_node_graph_id = from_node.graph_node.id();

            // to node
            let to_node = &mut nodes[edge_to_node_idx];
            to_node.from_edge_index.push(edge_idx as u64);
            let to_node_id = to_node.id;
            let to_node_graph_id = to_node.graph_node.id();

            //update graph
            graph.add_edge(from_node_graph_id, to_node_graph_id, 0);

            edges.push(Edge {
                edge_index: edge_idx as u64,
                edge_type_index: all_edges[edge_base_idx],
                name_or_index_raw: all_edges[edge_base_idx + 1],
                to_node_index: edge_to_node_idx as u64,
                to_node_id,
                target: to_node_id,
                from_node_index: edge_from_node_idx as u64,
                from_node_id,
                source: from_node_id,
            });

            edge_from_node_acc += 1;

            // reset from node idx if needed
            if edge_from_node_acc >= nodes[edge_from_node_idx].edge_count as usize {
                edge_from_node_idx += 1;
                edge_from_node_acc = 0;
            }
        }

        DeserializationResult {
            nodes,
            node_types,
            edges,
            edge_types,
            graph,
        }
    }

    pub fn dump0(data: &[EdgeOrNodeType]) -> Vec<String> {
        match data.get(0).unwrap() {
            EdgeOrNodeType::MultiType(values) => values.clone(),
            EdgeOrNodeType::SingleType(_) => vec![],
        }
    }

    pub fn get_neighbors(&self, node: &Node) -> Vec<&Node> {
        let mut arr: Vec<&Node> = Vec::with_capacity(10000);
        for nei in self
            .sample
            .graph
            .neighbors_directed(node.graph_node, Direction::Outgoing)
        {
            let val = self.sample.graph.node_weight(nei).unwrap();
            let nei_node = &self.sample.nodes[*val];
            arr.push(nei_node);
        }
        arr
    }

    pub fn constructors(&self, query: &ConstructorQuery) -> HashMap<&String, ConstructorResponse> {
        match &query.q {
            Some(q) => {
                let q = &q[0];
                for node in &self.sample.nodes {
                    if &node.name == q {
                        let neighbors = self.get_neighbors(node);
                        let mut size = 0;
                        for neighbor in neighbors {
                            size += neighbor.self_size;
                            println!("{neighbor:?}");
                        }
                        println!("total size: {size}");
                    }
                }
            }
            None => {}
        };

        let mut map: HashMap<&String, ConstructorResponse> = HashMap::new();

        for node in &self.sample.nodes {
            let node_type = &node.node_type;
            if [NodeType::Object, NodeType::Native].contains(node_type) {
                let node_name = &node.name;
                match map.get_mut(node_name) {
                    Some(c) => {
                        c.total_self_size += node.self_size;
                        c.count += 1;
                    }
                    None => {
                        map.insert(
                            node_name,
                            ConstructorResponse {
                                total_self_size: node.self_size,
                                count: 1,
                            },
                        );
                    }
                }
            }
        }

        map
    }

    pub fn meta(&self) -> MetaInfo {
        MetaInfo {
            edge_count: self.sample.edge_count,
            node_count: self.sample.node_count,
            node_types: self.sample.node_types.clone(),
            edge_types: self.sample.edge_types.clone(),
        }
    }

    pub fn search(&self, query: &SearchQuery) -> Value {
        println!("{query:?}");
        json!({"nodes":[], "edges": []})
    }

    pub fn statistics(&self) -> Value {
        let mut node_types_map: HashMap<&NodeType, u64> = HashMap::new();
        let mut total_bytes = 0;

        for node in &self.sample.nodes {
            let node_type = &node.node_type;
            let new_val = match node_types_map.get(node_type) {
                Some(val) => val + node.self_size,
                None => node.self_size,
            };
            node_types_map.insert(node_type, new_val);
            total_bytes += node.self_size
        }

        json!({ "percent": node_types_map, "total_bytes": total_bytes })
    }
}
