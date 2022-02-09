use crate::analysis::SnapshotAnalysis;
use crate::filter::{
    FilterCondition, FILTER_FROM_CLOSURE_NAME, FILTER_FROM_CONSTRUCTOR_NAME,
    FILTER_FROM_STRING_VALUE,
};
use crate::result::{Result, ResultEdge, ResultNode};
use snapshot_parser::consts::{
    NODE_TYPE_CLOSURE, NODE_TYPE_NATIVE, NODE_TYPE_OBJECT, STRING_NODE_TYPE, USER_NODE_TYPE,
};
use snapshot_parser::snapshot::{Edge, Node};
use snapshot_parser::snapshot_provider::SnapshotProvider;
use wasm_bindgen::prelude::*;

impl SnapshotAnalysis {
    pub(crate) fn nodes_to_result_node(nodes: &[&Node]) -> Vec<ResultNode> {
        nodes
            .iter()
            .map(|node| ResultNode::from_node(node))
            .collect()
    }

    pub(crate) fn edges_to_result_edge(edges: &[&Edge]) -> Vec<ResultEdge> {
        edges
            .iter()
            .map(|edge| ResultEdge::from_edge(edge))
            .collect()
    }

    pub(crate) fn convert_graph_to_js(nodes: &[&Node], edges: &[&Edge]) -> JsValue {
        JsValue::from_serde(&Result::new(
            SnapshotAnalysis::nodes_to_result_node(nodes),
            SnapshotAnalysis::edges_to_result_edge(edges),
        ))
        .expect_throw("Failed parse SearchResult")
    }

    pub(crate) fn filter_nodes(provider: &SnapshotProvider, cond: FilterCondition) -> Vec<&Node> {
        let (strings, strings_len) = provider.get_strings();
        let (node_types, node_types_len) = provider.get_node_types();

        let filter_constructor = FILTER_FROM_CONSTRUCTOR_NAME.to_string();
        let filter_closure = FILTER_FROM_CLOSURE_NAME.to_string();
        let filter_string = FILTER_FROM_STRING_VALUE.to_string();

        let result: Vec<&Node> = provider
            .nodes
            .iter()
            .filter(|node| {
                let node_type = node.get_node_type(node_types, node_types_len);

                // if search from constructor, node type must by object or native
                if cond.filter_from.contains(&filter_constructor)
                    && (node_type != NODE_TYPE_OBJECT && node_type != NODE_TYPE_NATIVE)
                {
                    return false;
                }

                // if search from closure, node type must by closure
                if cond.filter_from.contains(&filter_closure) && node_type != NODE_TYPE_CLOSURE {
                    return false;
                }

                // if search from string, node type must by string node
                if cond.filter_from.contains(&filter_string)
                    && (!STRING_NODE_TYPE.contains(&node_type))
                {
                    return false;
                }

                if cond.ignore_system_node && !USER_NODE_TYPE.contains(&node_type) {
                    return false;
                }

                let node_name = node.get_node_name(strings, strings_len);
                if !node_name.contains(&cond.filter_name) {
                    return false;
                }

                true
            })
            .collect();

        if result.len() < cond.nodes_limit {
            return result;
        }

        result[0..(cond.nodes_limit)].to_vec()
    }

    pub(crate) fn get_child_graph<'a, 'b: 'a>(
        provider: &'b SnapshotProvider,
        source_node: &'b Node,
    ) -> (Vec<&'a Node>, Vec<&'a Edge>) {
        let mut result_nodes: Vec<&'a Node> = Vec::new();
        let mut result_edges: Vec<&'a Edge> = Vec::new();

        source_node.to_edge_index.iter().for_each(|edge_index| {
            let edge = &provider.edges[*edge_index];
            result_edges.push(edge);

            let to_node = &provider.nodes[edge.to_node_index];
            result_nodes.push(to_node);
        });

        (result_nodes, result_edges)
    }

    pub(crate) fn get_children_graph<'a, 'b: 'a>(
        provider: &'b SnapshotProvider,
        source_nodes: &'b [&Node],
    ) -> (Vec<&'a Node>, Vec<&'a Edge>) {
        let mut result_node: Vec<&Node> = Vec::new();
        let mut result_edge: Vec<&Edge> = Vec::new();

        source_nodes.iter().for_each(|node| {
            result_node.push(node);
            let (mut r_nodes, mut r_edges) = SnapshotAnalysis::get_child_graph(provider, node);
            result_node.append(&mut r_nodes);
            result_edge.append(&mut r_edges);
        });

        (result_node, result_edge)
    }
}
