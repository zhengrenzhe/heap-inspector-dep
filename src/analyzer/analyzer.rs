use serde::{Deserialize, Serialize};
use serde_json::from_slice;

use crate::analyzer::snapshot::{parse_snapshot, Snapshot, SnapshotDataProvider};

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchQuery {
    filter_from: Vec<String>,
    filter_name: String,
    self_size_mode: String,
    self_size: usize,
    retained_size_mode: String,
    retained_size: usize,
    depth: usize,
}

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

    pub fn search(&self, query: &SearchQuery) {}
}
