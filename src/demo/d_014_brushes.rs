use crate::demo::user_data::DemoUserData;
use graph1::core::context::{GraphContext};

use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

use graph1::primitives::{plane::RectArea, point::Point};

use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::oscillator;

use crate::demo::elements::d_014_spray_config::{
    BlendSettings, BrushSettings, PathBounds, SprayDemoSceneConfig,
};
use crate::global_consts::{WIN_HEIGHT, WIN_WIDTH};
use graph1::core::context_utils::line_clipping_style::LineClippingStyle;
use graph1::draw::rectangle;
use graph1::draw::tools::brush::Brush;
use graph1::draw::tools::spray;
use graph1::fx;
use graph1::fx::scanline;
use graph1::primitives::math::{Bound, Shell};
use graph1::primitives::plane::Dimensions2d;
use graph1::utils::color::gradient;
use graph1::utils::color::math::{rgba_operation, ColorOperation};


//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct BrushUserData {
    /// Dimensions of the window, used to calculate the brush sizes in the scenes
    pub win: Dimensions2d,
    pub num_pixels: u32,
    pub scene_configs: Vec<SprayDemoSceneConfig>,
}

pub fn get_brush_user_data() -> BrushUserData {
    let greens = gradient::linear(RetroNeon::MATRIX_GREEN, 0x00_00_00_ff, 255);

    let reds_whites = gradient::linear(RetroNeon::GLITCH_RED, RetroNeon::STROBE_WHITE, 255);
    let reds_blacks = gradient::linear(RetroNeon::GLITCH_RED, 0x00_00_00_ff, 255);
    let orange_reds = gradient::linear(RetroNeon::GLITCH_RED, RetroNeon::NEON_ORANGE, 255);
    let orange_whites = gradient::linear(RetroNeon::NEON_ORANGE, RetroNeon::STROBE_WHITE, 255);

    let win = Dimensions2d::new(WIN_WIDTH, WIN_HEIGHT);
    let num_pixels = win.w * win.h;

    BrushUserData {
        win,
        num_pixels,
        scene_configs: vec![
            //
            // ###[ INFINITY ]######################################################################
            //
            SprayDemoSceneConfig {
                grid_num_cols: 10,
                grid_num_rows: 5,

                // Background color of the entire scene
                background_color: RetroNeon::DEEP_SPACE_BLUE,

                // Outer and inner grid line colors
                grid_colors: Shell {
                    outer: RetroNeon::ELECTRIC_BLUE, // grid_outer_color
                    inner: RetroNeon::DIGITAL_GOLD,  // grid_inner_color
                },

                // Frequencies for X and Y axis distortion
                frequency: Point {
                    x: 0.0045, // freq_x
                    y: 0.0090, // freq_y
                },

                // Blend modulation frequency and color
                blend: BlendSettings {
                    frequency: 4, // blend_freq
                    color: 0x04_04_03_ff, // blend_color
                                  // color: 0x00_04_06_ff,   //TODO: this blend_color is AWESOME! DEFINITELY USE IT!
                },

                /*
                pub const WIN_WIDTH: u32 = 480;
                pub const WIN_HEIGHT: u32 = 240;
                num_pixels: u32 = 480 * 240 = 115200
                 */

                // Outer brush styling
                outer_brush:{
                    let div_w = 8;
                    let div_h = 4;
                    let div_d = 886;
                    BrushSettings {
                        div_w,
                        div_h,
                        div_d,
                    dimensions: Dimensions2d::new(win.w / div_w, win.h / div_h),
                    density: num_pixels / div_d,
                    colors: vec![
                        RetroNeon::DEEP_SPACE_BLUE,
                        RetroNeon::CYBER_BLUE,
                        RetroNeon::PSYCHEDELIC_BLUE,
                        RetroNeon::ELECTRIC_BLUE,
                        RetroNeon::PULSING_PURPLE,
                    ],
                }},




                // Inner brush styling
                inner_brush:{
                    let div_w = 11;
                    let div_h = 20;
                    let div_d = 600;
                    BrushSettings {
                        div_w,
                        div_h,
                        div_d,
                    dimensions: Dimensions2d::new(win.w / div_w, win.h / div_h),
                    density: num_pixels / div_d,
                    colors: vec![
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::CYBER_YELLOW,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::NEON_ORANGE,
                    ],
                }},

                // Bounds for the animated spray path (X and Y)
                path_bounds: PathBounds {
                    x: Bound {
                        lower: 38, // path_x_lower_bound
                        upper: 38, // path_x_upper_bound
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
                    outer: 4,  // cell_outer_delta
                    inner: 14, // cell_inner_delta
                },
            },
            //
            // ###[ GREENISH GLOW ]######################################################################
            //
            SprayDemoSceneConfig {
                grid_num_cols: 3,
                grid_num_rows: 4,

                // Background color of the entire scene
                background_color: greens[190],

                // Outer and inner grid line colors
                grid_colors: Shell {
                    outer: greens[5], // grid_outer_color
                    // inner: greens[35],      // grid_inner_color
                    inner: RetroNeon::LASER_LIME,
                },

                // Grid cell delta sizes for outer and inner squares
                cell_deltas: Shell {
                    outer: 2, // cell_outer_delta
                    inner: 4, // cell_inner_delta
                },

                frequency: Point {
                    x: 0.004,
                    y: 0.0032,
                },

                // Outer brush styling
                outer_brush: {
                    let div_w = 9;
                    let div_h = 5;
                    let div_d = 2618;
                    BrushSettings {
                    div_w,
                    div_h,
                    div_d,
                    dimensions: Dimensions2d::new(win.w / div_w, win.h / div_h),
                    density: num_pixels / div_d,
                    colors: vec![
                        greens[25],
                        greens[50],
                        greens[75],
                        greens[100],
                        greens[125],
                        greens[150],
                    ],
                }
            },


                // Inner brush styling
                inner_brush: {
                    let div_w = 40;
                    let div_h = 20;
                    let div_d = 900;
                    BrushSettings {
                        div_w,
                        div_h,
                        div_d,
                        dimensions: Dimensions2d::new(win.w / div_w, win.h / div_h),
                        density: num_pixels / div_d,
                    colors: vec![
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::CYBER_YELLOW,
                        RetroNeon::MATRIX_GREEN,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::LASER_LIME,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::CYBER_YELLOW,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::LASER_LIME,
                        RetroNeon::CYBER_YELLOW,
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::NEON_ORANGE,
                    ],
                }
                },

                // Bounds for the animated spray path (X and Y)
                path_bounds: PathBounds {
                    x: Bound {
                        lower: 21, // path_x_lower_bound
                        upper: 25, // path_x_upper_bound
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
                    frequency: 3, // blend_freq
                    color: 0x01_01_01_ff, // blend_color
                                  // color: 0x00_04_06_ff,   //TODO: this blend_color is AWESOME! DEFINITELY USE IT!
                },
            },
            //
            // ###[ HOT METAL ]######################################################################
            //
            SprayDemoSceneConfig {
                grid_num_cols: 16,
                grid_num_rows: 8,

                // Background color of the entire scene
                background_color: reds_blacks[244],

                // Outer and inner grid line colors
                grid_colors: Shell {
                    outer: RetroNeon::VAPORWAVE_GRAY,
                    // inner: gradient::linear_step(RetroNeon::GLITCH_RED, RetroNeon::FUTURE_BRONZE, 32, 24),
                    inner: RetroNeon::FUTURE_BRONZE,
                },

                // Grid cell delta sizes for outer and inner squares
                cell_deltas: Shell {
                    outer: 10,
                    inner: 2,
                },

                frequency: Point {
                    x: 0.006094,
                    y: 0.018,
                },



                // Outer brush styling
                outer_brush: {
                    let div_w = 16;
                    let div_h = 8;
                    let div_d = 1800;
                    BrushSettings {
                        div_w,
                        div_h,
                        div_d,
                        dimensions: Dimensions2d::new(win.w / div_w, win.h / div_h),
                        density: num_pixels / div_d,
                    colors: vec![
                        RetroNeon::STROBE_WHITE,
                        orange_reds[100],
                        reds_whites[200],
                        orange_whites[200],
                    ],
                }
                },





                /*
        pub const WIN_WIDTH: u32 = 480;
        pub const WIN_HEIGHT: u32 = 240;
        num_pixels: u32 = 480 * 240 = 115200
*/
                /*
                // Outer brush styling
                outer_brush:{
                    let div_w = 8;
                    let div_h = 4;
                    let div_d = 886;
                    BrushSettings {
                        div_w,
                        div_h,
                        div_d,
                    dimensions: Dimensions2d::new(win.w / div_w, win.h / div_h),
                    density: num_pixels / div_d,
                        */



                // Inner brush styling
                inner_brush:{
                    let div_w = 30;
                    let div_h = 10;
                    let div_d = 518;

                    BrushSettings {
                        div_w,
                        div_h,
                        div_d,
                        dimensions: Dimensions2d::new(win.w / div_w, win.h / div_h),
                        density: num_pixels / div_d,
                    colors: vec![
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::ELECTRIC_BLUE,
                        reds_whites[200],
                        RetroNeon::STROBE_WHITE,
                        // reds_whites[120],
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::CHROME_CYAN,
                        // RetroNeon::GLITCH_RED,
                        RetroNeon::PSYCHEDELIC_BLUE,
                        RetroNeon::STROBE_WHITE,
                        orange_whites[180],
                        RetroNeon::STROBE_WHITE,
                        // orange_whites[20],
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::CHROME_CYAN,
                        // orange_whites[30],
                        RetroNeon::STROBE_WHITE,
                        RetroNeon::ELECTRIC_BLUE,
                    ],
                }
                },


                // Bounds for the animated spray path (X and Y)
                path_bounds: PathBounds {
                    x: Bound {
                        lower: 42,
                        upper: 42,
                    },
                    y: {
                        Bound {
                            lower: 42,
                            upper: 42,
                        }
                    },
                },

                // Blend modulation frequency and color
                blend: BlendSettings {
                    frequency: 8,
                    color: 0x01_04_06_ff,
                },
            },
        ],
    }
}

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {

    // Select the scene configuration based on the frame count
    //----------------------------------------------------------------------------------------------
    let mut cfg: SprayDemoSceneConfig = ctx.user_data.brush_demo.scene_configs[0].clone();

    let scene_selector = ctx.frame_count % 7500;

    if scene_selector == 2500 {
        cfg = ctx.user_data.brush_demo.scene_configs[1].clone();
        ctx.win.background_color = cfg.background_color;
    }

    if scene_selector > 2500 && scene_selector < 5000 {
        cfg = ctx.user_data.brush_demo.scene_configs[1].clone();
    }

    if scene_selector == 5000 {
        cfg = ctx.user_data.brush_demo.scene_configs[2].clone();
        ctx.win.background_color = cfg.background_color;
    }

    if scene_selector > 5000 {
        cfg = ctx.user_data.brush_demo.scene_configs[2].clone();
    }

    // let cfg = ctx.user_data.brush_demo.scene_configs[2].clone();

    // Check if the window dimensions have changed and update the user data and brush sizes accordingly
    // if ctx.user_data.brush_demo.win.w != ctx.win.w || ctx.user_data.brush_demo.win.h != ctx.win.h {
        // Update the window dimensions in the user data

        ctx.user_data.brush_demo.win = ctx.win.dimensions.clone();

        let num_pixels = ctx.win.w * ctx.win.h;
        // Update the number of pixels in the user data
        ctx.user_data.brush_demo.num_pixels = num_pixels;


        let mut ratio_factor = 1;

        let min = u32::min(ctx.win.w, ctx.win.h);
        let max = u32::max(ctx.win.w, ctx.win.h);
        if min != 0  {
            ratio_factor = max / min;
        }


        // Update the brush sizes based on the new window dimensions
        cfg.outer_brush.dimensions.w = ctx.win.w / cfg.outer_brush.div_w;
        cfg.outer_brush.dimensions.h = ctx.win.h / cfg.outer_brush.div_h * ratio_factor;
        cfg.outer_brush.density = num_pixels / cfg.outer_brush.div_d;

        cfg.inner_brush.dimensions.w = ctx.win.w / cfg.inner_brush.div_w;
        cfg.inner_brush.dimensions.h = ctx.win.h / cfg.inner_brush.div_h * ratio_factor;
        cfg.inner_brush.density = num_pixels / cfg.inner_brush.div_d;

    // }


    //**********************************************************************************************

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
        Some(vec![cfg.grid_colors.outer]),
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

    let x = oscillator::sine(
        ctx.frame_count,
        cfg.frequency.x,
        cfg.path_bounds.x.lower,
        ctx.win.w - cfg.path_bounds.x.upper,
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
        &brush_head,
        cfg.outer_brush.density,
        &cfg.outer_brush.colors,
    );

    //==============================================================================================
    // INNER BRUSH

    if ctx.win.w == WIN_WIDTH && ctx.win.h == WIN_HEIGHT {
        ctx.brush = Brush::new_rectangle(cfg.inner_brush.dimensions.w, cfg.inner_brush.dimensions.h);
    } else {
        ctx.brush = Brush::new_circle(cfg.inner_brush.dimensions.h as f64*0.64);
    }


    spray::simple(
        ctx,
        &brush_head,
        cfg.inner_brush.density,
        &cfg.inner_brush.colors,
    );


    //
    //----------------------------------------------------------------------------------------------

    // Gradually fade out the content of the draft buffer
    if ctx.frame_count % cfg.blend.frequency == 0 {
        fx::fade(ctx, cfg.blend.color, ColorOperation::Subtract, false);
    }
}
