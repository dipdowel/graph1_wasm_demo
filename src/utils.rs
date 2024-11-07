use graph1::draw;
use graph1::core::context::GraphContext;
use web_sys::console;
use crate::demo::user_data::DemoUserData;

pub fn console_log(msg: &str) {
    console::log_1(&msg.into());
}
//
// /// Fills  the frame buffer with the background color of the window (`ctx.win.background_color`)
// pub fn (ctx: &mut GraphContext<DemoUserData>) {
//     draw::tools::fill::buffer(ctx.frame_buf, ctx.win.background_color);
// }
