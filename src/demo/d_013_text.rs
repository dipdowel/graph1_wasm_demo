use crate::demo::user_data::DemoUserData;
use crate::utils::console_log;
use graph1::buffer_op::scale;
use graph1::core::context::GraphContext;
use graph1::core::context_utils::context_snapshot::ContextSnapshot;
use graph1::fx::scanline;
use graph1::primitives::data_structs::variant::Variant;
use graph1::primitives::math::{Displacement, MinMax};
use graph1::primitives::plane::Dimensions2d;
use graph1::primitives::{plane::RectArea, point::Point};
use graph1::text::char_grid::MonospacedCharGrid;
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer::Align;
use graph1::text::{char_grid, printer};
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::palettes::RetroNeon;
use graph1::{buffer_op, draw};

const ON: bool = true;
const OFF: bool = false;

// Configure the user data for the demo
pub struct TextUserData {
    // cur_char_cell: Option<RectArea>,
    cursor_state: bool,

    /// Area under the cursor (for restoring after blinking)
    cursor_area: Option<RectArea>,
    /// Pixel data under the cursor (for restoring after blinking)
    cursor_data: Vec<u32>,
    /// Character grid for the text page with the "Marcel van Deijl" text
    page_marcel_grid: Option<MonospacedCharGrid>,
    /// Coordinates of all character cells on the "Marcel van Deijl" text page
    page_marcel_char_cells: Vec<(usize, usize)>,
    /// Index of the current character cell to interact with
    char_cell_idx: usize,
}

const BLACK: u32 = 0x000000ff;

pub fn get_text_user_data() -> TextUserData {
    TextUserData {
        cursor_state: OFF,
        // cur_char_cell: None,
        cursor_area: None,
        cursor_data: vec![],
        page_marcel_grid: None,
        page_marcel_char_cells: vec![],
        char_cell_idx: 0,
    }
}

//
//
// ===[ NAMED CONSTANTS FOR FRAME BUFFERS ]========================================================
//
const BUF_0_MAIN: usize = 0;
const BUF_1_SCROLLER: usize = 1;
const BUF_2_PAGE_MARCEL: usize = 2;
const BUF_3_PAGE_N3TRUNN3R: usize = 3;

//
//
// ===[ PREPARE SCROLLING TITLE ]===================================================================
//
const SCROLLER_TEXT: &str = "   Pixel fonts!  ←     ";

fn prepare_scrolling_title(ctx: &mut GraphContext<DemoUserData>) {
    let original_window = ctx.win.get_context();

    ctx.set_active_frame_buf(BUF_1_SCROLLER).ok();

    ctx.win.background_color = RetroNeon::ELECTRIC_BLUE;
    ctx.win.foreground_color = RetroNeon::CYBER_YELLOW;
    clear_screen(ctx);

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
        &SCROLLER_TEXT,
    );

    title_font_props.color = Some(RetroNeon::CYBER_YELLOW);

    printer::print_line(
        ctx,
        &Point { x: 0, y: 29 },
        &title_font,
        &title_font_props,
        &SCROLLER_TEXT,
    );

    // restore the original window context
    ctx.win.set_context(original_window);

    // Switch back to the main frame buffer (index 0)
    ctx.set_active_frame_buf(BUF_0_MAIN).ok();
}

//
//
// ===[ PREPARE TEXT PAGES ]========================================================================
//
const TEXT_FONT_SCALE: u8 = 2;
const TEXT_FONT_SPACING: Spacing = Spacing {
    kerning_px: 2,
    leading_px: 3,
};
// —
// –

const MARCEL_PAGE_TEXT: [&str; 8] = [
    "Marcel van Deijl designed two fonts",
    "which are shipped with Graph1:",
    "••••••••••••••••••••••••••••••",
    "→ Matriks Uaxactun",
    "→ Matriks Uaxactun Mono",
    "••••••••••••••••••••••••••••••",
    "Each font contains 222 characters,",
    "that covers most European languages",
    // "",
    // "→ Red Alert Inet",
    // "→ Red Alert Lan",
    // "     by N3trunn3r",
];

fn prepare_text_pages(ctx: &mut GraphContext<DemoUserData>) {
    let original_window = ctx.win.get_context();

    ctx.set_active_frame_buf(BUF_2_PAGE_MARCEL).ok();

    ctx.win.background_color = BLACK;
    ctx.win.foreground_color = RetroNeon::MATRIX_GREEN;
    clear_screen(ctx);

    let text_font = instantiate_embedded_font(
        EmbeddedFonts::MatriksUaxactunMono,
        TEXT_FONT_SCALE,
        Some(TEXT_FONT_SPACING),
        None,
    );

    let title_font_props: printer::ColorProperties = printer::ColorProperties {
        color: Some(RetroNeon::ACID_GREEN),
        color_transformer: None,
        data: None,
    };

    let text_top_left: Point = Point { x: 26, y: 20 };

    let text_dims = printer::print(
        ctx,
        &text_top_left,
        &text_font,
        &title_font_props,
        &MARCEL_PAGE_TEXT,
        Align::Left,
    );

    let dimensions_input: Variant<&[&str], Dimensions2d> = Variant::Primary(&MARCEL_PAGE_TEXT);

    let page_marcel_grid = MonospacedCharGrid::new(
        &text_font,
        dimensions_input,
        text_top_left,
        Some(RetroNeon::NEON_PINK),
    )
    .expect("Failed to create MonospacedCharGrid for Marcel page");

    ctx.user_data
        .text
        .page_marcel_grid
        .get_or_insert(page_marcel_grid);

    for (line_idx, char_idx) in MARCEL_PAGE_TEXT.iter().enumerate() {
        for (char_index, ch) in char_idx.chars().enumerate() {
            ctx.user_data
                .text
                .page_marcel_char_cells
                .push((line_idx, char_index));
        }
    }

    // restore the original window context
    ctx.win.set_context(original_window);
    // Switch back to the main frame buffer (index 0)
    ctx.set_active_frame_buf(BUF_0_MAIN).ok();
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

// fn get_char_grid()

/// Store the area under the cursor for later restoration
/// NB: This function assumes the area is fully inside the frame buffer!
fn store_area_under_cursor(ctx: &mut GraphContext<DemoUserData>, cursor: &RectArea) {
    ctx.user_data.text.cursor_area = Some(cursor.clone());


    if ctx.user_data.text.cursor_data.len() == 0 {
        let cursor_data_size = (cursor.dimensions.w * cursor.dimensions.h) as usize;
        ctx.user_data.text.cursor_data = vec![BLACK; cursor_data_size];
    }

    ctx.user_data.text.cursor_area = Some(cursor.clone());

    buffer_op::copy::rect::to_another_buf(
        &ctx.frame_buf,
        &ctx.win.dimensions,
        cursor,
        &mut ctx.user_data.text.cursor_data,
        &cursor.dimensions,
        &Point { x: 0, y: 0 },
        false,
        1,
    );


}

/// Restore the area under the cursor from previously stored data
/// NB: This function assumes the area is fully inside the frame buffer!
fn restore_area_under_cursor(ctx: &mut GraphContext<DemoUserData>) {
    if ctx.user_data.text.cursor_area.is_none() {
        return;
    }
    let cursor = &ctx.user_data.text.cursor_area.unwrap().clone();
    let mut src_area = cursor.clone();
    src_area.top_left.x = 0;
    src_area.top_left.y = 0;

    buffer_op::copy::rect::to_another_buf(
        &ctx.user_data.text.cursor_data,
        &cursor.dimensions,
        &src_area,
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        &cursor.top_left,
        false,
        1,
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////
////// [ RENDER FRAME ] ////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////

const SCROLL_START: u32 = 0;
const SCROLL_END: u32 = 412;
const SCROLL_FADE_START: u32 = 408;
const SCROLL_FADE_END: u32 = 480;

const MARCEL_START: u32 = 480;

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let frame_count = ctx.frame_count as u32;

    // Init the demo on frame 0
    if ctx.frame_count == 0 {
        prepare_scrolling_title(ctx);
        prepare_text_pages(ctx);
    }

    // FIXME: Temporary fix for jumping frame count. REMOVE!!!!
    // FIXME: Temporary fix for jumping frame count. REMOVE!!!!
    // FIXME: Temporary fix for jumping frame count. REMOVE!!!!
    // FIXME: Temporary fix for jumping frame count. REMOVE!!!!
    // FIXME: Temporary fix for jumping frame count. REMOVE!!!!
    if ctx.frame_count < MARCEL_START as usize {
        ctx.frame_count = MARCEL_START as usize;
    }

    // Scroll the title across the screen
    if frame_count > SCROLL_START && frame_count < SCROLL_END {
        clear_screen(ctx);
        scroll_the_title(ctx);
    }

    // Fade the screen out into [black?]
    if frame_count > SCROLL_FADE_START && frame_count < SCROLL_FADE_END {
        ctx.win.background_color = gradient::linear_step(
            RetroNeon::ELECTRIC_BLUE,
            BLACK,
            (SCROLL_FADE_END - SCROLL_FADE_START) as usize,
            ctx.frame_count.saturating_sub(SCROLL_FADE_START as usize),
        );
        clear_screen(ctx);
    }

    if frame_count > MARCEL_START {
        ctx.set_active_frame_buf(BUF_2_PAGE_MARCEL).ok();
        let rand_mod = ctx.rng.get_u32(&MinMax { min: 3, max: 7 });
        if frame_count % rand_mod == 0 {
            let idx = ctx.user_data.text.char_cell_idx;

            // FIXME: uncomment
            // ctx.user_data.text.char_cell_idx += 1;

            let char_cell_coords = ctx.user_data.text.page_marcel_char_cells.get(idx);

            if char_cell_coords.is_some() {
                let char_cell_coords = char_cell_coords.unwrap();

                let char_cell = ctx
                    .user_data
                    .text
                    .page_marcel_grid
                    .as_ref()
                    .unwrap()
                    .get_cell(char_cell_coords.0, char_cell_coords.1)
                    .unwrap()
                    .rect_area()
                    .clone();
                // draw::rectangle::outline(ctx, &char_cell.unwrap().rect_area());

                // ctx.user_data.text.cur_char_cell = Some(char_cell);
                ctx.user_data.text.cursor_area = Some(char_cell);
            }
        }

        let next_cursor_state = if ctx.frame_count % 50 == 0 {
            !ctx.user_data.text.cursor_state
        } else {
            ctx.user_data.text.cursor_state
        };
        // The cursor logic must begin here!

        if next_cursor_state != ctx.user_data.text.cursor_state {

            if ctx.user_data.text.cursor_state == OFF {

                let idx = ctx.user_data.text.char_cell_idx;
                let char_cell_coords = ctx.user_data.text.page_marcel_char_cells.get(idx);

                if char_cell_coords.is_some() {
                    let char_cell_coords = char_cell_coords.unwrap();
                    let mut cursor = ctx
                        .user_data
                        .text
                        .page_marcel_grid.as_ref()
                        .unwrap()
                        .get_cell(char_cell_coords.0, char_cell_coords.1)
                        .unwrap()
                        .rect_area()
                        .clone();

                    cursor.color = Some(RetroNeon::CYBER_BLUE);
                    store_area_under_cursor(ctx, &cursor);
                    draw::rectangle::filled(ctx, &cursor);
                }
            }

            if ctx.user_data.text.cursor_state == ON {
                restore_area_under_cursor(ctx);
            }


        }

        // All cursor logic must end before this line!
        ctx.user_data.text.cursor_state = next_cursor_state;

        println!(
            "ctx.user_data.text.cursor_state: {}",
            ctx.user_data.text.cursor_state
        );

        // // if ctx.user_data.text.cur_char_cell.is_some(){
        //
        // // Blinking cursor
        // if (ctx.frame_count / 20) % 2 == 0 {
        //     let cursor = RectArea::new(26 , 20, 12, 28, None);
        //     store_area_under_cursor(ctx, &cursor);
        //     draw::rectangle::filled(ctx, &cursor);
        // } else {
        //     restore_area_under_cursor(ctx);
        // };

        /*
        let next_cursor_state = if ctx.frame_count % 20 == 0 {
            !ctx.user_data.text.cursor_state
        } else {
            ctx.user_data.text.cursor_state
        };


        if next_cursor_state != ctx.user_data.text.cursor_state {

            if next_cursor_state && ctx.user_data.text.cursor_area.is_some() {
                let cursor = ctx.user_data.text.cursor_area.unwrap().clone();
                store_area_under_cursor(ctx, &cursor);
                draw::rectangle::filled(ctx, &cursor);
            }

            if !next_cursor_state && ctx.user_data.text.cursor_data.is_some() {
                restore_area_under_cursor(ctx);
            }

        }
        ctx.user_data.text.cursor_state = next_cursor_state;
        */

        // ctx.user_data.text.cursor_on = !ctx.user_data.text.cursor_on;
        /*

                let cursor_exists = ctx.user_data.text.cursor_area.is_some();

                    // Blinking cursor
                    if cursor_exists {

                        if ctx.user_data.text.cursor_state {

                            let cursor_exists = ctx.user_data.text.cursor_area.is_some();

                            let cursor = ctx.user_data.text.cursor_area.unwrap().clone();
                            // let cursor = RectArea::new(26+12*7 , 20, 12, 28, None);

                            draw::rectangle::filled(ctx, &cursor);
                            println!("flip");
                        }
                        else {
                            restore_area_under_cursor(ctx);
                            println!("flop");
                        };            }
        */

        /*        */

        // }

        // if char_cell_idx < ctx.user_data.text.page_marcel_char_cells.len() {
        // }
    }

    //
    // if frame_count  < 500 {
    //     scanline::window(ctx, 2, 16);
    // }

    /*

        // Blinking cursor
        if (ctx.frame_count / 20) % 2 == 0 {
            let cursor = RectArea::new(26+12*7 , 20, 12, 28, None);
            store_area_under_cursor(ctx, &cursor);
            draw::rectangle::filled(ctx, &cursor);
        } else {
            restore_area_under_cursor(ctx);
        };
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

/*

/// Calculate the area of a single character cell (glyph + spacing)
/// # Arguments
/// * `ctx` - The graphics context containing user data and font information.
/// * `char_x` - The horizontal character index in the char grid (not the pixel position!).
/// * `char_y` - The vertical character index in the char grid (not the pixel position!).
/// # Returns
/// A `RectArea` representing the area of the character cell.
fn get_char_cell(ctx: &mut GraphContext<DemoUserData>, char_x: u32, char_y: u32) -> RectArea {
    let kerning = ctx.user_data.text.text_font_spacing.kerning_px as u32;
    let leading = ctx.user_data.text.text_font_spacing.leading_px as u32;
    let scale_factor = ctx.user_data.text.text_font_scale as u32;

    let glyph = ctx.user_data.text.glyph_dims.unwrap();

    let top_left = ctx
        .user_data
        .text
        .text_area_marcel
        .unwrap()
        .top_left
        .clone();
    let top_left = Point {
        x: top_left.x + char_x * (glyph.w + scale_factor * kerning),
        y: top_left.y + char_y * (glyph.h + scale_factor * leading),
    };

    let dimensions = Dimensions2d {
        w: glyph.w + scale_factor * kerning,
        h: glyph.h + scale_factor * leading,
    };

    let char_cell = RectArea {
        top_left,
        dimensions,
        color: Some(RetroNeon::NEON_PINK),
    };

    char_cell
}
 */
