#![feature(proc_macro)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[no_mangle]
pub extern fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
#[no_mangle]
pub extern fn alert_add(a: i32, b: i32) -> i32 {
    let c = a + b;
    alert(&format!("Hello from Rust! {} + {} = {}", a, b, c));
    c
}
