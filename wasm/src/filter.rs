use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub const FILTER_FROM_CONSTRUCTOR_NAME: &str = "constructor_name";
pub const FILTER_FROM_CLOSURE_NAME: &str = "closure_name";
pub const FILTER_FROM_STRING_VALUE: &str = "string_value";

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE4: &'static str = r#"
export type IFilterFrom = "constructor_name" | "closure_name" | "string_value";
"#;

#[derive(Serialize, Deserialize)]
pub struct FilterCondition {
    pub filter_from: Vec<String>,
    pub filter_name: String,
    pub self_size: u32,
    pub retain_size: u32,
    pub reference_depth: u32,
    pub nodes_limit: u32,
    pub self_size_compare_mode: u32,
    pub retain_size_compare_mode: u32,
    pub reference_depth_compare_mode: u32,
    pub ignore_system_node: bool,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
export interface IFilterCondition {
  filter_from: IFilterFrom[];
  filter_name: string;
  self_size: number;
  retain_size: number;
  reference_depth: number;
  nodes_limit: number;
  self_size_compare_mode: CompareMode;
  retain_size_compare_mode: CompareMode;
  reference_depth_compare_mode: CompareMode;
  ignore_system_node: boolean;
}
"#;

pub enum CompareMode {
    LessThan = 0,
    MoreThan = 1,
}

#[derive(Serialize, Deserialize)]
pub struct SameStringCondition {
    pub more_than_same_times: usize,
    pub minimum_string_len: usize,
    pub includes: Vec<String>,
    pub excludes: Vec<String>,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE3: &'static str = r#"
export interface ISameStringCondition {
  more_than_same_times: number;
  minimum_string_len: number;
  includes: string[];
  excludes: string[];
}
"#;
