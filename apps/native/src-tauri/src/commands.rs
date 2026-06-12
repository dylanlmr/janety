use janety_core::CompileResult;

#[tauri::command]
pub fn compile_code(code: String) -> CompileResult {
    janety_core::compile(&code)
}