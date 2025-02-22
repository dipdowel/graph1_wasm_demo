use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;
use graph1::draw::rectangle;
use graph1::primitives::plane::RectArea;
use graph1::primitives::point::Point;
use graph1::utils::color::desaturate::intensity::rgba_region_intensity;
use graph1::utils::color::desaturate::luminance::rgba_region_luminance;

use graph1::utils::color::palettes::{RetroNeon, SunsetGlow};

//---------------------------------------------------------------------
// Configure the user data for the Ghosts for luminance_vs_intensity demo
pub struct GhostsUserData {
    pub direction: Point<i32>,
    pub current_point: Point<i32>,
}

pub const GHOSTS_USER_DATA: GhostsUserData = GhostsUserData {
    direction: Point { x: 0, y: 0 },
    current_point: Point { x: 0, y: 0 },
};
//---------------------------------------------------------------------

/// Number of columns in the background
const NUM_LANES: u32 = 4;

fn render_lanes(ctx: &mut GraphContext<DemoUserData>) {
    // make the horizontal lanes where the ghosts will move
    let lane_width = ctx.win.w;
    let lane_height = ctx.win.h / NUM_LANES;

    // Choose a color for each lane
    for i in 0..NUM_LANES {
        let color = match i % NUM_LANES {
            0 => SunsetGlow::NIGHTFALL_BLUE,
            1 => SunsetGlow::SOFT_ORANGE,
            2 => SunsetGlow::DUSTY_LAVENDER,
            3 => SunsetGlow::DEEP_TEAL,
            _ => SunsetGlow::DUSK_GRAY,
        };

        // Render the lane
        rectangle::filled(
            ctx,
            &RectArea::new(0, lane_height * i, lane_width, lane_height, Some(color)),
        );
    }
}

const GHOST_BLOCK_SIZE: u32 = 8; // Pixel size for each block that ghosts are made of
const GHOST_HEIGHT: u32 = GHOST_BLOCK_SIZE * 6; // Height of the ghost, in pixels

/// Draws a ghost character from Pac-Man using pixel blocks.
///
/// # Arguments
/// * `ctx` - The drawing context.
/// * `offset_x` - X-axis starting offset for drawing the ghost.
/// * `offset_y` - Y-axis starting offset for drawing the ghost.
///
/// The ghost is drawn in a 6x7 pixel grid. You can adjust colors and positions as needed.
fn draw_ghost(
    ctx: &mut GraphContext<DemoUserData>,
    frame: u32,
    offset_x: u32,
    offset_y: u32,
    body_color: u32,
    eyes_color: u32,
    dx: i32,
) {
    let eye_white = 0xff_ff_ff_ff;

    // Body pixel layout for frame 1 of the ghost animation.
    #[rustfmt::skip]
    let body_pixels_frame1 = [
        // Row 1: Rounded head
        (2, 0), (3, 0),
        // Row 2
        (1, 1), (2, 1), (3, 1), (4, 1),
        // Row 3
        (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2),
        // Rows 4-5: Full body
        (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3),
        (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
        // Row 6: Wavy bottom pattern (frame 1)
        (0, 5), (2, 5), (4, 5),
    ];

    // Body pixel layout for frame 2 of the ghost animation.
    #[rustfmt::skip]
    let body_pixels_frame2 = [
        // Row 1: Rounded head
        (2, 0), (3, 0),
        // Row 2
        (1, 1), (2, 1), (3, 1), (4, 1),
        // Row 3
        (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2),
        // Rows 4-5: Full body
        (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3),
        (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
        // Row 6: Wavy bottom pattern (frame 2 - shifted)
        (1, 5), (3, 5), (5, 5),
    ];

    let body = if frame % 16 >= 8 {
        body_pixels_frame1
    } else {
        body_pixels_frame2
    };

    // Draw body pixels
    for &(dx, dy) in &body {
        rectangle::filled(
            ctx,
            &RectArea::square(
                offset_x + dx * GHOST_BLOCK_SIZE,
                offset_y + dy * GHOST_BLOCK_SIZE,
                GHOST_BLOCK_SIZE,
                Some(body_color),
            ),
        );
    }

    // Draw eyes (white squares for eyes)
    let eye_pixels = [
        (1, 3), // Left eye
        (4, 3), // Right eye
    ];

    for &(dx, dy) in &eye_pixels {
        rectangle::filled(
            ctx,
            &RectArea::square(
                offset_x - 2 + dx * GHOST_BLOCK_SIZE,
                offset_y + dy * GHOST_BLOCK_SIZE,
                GHOST_BLOCK_SIZE,
                Some(eye_white),
            ),
        );
    }

    // Draw pupils
    let pupil_offsets = [
        (1, 3), // Left pupil
        (4, 3), // Right pupil
    ];

    // helps to direct the eyes to the left or right
    let pupil_dx: i32 = if dx > 0 { 2 } else { -2 };

    for &(dx, dy) in &pupil_offsets {
        rectangle::filled(
            ctx,
            &RectArea::square(
                ((offset_x + dx * GHOST_BLOCK_SIZE) as i32 + pupil_dx) as u32,
                offset_y + dy * GHOST_BLOCK_SIZE + 2,
                GHOST_BLOCK_SIZE / 2,
                Some(eyes_color),
            ),
        );
    }
}

/// Illustrates how color Intensity and color Luminance are different
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let mut current_frame = ctx.frame_count as i32;

    // initialize the data maintained between frames
    if current_frame == 0 {
        ctx.user_data.ghosts.current_point.x = 26;
        ctx.user_data.ghosts.current_point.y = 0;
        ctx.user_data.ghosts.direction.x = 1;
        ctx.user_data.ghosts.direction.y = 0;
        return;
    }

    // Clear the screen
    draw::tools::fill::buffer(&mut ctx.frame_buf, 0xff_00_ff_ff, ctx.num_threads);

    // Change direction when the ghost approaches the edge of the screen
    if current_frame % (ctx.win.w_i32 - 106) == 0 {
        ctx.user_data.ghosts.direction.x *= -1;
    }

    // update the position of the ghosts
    let dx = ctx.user_data.ghosts.direction.x;
    ctx.user_data.ghosts.current_point.x += dx;

    let pos_x = ctx.user_data.ghosts.current_point.x as u32;

    // Render the background lanes
    render_lanes(ctx);

    let y = ctx.win.h / NUM_LANES;

    let current_frame = current_frame as u32;

    // draw 4 ghosts with a horizontal offset to animate them

    let lane_height = ctx.win.h / NUM_LANES;
    let y_offset = lane_height / 2 - GHOST_HEIGHT / 2;

    // NB: Number of threads is relevant for non-web environments.
    // NB: Currently, only 1 thread is expected to be used in the browser.
    // Temporarily set the number of threads to 1, since we draw many small rectangles.
    // It's faster to use a single thread to avoid all the thread creation overhead in such case.
    let num_threads = ctx.num_threads;
    ctx.num_threads = 1;

    draw_ghost(
        ctx,
        current_frame,
        pos_x + 6,
        0 * y + y_offset,
        RetroNeon::NEON_PINK,
        0x000000ff,
        dx,
    );
    draw_ghost(
        ctx,
        current_frame,
        pos_x + 12,
        1 * y + y_offset,
        RetroNeon::CYBER_BLUE,
        0x000000ff,
        dx,
    );
    draw_ghost(
        ctx,
        current_frame,
        pos_x + 18,
        2 * y + y_offset,
        RetroNeon::ELECTRIC_PURPLE,
        0x000000ff,
        dx,
    );
    draw_ghost(
        ctx,
        current_frame,
        pos_x + 24,
        3 * y + y_offset,
        RetroNeon::TURQUOISE_TEAL,
        0x000000ff,
        dx,
    );

    // Done drawing many small rectangles (the ghosts), restore the number of threads
    ctx.num_threads = num_threads;


    // DRAW THE 3 VERTICAL DESATURATION SECTIONS

    // width of each desaturated section
    let section_width = ctx.win.w / 5;

    // Section one, the basic intensity desaturation
    rgba_region_intensity(
        ctx,
        &RectArea::new(1 * section_width, 0, section_width, ctx.win.h, None),
        false,
    );

    // Section two, the "physical" intensity desaturation ()
    rgba_region_intensity(
        ctx,
        &RectArea::new(2 * section_width, 0, section_width, ctx.win.h, None),
        true,
    );
    rgba_region_luminance(
        ctx,
        &RectArea::new(3 * section_width, 0, section_width, ctx.win.h, None),
    );
}
