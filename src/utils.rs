use wasm_bindgen::JsValue;

pub fn decode_js_value<T>(data: &JsValue) -> T
where
    T: for<'a> serde::de::Deserialize<'a>,
{
    data.into_serde::<T>().expect("failed to decode")
}
