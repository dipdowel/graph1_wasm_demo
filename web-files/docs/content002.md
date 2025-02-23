# Basic concepts, pt.2

## Primitives
Module `primitives` contains building blocks for describing positions, areas, bounding boxes, and other logical __primitives__ used in computer graphics.   

### struct `Point`
Struct `Point` represents a point on a plane with coordinates `x` and `y`. The coordinates can be either integers or floating-point numbers, constrained by trait `Numeric`.

### trait `Numeric`
Trait `Numeric` provides a convenient conversion between `u32`, `i32`, `f32` and `f64`, which comes in handy when working with points, areas, and other primitives. Coordinates of a pixel on the screen are always positive integers (`u32`), however, many graphical calculations require negative and/or floating-point numbers (`i32`, `f32`, `f64`). The next section illustrates how trait `Numeric` works with `Point<T: Numeric = u32>`.

### `Point` + `Numeric`
When points are used in calculations, floating point format of the coordinates is often required for more accurate results. When it comes to rendering, the coordinates are rounded to integers. 

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

### struct `Pixel`
Struct `Pixel` is similar to `Point`, but its `x` and `y` coordinates are always `u32`. It also has `color` property, 
also a `u32`. 

// TODO: write about conversion from `Pixel` to `Point` and back

### struct  `Dimensions2d`

### struct  `RectArea`


