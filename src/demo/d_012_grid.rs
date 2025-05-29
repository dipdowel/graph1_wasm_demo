use crate::demo::user_data::DemoUserData;
use graph1::core::context::GraphContext;
use graph1::{draw, hash_random_u32};

use graph1::fx::{glitch, scanline};
use graph1::utils::color::palettes::{OceanBreeze, RetroNeon};
use graph1::utils::{clear_screen, grid};

use graph1::draw::curve::bezier_segment::BezierSegment;
use graph1::draw::polygons::{closed_perimeter, polygon, star, PolygonProperties, StarProperties};
use graph1::draw::tools::fill;
use graph1::primitives::neighborhood::NeighborhoodType;
use graph1::primitives::numeric::Numeric;
use graph1::primitives::plane::{Dimensions2d, RectArea};
use graph1::primitives::point::{Point, POINT_ONE};
use graph1::primitives::Pixel;
use graph1::text::font::{PixelFont, Spacing};
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::printer;
use graph1::utils::color::{alpha, gradient};

use graph1::utils::grid::uniform::UniformGrid;
use graph1::utils::math::geometry::region::Region;
use graph1::utils::math::oscillator;

use graph1::draw::rectangle::filled as draw_rect;
use graph1::draw::tools::fill::flood;
use graph1::fx::glitch::HorizontalGlitchProps;

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
//---------------------------------------------------------------------

/// Struct holding customizable properties of the 3D bar
#[derive(Debug, Clone, Copy)]
pub struct Bar3DProps {
    /// X-coordinate of the bar base
    pub x: u32,
    /// Y-coordinate of the bar base
    pub y: u32,
    /// Width of the bar front face
    pub width: u32,
    /// Height of the bar (on the screen)
    pub height: u32,
    /// Depth of the 3D bar (isometric projection)
    pub depth: u32,
    /// Color of the front face
    pub color_front: u32,
    /// Color of the top face
    pub color_top: u32,
    /// Color of the side face
    pub color_side: u32,
}

// TODO:
// TODO:
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO: Consinder moving `bar_3d()` to Graph1 !!!!
// TODO:
// TODO:

/// Draws a pseudo-3D bar in isometric projection using polygons and flood fill.
/// Each face is drawn using `closed_perimeter()` and filled with `flood()`.
///
/// # Arguments
/// * `ctx` - Mutable reference to the GraphContext
/// * `props` - Configuration for the bar appearance and position
pub fn bar_3d<UserData>(ctx: &mut GraphContext<UserData>, props: &Bar3DProps) {
    let Bar3DProps {
        x,
        y,
        width,
        height,
        depth,
        color_front,
        color_top,
        color_side,
    } = *props;

    // FRONT face (rectangle)
    // Front face of the bar (flat rectangle on screen)
    let front = RectArea::new(x, y - height, width, height + 1, Some(color_front));
    draw_rect(ctx, &front);

    // TOP face (parallelogram) drawn using horizontal lines
    for i in 0..=depth {
        let start = Point::new((x + i) as i32, (y - height - i) as i32);
        draw::line::horizontal(ctx, &start, width, Some(color_top));
    }

    // SIDE face (slanted parallelogram)
    let side = vec![
        Point::new((x + width) as i32, (y - height) as i32),
        Point::new((x + depth + width) as i32, (y - height - depth) as i32),
        Point::new((x + depth + width) as i32, (y - depth) as i32),
        Point::new((x + width) as i32, y as i32),
    ];
    closed_perimeter(ctx, &side, Some(color_side));
    flood(
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        // &(side[0] + Point::new(2, 2)).to_pixel(color_side),
        &(side[0] + Point::new(2, 0)).to_pixel(color_side),
    );
}

//---------------------------------------------------------------------
//---------------------------------------------------------------------

pub fn render_frame(ctx: &mut GraphContext<DemoUserData>) {
    // initial context setup
    if ctx.frame_count == 0 {
        ctx.win.foreground_color = RetroNeon::GLITCH_RED;

        // let mid_blue: u32 = gradient::linear_step(OceanBreeze::DEEP_SEA_BLUE, OceanBreeze::LAGOON_BLUE, 40, 22);
        let mid_blue: u32 =
            gradient::linear_step(OceanBreeze::HARBOR_NAVY, OceanBreeze::LAGOON_BLUE, 40, 28);
        ctx.win.background_color = mid_blue;
    }

    clear_screen(ctx);

    // #############################################################################################

    let mut num_cols: u32 = 8;
    let mut num_rows: u32 = 2;

    if ctx.frame_count % 1800 > 600 {
        num_cols = 16; // good!
        num_rows = 5; // good!
    }

    if ctx.frame_count % 1800 > 1200 {
        num_cols = 24;
        num_rows = 3;
    }


    let cell_width: u32 = ctx.win.w / num_cols;
    let cell_height: u32 = ctx.win.h / num_rows;
    ctx.line.set_int_no_aa(Some(2));


    let mut grid = UniformGrid::new(
        RectArea::new(0, 0, cell_width, cell_height, Some(RetroNeon::LASER_AQUA)),
        num_rows as usize,
        num_cols as usize,
        Some(vec![
            RetroNeon::CYBER_YELLOW, // RetroNeon::PSYCHEDELIC_BLUE, RetroNeon::DEEP_SPACE_BLUE, RetroNeon::ELECTRIC_BLUE,
        ]),
    );

    //==============================================================================================
    //==============================================================================================

    // let w_osc = oscillator::sine(ctx.frame_count, 0.005, cell_width, cell_width*10/8) as u32;

    // grid.resize_cells(Dimensions2d::new(w_osc, cell_height));

    // grid.resize_grid(num_rows as usize *2, num_cols as usize, None );

    let origin = grid.get_cell(0, 0).unwrap().bottom_left();

    //
    // Set up the first 3D bar
    let mut props = Bar3DProps {
        x: origin.x,
        y: origin.y,
        width: cell_width - 1,
        height: cell_height / 2,
        depth: 16,
        color_front: OceanBreeze::SEAFOAM,
        color_top: OceanBreeze::AQUAMARINE,
        color_side: OceanBreeze::LAGOON_BLUE,
    };
    //

    let height_bound = (cell_height as f32 * 0.78) as u32;

    // Generate oscillating values for animating the bar heights
    let mut osc = [
        oscillator::sine(ctx.frame_count, 0.0060, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0065, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0070, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0075, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0080, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0085, 4, height_bound) as u32,
        oscillator::sine(ctx.frame_count, 0.0090, 4, height_bound) as u32,
    ];


    // Iterate over the grid cells and draw a 3D bar per each cell
    grid.iter().enumerate().for_each(|(i,cell)| {
        let cell_point = cell.bottom_left();
        props.x = cell_point.x;
        props.y = cell_point.y;
        props.height = osc[ i % osc.len()];
        bar_3d(ctx, &props);
        osc.rotate_left(1);
    });




    // GRID RENDERING
    // grid::uniform::render(ctx, &grid, true);

    //
    // for c in 0..num_cols {
    //     bar_3d(ctx, &props);
    //     props.x += cell_width;
    // }
    // props.y += cell_height;
    // props.x = 0;
    // for c in 0..num_cols {
    //     bar_3d(ctx, &props);
    //     props.x += cell_width;
    // }

    //
    // props.x += 21;
    // props.height -= 24;
    // bar_3d(ctx, &props);
    //

    //==============================================================================================
    //==============================================================================================

    // // let neighborhood = NeighborhoodType::Circle {radius: 16};
    // // let neighborhood = NeighborhoodType::Diamond {distance: 12};
    // // let neighborhood = NeighborhoodType::Diagonal;
    // // let neighborhood = NeighborhoodType::Immediate;
    // // let neighborhood = NeighborhoodType::Orthogonal;
    // let neighborhood = NeighborhoodType::Square {distance: 4};
    //
    // let neighbors = grid.get_neighbors( 18,25, &neighborhood);
    //
    // ctx.line.set_int_no_aa(Some(1));
    //
    // neighbors.iter().for_each(|cell| {
    //
    //     // fill::flood(&mut ctx.frame_buf, &ctx.win.dimensions, &cell.cell.center().to_pixel(RetroNeon::LASER_LIME));
    //     let mut cell_area = cell.cell.rect_area();
    //     cell_area.color = Some(RetroNeon::LASER_LIME);
    //     draw::rectangle::filled(ctx, &cell_area);
    // });

    // #############################################################################################

    // println!("\n\nsurrounding: {:?}", surrounding);

    // grid.iter().for_each(|cell| {
    //
    //     let p:Point;
    //
    //
    //     match ctx.frame_count % 80 {
    //         0..=10 => { p=cell.top(); },
    //         11..=20 => { p=cell.top_right(); },
    //         21..=30 => { p=cell.right(); },
    //         31..=40 => { p=cell.bottom_right(); },
    //         41..=50 => { p=cell.bottom(); },
    //         51..=60 => { p=cell.bottom_left(); },
    //         61..=70 => { p=cell.left(); },
    //         71..=80 => { p=cell.top_left(); },
    //         // 81..=90 => { p=cell.center(); },
    //         _ =>   p=cell.center(), // defensive: % 8 guarantees 0–7
    //     }
    //
    //
    //     draw::rectangle::filled(ctx,
    //         &RectArea {
    //             top_left: Point::new(p.x , p.y ),
    //             dimensions: Dimensions2d::square(ctx.win.h/20),
    //             color: Some( alpha::set_alpha(RetroNeon::STROBE_WHITE, 190)),
    //         }
    //     )
    // });

    let is_transition = (ctx.frame_count % 1800 > 585 && ctx.frame_count % 1800 < 615)
        || (ctx.frame_count % 1800 > 1185 && ctx.frame_count % 1800 < 1215)
        || (ctx.frame_count % 1800 > 1785 || ctx.frame_count % 1800 < 15);

    if is_transition {
        glitch::horizontal_glitch(
            ctx,
            &mut HorizontalGlitchProps {
                strength: 52,
                chance: 230,
                left_right_balance: 128,
            },
            None,
        );

    }


    // let os: u32 = oscillator::sine(50 + ctx.frame_count, 0.0005, 0, 200) as u32;
    // let is_glitching = os > 195 || os < 5 || (os > 90 && os < 95) || (os > 41 && os < 45);
    // if is_glitching {
    // glitch::horizontal_glitch(
    //     ctx,
    //     &mut HorizontalGlitchProps {
    //         strength: 4,
    //         chance: 40,
    //         left_right_balance: 128,
    //     },
    //     None,
    // );}

    scanline::window(ctx, 1, 66);
}
