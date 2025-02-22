use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;
use graph1::primitives::point::Point;

//---------------------------------------------------------------------
// Configure the user data
// pub struct ShapesUserData {
//     pub direction: Point<i32>,
//     pub current_point: Point<i32>,
// }
// 
// pub const SHAPES_USER_DATA: ShapesUserData = ShapesUserData {
//     direction: Point { x: 0, y: 0 },
//     current_point: Point { x: 0, y: 0 },
// };
//---------------------------------------------------------------------
 
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
    draw::tools::fill::buffer(&mut ctx.frame_buf, 0xff_00_ff_00, ctx.num_threads);

    // Change direction when the ghost approaches the edge of the screen
    // if current_frame % (ctx.win.w_i32 - 106) == 0 {
    //     ctx.user_data.ghosts.direction.x *= -1;
    // } 
}
