use crate::analyzer::snapshot::SnapshotDataProvider;

pub mod api;
pub mod consts;
pub mod snapshot;

pub struct Analyzer {
    pub data_provider: SnapshotDataProvider,
    pub file_size: usize,
}

impl Analyzer {
    pub fn from_bytes(bytes: &[u8]) -> Analyzer {
        Analyzer {
            data_provider: SnapshotDataProvider::from_slice(bytes),
            file_size: bytes.len(),
        }
    }
}
