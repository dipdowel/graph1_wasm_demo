mod web_utils;

use crate::web_utils::console_log;
use graph1::graph1_core::context::WindowContext;
use wasm_bindgen::prelude::*;

/// Width of the window, in pixels
const WIN_WIDTH: u32 = 640;
/// Height of the window, in pixels
const WIN_HEIGHT: u32 = 240;
/// Framebuffer size, in bytes
const BUF_SIZE: usize = 4 * (WIN_WIDTH * WIN_HEIGHT) as usize;

#[wasm_bindgen]
/// This data is passed to the JS-world to render the framebuffer on a 2D-canvas.
pub struct InitStateResult {
    /// Points to the start of the frame buffer
    pub pointer: *const u8,
    /// Size of the frame buffer in bytes
    pub buf_size: usize,
    /// Width of the window
    pub width: u32,
    /// Height of the window
    pub height: u32,
}

/// A default instance of `InitStateResult`
const DEFAULT_INIT_RESULT: InitStateResult = InitStateResult {
    pointer: std::ptr::null(),
    buf_size: BUF_SIZE,
    width: WIN_WIDTH,
    height: WIN_HEIGHT,
};

struct AppState {
    /// Framebuffer
    buf: [u8; BUF_SIZE],
    /// Current frame count
    frame_count: usize,
    /// Graph1 window context
    win_ctx: WindowContext,
}
impl AppState {
    fn new(frame_count: usize) -> Self {
        AppState {
            frame_count,
            win_ctx: WindowContext::new(WIN_WIDTH, WIN_HEIGHT),
            buf: [0x54; BUF_SIZE],
        }
    }
}

/// The Application state, maintained between frames
static mut STATE: Option<AppState> = None;

#[wasm_bindgen]
/// Initialize the WASM module: create `STATE` and populate it with the initial values
pub fn init_state(frame: usize) -> InitStateResult {
    unsafe {
        if STATE.is_none() {
            STATE = Some(AppState::new(frame));
        }

        if let Some(state) = STATE.as_ref() {
            console_log(&format!("Current window context: {:#?}", state.win_ctx));

            return InitStateResult {
                pointer: state.buf.as_ptr(),
                ..DEFAULT_INIT_RESULT
            };
        }
    }

    DEFAULT_INIT_RESULT
}

#[wasm_bindgen]
/// Set current frame (if STATE exists)
pub fn update_frame(frame: usize) {
    unsafe {
        if let Some(state) = STATE.as_mut() {
            state.frame_count = frame;
            // console_log(&format!("Current frame: {}", state.frame_count));

            for pixel in state.buf.chunks_mut(4) {
                pixel[0] = 0xff;
                pixel[1] = 0xaa * frame as u8;
                pixel[2] = 0xbb * frame as u8;
                pixel[3] = 0xcc * frame as u8;
            }
        }
    }
}
