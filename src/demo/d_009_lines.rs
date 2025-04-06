use crate::demo::user_data::DemoUserData;
use graph1::core::context::{AntiAliasingMethod, GraphContext, RasterizationMethod};
use graph1::draw::line;
use graph1::fx::scanline;
use graph1::primitives::point::Point;
use graph1::text::font::Spacing;
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::text::printer::Align;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::{RetroNeon, SunsetGlow};
use graph1::utils::math::oscillator;

const NUM_GRADIENT_STEPS: usize = 120;

/// Illustrate the use of polygons and stars
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    let current_frame = ctx.frame_count as i32;

    // initialize the context
    if current_frame == 0 {
        ctx.win.background_color = SunsetGlow::NIGHTFALL_BLUE;
        ctx.win.foreground_color = RetroNeon::CIRCUIT_GREEN;
    }

    // Clear the screen
    clear_screen(ctx);

    let s_len = 70;

    fn draw(ctx: &mut GraphContext<DemoUserData>, start: Point<i32>, len: i32) {
        // starting coordinates
        let x = start.x;
        let y = start.y;

        let p0: Point<i32> = Point::new(x, y);
        let p1: Point<i32> = Point::new(x, y + len);
        line::between_two_points(ctx, &p0, &p1, Some(ctx.win.foreground_color));

        let p0: Point<i32> = Point::new(x + 10, y);
        let p1: Point<i32> = Point::new(x + len, y);
        line::between_two_points(ctx, &p0, &p1, Some(ctx.win.foreground_color));

        let p0: Point<i32> = Point::new(x + 10, y + 10);
        let p1: Point<i32> = Point::new(x + len, y + 23);
        line::between_two_points(ctx, &p0, &p1, Some(ctx.win.foreground_color));

        let p0: Point<i32> = Point::new(x + 20, y + 20);
        let p1: Point<i32> = Point::new(x + len, y + len);
        line::between_two_points(ctx, &p0, &p1, Some(ctx.win.foreground_color));

        let p0: Point<i32> = Point::new(x + 40, y + 30);
        let p1: Point<i32> = Point::new(x + len, y + 33);
        line::between_two_points(ctx, &p0, &p1, Some(ctx.win.foreground_color));

        let p0: Point<i32> = Point::new(x + 10, y + 20);
        let p1: Point<i32> = Point::new(x + 45, y + len);
        line::between_two_points(ctx, &p0, &p1, Some(ctx.win.foreground_color));

        let p0: Point<i32> = Point::new(x + 10, y + 30);
        let p1: Point<i32> = Point::new(x + 17, y + len);
        line::between_two_points(ctx, &p0, &p1, Some(ctx.win.foreground_color));
    }

    #[inline(always)]
    fn p(x: i32, y: i32, s_len: i32, dx: i32, dy: i32, sx: i32, sy: i32) -> Point<i32> {
        Point::new(x + s_len * sx + dx * sx, y + s_len * sy + dy * sy)
    }

    let lw = oscillator::sine(ctx.frame_count, 0.0028, 1, 9) as usize;
    let line_width = lw;

    // starting coordinates
    let x = 20;
    let y = 26;

    // spacing between the groups of lines
    let dx = 52;
    let dy = 52;

    // step along each axis
    let mut sx = 0;
    let mut sy = 0;

    ctx.line.width_int = line_width as u16;
    ctx.line.width_float = line_width as f32;

    // Start of the top raw of sample line groups. there are 4 groups of lines in the row

    sx = 0;
    sy = 0;
    ctx.line.set_int_no_aa(None);
    draw(ctx, p(x, y, s_len, dx, dy, sx, sy), s_len);

    sx = 1;
    sy = 0;
    ctx.line.set_float_no_aa(None);
    draw(ctx, p(x, y, s_len, dx, dy, sx, sy), s_len);

    sx = 2;
    sy = 0;
    ctx.line.set_int_aa_int(Some(ctx.line.width_int + 1));
    draw(ctx, p(x, y, s_len, dx, dy, sx, sy), s_len);
    ctx.line.width_int -= 1;

    sx = 3;
    sy = 0;
    ctx.line.set_float_aa_int(None);

    draw(ctx, p(x, y, s_len, dx, dy, sx, sy), s_len);

    // Start of the bottom raw of sample line groups. there are 2 groups of lines in the row

    sx = 1;
    sy = 1;
    ctx.line.set_int_aa_float(None);
    draw(ctx, p(x, y, s_len, dx, dy, sx, sy), s_len);

    sx = 2;
    sy = 1;
    ctx.line
        .set_float_aa_float(Some(ctx.line.width_float + 1.0));
    draw(ctx, p(x, y, s_len, dx, dy, sx, sy), s_len);
    ctx.line.width_float -= 1.0;

    ///////////////////////--[ PRINT THE CURRENT LINE WIDTH ]--///////////////////////

    let mut props: printer::ColorProperties = printer::ColorProperties {
        color: Some(RetroNeon::CIRCUIT_GREEN),
        color_transformer: None,
        data: None,
    };

    let sp = Some(Spacing::new(4, 4));
    let font1 = instantiate_embedded_font(EmbeddedFonts::CCRedAlertLan, 1, sp.clone(), None);
    let font2 = instantiate_embedded_font(EmbeddedFonts::CCRedAlertInet, 2, sp.clone(), None);

    let lw_str = format!("{}", line_width);

    printer::print(
        ctx,
        &Point { x: 382, y: 198 },
        &font1,
        &props,
        &[&"LINE", &"WIDTH"],
        Align::Right,
    );

    props.color = Some(RetroNeon::ELECTRIC_BLUE);
    printer::print_line(ctx, &Point { x: 446, y: 202 }, &font2, &props, &lw_str);

    scanline::window(ctx, 1, 48);
}
