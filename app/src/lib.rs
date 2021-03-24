#![recursion_limit="512"]

mod components;
mod services;
mod pages;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<pages::MainPage>::new().mount_to_body();
}