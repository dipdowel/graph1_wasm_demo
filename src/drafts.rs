use crate::demo::user_data::DemoUserData;
use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::palettes::RetroNeon;

pub struct Bouncy {
    /// starting position, x-coordinate
    pub x: i32,
    /// starting position, y-coordinate
    pub y: i32,
    /// starting direction and speed, x-axis
    pub dx: i32,
    /// starting direction and speed, y-axis
    pub dy: i32,
}

impl Default for Bouncy {
    fn default() -> Self {
        Self {
            x: 10,
            y: 10,
            dx: 1,
            dy: 1,
        }
    }
}

fn main() {
    let bouncy_user_data = Some(Bouncy::default());

    // Graph context
    let mut ctx: GraphContext<Bouncy> =
        GraphContext::new(WindowContext::default(), false, false, bouncy_user_data);

    // Screen rendering code is omitted for brevity
}
