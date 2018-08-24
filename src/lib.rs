#![feature(use_extern_macros)]
extern crate tensorflow;
extern crate wasm-bindgen;
#[macro_use]
extern crate ndarray;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}
