use serde_json::{from_slice, from_str};

use crate::analyzer::snapshot::{Snapshot, SnapshotDataProvider};

pub struct Analyzer<'a> {
    data_provider: SnapshotDataProvider<'a>,
}

impl<'a> Analyzer<'a> {
    pub fn from_bytes(bytes: &[u8]) -> Analyzer {
        let snapshot: Snapshot = match from_slice(bytes) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        };

        let data_provider = snapshot.parse();
        Analyzer { data_provider }
    }

    pub fn from_str(str: &str) -> Analyzer {
        let snapshot: Snapshot = match from_str(str) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        };
        let data_provider = snapshot.parse();
        Analyzer { data_provider }
    }
}
