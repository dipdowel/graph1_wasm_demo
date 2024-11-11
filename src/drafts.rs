use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::plane::RectArea;

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
                          false,
                          bouncy_user_data
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

    // Finally, render Bouncy on the window surface
    draw::rectangle::filled(ctx, &RectArea::square(x as u32, y as u32, 20, None));
}
