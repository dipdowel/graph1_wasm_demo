use graph1::graph1_core::context::{GraphContext, WindowContext};
use graph1::primitives::primitives::{BufferRGBA, Dimensions2d};
use graph1::utils::color::adapters::rgba_to_abgr;
use graph1::utils::color::math::{rgba_operation, ColorOperation};
use wasm_bindgen::prelude::*;
use web_sys::console;
fn console_log(msg: &str) {
    console::log_1(&msg.into());
}

/// Width of the window, in pixels
const WIN_WIDTH: u32 = 320;
/// Height of the window, in pixels
const WIN_HEIGHT: u32 = 240;
/// Framebuffer size, in bytes
const BUF_SIZE: usize = 4 * (WIN_WIDTH * WIN_HEIGHT) as usize;

const DEFAULT_COLOR_RGBA: u32 = 0x33_33_33_ff;

const BACKGROUND_COLOR_RGBA: u32 = 0xff_33_ee_ff;

static mut FRAME_BUF: [u32; BUF_SIZE] = [BACKGROUND_COLOR_RGBA; BUF_SIZE];
static mut DRAFT_BUF: [u32; BUF_SIZE] = [BACKGROUND_COLOR_RGBA; BUF_SIZE];

// JavaScript and HTML Canvas use ABGR model, so the result produced by Graph1 needs to be converted
static mut CANVAS_BUF_ABGR: [u32; BUF_SIZE] = [0xff_ff_00_ff; BUF_SIZE];

const WIN_CTX: WindowContext = WindowContext {
    w: WIN_WIDTH,
    h: WIN_WIDTH,
    w_usize: WIN_WIDTH as usize,
    h_usize: WIN_HEIGHT as usize,
    size: BUF_SIZE,
    dimensions: Dimensions2d {
        w: WIN_WIDTH,
        h: WIN_HEIGHT,
    },
};

#[wasm_bindgen]
/// This data is passed to the JS-world to render the framebuffer on a 2D-canvas.
pub struct InitStateResult {
    /// Points to the start of the frame buffer
    pub pointer: *const u32,
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

// struct AppState {
//     /// Framebuffer
//     buf: [u8; BUF_SIZE],
//     /// Current frame count
//     frame_count: usize,
//     /// Graph1 context
//     ctx: GraphContext<'static>,
// }
// impl AppState {
//     fn new(frame_count: usize) -> Self {
//         AppState {
//             frame_count,
//             win_ctx: WindowContext::new(WIN_WIDTH, WIN_HEIGHT),
//             buf: [0x54; BUF_SIZE],
//         }
//     }
// }

/// The Application state, maintained between frames
static mut STATE: Option<GraphContext> = None;

#[wasm_bindgen]
/// Initialize the WASM module: create `STATE` and populate it with the initial values
pub fn init_state(frame: Option<usize>) -> InitStateResult {

    let frame:usize = frame.unwrap_or(0);



    unsafe {
        if STATE.is_none() {

            let ctx: GraphContext = GraphContext {
                frame_buf: &mut FRAME_BUF, //&mut [BACKGROUND_COLOR_RGBA; BUF_SIZE],
                draft_buf: Some(&mut  DRAFT_BUF),
                win: &WIN_CTX,
                default_color: DEFAULT_COLOR_RGBA,
                bezier: None,
                frame_count: frame,
            };

            STATE = Some(ctx);
        }

        if let Some(state) = STATE.as_ref() {
            // console_log(&format!("Current window context: {:#?}", state.win_ctx));
            return InitStateResult {
                pointer: CANVAS_BUF_ABGR.as_ptr(),
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

             for pixel in state.frame_buf.as_mut() {
                 *pixel = rgba_operation(pixel, &0x00_00_00_02, &ColorOperation::Subtract);
            }

            // Convert the internal RGBA buffer to ABGR, so it can be rendered on the HTML canvas
            rgba_to_abgr(&mut CANVAS_BUF_ABGR, &state.frame_buf);

        }
    }
}
