use wasm_bindgen::prelude::*;
use crate::JanetyResult;

#[wasm_bindgen]
pub fn compile(code: &str) -> JanetyResult {
    crate::compile(code)
}
