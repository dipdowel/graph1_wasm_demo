// use graph1::core::context::WindowContext;
// use graph1::core::default_colors;
//
// fn draft1() {
//     // Create a window context, which contains the window dimensions and background color
//     // It exposes
//
//
//     // The simplest way to create a window context with default settings
//     let win_ctx = WindowContext::default();
//
//     // Under the hood, calling `WindowContext::default()` is equivalent to:
//     let win_ctx = WindowContext::new(
//         320, // width of the window
//         240, // height of the window
//         Some(default_colors::BACKGROUND), // background color that will be used to clear the screen
//         Some(default_colors::FOREGROUND), // foreground color that will be used to draw
//     );
//
//     // The `WindowContext` struct exposes the window's width and height as `u32`, `usize`, and `i32`
//     // as these types are often used in various calculations and operations.
//     println!("Window width as u32{}", win_ctx.w); // 320
//     println!("Window width as usize{}", win_ctx.w_usize); // 320
//     println!("Window width as i32{}", win_ctx.w_i32); // 320
//     println!("Window dimensions as `Dimensions2d`{:?}", win_ctx.dimensions); // Dimensions2d { w: 320, h: 240 }
//
//     // Graph1 is relatively low-level, so it's expected that you know what you're doing
//     // and that you won't overwrite, say, `win_ctx.w_i32` with a value that's not a valid window width.
//     // Introducing getters could make it safer, but it would also slow things down if window properties
//     // are accessed in a tight loop.
// }
