use crate::demo::user_data::DemoUserData;
use graph1::draw::rectangle;
use graph1::core::alpha::AlphaMethod;
use graph1::core::context::GraphContext;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::palettes;

/// Side of Bouncy, in pixels
const SQUARE_SIDE_PX: i32 = 96;

/// Render the background of the window
fn render_background(ctx: &mut GraphContext<DemoUserData>) {
    // Width of each column in the background is 20% of the window width
    let column_width = (ctx.win.w as f64 / 100.0 * 20.0) as u32;
    // Number of columns in the background
    let num_columns = ctx.win.w / column_width;

    // Render the background columns with alternating colors
    for i in 0..num_columns {
        let color = match i % num_columns {
            0 =>  0x66_66_11ff,//palettes::Grayscale::X07_MEDIUM_GRAY,
            1 => 0x11_66_66_ff, //0x39_0046_ff,
            2 => 0x0018_18_ff,//palettes::Grayscale::X05_PALE_GRAY,
            3 => palettes::Grayscale::X08_DARK_GRAY,
            4 => palettes::Grayscale::X09_CHARCOAL,
            _ => palettes::Grayscale::X09_CHARCOAL,
        };

        rectangle::filled(
            ctx,
            &RectArea::new(i * column_width, 0, column_width, ctx.win.h, Some(color)),
        );
    }
}



/// Render a frame with Bouncy, who is just a square bouncing on the screen
/// Bouncy is the hero of this demo.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {

    // The animation lasts only for the first 120 frames
    if ctx.frame_count > 120 {
        return;
    }

    let frame_delta:u32 = (120 - ctx.frame_count) as u32/2;

    // Render the background with colorful columns
    render_background(ctx);

    let height = ctx.win.h / 4 ;

    let color = 0x00_ff_ef_88;

    // Render a rectangle with Integer alpha blending
    ctx.alpha.enabled = true;
    ctx.alpha.method = AlphaMethod::Int;
    rectangle::filled(ctx, &RectArea::new(0, height - frame_delta, ctx.win.w, height, Some(color)));

    // Render a rectangle with Float alpha blending
    ctx.alpha.method = AlphaMethod::Float;
    rectangle::filled(ctx, &RectArea::new(0, height*2 + frame_delta, ctx.win.w, height, Some(color)));

    // Depending on your monitor and eyesight, you may or may not see the difference between how the top and bottom
    // rectangles are blended with the background.
    // - The top cyan rectangle is blended using Integer alpha method, which is faster but a little less accurate.
    // - The bottom cyan rectangle is blended using Float alpha method, which is slower but a little more accurate.
    // In most cases the difference is negligible, so the Integer method is used by default.
}
