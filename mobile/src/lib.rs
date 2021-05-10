use interface;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_request(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let input_bytes = c_str.to_bytes();
    let response_bytes = interface::call_api(&input_bytes);
    CString::new(response_bytes).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rust_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
