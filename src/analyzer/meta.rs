use crate::analyzer::analyzer::Analyzer;

pub struct MetaInfo {
    pub edge_count: u32,
    pub node_count: u32,
    pub file_size: usize,
}

impl Analyzer {
    pub fn meta(&self) -> MetaInfo {
        MetaInfo {
            edge_count: self.data_provider.edge_count,
            node_count: self.data_provider.node_count,
            file_size: self.file_size,
        }
    }
}
