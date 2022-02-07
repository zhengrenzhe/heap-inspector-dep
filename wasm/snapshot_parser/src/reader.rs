use crate::snapshot::Snapshot;
use crate::snapshot_provider::SnapshotProvider;
use serde_json::from_slice;

pub struct Reader {}

impl Reader {
    pub fn from_bytes(b: &[u8]) -> SnapshotProvider {
        let snapshot: Snapshot = match from_slice(b) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        };

        let (nodes, edges, node_types, edge_types) = snapshot.decode();

        SnapshotProvider {
            nodes,
            edges,
            strings: snapshot.strings,
            edge_count: snapshot.snapshot.edge_count,
            node_count: snapshot.snapshot.node_count,
            edge_types,
            node_types,
        }
    }
}
