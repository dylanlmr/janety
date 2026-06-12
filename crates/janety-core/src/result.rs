use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    pub success: bool,

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub output: Option<String>,

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub type_errors: Vec<String>,

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub parse_errors: Vec<String>,
}

impl CompileResult {
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output: Some(output),
            type_errors: vec![],
            parse_errors: vec![],
        }
    }

    pub fn type_error(errors: Vec<String>) -> Self {
        Self {
            success: false,
            output: None,
            type_errors: errors,
            parse_errors: vec![],
        }
    }

    pub fn parse_error(errors: Vec<String>) -> Self {
        Self {
            success: false,
            output: None,
            type_errors: vec![],
            parse_errors: errors,
        }
    }
}