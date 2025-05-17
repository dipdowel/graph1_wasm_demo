use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;

use graph1::fx::scanline;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

use graph1::draw::curve::bezier_segment::BezierSegment;
use graph1::draw::polygons::{polygon, star, PolygonProperties, StarProperties};
use graph1::draw::tools::fill;
use graph1::primitives::point::Point;
use graph1::primitives::Pixel;
use graph1::primitives::plane::{Dimensions2d, RectArea};
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::utils::color::gradient;
use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::oscillator;

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct BezierCurvesUserData {
    pub text_color_props: printer::ColorProperties<'static>,
    pub text_font: Option<PixelFont>,
}

pub const BEZIER_CURVES_USER_DATA: BezierCurvesUserData = BezierCurvesUserData {
    text_color_props: printer::ColorProperties {
        color: Some(RetroNeon::STROBE_WHITE),
        color_transformer: None,
        data: None,
    },
    // Let's put an instantiated font into the user data
    // so that we don't have to instantiate it on every frame
    text_font: None,
};
//---------------------------------------------------------------------

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // initial context setup
    if ctx.frame_count == 0 {
 
    }

    clear_screen(ctx);


    let mut grid = UniformGrid::new(RectArea::new(
        0,
        0,
        20,
        20,
        Some(ctx.win.background_color),
    ),
        
        8,
        8,
     );


    let mut colors = gradient::linear(
        0x0000ffff,
        0x00ffffff,
        grid.num_cells(), 
    );
    
    grid.iter().for_each(|cell| {
        let color = colors.pop();
        ctx.win.foreground_color = color.unwrap();
        draw::rectangle::filled(
            ctx, 
            &RectArea{
                top_left: Point::new(cell.center().x, cell.center().y),
                // dimensions: Dimensions2d::new(10,10),
                dimensions: Dimensions2d::new(cell.rect_area().dimensions.w,cell.rect_area().dimensions.h),
                color: Some(ctx.win.foreground_color),
            }
        );
    });


    grid.iter().for_each(|cell| {

        draw::rectangle::filled(
            ctx,
            &RectArea{
                top_left: Point::new(cell.bottom_right().x, cell.bottom_right().y),
                dimensions: Dimensions2d::new(8,8),
                
                color: Some(0x00bbeeff),
            }
        );
    })
    
    
    
    
    // 
    // for i in 0..grid.num_cells() {
    //     let color = colors[i];
    //     draw::rectangle::filled(
    //         ctx, 
    //         &RectArea::new(
    //             grid.cell(i).x,
    //             grid.cell(i).y,
    //             grid.cell(i).w,
    //             grid.cell(i).h,
    //             Some(color),
    //         ),
    //  
    // 
    ///////////////////////////////////////////////////////////////////////////////////////////////////
}
