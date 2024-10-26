use graph1::draw;
use graph1::graph1_core::context::{GraphContext, WindowContext};
use graph1::primitives::plane::{Dimensions2d, RectArea};
use graph1::utils::color::adapters::rgba_to_abgr;
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

const FOREGROUND_COLOR_RGBA: u32 = 0xbb_22_33_ff;

const BACKGROUND_COLOR_RGBA: u32 = 0xcc_cc_cc_ff;

/// Frame buffer, gets rendered on the HTML canvas
static mut FRAME_BUF: [u32; BUF_SIZE] = [BACKGROUND_COLOR_RGBA; BUF_SIZE];

/// Draft buffer, used for intermediate graphics operations and manipulations
static mut DRAFT_BUF: [u32; BUF_SIZE] = [BACKGROUND_COLOR_RGBA; BUF_SIZE];

/// JavaScript and HTML Canvas use ABGR model, hence
/// the result produced by Graph1 needs to be converted from RGBA to ABGR.
/// JS renders `CANVAS_BUF_ABGR` on the HTML canvas, not `FRAME_BUF` directly.
static mut CANVAS_BUF_ABGR: [u32; BUF_SIZE] = [0xff_ff_00_ff; BUF_SIZE];

/// Window context, a sub-context of the `GraphContext`
const WIN_CTX: WindowContext = WindowContext {
    w: WIN_WIDTH,
    h: WIN_WIDTH,
    w_usize: WIN_WIDTH as usize,
    h_usize: WIN_HEIGHT as usize,
    size: BUF_SIZE,
    background_color: BACKGROUND_COLOR_RGBA,
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

/// Global context container, maintains the state between frames
static mut CONTEXT_CONTAINER: Option<GraphContext> = None;

#[wasm_bindgen]
/// Initialize the WASM module:
/// 1. Create the `GraphContext` context and store it in the global container.
/// 2. Inform the JS-world on where to look for the frame buffer, what its size is, etc.
pub fn init_state(frame: Option<usize>) -> InitStateResult {
    let frame: usize = frame.unwrap_or(0);

    unsafe {
        if CONTEXT_CONTAINER.is_none() {
            let ctx: GraphContext = GraphContext {
                frame_buf: &mut FRAME_BUF,
                draft_buf: Some(&mut DRAFT_BUF),
                user_data: Box::new(Vec::new()),
                win: &WIN_CTX,
                default_color: FOREGROUND_COLOR_RGBA,
                bezier: None,
                frame_count: frame,
                use_alpha: true,
            };

            // Place the context into the global container
            // so that it persists between frames
            CONTEXT_CONTAINER = Some(ctx);

            // Update the JS-world with the details on the frame buffer
            return InitStateResult {
                pointer: CANVAS_BUF_ABGR.as_ptr(),
                ..DEFAULT_INIT_RESULT
            };
        }
    }
    DEFAULT_INIT_RESULT
}

fn clear_screen(ctx: &mut GraphContext) {
    draw::tools::fill::buffer(ctx.frame_buf, ctx.win.background_color);
}

#[wasm_bindgen]
/// Tell the app which frame to render
pub fn update_frame(frame: usize) {
    unsafe {
        if let Some(mut ctx) = CONTEXT_CONTAINER.as_mut() {
            // console_log(format!("Frame: {}", frame).as_str());
            ctx.frame_count = frame;

            // Set the square movement directions
            if frame == 0 {
                &ctx.user_data.push(30); // start for X
                &ctx.user_data.push(12); // start for Y
                &ctx.user_data.push(1); // direction for X
                &ctx.user_data.push(1); // direction for Y
            }

            let mut x = ctx.user_data[0];
            let mut y = ctx.user_data[1];
            let mut dx = ctx.user_data[2];
            let mut dy = ctx.user_data[3];

            // TODO: Continue here!
            // TODO: Make use of `dx` and `dy` to make the square bounce around!

            console_log(format!("x: {}, y: {}, dx: {}, dy: {}", x, y, dx, dy).as_str());

            x = x + dx;
            y = y + dy;

            // if x > ctx.win.w as i32 - 10 || x < 0 {
            //     direction_x = -direction_x;
            // }

            // &ctx.user_data.insert(0, 30); // start for X

            // &ctx.user_data.insert(1, 332);

            // let direction_x= ctx.user_data[0];
            // let direction_y= ctx.user_data[1];

            clear_screen(ctx);

            let local_frame = frame % ctx.win.w_usize;
            let win_middle = ctx.win.w_usize / 2;

            let mut direction = 1;
            if local_frame > win_middle {
                direction = -1;
            }

            let x = frame as u32;
            let y = frame as u32;

            let side = 20;

            draw::rectangle::filled(ctx, &RectArea::square(x, y, side, Some(0xee_44_44_ff)));

            /*
            let c2 = 0x00_00_00_02;
            // With each frame, gradually decrease the Alpha from 0xFF to 0x00
            for pixel in ctx.frame_buf.as_mut() {
                *pixel = rgba_operation(pixel, &c2, &ColorOperation::Subtract, ctx.use_alpha);
            }
            */

            // Convert the internal RGBA buffer to ABGR and write it to `CANVAS_BUF_ABGR`.
            // JS renders `CANVAS_BUF_ABGR` on the HTML canvas, not `FRAME_BUF`.
            rgba_to_abgr(&mut CANVAS_BUF_ABGR, &ctx.frame_buf);
        }
    }
}
