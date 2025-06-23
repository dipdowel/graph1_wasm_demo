use crate::demo::user_data::DemoUserData;
use graph1::core::context::{GraphContext, Quadrants, RasterizationMethod};

use crate::demo::common::gemstones;
use crate::utils::console_log;
use graph1::draw::line;
use graph1::draw::tools::fill;
use graph1::fx::scanline;
use graph1::primitives::point::Point;
use graph1::primitives::Pixel;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::palettes::{RetroNeon, UrbanConcrete};
use graph1::utils::math::geometry::region::Region;

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
    let frame_count = ctx.frame_count / 4;

    // initial context setup
    if frame_count == 0 {
        ctx.win.background_color = UrbanConcrete::ASPHALT_GRAY;
        ctx.win.foreground_color = RetroNeon::LASER_LIME;
        ctx.line.anti_aliasing.enabled = false;
        ctx.line.width_int = 1;
        ctx.line.width_float = 1.0;
        ctx.line.rasterization = RasterizationMethod::Int;
    }

    clear_screen(ctx);

    // These points allow drawing a 'gemstone'
    // by connecting the top, right, bottom and left midpoints of each side of the window.
    let gem_stone_points: Vec<Point<u32>> = get_main_gem_stone_points(&ctx);

    let step = frame_count / 128;
    let idx = step % GEMSTONES.len();
    let stone = GEMSTONES[idx];

    // Make a cyclic color gradient
    let mut colors = gradient::linear(stone.0, stone.1, 5);
    let mut colors_second_half: Vec<u32> = colors.clone();
    colors.pop();
    colors_second_half.remove(0);
    colors_second_half.reverse();
    colors.append(&mut colors_second_half);

    if frame_count % 64 > 12 && frame_count % 64 < 24 {
        if frame_count % 128 < 24 {
            colors.rotate_left(frame_count % 8);
        } else {
            colors.rotate_right(frame_count % 8);
        }
    }

    let color = Some(gradient::linear_step(stone.0, stone.1, 64, 28));

    // Draw the gemstone
    for pair in gem_stone_points.windows(2) {
        line::between_two_points(ctx, &pair[0].convert(), &pair[1].convert(), color);
    }

    // draw a vertical and a horizontal lines through the center of the gemstone
    let win_region: Region<i32> = ctx.win.region.convert();
    line::between_two_points(ctx, &win_region.top(), &win_region.bottom(), color);
    line::between_two_points(ctx, &win_region.left(), &win_region.right(), color);

    // Draw the diagonal lines through the center of the gemstone
    let q: Quadrants<i32> = ctx.win.quadrants.convert();
    line::between_two_points(ctx, &q.top_left.center(), &q.bottom_right.center(), color);
    line::between_two_points(ctx, &q.top_right.center(), &q.bottom_left.center(), color);

    let center: Point<u32> = ctx.win.center;

    let fill_pixels = [
        Pixel {
            x: &center.x - 100,
            y: &center.y + 2,
            color: colors.pop().unwrap(),
        },
        Pixel {
            x: &center.x - 100,
            y: &center.y - 2,
            color: colors.pop().unwrap(),
        },
        Pixel {
            x: &center.x - 2,
            y: &center.y - 2,
            color: colors.pop().unwrap(),
        },
        Pixel {
            x: &center.x + 2,
            y: &center.y - 2,
            color: colors.pop().unwrap(),
        },
        Pixel {
            x: &center.x + 100,
            y: &center.y - 2,
            color: colors.pop().unwrap(),
        },
        Pixel {
            x: &center.x + 100,
            y: &center.y + 2,
            color: colors.pop().unwrap(),
        },
        Pixel {
            x: &center.x + 2,
            y: &center.y + 2,
            color: colors.pop().unwrap(),
        },
        Pixel {
            x: &center.x - 2,
            y: &center.y + 2,
            color: colors.pop().unwrap(),
        },
    ];

    // Fill the stone with colors
    for fill_pixel in fill_pixels {
        fill::paint_bucket(ctx, &fill_pixel);
    }

    scanline::window(ctx, 1, 58);
}
