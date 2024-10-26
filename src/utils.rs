use graph1::draw;
use graph1::graph1_core::context::GraphContext;
use web_sys::console;
use crate::demo::user_data::DemoUserData;

pub fn console_log(msg: &str) {
    console::log_1(&msg.into());
}

pub fn clear_screen(ctx: &mut GraphContext<DemoUserData>) {
    draw::tools::fill::buffer(ctx.frame_buf, ctx.win.background_color);
}
