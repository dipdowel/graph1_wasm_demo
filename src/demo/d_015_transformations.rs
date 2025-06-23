use crate::demo::user_data::DemoUserData;
use graph1::core::context::{FrameBuffer, GraphContext};

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
use graph1::fx::scanline;
use graph1::primitives::math::{Bound, Shell};
use graph1::primitives::plane::Dimensions2d;
use graph1::utils::color::gradient;
use graph1::utils::color::math::{rgba_operation, ColorOperation};


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
        ctx.win.background_color = RetroNeon::CYBER_BLUE;
        ctx.win.foreground_color = RetroNeon::CYBER_YELLOW;

    }
    clear_screen(ctx);

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
    let start = Instant::now(); // Start timer


    fill::paint_bucket(ctx,&starting_pixel);


    // fill::flood(
    //     &mut ctx.frame_buf,
    //     &ctx.win.dimensions,
    //     &starting_pixel
    // );

        let duration = start.elapsed(); // Compute elapsed time
    println!("draw() took: {} ms", duration.as_millis());

}
