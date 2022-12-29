use std::time::SystemTime;

use serde_json::from_slice;

use crate::analyzer::snapshot::{parse_snapshot, Snapshot, SnapshotDataProvider};

pub struct Analyzer {
    pub data_provider: SnapshotDataProvider,
    pub file_size: usize,
}

impl Analyzer {
    pub fn from_bytes(bytes: &[u8]) -> Analyzer {
        let snapshot: Snapshot = match from_slice(bytes) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        };

        let ds = parse_snapshot(snapshot);

        Analyzer {
            data_provider: ds,
            file_size: bytes.len(),
        }
    }
}
