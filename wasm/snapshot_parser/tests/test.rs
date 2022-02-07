#[cfg(test)]
mod tests {
    use snapshot_parser::reader::Reader;

    #[test]
    fn test_parse_from_buffer() {
        let raw = include_bytes!("test.heapsnapshot");
        let snapshot = Reader::from_bytes(raw);

        assert_eq!(snapshot.nodes.len(), snapshot.node_count as usize);
        assert_eq!(snapshot.edges.len(), snapshot.edge_count as usize);
    }
}
