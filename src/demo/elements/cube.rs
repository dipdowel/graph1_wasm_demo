use crate::utils::console_log;
use graph1::core::context::GraphContext;
use graph1::draw::line;
use graph1::primitives::point::{Point, Point3D};
use graph1::primitives::Pixel;
use graph1::utils::math::oscillator;
use std::f64::consts::PI;

#[rustfmt::skip]
static EDGES: [[usize; 2]; 12] = [
[0, 1], [1, 2], [2, 3], [3, 0], // back face
[4, 5], [5, 6], [6, 7], [7, 4], // front face
[0, 4], [1, 5], [2, 6], [3, 7] // connecting sides
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

    /// Start point of an edge + color
    cube_pixel: Pixel,
    /// End point of an edge
    cube_point: Point,
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

            // Default initial values
            cube_pixel: Pixel { x: 0, y: 0, color },

            // Default initial values
            cube_point: Point { x: 0, y: 0 },

            // size,
            vertices: Vec::new(),
        };

        let Cube { cx, cy, cz, .. } = cube;

        // TODO: If we want to change the size during rendering (in runtime),
        // TODO: `vertices` will have to be recalculated.
        // TODO: Should the code below go into a separate function
        // TODO: that can be called also from `render()`?

        #[rustfmt::skip]
            let  vertices: Vec<Point3D<f64>> = vec![
            Point3D { x: cx - size, y: cy - size, z: cz - size },
            Point3D { x: cx + size, y: cy - size, z: cz - size },
            Point3D { x: cx + size, y: cy + size, z: cz - size },
            Point3D { x: cx - size, y: cy + size, z: cz - size },
            Point3D { x: cx - size, y: cy - size, z: cz + size },
            Point3D { x: cx + size, y: cy - size, z: cz + size },
            Point3D { x: cx + size, y: cy + size, z: cz + size },
            Point3D { x: cx - size, y: cy + size, z: cz + size },
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
        let angle = rotation * 0.001 * self.speed_z * PI * 2_f64;        
        let mut cx = self.cx; /* + oscillator; */
        let mut cy = self.cy; /* + oscillator; */
        let mut cz = self.cz; /*+ oscillator as f64; */

        // apply translations only if provided.
        match translation {
            Some(delta) => {
                cx = cx + delta.x;
                cy = cy + delta.y;
                cz = cz + delta.z;
            }
            None => (),
        }

        for v in &mut self.vertices {
            let dx: f64 = v.x - cx;
            let dy = v.y - cy;
            let x = dx * f64::cos(angle) - dy * f64::sin(angle);
            let y = dx * f64::sin(angle) + dy * f64::cos(angle);
            v.x = x + cx;
            v.y = y + cy;
        }

        // rotate the cube along the X axis
        let angle = rotation * 0.001 * self.speed_x * PI * 2_f64;        
        for v in &mut self.vertices {
            let dy = v.y - cy;
            let dz = v.z - cz;
            let y = dy * f64::cos(angle) - dz * f64::sin(angle);
            let z = dy * f64::sin(angle) + dz * f64::cos(angle);
            v.y = y + cy;
            v.z = z + cz;
        }

        // rotate the cube along the Y axis
        let angle = rotation * 0.001 * self.speed_y * PI * 2_f64;        
        for v in &mut self.vertices {
            let dx = v.x - cx;
            let dz = v.z - cz;
            let x = dz * f64::sin(angle) + dx * f64::cos(angle);
            let z = dz * f64::cos(angle) - dx * f64::sin(angle);
            v.x = x + cx;
            v.z = z + cz;
        }

        // draw each edge
        for edge in EDGES {
            self.cube_pixel.x = self.vertices[edge[0]].x as u32;
            self.cube_pixel.y = self.vertices[edge[0]].y as u32;
            self.cube_point.x = self.vertices[edge[1]].x as u32;
            self.cube_point.y = self.vertices[edge[1]].y as u32;
            // println!("cube_pixel: {:?}", cube_pixel);
            // println!("cube_point: {:?}", cube_point);

            line::between_two_points(ctx, &self.cube_pixel, &self.cube_point);
        }
    }
}
