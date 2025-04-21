use crate::demo::common::gemstones;
use crate::demo::user_data::DemoUserData;
use crate::utils::console_log;
use graph1::core::context::{GraphContext, Quadrants, RasterizationMethod};
use graph1::draw;

use graph1::draw::tools::fill;
use graph1::fx::scanline;
use graph1::primitives::point::Point;
use graph1::primitives::Pixel;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::palettes::{RetroNeon, UrbanConcrete};
use graph1::utils::math::geometry::region::Region;

use graph1::draw::curve::bezier;
use graph1::draw::polygons::{polygon, star, PolygonProperties, StarProperties};
use graph1::draw::tools::fill::flood;

static GEMSTONES: [(u32, u32); 6] = [
    gemstones::EMERALD,
    gemstones::SAPPHIRE,
    gemstones::AMETHYST,
    gemstones::GARNET,
    gemstones::TOPAZ,
    gemstones::AQUAMARINE,
];

fn get_main_gem_stone_points(ctx: &GraphContext<DemoUserData>) -> Vec<Point<u32>> {
    vec![
        ctx.win.region.top(),
        ctx.win.region.right(),
        ctx.win.region.bottom(),
        ctx.win.region.left(),
        ctx.win.region.top(),
    ]
}

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // // initial context setup
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::CYBERPUNK_FUCHSIA;
    }

    clear_screen(ctx);

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    // SOME BASIC CRUDE OSCILLATION
    let frequency_adjustment_factor = 9.0; // Frequency of the change
    let sine_input = (ctx.frame_count as f32 / frequency_adjustment_factor).sin();
    let normalized_value = (sine_input + 1.0) / 2.0; // Now between 0 and 1
    let oscillator = (normalized_value * 33_f32) as usize;
    let oscillator = if oscillator == 0 { 1 } else { oscillator };

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Assumed available: `ctx`, `ctx_draft`, `oscillator`, `local_frame_count`,
    // `FINAL_GROUND_FIRST_FRAME`, `FLOWER_WIRE_FIRST_FRAME`, `BORDER_1`, `BORDER_3`, `GRADIENT_DOUBLE_LEN`, `res_delta`

    let star_props_1 = StarProperties {
        center: ctx.win.center_pixel(true),
        num_rays: 45,
        inner_radius: 34 - oscillator as u32 + 90,
        outer_radius: oscillator as u32 + 80,
        rotation_angle: ctx.frame_count as f64 / 2.0,
        skip_rendering: true,
    };

    let mut star_props_2 = star_props_1;
    star_props_2.num_rays = 24;
    star_props_2.inner_radius = oscillator as u32 + 56;
    star_props_2.outer_radius = 2 * oscillator as u32 + 10;
    star_props_2.rotation_angle *= -1.0;
    star_props_2.center.color = 0x00000000;

    let star_vertices = star(ctx, &star_props_1);
    let star_vertices_2 = star(ctx, &star_props_2);

    let polygon_props = PolygonProperties {
        radius: oscillator as u32 + 10,
        num_sides: 6,
        rotation_angle: ctx.frame_count as f64 / 2.0,
        center: ctx.win.center.to_pixel(RetroNeon::CHROME_CYAN),

        skip_rendering: false,
    };

    let mut control_points: Vec<Point<i32>> = Vec::new();
    let mut start_end_points: Vec<Point<u32>> = Vec::new();

    start_end_points.push(star_vertices[star_vertices.len() - 1].convert());

    for i in (0..star_vertices.len() - 1).step_by(3) {
        control_points.push(star_vertices[i + 0]);
        control_points.push(star_vertices[i + 1]);
        start_end_points.push(star_vertices[i + 2].convert());
    }

    control_points.clear();
    start_end_points.clear();

    start_end_points.push(star_vertices_2[star_vertices_2.len() - 1].convert());

    for i in (0..star_vertices_2.len() - 1).step_by(3) {
        control_points.push(star_vertices_2[i + 0]);
        control_points.push(star_vertices_2[i + 1]);
        start_end_points.push(star_vertices_2[i + 2].convert());
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ctx.bezier.control_color = Some(RetroNeon::DEEP_INDIGO);
    ctx.bezier.start_end_points_color = Some(RetroNeon::HOT_PINK);

    ctx.bezier.render_controls = false;
    ctx.bezier.render_levers = true;

    draw::curve::bezier(
        ctx,
        &start_end_points,
        &control_points,
        &[RetroNeon::GLITCH_RED],
        0.01,
    );

    let mut pix = Pixel::from(ctx.win.center.to_pixel(RetroNeon::CYBER_YELLOW));
    fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pix);

    ctx.line.width_int = 3;
    polygon(ctx, &polygon_props);
    ctx.line.width_int = 1;

    pix.color = RetroNeon::VIBRANT_CYAN;
    fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pix);

    scanline::window(ctx, 1, 80);
}
