# Polygons, stars, gradients
  
Module `draw::polygons` contains functions for drawing polygons and stars. 
Each property can be animated, just like in the demo above!<br />
Module `utils::color::gradient` contains functions for creating gradients between colors.

## Polygons
Here's the minimal setup to draw a pentagon:

```rust
    let polygon_props = PolygonProperties {
        center: Pixel {
            color: RetroNeon::MAGENTA_GLOW,
            x: 100,
            y: 100,
        },
        // `3` for a triangle, `4` for a square, `5` for a pentagon, etc.
        num_sides: 5,
        // how far from the center the polygon's vertices are
        radius: 70,        
        // Rotation angle of the polygon, in degrees.
        rotation_angle: 0.0,
        skip_rendering: false,
    };
    polygons::polygon(ctx, &polygon_props);
```


## Star
Here's the minimal setup to draw a penta-star:
```rust
    let star_props = StarProperties {
        center: Pixel {
            color: RetroNeon::HOT_PINK,
            x: 100,
            y: 100,
        },
        num_rays: 5,
        // how far from the center the rays start
        inner_radius: 12,
        // how far from the center the rays reach
        outer_radius: 30,
        // Rotation angle of the star, in degrees.
        rotation_angle: 0.0,
        skip_rendering: false,
    };
    polygons::star(ctx, &star_props);
```
<br />
Both `polygons::polygon()` and `polygons::star()` return vectors of points representing their vertices. This can be useful for further calculations or for creating custom shapes, based on the coordinates of the vertices.

## Gradient
`utils::color::gradient` allows building gradients between colors. 

### Linear
Here's how to create a linear gradient from `RetroNeon::DARK_PURPLE` to `RetroNeon::NEON_PINK` with 256 steps:
```rust
let gradient:Vec<u32> = 
    gradient::linear(RetroNeon::DARK_PURPLE, RetroNeon::NEON_PINK, 256);
```

### Linear, single step
Sometimes you need just one color from the gradient. For example, you want a color that sits in between white and pink. If we have a gradient with 256 steps, we should take the color at step 128. By sliding the `step`, we can get a color closer to white or NEON_PINK.


```rust
let step:u32 = 128;
let mid_color:u32 = 
    gradient::linear_step(0xff_ff_ff_ff, RetroNeon::NEON_PINK, 256, step);
```
<br />
- - - - 
- [Code of this demo on Github](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_008_polygons.rs)

