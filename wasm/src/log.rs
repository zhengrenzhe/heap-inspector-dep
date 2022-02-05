use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg(msg: String, params: JsValue);
}

pub struct Log {}

impl Log {
    pub fn info(msg: &str) {
        set_msg(String::from(msg), JsValue::null());
    }

    pub fn info2(msg: &str, params: Vec<String>) {
        set_msg(
            String::from(msg),
            JsValue::from(
                params
                    .into_iter()
                    .map(|x| JsValue::from_str(&x))
                    .collect::<Array>(),
            ),
        );
    }
}
