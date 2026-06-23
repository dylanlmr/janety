use janety_core::JanetyResult;

#[tauri::command]
pub fn compile_code(code: &str) -> JanetyResult {
    janety_core::compile(code)
}

#[tauri::command]
pub fn run_code(code: &str) -> JanetyResult {
    janety_core::run(code, "tauri")
}

#[tauri::command]
pub fn compile_and_run_code(code: &str) -> JanetyResult {
    janety_core::compile_and_run(code, "tauri")
}