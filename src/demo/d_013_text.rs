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
use graph1::utils::grid::grid_position::GridPosition;

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

    cursor_position:GridPosition,

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
        cursor_position: GridPosition::new(0,0),
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
    kerning_px: 3,
    leading_px: 4,
};
// —
// –
//which are shipped with Graph1:
const MARCEL_PAGE_TEXT: [&str; 8] = [
    " Graph1 is shipped with a pixel ",
    " font family \"Matriks Uaxactun\",",
    " designed by Marcel van Deijl." ,
    "---------------------------------",
    " You're reading this in:",
    "  → [✔] Matriks Uaxactun Mono",
    "  → [ ] Matriks Uaxactun Regular",
    "---------------------------------",

    // " → Matriks Uaxactun Mono",

    // " ", //"• • • • • • • • • • • • • ",
    // "Each font contains 222 characters,",
    // "that covers most European languages. ",
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

    let text_top_left: Point = Point { x: 26, y: 18 };

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
        Some(RetroNeon::MATRIX_GREEN),
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


fn set_cursor(ctx: &mut GraphContext<DemoUserData>, pos:GridPosition) {

    if pos == ctx.user_data.text.cursor_position {
        return;
    }
    // extract row and col from pos
    let GridPosition{row, col} = pos;

    if ctx.user_data.text.cursor_state == ON {
        // If the cursor is currently ON, we need to restore the area under it first
        restore_area_under_cursor(ctx);

        // TODO: make sure `.unwrap()` won't fail
        let new_cursor = ctx.user_data.text.page_marcel_grid.as_ref().unwrap().get_cell(row, col);

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
        let new_cursor = ctx.user_data.text.page_marcel_grid.as_ref().unwrap().get_cell(row, col);

        if new_cursor.is_some() {
            let new_cursor = new_cursor.unwrap().rect_area().clone();
            store_area_under_cursor(ctx, &new_cursor);
            ctx.user_data.text.cursor_area = Some(new_cursor);
            ctx.user_data.text.cursor_position = pos;
        }
    }

    // cursor_area.top_left.x +=
    // store_area_under_cursor(ctx, &ctx.user_data.text.cursor_area.unwrap());
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
        let mut char_dst_area = RectArea::new(0, 0, 1, 1, None);
        if ctx.user_data.text.cursor_area.is_some() {
            char_dst_area = ctx.user_data.text.cursor_area.unwrap().clone()
        }


        // ctx.set_active_frame_buf(BUF_2_PAGE_MARCEL).ok();

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
            false,
            1,
        );


        let next_char_cell_coords = ctx.user_data.text.page_marcel_char_cells.get(ctx.user_data.text.char_cell_idx);
        let mut min = 6;
        let mut max = 10;
        if next_char_cell_coords.is_some() && next_char_cell_coords.unwrap().1 == 0 {
            min = 59 ;
            max = 63;
        }

        // FIXME: uncomment
        let rand_mod = ctx.rng.get_u32(&MinMax { min, max });
        if frame_count % rand_mod == 0 {
            ctx.user_data.text.char_cell_idx += 1;
            let char_cell_coords = ctx.user_data.text.page_marcel_char_cells.get(ctx.user_data.text.char_cell_idx);
            ctx.user_data.text.cursor_state = ON;
            if char_cell_coords.is_some() {
                let char_cell_coords = char_cell_coords.unwrap();
                let (row, col) = (char_cell_coords.0, char_cell_coords.1);
                let new_position = GridPosition::new(row, col);
                set_cursor(ctx, new_position); // TODO uncomment!

            }

        }


        /*
        let next_cursor_state = if ctx.frame_count % 30 == 0 {
            !ctx.user_data.text.cursor_state
        } else {
            ctx.user_data.text.cursor_state
        };

        let idx = ctx.user_data.text.char_cell_idx;
        let char_cell_coords = ctx.user_data.text.page_marcel_char_cells.get(idx);
        // The cursor logic must begin here!
        if char_cell_coords.is_some() {
            let char_cell_coords = char_cell_coords.unwrap();
            let (row, col) = (char_cell_coords.0, char_cell_coords.1);
            let cursor = ctx
                .user_data
                .text
                .page_marcel_grid
                .as_ref()
                .unwrap()
                .get_cell(row, col)
                .unwrap()
                .rect_area()
                .clone();

            if next_cursor_state != ctx.user_data.text.cursor_state {
                if ctx.user_data.text.cursor_state == OFF {
                    store_area_under_cursor(ctx, &cursor);
                    draw::rectangle::filled(ctx, &cursor);
                }
                if ctx.user_data.text.cursor_state == ON {
                    restore_area_under_cursor(ctx);
                }
            }
        }


        // Update the cursor state in user data
        ctx.user_data.text.cursor_state = next_cursor_state;
        //---------------------------------------------------------------
        // All cursor logic must end before this line!
*/


    }

}
