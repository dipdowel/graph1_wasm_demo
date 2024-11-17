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
use graph1::utils::color::palettes::{ForestMist, RetroNeon, TropicalParadise};

const SCREEN_BG_COLOR:u32 = 0x22_09_00_ff;

/// Render a frame with a clear screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    if ctx.frame_count == 0 {}
        ctx.win.background_color = SCREEN_BG_COLOR;
        ctx.win.foreground_color = TropicalParadise::MANGO_ORANGE;
        // ctx.win.background_color = 0x000000ff;
        clear_screen(ctx);


        let text_color_props: printer::ColorProperties = printer::ColorProperties {
            color: Some(ctx.win.foreground_color),
            color_transformer: None,
            data:None
        };

        let text_font = Box::new(instantiate_embedded_font(
            EmbeddedFonts::CCRedAlertInet,
            // EmbeddedFonts::CCRedAlertLan,
            2,
            Some(Spacing {
                kerning_px: 2,
                leading_px: 3,
            }),
            None,
        ));


        let mut text_attr = printer::print(
            ctx,
            &Point { x: 40, y: 110 },
            &text_font,
            &text_color_props,
            &["> Welcome to Basic Concepts pt. 1"],
            Align::Left,
        );


    // Blinking cursor
    let color:u32 = if (ctx.frame_count/40) % 2 == 0 {
        SCREEN_BG_COLOR
    } else {
        ctx.win.foreground_color
    };

    draw::rectangle::filled(
        ctx,
        &RectArea::new(400, 104, 12, 28, Some(color)),
    );

    // apply the scanline effect
    scanline::buffer(ctx, 1, 0xf8);

}
