use crate::demo::user_data::{Bouncy, DemoUserData};
use crate::utils::clear_screen;
use graph1::draw;
use graph1::graph1_core::context::GraphContext;
use graph1::primitives::plane::RectArea;

/// Side of Bouncy, in pixels
const SQUARE_SIDE_PX:i32 = 16;

/// Render a frame with Bouncy, who is just a square bouncing on the screen
/// Bouncy is the hero of this demo.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>){

    // Initialize the animation variables on the zero-th frame
    if ctx.frame_count == 0 {
        // Set the starting `x` and `y` for Bouncy.
        ctx.user_data.bouncy.x = 0;
        ctx.user_data.bouncy.y = 27;

        // Set the starting per-frame position increments for `x` and `y`
        // So on each frame we'll be changing `x` and `y` by `dx` and `dy` correspondingly.
        ctx.user_data.bouncy.dx = 2;
        ctx.user_data.bouncy.dy = 2;
    }

    // Read the animation values from the context
    let Bouncy { mut x, mut y, mut dx, mut dy } = ctx.user_data.bouncy;

    // Don't let Bouncy go off-screen horizontally
    if x+dx > (ctx.win.w_i32 - SQUARE_SIDE_PX) || x + dx < 0{
        dx = -dx;
    }

    // Don't let Bouncy go off-screen vertically
    if y + dy > (ctx.win.h_i32 - SQUARE_SIDE_PX) || y + dy < 0{
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

    clear_screen(ctx);

    let x = x as u32;
    let y = y as u32;
    let side = SQUARE_SIDE_PX as u32;

    // Finally, render Bouncy on the window surface
    draw::rectangle::filled(ctx, &RectArea::square(x, y, side, None));
}