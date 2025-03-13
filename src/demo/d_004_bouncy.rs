use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;
use graph1::fx::scanline;
use graph1::primitives::plane::RectArea;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

//---------------------------------------------------------------------
// Configure the user data for Bouncy demo
pub struct BouncyUserData {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
}
pub const BOUNCY_USER_DATA: BouncyUserData = BouncyUserData {
    x: 10,
    y: 10,
    dx: 1,
    dy: 1,
};
//---------------------------------------------------------------------

/// Side of Bouncy, in pixels
const SQUARE_SIDE_PX: i32 = 16;

/// Render a frame with Bouncy, who is just a square bouncing on the screen
/// Bouncy is the hero of this demo.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // Initialize the animation variables on the zero-th frame
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::ELECTRIC_BLUE;
        ctx.win.foreground_color = RetroNeon::LASER_LIME;
        // Set the starting `x` and `y` for Bouncy.
        ctx.user_data.bouncy.x = 0;
        ctx.user_data.bouncy.y = 27;

        // So on each frame we'll be changing `x` and `y` by `dx` and `dy` correspondingly.
        ctx.user_data.bouncy.dx = 2;
        ctx.user_data.bouncy.dy = 2;
    }

    // Read the animation values from the context
    let BouncyUserData {
        mut x,
        mut y,
        mut dx,
        mut dy,
    } = ctx.user_data.bouncy;

    // Don't let Bouncy go off-screen horizontally
    if x + dx > (ctx.win.w_i32 - SQUARE_SIDE_PX) || x + dx < 0 {
        dx = -dx;
    }

    // Don't let Bouncy go off-screen vertically
    if y + dy > (ctx.win.h_i32 - SQUARE_SIDE_PX) || y + dy < 0 {
        dy = -dy;
    }

    // Move Bouncy by 1 step.
    x = x + dx;
    y = y + dy;

    // Write the updated animation values back to the context.
    ctx.user_data.bouncy.x = x;
    ctx.user_data.bouncy.y = y;
    ctx.user_data.bouncy.dx = dx;
    ctx.user_data.bouncy.dy = dy;

    // Clearing the screen on every frame is expensive,
    // and it might not perform well with a large size of the window on all platforms.
    // However, for the sake of this demo, we'll clear the screen on every frame.
    clear_screen(ctx);

    let x = x as u32;
    let y = y as u32;
    let side = SQUARE_SIDE_PX as u32;

    // Finally, render Bouncy on the window surface
    draw::rectangle::filled(ctx, &RectArea::square(x, y, side, None));

    // apply the scanline effect
    scanline::window(ctx, 1, 0x20);
}
