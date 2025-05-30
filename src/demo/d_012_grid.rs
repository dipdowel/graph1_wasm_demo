use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;

use graph1::fx::{glitch, scanline};
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::{OceanBreeze, RetroNeon};

use graph1::draw::polygons::closed_perimeter;
use graph1::primitives::plane::{Dimensions2d, RectArea};
use graph1::primitives::point::Point;
use graph1::text::font::PixelFont;
use graph1::text::printer;
use graph1::utils::color::gradient;

use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::oscillator;

use graph1::draw::rectangle::filled as draw_rect;
use graph1::draw::tools::fill::flood;
use graph1::fx::glitch::HorizontalGlitchProps;
use graph1::primitives::numeric::Numeric;

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct BezierCurvesUserData {
    pub text_color_props: printer::ColorProperties<'static>,
    pub text_font: Option<PixelFont>,
}

pub const BEZIER_CURVES_USER_DATA: BezierCurvesUserData = BezierCurvesUserData {
    text_color_props: printer::ColorProperties {
        color: Some(RetroNeon::STROBE_WHITE),
        color_transformer: None,
        data: None,
    },
    // Let's put an instantiated font into the user data
    // so that we don't have to instantiate it on every frame
    text_font: None,
};
//---------------------------------------------------------------------
//---------------------------------------------------------------------

/// Struct holding customizable properties of the 3D bar
#[derive(Debug, Clone, Copy)]
pub struct Bar3DProps {
    /// X-coordinate of the bar base
    pub x: i32,
    /// Y-coordinate of the bar base
    pub y: i32,
    /// Width of the bar front face
    pub width: i32,
    /// Height of the bar (on the screen)
    pub height: i32,
    /// Depth of the 3D bar (isometric projection)
    pub depth: i32,
    /// Color of the front face
    pub color_front: u32,
    /// Color of the top face
    pub color_top: u32,
    /// Color of the side face
    pub color_side: u32,
}

// TODO:
// TODO:
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO:
// TODO:

/// Draws a pseudo-3D bar in isometric projection using polygons and flood fill.
/// Each face is drawn using `closed_perimeter()` and filled with `flood()`.
///
/// # Arguments
/// * `ctx` - Mutable reference to the GraphContext
/// * `props` - Configuration for the bar appearance and position
pub fn bar_3d<UserData>(ctx: &mut GraphContext<UserData>, props: &Bar3DProps) {
    let Bar3DProps {
        x,
        y,
        width,
        height,
        depth,
        color_front,
        color_top,
        color_side,
    } = *props;

    // FRONT face (rectangle)
    // Front face of the bar (flat rectangle on screen)
    let front = RectArea::new(x, y - height, width, height + 1, Some(color_front));
    draw_rect(ctx, &front);

    // TOP face (parallelogram) drawn using horizontal lines
    for i in 0..=depth {
        let start = Point::new(x + i, y - height - i);
        draw::line::horizontal(ctx, &start, i32::to_u32(width), Some(color_top));
    }

    // RIGHT-SIDE face (slanted parallelogram)
    let side = vec![
        Point::new(x + width, y - height),
        Point::new(x + depth + width, y - height - depth),
        Point::new(x + depth + width, y - depth),
        Point::new(x + width, y),
    ];
    closed_perimeter(ctx, &side, Some(color_side));

    flood(
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        &(side[0] + Point::new(2, 0)).to_pixel(color_side),
    );
}

//---------------------------------------------------------------------
//---------------------------------------------------------------------

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // initial context setup
    if ctx.frame_count == 0 {
        ctx.win.foreground_color = RetroNeon::GLITCH_RED;

        // let mid_blue: u32 = gradient::linear_step(OceanBreeze::DEEP_SEA_BLUE, OceanBreeze::LAGOON_BLUE, 40, 22);
        let mid_blue: u32 =
            gradient::linear_step(OceanBreeze::HARBOR_NAVY, OceanBreeze::LAGOON_BLUE, 40, 28);
        ctx.win.background_color = mid_blue;
    }

    clear_screen(ctx);

    // #############################################################################################

    let mut num_cols: i32 = 8;
    let mut num_rows: i32 = 2;

    if ctx.frame_count % 1800 > 600 {
        num_cols = 16; // good!
        num_rows = 5; // good!
    }

    if ctx.frame_count % 1800 > 1200 {
        num_cols = 24;
        num_rows = 3;
    }

    let mut cell_width = ctx.win.w_i32 / num_cols;
    let mut cell_height = ctx.win.h_i32 / num_rows;
    ctx.line.set_int_no_aa(Some(2));

    let mut grid: UniformGrid<i32> = UniformGrid::new(
        RectArea::new(0, 0, cell_width, cell_height, Some(RetroNeon::LASER_AQUA)),
        num_rows as usize,
        num_cols as usize,
        Some(vec![
            RetroNeon::CYBER_YELLOW, // RetroNeon::PSYCHEDELIC_BLUE, RetroNeon::DEEP_SPACE_BLUE, RetroNeon::ELECTRIC_BLUE,
        ]),
    );

    // if ctx.frame_count % 1800 > 600 {
    if ctx.frame_count % 300 > 100 {
        num_rows = 5;
        num_cols = 16;
        cell_width = ctx.win.w_i32 / num_cols;
        cell_height = ctx.win.h_i32 / num_rows;
        grid.resize(
            Dimensions2d::new(cell_width, cell_height),
            num_rows as usize,
            num_cols as usize,
            None,
        );
        // grid.resize_grid_auto(num_rows, num_cols, None)
    }

    // if ctx.frame_count % 1800 > 1200 {
    if ctx.frame_count % 300 > 200 {
        num_rows = 3;
        num_cols = 24;
        cell_width = ctx.win.w_i32 / num_cols;
        cell_height = ctx.win.h_i32 / num_rows;
        grid.resize(
            Dimensions2d::new(cell_width, cell_height),
            num_rows as usize,
            num_cols as usize,
            None,
        );
        // grid.resize_grid_auto(num_rows, num_cols, None)
    }

    //----------------------------------------------------------------------------------------------
    // Let's use the bottom-left corner of the first cell as the starting point for the 3D bars
    let origin = grid.get_cell(0, 0).unwrap().bottom_left();
    // Reusable configuration for the 3D bar
    let mut props = Bar3DProps {
        x: origin.x,
        y: origin.y,
        width: cell_width - 1,
        height: cell_height / 2,
        depth: 16,
        color_front: OceanBreeze::SEAFOAM,
        color_top: OceanBreeze::AQUAMARINE,
        color_side: OceanBreeze::LAGOON_BLUE,
    };
    //

    // Max height for the bars, relative to the grid cell height
    let height_bound = (cell_height as f32 * 0.78) as u32;

    //
    //----------------------------------------------------------------------------------------------
    // Generate oscillated values for animating the bar heights
    // The frequency is slightly varied to create a wave-like effect,
    // yet irregular enough to be entertaining.
    let mut osc = [
        oscillator::sine(ctx.frame_count, 0.0060, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0065, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0070, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0075, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0080, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0085, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0090, 4, height_bound) as u32,
    ];

    //
    //----------------------------------------------------------------------------------------------
    // let dbg_vect: Vec<&Region<i32>> = grid.iter().collect();

    // Iterate over the grid cells and draw a 3D bar per each cell
    grid.iter().enumerate().for_each(|(i, cell)| {
        // Let's use the bottom-left corner of each cell as the base point for the current 3D bar
        let cell_point = (*cell).bottom_left();
        props.x = cell_point.x;
        props.y = cell_point.y;
        // Let's pick up the height from the oscillated values, using the index to rotate through them
        props.height = osc[i % osc.len()] as i32;
        bar_3d(ctx, &props);
        // An extra offset to make each row look 1 step different from the previous one
        osc.rotate_left(1);
    });

    //
    //----------------------------------------------------------------------------------------------
    // Transition between scenes of the demo
    let is_transition = (ctx.frame_count % 1800 > 585 && ctx.frame_count % 1800 < 615)
        || (ctx.frame_count % 1800 > 1185 && ctx.frame_count % 1800 < 1215)
        || (ctx.frame_count % 1800 > 1785 || ctx.frame_count % 1800 < 15);

    if false && is_transition {
        glitch::horizontal_glitch(
            ctx,
            &mut HorizontalGlitchProps {
                strength: 52,
                chance: 230,
                left_right_balance: 128,
            },
            None,
        );
    }

    //----------------------------------------------------------------------------------------------
    // The usual scanline effect. Classic stuff! 😌
    scanline::window(ctx, 1, 66);
}
