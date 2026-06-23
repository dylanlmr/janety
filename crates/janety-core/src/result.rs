use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JanetyResult {
    pub success: bool,
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub phase: String,
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub output: Option<String>,
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub console_output: String,
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub errors: Vec<String>,
}

impl JanetyResult {
    pub fn success(phase: &str, output: String, console: String) -> Self {
        Self {
            success: true,
            phase: phase.into(),
            output: Some(output),
            console_output: console,
            errors: vec![],
        }
    }

    pub fn error(phase: &str, console: String, errors: Vec<String>) -> Self {
        Self {
            success: false,
            phase: phase.into(),
            output: None,
            console_output: console,
            errors,
        }
    }
}