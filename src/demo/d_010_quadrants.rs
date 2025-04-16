use crate::demo::user_data::DemoUserData;
use graph1::core::context::{AntiAliasingMethod, GraphContext, RasterizationMethod};
use graph1::draw;
use graph1::draw::line;
use graph1::draw::tools::fill;
use graph1::fx::scanline;
use graph1::primitives::Pixel;
use graph1::primitives::plane::RectArea;
use graph1::primitives::point::Point;
use graph1::text::font::Spacing;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::text::printer::Align;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::palettes::{RetroNeon, SunsetGlow};
use graph1::utils::math::oscillator;
use crate::utils::console_log;

const NUM_GRADIENT_STEPS: usize = 120;

/// Illustrate the use of polygons and stars
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // initialize the data maintained between frames

    clear_screen(ctx);
  /*
    line::between_two_points(ctx,
                             &ctx.win.quadrants.top_left.center().convert(),
                             &ctx.win.quadrants.bottom_right.center().convert(),
                             None);

    line::between_two_points(ctx,
                             &ctx.win.quadrants.top_right.center().convert(),
                             &ctx.win.quadrants.bottom_left.center().convert(),
                             None);

    line::between_two_points(ctx,
                             &ctx.win.quadrants.top_left.right().convert(),
                             &ctx.win.quadrants.bottom_right.center().convert(),
                             None);
*/
    
    
    let mut all_points: Vec<Point<u32>> = Vec::new();

    let q = ctx.win.quadrants.clone();

    all_points.push( q.top_left.bottom_left().convert() );
    all_points.push( q.top_left.top_right().convert() );
    all_points.push( q.top_right.bottom_right().convert() );
    all_points.push( q.bottom_right.bottom_left().convert() );
    all_points.push( q.bottom_right.bottom_left().convert() );
    all_points.push( q.top_left.bottom_left().convert() );

    let color = Some(RetroNeon::ELECTRIC_BLUE);

    for pair in all_points.windows(2) {
        line::between_two_points(ctx, &pair[0].convert(), &pair[1].convert(), color);
    }

    line::between_two_points(ctx, &q.top_left.top_right().convert(), &q.bottom_left.bottom_right().convert(), color);
    line::between_two_points(ctx, &q.top_left.bottom_left().convert(), &q.bottom_right.top_right().convert(), color);

    let center = ctx.win.center.convert();

    line::between_two_points(ctx, &q.top_left.center().convert(), &center, color);
    line::between_two_points(ctx, &q.top_right.center().convert(), &center, color);
    line::between_two_points(ctx, &q.bottom_right.center().convert(), &center, color);
    line::between_two_points(ctx, &q.bottom_left.center().convert(), &center, color);


    let mut colors = gradient::linear(RetroNeon::DEEP_INDIGO, RetroNeon::CYBER_YELLOW, 8);

    let center = ctx.win.center;
    let fill_pixels = [
        Pixel { x: &center.x+2, y: &center.y+2, color: colors.pop().unwrap() },
        Pixel { x: &center.x+100, y: &center.y+2, color: colors.pop().unwrap() },

        Pixel { x: &center.x-2, y: &center.y+2, color:colors.pop().unwrap() },
        Pixel { x: &center.x-100, y: &center.y+2, color: colors.pop().unwrap() },
        Pixel { x: &center.x+2, y: &center.y-2, color: colors.pop().unwrap() },
        Pixel { x: &center.x+100, y: &center.y-2, color: colors.pop().unwrap() },
        Pixel { x: &center.x-2, y: &center.y-2, color: colors.pop().unwrap() },
        Pixel { x: &center.x-100, y: &center.y-2, color: colors.pop().unwrap() },

    ];


    // fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &Pixel { x: &center.x+2, y: &center.y+2, color: 0xffffffff });

    for fill_pixel in fill_pixels {
        // ctx.frame_buf[fill_pixel.y as usize * ctx.win.w_usize + fill_pixel.x as usize] = fill_pixel.color;
        fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &fill_pixel);
    }


    // line::between_two_points(ctx, &all_points[0].convert(), &all_points.last().unwrap().convert(), Some(0x00ff00ff));





    // line::between_two_points(ctx, &pair[0], &pair[1], Some(0x00ff00ff));
    // line::between_two_points(ctx, &a[0], &a.last().unwrap(), Some(0x00ff00ff));

    // let mut a: Vec<Point<i32>> = ctx
    //     .win
    //     .quadrants
    //     .top_left
    //     .get_points();
    // a.remove(0);
    // 
    // for i in (1..a.len() - 2).step_by(4) {
    //     a.swap(i, i + 2);
    // }
    // 
    // for pair in a.windows(2) {
    //     line::between_two_points(ctx, &pair[0], &pair[1], Some(0x00ff00ff));
    // }
    // line::between_two_points(ctx, &a[0], &a.last().unwrap(), Some(0x00ff00ff));



    // a.0.


    // draw::rectangle::filled(ctx, &RectArea::new(20, 20, 100, 100, None));

    scanline::window(ctx, 1, 48);
}
