# Lines

There are some settings in `LineContext` that allow configuring rendering of lines via `draw::line::between_two_points()` and similar functions. As of April 2025, line rendering is in an MVP state — functional, flexible, but still being refined.

---

## ✏️ Minimal Example

```rust
use graph1::draw::line;
use graph1::primitives::point::Point;

let start = Point::new(50, 50);
let end = Point::new(200, 120);
line::between_two_points(&mut ctx, &start, &end, Some(0xff00ffff));
```

This will render a simple cyan line using the current settings in your `GraphContext.line`.

---

## 🛠️ LineContext Overview

All line rendering is controlled by `ctx.line`, which is a `LineContext` struct with the following capabilities:

- **Thickness**
    - `width_int`: integer pixel width
    - `width_float`: subpixel width for float rasterization
- **Anti-Aliasing**
    - Enable with `ctx.line.anti_aliasing.enabled = true;`
    - Method: 
      - `Int` - fixed-point Wu 
      - `Float` - high-quality Wu
      - **Wu** algorithm is mentioned below in **Rendering Details**.
- **Rasterization**
    - `Int`: uses Bresenham or integer geometry
    - `Float`: smooth, subpixel-accurate rendering
- **Clipping** (optional endpoint clipping)
    - `ElasticSlide`, `CohenSutherland`, or `LiangBarsky`

There are multiple helpers in `LineContext` for quick selection of line settings. Here are two of them for example:
```rust
// integer rasterization, no anti-aliasing
ctx.line.set_int_no_aa(Some(1)); 

// float rasterization, float anti-aliasing, 2.5px width
ctx.line.set_float_aa_float(Some(2.5)); 
```

---

## 📏 Rendering algorithms

Here’s a quick overview of the algorithms used in line rendering and clipping:

| Algorithm | Description |
|----------|-------------|
| [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm) | Efficient integer-based method for drawing straight lines. |
| [Xiaolin Wu's line algorithm](https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm) | Anti-aliased line rendering with smooth edges. |
| [Cohen–Sutherland algorithm](https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm) | Fast line clipping against a rectangular viewport. |
| [Liang–Barsky algorithm](https://en.wikipedia.org/wiki/Liang%E2%80%93Barsky_algorithm) | Mathematically precise line clipping using parametric equations. |
| **ElasticSlide** | A custom method that makes clipped lines appear to “slide” or stretch elastically toward the clipping boundary, creating a visually continuous and dynamic effect. |

### Some extra details:
- If line width is 1, anti-aliasing is off, and rasterization is integer → falls back to fast Bresenham.
- Horizontal/vertical lines are optimized.
- Float mode allows subpixel thickness like `0.5` or `2.75`.
- Anti-aliasing darkens edge pixels smoothly (via [Xiaolin Wu](https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm) algorithm).

---

💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_009_lines.rs)

