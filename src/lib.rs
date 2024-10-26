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

const FOREGROUND_COLOR_RGBA: u32 = 0x11_ee_22_ff;

const BACKGROUND_COLOR_RGBA: u32 = 0x00_44_11_ff;

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
    w_i32: WIN_WIDTH as i32,
    h_i32: WIN_HEIGHT as i32,
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

            ctx.frame_count = frame;

            // TODO: make it a constant!
            let square_side:i32 = 8;

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

            // console_log(format!("x: {}, y: {}, dx: {}, dy: {}", x, y, dx, dy).as_str());

            if x+dx > (ctx.win.w_i32 - square_side) || x + dx < 0{
                dx = -dx;
            }

            if y + dy > (ctx.win.h_i32 - square_side) || y + dy < 0{
                dy = -dy;
            }

            x = x + dx;
            y = y + dy;

            &ctx.user_data.insert(0, x); // current X
            &ctx.user_data.insert(1, y); // current y
            &ctx.user_data.insert(2, dx); // current dx
            &ctx.user_data.insert(3, dy); // current dy

            // console_log(format!("x: {}, y: {}, dx: {}, dy: {}", x, y, dx, dy).as_str());

            clear_screen(ctx);

            let x = x as u32;
            let y = y as u32;
            let side = square_side as u32;

            draw::rectangle::filled(ctx, &RectArea::square(x, y, side, Some(FOREGROUND_COLOR_RGBA)));

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
