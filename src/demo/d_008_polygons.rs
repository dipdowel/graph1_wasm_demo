use graph1::core::context::GraphContext;
use graph1::draw::polygons;
use graph1::draw::polygons::{closed_perimeter, PolygonProperties};
use graph1::primitives::Pixel;
use graph1::primitives::point::Point;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::{Grayscale, RetroNeon, SunsetGlow};
use crate::demo::user_data::DemoUserData;

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
        ctx.win.background_color = SunsetGlow::GENTLE_INDIGO;
        /*
        let vertices = &vec![
            Point { x: 210, y: 110 },
            Point { x: 250, y: 110 },
            Point { x: 270, y: 145 },
            Point { x: 250, y: 180 },
            Point { x: 210, y: 180 },
            Point { x: 190, y: 145 }
        ];

        closed_perimeter(ctx, &vertices, None);
        return;
         */
    }

    // Clear the screen
    clear_screen(ctx);


        polygons::polygon(ctx, &PolygonProperties{
            center: ctx.win.center.to_pixel(RetroNeon::HOT_PINK),
            num_sides: 3,
            radius: 60,
            rotation_angle: current_frame as f64 * 2.5,
            skip_rendering: false,
        });






    // Change direction when the ghost approaches the edge of the screen
    // if current_frame % (ctx.win.w_i32 - 106) == 0 {
    //     ctx.user_data.ghosts.direction.x *= -1;
    // } 
}
