use wasm_bindgen::prelude::*;

pub mod analysis;
pub mod analysis_impl;
pub mod consts;
pub mod filter;
pub mod log;
pub mod reader;
pub mod result;
pub mod search;
pub mod snapshot;
pub mod snapshot_provider;
pub mod utils;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
