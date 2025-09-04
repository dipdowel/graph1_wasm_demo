use graph1::{buffer_op, draw};
use graph1::buffer_op::scale::scale_direction::ScaleDirection;
use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::core::default_colors;
use graph1::draw::polygons;
use graph1::draw::polygons::StarProperties;
use graph1::draw::tools::fill;
use graph1::fx::scanline;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};


use graph1::utils::color::palettes::{DesertDusk, ForestMist, OceanBreeze, RetroNeon};

use graph1::utils::grid::flex_row::{FlexRowGrid, FlexRow};

use graph1::primitives::{neighborhood, plane::RectArea, point::Point, Pixel};
use graph1::primitives::align::Align;
use graph1::primitives::helper_types::PixelColorTransformerFn;
use graph1::primitives::math::Displacement;
use graph1::primitives::plane::Dimensions2d;
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::printer;
use graph1::utils::{clear_screen, grid};
use graph1::utils::color::gradient;
use crate::global_consts::{WIN_HEIGHT, WIN_WIDTH};
use crate::utils::console_log;
/*
use graph1::utils::{clear_screen, grid};
use std::collections::HashMap;
use graph1::utils::color::gradient;

use graph1::utils::grid::uniform::{Neighbor, UniformGrid};
use graph1::utils::math::oscillator;


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

const DARK_BROWN:u32 = 0x342B17ff;
pub fn get_text_user_data() -> TextUserData {

    let text_color = gradient::linear_step(RetroNeon::FUTURE_BRONZE, DARK_BROWN , 200, 120);
    // let text_color = RetroNeon::FUTURE_BRONZE;

    TextUserData {

        color_props: printer::ColorProperties {
            // color: Some( RetroNeon::DIGITAL_GOLD      ),
            color: Some(  text_color  ),
            color_transformer: None,
            data: None,
        },
        // Let's put an instantiated font into the user data
        // so that we don't have to instantiate it on every frame
        font: Some(instantiate_embedded_font(
            EmbeddedFonts::MatriksUaxactun,
            2,
            Some(Spacing {
                kerning_px: 2,
                leading_px: 4,
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


const TEXT_LINE_1: &str = "Welcome to Graph1 text!";

const TITLE_1: &str = "Graph1 text";

const TEXT_LINES:[&str; 4] = [
                              "TODO:",
                              "- Write some intro on texts",
                              "- Introduce the available fonts",
                              "- Credit Marcel for 'Matriks Uaxactun'",
];




pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {


    let font = ctx.user_data.text.font.clone().unwrap();

    if ctx.frame_count == 0 {

        // Activate another frame buffer (index 1)
        // We'll prepare some assets in thet buffer,
        // and we'll copy them to the main frame buffer during the animation.
        let res = ctx.set_active_frame_buf(1);

        ctx.win.background_color = default_colors::TRANSPARENT_BLACK;
        ctx.win.foreground_color = RetroNeon::STROBE_WHITE;
        clear_screen(ctx);

        let title_font = instantiate_embedded_font(
            EmbeddedFonts::MatriksUaxactun,
            7,
            Some(Spacing {
                kerning_px: 5,
                leading_px: 2,
            }),
            None,
        );

        let color_transformer: PixelColorTransformerFn = | _, x:u32, y: u32,  w:u32, _, _ | -> u32 {
            if x == 0 || x == w-1{
                return RetroNeon::FUTURE_BRONZE;
            }
            if y % 2 == 0  {
                return RetroNeon::DIGITAL_GOLD;
            }
             RetroNeon::LASER_LIME
        };
        let title_font_props: printer::ColorProperties = printer::ColorProperties {
            // color: Some(DARK_BROWN),
            color: None,
            // color_transformer: None,
            color_transformer: Some(color_transformer),
            data: None
        };

        printer::print_line(
            ctx,
            &Point { x: 14+24, y: 10 },
            &title_font,
            &title_font_props,
            &"Graph1",
        );

        printer::print_line(
            ctx,
            &Point { x: 14+278, y: 10 },
            &title_font,
            &title_font_props,
            &"Text",
        );


        /*
        let color_props = ctx.user_data.text.color_props;
        printer::print(
            ctx,
            &Point { x: 24, y: 110 },
            &font,
            &color_props,
            &TEXT_LINES,
            printer::Align::Left,
        );
        */





        // scanline::window(ctx, 1, 106);

        // let active_buf_idx = ctx.get_active_frame_buf_index();
        // console_log(&format!("active_buf_idx: {:?}", active_buf_idx));









        ctx.win.background_color = RetroNeon::DIGITAL_GOLD;
        ctx.win.foreground_color = RetroNeon::STROBE_WHITE;

        // FIXME: Don't forget to switch back to the main frame buffer (index 0)
        // switch back to the main frame buffer (index 0)
        // let res = ctx.set_active_frame_buf(0);

    }

}




    // font.
    //
    // TEXT_LINE_1.chars().for_each(|c| {
    //     let glyph = font.get_glyph(&c);
    //     if let Some(g) = glyph {
    //         let src_area = RectArea::new(0, 0, g.width, g.height, None);
    //         buffer_op::copy::rect::within_buf(
    //             &mut ctx.frame_buf,
    //             &ctx.win.dimensions,
    //             &src_area,
    //             &Point { x: 10 + (char_num as u32) * 12, y: 50 },
    //             2,
    //             Some(&Displacement { dx: 1, dy: 1 }),
    //             ScaleDirection::Up,
    //         );
    //     }
    // })

    // let char_num = (ctx.frame_count / 6) % 95;
    // let a = font.get_glyph(&'W');



    // clear_screen(ctx);
    //
    // FIXME: this is a working examples
    //

    // console_log(&format!("Color props: {:?}", color_props));
    // console_log(&format!("RetroNeon::MATRIX_GREEN: {:?}", RetroNeon::MATRIX_GREEN));


    /*
    printer::print(
        ctx,
        &Point { x: 10, y: 10 },
        &font,
        &color_props,
        &[" Check... check... ⁴€←↑→↓−∕✔✕� ",
        " The quick brown fox jumps over the lazy dog. 0123456789 ",
        " Sphinx of black quartz, judge my vow. ",
        " Pack my box with five dozen liquor jugs. ",
        " How vexingly quick daft zebras jump! ",
        " Bright vixens jump; dozy fowl quack. ",
        " Jackdaws love my big sphinx of quartz. "],
        graph1::text::printer::Align::Left
    );
     */

    /*
    let loader_symbol = match ctx.frame_count % 80 {
        0..10 => "R",
        11..20 => "Ro",
        21..30 => "Rob",
        31..40 => "robo",
        41..50 => "robot",
        51..60 => "ROBOTR",
        // _ => ".....|",
        61..70 => "Robotro",
        71..80 => "robotron",
        _ => "",
    };
    printer::print_line(
        ctx,
        &Point { x: 10, y: 10 },
        &font,
        &color_props,
        &loader_symbol,
    );
     */





    /*
    // ================ CURSOR ===================
    let color: u32 = if (ctx.frame_count / 40) % 2 == 0 {
        ctx.win.background_color
    } else {
        ctx.win.foreground_color
    };
    draw::rectangle::filled(ctx, &RectArea::new(400, 104, 12, 28, Some(color)));
    */

    /*
    let height_block = WIN_HEIGHT / 8;
    let rows:Vec<FlexRow<u32>> = vec![

        FlexRow::new(
            height_block * 2,
            vec![WIN_WIDTH; 1],
            vec![RetroNeon::MATRIX_GREEN,RetroNeon::CYBERPUNK_FUCHSIA],
            Align::Center,
        ),

        FlexRow::new(
            height_block * 2,

            vec![WIN_WIDTH/3, WIN_WIDTH/3*2],
            vec![RetroNeon::MATRIX_GREEN,RetroNeon::CYBERPUNK_FUCHSIA],
            Align::Center,
        ),

        FlexRow::new(
            height_block*4,
            // vec![WIN_WIDTH/2; 2],
            vec![WIN_WIDTH],
            vec![RetroNeon::MATRIX_GREEN,RetroNeon::CYBERPUNK_FUCHSIA],
            Align::Center,
        ),

    ];

    let mut layout_grid = FlexRowGrid::new(
        Point::new(0, 0),
      Some(rows),
        Some(WIN_WIDTH),
        // None
    );


    layout_grid.add_row(FlexRow::new(
        height_block,
        vec![WIN_WIDTH/3; 1],
        vec![RetroNeon::DIGITAL_GOLD],
        Align::Left
        // Align::Center
        // Align::Right
    ));



    grid::render(ctx, &layout_grid, true);
*/



    /*

  Some(vec![

        ]),


     */



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





