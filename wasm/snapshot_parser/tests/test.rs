#[cfg(test)]
mod tests {
    use snapshot_parser::reader::Reader;

    #[test]
    fn test_parse_from_buffer() {
        let raw = include_bytes!("test.heapsnapshot");
        let snapshot = Reader::from_bytes(raw);
        let (nodes, edges) = snapshot.get_graph();

        assert_eq!(nodes.len(), snapshot.snapshot.node_count as usize);
        assert_eq!(edges.len(), snapshot.snapshot.edge_count as usize);
    }
}
