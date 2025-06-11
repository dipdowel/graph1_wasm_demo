use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use std::collections::HashMap;

use graph1::utils::color::palettes::{DesertDusk, ForestMist, OceanBreeze, RetroNeon};
use graph1::utils::{clear_screen, grid};

use graph1::primitives::{neighborhood, plane::RectArea, point::Point, Pixel};
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::printer;
use graph1::utils::color::gradient;

use graph1::utils::grid::uniform::{Neighbor, UniformGrid};
use graph1::utils::math::oscillator;

use crate::utils::console_log;
use graph1::core::context_utils::line_clipping_style::LineClippingStyle;
use graph1::draw;
use graph1::draw::tools::brush::Brush;
use graph1::draw::tools::{fill, spray};
use graph1::draw::{line, rectangle};
use graph1::fx::glitch::HorizontalGlitchProps;
use graph1::fx::{glitch, scanline};
use graph1::primitives::math::MinMax;
use graph1::sprites::axonometric;
use graph1::sprites::axonometric::Bar3DProps;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::utils::color::alpha::set_alpha;
use graph1::utils::math::geometry::region::Region;
use graph1::utils::math::rng::XorShiftRng;

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct BrushUserData {
    //     pub text_color_props: printer::ColorProperties<'static>,
    //     pub text_font: Option<PixelFont>,
    //     pub text: Vec<String>,
    //     pub background_color: Option<u32>,
}

//---------------------------------------------------------------------
//---------------------------------------------------------------------

// const TILE_COLORS: [u32; 4] = [
//     DesertDusk::GOLDEN_SAND,
//     DesertDusk::DESERT_ROSE,
//     DesertDusk::RUSTY_ORANGE,
//     DesertDusk::CLAY_BROWN,
// ];

// fn render_cell(ctx: &mut GraphContext<DemoUserData>, cell: &Region<i32>, light: u8) {
//
//     // let mixer = DesertDusk::SHADOW_BROWN;
//     let mixer = ForestMist::DEEP_BARK;
//     let steps: usize = 255;
//
//     let step = steps.saturating_sub(light as usize);
//     // let step = steps.saturating_sub(light) as usize;
//
//     let tile_colors = [
//         gradient::linear_step(TILE_COLORS[0], mixer, steps, step),
//         gradient::linear_step(TILE_COLORS[1], mixer, steps, step),
//         gradient::linear_step(TILE_COLORS[2], mixer, steps, step),
//         gradient::linear_step(TILE_COLORS[3], mixer, steps, step),
//     ];
//
//     ////////////////////////////////////////////
//     // Check if the cell already has the designated colors
//     // If it does, we don't need to re-render it on this step.
//     // let  sample_point:Point<u32> = cell.top_left().convert();
//     let sample_point: Point<u32> = (cell.top_left() + Point::new(4, 2)).convert();
//     let pixel_index = (sample_point.y * ctx.win.w + sample_point.x) as usize;
//     let sample_color = ctx.frame_buf[pixel_index];
//     if sample_color == tile_colors[0] {
//         return;
//     }
//     // FIXME: This optimisation is very important! It allows us to skip rendering
//     // FIXME: cells that are not changing on this frame.
//     // FIXME: But it does not allow us to use the scanline effect.
//     // FIXME: Let's make an extra context in the user data and use it for all the rendering logic
//     // FIXME: And then just copy its frame buffer to the main context's framebuffer,
//     // FIXME: right before applying the scanline and other effects (if any).
//
//
//     ////////////////////////////////////////////
//
//     let line_color = Some(gradient::linear_step(
//         DesertDusk::SANDSTONE,
//         mixer,
//         steps,
//         step,
//     ));
//
//     let mut rect_area = cell.rect_area();
//     rect_area.color = line_color;
//
//     line::between_two_points(
//         ctx,
//         &cell.top_left().convert(),
//         &cell.bottom_right().convert(),
//         line_color,
//     );
//
//     line::between_two_points(
//         ctx,
//         &cell.top_right().convert(),
//         &cell.bottom_left().convert(),
//         line_color,
//     );
//
//     rectangle::outline(ctx, &rect_area);
//
//     let mut pixel = (cell.top_left() + Point::new(4, 2)).to_pixel(tile_colors[0]);
//     fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pixel);
//
//     pixel = (cell.top_right() + Point::new(-2, 4)).to_pixel(tile_colors[1]);
//     fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pixel);
//
//     pixel = (cell.bottom_right() + Point::new(-4, -2)).to_pixel(tile_colors[2]);
//     fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pixel);
//
//     pixel = (cell.bottom_left() + Point::new(2, -4)).to_pixel(tile_colors[3]);
//     fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pixel);
// }

pub fn get_brush_user_data() -> BrushUserData {
    BrushUserData {}
}

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let background_color = RetroNeon::DEEP_SPACE_BLUE;
    let grid_outer_color = RetroNeon::ELECTRIC_BLUE;
    let grid_inner_color = RetroNeon::DIGITAL_GOLD;

    // Frequencies
    let freq_x = 0.0045;
    let freq_y = 0.0090;

    // let freq_x = 0.0033;
    // let freq_y = 0.0055;

    // Outer brush properties
    let brush_side_outer = 54;
    let density_outer = 22;
    let colors_outer = [
        RetroNeon::DEEP_SPACE_BLUE,
        RetroNeon::ELECTRIC_BLUE,
        RetroNeon::CYBER_YELLOW,
    ];

    // Inner brush properties
    let brush_side_inner = 14;
    let density_inner = 150;
    let colors_inner = [
        RetroNeon::STROBE_WHITE,
        RetroNeon::MAGENTA_GLOW,
        RetroNeon::STROBE_WHITE,
        RetroNeon::CYBER_YELLOW,
        RetroNeon::STROBE_WHITE,
        RetroNeon::LASER_LIME,
        RetroNeon::STROBE_WHITE,
    ];

    // Path bounds
    let path_x_lower_bound = 24;
    let path_x_upper_bound = 24;

    let path_y_lower_bound = oscillator::sine(ctx.frame_count, 0.0007, 48, 60) as u32;
    let path_y_upper_bound = path_y_lower_bound;

    // let path_y_lower_bound = 60;
    // let path_y_upper_bound = 60;

    // Grid cell rendering properties
    let cell_outer_delta = 4; // can be both positive and negative
    let cell_inner_delta = 8; // can be both positive and negative

    //
    //
    //
    // initial context setup
    if ctx.frame_count == 0 {
        ctx.win.foreground_color = RetroNeon::STROBE_WHITE;
        ctx.win.background_color = background_color;
        ctx.alpha.enabled = false;
        ctx.alpha.set_method_float();
        clear_screen(ctx);
    }

    if ctx.frame_count % 8 == 0 {
        // #############################################################################################
        // THE GRID

        let num_cols = 12;
        let num_rows = 6;

        let cell_width = ctx.win.w_i32 / num_cols;
        let cell_height = ctx.win.h_i32 / num_rows;

        ctx.line.set_int_no_aa(Some(1));
        ctx.line.clipping = LineClippingStyle::CohenSutherland;

        let grid: UniformGrid<i32> = UniformGrid::new(
            RectArea::new(0, 0, cell_width, cell_height, None),
            num_rows as usize,
            num_cols as usize,
            Some(vec![grid_outer_color]),
        );

        grid.iter().enumerate().for_each(|(i, cell)| {
            let mut react_area = cell.rect_area();

            react_area.top_left =
                react_area.top_left + Point::new(cell_outer_delta, cell_outer_delta);
            react_area.dimensions.w -= cell_outer_delta * 2;
            react_area.dimensions.h -= cell_outer_delta * 2;

            rectangle::outline(ctx, &react_area);

            react_area.top_left =
                react_area.top_left + Point::new(cell_inner_delta, cell_inner_delta);
            react_area.dimensions.w -= cell_inner_delta * 2;
            react_area.dimensions.h -= cell_inner_delta * 2;
            react_area.color = Some(grid_inner_color);

            rectangle::outline(ctx, &react_area);
        });
        // #############################################################################################
    }

    // TODO: Parameterize the scanline effect, both cases!
    if ctx.frame_count % 8 == 0 {
        scanline::window(ctx, 1, 26);
    } else if ctx.frame_count % 26 == 0 {
        scanline::window(ctx, 3, 8);
    }

    // clear_screen(ctx);

    //// really cool!
    //     let freq_x = 0.004;
    //     let freq_y = 0.009;

    //  Infinity classic! 🥰
    // let freq_x = 0.001;
    // let freq_y = 0.002;

    // Infinity speedy!
    // let freq_x = 0.005;
    // let freq_y = 0.010;

    // Funny Ditch! 🥳
    // let freq_x = 0.0025;
    // let freq_y = 0.009;

    // // Eary classic!
    // let freq_x = 0.0025;
    // let freq_y = 0.010;

    //  Double pretzel! 🥨
    // let freq_x = 0.0033;
    // let freq_y = 0.0055;

    // let freq_x = 0.003257 / (ctx.frame_count % 4 ) as f64;
    // let freq_y = 0.0028291 / (ctx.frame_count % 4 ) as f64;

    // Quantum craziness! 🤪
    // let freq_x = 0.005 * (ctx.frame_count % 32 ) as f64;
    // let freq_y = 0.010 * (ctx.frame_count % 32 ) as f64;

    // Quantum craziness 2! 🤪
    // let freq_x = 0.005 / (ctx.frame_count % 32 ) as f64;
    // let freq_y = 0.010 / (ctx.frame_count % 32 ) as f64;

    // let freq_x =  oscillator::linear(ctx.frame_count, 0.0001, 2, 22);
    let x = oscillator::sine(
        ctx.frame_count,
        freq_x,
        path_x_lower_bound,
        ctx.win.w - path_x_upper_bound,
    );

    let y = oscillator::sine(
        ctx.frame_count,
        freq_y,
        path_y_lower_bound,
        ctx.win.h - path_y_upper_bound,
    );

    let brush_head: Point<u32> = Point::new(x as u32, y as u32);

    // let colors = [RetroNeon::NEON_MINT, RetroNeon::STROBE_WHITE];
    // let colors_outer = [RetroNeon::STROBE_WHITE,RetroNeon::DEEP_TEAL,RetroNeon::VIBRANT_CYAN,RetroNeon::NEON_MINT ];
    // let colors = [ RetroNeon::FUCHSIA_BLAZE, RetroNeon::MATRIX_GREEN];

    //==============================================================================================
    // OUTER BRUSH
    ctx.brush = Brush::new_rectangle(brush_side_outer, brush_side_outer);
    spray::simple(
        ctx,
        brush_head.x,
        brush_head.y,
        density_outer,
        Vec::from(colors_outer),
    );

    //==============================================================================================
    // INNER BRUSH
    ctx.brush = Brush::new_rectangle(brush_side_inner, brush_side_inner);
    spray::simple(
        ctx,
        brush_head.x,
        brush_head.y,
        density_inner,
        Vec::from(colors_inner),
    );

    //----------------------------------------------------------------------------------------------
    // The usual scanline effect. Classic stuff! 😌
    // scanline::window(ctx, 1, 5);

    //
    //----------------------------------------------------------------------------------------------

    //
}
