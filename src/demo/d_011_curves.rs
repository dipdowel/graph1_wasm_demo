use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;

use graph1::fx::scanline;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

use graph1::draw::curve::bezier_segment::BezierSegment;
use graph1::draw::polygons::{polygon, star, PolygonProperties, StarProperties};
use graph1::draw::tools::fill;
use graph1::primitives::Pixel;
use graph1::primitives::point::Point;
use graph1::utils::math::oscillator;


pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // // initial context setup
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::CYBERPUNK_FUCHSIA;
    }

    clear_screen(ctx);

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    // SOME OSCILLATION
    // let frequency_divisor = oscillator::sine(ctx.frame_count, 0.0005, 10.0, 11.0) as f32;
    let frequency_divisor = 9.0;
    let oscillator = oscillator::sine_discrete(ctx.frame_count, frequency_divisor, 33, true);

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Assumed available: `ctx`, `ctx_draft`, `oscillator`, `local_frame_count`,
    // `FINAL_GROUND_FIRST_FRAME`, `FLOWER_WIRE_FIRST_FRAME`, `BORDER_1`, `BORDER_3`, `GRADIENT_DOUBLE_LEN`, `res_delta`

    let star_props = StarProperties {
        center: ctx.win.center_pixel(true),
        num_rays: 24,
        inner_radius: oscillator as u32 + 56,
        outer_radius: 2 * oscillator as u32 + 10,
        rotation_angle: -1.0 * (ctx.frame_count as f64 / 2.0),
        skip_rendering: true,
    };

    // let star_vertices = star(ctx, &star_props_1);
    let star_vertices = star(ctx, &star_props);

    let polygon_props = PolygonProperties {
        radius: oscillator as u32 + 10,
        num_sides: 6,
        rotation_angle: ctx.frame_count as f64 / 2.0,
        center: ctx.win.center.to_pixel(RetroNeon::CHROME_CYAN),

        skip_rendering: false,
    };

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    let mut segments: Vec<BezierSegment<i32>> = Vec::new();

    segments.push(BezierSegment::new(
        star_vertices[star_vertices.len() - 1].convert(),
        star_vertices[2].convert(),
        star_vertices[0].convert(),
        star_vertices[1].convert(),
        RetroNeon::CYBER_YELLOW,
    ));

    for i in (3..star_vertices.len() - 1).step_by(3) {
        segments.push(BezierSegment::new(
            segments.last().unwrap().end,
            star_vertices[i + 2].convert(),
            star_vertices[i + 0].convert(),
            star_vertices[i + 1].convert(),
            RetroNeon::CYBER_YELLOW,
        ));
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    ctx.bezier.control_color = Some(RetroNeon::CIRCUIT_GREEN);
    // ctx.bezier.control_color = None;
    ctx.bezier.start_end_points_color = Some(RetroNeon::STROBE_WHITE);
    // ctx.bezier.start_end_points_color = None;

    ctx.bezier.render_controls = false;
    ctx.bezier.render_levers = false;

    draw::curve::bezier(ctx, &segments, 0.01);

    // The filling of the polygon and the curved body
    // Good stuff so, keep it
    let mut pix = Pixel::from(ctx.win.center.to_pixel(RetroNeon::CYBER_YELLOW));
    fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pix);

    ctx.line.width_int = 3;
    polygon(ctx, &polygon_props);
    ctx.line.width_int = 1;

    pix.color = RetroNeon::VIBRANT_CYAN;
    fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pix);

    ctx.win.foreground_color = RetroNeon::STROBE_WHITE;

    /*
        draw::curve::bezier(
            ctx,
            &[
                ctx.win.region.top(),
                ctx.win.region.right(),
                ctx.win.region.bottom(),
                ctx.win.region.left(),


            ],
            &[
                ctx.win.region.top_left().convert(),
                ctx.win.region.top_right().convert(),
                ctx.win.region.bottom_left().convert(),
                ctx.win.region.bottom_right().convert(),
                ctx.win.region.top_left().convert(),
            ],
            &[RetroNeon::STROBE_WHITE],
            0.01,
        );
    */

/*
    draw::curve::bezier(ctx, &[BezierSegment{
        start: Point::new(20.0,200.0),
        end: Point::new(50.0,100.0),
        start_control: Point::new(30.0,10.0),
        end_control: Point::new(40.0,10.0),
        color:0x0000ffff
    },
        BezierSegment{
            start: Point::new(50.0,100.0),
            end: Point::new(80.0,200.0),
            start_control: Point::new(60.0,10.0),
            end_control: Point::new(70.0,10.0),
            color:0x0000ffff
        }
    
    ], 0.0001);
  */  
    scanline::window(ctx, 1, 80);
}
