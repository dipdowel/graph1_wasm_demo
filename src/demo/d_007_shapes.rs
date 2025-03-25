use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;
use graph1::fx::{glitch, scanline};
use graph1::fx::glitch::HorizontalGlitchProps;
use graph1::fx::scanline::window;
use graph1::primitives::Pixel;
use graph1::primitives::plane::RectArea;
use graph1::primitives::point::Point;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::math::oscillator;

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


    // initialize the data maintained between frames
    // if current_frame == 0 {
    //     ctx.user_data.ghosts.current_point.x = 26;
    //     ctx.user_data.ghosts.current_point.y = 0;
    //     ctx.user_data.ghosts.direction.x = 1;
    //     ctx.user_data.ghosts.direction.y = 0;
    //     return;
    // }

    clear_screen(ctx);

    let c:(Pixel, Pixel, Pixel, Pixel) = (
        ctx.win.quadrants.top_left.center().to_pixel(RetroNeon::CIRCUIT_GREEN),
        ctx.win.quadrants.top_right.center().to_pixel(RetroNeon::ACID_GREEN),
        ctx.win.quadrants.bottom_left.center().to_pixel(RetroNeon::LASER_LIME),
        ctx.win.quadrants.bottom_right.center().to_pixel(RetroNeon::CYBER_YELLOW),
    );

    let max_radius = ctx.win.quadrants.bottom_left.width()/4;

    draw::circle::filled(ctx, &c.3, oscillator::sine(ctx.frame_count + 0, 0.02, 4, max_radius) as u32, 0);
    draw::circle::filled(ctx, &c.1, oscillator::sine(ctx.frame_count + 35, 0.02, 4, max_radius)  as u32, 0);
    draw::circle::filled(ctx, &c.2, oscillator::sine(ctx.frame_count  + 70, 0.02, 4, max_radius) as u32, 0);
    draw::circle::filled(ctx, &c.0, oscillator::sine(ctx.frame_count + 105, 0.02, 4, max_radius) as u32, 0);



    // glitch::horizontal_glitch(
    //     ctx,
    //     &mut HorizontalGlitchProps {
    //         strength: 30,
    //         chance: 10,
    //         left_right_balance: 128,
    //     },
    //     None,
    // );

}
