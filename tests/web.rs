// panicked at 'time not implemented on this platform'

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use ulid_wasm::greet;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!("Hello, WASM!".to_string(), greet());
}
