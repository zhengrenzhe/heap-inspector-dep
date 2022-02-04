use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct FilterCondition {
    pub constructor_name: String,
    pub self_size: u32,
    pub retain_size: u32,
    pub reference_depth: u32,
    pub nodes_limit: u32,
    pub self_size_compare_mode: CompareMode,
    pub retain_size_compare_mode: CompareMode,
    pub reference_depth_compare_mode: CompareMode,
    pub ignore_system_node: bool,
}

#[derive(Serialize, Deserialize)]
pub enum CompareMode {
    LessThan = 0,
    MoreThan = 1,
    Equal = 2,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE4: &'static str = r#"
export enum CompareMode {
    LessThan = 0,
    MoreThan = 1,
    Equal = 2,
}

export interface IFilterCondition {
    constructor_name: string;
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
