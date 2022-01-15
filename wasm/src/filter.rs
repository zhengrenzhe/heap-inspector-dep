use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct FilterCondition {
    constructor_name: String,
    self_size: u32,
    retain_size: u32,
    reference_depth: u32,
    self_size_compare_mode: u32,
    retain_size_compare_mode: u32,
    reference_depth_compare_mode: u32,
    nodes_limit: u32,
}

#[wasm_bindgen]
impl FilterCondition {
    #[wasm_bindgen(getter)]
    pub fn constructor_name(&self) -> String {
        self.constructor_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn self_size(&self) -> u32 {
        self.self_size
    }

    #[wasm_bindgen(getter)]
    pub fn retain_size(&self) -> u32 {
        self.retain_size
    }

    #[wasm_bindgen(getter)]
    pub fn reference_depth(&self) -> u32 {
        self.reference_depth
    }

    #[wasm_bindgen(getter)]
    pub fn self_size_compare_mode(&self) -> u32 {
        self.self_size_compare_mode
    }

    #[wasm_bindgen(getter)]
    pub fn retain_size_compare_mode(&self) -> u32 {
        self.retain_size_compare_mode
    }

    #[wasm_bindgen(getter)]
    pub fn reference_depth_compare_mode(&self) -> u32 {
        self.reference_depth_compare_mode
    }

    #[wasm_bindgen(getter)]
    pub fn nodes_limit(&self) -> u32 {
        self.nodes_limit
    }
}
