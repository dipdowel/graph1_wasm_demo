# Bézier curves

`Graph1` supports rendering [Bézier curves](https://en.wikipedia.org/wiki/B%C3%A9zier_curve), including visualization of control points and tangents for interactive and animated curve demos.

---

## 🎨 Bézier Curves

Graph1 provides full support for drawing Bézier curves with the `draw::curve::bezier()` function. These curves are built from sequences of `BezierSegment` structs, each specifying a start and end point, two control points, and a color:

```rust
use graph1::draw::curve::bezier;
use graph1::draw::curve::bezier_segment::BezierSegment;
use graph1::primitives::point::Point;

let seg = BezierSegment::new(
    Point::new(30, 90),
    Point::new(290, 90),
    Point::new(90, 10),
    Point::new(220, 160),
    0xff00ffff, // cyan
);
bezier(&mut ctx, &[seg], 0.04);
```

---

## 🛠️ BezierContext Options

Control how Bézier curves render by adjusting `ctx.bezier`, the context settings for curve drawing:

* `enabled` — toggle curve rendering
* `render_controls` — show start/end points and control points
* `render_levers` — show lines ("levers") between control points
* `control_color` — optional override for control point color
* `start_end_points_color` — optional override for main points

```rust
ctx.bezier.render_controls = true;
ctx.bezier.render_levers = true;
ctx.bezier.control_color = Some(0xffff99ff); // light pink
```

If no colors are specified, Graph1 inverts pixel colors to emphasize control points against any background.

---

## 🔁 Animating Curves

The demo animates multiple Bézier segments over time, adjusting their control points on a sine wave to create smooth, dynamic motion. Each curve's control point is perturbed using the current `ctx.frame_count`, enabling continuous animation at a fixed update rate.

---

## ⚡ Performance Tips

* `resolution_delta` controls the sampling density of the curve. Smaller values yield smoother curves but require more processing.
* Curves are rendered via `draw::line::between_two_points()` calls, so the settings of `LineContext` apply (anti-aliasing, width, etc.).

---

💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_011_curves.rs)
