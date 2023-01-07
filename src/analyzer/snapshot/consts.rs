#![allow(dead_code)]

use serde::Serialize;

use crate::analyzer::snapshot::consts::NodeType::{
    Array, Bigint, Closure, Code, ConcatenatedString, Hidden, Native, Number, Object, ObjectShape,
    Regexp, SlicedString, String, Symbol, Synthetic,
};

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
            NODE_TYPE_HIDDEN => Hidden,
            NODE_TYPE_ARRAY => Array,
            NODE_TYPE_STRING => String,
            NODE_TYPE_OBJECT => Object,
            NODE_TYPE_CODE => Code,
            NODE_TYPE_CLOSURE => Closure,
            NODE_TYPE_REGEXP => Regexp,
            NODE_TYPE_NUMBER => Number,
            NODE_TYPE_NATIVE => Native,
            NODE_TYPE_SYNTHETIC => Synthetic,
            NODE_TYPE_CONCATENATED_STRING => ConcatenatedString,
            NODE_TYPE_SLICED_STRING => SlicedString,
            NODE_TYPE_SYMBOL => Symbol,
            NODE_TYPE_BIGINT => Bigint,
            NODE_TYPE_OBJECT_SHAPE => ObjectShape,
            _ => panic!("unknown node type {}", str),
        }
    }
}
