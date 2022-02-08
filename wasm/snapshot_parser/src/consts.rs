pub const NODE_TYPE_HIDDEN: &str = "hidden";
pub const NODE_TYPE_ARRAY: &str = "array";
pub const NODE_TYPE_STRING: &str = "string";
pub const NODE_TYPE_OBJECT: &str = "object";
pub const NODE_TYPE_CODE: &str = "code";
pub const NODE_TYPE_CLOSURE: &str = "closure";
pub const NODE_TYPE_REGEXP: &str = "regexp";
pub const NODE_TYPE_NUMBER: &str = "number";
pub const NODE_TYPE_NATIVE: &str = "native";
pub const NODE_TYPE_SYNTHETIC: &str = "synthetic";
pub const NODE_TYPE_CONCATENATED_STRING: &str = "concatenated_string";
pub const NODE_TYPE_SLICED_STRING: &str = "sliced_string";
pub const NODE_TYPE_SYMBOL: &str = "symbol";
pub const NODE_TYPE_BIGINT: &str = "bigint";

pub const STRING_NODE_TYPE: [&str; 3] = [
    NODE_TYPE_STRING,
    NODE_TYPE_SLICED_STRING,
    NODE_TYPE_CONCATENATED_STRING,
];

pub const USER_NODE_TYPE: [&str; 10] = [
    NODE_TYPE_ARRAY,
    NODE_TYPE_STRING,
    NODE_TYPE_OBJECT,
    NODE_TYPE_CLOSURE,
    NODE_TYPE_REGEXP,
    NODE_TYPE_NUMBER,
    NODE_TYPE_CONCATENATED_STRING,
    NODE_TYPE_SLICED_STRING,
    NODE_TYPE_SYMBOL,
    NODE_TYPE_BIGINT,
];
