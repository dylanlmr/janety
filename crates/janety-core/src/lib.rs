mod ast;
mod parser;
mod result;
mod semantics;
mod transpiler;
#[cfg(feature = "wasm")]
mod wasm;

use chumsky::Parser;

pub use result::CompileResult;

pub fn compile(code: &str) -> CompileResult {
    let parser = parser::file_parser();
    let parse_result = parser.parse(code);

    match parse_result.into_result() {
        Ok(ast) => {
            match semantics::typeck::check_types(&ast) {
                Ok(()) => {
                    let output = transpiler::transpile(&ast);
                    CompileResult::success(output)
                }
                Err(errors) => {
                    let type_errors: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
                    CompileResult::type_error(type_errors)
                }
            }
        }
        Err(errors) => {
            let parse_errors: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
            CompileResult::parse_error(parse_errors)
        }
    }
}