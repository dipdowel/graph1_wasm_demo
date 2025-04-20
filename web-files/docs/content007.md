# Shapes & Quadrants

Graph1 provides basic 2D shape drawing capabilities. In this demo, we focus on **circles** and **quadrants**.

---

## ⚪ Circle

Circles can be animated, styled, and drawn using `draw::circle::filled`. This function is multithreaded and optimized for real-time rendering:

```rust
use graph1::draw;
use graph1::primitives::Pixel;

let center: Pixel = ctx.win.center.to_pixel(RetroNeon::STROBE_WHITE);
let radius = max_radius * 100 / 45;
draw::circle::filled(ctx, &center, radius, 4);
```

You can render multiple circles in different quadrants:

```rust
let center = ctx.win.quadrants.bottom_right.center().to_pixel(color);
draw::circle::filled(ctx, &center, r, 0);
```

Graph1 also supports stylized circles by skipping every N-th line in the fill.
The fourth parameter in `filled()` is `skip_every`, which controls this effect:
- `0` and `1`: draw all lines (solid circle)
- `2`: draw every other line
- Higher values produce sparser, stylized fills

---

## 🧭 Quadrants

Each `WindowContext` is divided into 4 logical regions:
- `top_left`
- `top_right`
- `bottom_left`
- `bottom_right`

These are accessed via `ctx.win.quadrants`, and each quadrant has helpful methods:
- `.center()` — center point of the quadrant
- `.rect_area()` — the region as a `RectArea`
- `.width()` / `.height()` — dimensions

Quadrants make it easy to animate shapes separately in different parts of the window.

---

💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_007_shapes_and_quadrants.rs)

