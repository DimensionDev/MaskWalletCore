use interface;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn request(input: &[u8]) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    interface::call_api(input)
}
