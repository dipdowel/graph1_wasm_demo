use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::palettes::RetroNeon;

const WIN_WIDTH: u32 = 480;
const WIN_HEIGHT: u32 = 240;
fn main() {

    // The window context
    let win_ctx = WindowContext::new(
        WIN_WIDTH,
        WIN_HEIGHT,
        Some(RetroNeon::CYBER_BLUE),
        Some(RetroNeon::LASER_LIME),
    );

    // Disable alpha blending when you don't need it. It improves performance.
    let use_alpha = false;

    // Disabled the draft buffer when you don't need it. It saves some memory.
    let use_draft_buf = false;

    // No user data needed for this demo
    let user_data = None;

    // Graph context
    let mut ctx: GraphContext =
                        GraphContext::new(win_ctx, use_alpha, use_draft_buf, user_data);

    // Rectangle #1:
    // x = 20, y = 20, width = 60, height = 40,
    // `None` =  the default window foreground color
    let rect_1 = RectArea::new(20, 20, 60, 40, None);

    // Rectangle #2:
    // x = 80, y = 60, width = 60, height = 40, vibrant cyan color
    let rect_2 = RectArea::new(80, 60, 60, 40, Some(RetroNeon::VIBRANT_CYAN));

    // Draw both rectangles on the screen
    draw::rectangle::filled(&mut ctx, &rect_1);
    draw::rectangle::filled(&mut ctx, &rect_2);

    // Screen rendering code is omitted for brevity
}
