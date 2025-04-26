# Basic Concepts, Pt. 2
## Primitives — the Building Blocks

Before we dive into drawing, animation, and effects, let’s get familiar with the core types that make everything tick: **primitives**.

These are the foundational types in Graph1 used to describe positions, areas, rectangles, ratios, and more. They're lean, flexible, and (mostly) won't yell at you. You’ll find them all under the `graph1::primitives` module.

---

### 📍 `Point<T>`

A generic 2D point. That's it. No surprises here. Just an `(x, y)` coordinate pair that plays nice with any type that implements `Numeric` — like `u32`, `i32`, `f32`, or `f64`.

```rust
let pt = Point::<f64>::new(12.5, -4.0);
```

Need it as an integer? No sweat:
```rust
let pt_int: Point<i32> = pt.convert();
```

Or turn it into a `Pixel` with a color:
```rust
let pix = pt.to_pixel(0xff_00_ff_00);
```

---

### 🧠 `Numeric`

This internal trait powers all numeric operations in Graph1. It lets you write generic code without tying yourself to a specific number type. You get conversions (`to_f64`, `to_u32`, etc.), arithmetic traits, and goodies like `.is_zero()` and `.get_type()`.

It’s not meant to be used directly — more like a loyal behind-the-scenes helper.

---

### 🧊 `Pixel`

A `Pixel` is just a colored point on the screen:
```rust
let pixel = Pixel { x: 100, y: 200, color: 0xff_ff_00_00 }; // Red pixel
```

Pixels convert *from* `Point`s and *into* them too — across all the usual numeric types.

---

### 📏 `Dimensions2d<T>`

Width and height bundled together. Used everywhere — windows, areas, boxes, sandwiches (ok, maybe not that one):
```rust
let dims = Dimensions2d::new(640, 480);
```

Also supports conversion, because everything in Graph1 is just a few `from_f64()`s away.

---

### 🧱 `RectArea<T>`

Think of it like a rectangle with a top-left `Point`, some `Dimensions2d`, and maybe a color.
```rust
let rect = RectArea::new(0, 0, 100, 50, Some(0xff00ff00));
```

Handy methods include:
- `.contains_point(&Point)` — Is that point inside this box?
- `.contains(&RectArea)` — Is this rectangle entirely within another?
- `.is_line_segment_outside(start, end)` — A fast rejection test for lines

---

### 🔍 `Containable`

This trait lets you write logic like:
```rust
if my_point.is_contained_in(&my_rect) { /* do something */ }
```
Implemented for `Point`, `Pixel`, and `RectArea`. Easy.

---

### 📐 `Ratio<T>`

For when you want to say "4:3" like a pro:
```rust
let r = Ratio::new(4, 3)?;
let (w, h) = r.to_dimensions(300); // => (400, 300)
```

You can simplify, invert, or check if another width/height pair matches your ratio.

---

### 🧮 Odds & Ends
- `MinMax<T>` — holds a min and max value
- `Range<T>` — start and end values
- `ColorPair` — tuple of two colors, e.g. for gradients
- `BufferRGBA` — alias for a pixel buffer: `&mut [u32]`
- `PixelColorTransformerFn` — callback type for pixel shaders

---

And that’s the primitive tour! 🎉

These types might look humble, but they power almost everything in Graph1 — from layout and math to drawing and effects.

Next up: we put these to work.

