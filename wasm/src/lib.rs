use wasm_bindgen::prelude::*;

pub mod analysis;
pub mod analysis_impl;
pub mod filter;
pub mod log;
pub mod result;
pub mod search;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
