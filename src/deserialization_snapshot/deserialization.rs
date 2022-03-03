use nom::IResult;
use std::str;

use crate::deserialization_snapshot::match_data::match_data;
use crate::deserialization_snapshot::match_snapshot::{match_snapshot, SnapshotMeta};
use crate::deserialization_snapshot::match_strings::match_strings;
use crate::deserialization_snapshot::utils::enter_bracket;

type SnapshotInfo = (SnapshotMeta, Vec<u32>, Vec<u32>, Vec<String>);

pub fn deserialization(bytes: &[u8]) -> IResult<&str, SnapshotInfo> {
    let json_str = match str::from_utf8(bytes) {
        Ok(v) => v,
        Err(_) => panic!("convert &[u8] to &str error"),
    };

    let (input, _) = enter_bracket(json_str)?;

    let (input, (node_types, edge_types, node_count, edge_count)) = match_snapshot(input)?;
    let (input, nodes) = match_data(input, "nodes")?;
    let (input, edges) = match_data(input, "edges")?;
    let (_, strings) = match_strings(input)?;

    Ok((
        "",
        (
            (node_types, edge_types, node_count, edge_count),
            nodes,
            edges,
            strings,
        ),
    ))
}

#[test]
fn test_decode_snapshot() {
    let raw = include_bytes!("../../tests/test.heapsnapshot.json");
    if let Ok(g) = deserialization(raw) {
        let (_, ((node_types, edge_types, node_count, edge_count), nodes, edges, strings)) = g;
        assert_eq!(node_types.len(), 14);
        assert_eq!(edge_types.len(), 7);
        assert_eq!(node_count, 87927);
        assert_eq!(edge_count, 369511);
        assert_eq!(nodes.len(), (node_count as usize) * 7);
        assert_eq!(edges.len(), (edge_count as usize) * 3);
        assert_eq!(strings.len(), 26452);
    }
}
