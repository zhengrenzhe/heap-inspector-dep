use crate::analyzer::snapshot::provider::SnapshotProvider;

pub mod api;
pub mod snapshot;

pub struct Analyzer {
    pub data_provider: SnapshotProvider,
    pub file_size: usize,
}

impl Analyzer {
    pub fn from_bytes(bytes: &[u8]) -> Analyzer {
        Analyzer {
            data_provider: SnapshotProvider::from_slice(bytes),
            file_size: bytes.len(),
        }
    }
}
