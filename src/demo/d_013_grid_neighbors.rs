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
use graph1::draw::tools::{fill, spray};
use graph1::draw::{line, rectangle};
use graph1::draw::tools::brush::Brush;
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
pub struct GridUserData {
    pub text_color_props: printer::ColorProperties<'static>,
    pub text_font: Option<PixelFont>,
    pub text: Vec<String>,
    pub background_color: Option<u32>,
}

//---------------------------------------------------------------------
//---------------------------------------------------------------------

const TILE_COLORS: [u32; 4] = [
    DesertDusk::GOLDEN_SAND,
    DesertDusk::DESERT_ROSE,
    DesertDusk::RUSTY_ORANGE,
    DesertDusk::CLAY_BROWN,
];

fn render_cell(ctx: &mut GraphContext<DemoUserData>, cell: &Region<i32>, light: u8) {

    // let mixer = DesertDusk::SHADOW_BROWN;
    let mixer = ForestMist::DEEP_BARK;
    let steps: usize = 255;

    let step = steps.saturating_sub(light as usize);
    // let step = steps.saturating_sub(light) as usize;

    let tile_colors = [
        gradient::linear_step(TILE_COLORS[0], mixer, steps, step),
        gradient::linear_step(TILE_COLORS[1], mixer, steps, step),
        gradient::linear_step(TILE_COLORS[2], mixer, steps, step),
        gradient::linear_step(TILE_COLORS[3], mixer, steps, step),
    ];

    ////////////////////////////////////////////
    // Check if the cell already has the designated colors
    // If it does, we don't need to re-render it on this step.
    // let  sample_point:Point<u32> = cell.top_left().convert();
    let sample_point: Point<u32> = (cell.top_left() + Point::new(4, 2)).convert();
    let pixel_index = (sample_point.y * ctx.win.w + sample_point.x) as usize;
    let sample_color = ctx.frame_buf[pixel_index];
    if sample_color == tile_colors[0] {
        return;
    }
    // FIXME: This optimisation is very important! It allows us to skip rendering
    // FIXME: cells that are not changing on this frame.
    // FIXME: But it does not allow us to use the scanline effect.
    // FIXME: Let's make an extra context in the user data and use it for all the rendering logic
    // FIXME: And then just copy its frame buffer to the main context's framebuffer,
    // FIXME: right before applying the scanline and other effects (if any).


    ////////////////////////////////////////////

    let line_color = Some(gradient::linear_step(
        DesertDusk::SANDSTONE,
        mixer,
        steps,
        step,
    ));

    let mut rect_area = cell.rect_area();
    rect_area.color = line_color;

    line::between_two_points(
        ctx,
        &cell.top_left().convert(),
        &cell.bottom_right().convert(),
        line_color,
    );

    line::between_two_points(
        ctx,
        &cell.top_right().convert(),
        &cell.bottom_left().convert(),
        line_color,
    );

    rectangle::outline(ctx, &rect_area);

    let mut pixel = (cell.top_left() + Point::new(4, 2)).to_pixel(tile_colors[0]);
    
    fill::paint_bucket( ctx,  &pixel);

    pixel = (cell.top_right() + Point::new(-2, 4)).to_pixel(tile_colors[1]);
    
    fill::paint_bucket( ctx,  &pixel);
    
    pixel = (cell.bottom_right() + Point::new(-4, -2)).to_pixel(tile_colors[2]);
    
    fill::paint_bucket( ctx,  &pixel);
    pixel = (cell.bottom_left() + Point::new(2, -4)).to_pixel(tile_colors[3]);    
    fill::paint_bucket( ctx,  &pixel);
}

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
        clear_screen(ctx);
    }

    // clear_screen(ctx);
    // #############################################################################################

    // TODO: Implement cycling between different grid sizes!!!
    let num_cols = 60;
    let num_rows = 30;
    // let num_cols = 30;
    // let num_rows = 15;
    // let num_cols = 20;
    // let num_rows = 10;

    let cell_width = ctx.win.w_i32 / num_cols;
    let cell_height = ctx.win.h_i32 / num_rows;

    ctx.line.set_int_no_aa(Some(1));
    ctx.line.clipping = LineClippingStyle::CohenSutherland;

    let grid: UniformGrid<i32> = UniformGrid::new(
        RectArea::new(0, 0, cell_width, cell_height, Some(DesertDusk::SANDSTONE)),
        num_rows as usize,
        num_cols as usize,
        Some(vec![DesertDusk::SANDSTONE]),
    );

    // TODO: Implement cycling between different neighborhood types!!!
    let neighborhood_type = neighborhood::NeighborhoodType::Circle { radius: 8};
    let light_multiplier:u8 = 29;
    // let neighborhood_type = neighborhood::NeighborhoodType::Diamond {distance:8};
    // let light_multiplier:u8 = 27;
    // let neighborhood_type = neighborhood::NeighborhoodType::Square { distance: 5 };
    // let light_multiplier: u8 = 43;


    // let booster = oscillator::sine(ctx.frame_count, 0.002, 0, 4);

    let row = oscillator::sine(ctx.frame_count, 0.004946, 0, num_rows) as usize;
    let col = oscillator::linear(ctx.frame_count, 0.1293, 0, num_cols) as usize;



    // let row = oscillator::linear(ctx.frame_count,  0.2946, 0, num_rows) as usize;
    // let col = oscillator::linear_fast(ctx.frame_count as isize,   0, num_cols as isize) as usize;
    // let row = oscillator::linear_fast(ctx.frame_count as isize,   0, num_rows as isize) as usize;

    // let row = grid.rows / 2;
    // let col = grid.cols / 2;

    let neighborhood: Vec<Neighbor<i32>> = grid.get_neighbors(row, col, &neighborhood_type, true);

    // Make a map of neighbors with their cell indices
    // This will allow us to quickly check if a cell is a neighbor
    let neighbor_map: HashMap<usize, Neighbor<i32>> = neighborhood
        .into_iter()
        .map(|n| (n.cell_index, n))
        .collect();

    grid.iter().enumerate().for_each(|(i, cell)| {
        let light: u8 = if neighbor_map.contains_key(&i) {
            let n = neighbor_map.get(&i).unwrap();
            255_u8.saturating_sub((n.distance_to_center as u8).saturating_mul(light_multiplier))
        } else {
            22
        };
        render_cell(ctx, cell, light);
    });
    //
    // let glow_center:Point<u32> = Point::new(
    //     oscillator::sine(ctx.frame_count, 0.001, 10, ctx.win.w-10) as u32,
    //     oscillator::sine(ctx.frame_count, 0.002, 10, ctx.win.h-10) as u32,
    // );
    // //
    // // let x_rnd = ctx.rng.get_u32(&MinMax{min: 0, max: 35}) ;
    // // let y_rnd = ctx.rng.get_u32(&MinMax{min: 0, max: 35}) ;
    // // let pix_x:u32 = glow_center.x+x_rnd;
    // // let pix_y:u32 = glow_center.y+y_rnd;
    // // ctx.set_pixel(pix_x, pix_y, 0xffffffff);
    // //
    //
    // let brush_side = 14;
    // ctx.brush = Brush::new_rectangle(brush_side,brush_side);
    //
    // spray::simple(
    //     ctx,
    //     glow_center.x,
    //     glow_center.y,
    //     32,
    //     0xffffffff,
    // );
    //
    //
    // spray::simple(
    //     ctx,
    //     ctx.win.w / 2,
    //     ctx.win.h / 2,
    //     250,
    //     0xffffffff,
    // );


    //----------------------------------------------------------------------------------------------
    // The usual scanline effect. Classic stuff! 😌
    // scanline::window(ctx, 1, 34);

    //
    //----------------------------------------------------------------------------------------------

    //
}
