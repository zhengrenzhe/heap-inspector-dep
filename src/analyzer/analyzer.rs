use serde_json::{from_slice, from_str};

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

    pub fn from_str(str: &str) -> Analyzer {
        let snapshot: Snapshot = match from_str(str) {
            Ok(snapshot) => snapshot,
            Err(e) => panic!("parse snapshot error: {}", e),
        };
        Analyzer {
            data_provider: parse_snapshot(snapshot),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::analyzer::analyzer::Analyzer;

    #[test]
    fn it_works() {
        let bytes = include_bytes!("/Users/zheng/Downloads/Heap-20221215T222109.heapsnapshot");
        let a = Analyzer::from_bytes(bytes);
        println!("{:?}", a.data_provider.edge_types);
    }
}
