use crate::utils::console_log;
use graph1::core::context::GraphContext;
use graph1::draw::line;
use graph1::primitives::point::{Point, Point3D};
use graph1::utils::math::oscillator;
use std::f64::consts::PI;

#[rustfmt::skip]
static EDGES: [[usize; 2]; 12] = [
    [0, 1], [1, 2], [2, 3], [3, 0], // back face
    [4, 5], [5, 6], [6, 7], [7, 4], // front face
    [0, 4], [1, 5], [2, 6], [3, 7]  // connecting sides
];

pub struct Cube {
    #[rustfmt::skip]
    /// X-rotation speed (RPS)
    speed_x: f64,
    /// Y-rotation speed (RPS)
    speed_y: f64,
    /// Z-rotation speed (RPS)
    speed_z: f64,
    /// X-center (the location of the cube on the X axis)
    cx: f64,
    /// Y-center (the location of the cube on the Y axis)
    cy: f64,
    /// Z-center (the location of the cube on the Z axis)
    cz: f64,
    /// Length of the cube's edge
    // size: f64,
    /// vertices
    vertices: Vec<Point3D<f64>>,

    color: u32,
}

impl Cube {
    pub fn new(speed: Point3D<f64>, center: Point3D<f64>, size: f64, color: u32) -> Cube {
        let mut cube = Cube {
            speed_x: speed.x,
            speed_y: speed.y,
            speed_z: speed.z,
            cx: center.x,
            cy: center.y,
            cz: center.z,
            vertices: Vec::new(),
            color,
        };

        let Cube { cx, cy, cz, .. } = cube;

        // TODO: If we want to change the size during rendering (in runtime),
        // TODO: `vertices` will have to be recalculated.
        // TODO: Should the code below go into a separate function
        // TODO: that can be called also from `render()`?

        #[rustfmt::skip]
        let vertices = vec![
            Point3D::new(cx - size, cy - size, cz - size),
            Point3D::new(cx + size, cy - size, cz - size),
            Point3D::new(cx + size, cy + size, cz - size),
            Point3D::new(cx - size, cy + size, cz - size),
            Point3D::new(cx - size, cy - size, cz + size),
            Point3D::new(cx + size, cy - size, cz + size),
            Point3D::new(cx + size, cy + size, cz + size),
            Point3D::new(cx - size, cy + size, cz + size),
        ];

        cube.vertices = vertices;
        cube
    }

    pub fn render<DemoUserData>(
        &mut self,
        ctx: &mut GraphContext<DemoUserData>,
        translation: Option<&Point3D<f64>>,
    ) {
        let rotation = oscillator::sine(ctx.frame_count as f64, 0.0003789, -1008.0, 1021.0);

        // rotate the cube along the Z axis
        let angle_z = rotation * 0.001 * self.speed_z * PI * 2.0;
        let angle_x = rotation * 0.001 * self.speed_x * PI * 2.0;
        let angle_y = rotation * 0.001 * self.speed_y * PI * 2.0;

        let (mut cx, mut cy, mut cz) = (self.cx, self.cy, self.cz);
        if let Some(delta) = translation {
            cx += delta.x;
            cy += delta.y;
            cz += delta.z;
        }

        // Z-rotation
        for v in &mut self.vertices {
            let dx = v.x - cx;
            let dy = v.y - cy;
            let x = dx * f64::cos(angle_z) - dy * f64::sin(angle_z);
            let y = dx * f64::sin(angle_z) + dy * f64::cos(angle_z);
            v.x = x + cx;
            v.y = y + cy;
        }

        // X-rotation
        for v in &mut self.vertices {
            let dy = v.y - cy;
            let dz = v.z - cz;
            let y = dy * f64::cos(angle_x) - dz * f64::sin(angle_x);
            let z = dy * f64::sin(angle_x) + dz * f64::cos(angle_x);
            v.y = y + cy;
            v.z = z + cz;
        }

        // Y-rotation
        for v in &mut self.vertices {
            let dx = v.x - cx;
            let dz = v.z - cz;
            let x = dz * f64::sin(angle_y) + dx * f64::cos(angle_y);
            let z = dz * f64::cos(angle_y) - dx * f64::sin(angle_y);
            v.x = x + cx;
            v.z = z + cz;
        }

        // Draw each edge of the cube
        for edge in EDGES {
            let v0 = &self.vertices[edge[0]];
            let v1 = &self.vertices[edge[1]];

            // Project to 2D space (simple orthographic projection)
            let p0 = Point::new(v0.x.round() as i32, v0.y.round() as i32);
            let p1 = Point::new(v1.x.round() as i32, v1.y.round() as i32);

            line::between_two_points(ctx, &p0, &p1, Some(self.color));
        }
    }
}
