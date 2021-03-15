#![recursion_limit="256"]

mod components;

use wasm_bindgen::prelude::*;
use yew::prelude::*;


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<components::App>::new().mount_to_body();
}