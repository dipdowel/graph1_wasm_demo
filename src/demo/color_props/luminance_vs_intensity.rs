use crate::demo::user_data::DemoUserData;
use graph1::draw::rectangle;
use graph1::graph1_core::context::GraphContext;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::palettes;
use graph1::utils::color::properties::intensity::rgba_region_intensity;
use graph1::utils::color::properties::luminance::rgba_region_luminance;

/// Side of Bouncy, in pixels
// const SQUARE_SIDE_PX: i32 = 96;

/// Number of columns in the background
const NUM_COLUMNS:u32 = 8;

fn render_background(ctx: &mut GraphContext<DemoUserData>) {

    // width of each column in the background, it is a 1/8 of the window width
    let column_width = ctx.win.w / NUM_COLUMNS;

    // Choose a color for each column
    for i in 0..NUM_COLUMNS + 2 {
        let color = match i % NUM_COLUMNS {
            0 => palettes::RetroNeon::NEON_PINK,
            1 => palettes::RetroNeon::CYBER_BLUE,
            2 => palettes::RetroNeon::LASER_LIME,
            3 => palettes::AutumnHarvest::OLIVE_GREEN,
            4 => palettes::ForestMist::MOSSY_ROCK,
            5 => palettes::ForestMist::FOREST_GREEN,
            6 => palettes::ForestMist::CEDAR_BROWN,
            7 => palettes::AutumnHarvest::CHESTNUT_BROWN,
            _ => palettes::UrbanConcrete::SLATE_GRAY,
        };

        // Render the column
        rectangle::filled(
            ctx,
            &RectArea::new(i * column_width, 0, column_width, ctx.win.h, Some(color)),
        );
    }
}
/// Illustrates how color Intensity and color Luminance are different
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // Let's have a frame number as u32 for easier calculations
    let current_frame = ctx.frame_count as u32;

    // Quarter of the window height
    let quarter_height = ctx.win.h / 4;

    // The animation lasts only for the first 120 frames
    if current_frame > 2 * quarter_height {
        return;
    }

    render_background(ctx);

    // The bar moving from top to bottom illustrates the Intensity
    let top_bar_y = current_frame / 2;
    let top_bar = RectArea::new(0, top_bar_y, ctx.win.w, quarter_height, None);

    // The bar moving from bottom to top illustrates the Luminance
    let bottom_bar_y = ctx.win.h - current_frame / 2 - quarter_height;
    let bottom_bar = RectArea::new(0, bottom_bar_y, ctx.win.w, quarter_height, None);

    // Render the top bar with Luminance
    rgba_region_luminance(ctx, &top_bar);

    // Render the bottom bar with Intensity
    rgba_region_intensity(ctx, &bottom_bar, false);
}
