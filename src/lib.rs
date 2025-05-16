mod drafts;
mod utils;

use crate::demo::user_data::DemoUserData;

use crate::demo::{d_000_intro, d_001_basic_concepts_pt1, d_002_basic_concepts_pt2, d_003_basic_concepts_pt3, d_004_bouncy, d_005_alpha, d_006_luminance_vs_intensity, d_007_shapes_and_quadrants, d_008_polygons, d_009_lines, d_010_quadrants, d_011_curves, d_012_grid};
use crate::utils::console_log;
use graph1::core::context::{GraphContext, WindowContext};
use graph1::utils::color;
use graph1::utils::color::adapters::{rgba_to_abgr, rgba_to_abgr_unsafe};
use graph1::utils::color::palettes::RetroNeon;
/// A collection of demo modules
pub mod demo {
    pub mod d_000_intro;

    pub mod common {
        pub mod basic_concepts;
        pub mod gemstones;
    }
    pub mod d_001_basic_concepts_pt1;
    pub mod d_002_basic_concepts_pt2;
    pub mod d_003_basic_concepts_pt3;
    /// Bouncy demo. Helps to understand the basics of rendering and animation.
    pub mod d_004_bouncy;

    pub mod d_006_luminance_vs_intensity;

    /// Alpha blending demo
    pub mod d_005_alpha;

    pub mod d_007_shapes_and_quadrants;
    pub mod d_008_polygons;

    pub mod d_009_lines;
    pub mod d_010_quadrants;
    pub mod d_011_curves;
    pub mod d_012_grid;


    /// User data, used to store arbitrary data that needs to be persisted between frames
    pub mod user_data;

    pub mod elements {
        pub mod cube;
        pub mod snake;
    }
    //     /// A minimal example of displaying and animating a square on the screen
    //     pub mod bouncy;
    //     /// Demo of slower but more accurate Float alpha blending
    //     pub mod bouncy_alpha_float;
    //     /// Demo of fast but less accurate Integer alpha blending
    //     pub mod bouncy_alpha_int;
    // }
    // pub mod desaturate {
    //     pub mod luminance_vs_intensity;
    // }
    //
    // pub mod shapes {
    //     pub mod circles;
    // }
    //
    // pub mod screen_saver;
}

use wasm_bindgen::prelude::*;

/// Width of the window, in pixels
const WIN_WIDTH: u32 = 480;
/// Height of the window, in pixels
const WIN_HEIGHT: u32 = 240;
/// Framebuffer size, in pixels (u32)
const BUF_LEN: usize = (WIN_WIDTH * WIN_HEIGHT) as usize;
/// Framebuffer size, in bytes
const BUF_SIZE_BYTES: usize = BUF_LEN * 4;

/// JavaScript and HTML Canvas use ABGR model, hence
/// the result produced by Graph1 needs to be converted from RGBA to ABGR.
/// JS renders `CANVAS_BUF_ABGR` on the HTML canvas, not `FRAME_BUF` directly.
static mut CANVAS_BUF_ABGR: [u32; BUF_LEN] = [0xff_ff_00_ff; BUF_LEN];

static mut ACTIVE_DEMO_ID: u32 = 0;

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
    buf_size: BUF_SIZE_BYTES,
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
            let win_ctx = WindowContext::new(
                WIN_WIDTH,
                WIN_HEIGHT,
                Some(RetroNeon::CYBER_BLUE), // background color RGBA
                Some(RetroNeon::LASER_LIME), // foreground color RGBA
            );

            // Create a context with the basic configuration
            let ctx: GraphContext<DemoUserData> =
                GraphContext::new(win_ctx, true, true, None, 1, None);

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
pub fn set_active_demo(id: u32) {
    console_log(format!("WASM: set_active_demo(), id:{}", id).as_str());
    unsafe {
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
    // don't overflow the frame counter
    if frame > usize::MAX - 1 {
        return PixelStats {
            average_red: 0,
            average_green: 0,
            average_blue: 0,
            average_color: 0,
            average_luminance: 0,
            average_intensity: 0,
        };
    }

    unsafe {
        if let Some(mut ctx) = CONTEXT_CONTAINER.as_mut() {
            ctx.frame_count = frame;

            match ACTIVE_DEMO_ID {
                0 => d_000_intro::render_frame(&mut ctx),
                1 => d_001_basic_concepts_pt1::render_frame(&mut ctx),
                2 => d_002_basic_concepts_pt2::render_frame(&mut ctx),
                3 => d_003_basic_concepts_pt3::render_frame(&mut ctx),
                4 => d_004_bouncy::render_frame(&mut ctx),
                5 => d_005_alpha::render_frame(&mut ctx),
                6 => d_006_luminance_vs_intensity::render_frame(&mut ctx),
                7 => d_007_shapes_and_quadrants::render_frame(&mut ctx),
                8 => d_008_polygons::render_frame(&mut ctx),
                9 => d_009_lines::render_frame(&mut ctx),
                10 => d_010_quadrants::render_frame(&mut ctx),
                11 => d_011_curves::render_frame(&mut ctx),
                12 => d_012_grid::render_frame(&mut ctx),

                // 5 => circles::render_frame(&mut ctx),
                _ => d_011_curves::render_frame(&mut ctx),
            }

            // console_log(&format!("CANVAS_BUF_ABGR size: {:?}", CANVAS_BUF_ABGR.len()));
            // console_log(&format!("ctx.frame_buf size: {:?}", ctx.frame_buf.len()));

            // Convert the internal RGBA buffer to ABGR and write it to `CANVAS_BUF_ABGR`.
            // JS renders `CANVAS_BUF_ABGR` on the HTML canvas, not `FRAME_BUF`.
            let stats = rgba_to_abgr(&mut CANVAS_BUF_ABGR, &ctx.frame_buf, true).unwrap();
            // let stats = rgba_to_abgr_unsafe(&mut CANVAS_BUF_ABGR, &ctx.frame_buf, true).unwrap();

            return PixelStats {
                average_red: stats.average_red,
                average_green: stats.average_green,
                average_blue: stats.average_blue,
                average_color: stats.average_color.clone(),
                average_luminance: color::desaturate::luminance::rgba_pixel_luminance(
                    stats.average_color,
                ) as u32,
                average_intensity: color::desaturate::intensity::rgba_pixel_intensity(
                    stats.average_color,
                    false,
                ) as u32,
            };
        }
    }

    PixelStats {
        average_red: 0,
        average_green: 0,
        average_blue: 0,
        average_color: 0,
        average_luminance: 0,
        average_intensity: 0,
    }
}
