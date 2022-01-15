use crate::snapshot::Snapshot;
use serde_json::from_slice;

pub struct Reader {}

impl Reader {
    pub fn from_bytes(b: &[u8]) -> Snapshot {
        match from_slice(b) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_from_buffer() {
        use super::*;
        let raw = include_bytes!("/Users/zheng/Downloads/Heap-20220104T133022.heapsnapshot");
        let snapshot = Reader::from_bytes(raw);
        let (nodes, edges) = snapshot.get_graph();
        assert_eq!(nodes.len(), snapshot.snapshot.node_count as usize);
        assert_eq!(edges.len(), snapshot.snapshot.edge_count as usize);
    }
}
