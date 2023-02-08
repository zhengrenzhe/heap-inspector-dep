#![allow(dead_code)]

use serde::Serialize;

const NODE_TYPE_HIDDEN: &str = "hidden";
const NODE_TYPE_ARRAY: &str = "array";
const NODE_TYPE_STRING: &str = "string";
const NODE_TYPE_OBJECT: &str = "object";
const NODE_TYPE_CODE: &str = "code";
const NODE_TYPE_CLOSURE: &str = "closure";
const NODE_TYPE_REGEXP: &str = "regexp";
const NODE_TYPE_NUMBER: &str = "number";
const NODE_TYPE_NATIVE: &str = "native";
const NODE_TYPE_SYNTHETIC: &str = "synthetic";
const NODE_TYPE_CONCATENATED_STRING: &str = "concatenated string";
const NODE_TYPE_SLICED_STRING: &str = "sliced string";
const NODE_TYPE_SYMBOL: &str = "symbol";
const NODE_TYPE_BIGINT: &str = "bigint";
const NODE_TYPE_OBJECT_SHAPE: &str = "object shape";

#[derive(Debug, Hash, Eq, PartialEq, Serialize)]
pub enum NodeType {
    Hidden,
    Array,
    String,
    Object,
    Code,
    Closure,
    Regexp,
    Number,
    Native,
    Synthetic,
    ConcatenatedString,
    SlicedString,
    Symbol,
    Bigint,
    ObjectShape,
}

impl NodeType {
    pub fn from(str: &str) -> NodeType {
        match str {
            NODE_TYPE_HIDDEN => NodeType::Hidden,
            NODE_TYPE_ARRAY => NodeType::Array,
            NODE_TYPE_STRING => NodeType::String,
            NODE_TYPE_OBJECT => NodeType::Object,
            NODE_TYPE_CODE => NodeType::Code,
            NODE_TYPE_CLOSURE => NodeType::Closure,
            NODE_TYPE_REGEXP => NodeType::Regexp,
            NODE_TYPE_NUMBER => NodeType::Number,
            NODE_TYPE_NATIVE => NodeType::Native,
            NODE_TYPE_SYNTHETIC => NodeType::Synthetic,
            NODE_TYPE_CONCATENATED_STRING => NodeType::ConcatenatedString,
            NODE_TYPE_SLICED_STRING => NodeType::SlicedString,
            NODE_TYPE_SYMBOL => NodeType::Symbol,
            NODE_TYPE_BIGINT => NodeType::Bigint,
            NODE_TYPE_OBJECT_SHAPE => NodeType::ObjectShape,
            _ => panic!("unknown node type {}", str),
        }
    }
}
