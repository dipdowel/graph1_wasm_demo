mod web_utils;

use wasm_bindgen::prelude::*;
use crate::web_utils::console_log;

/// WASM module entry point
#[wasm_bindgen]
pub fn greet(name: &str)  {
    console_log(&format!("Hello, {}!", name));

}