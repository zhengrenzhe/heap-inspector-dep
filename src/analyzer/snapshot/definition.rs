use crate::analyzer::snapshot::consts::NodeType;

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub name: String,
    pub name_index: u64,
    pub id: u64,
    pub self_size: u64,
    pub edge_count: u64,
    pub trace_node_id: u64,
    pub detachedness: u64,
    pub from_edge_index: Vec<u64>,
    pub to_edge_index: Vec<u64>,
}

#[derive(Debug)]
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
