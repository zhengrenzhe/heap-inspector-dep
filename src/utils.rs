use wasm_bindgen::JsValue;
use web_sys::console;

pub struct Log {}

impl Log {
    pub fn str(v: &str) {
        console::log_1(&JsValue::from_str(v));
    }

    pub fn string(v: String) {
        console::log_1(&JsValue::from_str(v.as_str()));
    }
}
