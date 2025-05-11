use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;

use graph1::fx::scanline;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

use graph1::draw::curve::bezier_segment::BezierSegment;
use graph1::draw::polygons::{polygon, star, PolygonProperties, StarProperties};
use graph1::draw::tools::fill;
use graph1::primitives::point::Point;
use graph1::primitives::Pixel;
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::utils::color::gradient;
use graph1::utils::math::oscillator;

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct BezierCurvesUserData {
    pub text_color_props: printer::ColorProperties<'static>,
    pub text_font: Option<PixelFont>,
}

pub const BEZIER_CURVES_USER_DATA: BezierCurvesUserData = BezierCurvesUserData {
    text_color_props: printer::ColorProperties {
        color: Some(RetroNeon::STROBE_WHITE),
        color_transformer: None,
        data: None,
    },
    // Let's put an instantiated font into the user data
    // so that we don't have to instantiate it on every frame
    text_font: None,
};
//---------------------------------------------------------------------

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // initial context setup
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::CYBERPUNK_FUCHSIA;
        ctx.line.set_int_no_aa(Some(1));
        
        let text_color =    
            gradient::linear_step(RetroNeon::NEON_PINK, RetroNeon::DEEP_INDIGO, 32, 27);

        ctx.user_data.bezier_curves.text_color_props.color = Some(text_color);
        ctx.user_data.bezier_curves.text_font = Some(instantiate_embedded_font(
            EmbeddedFonts::CCRedAlertInet,
            2,
            Some(Spacing {
                kerning_px: 2,
                leading_px: 2,
            }),
            None,
        ));
    }

    clear_screen(ctx);

    

    // SOME OSCILLATION    
    let frequency_divisor = 9.0;
    let oscillator = oscillator::sine_discrete(ctx.frame_count, frequency_divisor, 33, true);

    
    let star_props = StarProperties {
        center: ctx.win.center_pixel(true),
        num_rays: 24,
        inner_radius: oscillator as u32 + 56,
        outer_radius: 2 * oscillator as u32 + 10,
        rotation_angle: -1.0 * (ctx.frame_count as f64 / 2.0),
        skip_rendering: true,
    };

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

    let mut show_control_points = false;
    let mut show_controls_levers = false;

    let frame_phase = ctx.frame_count % 1200;

    // decide when to show the control points and levers
    if frame_phase > 600 {
        show_control_points = true;

        if frame_phase > 900 {
            show_controls_levers = true;
        }
    }

    // force control points and levers to be drawn always
    // show_control_points = true;
    // show_controls_levers = true;

    ctx.bezier.control_color = Some(RetroNeon::DEEP_INDIGO);
    ctx.bezier.render_controls = show_control_points;
    ctx.bezier.render_levers = show_controls_levers;

    draw::curve::bezier(ctx, &segments, 0.01);


    if !show_control_points {
        // Color filling of the polygon and the curved body
        let mut pix = Pixel::from(ctx.win.center.to_pixel(RetroNeon::CYBER_YELLOW));
        fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pix);
        ctx.line.width_int = 3;
        polygon(ctx, &polygon_props);
        ctx.line.width_int = 1;
        pix.color = RetroNeon::VIBRANT_CYAN;
        fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &pix);
    }
    ctx.win.foreground_color = RetroNeon::STROBE_WHITE;

    scanline::window(ctx, 1, 80);

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // Print some info regarding the control points and levers
    //
    // Resources to print the text
    let text_color_prop = ctx.user_data.bezier_curves.text_color_props.clone();
    let text_font = ctx.user_data.bezier_curves.text_font.clone().unwrap();
    let dst = ctx.win.quadrants.bottom_right.right();

    let mut text: Vec<&str> = vec![];

    if show_control_points {
        text.push("Control points");
        if show_controls_levers {
            text.insert(0, "Levers and");
        } else {
            text.insert(0, " ");
        }
    }

    if text.len() > 0 {
        // Print the text
        printer::print(
            ctx,
            &Point {
                x: dst.x - 148,
                y: dst.y + 16,
            },
            &text_font,
            &text_color_prop,
            &text,
            printer::Align::Right,
        );
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
}
