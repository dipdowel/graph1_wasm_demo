use crate::demo::elements::cube::Cube;
use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::fx::scanline;
use graph1::primitives::point::Point3D;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::math::oscillator;

const SPEED: Point3D<f64> = Point3D {
    x: 1.8,
    y: -1.8,
    z: 1.8,
};

const SPEED_2: Point3D<f64> = Point3D {
    x: 1.85,
    y: -1.85,
    z: 1.85,
};

/// Render a frame with a clear screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let background_1 = gradient::linear_step(RetroNeon::NEON_PINK, 0x0a0a0aff, 255, 120);
    let background_2 = gradient::linear_step(RetroNeon::CYBER_BLUE, 0x0a0a0aff, 255, 40);

    let gradient_step = oscillator::sine(ctx.frame_count, 0.0008, 4, 198) as usize;

    ctx.win.background_color =
        gradient::linear_step(background_1, background_2, 400, gradient_step);

    clear_screen(ctx);

    let cube_size = 52.0;

    let center = Point3D {
        x: ctx.win.center.x as f64 + oscillator::sine(ctx.frame_count, 0.0008, -160, 160),
        y: ctx.win.center.y as f64  + oscillator::sine(ctx.frame_count, 0.0008, -80, 40),
        z: 0_f64,
    };

    let mut cube_inner = Cube::new(SPEED_2, center, cube_size / 3.0, RetroNeon::LASER_LIME);

    cube_inner.render(
        ctx,
        Some(&Point3D {
            x: 10.0,
            y: 40.0,
            z: 0.0,
        }),
    );

    // let mut cube = Cube::new(SPEED, center, cube_size, RetroNeon::ACID_GREEN);
    let mut cube = Cube::new(
        SPEED,
        center,
        cube_size,
        gradient::linear_step(ctx.win.background_color, RetroNeon::LASER_LIME, 255, 160),
    );

    cube.render(
        ctx,
        Some(&Point3D {
            x: 10.0,
            y: 40.0,
            z: 0.0,
        }),
    );

    let mut cube = Cube::new(
        SPEED,
        center,
        cube_size * 1.045,
        gradient::linear_step(ctx.win.background_color, RetroNeon::LASER_LIME, 255, 100),
    );

    cube.render(
        ctx,
        Some(&Point3D {
            x: 10.0,
            y: 40.0,
            z: 0.0,
        }),
    );

    scanline::window(ctx, 1, 48);
}
