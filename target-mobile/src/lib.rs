use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// # Safety
///
/// The caller should provide a pointer that points to a valid C string with a NUL terminator of size less than `isize::MAX`
#[no_mangle]
pub unsafe extern "C" fn rust_request(input: *const c_char) -> *mut c_char {
    let c_str = CStr::from_ptr(input);
    let input_bytes = c_str.to_bytes();
    let response_bytes = interface::call_api(&input_bytes);
    CString::new(response_bytes).unwrap().into_raw()
}

/// # Safety
///
/// The caller should provide a pointer that points to a valid C string with a NUL terminator of size less than `isize::MAX`.
#[no_mangle]
pub unsafe extern "C" fn rust_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}
