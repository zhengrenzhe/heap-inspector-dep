use crate::analyzer::analyzer::Analyzer;

pub struct MetaInfo {
    pub edge_count: u64,
    pub node_count: u64,
    pub file_size: usize,
    pub node_types: Vec<String>,
    pub edge_types: Vec<String>,
}

impl Analyzer {
    pub fn meta(&self) -> MetaInfo {
        MetaInfo {
            edge_count: self.data_provider.edge_count,
            node_count: self.data_provider.node_count,
            node_types: self.data_provider.node_types.clone(),
            edge_types: self.data_provider.edge_types.clone(),
            file_size: self.file_size,
        }
    }
}
