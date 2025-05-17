use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;

use graph1::fx::scanline;
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::{clear_screen, grid};

use graph1::draw::curve::bezier_segment::BezierSegment;
use graph1::draw::polygons::{polygon, star, PolygonProperties, StarProperties};
use graph1::draw::tools::fill;
use graph1::primitives::numeric::Numeric;
use graph1::primitives::plane::{Dimensions2d, RectArea};
use graph1::primitives::point::Point;
use graph1::primitives::Pixel;
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::utils::color::{alpha, gradient};

use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::geometry::region::Region;
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
        ctx.win.foreground_color = RetroNeon::GLITCH_RED;
        ctx.win.background_color = RetroNeon::PSYCHEDELIC_BLUE;
    }

    clear_screen(ctx);

    let num_cols: u32 = 20;
    let num_rows: u32 = 10;
    let cell_width: u32 = ctx.win.w / num_cols;
    let cell_height: u32 = ctx.win.h / num_rows;

    // let mut colors = gradient::linear(0x0000ffff, 0x00ffffff, grid.num_cells());



    let mut grid = UniformGrid::new(
        RectArea::new(0, 0, cell_width, cell_height, Some(RetroNeon::LASER_AQUA)),
        num_rows as usize,
        num_cols as usize,
        Some(vec![
            RetroNeon::PSYCHEDELIC_BLUE,128,
            RetroNeon::DEEP_SPACE_BLUE,
            RetroNeon::ELECTRIC_BLUE,
        ]),
    );

    grid::uniform::render(ctx, &grid, false);

    grid.iter().for_each(|cell| {

        let p:Point;


        match ctx.frame_count % 80 {
            0..=10 => { p=cell.top(); },
            11..=20 => { p=cell.top_right(); },
            21..=30 => { p=cell.right(); },
            31..=40 => { p=cell.bottom_right(); },
            41..=50 => { p=cell.bottom(); },
            51..=60 => { p=cell.bottom_left(); },
            61..=70 => { p=cell.left(); },
            71..=80 => { p=cell.top_left(); },
            // 81..=90 => { p=cell.center(); },
            _ =>   p=cell.center(), // defensive: % 8 guarantees 0–7
        }


        draw::rectangle::filled(ctx,
            &RectArea {
                top_left: Point::new(p.x , p.y ),
                dimensions: Dimensions2d::square(ctx.win.h/20),
                color: Some( alpha::set_alpha(RetroNeon::STROBE_WHITE, 190)),
            }
        )
    });

}
