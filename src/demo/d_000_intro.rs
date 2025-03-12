use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::primitives::point::Point3D;
use graph1::utils::clear_screen;
use graph1::utils::color::palettes::RetroNeon;
use crate::demo::elements::cube::Cube;

/// Render a frame with a clear screen.
pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // if ctx.frame_count == 0 {
        // ctx.win.background_color = RetroNeon::VAPORWAVE_GRAY;
        // clear_screen(ctx);
    // }

    clear_screen(ctx);

    let cube_size = 20_f32;

    let mut cube = Cube::new(
        Point3D {
            x: 6_f32,
            y: -6_f32,
            z: 6_f32,
        },
        // Position the cube on the lower left part of the draft buffer
        Point3D {
            x: ctx.win.center.x as f32,
            y: ctx.win.center.y as f32,
            z: 0_f32,
        },
        cube_size,
        RetroNeon::ACID_GREEN,
    );

    cube.render_frame(ctx, None)

}
