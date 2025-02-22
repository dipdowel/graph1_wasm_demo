# Basic concepts, pt.2

## Primitives
Module `primitives` contains building blocks for describing positions, areas, bounding boxes, and other logical __primitives__ used in computer graphics.   

### struct `Point`
Represents a point on a plane with coordinates `x` and `y`. The coordinates can be integer or floating-point numbers, defined through the `Numeric` trait.

### trait `Numeric`
Trait `Numeric` provides a convenient conversion between `u32`, `i32`, `f32` and `f64`, which comes in handy when working with points, areas, and other primitives. Coordinates of a pixel on the screen are always positive integers (`u32`), however, many graphical calculations require floating-point and/or negative numbers (`i32`, `f32`, `f64`). The next section illustrates how `Numeric` works with `Point<T: Numeric = u32>`.

### `Point` + `Numeric`
When points are used in calculations, floating point precision is often required for more accurate results. When it comes to rendering, the coordinates are rounded to integers. 

```rust
let x: f64 = -10.0123456789;
let y: f64 = 20.987654321;

// Define a point with coordinates of f64 type 
let point_f64: Point<f64> = Point::new(x, y);

// Conversion to `Point<f32>` results in loss of precision per coordinate.
let point_f32: Point<f32> = point_f64.convert();

// Conversion to `Point<i32>` truncates the floating-point part
// NB: `20.987654` is rounded to `21`, `-10.012345` is rounded to `-10`
let point_i32: Point<i32> = point_f32.convert();

// Conversion to `Point<u32>` truncates negative coordinates to zero  
let point_u32: Point<u32> = point_i32.convert();

// Any `Numeric` type can be converted to any other `Numeric` type
let another_point_u32: Point<u32> = point_f64.convert();

println!(
    "point_f64: {:?}\n\
        point_f32: {:?}\n\
        point_i32: {:?}\n\
        point_u32: {:?}\n\
        another_point_u32: {:?}",
    point_f64, point_f32, point_i32, point_u32, another_point_u32
);
```


Output: 
```text
point_f64: Point { x: -10.0123456789, y: 20.987654321 }
point_f32: Point { x: -10.012345, y: 20.987654 }
point_i32: Point { x: -10, y: 21 }
point_u32: Point { x: 0, y: 21 }
another_point_u32: Point { x: 0, y: 21 }
```




## Graph Context

The `GraphContext` struct is essential for all the drawing operations. 
- Let's look at the minimal setup to get a working `GraphContext` instance.
- Also, we'll draw two filled rectangles on the screen.
 
```rust
use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::palettes::RetroNeon;

const WIN_WIDTH: u32 = 480;
const WIN_HEIGHT: u32 = 240;

fn main() {  
  // The window context
  let win_ctx = WindowContext::new(
    WIN_WIDTH,
    WIN_HEIGHT,
    Some(RetroNeon::CYBER_BLUE),
    Some(RetroNeon::LASER_LIME),
  );

  // Disable alpha blending when you don't need it. It improves performance.
  let use_alpha = false;

  // Disabled the draft buffer when you don't need it. It saves some memory.
  let use_draft_buf = false;

  // No user data needed for this demo
  let user_data = None;

  // Graph context
  let mut ctx: GraphContext =
          GraphContext::new(win_ctx, use_alpha, use_draft_buf, user_data);
    
  // rectangles have the same width  
  let width = ctx.win.w - 40;
    
  // Rectangle #1: 
  //  x = 20, y = 20, width = 440, height = 20, 
  // `None` =  the default window foreground color
  let rect_1 = RectArea::new(20, 20, width, 20, None);

  // Rectangle #2:
  // x = 20, y = 200, width = 440, height = 20, 
  // vibrant cyan color 
  let color_2 = Some(RetroNeon::VIBRANT_CYAN);
  let rect_2 = RectArea::new(20, ctx.win.h - 40, width, 20, color_2);

  // Draw both rectangles on the screen
  draw::rectangle::filled(&mut ctx, &rect_1);
  draw::rectangle::filled(&mut ctx, &rect_2);

  // The code that displays the text 
  // and renders the screen was omitted for brevity    
    
  
}

```
We'll cover what `user_data = None` means in the [next section](http://localhost:8080/?demo=3).

- - - - 
Also, see:
- [Alpha blending](http://localhost:8080/?demo=999) for details on `use_alpha = false`
- [Draft buffer](http://localhost:8080/?demo=999) for details on `use_draft_buf = false`

<br /><br />
- - - - 
- [Code of this demo on Github](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_002_basic_concepts_pt2.rs)
