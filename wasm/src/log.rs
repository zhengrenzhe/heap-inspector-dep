use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg(msg: String, params: Array);

    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg_1_number(msg: String, num1: String);

    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg_2_number(msg: String, num1: String, num2: String);
}

pub struct Log {}

impl Log {
    pub fn info(msg: &str) {
        set_msg(String::from(msg), Array::new());
    }

    pub fn info1_usize(msg: &str, num1: usize) {
        set_msg_1_number(String::from(msg), num1.to_string());
    }

    pub fn info2_usize(msg: &str, num1: usize, num2: usize) {
        set_msg_2_number(String::from(msg), num1.to_string(), num2.to_string());
    }
}
