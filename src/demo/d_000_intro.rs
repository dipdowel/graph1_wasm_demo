use graph1::core::context::GraphContext;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;
use crate::demo::user_data::DemoUserData;

/// Render a frame with a clear screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>){
    if ctx.frame_count == 0 {
        ctx.win.background_color = RetroNeon::VAPORWAVE_GRAY;
        clear_screen(ctx);
    }

}