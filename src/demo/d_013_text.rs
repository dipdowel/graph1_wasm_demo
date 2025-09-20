use crate::demo::user_data::DemoUserData;
use crate::utils::console_log;
use graph1::buffer_op::scale;
use graph1::core::context::GraphContext;
use graph1::{buffer_op, draw};
use graph1::fx::scanline;
use graph1::primitives::math::Displacement;
use graph1::primitives::{plane::RectArea, point::Point};
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::text::printer::Align;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::palettes::RetroNeon;

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct TextUserData {
    pub cursor_area: Option<RectArea>,
    pub cursor_data: Option<Vec<u32>>
}

const DARK_BROWN: u32 = 0x342B17ff;
const BLACK: u32 = 0x000000ff;
pub fn get_text_user_data() -> TextUserData {
    // TODO: Check whether this initialization is even needed at all...
    // TODO: Check whether this initialization is even needed at all...
    // TODO: Check whether this initialization is even needed at all...
    // TODO: Check whether this initialization is even needed at all...
    // TODO: Check whether this initialization is even needed at all...

    let text_color = gradient::linear_step(RetroNeon::FUTURE_BRONZE, DARK_BROWN, 200, 120);

    // let text_color = RetroNeon::FUTURE_BRONZE;

    TextUserData {
        cursor_area: None,
        cursor_data: None,
    }
}

//---------------------------------------------------------------------

fn init_text_demo(ctx: &mut GraphContext<DemoUserData>) {
    ctx.win.background_color = RetroNeon::ELECTRIC_BLUE;
    ctx.win.foreground_color = RetroNeon::CYBER_YELLOW;
    clear_screen(ctx);

    // Activate frame buffer #1.
    ctx.set_active_frame_buf(1).ok();
    clear_screen(ctx);

    // Prepare some assets for the text demo
    // and save them into the buffer #1 for later use

    // Title that will be scrolled across the screen
    let title_font = instantiate_embedded_font(
        EmbeddedFonts::MatriksUaxactun,
        // EmbeddedFonts::MatriksUaxactunMono,
        4,
        Some(Spacing {
            kerning_px: 4,
            leading_px: 2,
        }),
        None,
    );

    let shadow_color = gradient::linear_step(RetroNeon::ELECTRIC_BLUE, BLACK, 100, 60);

    let mut title_font_props: printer::ColorProperties = printer::ColorProperties {
        color: Some(shadow_color),
        color_transformer: None,
        data: None,
    };

    printer::print_line(
        ctx,
        &Point { x: 1, y: 30 },
        &title_font,
        &title_font_props,
        &"   Pixel fonts!  ←     ",
    );

    title_font_props.color = Some(RetroNeon::CYBER_YELLOW);

    printer::print_line(
        ctx,
        &Point { x: 0, y: 29 },
        &title_font,
        &title_font_props,
        &"   Pixel fonts!  ←     ",
    );

    // color:

    // Switch back to the main frame buffer (index 0)
    ctx.set_active_frame_buf(0).ok();
}

fn scroll_the_title(ctx: &mut GraphContext<DemoUserData>) {
    let frame_count = ctx.frame_count as u32;
    let win_dims = ctx.win.dimensions.clone();
    let buf_result = ctx
        .get_multi_frame_bufs(&[0, 1])
        .expect("Failed to get multiple frame buffers");
    let dst_buf = buf_result.active;
    let src_buf = buf_result.immut[0].frame_buf;

    scale::up::sparse::to_another_buf(
        &src_buf,
        &win_dims,
        &RectArea::new(frame_count, 30, 278, 200, None),
        dst_buf,
        &win_dims,
        &Point { x: 4, y: 38 },
        3,
        &Displacement { dx: 1, dy: 1 },
        1,
    );
}

//---------------------------------------------------------------------

/// Store the area under the cursor for later restoration
/// NB: This function assumes the area is fully inside the frame buffer!
fn store_area_under_cursor(ctx: &mut GraphContext<DemoUserData>, cursor:&RectArea) {
        ctx.user_data.text.cursor_area = Some(cursor.clone());
        buffer_op::copy::rect::to_another_buf(
            &ctx.frame_buf,
            &ctx.win.dimensions,
            cursor,
            // &mut ctx.user_data.text.cursor_data.get_or_insert(vec![0; (cursor.dimensions.w * cursor.dimensions.h) as usize]),
            &mut ctx.user_data.text.cursor_data.insert(vec![0; (cursor.dimensions.w * cursor.dimensions.h) as usize]),
            &ctx.user_data.text.cursor_area.unwrap().dimensions ,
            &Point { x: 0, y: 0 },
            false,
            1,
        )
}

/// Restore the area under the cursor from previously stored data
/// NB: This function assumes the area is fully inside the frame buffer!
fn restore_area_under_cursor(ctx: &mut GraphContext<DemoUserData>) {

    if ctx.user_data.text.cursor_area.is_none(){
        return;
    }

    let cursor_dims = &ctx.user_data.text.cursor_area.unwrap().dimensions.clone();

    let cursor = match &ctx.user_data.text.cursor_area {
        Some(c) => c,
        None => return,
    };
    let cursor_data = match &ctx.user_data.text.cursor_data {
        Some(d) => d,
        None => return,
    };

    buffer_op::copy::rect::to_another_buf(
        &cursor_data,
        &cursor_dims,
        &ctx.user_data.text.cursor_area.unwrap(),
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        &cursor.top_left,
        false,
        1,
    )
}




pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {

    let frame_count = ctx.frame_count as u32;




        ////////////////////////////////////////////////////////////////////////////////////////////////
        ctx.win.background_color = BLACK;
        ctx.win.foreground_color = RetroNeon::MATRIX_GREEN;
        clear_screen(ctx);


        // Prepare some assets for the text demo
        // and save them into the buffer #1 for later use


        let text_font = instantiate_embedded_font(
            EmbeddedFonts::MatriksUaxactunMono,
            2,
            Some(Spacing {
                kerning_px: 2,
                leading_px: 3,
            }),
            None,
        );

        let title_font_props: printer::ColorProperties = printer::ColorProperties {
            color: Some(RetroNeon::ACID_GREEN),
            color_transformer: None,
            data: None,
        };

    let text = [
        "Marcel van Deijl designed two fonts",
        "which are shipped with Graph1:",
        "",
        "→ Matriks Uaxactun",
        "→ Matriks Uaxactun Mono",
        "",
        "Each font contains 222 characters,",
        "that covers most European languages",
        // "",
        // "→ Red Alert Inet",
        // "→ Red Alert Lan",
        // "     by N3trunn3r",
    ];

        let area = printer::print(
            ctx,
            &Point { x: 26, y: 20 },
            &text_font,
            &title_font_props,
            &text,
            Align::Left
        );



    // Blinking cursor
    if (ctx.frame_count / 20) % 2 == 0 {
        let cursor = RectArea::new(26+12*7 , 20, 12, 28, None);
        store_area_under_cursor(ctx, &cursor);
        draw::rectangle::filled(ctx, &cursor);
    } else {

        restore_area_under_cursor(ctx);
    };



    // console_log(&format!("Area: {:?}", area));



    ////////////////////////////////////////////////////////////////////////////////////////////////






    /*
        clear_screen(ctx);

    // Init the demo on frame 0
    if ctx.frame_count == 0 {
        init_text_demo(ctx);
    }

    // Scroll the title across the screen
    if frame_count<412 {
        scroll_the_title(ctx);
    }


    if frame_count > 412 && frame_count < 496 {
        ctx.win.background_color = gradient::linear_step(RetroNeon::ELECTRIC_BLUE, BLACK, 80, ctx.frame_count - 412);
    }
    scanline::window(ctx, 2, 16);
*/






    // frame_count = 518 { }

    // console_log(&format!("frame: {:?}", frame_count))
}




// let font = ctx.user_data.text.font.clone().unwrap();
// first "Pixel fonts!" runs till frame 418

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



/*
        src_buf: &[u32],
        src_dims: &Dimensions2d<u32>,
        src_area: &RectArea<u32>,
        dst_buf: &mut [u32],
        dst_dims: &Dimensions2d<u32>,
        dst_start: &Point<u32>,
        use_absolute_alpha: bool,
        num_threads: usize,
 */