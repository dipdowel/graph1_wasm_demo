use graph1::{buffer_op, draw};
use graph1::buffer_op::scale::scale_direction::ScaleDirection;
use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw::polygons;
use graph1::draw::polygons::StarProperties;
use graph1::draw::tools::fill;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};


use graph1::utils::color::palettes::{DesertDusk, ForestMist, OceanBreeze, RetroNeon};

use graph1::utils::grid::row_flex::{RowFlexGrid, FlexRow};

use graph1::primitives::{neighborhood, plane::RectArea, point::Point, Pixel};
use graph1::primitives::align::Align;
use graph1::primitives::math::Displacement;
use graph1::primitives::plane::Dimensions2d;
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::printer;
use graph1::utils::{clear_screen, grid};
use crate::global_consts::{WIN_HEIGHT, WIN_WIDTH};
/*
use graph1::utils::{clear_screen, grid};
use std::collections::HashMap;
use graph1::utils::color::gradient;

use graph1::utils::grid::uniform::{Neighbor, UniformGrid};
use graph1::utils::math::oscillator;

use crate::utils::console_log;
use graph1::core::context_utils::line_clipping_style::LineClippingStyle;
use graph1::draw;
use graph1::draw::tools::{fill, spray};
use graph1::draw::{line, rectangle};
use graph1::draw::tools::brush::Brush;
use graph1::fx::glitch::HorizontalGlitchProps;
use graph1::fx::{glitch, scanline};
use graph1::primitives::math::MinMax;
use graph1::sprites::axonometric;
use graph1::sprites::axonometric::Bar3DProps;

use graph1::utils::color::alpha::set_alpha;
use graph1::utils::math::geometry::region::Region;
use graph1::utils::math::rng::XorShiftRng;
use crate::demo::d_012_grid::GridUserData;
*/
//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct TextUserData {
    pub color_props: printer::ColorProperties<'static>,
    pub font: Option<PixelFont>,
    // pub text: Vec<String>,
    // pub background_color: Option<u32>,
}


pub fn get_text_user_data() -> TextUserData {
    TextUserData {

        color_props: printer::ColorProperties {
            // color: Some(OceanBreeze::FOAM_WHITE),
            color: Some(OceanBreeze::SEAFOAM),
            color_transformer: None,
            data: None,
        },
        // Let's put an instantiated font into the user data
        // so that we don't have to instantiate it on every frame
        font: Some(instantiate_embedded_font(
            EmbeddedFonts::MatriksUaxactun,
            3,
            Some(Spacing {
                kerning_px: 4,
                leading_px: 2,
            }),
            None,
        )),

        // text: vec![
        //     "Grid:  8x2".to_string(),
        //     "Grid: 16x5".to_string(),
        //     "Grid: 24x3".to_string(),
        // ],
    }
}

//---------------------------------------------------------------------
//---------------------------------------------------------------------



pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {

    //
    // FIXME: this is a working examples
    //
    let color_props = ctx.user_data.grid.text_color_props;
    let font = ctx.user_data.text.font.clone().unwrap();
    printer::print_line(
        ctx,
        &Point { x: 40, y: 110 },
        &font,
        &color_props,
        &" Check... check... ⁴€←↑→↓−∕✔✕� ",
    );


    // ================ CURSOR ===================
    //  WIN_WIDTH


    // Blinking cursor
    let color: u32 = if (ctx.frame_count / 40) % 2 == 0 {
        ctx.win.background_color
    } else {
        ctx.win.foreground_color
    };
    draw::rectangle::filled(ctx, &RectArea::new(400, 104, 12, 28, Some(color)));

    let row_height = WIN_HEIGHT / 3;
    let rows:Vec<FlexRow<u32>> = vec![
        FlexRow::new(
        row_height,
        vec![WIN_WIDTH/4; 4],
        vec![RetroNeon::MATRIX_GREEN,RetroNeon::CYBERPUNK_FUCHSIA],
        Align::Left,
    ),
        FlexRow::new(
            row_height,
            vec![WIN_WIDTH; 1],
            vec![RetroNeon::MATRIX_GREEN,RetroNeon::CYBERPUNK_FUCHSIA],
            Align::Left,
        ),
        FlexRow::new(
            row_height,
            vec![WIN_WIDTH/2; 2],
            vec![RetroNeon::MATRIX_GREEN,RetroNeon::CYBERPUNK_FUCHSIA],
            Align::Left,
        ),
    ];

    let layout = RowFlexGrid::new(
        Point::new(0, 0),
      Some(rows),
        Some(400),
    );


    grid::render(ctx, &layout, true);

    /*

  Some(vec![

        ]),


     */


}

/*
        pub fn within_buf(
    buf: &mut [u32],
    dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_start: &Point<u32>,
    scale_factor: u32,
    src_pixel_displacement: Option<&Displacement<u8>>,
    direction: ScaleDirection,
)
         */
/*

    if ctx.frame_count == 0 {
        clear_screen(ctx);

        let star_props = StarProperties {
            center: Pixel {
                color: RetroNeon::CYBER_YELLOW,
                x: 32,
                y: 32,
            },

            num_rays: 5,
            inner_radius: 12,
            outer_radius: 26,
            rotation_angle: 0.0,
            skip_rendering: false,
        };
        polygons::star(ctx, &star_props);

        fill::paint_bucket(ctx, &star_props.center);
    }
        buffer_op::scale::rect::within_buf(
            &mut ctx.frame_buf,
            &ctx.win.dimensions,
            &RectArea {
                top_left: Point { x: 0, y: 0 },
                dimensions: Dimensions2d { w: 64, h: 64 },
                color: None,
            },
            &Point { x: 96, y: 16 },
            3,
            Some(&Displacement{dx:18, dy:15}),
            ScaleDirection::Down,
        );
    }
*/





