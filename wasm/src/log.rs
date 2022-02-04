use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg(msg: String);

    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg2(m1: String, m2: String);
}

pub struct Log {}

impl Log {
    pub fn info(v: &str) {
        console::log_1(&JsValue::from_str(v));
        set_msg(String::from(v));
    }

    pub fn info2(m1: &str, m2: String) {
        console::log_2(&JsValue::from_str(m1), &JsValue::from_str(m2.as_str()));
        set_msg2(String::from(m1), m2);
    }
}
