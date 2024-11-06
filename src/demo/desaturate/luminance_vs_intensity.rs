use graph1::draw;
use crate::demo::user_data::DemoUserData;
use graph1::draw::rectangle;
use graph1::graph1_core::context::GraphContext;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::palettes;
use graph1::utils::color::desaturate::intensity::rgba_region_intensity;
use graph1::utils::color::desaturate::luminance::rgba_region_luminance;
use graph1::utils::color::palettes::RetroNeon;


/// Number of columns in the background
const NUM_COLUMNS:u32 = 8;


fn render_lanes(ctx: &mut GraphContext<DemoUserData>) {

    // width of each column in the background, it is a 1/8 of the window width
    let column_width = ctx.win.w;
    let column_height = ctx.win.h / 4;

    // Choose a color for each column
    for i in 0..NUM_COLUMNS  {
        let color = match i % NUM_COLUMNS {
            0 => palettes::SunsetGlow::NIGHTFALL_BLUE,
            1 => palettes::SunsetGlow::SOFT_ORANGE,
            2 => palettes::SunsetGlow::DUSTY_LAVENDER,
            3 => palettes::SunsetGlow::DEEP_TEAL,
            _ => palettes::SunsetGlow::DEEP_TEAL,
        };

        // Render the column
        rectangle::filled(
            ctx,
            &RectArea::new( 0, column_height*i, column_width, column_height, Some(color)),
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
fn draw_ghost(ctx: &mut GraphContext<DemoUserData>, frame:u32,  offset_x: u32, offset_y: u32, body_color:u32, eyes_color:u32) {


    let eye_white = 0xff_ff_ff_ff;

    let body_pixels_frame1 = [
        // Row 1: Rounded head
        (2, 0), (3, 0),
        // Row 2
        (1, 1), (2, 1), (3, 1), (4, 1),
        // Row 3
        (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2),
        // Row 4-5: Full body
        (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3),
        (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
        // Row 6: Wavy bottom pattern (frame 1)
        (0, 5), (2, 5), (4, 5),
    ];

    /// Body pixel layout for frame 2 of the ghost animation.
    let body_pixels_frame2 = [
        // Row 1: Rounded head
        (2, 0), (3, 0),
        // Row 2
        (1, 1), (2, 1), (3, 1), (4, 1),
        // Row 3
        (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2),
        // Row 4-5: Full body
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
            &RectArea::square(offset_x + dx * SIDE, offset_y + dy * SIDE, SIDE, Some(body_color)),
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
            &RectArea::square(offset_x -2+ dx * SIDE, offset_y + dy * SIDE, SIDE, Some(eye_white)),
        );
    }

    // Draw pupils
    let pupil_offsets = [
        (1, 3), // Left pupil
        (4, 3), // Right pupil
    ];
    for &(dx, dy) in &pupil_offsets {
        rectangle::filled(
            ctx,
            &RectArea::square(offset_x + dx * SIDE + 2, offset_y + dy * SIDE + 2, SIDE / 2, Some(eyes_color)),
        );
    }
}


/// Illustrates how color Intensity and color Luminance are different
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {

    // Let's have a frame number as u32 for easier calculations
    let mut current_frame = ctx.frame_count as u32;

    draw::tools::fill::buffer(ctx.frame_buf, 0xff_00_ff_ff);

    render_lanes(ctx);

    let dx = current_frame/2+32;

    // let dy = current_frame/(ctx.win.w*2);
    let dy = 0;


    draw_ghost(ctx, current_frame, dx-8, 2-dy, RetroNeon::NEON_PINK, 0x000000ff);
    draw_ghost(ctx, current_frame, dx-16, 66-dy, RetroNeon::CYBER_BLUE, 0x000000ff);
    draw_ghost(ctx, current_frame, dx-24, 126-dy, RetroNeon::ELECTRIC_PURPLE, 0x000000ff);
    draw_ghost(ctx, current_frame, dx-32, 186-dy, RetroNeon::TURQUOISE_TEAL, 0x000000ff);


    let section_width = ctx.win.w / 5;

    rgba_region_intensity(ctx, &RectArea::new(2*section_width, 0, section_width, ctx.win.h, None),false);
    rgba_region_intensity(ctx, &RectArea::new(3*section_width, 0, section_width, ctx.win.h, None), true);
    rgba_region_luminance(ctx, &RectArea::new(4*section_width, 0, section_width, ctx.win.h, None));

    /*


    // Quarter of the window height
    let quarter_height = ctx.win.h / 4;

    // The animation lasts only for the first 120 frames
    if current_frame > 2 * quarter_height {
        return;
    }

    render_background(ctx);

    // The bar moving from top to bottom illustrates the Intensity
    let top_bar_y = current_frame / 2;
    let top_bar = RectArea::new(0, top_bar_y, ctx.win.w, quarter_height, None);

    // The bar moving from bottom to top illustrates the Luminance
    let bottom_bar_y = ctx.win.h - current_frame / 2 - quarter_height;
    let bottom_bar = RectArea::new(0, bottom_bar_y, ctx.win.w, quarter_height, None);

    // Render the top bar with Luminance
    rgba_region_luminance(ctx, &top_bar);

    // Render the bottom bar with Intensity
    rgba_region_intensity(ctx, &bottom_bar, false);

     */
}
