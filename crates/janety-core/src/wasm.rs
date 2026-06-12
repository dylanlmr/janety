use wasm_bindgen::prelude::*;

use crate::CompileResult;

#[wasm_bindgen]
pub fn compile(code: &str) -> CompileResult {
    crate::compile(code)
}