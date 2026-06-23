use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    unsafe fn evaluate_janet_code(
        code: *const c_char, 
        source_name: *const c_char, 
        status: *mut c_int,
        out_prints: *mut *const c_char
    ) -> *const c_char;
}

pub fn run_in_janet(code: &str, source_name: &str) -> (Result<String, String>, String) {
    let c_code = CString::new(code).expect("Failed to convert code to CString");
    let c_source = CString::new(source_name).expect("Failed to convert source name to CString");
    
    let mut status: c_int = 0; 
    let mut out_prints_ptr: *const c_char = std::ptr::null();

    let (result_string, console_string) = unsafe {
        let c_result_ptr = evaluate_janet_code(c_code.as_ptr(), c_source.as_ptr(), &mut status, &mut out_prints_ptr);
        
        let res = CStr::from_ptr(c_result_ptr).to_string_lossy().into_owned();
        let console = CStr::from_ptr(out_prints_ptr).to_string_lossy().into_owned();
        
        (res, console)
    };

    if status == 0 {
        (Ok(result_string), console_string)
    } else {
        (Err(result_string), console_string)
    }
}