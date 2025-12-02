use wasm_bindgen::prelude::*;
use web_sys::js_sys::JsString;
#[wasm_bindgen]
pub fn geo_test()->JsString{
  JsString::from("hello geo_wasm")
}
