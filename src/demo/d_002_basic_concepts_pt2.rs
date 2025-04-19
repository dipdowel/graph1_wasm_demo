use crate::demo::elements::snake::Snake;
use crate::demo::user_data::DemoUserData;
use graph1::core::context::{GraphContext, WindowContext};
use graph1::fx::scanline;
use graph1::primitives::point::Point;
use graph1::utils::color::math::{rgba_operation, ColorOperation};
use graph1::utils::color::palettes::{AutumnHarvest, RetroNeon};

const WIN_WIDTH: u32 = 480;
const WIN_HEIGHT: u32 = 240;

const WIDTH_TILES: u32 = 24; // | 12 | 24 | 48
const HEIGHT_TILES: u32 = 12; // | 6 | 12 | 24

fn get_speed_divider(speed_points: u32) -> usize {
    if speed_points > 60 {
        2
    } else if speed_points > 50 {
        3
    } else if speed_points > 40 {
        4
    } else if speed_points > 20 {
        5
    } else {
        6
    }
}

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
        3,
        WIDTH_TILES,
        HEIGHT_TILES,
        1025,
        vec![

            Point::new(23, 11),
            Point::new(9, 6),
            Point::new(12, 6),
            Point::new(17, 9),
            Point::new(1, 11),
            Point::new(3, 2),
            Point::new(14, 3),
            Point::new(7, 7),
            Point::new(5, 10),
            Point::new(4, 5),
            Point::new(8, 3),
            Point::new(11, 8),
            Point::new(13, 4),

            /*
            Point::new(7, 7),
            Point::new(9, 6),
            Point::new(23, 11),
            Point::new(22, 5),
            Point::new(1, 11),
            Point::new(3, 2),
            Point::new(4, 5),
            Point::new(8, 3),
            Point::new(5, 10),
            Point::new(12, 6),
            Point::new(17, 9),
            Point::new(14, 3),

             */
        ],
    )
}

/// Render a frame with two rectangles on the screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    //
    // Prepare resources for displaying text
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::CYBER_BLUE;
        ctx.win.foreground_color = RetroNeon::LASER_LIME;

        // reset the snake, helps the `restart` button work correctly
        ctx.user_data.snake = get_snake();
    }

    let speed_points = ctx.user_data.snake.speed_points;

    // Divider for defining the snake speed
    // (depends on speed points that the snake got after eating an apple)
    let div = get_speed_divider(speed_points);

    if ctx.frame_count % div == 0 {
        ctx.user_data.snake.move_forward();
    }

    let snake_buffer = ctx.user_data.snake.get_frame_buffer();

    for i in 0..ctx.frame_buf.len() {
        if i % 9 == 0 {
            ctx.frame_buf[i] = rgba_operation(
                ctx.frame_buf[i],
                0x20_20_20_ff,
                ColorOperation::Subtract,
                false,
            );
        } else {
            ctx.frame_buf[i] = snake_buffer[i];
        }
    }

    scanline::window(ctx, 1, 0x36);
}
