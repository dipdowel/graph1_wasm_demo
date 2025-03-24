use crate::demo::elements::cube::Cube;
use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;

use graph1::fx::glitch::HorizontalGlitchProps;
use graph1::fx::noise::{WhiteNoise, WhiteNoiseProps};
use graph1::fx::{glitch, scanline};
use graph1::hash_random_u32;
use graph1::primitives::point::Point3D;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::math::ColorOperation;
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::math::oscillator;

const SPEED: Point3D<f64> = Point3D {
    x: 1.4,
    y: -2.1,
    z: 1.7,
};

pub struct IntroUserData<'a> {
    pub noise: WhiteNoise<'a>,
}

const NOISE_PROPS: WhiteNoiseProps = WhiteNoiseProps {
    min_color: 0x00,
    max_color: 0x25,
    min_alpha: 0xff,
    max_alpha: 0xff,
    operation: Some(ColorOperation::Add),
    step: Some(4),
};

pub fn get_user_data<'a>() -> IntroUserData<'a> {
    let noise = WhiteNoise::new(123, &NOISE_PROPS);
    IntroUserData { noise }
}

/// Render a frame with a clear screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let color_start = gradient::linear_step(RetroNeon::NEON_PINK, 0x0a0a0aff, 255, 120);
    let color_end = gradient::linear_step(RetroNeon::CYBER_BLUE, 0x0a0a0aff, 255, 40);

    let gradient_step = oscillator::sine(ctx.frame_count, 0.0008, 4, 198) as usize;

    ctx.win.background_color = gradient::linear_step(color_start, color_end, 400, gradient_step);

    clear_screen(ctx);

    // Add some noise to the background
    ctx.user_data
        .intro
        .noise
        .generate_32(&mut ctx.frame_buf, None, None);

    let cube_size = 60.0;

    let center = Point3D {
        x: ctx.win.center.x as f64 + oscillator::sine(ctx.frame_count, 0.001, -170, 170),
        y: ctx.win.center.y as f64 + oscillator::sine(ctx.frame_count, 0.005, -48, 64),
        z: 0_f64,
    };

    let speed_gain = oscillator::sine(ctx.frame_count, 0.01, -0.09, 0.09);

    let speed_accelerated = Point3D {
        x: SPEED.x + speed_gain,
        y: SPEED.y - speed_gain,
        z: SPEED.z + speed_gain,
    };

    // console_log(&format!("{:?}", speed_accelerated));

    let mut cube_inner = Cube::new(
        speed_accelerated,
        center,
        cube_size / 2.75,
        RetroNeon::LASER_LIME,
    );

    cube_inner.render(
        ctx,
        Some(&Point3D {
            x: speed_gain * -30.0,
            y: speed_gain * 40.0,
            z: speed_gain * -50.0,
        }),
    );

    let mut cube = Cube::new(
        SPEED,
        center,
        cube_size,
        gradient::linear_step(ctx.win.background_color, RetroNeon::LASER_LIME, 255, 120),
    );
    cube.render(ctx, None);

    let mut cube = Cube::new(
        SPEED,
        center,
        cube_size * 1.045,
        gradient::linear_step(ctx.win.background_color, RetroNeon::LASER_LIME, 255, 80),
    );
    cube.render(ctx, None);

    let os: u32 = oscillator::sine(50+ctx.frame_count, 0.0005, 0, 200) as u32;
    let is_glitching = os > 195 || os < 5 || (os > 90 && os < 95) || (os > 41 && os < 45);

    if is_glitching {


        let random = hash_random_u32!(ctx.frame_count);

        if random % 9 == 0 {
            glitch::horizontal_glitch(
                ctx,
                &mut HorizontalGlitchProps {
                    strength: 25,
                    chance: 220,
                    left_right_balance: 128,
                },
                None,
            );
        }
        if random % 42 == 0 {
            glitch::horizontal_glitch(
                ctx,
                &mut HorizontalGlitchProps {
                    strength: ctx.win.w / 2,
                    chance: 180,
                    left_right_balance: 128,
                },
                None,
            );
        }
    }

    scanline::window(ctx, 1, 48);
}
