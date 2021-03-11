use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test() -> Vec<u8> {
    [1,2,3].to_vec()
}