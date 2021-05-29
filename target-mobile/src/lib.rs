use std::slice;

#[repr(C)]
pub struct RustByteSlice {
    pub bytes: *const u8,
    pub len: usize,
}

/// # Safety
///
/// The caller should provide a pointer that points to a valid bytes array of size eqqual to `len`
#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn rust_request(bytes: *const u8, len: usize) -> RustByteSlice {
    let byte_slice = slice::from_raw_parts(bytes, len as usize);

    let response_bytes = interface::call_api(byte_slice);
    let bytes_ptr = response_bytes.as_ptr();
    let bytes_len = response_bytes.len();
    std::mem::forget(response_bytes);
    RustByteSlice {
        bytes: bytes_ptr,
        len: bytes_len,
    }
}

/// # Safety
///
/// The caller should provide a `RustByteSlice` struct with a valid bytes array and its size
#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn rust_free(input: RustByteSlice) {
    let slice = slice::from_raw_parts_mut(input.bytes as *mut u8, input.len as usize);
    let _: Box<[u8]> = Box::from_raw(slice);
}
