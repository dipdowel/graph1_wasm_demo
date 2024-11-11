use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::draw;
use graph1::primitives::plane::RectArea;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;

/// Render a frame with two rectangles on the screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // Draw two rectangles on the zero-th frame, that's it.
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::ELECTRIC_BLUE;

        clear_screen(ctx);

        // Draw rectangle #1
        // x = 20, y = 20, width = 60, height = 40, default window foreground color
        draw::rectangle::filled(ctx, &RectArea::new(20, 20, 60, 40, None));

        // Draw rectangle #2
        // x = 80, y = 60, width = 60, height = 40, vibrant cyan color
        draw::rectangle::filled(
            ctx,
            &RectArea::new(80, 60, 60, 40, Some(RetroNeon::VIBRANT_CYAN)),
        );
    }
}
