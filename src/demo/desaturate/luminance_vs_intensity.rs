use crate::demo::user_data::DemoUserData;
use graph1::draw;
use graph1::draw::rectangle;
use graph1::graph1_core::context::GraphContext;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::desaturate::intensity::rgba_region_intensity;
use graph1::utils::color::desaturate::luminance::rgba_region_luminance;

use graph1::utils::color::palettes::{RetroNeon, SunsetGlow};

/// Number of columns in the background
const NUM_COLUMNS: u32 = 8;

fn render_lanes(ctx: &mut GraphContext<DemoUserData>) {
    // make the horizontal lanes where the ghosts will move
    let lane_width = ctx.win.w;
    let lane_height = ctx.win.h / 4;

    // Choose a color for each lane
    for i in 0..NUM_COLUMNS {
        let color = match i % NUM_COLUMNS {
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

const SIDE: u32 = 8; // Pixel size for each square unit

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
                offset_x + dx * SIDE,
                offset_y + dy * SIDE,
                SIDE,
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
                offset_x - 2 + dx * SIDE,
                offset_y + dy * SIDE,
                SIDE,
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
                ((offset_x + dx * SIDE) as i32 + pupil_dx) as u32,
                offset_y + dy * SIDE + 2,
                SIDE / 2,
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
    draw::tools::fill::buffer(ctx.frame_buf, 0xff_00_ff_ff);



    // Change direction when the ghost approaches the edge of the screen
    if current_frame % (ctx.win.w_i32 - 66) == 0 {
        ctx.user_data.ghosts.direction.x *= -1;
    }

    // update the position of the ghosts
    let dx = ctx.user_data.ghosts.direction.x;
    ctx.user_data.ghosts.current_point.x += dx;

    let pos_x = ctx.user_data.ghosts.current_point.x as u32;

    // Render the background lanes
    render_lanes(ctx);

    let y = ctx.win.h / 4;

    let current_frame = current_frame as u32;

    // draw 4 ghosts with a horizontal offset to animate them

    draw_ghost(
        ctx,
        current_frame,
        pos_x - 6,
        0 * y + 6,
        RetroNeon::NEON_PINK,
        0x000000ff,
        dx,
    );
    draw_ghost(
        ctx,
        current_frame,
        pos_x - 12,
        1 * y + 6,
        RetroNeon::CYBER_BLUE,
        0x000000ff,
        dx,
    );
    draw_ghost(
        ctx,
        current_frame,
        pos_x - 18,
        2 * y + 6,
        RetroNeon::ELECTRIC_PURPLE,
        0x000000ff,
        dx,
    );
    draw_ghost(
        ctx,
        current_frame,
        pos_x - 24,
        3 * y + 6,
        RetroNeon::TURQUOISE_TEAL,
        0x000000ff,
        dx,
    );

    // Draw the 3 vertical desaturation sections

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
