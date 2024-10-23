mod web_utils;

use crate::web_utils::console_log;
use graph1::graph1_core::context::WindowContext;
use wasm_bindgen::prelude::*;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 240;
const BUF_SIZE: usize = 4 * (WINDOW_WIDTH * WINDOW_HEIGHT) as usize;

struct AppState {
    /// Memory for the window
    buf: [u8; BUF_SIZE],
    /// Current frame count
    frame_count: usize,
    /// Name of the module (TODO: shall we remove it?)
    name: String,
    /// Graph1 window context
    win_ctx: WindowContext,
}

impl AppState {
    fn new(name: String, frame_count: usize) -> Self {
        AppState {
            name,
            frame_count,
            win_ctx: WindowContext::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            buf: [0x54; BUF_SIZE],
        }
    }

}

static mut STATE: Option<AppState> = None;

#[wasm_bindgen]
/// Initialize the WASM module: create `STATE` and populate it with the initial values
pub fn init_state(name: &str, frame: usize) -> usize {
    unsafe {
        if STATE.is_none() {
            STATE = Some(AppState::new(name.to_string(), frame));
        }

        if let Some(state) = STATE.as_ref() {
            console_log(&format!("Current window context: {:#?}", state.win_ctx));
            return state.buf.len();
        }
    }
    return 0;
}

#[wasm_bindgen]
/// Set current frame (if STATE exists)
pub fn update_frame(frame: usize) -> *const u8 {
    unsafe {
        // If STATE is Some, update the frame_count with the provided frame value
        if let Some(state) = STATE.as_mut() {
            state.frame_count = frame;
            console_log(&format!(
                "[ {} ] Current frame: {}",
                state.name, state.frame_count
            ));

            for pixel in state.buf.chunks_mut(4) {
                pixel[0] = 0xff;
                pixel[1] = 0xaa * frame as u8;
                pixel[2] = 0xbb * frame as u8;
                pixel[3] = 0xcc * frame as u8;
            }
        }
        if let Some(state) = STATE.as_ref() {
            return state.buf.as_ptr()
        }
    }
    std::ptr::null()
}

#[wasm_bindgen]
/// Set name (if STATE exists)
pub fn set_name(name: String) {
    unsafe {
        // If STATE is Some, update the name with the provided value
        if let Some(state) = STATE.as_mut() {
            state.name = name;
            console_log(&format!(
                "Name updated to: {}! Current frame: {}",
                state.name, state.frame_count
            ));
        }
    }
}
