use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;

use graph1::utils::clear_screen;
use graph1::utils::color::palettes::{OceanBreeze, RetroNeon};

use graph1::primitives::{plane::RectArea, point::Point};
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::printer;
use graph1::utils::color::gradient;

use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::oscillator;

use graph1::core::context_utils::line_clipping_style::LineClippingStyle;
use graph1::draw::rectangle;
use graph1::fx::glitch::HorizontalGlitchProps;
use graph1::fx::{glitch, scanline};
use graph1::sprites::axonometric;
use graph1::sprites::axonometric::Bar3DProps;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::utils::color::alpha::set_alpha;

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct GridUserData {
    pub text_color_props: printer::ColorProperties<'static>,
    pub text_font: Option<PixelFont>,
    pub text: Vec<String>,
    pub background_color: Option<u32>,
}

//---------------------------------------------------------------------
//---------------------------------------------------------------------

pub fn get_grid_user_data() -> GridUserData {
    GridUserData {
        background_color: Some(set_alpha(OceanBreeze::AQUAMARINE, 80)),
        text_color_props: printer::ColorProperties {
            // color: Some(OceanBreeze::FOAM_WHITE),
            color: Some(OceanBreeze::SEAFOAM),
            color_transformer: None,
            data: None,
        },
        // Let's put an instantiated font into the user data
        // so that we don't have to instantiate it on every frame
        text_font: Some(instantiate_embedded_font(
            EmbeddedFonts::CCRedAlertInet,
            2,
            Some(Spacing {
                kerning_px: 2,
                leading_px: 2,
            }),
            None,
        )),

        text: vec![
            "Grid:  8x2".to_string(),
            "Grid: 16x5".to_string(),
            "Grid: 24x3".to_string(),
        ],
    }
}

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // initial context setup
    if ctx.frame_count == 0 {
        ctx.win.foreground_color = RetroNeon::GLITCH_RED;

        let mid_blue: u32 =
            gradient::linear_step(OceanBreeze::HARBOR_NAVY, OceanBreeze::LAGOON_BLUE, 40, 28);
        ctx.win.background_color = mid_blue;
        ctx.alpha.enabled = true;
        ctx.alpha.set_method_float();

    }

    clear_screen(ctx);
    // #############################################################################################

    let mut num_cols: i32 = 8;
    let mut num_rows: i32 = 2;
    let mut text: String = ctx.user_data.grid.text[0].clone();

    if ctx.frame_count % 1800 > 600 {
        num_cols = 16;
        num_rows = 5;
        text = ctx.user_data.grid.text[1].clone();
    }

    if ctx.frame_count % 1800 > 1200 {
        num_cols = 24;
        num_rows = 3;
        text = ctx.user_data.grid.text[2].clone();
    }

    let mut cell_width = ctx.win.w_i32 / num_cols;
    let mut cell_height = ctx.win.h_i32 / num_rows;

    ctx.line.set_int_no_aa(Some(2));
    ctx.line.clipping = LineClippingStyle::CohenSutherland;

    let mut grid: UniformGrid<i32> = UniformGrid::new(
        RectArea::new(0, 0, cell_width, cell_height, Some(RetroNeon::LASER_AQUA)),
        num_rows as usize,
        num_cols as usize,
        Some(vec![
            RetroNeon::CYBER_YELLOW, // RetroNeon::PSYCHEDELIC_BLUE, RetroNeon::DEEP_SPACE_BLUE, RetroNeon::ELECTRIC_BLUE,
        ]),
    );

    //----------------------------------------------------------------------------------------------
    // Reusable configuration for the 3D bar
    let mut props_bar_3d = Bar3DProps {
        x: 0, // will be set later in the code
        y: 0, // will be set later in the code
        width: cell_width - 11,
        height: 0, // will be set later in the code
        depth: 16,
        color_front: OceanBreeze::SEAFOAM,
        color_top: OceanBreeze::AQUAMARINE,
        color_side: OceanBreeze::LAGOON_BLUE,
        slant: Point::new(1, 1),
        project_to_right: true,
    };

    if ctx.frame_count % 1800 > 600 {
        //    if ctx.frame_count % 300 > 100 {
        num_rows = 5;
        num_cols = 16;
        cell_width = ctx.win.w_i32 / num_cols;
        cell_height = ctx.win.h_i32 / num_rows;
        grid.resize_grid_auto(num_rows as usize, num_cols as usize, None);
        props_bar_3d.width = cell_width - 4; // Adjust width of the 3D bars
    }

    if ctx.frame_count % 1800 > 1200 {
        // if ctx.frame_count % 300 > 200 {
        num_rows = 3;
        num_cols = 24;
        cell_width = ctx.win.w_i32 / num_cols;
        cell_height = ctx.win.h_i32 / num_rows;
        grid.resize_grid_auto(num_rows as usize, num_cols as usize, None);
        props_bar_3d.width = cell_width - 1;
    }

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
    // Iterate over the grid cells and draw a 3D bar per each cell
    grid.iter().enumerate().for_each(|(i, cell)| {
        // if i != 9 {     return; }
        // Let's use the bottom-left corner of each cell as the base point for the current 3D bar
        let cell_point = (*cell).bottom_left();
        props_bar_3d.x = cell_point.x;
        props_bar_3d.y = cell_point.y;
        // props_bar_3d.y = cell_point.y - 20;
        // Let's pick up the height from the oscillated values, using the index to rotate through them
        props_bar_3d.height = osc[i % osc.len()] as i32;
        axonometric::bar_3d(ctx, &props_bar_3d);
        // An extra offset to make each row look 1 step different from the previous one
        osc.rotate_left(1);
    });

    //
    //----------------------------------------------------------------------------------------------
    // Text rendering preparation

    let text_offset = Point { x: 68, y: 42 };
    let text_bg_offset = Point { x: 4, y: 4 };

    let text_color_prop = ctx.user_data.grid.text_color_props;
    let text_font = ctx.user_data.grid.text_font.clone().unwrap();

    let text_dst = ctx.win.quadrants.bottom_left.center() + text_offset;
    let text_bg_dst = text_dst - text_bg_offset;

    rectangle::filled(
        ctx,
        &RectArea::new(
            0,
            text_bg_dst.y,
            ctx.win.w,
            22,
            ctx.user_data.grid.background_color,
        ),
    );

    //----------------------------------------------------------------------------------------------
    // The usual scanline effect. Classic stuff! 😌
    scanline::window(ctx, 1, 66);

    //
    //----------------------------------------------------------------------------------------------
    // Text rendering

    printer::print_line(
        ctx,
        &Point {
            x: text_dst.x,
            y: text_dst.y,
        },
        &text_font,
        &text_color_prop,
        &text,
    );

    //
    //----------------------------------------------------------------------------------------------
    // Transition between scenes of the demo
    let is_transition = (ctx.frame_count % 1800 > 585 && ctx.frame_count % 1800 < 615)
        || (ctx.frame_count % 1800 > 1185 && ctx.frame_count % 1800 < 1215)
        || (ctx.frame_count % 1800 > 1785 || ctx.frame_count % 1800 < 15);

    if is_transition {
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
}
