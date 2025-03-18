use crate::demo::elements::cube::Cube;
use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::fx::scanline;
use graph1::primitives::math::MinMax;
// use graph1::fx::noise::PerlinNoiseProps;
use graph1::primitives::point::Point3D;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::math::{rgba_operation, ColorOperation};
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::math::oscillator;
use graph1::utils::math::rng::color::{ ColorRng};
use graph1::utils::math::rng::gray::GrayRng;
use graph1::utils::math::rng::XorShiftRng;
use crate::utils::console_log;

const SPEED: Point3D<f64> = Point3D {
    x: 1.4,
    y: -2.1,
    z: 1.7,
};

/// Render a frame with a clear screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let color_start = gradient::linear_step(RetroNeon::NEON_PINK, 0x0a0a0aff, 255, 120);
    let color_end = gradient::linear_step(RetroNeon::CYBER_BLUE, 0x0a0a0aff, 255, 40);

    let gradient_step = oscillator::sine(ctx.frame_count, 0.0008, 4, 198) as usize;

    ctx.win.background_color = gradient::linear_step(color_start, color_end, 400, gradient_step);

    clear_screen(ctx);

    /*
        // TODO: can we initialize the RNG only once and put it into heap?
        let range:MinMax<u32> = MinMax::new(255, 255 * 150);
        let mut rng = XorShiftRng::new(ctx.frame_count as u32, ctx.frame_count as u64);
        let random_colors = rng.get_vec_u32(ctx.frame_buf.len(), &range);

        // TODO: consider turning this into a graph1 library function for simple noise? + the step parameter for skipping pixels
        for i in (0..ctx.frame_buf.len()).step_by(4) {
            ctx.frame_buf[i] = rgba_operation(
                ctx.frame_buf[i],
                random_colors[i],
                ColorOperation::Add,
                false,
            );
        }
    */

    // TODO: can we initialize the RNG only once and put it into heap?
    let mut rng = GrayRng::new(ctx.frame_count as u32);
    let random_colors = rng.get_random_grays_32(

        ctx.frame_buf.len()/4,
        0x00,
        0x25,
        0xff,
        0xff,
        // Some(ctx.frame_count as u32),
        None
    );

    //
    // let mut rng = ColorRng::new(500, 500);
    // let random_colors = rng.get_random_colors_32(
    //     ctx.frame_buf.len(),
    //     0x00_00_00_ff,
    //     0x33_88_33_ff,
    //     Some(ctx.frame_count as u32),
    //     // Some(ctx.frame_count as u64),
    // //     None,
    // );






    // console_log(&format!(">>> random_mono_colors len {}", random_mono_colors.len()));
    // console_log(&format!(">>> num pixels {}", ctx.win.get_num_pixels()));


    // TODO: consider turning this into a graph1 library function for simple noise? + the step parameter for skipping pixels
    for i in (0..ctx.frame_buf.len()).step_by(4) {
        // ctx.frame_buf[i] = random_grays[i];
        ctx.frame_buf[i] = rgba_operation(
            ctx.frame_buf[i],
            random_colors[i/4],
            &ColorOperation::Add,
            false,
        );
    }

    // noise::perlin(&mut ctx.frame_buf, &ctx.win.dimensions_usize, &PerlinNoiseProps {
    //     octaves: 5,
    //     persistence: 13.78,
    //     lacunarity: 2.5,
    //     scale: 6.0,
    //     seed_offset: 10.0,
    //     offset: Point { x: 20.0, y:50.0},
    //     tile_size: None,
    //     seed: 225,
    //
    // });

    let cube_size = 60.0;

    let center = Point3D {
        x: ctx.win.center.x as f64 + oscillator::sine(ctx.frame_count, 0.001, -170, 170),
        y: ctx.win.center.y as f64 + oscillator::sine(ctx.frame_count, 0.005, -48, 64),
        z: 0_f64,
    };

    let speed_gain = oscillator::sine(ctx.frame_count, 0.01, -0.09, 0.09);
    // console_log(&format!("frame:{} val:{}", ctx.frame_count, speed_gain));

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

    scanline::window(ctx, 1, 48);
}
