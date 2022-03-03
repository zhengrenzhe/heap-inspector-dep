use wasm_bindgen::prelude::*;

pub mod analysis;
pub mod decode_snapshot;
pub mod deserialization_snapshot;
pub mod utils;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
