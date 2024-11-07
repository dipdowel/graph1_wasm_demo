# Basic concepts pt.1
## The contexts
Struct `GraphContext` is the heart of Graph1. It contains all the core properties that allow to draw on the screen, keep track of the animation, etc. <br />
`GraphContext`  references some 'sub-contexts', such as:
- `WindowContext` -- all the window-related properties 
- `BezierContext` -- properties for Bézier curves

## The Window Context

It all begins with the `WindowContext` struct. It represents the window where all the graphics will appear. 
- `Q:` But why "window"? Why not "canvas" or "stage"? 
- `A:` Well, Graph1 was created with native applications in mind,
  so `WindowContext`  matches the concept of window in [minifb](https://github.com/emoon/rust_minifb) and other similar libraries well.



#### A default window
```rust
use graph1::core::context::WindowContext;
use graph1::core::default_colors;

// The simplest way to create a window context with default settings
let win_ctx = WindowContext::default();
```

#### A 320 x 240 window
```rust
// Under the hood, calling `WindowContext::default()` is equivalent to:
let win_ctx = WindowContext::new( 
                                  320, 240,  // width, height
          Some(default_colors::BACKGROUND),  // background color 
          Some(default_colors::FOREGROUND)   // foreground color
);
```
#### Convenience properties

```rust
// The window's width and height are exposed as `u32`, `usize`, and `i32` 
// as these types are often used in various calculations and operations.

println!("Width as u32{}", win_ctx.w); // 320
println!("Width as usize{}", win_ctx.w_usize); // 320
println!("Width as i32{}", win_ctx.w_i32); // 320

println!("Window dimensions as `Dimensions2d`{:?}", win_ctx.dimensions);
// Dimensions2d { w: 320, h: 240 }
```
Graph1 is relatively low-level, so it's expected that you know what you're doing and that you won't overwrite, say, `win_ctx.w_i32` with a value that's not a valid window width. Introducing getters could make it safer, but it would also slow things down if window properties are accessed in a tight loop.

### The Graph Context

<!--
The `GraphContext` struct is the main entry point for all drawing operations. It is created by calling the `GraphContext::new()` function. 
```rust 
necessary information about the window, the canvas, and the drawing state. The `GraphContext` is the main entry point for all drawing operations. It is created by calling the `GraphContext::new()` function. 
```rust
use graph1::core::context::{GraphContext, WindowContext};
```


### NB: Why window context and not canvas or stage or something like that?
Because Graph1 was created with a focus on native applications, where the window is the main drawing area. 
 

It represents a 2D drawing area where you can draw shapes, images, and text. The `Canvas` is a pixel buffer that you can draw on using various methods. 
`Canvas` struct. It represents a 2D drawing area where you can draw shapes, images, and text. The `Canvas` is a pixel buffer that you can draw on using various methods.
-->