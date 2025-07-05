use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::draw::rectangle;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::adapters::rgba_to_abgr;
use graph1::utils::color::palettes;
use graph1::utils::color::palettes::RetroNeon;
use crate::{BUF_LEN, CANVAS_BUF_ABGR};

/// Our bouncing hero!
struct Bouncy {
    /// starting position, x-coordinate
    pub x: i32,
    /// starting position, y-coordinate
    pub y: i32,
    /// starting direction and speed, x-axis
    pub dx: i32,
    /// starting direction and speed, y-axis
    pub dy: i32,
}

struct UserData {
    bouncy: Bouncy,
}

impl Default for UserData {
    // The user data type must implement `Default` trait!
    fn default() -> Self {
        Self {
            bouncy: Bouncy {
                x: 10,
                y: 10,
                dx: 1,
                dy: 1,
            },
        }
    }
}

fn main() {
    let bouncy_user_data = Some(UserData::default());
    let mut ctx: GraphContext<UserData> =
        GraphContext::new(WindowContext::default(),
                          false,
                          0,
                          bouncy_user_data,
                          1,
                          None

        );

    // Here should be the animation loop, which calls `render_frame()`
    // with a pre-defined frame rate. The animation loop logic depends
    // on your rendering library. In the end of this page you'll find information
    // on how to make an animation loop with `minifb` in a native application,
    // as well as how it can be done on the web.
}

const SQUARE_SIDE_PX: i32 = 16;

/// Render a frame with Bouncy, who is just a square bouncing on the screen
/// Bouncy is the hero of this demo.
pub fn render_frame(ctx: &mut GraphContext<UserData>) {
    // Read the animation values from the user data in the context
    let Bouncy {
        mut x,
        mut y,
        mut dx,
        mut dy,
    } = ctx.user_data.bouncy;

    // Move Bouncy by 1 step.
    x = x + dx;
    y = y + dy;

    // Write the updated animation values back to the context.
    ctx.user_data.bouncy.x = x;
    ctx.user_data.bouncy.y = y;
    ctx.user_data.bouncy.dx = dx;
    ctx.user_data.bouncy.dy = dy;

    // take a blue color `0x00_9a_ff` and
    // set the alpha channel to `0x70` (112 in decimal)
    let color:u32 = 0x00_9a_ff_70;



    draw::rectangle::filled(ctx, &RectArea::square(0, 0, 100, Some(color)));

    let buf_len = ctx.frame_buf.len();
    let mut canvas_buf_abgr: Vec<u32> = vec![0x00; buf_len];
    rgba_to_abgr(&mut canvas_buf_abgr, &ctx.frame_buf, true).unwrap();


}
