use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg(msg: String);

    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg_1_number(msg: String, num1: usize);

    #[wasm_bindgen(js_namespace = Log)]
    fn set_msg_2_number(msg: String, num1: usize, num2: usize);
}

pub struct Log {}

impl Log {
    #![cfg(target_arch = "wasm32")]
    pub fn info(msg: &str) {
        set_msg(String::from(msg));
    }

    pub fn info1_usize(msg: &str, num1: usize) {
        set_msg_1_number(String::from(msg), num1);
    }

    pub fn info2_usize(msg: &str, num1: usize, num2: usize) {
        set_msg_2_number(String::from(msg), num1, num2);
    }
}

impl Log {
    #![cfg(not(target_arch = "wasm32"))]
    pub fn info(msg: &str) {
        println!("{}", msg);
    }

    pub fn info1_usize(msg: &str, num1: usize) {
        println!("{} {}", msg, num1);
    }

    pub fn info2_usize(msg: &str, num1: usize, num2: usize) {
        println!("{} {} {}", msg, num1, num2);
    }
}
