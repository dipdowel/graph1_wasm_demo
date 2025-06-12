use crate::demo::user_data::DemoUserData;
use graph1::core::context::{FrameBuffer,  GraphContext};

use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

use graph1::primitives::{plane::RectArea, point::Point};

use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::oscillator;

use graph1::core::context_utils::line_clipping_style::LineClippingStyle;
use graph1::draw::rectangle;
use graph1::draw::tools::brush::Brush;
use graph1::draw::tools::spray;
use graph1::fx::scanline;
use graph1::primitives::math::{Bound, Shell};
use graph1::primitives::plane::Dimensions2d;
use graph1::utils::color::math::{rgba_operation, ColorOperation};
use crate::demo::elements::d_014_spray_config::{BlendSettings, BrushSettings, PathBounds, SprayDemoSceneConfig};

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct BrushUserData {
    // pub brush_context: GraphContext
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
    BrushUserData {
        // brush_context: GraphContext::new(
        //     WindowContext::new(WIN_WIDTH, WIN_HEIGHT, Some(0x00_00_00_ff), Some(0x00_00_00_ff)),
        //     true, false, None, 1, None
        // ),
    }
}

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    //
    // Perform all the graphical operations on the draft buffer!
    //----------------------------------------------------------------------------------------------
    ctx.set_frame_buf_to(FrameBuffer::Draft);

     

    let cfg = SprayDemoSceneConfig {
        // Background color of the entire scene
        background_color: RetroNeon::DEEP_SPACE_BLUE,

        // Outer and inner grid line colors
        grid_colors: Shell {
            outer: RetroNeon::ELECTRIC_BLUE,     // grid_outer_color
            inner: RetroNeon::DIGITAL_GOLD,      // grid_inner_color
        },

        // Frequencies for X and Y axis distortion
        frequency: Point {
            x: 0.0045, // freq_x
            y: 0.0090, // freq_y
        },

        // Blend modulation frequency and color
        blend: BlendSettings {
            frequency: 4,           // blend_freq
            color: 0x03_03_03_ff,   // blend_color
        },

        // Outer brush styling
        outer_brush: BrushSettings {
            // dimensions: Dimensions2d::square(48), // brush_side_outer
            dimensions: Dimensions2d::new(48, 40), // brush_side_inner
            density: 88,                          // density_outer
            colors: vec![
                RetroNeon::DEEP_SPACE_BLUE,
                RetroNeon::ELECTRIC_BLUE,
                RetroNeon::DIGITAL_GOLD,
            ],
        },

        // Inner brush styling
        inner_brush: BrushSettings {
            // dimensions: Dimensions2d::square(14), // brush_side_inner
            dimensions: Dimensions2d::new(42, 12), // brush_side_inner
            density: 150, // density_inner
            colors: vec![
                RetroNeon::STROBE_WHITE,
                RetroNeon::LASER_LIME,
                RetroNeon::PULSING_PURPLE,
                RetroNeon::STROBE_WHITE,
            ],
        },

        // Bounds for the animated spray path (X and Y)
        path_bounds: PathBounds {
            x: Bound {
                lower: 42, // path_x_lower_bound
                upper: 42 // path_x_upper_bound
            },
            y: {
                
                let y = oscillator::sine(ctx.frame_count, 0.0014, 48, 60) as u32;
                
                Bound {
                    lower: y, // path_y_lower_bound
                    upper: y, // path_y_upper_bound
                }
            },
        },

        // Grid cell delta sizes for outer and inner squares
        cell_deltas: Shell {
            outer: 4, // cell_outer_delta
            inner: 8, // cell_inner_delta
        },
    };





    //**********************************************************************************************


    //
    //
    //
    // initial context setup
    if ctx.frame_count == 0 {
        ctx.win.foreground_color = RetroNeon::STROBE_WHITE;
        ctx.win.background_color = cfg.background_color;
        ctx.alpha.enabled = false;
        ctx.alpha.set_method_float();
        clear_screen(ctx);
    }

    // if ctx.frame_count % 8 == 0 {
    // if ctx.frame_count % 16 == 0 {
    if 0 == 0 {
        // #############################################################################################
        // THE GRID

        let num_cols = 10;
        let num_rows = 5;

        let cell_width = ctx.win.w_i32 / num_cols;
        let cell_height = ctx.win.h_i32 / num_rows;

        ctx.line.set_int_no_aa(Some(1));
        ctx.line.clipping = LineClippingStyle::CohenSutherland;

        let grid: UniformGrid<i32> = UniformGrid::new(
            RectArea::new(0, 0, cell_width, cell_height, None),
            num_rows as usize,
            num_cols as usize,
            Some(vec![ cfg.grid_colors.outer ]),
        );


        grid.iter().enumerate().for_each(|(i, cell)| {
            let mut react_area = cell.rect_area();



            react_area.top_left =
                react_area.top_left + Point::new(cfg.cell_deltas.outer, cfg.cell_deltas.outer);
            react_area.dimensions.w -= cfg.cell_deltas.outer * 2;
            react_area.dimensions.h -= cfg.cell_deltas.outer * 2;

            rectangle::outline(ctx, &react_area);

            react_area.top_left =
                react_area.top_left + Point::new(cfg.cell_deltas.inner, cfg.cell_deltas.inner);
            react_area.dimensions.w -= cfg.cell_deltas.inner * 2;
            react_area.dimensions.h -= cfg.cell_deltas.inner * 2;
            react_area.color = Some(cfg.grid_colors.inner);

            rectangle::outline(ctx, &react_area);
        });
        // #############################################################################################
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

    // Psychedelic stuff! 🥳
    // let freq_x: f64 = 0.0041;
    // let freq_y: f64 = 0.0094;

    // Quantum craziness 2! 🤪
    // let freq_x = 0.005 / (ctx.frame_count % 32 ) as f64;
    // let freq_y = 0.010 / (ctx.frame_count % 32 ) as f64;

    // let freq_x =  oscillator::linear(ctx.frame_count, 0.0001, 2, 22);
    let x = oscillator::sine(
        ctx.frame_count,
        cfg.frequency.x,
        cfg.path_bounds.x.lower,
        ctx.win.w - cfg.path_bounds.x.lower,
    );

    let y = oscillator::sine(
        ctx.frame_count,
        cfg.frequency.y,
        cfg.path_bounds.y.lower,
        ctx.win.h - cfg.path_bounds.y.upper,
    );

    let brush_head: Point<u32> = Point::new(x as u32, y as u32);

    //==============================================================================================
    // OUTER BRUSH
    ctx.brush = Brush::new_rectangle(cfg.outer_brush.dimensions.w, cfg.outer_brush.dimensions.h);
    spray::simple(
        ctx,
        brush_head.x,
        brush_head.y,
        cfg.outer_brush.density,
        &cfg.outer_brush.colors,
    );

    //==============================================================================================
    // INNER BRUSH
    ctx.brush = Brush::new_rectangle(cfg.inner_brush.dimensions.w, cfg.inner_brush.dimensions.h);
    spray::simple(
        ctx,
        brush_head.x,
        brush_head.y,
        cfg.inner_brush.density,
        &cfg.inner_brush.colors,
    );

    //----------------------------------------------------------------------------------------------

    // Use  the primary frame buffer again
    ctx.set_frame_buf_to(FrameBuffer::Primary);
    // Copy the draft buffer to the primary frame buffer
    ctx.copy_frame_buf(FrameBuffer::Draft, FrameBuffer::Primary);
    // The usual scanline effect. Classic stuff! 😌
    scanline::window(ctx, 1, 58);

    //
    //----------------------------------------------------------------------------------------------

    // Gradually fade out the content of the draft buffer
    if ctx.frame_count % cfg.blend.frequency == 0 {
        // apply some alpha to the draft buffer
        for i in 0..ctx.draft_buf.len() {
            ctx.draft_buf[i] = rgba_operation(
                ctx.draft_buf[i],
                cfg.blend.color,
                ColorOperation::Subtract,
                false,
            );
        }
    }
}
