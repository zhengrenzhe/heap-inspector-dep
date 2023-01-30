use crate::analyzer::snapshot::provider::SnapshotProvider;

pub mod api;
pub mod snapshot;

pub struct Analyzer<'a> {
    pub data_provider: SnapshotProvider,
    pub file_size: usize,
    pub val: &'a str,
}

impl<'a> Analyzer<'a> {
    pub fn from_bytes(bytes: Vec<u8>) -> Analyzer<'a> {
        Analyzer {
            data_provider: SnapshotProvider::from_slice(&bytes),
            file_size: bytes.len(),
            val: "X",
        }
    }
}
