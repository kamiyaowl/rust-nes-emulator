extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate rust_nes_emulator;
use rust_nes_emulator::*;
use rust_nes_emulator::interface::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}