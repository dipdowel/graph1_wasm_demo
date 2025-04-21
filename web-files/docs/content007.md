# Circles

Graph1 provides basic 2D shape drawing capabilities. In this demo, we focus on **circles**.

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

Graph1 also supports stylizing circles by skipping every N-th line in the fill.
The fourth parameter in `draw::circle::filled` is `skip_every`, which controls this effect. E.g.
- `1`: draw all lines (solid circle)
- `2`: draw every other line
- Higher values produce a sparser fill

---

💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_007_shapes_and_quadrants.rs)

