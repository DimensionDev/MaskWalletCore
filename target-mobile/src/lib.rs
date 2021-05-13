use std::os::raw::c_uint;
use std::slice;

#[repr(C)]
pub struct RustByteSlice {
    pub bytes: *const u8,
    pub len: c_uint,
}

/// # Safety
///
/// The caller should provide a pointer that points to a valid C string with a NUL terminator of size less than `isize::MAX`
#[no_mangle]
pub unsafe extern "C" fn rust_request(bytes: *const u8, len: c_uint) -> RustByteSlice {
    let byte_slice = slice::from_raw_parts(bytes, len as usize);

    // let c_str = CStr::from_ptr(input);
    // let input_bytes = c_str.to_bytes();
    let response_bytes = interface::call_api(byte_slice);
    // CString::new(response_bytes).unwrap().into_raw()

    RustByteSlice {
        bytes: response_bytes.as_ptr(),
        len: response_bytes.len() as u32,
    }
}

/// # Safety
///
/// The caller should provide a pointer that points to a valid C string with a NUL terminator of size less than `isize::MAX`.
#[no_mangle]
pub unsafe extern "C" fn rust_free(slice: RustByteSlice) {
    if slice.bytes.is_null() {
        return;
    }
}
