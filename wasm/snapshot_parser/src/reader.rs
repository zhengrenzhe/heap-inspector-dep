use crate::snapshot::Snapshot;
use serde_json::from_slice;

pub struct Reader {}

impl Reader {
    pub fn from_bytes(b: &[u8]) -> Snapshot {
        match from_slice(b) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        }
    }
}
