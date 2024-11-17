use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw::rectangle;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::palettes;

//---------------------------------------------------------------------
// Configure the user data for Alpha demo (very similar to Bouncy demo)
pub struct AlphaUserData {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
}
pub const ALPHA_USER_DATA:AlphaUserData = AlphaUserData {
    x: 10,
    y: 10,
    dx: 1,
    dy: 1,
};
//---------------------------------------------------------------------

/// Side of Alpha Bouncy, in pixels
const SQUARE_SIDE_PX: i32 = 96;

/// Render the background of the window
/// **NB:** In real-world applications complete redrawing of the background on every frame
/// is quite wasteful CPU-wise. In this demo we do it just for simplicity.
fn render_background(ctx: &mut GraphContext<DemoUserData>) {
    // Width of each column in the background is 20% of the window width
    let column_width = (ctx.win.w as f64 / 100.0 * 20.0) as u32;
    // Number of columns in the background
    let num_columns = ctx.win.w / column_width;

    // Render the background columns with alternating colors
    for i in 0..num_columns {
        let color = match i % num_columns {
            0 => palettes::DesertDusk::RUSTY_ORANGE,
            1 => palettes::DesertDusk::EARTH_RED,
            2 => palettes::DesertDusk::DESERT_ROSE,
            3 => palettes::DesertDusk::CLAY_BROWN,
            4 => palettes::DesertDusk::GOLDEN_SAND,
            _ => palettes::DesertDusk::SHADOW_BROWN,
        };

        rectangle::filled(
            ctx,
            &RectArea::new(i * column_width, 0, column_width, ctx.win.h, Some(color)),
        );
    }
}

const BOUNCY_COLOR_WITH_ALPHA: u32 = 0xff_33_00_85;

/// Render a frame with Bouncy, who is just a square bouncing on the screen
/// Bouncy is the hero of this demo.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // Initialize the animation variables on the zero-th frame
    if ctx.frame_count == 0 {
        // =====================================================================================
        //  By setting `use_alpha` to `true`, we instruct Graph1 to use alpha blending
        //  with each tool that supports it. Think of drawing rectangles, circles, lines, etc.
        // =====================================================================================
        ctx.alpha.enabled = true;

        // Set the starting `x` and `y` for Bouncy.
        ctx.user_data.alpha.x = 0;
        ctx.user_data.alpha.y = ctx.win.h_i32 / 2 - SQUARE_SIDE_PX / 2;

        // Set the increments for `x` and `y`
        // At each frame we'll be adding `dx` to `x` and  `dy` to `y` to move Bouncy around.
        // If `dx` is positive, Bouncy will move to the right, if negative, to the left.
        // If `dy` is positive, Bouncy will move down, if negative -- up.
        ctx.user_data.alpha.dx = 1;
        ctx.user_data.alpha.dy = 1;
    }


    // Read the animation values from the context
    let AlphaUserData {
        mut x,
        mut y,
        mut dx,
        mut dy,
    } = ctx.user_data.alpha;

    // Don't let Bouncy go off-screen horizontally
    if x + dx > (ctx.win.w_i32 - SQUARE_SIDE_PX) || x + dx < 0 {
        dx = -dx;
        // If Bouncy hits the wall on the right or left, toggle the `use_alpha` flag!
        ctx.alpha.enabled = !ctx.alpha.enabled;
    }

    // Don't let Bouncy go off-screen vertically
    if y + dy > (ctx.win.h_i32 - SQUARE_SIDE_PX) || y + dy < 0 {
        dy = -dy;
    }

    // Move Bouncy by 1 step.
    x = x + dx;
    y = y + dy;

    // Write the updated animation values back to the context.
    ctx.user_data.alpha.x = x;
    ctx.user_data.alpha.y = y;
    ctx.user_data.alpha.dx = dx;
    ctx.user_data.alpha.dy = dy;

    // Render the background with colorful columns
    render_background(ctx);

    let x = x as u32;
    let y = y as u32;
    let side = SQUARE_SIDE_PX as u32;

    // Finally, render Alpha Bouncy on the window surface
    rectangle::filled(ctx, &RectArea::square(x, y, side, Some(BOUNCY_COLOR_WITH_ALPHA)));

}
