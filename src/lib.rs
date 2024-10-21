mod web_utils;

use wasm_bindgen::prelude::*;
use crate::web_utils::console_log;

static mut NAMES: Option<Vec<String>> = None;


/// WASM module entry point
#[wasm_bindgen]
pub fn greet(name: &str) {
    // Use an unsafe block only where accessing the static mutable variable
    unsafe {
        if NAMES.is_none() {
            NAMES = Some(Vec::new());
        }
        NAMES.as_mut().unwrap().push(name.to_string());
    }

    // Safely log the names outside the unsafe block
    let all_names = unsafe { NAMES.as_ref().unwrap().join(", ") };
    console_log(&format!("Hello, {}! Previous names: {}", name, all_names));
}