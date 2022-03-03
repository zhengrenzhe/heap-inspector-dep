const EMPTY_STR: &str = "";

pub struct Node {
    pub node_type_index: usize,
    pub name_index: usize,
    pub id: u32,
    pub self_size: u32,
    pub edge_count: u32,
    pub trace_node_id: u32,
    pub detachedness: u32,
    pub from_edge_index: Vec<usize>,
    pub to_edge_index: Vec<usize>,
}

impl Node {
    pub fn get_node_name<'a>(&self, strings: &'a [String], strings_len: usize) -> &'a str {
        if self.name_index <= strings_len {
            return strings.get(self.name_index).unwrap();
        }
        EMPTY_STR
    }

    pub fn get_node_type<'a>(&self, node_types: &'a [String], node_types_len: usize) -> &'a str {
        if self.node_type_index <= node_types_len {
            return node_types.get(self.node_type_index).unwrap();
        }
        EMPTY_STR
    }
}

pub struct Edge {
    pub edge_index: usize,
    pub edge_type_index: usize,
    pub name_or_index_raw: usize,
    pub to_node_index: usize,
    pub to_node_id: u32,
    pub from_node_index: usize,
    pub from_node_id: u32,
}

impl Edge {
    pub fn get_edge_name<'a>(&self, strings: &'a [String], strings_len: usize) -> &'a str {
        if self.name_or_index_raw <= strings_len {
            return strings.get(self.name_or_index_raw).unwrap();
        }
        EMPTY_STR
    }

    pub fn get_edge_type<'a>(&self, edge_types: &'a [String], edge_types_len: usize) -> &'a str {
        if self.edge_type_index <= edge_types_len {
            return edge_types.get(self.edge_type_index).unwrap();
        }
        EMPTY_STR
    }
}
