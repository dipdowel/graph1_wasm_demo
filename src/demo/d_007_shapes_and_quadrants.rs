use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;
use graph1::primitives::Pixel;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::color::gradient;
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

    if ctx.frame_count == 0 {
        ctx.win.background_color = 0x33333300;
        // ctx.win.background_color = 0x000000ff   ;
    }

    clear_screen(ctx);

    let max_radius = ctx.win.quadrants.bottom_left.width() / 4;

    let r: (u32, u32, u32, u32) = (
        oscillator::sine(ctx.frame_count + 0, 0.015, 4, max_radius) as u32,
        oscillator::sine(ctx.frame_count + 40, 0.015, 4, max_radius) as u32,
        oscillator::sine(ctx.frame_count + 80, 0.015, 4, max_radius) as u32,
        oscillator::sine(ctx.frame_count + 120, 0.015, 4, max_radius) as u32,
    );
    let bg = RetroNeon::DEEP_SPACE_BLUE;

    // let fg:Vec<u32> = gradient::linear(RetroNeon::CYBER_BLUE, RetroNeon::NEON_PINK, 14);

    let fg = [
        gradient::linear(RetroNeon::ELECTRIC_BLUE, RetroNeon::CYBER_YELLOW, 8),
        gradient::linear(RetroNeon::CYBER_YELLOW, RetroNeon::ELECTRIC_BLUE, 8),
    ]
    .concat();

    let mut i: usize =
        oscillator::sine(ctx.frame_count, 0.000055 * max_radius as f64, 0, 7) as usize;
    i = i.min(0);

    let color: (u32, u32, u32, u32) = (
        gradient::linear_step(bg, fg[i], max_radius as usize, r.0 as usize),
        gradient::linear_step(bg, fg[i + 1], max_radius as usize, r.1 as usize),
        gradient::linear_step(bg, fg[i + 2], max_radius as usize, r.2 as usize),
        gradient::linear_step(bg, fg[i + 3], max_radius as usize, r.3 as usize),
    );

    let center: (Pixel, Pixel, Pixel, Pixel) = (
        ctx.win.quadrants.top_left.center().to_pixel(color.0),
        ctx.win.quadrants.top_right.center().to_pixel(color.1),
        ctx.win.quadrants.bottom_left.center().to_pixel(color.2),
        ctx.win.quadrants.bottom_right.center().to_pixel(color.3),
    );

    draw::circle::filled(ctx, &center.3, r.0, 0);
    draw::circle::filled(ctx, &center.1, r.1, 0);
    draw::circle::filled(ctx, &center.2, r.2, 0);
    draw::circle::filled(ctx, &center.0, r.3, 0);

    draw::circle::filled(
        ctx,
        &ctx.win.center.to_pixel(RetroNeon::STROBE_WHITE),
        max_radius * 100 / 45,
        4,
    );


    // draw::rectangle::filled(ctx, &RectArea {
    //     top_left: ctx.win.quadrants.top_left.right() ,
    //     dimensions: Dimensions2d::new(100, 100),
    //     color: Some(RetroNeon::GLITCH_RED),
    // });
        
        

    // invert::invert_colors(
    //     ctx,
    //     &RectArea {
    //         top_left: ctx.win.quadrants.top_left.center(),
    //         dimensions: ctx.win.quadrants.top_left.rect_area().dimensions,
    //         color: None,
    //     },
    // );

    // scanline::window(ctx, 1, 0x25);
}
