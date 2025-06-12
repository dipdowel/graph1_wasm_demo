use graph1::primitives::math::{Bound, Shell};
use graph1::primitives::plane::Dimensions2d;
use graph1::primitives::point::Point;

/// Configuration for rendering a graphical grid structure.
pub struct SprayDemoSceneConfig {
    pub background_color: u32,
    pub grid_colors: Shell<u32>,
    /// X and Y frequencies
    pub frequency: Point<f64>,
    pub blend: BlendSettings,
    pub outer_brush: BrushSettings,
    pub inner_brush: BrushSettings,
    pub path_bounds: PathBounds,
    /// Inner square and  outer square deltas (define their sizes)
    pub cell_deltas: Shell<i32>,
}

/// Outer and inner grid color configuration.
pub struct GridColors {
    pub outer: u32,
    pub inner: u32,
}

/// Blend settings used in color modulation or merging.
pub struct BlendSettings {
    pub frequency: usize,
    pub color: u32,
}

/// Brush configuration for rendering grid cells.
pub struct BrushSettings {
    pub dimensions: Dimensions2d<u32>, 
    pub density: u32,
    pub colors: Vec<u32>,
}

/// Bounds for rendering or animation path.
pub struct PathBounds {
    /// lower and upper bounds on X
    pub x: Bound<u32>,
    /// lower and upper bounds on Y
    pub y: Bound<u32>,
}
