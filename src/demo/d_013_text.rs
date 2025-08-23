use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};


use graph1::utils::color::palettes::{DesertDusk, ForestMist, OceanBreeze, RetroNeon};


use graph1::primitives::{neighborhood, plane::RectArea, point::Point, Pixel};
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::printer;
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
            2,
            Some(Spacing {
                kerning_px: 2,
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

    let color_props = ctx.user_data.grid.text_color_props;
    let font = ctx.user_data.text.font.clone().unwrap();
    printer::print_line(
        ctx,
        &Point { x: 40, y: 110 },
        &font,
        &color_props,
        &" Check... check... ⁴€←↑→↓−∕✔✕� ",
    );

}
