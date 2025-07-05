use crate::demo::user_data::DemoUserData;
use graph1::core::context::{ GraphContext};

use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

use graph1::primitives::{plane::RectArea, point::Point, Pixel};
use std::time::Instant;

use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::oscillator;

use crate::demo::elements::d_014_spray_config::{
    BlendSettings, BrushSettings, PathBounds, SprayDemoSceneConfig,
};
use crate::global_consts::{WIN_HEIGHT, WIN_WIDTH};
use graph1::core::context_utils::line_clipping_style::LineClippingStyle;
use graph1::draw::polygons::{polygon, PolygonProperties};
use graph1::draw::rectangle;
use graph1::draw::tools::brush::Brush;
use graph1::draw::tools::{fill, spray};
use graph1::{draw, fx};
// use graph1::buffer_op::{horizontal_lines_x3, horizontal_lines_x4, horizontal_lines_x4_threaded, horizontal_lines_x3_threaded};
use graph1::fx::scanline;
use graph1::primitives::math::{Bound, MinMax, Shell};
use graph1::primitives::plane::Dimensions2d;
use graph1::utils::color::gradient;
use graph1::utils::color::math::{rgba_operation, ColorOperation};
use crate::utils::console_log;

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct TransformationsUserData {

    // pub win: Dimensions2d,
    // pub num_pixels: u32,
    // pub scene_configs: Vec<SprayDemoSceneConfig>,
}

// pub fn get_brush_user_data() -> BrushUserData {
//
// }

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::STROBE_WHITE;
        ctx.win.foreground_color = 0x000000ff;
    }
    clear_screen(ctx);


    let x_start_range: MinMax<u32> = MinMax {
        min: 0,
        max: ctx.win.w / 2 - 100,
    };

    let x_end_range: MinMax<u32> = MinMax {
        min: ctx.win.w / 2 - 100,
        max: ctx.win.w,
    };

    let y_range: MinMax<u32> = MinMax {
        min: 0,
        max: ctx.win.h+5,
    };



    
        let x_starts = ctx.rng.get_vec_u32(400, &x_start_range);
        let x_ends = ctx.rng.get_vec_u32(400, &x_end_range);
        let ys = ctx.rng.get_vec_u32(400, &y_range);

    // let colors = &mut gradient::linear(RetroNeon::CYBER_YELLOW, RetroNeon::STROBE_WHITE, 10);

    let colors =  vec![
        0x000000ff,
         RetroNeon::STROBE_WHITE,
            RetroNeon::LASER_LIME,
            RetroNeon::CHROME_CYAN,

        0x0000ffff,
        0x0000ffff,
        0x0000ffff,
        0x0000ffff,
        // RetroNeon::CYBERPUNK_FUCHSIA,
        //     RetroNeon::DARK_PURPLE,
            RetroNeon::NEON_PINK,
        RetroNeon::CIRCUIT_GREEN,

    ];
    
    let mut count: usize = 0;
    
    let mut lines: Vec<Vec<u32>> = Vec::new();
    for color in colors.iter() {

        let mut lines_of_same_color :Vec<u32> = Vec::new();
        lines_of_same_color.push(*color);

        for i in count..count+40 {
            let x_start = x_starts[i];
            let x_end = x_ends[i];
            let y = ys[i];
            lines_of_same_color.push(x_start);
            lines_of_same_color.push(x_end);
            lines_of_same_color.push(y);
        }
        lines.push(lines_of_same_color);
        count+= 40;
    }

 
    //
    // horizontal_lines_x3_threaded(
    //     &mut ctx.frame_buf,
    //     &ctx.win.dimensions,
    //     &lines,
    //     ctx.num_threads
    //
    // );



/*
        let mut colors_pre = gradient::linear(RetroNeon::DARK_PURPLE, RetroNeon::CIRCUIT_GREEN, 200);
        let colors = &mut gradient::linear(RetroNeon::ELECTRIC_BLUE, RetroNeon::CYBERPUNK_FUCHSIA, 200);
        colors.append(&mut colors_pre);



        let mut lines: Vec<u32> = Vec::new();

        for i in 0..400 {
            lines.push(x_starts[i]);
            lines.push(x_ends[i]);
            lines.push(ys[i]);
            lines.push(colors[i]);
        }

        horizontal_lines_x4_threaded(
            &mut ctx.frame_buf,
            &ctx.win.dimensions,
            &lines,
            ctx.num_threads
        );

 */


}



/*
    let rects:Vec<RectArea> = vec![
        RectArea {
        top_left: Point::new(0, 0),
        dimensions: Dimensions2d::new(20, 40),
        color: Some(RetroNeon::DEEP_INDIGO),
    },
        RectArea {
            top_left: Point::new(15, 0),
            dimensions: Dimensions2d::new(20, 40),
            color: Some(RetroNeon::CYBER_YELLOW),
        },
        RectArea {
            top_left: Point::new(60, 50),
            dimensions: Dimensions2d::new(820, 40),
            color: Some(RetroNeon::CHROME_CYAN),
        },
        RectArea {
            top_left: Point::new(90, 0),
            dimensions: Dimensions2d::new(20, 840),
            color: Some(RetroNeon::CIRCUIT_GREEN),
        },
    ];
    // draw::rectangle::filled_multiple(ctx, &rects.iter().collect::<Vec<&RectArea>>());
*/



    // draw::rectangle::filled(ctx, &rects[0]);
    // draw::rectangle::filled(ctx, &rects[1]);
    // draw::rectangle::filled(ctx, &rects[2]);
    // draw::rectangle::filled(ctx, &rects[3]);










/*
    let mut starting_pixel = Pixel::new(400, 220, RetroNeon::LASER_LIME);
    starting_pixel.color += ctx.frame_count as u32 / 10;

    let polygon_props: PolygonProperties = PolygonProperties {
        radius: 204,
        center: starting_pixel.clone(),
        num_sides:8,
        rotation_angle: 45.0,
        skip_rendering: false,
    };

    draw::polygons::polygon(ctx, &polygon_props);
    // let start = Instant::now(); // Start timer

    fill::paint_bucket(ctx,&starting_pixel);
*/

    // fill::flood(
    //     &mut ctx.frame_buf,
    //     &ctx.win.dimensions,
    //     &starting_pixel
    // );

        // let duration = start.elapsed(); // Compute elapsed time
    // println!("draw() took: {} ms", duration.as_millis());


