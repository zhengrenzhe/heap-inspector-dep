use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FilterCondition {
    pub constructor_name: String,
    pub self_size: u32,
    pub retain_size: u32,
    pub reference_depth: u32,
    pub nodes_limit: u32,
    pub self_size_compare_mode: u32,
    pub retain_size_compare_mode: u32,
    pub reference_depth_compare_mode: u32,
    pub ignore_system_node: bool,
}

pub enum CompareMode {
    LessThan = 0,
    MoreThan = 1,
}
