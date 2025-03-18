use crate::demo::elements::snake::Snake;
use crate::demo::user_data::DemoUserData;
use graph1::core::context::{GraphContext, WindowContext};
use graph1::fx::scanline;
use graph1::utils::color::math::{rgba_operation, ColorOperation};
use graph1::utils::color::palettes::{AutumnHarvest, RetroNeon};
use graph1::utils::math::oscillator;

const WIN_WIDTH: u32 = 480;
const WIN_HEIGHT: u32 = 240;

const WIDTH_TILES: u32 = 24; // | 12 | 24 | 48
const HEIGHT_TILES: u32 = 12; // | 6 | 12 | 24

pub fn get_snake() -> Snake {
    let win_context = WindowContext::new(
        WIN_WIDTH,
        WIN_HEIGHT,
        Some(AutumnHarvest::BURNT_SIENNA),
        Some(AutumnHarvest::MUSTARD_YELLOW),
    );

    Snake::new(
        win_context,
        WIDTH_TILES / 2,
        HEIGHT_TILES / 4,
        1 + HEIGHT_TILES / 5 * 4,
        WIDTH_TILES,
        HEIGHT_TILES,
        321,
    )
}

/// Render a frame with two rectangles on the screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    //
    // Prepare resources for displaying text
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::CYBER_BLUE;
        ctx.win.foreground_color = RetroNeon::LASER_LIME;
    }

    // Divider for defining the snake speed
    let div = oscillator::sine(ctx.frame_count, 0.0008, 2, 6) as usize;

    if ctx.frame_count % div == 0 {
        ctx.user_data.snake.move_forward();
    }

    let snake_buffer = ctx.user_data.snake.get_frame_buffer();

    for i in 0..ctx.frame_buf.len() {
        if i % 9 == 0 {
            ctx.frame_buf[i] = rgba_operation(
                ctx.frame_buf[i],
                0x20_20_20_ff,
                &ColorOperation::Subtract,
                false,
            );
        } else {
            ctx.frame_buf[i] = snake_buffer[i];
        }
    }

    /*
        let mut noise:WhiteNoise = WhiteNoise::new(ctx.frame_count as u32, &WhiteNoiseProps {
            min_color: 0x5f,
            max_color: 0x5f,
            min_alpha: 0xff,
            max_alpha: 0xff,
            operation: Some(ColorOperation::Subtract),
            step: Some(9),
        });
        noise.generate_32(&mut ctx.frame_buf, None, None);
    */

    scanline::window(ctx, 1, 0x36);
}

// // TODO: can we initialize the RNG only once and put it into heap?
// let mut rng = GrayRng::new(ctx.frame_count as u32);
// let random_colors = rng.get_random_grays_32(ctx.frame_buf.len(), 0x00, 0x52, 0xff, 0xff, None);
//
//
// let step = 9;
// for i in (0..ctx.frame_buf.len()).step_by(step) {
//     // ctx.frame_buf[i] = random_grays[i];
//     ctx.frame_buf[i] = rgba_operation(
//         ctx.frame_buf[i],
//         random_colors[i / step],
//
//         &ColorOperation::Subtract,
//         false,
//     );
// }
