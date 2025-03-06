use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw::polygons::{PolygonProperties, StarProperties};
use graph1::draw::tools::fill;
use graph1::draw::{polygons, rectangle};
use graph1::primitives::plane::RectArea;
use graph1::utils::clear_screen;
use graph1::utils::color::math::gradient;
use graph1::utils::color::palettes::{RetroNeon, SunsetGlow};

const NUM_GRADIENT_STEPS: usize = 120;

/// Illustrate the use of polygons and stars
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let current_frame = ctx.frame_count as i32;
    let current_frame_f64 = ctx.frame_count as f64;

    // initialize the context
    if current_frame == 0 {
        ctx.win.background_color = SunsetGlow::GENTLE_INDIGO;
    }

    // Clear the screen
    clear_screen(ctx);

    // Width of each column in the background is 20% of the window width
    let column_width = (ctx.win.w as f64 / NUM_GRADIENT_STEPS as f64) as u32;

    let light_pink: u32 = gradient::single_step(RetroNeon::MAGENTA_GLOW, 0xff_ff_ff_ff, 40, 24);
    let light_pink_2: u32 = gradient::single_step(RetroNeon::MAGENTA_GLOW, 0xff_ff_ff_ff, 40, 26);
    let gradient = gradient::simple(light_pink, RetroNeon::NEON_PINK, NUM_GRADIENT_STEPS);

    for i in 0..NUM_GRADIENT_STEPS {
        rectangle::filled(
            ctx,
            &RectArea::new(
                i as u32 * column_width,
                0,
                column_width,
                ctx.win.h,
                Some(gradient[i]),
            ),
        );
    }

    // Initial state of the polygon properties
    let mut polygon_props = PolygonProperties {
        center: ctx.win.center.to_pixel(light_pink_2),
        num_sides: 5,
        radius: 76 + (f64::sin(current_frame_f64 / 100.0) * 44.0) as u32,
        rotation_angle: current_frame_f64 * 2.4,
        skip_rendering: false,
    };

    // Draw the rotating polygons + modify the properties for each next polygon
    for i in 0..12 {
        polygons::polygon(ctx, &polygon_props);
        polygon_props.radius += 4;
        polygon_props.rotation_angle -= 360.0 / 12.0;
        polygon_props.rotation_angle *= 1.025;
    }

    // Vertical strips in the center, within the bounds of the rotating polygons
    for i in -12..12 {
        let mut pixel = ctx.win.center.to_pixel(gradient[NUM_GRADIENT_STEPS / 4]);
        pixel.x = (pixel.x as i32 + i * 8) as u32;
        fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pixel);
    }

    // Initial state of the star properties
    let mut star_props = StarProperties {
        center: ctx.win.center.to_pixel(0xff_ff_ff_ff),
        num_rays: 5,
        inner_radius: 12 + (f64::sin(current_frame_f64 / 100.0) * 40.0) as u32,
        outer_radius: 30 + (f64::sin(current_frame_f64 / 100.0) * 58.0) as u32,
        rotation_angle: current_frame_f64,
        skip_rendering: false,
    };

    // Draw the rotating stars + modify the properties for each next star
    for i in 0..4 {
        polygons::star(ctx, &star_props);
        star_props.center.color =
            gradient::single_step(0xff_ff_ff_ff, RetroNeon::NEON_PINK, 40, 10 * i as usize);
        star_props.inner_radius += 3;
        star_props.outer_radius += i * 4;
    }
}
