use crate::demo::user_data::DemoUserData;
use graph1::core::context::{GraphContext, Quadrants};

use crate::demo::common::gem_stones;
use graph1::draw::line;
use graph1::draw::tools::fill;
use graph1::fx::scanline;
use graph1::primitives::point::Point;
use graph1::primitives::Pixel;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::math::geometry::region::Region;
use graph1::utils::math::oscillator;

fn get_main_gem_stone_points(ctx: &GraphContext<DemoUserData>) -> Vec<Point<u32>> {
    vec![
        ctx.win.region.top(),
        ctx.win.region.right(),
        ctx.win.region.bottom(),
        ctx.win.region.left(),
        ctx.win.region.top(),
    ]
}

/// Illustrate the use of polygons and stars
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let frame_count = ctx.frame_count;

    // initial context setup
    if frame_count == 0 {
        ctx.win.background_color = RetroNeon::FUTURE_BRONZE;
        ctx.win.foreground_color = RetroNeon::LASER_LIME;
    }

    // Skip some frames to avoid vomit-inducing effect :]
    if frame_count % 7 != 0 {
        return;
    }
    let local_count = frame_count % 480;
    if local_count > 128 {
        return;
    }

    clear_screen(ctx);

    // These points allow drawing a 'diamond'
    // by connecting the top, right, bottom and left midpoints of each side of the window.
    let gem_stone_points: Vec<Point<u32>> = get_main_gem_stone_points(&ctx);

    // let mut colors = gradient::linear(3856274175, 4143056639, 4);

    // Switching the colours a couple of times.
    ctx.win.background_color = RetroNeon::FUTURE_BRONZE;
    let mut stone = gem_stones::TOPAZ;
    if frame_count > 2248 {
        ctx.win.background_color = RetroNeon::CYBER_BLUE;
        stone = gem_stones::SAPPHIRE;
    }
    if frame_count > 2248 * 2 {
        ctx.win.background_color = RetroNeon::FUTURE_BRONZE;
        stone = gem_stones::TOPAZ;
    }

    // Make a cyclic color gradient
    let mut colors = gradient::linear(stone.0, stone.1, 5);
    let mut colors_second_half: Vec<u32> = colors.clone();
    colors.pop();
    colors_second_half.remove(0);
    colors_second_half.reverse();
    colors.append(&mut colors_second_half);
    let color = Some(colors[5]);
    colors.rotate_right(local_count % 8);


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
        Pixel {
            x: &center.x - 100,
            y: &center.y + 2,
            color: colors.pop().unwrap(),
        },
    ];

    // Fill the stone with colors
    for fill_pixel in fill_pixels {
        fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &fill_pixel);
    }


    scanline::window(ctx, 1, 58);


}
