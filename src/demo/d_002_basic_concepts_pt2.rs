use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;
use graph1::fx::scanline;
use graph1::primitives::plane::RectArea;
use graph1::primitives::point::Point;
use graph1::text::font::Spacing;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::text::printer::Align;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

/// Render a frame with two rectangles on the screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    //
    // Prepare resources for displaying text
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::CYBER_BLUE;
        ctx.win.foreground_color = RetroNeon::LASER_LIME;
        ctx.user_data.basic_concepts_pt1.text_font = Some(instantiate_embedded_font(
            EmbeddedFonts::CCRedAlertInet,
            2,
            Some(Spacing {
                kerning_px: 2,
                leading_px: 3,
            }),
            None,
        ));
    }

    clear_screen(ctx);

    let width = ctx.win.w - 40;

    // Draw rectangle #1
    // x = 20, y = 20, width = 440, height = 20, default window foreground color
    draw::rectangle::filled(ctx, &RectArea::new(20, 20, width, 20, None));

    // Draw rectangle #2
    // x = 20, y = 200, width = 440, height = 20, vibrant cyan color
    draw::rectangle::filled(
        ctx,
        &RectArea::new(20, ctx.win.h - 40, width, 20, Some(RetroNeon::VIBRANT_CYAN)),
    );

    // Resources to print the text
    let text_color_prop = ctx.user_data.basic_concepts_pt1.text_color_props.clone();
    let text_font = ctx.user_data.basic_concepts_pt1.text_font.clone().unwrap();

    // Print the text
    printer::print_line(
        ctx,
        &Point { x: 40, y: 110 },
        &text_font,
        &text_color_prop,
        &"> Welcome to Basic Concepts pt. 2",
    );

    // Blinking cursor
    let color: u32 = if (ctx.frame_count / 40) % 2 == 0 {
        ctx.win.background_color
    } else {
        ctx.win.foreground_color
    };
    draw::rectangle::filled(ctx, &RectArea::new(400, 104, 12, 28, Some(color)));

    // apply the scanline effect
    scanline::window(ctx, 1, 0x25);
}
