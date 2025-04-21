# Quadrants

Graph1 provides a predefined division of the **window** into four regions called **quadrants**. This helps organize content, manage layout, and snap lines or shapes to some common points on the screen.

The quadrants are powered by the `Region<T>` structure, which exposes commonly used reference points (center, edges, corners) and basic geometry like width, height, and area.

---

## 🧭 What Are Quadrants?

Accessible through `ctx.win.quadrants`, the quadrants divide the window into:
- `top_left`
- `top_right`
- `bottom_left`
- `bottom_right`

Each of these quadrants is a `Region<u32>` that provides geometry helpers like:
- `.center()` — center point of the quadrant
- `.top()` / `.right()` / `.bottom()` / `.left()`
- `.top_left()` / `.top_right()` / `.bottom_left()` / `.bottom_right()`
- `.width()` / `.height()` / `.area()`
- `.rect_area()` — the underlying `RectArea`

You can convert a quadrant’s coordinates to a different numeric type with `.convert::<i32>()`.

---

## 📐 How Are They Constructed?

Internally, Graph1 splits the window:
- Horizontally: top vs bottom halves
- Vertically: left vs right halves

This gives you 4 equally sized quadrants, automatically resized whenever the window size changes. The implementation ensures:
- All quadrants are tightly packed (no gaps)
- They always cover the entire screen, even if dimensions are odd

---

## 📌 Quadrants vs Region

Both `ctx.win.quadrants` and `ctx.win.region` help you reason about layout:
- `ctx.win.quadrants` gives you **4 precomputed** regions, each  representing 1/4 of the screen
- `ctx.win.region` gives you the **entire window** as a `Region`, with edge midpoints, corners, and the center. 

You can use `ctx.win.region` when you want to:
- Draw across the full window
- Access midpoints of the top, bottom, left, and right edges of the window
- Access corners of the window
- Use `.get_points()` to get all 8 compass points (+center, optionally)

Use `ctx.win.quadrants` when you want to:
- access the same points as `ctx.win.region`, but per 1/4 of the window

Both share the same `Region` API!

---

## 🎨 What You Can Do with Them

Quadrants are great for:
- Layouts: position different elements in each corner
- Animations: snap motion or transitions to predefined points
- Geometry: draw diagonals, align shapes, reflect objects horizontally/vertically

Example use:
```rust
let c = ctx.win.quadrants.top_right.center();
draw::circle::filled(ctx, &c.to_pixel(color), 40, 0);
```

---
> ⚠️ Note: Quadrants are basically a simple version of a grid with the size of 2x2. 
> * See `grid` for more details (`TODO: implement the grid, write a doc`).

---

## 💎 In Action: The Gemstone Demo

In the Quadrants demo, the midpoints of each window side are connected to draw a gemstone:
```rust
let gem = vec![ctx.win.region.top(), ctx.win.region.right(), ...];
```

Then, diagonal lines are drawn between opposite quadrants:
```rust
let q = ctx.win.quadrants.convert();
line::between_two_points(ctx, &q.top_left.center(), &q.bottom_right.center(), color);
```

The shape is then filled with color using `fill::flood()`, and animated with palette gradients. This demonstrates how quadrant geometry and region points can be combined with line drawing and filling.

---

💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_010_quadrants.rs)

