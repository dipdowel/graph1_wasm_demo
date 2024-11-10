// use graph1::core::context::{GraphContext, WindowContext};
// use graph1::draw;
// use graph1::primitives::plane::RectArea;
// use graph1::utils::color::palettes::RetroNeon;
//
// const WIN_WIDTH: u32 = 640;
// const WIN_HEIGHT: u32 = 480;
// // Each pixel needs 4 bytes (RGBA), hence `4 * width * height` bytes are needed
// const BUF_SIZE: usize = 4 * (WIN_WIDTH * WIN_HEIGHT) as usize;
//
// fn main() {
//     // The window context
//     let win_ctx = WindowContext::new(
//         WIN_WIDTH,
//         WIN_HEIGHT,
//         Some(RetroNeon::CYBER_BLUE),
//         Some(RetroNeon::LASER_LIME),
//     );
//
//     // The frame buffer is a memory region that gets rendered onto the screen
//     let mut frame_buf: [u32; BUF_SIZE] = [win_ctx.background_color; BUF_SIZE];
//
//     // let mut data:Vec<u32> = Vec::new();
//     // data.resize(BUF_SIZE, 0);
//     // data[3] = 0xff_ff_ff_ff;
//
//
//     // Graph context
//     let mut ctx = GraphContext::new(&win_ctx, &mut frame_buf, true, None);
//
//     // Draw a rectangle of size 40x20 at the top-left corner of the window
//     draw::rectangle::filled(&mut ctx, &RectArea::new(0, 0, 40, 20, None));
// }
