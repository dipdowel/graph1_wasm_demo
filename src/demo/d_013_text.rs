use crate::demo::user_data::DemoUserData;
use graph1::buffer_op::{copy, scale};
use graph1::core::context::{GraphContext, WindowContext};
use graph1::core::context_utils::context_snapshot::ContextSnapshot;
use graph1::fx;
use graph1::fx::{glitch, scanline};
use graph1::primitives::data_structs::variant::Variant;
use graph1::primitives::math::{Displacement, MinMax};
use graph1::primitives::plane::Dimensions2d;
use graph1::primitives::{plane::RectArea, point::Point};

use graph1::fx::glitch::HorizontalGlitchProps;
use graph1::text::char_grid::make_monospaced_char_grid;
use graph1::text::font::Spacing;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::text::printer::Align;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::math::ColorOperation;
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::grid::grid_position::GridPosition;
use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::geometry::region::Region;
use graph1::utils::math::oscillator;
use graph1::utils::math::rng::{shuffle, XorShiftRng};
use graph1::{buffer_op, draw, hash_random_u32};

const ON: bool = true;
const OFF: bool = false;

// Configure the user data for the demo
pub struct TextUserData {
    cursor_state: bool,

    /// Area under the cursor (for restoring after blinking)
    cursor_area: Option<RectArea>,
    /// Pixel data under the cursor (for restoring after blinking)
    cursor_data: Vec<u32>,

    cursor_position: GridPosition,

    /// Character grid for the text page with the "Marcel van Deijl" text
    page_marcel_grid: Option<UniformGrid<u32>>,
    /// Coordinates of all character cells on the "Marcel van Deijl" text page
    page_marcel_char_cells: Vec<(usize, usize)>,

    /// Index of the character cell with the checkmark (✔) on the "Marcel van Deijl" text page
    page_marcel_checkmark_char_idx: usize,

    page_marcel_shuffled_cells: Vec<Region>,
    page_n3trunn3r_shuffled_cells: Vec<Region>,
    scroller_context: GraphContext<Vec<u32>>,

    /// Index of the current character cell to interact with
    char_cell_idx: usize,
}

impl TextUserData {
    pub fn get_scroller_context(&self) -> &GraphContext<Vec<u32>> {
        &self.scroller_context
    }
}

const BLACK: u32 = 0x00_00_00_ff;
const TRANSPARENT: u32 = 0x00_00_00_00;

// 1000*120
fn make_scroller_context() -> GraphContext<Vec<u32>> {
    GraphContext::new(
        WindowContext::new(1170, 116, Some(0xffffffff), Some(0x000000ff)),
        false,
        1,
        None,
        1,
        None,
    )
}

pub fn get_text_user_data() -> TextUserData {
    TextUserData {
        cursor_state: OFF,
        cursor_area: None,
        cursor_data: vec![],
        cursor_position: GridPosition::new(0, 0),
        page_marcel_grid: None,
        page_marcel_char_cells: vec![],
        page_marcel_checkmark_char_idx: 0,
        char_cell_idx: 0,
        page_marcel_shuffled_cells: vec![],
        page_n3trunn3r_shuffled_cells: vec![],
        scroller_context: make_scroller_context(),
    }
}

/// Reset the demo state, reset the RNG in the context.
fn reset(ctx: &mut GraphContext<DemoUserData>) {
    ctx.user_data.text.cursor_state = OFF;
    ctx.user_data.text.cursor_area = None;
    ctx.user_data.text.cursor_data = vec![];
    ctx.user_data.text.cursor_position = GridPosition::new(0, 0);
    ctx.user_data.text.page_marcel_grid = None;
    ctx.user_data.text.page_marcel_char_cells = vec![];
    ctx.user_data.text.page_marcel_checkmark_char_idx = 0;
    ctx.user_data.text.char_cell_idx = 0;
    ctx.rng = XorShiftRng::default();
    ctx.user_data.text.page_marcel_shuffled_cells = vec![];
    ctx.user_data.text.page_n3trunn3r_shuffled_cells = vec![];
    ctx.user_data.text.scroller_context = make_scroller_context();
}

//
//
// ===[ NAMED CONSTANTS FOR FRAME BUFFERS ]========================================================
//
const BUF_0_MAIN: usize = 0;
const BUF_1_SCROLLER: usize = 1;
const BUF_2_PAGE_MARCEL: usize = 2;
const BUF_3_PAGE_N3TRUNN3R: usize = 3;
const BUF_4_PAGE_CLOSING: usize = 4;

//
//
// ===[ COLORS ]========================================================
//
const SCROLLER_BG_COLOR: u32 = RetroNeon::ELECTRIC_BLUE;
const SCROLLER_TEXT_COLOR: u32 = RetroNeon::CYBER_YELLOW;

const DARK_GREEN: u32 = 0x00_04_00_ff;

const MATRIKS_BG_COLOR: u32 = DARK_GREEN;
const MATRIKS_TEXT_COLOR: u32 = 0x6f_ff_43_ff;
const MATRIKS_CURSOR_COLOR: u32 = 0x11_bb_05_ff;
const RED_ALERT_TEXT_COLOR: u32 = MATRIKS_TEXT_COLOR;

//
//
// ===[ PREPARE SCROLLING TITLE ]===================================================================
//
const SCROLLER_TEXT: &str = "  Pixel fonts!  ←";
// const SCROLLER_TEXT: &str = "  Pixel fonts!  ←";
fn prepare_scrolling_title(ctx: &mut GraphContext<DemoUserData>) {
    ctx.user_data.text.scroller_context.win.background_color = SCROLLER_BG_COLOR;
    ctx.user_data.text.scroller_context.win.foreground_color = SCROLLER_TEXT_COLOR;
    clear_screen(&mut ctx.user_data.text.scroller_context);

    // Title that will be scrolled across the screen
    let title_font = instantiate_embedded_font(
        EmbeddedFonts::MatriksUaxactun,
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
        &mut ctx.user_data.text.scroller_context,
        &Point { x: 1, y: 1 },
        &title_font,
        &title_font_props,
        &SCROLLER_TEXT,
    );

    title_font_props.color = Some(RetroNeon::CYBER_YELLOW);

    printer::print_line(
        &mut ctx.user_data.text.scroller_context,
        &Point { x: 0, y: 0 },
        &title_font,
        &title_font_props,
        &SCROLLER_TEXT,
    );

    //-----------------------------------------------------

    let dims = ctx.user_data.text.scroller_context.win.dimensions.clone();
    let scroller_frame_buf_copy = &ctx.user_data.text.scroller_context.frame_buf.clone();
    clear_screen(&mut ctx.user_data.text.scroller_context);

    scale::up::sparse::to_another_buf(
        &scroller_frame_buf_copy,
        &dims,
        &RectArea::new(0, 0, 416, 116, None),
        &mut ctx.user_data.text.scroller_context.frame_buf,
        &dims,
        &Point { x: 0, y: 0 },
        2,
        &Displacement { dx: 1, dy: 1 },
        1,
    );
}

//
//
// ===[ PREPARE TEXT PAGES ]========================================================================
//
const TEXT_FONT_SCALE: u8 = 2;
const TEXT_FONT_SPACING: Spacing = Spacing {
    kerning_px: 3,
    leading_px: 4,
};

const MARCEL_PAGE_TEXT: [&str; 8] = [
    " Graph1 is shipped with a pixel ",
    " font family \"Matriks Uaxactun\", ",
    " designed by Marcel van Deijl. ",
    "--------------------------------- ",
    " You're reading this in: ",
    "  → [✔] Matriks Uaxactun Mono ",
    "  → [ ] Matriks Uaxactun Regular ",
    "---------------------------------     ",
];

const N3TRUNN3R_PAGE_TEXT: [&str; 8] = [
    "  We have 2 more fonts for you¹,",
    "  created by N3tRunn3r in 2008:",
    "   → •·C&C Red Alert [INET] ",
    "   → •·C&C Red Alert [LAN]  ",
    "  Yeah… Time flies…˚°•·¤·•°˚",
    "·································",
    "   ¹ — Used in other demos.",
    "·································",
    // "✕+✕+✕+✕+✕+✕+✕+✕+✕+✕+✕+✕+✕+✕+✕+✕+✕",
];

const CLOSING_PAGE_TEXT: [&str; 8] = [
    "  The \"Matriks Uaxactun\" fonts",
    "  contain 225 characters each, ",
    "  so you can display texts in ",
    "  most European languages ;-)",
    "••••••••••••••••••••••••••••••••• ",
    "  See module `text` in Graph1",
    "  for details on text rendering",
    "•••••••••••••••••••••••••••••••••     ",
];

///
/// Prepare the text page about Marcel van Deijl
///
fn prepare_text_marcel_page(ctx: &mut GraphContext<DemoUserData>) {
    let original_window = ctx.win.get_context();

    ctx.set_active_frame_buf(BUF_2_PAGE_MARCEL).ok();

    ctx.win.background_color = TRANSPARENT;
    ctx.win.foreground_color = MATRIKS_TEXT_COLOR;
    clear_screen(ctx);

    let text_font = instantiate_embedded_font(
        EmbeddedFonts::MatriksUaxactunMono,
        TEXT_FONT_SCALE,
        Some(TEXT_FONT_SPACING),
        None,
    );

    let title_font_props: printer::ColorProperties = printer::ColorProperties {
        color: Some(MATRIKS_TEXT_COLOR),
        color_transformer: None,
        data: None,
    };

    let text_top_left: Point = Point { x: 26, y: 19 };

    printer::print(
        ctx,
        &text_top_left,
        &text_font,
        &title_font_props,
        &MARCEL_PAGE_TEXT,
        Align::Left,
    );

    let dimensions_input: Variant<&[&str], Dimensions2d> = Variant::Primary(&MARCEL_PAGE_TEXT);

    let page_marcel_grid = make_monospaced_char_grid(
        &text_font,
        dimensions_input,
        text_top_left,
        Some(MATRIKS_CURSOR_COLOR),
    )
    .expect("Failed to create MonospacedCharGrid for Marcel page");

    let mut cells = page_marcel_grid
        .cells()
        .into_iter()
        .copied()
        .collect::<Vec<Region>>();

    // FIXME: is using `.ok()` really okay here?
    shuffle::slice(&mut cells, &mut XorShiftRng::default()).ok();

    ctx.user_data.text.page_marcel_shuffled_cells = cells;

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

            // Save the index of the checkmark character (✔) in the text,
            // so we can skip it during the initial text rendering, that's needed for a further animation
            if ch == '✔' {
                ctx.user_data.text.page_marcel_checkmark_char_idx =
                    ctx.user_data.text.page_marcel_char_cells.len();
            }
        }
    }

    // restore the original window context
    ctx.win.set_context(original_window);
    // Switch back to the main frame buffer (index 0)
    ctx.set_active_frame_buf(BUF_0_MAIN).ok();

    //-------------------------------------------------
    // END OF prepare_text_marcel_page()
}

///
/// Prepare the text page about N3trunn3r
///
fn prepare_text_n3trunn3r_page(ctx: &mut GraphContext<DemoUserData>) {
    let original_window = ctx.win.get_context();

    let res = ctx.set_active_frame_buf(BUF_3_PAGE_N3TRUNN3R); //.ok();
    if res.is_err() {
        println!("prepare_text_n3trunn3r_page(), Failed to set active frame buffer");
    }

    ctx.win.background_color = BLACK;
    ctx.win.foreground_color = MATRIKS_TEXT_COLOR;
    clear_screen(ctx);

    let text_font = instantiate_embedded_font(
        EmbeddedFonts::MatriksUaxactunMono,
        TEXT_FONT_SCALE,
        Some(TEXT_FONT_SPACING),
        None,
    );

    let title_font_props: printer::ColorProperties = printer::ColorProperties {
        color: Some(RED_ALERT_TEXT_COLOR),
        color_transformer: None,
        data: None,
    };

    let text_top_left: Point = Point { x: 26, y: 19 };

    printer::print(
        ctx,
        &text_top_left,
        &text_font,
        &title_font_props,
        &N3TRUNN3R_PAGE_TEXT,
        Align::Left,
    );

    // Create shuffled cells for the transition from N3TRUNN3R to CLOSING page
    let dimensions_input: Variant<&[&str], Dimensions2d> = Variant::Primary(&N3TRUNN3R_PAGE_TEXT);

    let page_n3trunn3r_grid = make_monospaced_char_grid(
        &text_font,
        dimensions_input,
        text_top_left,
        Some(MATRIKS_CURSOR_COLOR),
    )
    .expect("Failed to create MonospacedCharGrid for N3TRUNN3R page");

    let mut cells = page_n3trunn3r_grid
        .cells()
        .into_iter()
        .copied()
        .collect::<Vec<Region>>();

    // FIXME: is using `.ok()` really okay here?
    shuffle::slice(&mut cells, &mut XorShiftRng::default()).ok();

    ctx.user_data.text.page_n3trunn3r_shuffled_cells = cells;

    // restore the original window context
    ctx.win.set_context(original_window);
    // Switch back to the main frame buffer (index 0)
    ctx.set_active_frame_buf(BUF_0_MAIN).ok();

    //-------------------------------------------------
    // END OF prepare_text_n3trunn3r_page()
}

///
/// Prepare the text page with closing text
///
fn prepare_text_closing_page(ctx: &mut GraphContext<DemoUserData>) {
    let original_window = ctx.win.get_context();

    let res = ctx.set_active_frame_buf(BUF_4_PAGE_CLOSING); //.ok();
    if res.is_err() {
        println!("prepare_text_closing_page(), Failed to set active frame buffer");
    }

    ctx.win.background_color = BLACK;
    ctx.win.foreground_color = MATRIKS_TEXT_COLOR;
    clear_screen(ctx);

    let text_font = instantiate_embedded_font(
        EmbeddedFonts::MatriksUaxactunMono,
        TEXT_FONT_SCALE,
        Some(TEXT_FONT_SPACING),
        None,
    );

    let title_font_props: printer::ColorProperties = printer::ColorProperties {
        color: Some(MATRIKS_TEXT_COLOR),
        color_transformer: None,
        data: None,
    };

    let text_top_left: Point = Point { x: 26, y: 19 };

    printer::print(
        ctx,
        &text_top_left,
        &text_font,
        &title_font_props,
        &CLOSING_PAGE_TEXT,
        Align::Left,
    );

    // restore the original window context
    ctx.win.set_context(original_window);
    // Switch back to the main frame buffer (index 0)
    ctx.set_active_frame_buf(BUF_0_MAIN).ok();

    //-------------------------------------------------
    // END OF prepare_text_closing_page()
}

///
/// Scroll the title across the screen
///
fn scroll_the_title(ctx: &mut GraphContext<DemoUserData>) {
    let frame_count = ctx.frame_count as u32;

    let src_win_dims = ctx.user_data.text.scroller_context.win.dimensions.clone();
    let src_buf = &ctx.user_data.text.scroller_context.frame_buf.to_owned();

    let dst_win_dims = ctx.win.dimensions.clone();

    let buf_result = ctx
        .get_multi_frame_bufs(&[0, 1])
        .expect("Failed to get multiple frame buffers");

    let dst_buf = buf_result.active;

    copy::rect::to_another_buf(
        &src_buf,
        &src_win_dims,
        &RectArea::new(frame_count, 20, 678, 90, None),
        dst_buf,
        &dst_win_dims,
        &Point { x: 0, y: 72 },
        false,
        1,
    );
}

//---------------------------------------------------------------------

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

fn set_cursor(ctx: &mut GraphContext<DemoUserData>, pos: GridPosition) {
    if pos == ctx.user_data.text.cursor_position {
        return;
    }
    // extract row and col from pos
    let GridPosition { row, col } = pos;

    if ctx.user_data.text.cursor_state == ON {
        // If the cursor is currently ON, we need to restore the area under it first
        restore_area_under_cursor(ctx);

        // TODO: make sure `.unwrap()` won't fail
        let new_cursor = ctx
            .user_data
            .text
            .page_marcel_grid
            .as_ref()
            .unwrap()
            .get_cell(row, col);

        if new_cursor.is_some() {
            let new_cursor = new_cursor.unwrap().rect_area().clone();
            store_area_under_cursor(ctx, &new_cursor);
            draw::rectangle::filled(ctx, &new_cursor);
            ctx.user_data.text.cursor_area = Some(new_cursor);
            ctx.user_data.text.cursor_position = pos.clone();
        }
    }

    if ctx.user_data.text.cursor_state == OFF {
        // If the cursor is currently OFF, we just need to store the area under it
        let new_cursor = ctx
            .user_data
            .text
            .page_marcel_grid
            .as_ref()
            .unwrap()
            .get_cell(row, col);

        if new_cursor.is_some() {
            let new_cursor = new_cursor.unwrap().rect_area().clone();
            store_area_under_cursor(ctx, &new_cursor);
            ctx.user_data.text.cursor_area = Some(new_cursor);
            ctx.user_data.text.cursor_position = pos;
        }
    }
}

//
//
//
//
//
////////////////////////////////////////////////////////////////////////////////////////////////
////// [ RENDER FRAME ] ////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////
//
//
// FRAME POINTERS

const OFFSET: u32 = 500;
const SCROLL_START: u32 = 1;
const SCROLL_END: u32 = 412 + OFFSET;
const SCROLL_FADE_START: u32 = 418 + OFFSET;
const SCROLL_FADE_END: u32 = 455 + OFFSET;
const MARCEL_START: u32 = 455 + OFFSET;
const MARCEL_CHECKMARK: u32 = 1830 + OFFSET;
const MARCEL_END: u32 = 1957 + OFFSET;
const MARCEL_CHECKMARK_END: u32 = 2000 + OFFSET;
// const MARCEL_PAGE_FADE_OUT_START: u32 = 2222;
const MARCEL_PAGE_FADE_OUT_START: u32 = 2600 + OFFSET;
const TRANSITION_TO_N3TRUNN3R_PAGE: u32 = 2750 + OFFSET;
const N3TRUNN3R_PAGE_FADE_OUT_START: u32 = 3600 + OFFSET;
const DEMO_END: u32 = 4440 + OFFSET;

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let frame_count = ctx.frame_count as u32;

    if frame_count >= DEMO_END {
        return;
    }

    //=====[ DEMO INIT on frame 0 ] ===============================================================
    if frame_count == 0 {
        reset(ctx);
        clear_screen(ctx);
        prepare_scrolling_title(ctx);
        prepare_text_marcel_page(ctx);
        prepare_text_n3trunn3r_page(ctx);
        prepare_text_closing_page(ctx);
        ctx.win.background_color = SCROLLER_BG_COLOR;
        ctx.win.foreground_color = SCROLLER_TEXT_COLOR;
        clear_screen(ctx); // Clear the main buffer first
        return;
    }

    //
    //=====[ TITLE SCROLLER: YELLOW ON BLUE ] ======================================================
    if frame_count > SCROLL_START && frame_count < SCROLL_END {
        clear_screen(ctx);
        scroll_the_title(ctx);
        // Add some noise to the background
        ctx.user_data
            .intro
            .noise
            .generate_32(&mut ctx.frame_buf, None, None, &mut ctx.gpu_context);
        scanline::window(ctx, 1, 12);
    }

    //
    //=====[ FADE TO BLACK ] =======================================================================
    if frame_count > SCROLL_FADE_START && frame_count < SCROLL_FADE_END {
        fx::fade(ctx, 0x07_10_10_00, ColorOperation::Subtract, false);
    }

    //=====[ BLINKING CURSOR START ] ===============================================================
    //
    // Calculate the range of frames at which the cursor should be blinking on its initial position
    let blinking_cursor_start = SCROLL_FADE_START + (SCROLL_FADE_END - SCROLL_FADE_START) / 5 * 3;
    let blinking_cursor_end = MARCEL_START + 65;
    if frame_count > blinking_cursor_start && frame_count < blinking_cursor_end {
        if frame_count % 16 == 0 {
            ctx.user_data.text.cursor_state = !ctx.user_data.text.cursor_state;
        }
        let mut initial_cursor = ctx
            .user_data
            .text
            .page_marcel_grid
            .as_ref()
            .unwrap()
            .proto_cell
            .clone();
        initial_cursor.top_left.x += initial_cursor.dimensions.w;
        if ctx.user_data.text.cursor_state {
            initial_cursor.color = Some(BLACK);
        }
        draw::rectangle::filled(ctx, &initial_cursor);
        scanline::window(ctx, 1, 55);
    }
    if frame_count == blinking_cursor_end {
        let mut initial_cursor = ctx
            .user_data
            .text
            .page_marcel_grid
            .as_ref()
            .unwrap()
            .proto_cell
            .clone();
        initial_cursor.top_left.x += initial_cursor.dimensions.w;
        initial_cursor.color = Some(BLACK);
        draw::rectangle::filled(ctx, &initial_cursor);
    }

    //
    //=====[ RENDER MARCEL VAN DEIJL PAGE WITH TYPING CURSOR ] =====================================
    if frame_count > MARCEL_START && frame_count < MARCEL_END {
        let mut char_dst_area = RectArea::new(0, 0, 1, 1, None);
        if ctx.user_data.text.cursor_area.is_some() {
            char_dst_area = ctx.user_data.text.cursor_area.unwrap().clone();
            char_dst_area.top_left.x = char_dst_area
                .top_left
                .x
                .saturating_sub(char_dst_area.dimensions.w);
        }

        let win_dims = ctx.win.dimensions.clone();
        let mut buf_result = ctx
            // .get_multi_frame_bufs(&[0, 1])
            .get_multi_frame_bufs(&[BUF_2_PAGE_MARCEL])
            .expect("Failed to get multiple frame buffers");
        // let dst_buf = buf_result.active;
        let src_buf = buf_result.immut[0].frame_buf;

        buffer_op::copy::rect::to_another_buf(
            src_buf,
            &win_dims,
            &char_dst_area,
            &mut buf_result.active,
            &win_dims,
            &char_dst_area.top_left,
            true,
            1,
        );

        let next_char_cell_coords = ctx
            .user_data
            .text
            .page_marcel_char_cells
            .get(ctx.user_data.text.char_cell_idx);

        // `min` and `max` define the speed range of the cursor
        let mut min = 4;
        let mut max = 9;

        // Slow down the cursor when it junps to the new line
        if next_char_cell_coords.is_some() {
            if next_char_cell_coords.unwrap().1 == 0 {
                min = 59;
                max = 63;
            }

            if next_char_cell_coords.unwrap().0 == 3 || next_char_cell_coords.unwrap().0 == 7 {
                min = 4;
                max = 5;
            }
        }

        // The speed of cursor movement is randomized here
        let rand_mod = ctx.rng.get_u32(&MinMax { min, max });

        let checkmark_idx = ctx.user_data.text.page_marcel_checkmark_char_idx;

        // Skip rendering the checkmark character (✔) during the initial text rendering
        if ctx.user_data.text.char_cell_idx == checkmark_idx {
            ctx.user_data.text.char_cell_idx += 1;
            return;
        }

        let keep_on_rendering =
            ctx.user_data.text.char_cell_idx < ctx.user_data.text.page_marcel_char_cells.len();

        if keep_on_rendering && frame_count % rand_mod == 0 {
            // get coords (row, column) of the current char cell
            let char_cell_coords = ctx
                .user_data
                .text
                .page_marcel_char_cells
                .get(ctx.user_data.text.char_cell_idx);
            ctx.user_data.text.cursor_state = ON;
            if char_cell_coords.is_some() {
                let char_cell_coords = char_cell_coords.unwrap();
                let (row, col) = (char_cell_coords.0, char_cell_coords.1);
                let new_position = GridPosition::new(row, col);
                set_cursor(ctx, new_position); // TODO uncomment!
                ctx.user_data.text.char_cell_idx += 1;
            }
        }
    }

    //
    //=====[ RENDER THE CHECKMARK APPEARING ] ======================================================
    if frame_count > MARCEL_START && frame_count < MARCEL_CHECKMARK_END {
        scanline::window(ctx, 1, 15);
    }

    //
    //=====[ FADE OUT MARCEL PAGE ] ================================================================
    if frame_count > MARCEL_CHECKMARK && frame_count < MARCEL_PAGE_FADE_OUT_START {
        ctx.user_data.text.cursor_state = OFF;
        let checkmark_coords = ctx
            .user_data
            .text
            .page_marcel_char_cells
            .get(ctx.user_data.text.page_marcel_checkmark_char_idx - 1);
        if checkmark_coords.is_some() {
            let (row, col) = (checkmark_coords.unwrap().0, checkmark_coords.unwrap().1);
            let new_position = GridPosition::new(row, col);

            set_cursor(ctx, new_position); // TODO uncomment!
        }

        let char_dst_area = ctx.user_data.text.cursor_area.unwrap().clone();

        let win_dims = ctx.win.dimensions.clone();
        let mut buf_result = ctx
            .get_multi_frame_bufs(&[BUF_2_PAGE_MARCEL])
            .expect("Failed to get multiple frame buffers");
        let src_buf = buf_result.immut[0].frame_buf;

        buffer_op::copy::rect::to_another_buf(
            src_buf,
            &win_dims,
            &char_dst_area,
            &mut buf_result.active,
            &win_dims,
            &char_dst_area.top_left,
            true,
            1,
        );
        ctx.user_data.text.cursor_state = ON;
    }

    //
    //=====[ TRANSITION FROM MARCEL PAGE TO N3TRUNN3R PAGE ] =======================================
    if frame_count > MARCEL_PAGE_FADE_OUT_START && frame_count < TRANSITION_TO_N3TRUNN3R_PAGE {
        let cells_per_step = 3;
        let base_cell_index: usize =
            (frame_count - MARCEL_PAGE_FADE_OUT_START - 1) as usize * cells_per_step;

        let win_dims = ctx.win.dimensions.clone();

        for offset in 0..4 {
            let cell = ctx
                .user_data
                .text
                .page_marcel_shuffled_cells
                .get(base_cell_index + offset);
            if let Some(cell) = cell {
                let mut cell_area = cell.rect_area();

                let mut buf_result = ctx
                    .get_multi_frame_bufs(&[BUF_3_PAGE_N3TRUNN3R])
                    .expect("Failed to get multiple frame buffers");
                let src_buf = buf_result.immut[0].frame_buf;

                buffer_op::copy::rect::to_another_buf(
                    src_buf,
                    &win_dims,
                    &cell_area,
                    &mut buf_result.active,
                    &win_dims,
                    &cell_area.top_left,
                    true,
                    1,
                );
            }
        }
        //------------------------------------------
        scanline::window(ctx, 1, 4);
    }

    //
    //=====[ TRANSITION FROM N3TRUNN3R PAGE TO CLOSING PAGE ] ======================================
    if frame_count > N3TRUNN3R_PAGE_FADE_OUT_START
        && frame_count < N3TRUNN3R_PAGE_FADE_OUT_START + 700
    {
        let cells_per_step = 3;
        let base_cell_index: usize =
            (frame_count - N3TRUNN3R_PAGE_FADE_OUT_START - 1) as usize * cells_per_step;

        let win_dims = ctx.win.dimensions.clone();

        for offset in 0..4 {
            let cell = ctx
                .user_data
                .text
                .page_n3trunn3r_shuffled_cells
                .get(base_cell_index + offset);
            if let Some(cell) = cell {
                let mut cell_area = cell.rect_area();

                let mut buf_result = ctx
                    .get_multi_frame_bufs(&[BUF_4_PAGE_CLOSING])
                    .expect("Failed to get multiple frame buffers");
                let src_buf = buf_result.immut[0].frame_buf;

                buffer_op::copy::rect::to_another_buf(
                    src_buf,
                    &win_dims,
                    &cell_area,
                    &mut buf_result.active,
                    &win_dims,
                    &cell_area.top_left,
                    true,
                    1,
                );
            }
        }
        //------------------------------------------
        scanline::window(ctx, 1, 4);
    }
}
