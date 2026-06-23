pub mod ast;
pub mod parser;
pub mod result;
pub mod semantics;
pub mod transpiler;
#[cfg(not(target_arch = "wasm32"))]
pub mod vm;
#[cfg(feature = "wasm")]
pub mod wasm;

use chumsky::Parser;
pub use result::JanetyResult;

pub fn compile(code: &str) -> JanetyResult {
    let parser = parser::file_parser();
    match parser.parse(code).into_result() {
        Ok(ast) => match semantics::typeck::check_types(&ast) {
            Ok(()) => {
                let janet_code = transpiler::transpile(&ast);
                JanetyResult::success("compile", janet_code, String::new()) 
            }
            Err(errors) => {
                let errs = errors.into_iter().map(|e| e.to_string()).collect();
                JanetyResult::error("typecheck", String::new(), errs)
            }
        },
        Err(errors) => {
            let errs = errors.into_iter().map(|e| e.to_string()).collect();
            JanetyResult::error("parsing", String::new(), errs)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run(janet_code: &str, source_name: &str) -> JanetyResult {
    let (execution_result, console_output) = vm::run_in_janet(janet_code, source_name);
    
    match execution_result {
        Ok(valeur) => JanetyResult::success("runtime", valeur, console_output),
        Err(runtime_err) => JanetyResult::error("runtime", String::new(), vec![runtime_err]),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn compile_and_run(code: &str, source_name: &str) -> JanetyResult {
    let parser = parser::file_parser();
    match parser.parse(code).into_result() {
        Ok(ast) => match semantics::typeck::check_types(&ast) {
            Ok(()) => {
                let janet_code = transpiler::transpile(&ast);
                
                let (execution_result, console_output) = vm::run_in_janet(&janet_code, source_name);
                
                match execution_result {
                    Ok(valeur) => JanetyResult::success("runtime", valeur, console_output),
                    Err(runtime_err) => JanetyResult::error("runtime", String::new(), vec![runtime_err]),
                }
            }
            Err(errors) => {
                let errs = errors.into_iter().map(|e| e.to_string()).collect();
                JanetyResult::error("typecheck", String::new(), errs)
            }
        },
        Err(errors) => {
            let errs = errors.into_iter().map(|e| e.to_string()).collect();
            JanetyResult::error("parsing", String::new(), errs)
        }
    }
}
