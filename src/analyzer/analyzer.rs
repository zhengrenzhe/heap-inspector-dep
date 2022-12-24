use serde_json::from_slice;

use crate::analyzer::snapshot::{parse_snapshot, Snapshot, SnapshotDataProvider};

pub struct Analyzer {
    pub data_provider: SnapshotDataProvider,
}

impl Analyzer {
    pub fn from_bytes(bytes: &[u8]) -> Analyzer {
        let snapshot: Snapshot = match from_slice(bytes) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        };

        Analyzer {
            data_provider: parse_snapshot(snapshot),
        }
    }
}
