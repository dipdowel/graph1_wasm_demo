use std::ptr;
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
use graph1::utils::color::gradient;
use graph1::utils::color::math::{rgba_operation, ColorOperation};
use crate::demo::elements::d_014_spray_config::{BlendSettings, BrushSettings, PathBounds, SprayDemoSceneConfig};

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct BrushUserData {
    pub  scene_configs:   Vec<SprayDemoSceneConfig>
}


pub fn get_brush_user_data() -> BrushUserData {


    let greens = gradient::linear(RetroNeon::MATRIX_GREEN, 0x00_00_00_ff, 255);

    BrushUserData {
        scene_configs:  vec![
            //
            // ###[ INFINITY ]######################################################################
            //
            SprayDemoSceneConfig {
                grid_num_cols:10,
                grid_num_rows:5,


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
                    color: 0x04_04_03_ff,   // blend_color
                    // color: 0x00_04_06_ff,   //TODO: this blend_color is AWESOME! DEFINITELY USE IT!
                },

                // Outer brush styling
                outer_brush: BrushSettings {
                    dimensions: Dimensions2d::square(60), // brush_side_outer
                    density: 130,                          // density_outer
                    colors: vec![
                        RetroNeon::DEEP_SPACE_BLUE,
                        RetroNeon::CYBER_BLUE ,
                        RetroNeon::PSYCHEDELIC_BLUE,
                        RetroNeon::ELECTRIC_BLUE,
                        RetroNeon::PULSING_PURPLE

                    ],
                },

                // Inner brush styling
                inner_brush: BrushSettings {
                    dimensions: Dimensions2d::new(42, 12), // brush_side_inner
                    density: 260, // density_inner
                    colors: vec![
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::CYBER_YELLOW,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::NEON_ORANGE,

                    ],
                },

                // Bounds for the animated spray path (X and Y)
                path_bounds: PathBounds {
                    x: Bound {
                        lower: 38, // path_x_lower_bound
                        upper: 38 // path_x_upper_bound
                    },
                    y: {
                        Bound {
                            lower: 44, // path_y_lower_bound
                            upper: 44, // path_y_upper_bound
                        }
                    },
                },

                // Grid cell delta sizes for outer and inner squares
                cell_deltas: Shell {
                    outer: 4, // cell_outer_delta
                    inner: 14, // cell_inner_delta
                },
            },
            //
            // ###[ GREENISH GLOW ]######################################################################
            //
            SprayDemoSceneConfig {

                grid_num_cols:3,
                grid_num_rows:4,

                // Background color of the entire scene
                background_color: greens[120],

                // Outer and inner grid line colors
                grid_colors: Shell {
                    outer: greens[5],     // grid_outer_color
                    // inner: greens[35],      // grid_inner_color
                    inner: RetroNeon::LASER_LIME
                },

                // Grid cell delta sizes for outer and inner squares
                cell_deltas: Shell {
                    outer: 2, // cell_outer_delta
                    inner: 4, // cell_inner_delta
                },

                // Frequencies for X and Y axis
                // frequency: Point {
                //     x: 0.003, // freq_x
                //     y: 0.00675, // freq_y
                // },

                frequency: Point {
                    x: 0.00402,
                    y: 0.0032,
                },



                // Outer brush styling
                outer_brush: BrushSettings {
                    dimensions: Dimensions2d::square(52),
                    density: 44,
                    colors: vec![
                        // RetroNeon::STROBE_WHITE,
                        // greens[90],
                        greens[25],
                        greens[50],
                        greens[75],
                        greens[100],
                        greens[125],
                        greens[150],
                        // greens[14],
                        // greens[180],
                        // RetroNeon::STROBE_WHITE,
                        // greens[20],
                        // greens[50],
                        // greens[80],
                        // greens[110],
                        // greens[170],
                    ],
                },

                // Inner brush styling
                inner_brush: BrushSettings {
                    dimensions: Dimensions2d::square(12), // brush_side_inner
                    density: 220, // density_inner
                    colors: vec![
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::MATRIX_GREEN,
                        RetroNeon::LASER_LIME,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::CYBER_YELLOW,
                        RetroNeon::LASER_LIME,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::STROBE_WHITE,
                    ],
                },

                // Bounds for the animated spray path (X and Y)
                path_bounds: PathBounds {
                    x: Bound {
                        lower: 21, // path_x_lower_bound
                        upper: 25 // path_x_upper_bound
                    },
                    y: {
                        Bound {
                            lower: 41, // path_y_lower_bound
                            upper: 44, // path_y_upper_bound
                        }
                    },
                },

                // Blend modulation frequency and color
                blend: BlendSettings {
                    frequency: 3,           // blend_freq
                    color: 0x01_01_01_ff,   // blend_color
                    // color: 0x00_04_06_ff,   //TODO: this blend_color is AWESOME! DEFINITELY USE IT!
                },

            },

        ],

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

  /*  // This is a temporary hack to switch between two scenes
    let mut cfg = ctx.user_data.brush_demo.scene_configs[0].clone();

    if ctx.frame_count == 1600 {
        cfg = ctx.user_data.brush_demo.scene_configs[1].clone();
        ctx.win.background_color = cfg.background_color;
        clear_screen(ctx);
    }

    if ctx.frame_count > 1600 {
        cfg = ctx.user_data.brush_demo.scene_configs[1].clone();
    }
*/
    let cfg = ctx.user_data.brush_demo.scene_configs[1].clone();

    // cfg.path_bounds.y.lower = oscillator::sine(ctx.frame_count, 0.01, 44, 64) as u32;
    // cfg.path_bounds.y.upper = cfg.path_bounds.y.lower;



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

        let num_cols = cfg.grid_num_cols;
        let num_rows = cfg.grid_num_rows;

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
    scanline::window(ctx, 1, 54);

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
