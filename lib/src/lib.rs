use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test() -> u8 {
    123
}

#[wasm_bindgen]
pub fn test2(input: u8) -> u8 {
    input * 2
}

#[wasm_bindgen]
pub fn test3() -> Vec<u8> {
    [1, 2, 3, 4, 6].to_vec()
}

#[wasm_bindgen]
pub fn test4(input: Vec<u8>) -> Vec<u8> {
    input
}