mod web_utils;

use wasm_bindgen::prelude::*;
use crate::web_utils::console_log;

struct AppState {
    frame_count:usize,
    name:String,
}

impl AppState {
    fn new(name:String, frame_count:usize) -> Self {
        AppState{
            name,
            frame_count
        }
    }
}

static mut STATE: Option<AppState> = None;



#[wasm_bindgen]
/// Initialize the WASM module: create `STATE` and populate it with the initial values
pub fn init_module(name: &str, frame:usize) {
    unsafe {
        if STATE.is_none() {
            STATE = Some(AppState::new(name.to_string(), frame));
        }
    }
}


#[wasm_bindgen]
/// Set current frame (if STATE exists)
pub fn update_frame(frame: usize) {
    unsafe {
        // If STATE is Some, update the frame_count with the provided frame value
        if let Some(state) = STATE.as_mut() {
            state.frame_count = frame;
            console_log(&format!(
                "[ {} ] Current frame: {}",
                state.name, state.frame_count
            ));
        }
    }
}

#[wasm_bindgen]
/// Set name (if STATE exists)
pub fn set_name(name: String) {
    unsafe {
        // If STATE is Some, update the name with the provided value
        if let Some(state) = STATE.as_mut() {
            state.name = name.clone();
            console_log(&format!(
                "Name updated to: {}! Current frame: {}",
                state.name, state.frame_count
            ));
        }
    }
}

