use graph1::draw;
use graph1::draw::tools::fill;
use crate::demo::user_data::DemoUserData;
use graph1::graph1_core::context::GraphContext;
use graph1::primitives::Pixel;
use graph1::primitives::plane::RectArea;

/// Side of Bouncy, in pixels
// const SQUARE_SIDE_PX: i32 = 96;

/// Render the background of the window
// fn render_background(ctx: &mut GraphContext<DemoUserData>) {
//     // Width of each column in the background is 20% of the window width
//     let column_width = (ctx.win.w as f64 / 100.0 * 20.0) as u32;
//     // Number of columns in the background
//     let num_columns = ctx.win.w / column_width;
//
//     // Render the background columns with alternating colors
//     for i in 0..num_columns {
//         let color = match i % num_columns {
//             0 => palettes::DesertDusk::RUSTY_ORANGE,
//             1 => palettes::DesertDusk::EARTH_RED,
//             2 => palettes::DesertDusk::DESERT_ROSE,
//             3 => palettes::DesertDusk::CLAY_BROWN,
//             4 => palettes::DesertDusk::GOLDEN_SAND,
//             _ => palettes::DesertDusk::SHADOW_BROWN,
//         };
//
//         rectangle::filled(
//             ctx,
//             &RectArea::new(i * column_width, 0, column_width, ctx.win.h, Some(color)),
//         );
//     }
// }
//
// const RED_COLOR: u32 = 0xff_33_00_88;

/// Render a frame with Bouncy, who is just a square bouncing on the screen
/// Bouncy is the hero of this demo.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // Initialize the animation variables on the zero-th frame
    // if ctx.frame_count == 0 {

    let current_frame = ctx.frame_count as u32;

    if current_frame < ctx.win.w {
        fill::buffer(ctx.frame_buf, 0x00_00_00_ff);
    }

    let box_width = ctx.win.w / 4;
    let box_mid = box_width / 2 ;


    if current_frame < ctx.win.w / 2 {
        draw::circle::filled(
            ctx,
            &Pixel{
                x: box_mid*4,
                y: box_mid*2,
                color: 0x22_00_00_ff,
            },
            (ctx.frame_count as u32 % ctx.win.w)*2,
            0
        );
    }
/*
    if current_frame >  (ctx.win.w/2 - 50) /* && current_frame < ctx.win.w*2 */ {
        draw::circle::filled(
            ctx,
            &Pixel{
                x: box_mid*4,
                y: box_mid*2,
                color: 0xff44_44_ff,
            },
            (current_frame - ctx.win.w/2),
            4
        );
    }
*/
    /*
    if current_frame >= ctx.win.w /4{
        draw::circle::filled(
            ctx,
            &Pixel{
                x: 20,
                y: 20,
                color: 0xddee_ff_ff,
            },
            10,
            2
        );
    }
*/







    //
    // graph1::draw::rectangle::filled(
    //     ctx,
    //     &RectArea::new(0, 0,   ctx.frame_count as u32 % ctx.win.w, ctx.frame_count as u32 % ctx.win.h, Some(0xdd__dd_ff)),
    // );

    // }

}
