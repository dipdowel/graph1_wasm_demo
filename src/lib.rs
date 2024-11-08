mod utils;
mod drafts;


use crate::demo::desaturate::luminance_vs_intensity;
use crate::demo::shapes::circles;
use crate::demo::user_data::DemoUserData;
use crate::demo::x01_bouncy::{bouncy, bouncy_alpha_float, bouncy_alpha_int};
use crate::utils::console_log;
use graph1::core::context::{GraphContext, WindowContext};

use graph1::primitives::plane::Dimensions2d;
use graph1::utils::color;
use graph1::utils::color::adapters::rgba_to_abgr;
use graph1::utils::color::palettes::RetroNeon;
use wasm_bindgen::prelude::*;

/// A collection of demo modules
pub mod demo {
    /// User data, used to store arbitrary data that needs to be persisted between frames
    pub mod user_data;
    /// Bouncy demo. Helps to understand the basics of rendering and animation.
    pub mod x01_bouncy {
        /// A minimal example of displaying and animating a square on the screen
        pub mod bouncy;
        /// Demo of fast but less accurate Integer alpha blending
        pub mod bouncy_alpha_int;
        /// Demo of slower but more accurate Float alpha blending
        pub mod bouncy_alpha_float;
    }
    pub mod desaturate {
        pub mod luminance_vs_intensity;
    }

    pub mod shapes {
        pub mod circles;
    }

    pub mod screen_saver;

}

/// Width of the window, in pixels
const WIN_WIDTH: u32 = 480;
/// Height of the window, in pixels
const WIN_HEIGHT: u32 = 240;
/// Framebuffer size, in bytes
const BUF_SIZE: usize = 4 * (WIN_WIDTH * WIN_HEIGHT) as usize;

// const FOREGROUND_COLOR_RGBA: u32 = RetroNeon::LASER_LIME;
const FOREGROUND_COLOR_RGBA: u32 = RetroNeon::LASER_LIME;

const BACKGROUND_COLOR_RGBA: u32 = RetroNeon::CYBER_BLUE;

/// Frame buffer, gets rendered on the HTML canvas
static mut FRAME_BUF: [u32; BUF_SIZE] = [BACKGROUND_COLOR_RGBA; BUF_SIZE];

/// Draft buffer, used for intermediate graphics operations and manipulations
static mut DRAFT_BUF: [u32; BUF_SIZE] = [BACKGROUND_COLOR_RGBA; BUF_SIZE];

/// JavaScript and HTML Canvas use ABGR model, hence
/// the result produced by Graph1 needs to be converted from RGBA to ABGR.
/// JS renders `CANVAS_BUF_ABGR` on the HTML canvas, not `FRAME_BUF` directly.
static mut CANVAS_BUF_ABGR: [u32; BUF_SIZE] = [0xff_ff_00_ff; BUF_SIZE];

static mut ACTIVE_DEMO_ID: u32 = 0;

/// Window context, a sub-context of the `GraphContext`
const WIN_CTX: WindowContext = WindowContext {
    w: WIN_WIDTH,
    h: WIN_HEIGHT,
    w_usize: WIN_WIDTH as usize,
    h_usize: WIN_HEIGHT as usize,
    w_i32: WIN_WIDTH as i32,
    h_i32: WIN_HEIGHT as i32,
    background_color: BACKGROUND_COLOR_RGBA,
    foreground_color: FOREGROUND_COLOR_RGBA,
    dimensions: Dimensions2d {
        w: WIN_WIDTH,
        h: WIN_HEIGHT,
    },
};

#[wasm_bindgen]
/// This data is passed back to the JS-world to render the framebuffer on a 2D-canvas.
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

/// A default instance of `InitStateResult`,
const DEFAULT_INIT_RESULT: InitStateResult = InitStateResult {
    pointer: std::ptr::null(),
    buf_size: BUF_SIZE,
    width: WIN_WIDTH,
    height: WIN_HEIGHT,
};

/// Global context container, maintains the state between frames
static mut CONTEXT_CONTAINER: Option<GraphContext<DemoUserData>> = None;

#[wasm_bindgen]
/// Initialize the WASM module:
/// 1. Create the `GraphContext` context and store it in the global container.
/// 2. Inform the JS-world on where to look for the frame buffer, what its size is, etc.
pub fn init_state(frame: Option<usize>) -> InitStateResult {

    let frame: usize = frame.unwrap_or(0);
    unsafe {
        if CONTEXT_CONTAINER.is_none() {
            // Create the application context,
            // which will be used to pass around the data essential for using `Graph1`

            // Create a context with the basic configuration
            let mut ctx: GraphContext<DemoUserData> = GraphContext::new(
                &WIN_CTX,
                &mut FRAME_BUF,
                true,
                None,
            );

            // Apply additional context configuration
            //----------------------------------------
            ctx.draft_buf = Some(&mut DRAFT_BUF); // A draft buffer for intermediate graphics operations


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

#[wasm_bindgen]
/// Tell the app which frame to render
pub  fn set_active_demo(id: u32) {
    console_log(format!("WASM: set_active_demo(), id:{}", id).as_str());
    unsafe{
        ACTIVE_DEMO_ID = id;
    }
}

#[wasm_bindgen]
pub struct PixelStats {
    pub average_red: u32,
    pub average_green: u32,
    pub average_blue: u32,
    pub average_color: u32,
    pub average_luminance: u32,
    pub average_intensity: u32,
}

#[wasm_bindgen]
/// Tell the app which frame to render
pub fn update_frame(frame: usize) -> PixelStats {

    unsafe {
        if let Some(mut ctx) = CONTEXT_CONTAINER.as_mut() {
            ctx.frame_count = frame;

            match ACTIVE_DEMO_ID {
                0 => bouncy::render_frame(&mut ctx),
                1 => bouncy::render_frame(&mut ctx),
                2 => bouncy_alpha_int::render_frame(&mut ctx),
                3 => bouncy_alpha_float::render_frame(&mut ctx),
                4 => luminance_vs_intensity::render_frame(&mut ctx),
                5 => circles::render_frame(&mut ctx),
                // FIXME: rename `screen_saver` to `test_card`
                _ => bouncy::render_frame(&mut ctx),
            }


            /*
            let c2 = 0x00_00_00_02;
            // With each frame, gradually decrease the Alpha from 0xFF to 0x00
            for pixel in ctx.frame_buf.as_mut() {
                *pixel = rgba_operation(pixel, &c2, &ColorOperation::Subtract, ctx.use_alpha);
            }
            */

            // Convert the internal RGBA buffer to ABGR and write it to `CANVAS_BUF_ABGR`.
            // JS renders `CANVAS_BUF_ABGR` on the HTML canvas, not `FRAME_BUF`.
            let stats = rgba_to_abgr(&mut CANVAS_BUF_ABGR, &ctx.frame_buf, true).unwrap();

            return PixelStats{
                average_red: stats.average_red,
                average_green: stats.average_green,
                average_blue: stats.average_blue,
                average_color: stats.average_color.clone(),
                average_luminance: color::desaturate::luminance::rgba_pixel_luminance(stats.average_color) as u32,
                average_intensity: color::desaturate::intensity::rgba_pixel_intensity(stats.average_color, false) as u32,
            }

        }
    }

    PixelStats{
        average_red: 0,
        average_green: 0,
        average_blue: 0,
        average_color: 0,
        average_luminance: 0,
        average_intensity: 0,
    }
}

