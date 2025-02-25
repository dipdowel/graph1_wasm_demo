use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw::polygons;
use graph1::draw::polygons::{PolygonProperties, StarProperties};
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::{RetroNeon, SunsetGlow};

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
 
/// Illustrate the use of polygons and stars
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let current_frame = ctx.frame_count as i32;
    let current_frame_f64 = ctx.frame_count as f64;

    // initialize the context
    if current_frame == 0 {
        ctx.win.background_color = SunsetGlow::GENTLE_INDIGO;
    }

    // Clear the screen
    clear_screen(ctx);


        polygons::polygon(ctx, &PolygonProperties{
            center: ctx.win.center.to_pixel(RetroNeon::HOT_PINK),
            num_sides: 5,
            radius: 80,
            rotation_angle: current_frame_f64 * 1.8,
            skip_rendering: false,
        });

    polygons::star(ctx, &StarProperties{
        center: ctx.win.center.to_pixel(RetroNeon::HOT_PINK),
        num_rays: 5,
        inner_radius: 28,
        outer_radius: 60,
        rotation_angle: current_frame_f64 * 1.2,
        skip_rendering: false,
    });
  
}
