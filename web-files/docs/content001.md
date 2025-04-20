# Basic Concepts, Pt. 1
## The Contexts

At the heart of Graph1 is the `GraphContext` — a struct that manages everything needed to draw on screen, animate, and apply effects.

It includes several subcontexts:

- `WindowContext` — window dimensions, background/foreground colors
- `LineContext` — line rendering options (e.g. anti-aliasing, width)
- `AlphaContext` — alpha blending settings
- `BezierContext` — Bézier curve rendering settings

We’ll focus on the essential part first: `WindowContext`.

---

## WindowContext: The Drawing Surface

Despite the name "window," it’s not tied to any GUI library. It simply represents the surface where all drawing happens — inspired by concepts from libraries like [minifb](https://github.com/emoon/rust_minifb).

### 🟦 Create a Default Window

```rust
use graph1::core::context::WindowContext;

let win_ctx = WindowContext::default();
```

Equivalent to:

```rust
use graph1::core::default_colors;

let win_ctx = WindowContext::new(
320, 240,                        // width, height
Some(default_colors::BACKGROUND),
Some(default_colors::FOREGROUND),
);
```

---

### 📊 Dimensions and Properties

```rust
println!("Width (u32): {}", win_ctx.w);         // 320
println!("Width (usize): {}", win_ctx.w_usize); // 320
println!("Width (i32): {}", win_ctx.w_i32);     // 320

println!("Dimensions: {:?}", win_ctx.dimensions);
// Output: Dimensions2d { w: 320, h: 240 }
```

Graph1 gives you multiple numeric types (`u32`, `usize`, `i32`) for width/height to simplify common calculations. <br /><br />
> ⚠️ Note: This is a low-level library — you are expected to avoid writing invalid values (e.g. setting `win_ctx.w_i32 = -640`). No enforced safety via getters — just raw access and trust.

---

### 📐 Resizing and Bounds Checking

You can resize the window at any time:

```rust
win_ctx.resize(640, 480);
```

This updates all related properties automatically — such as the window center, region, and quadrants (more on that below).

To check if something fits within the window:

```rust
use graph1::primitives::plane::RectArea;

let rect = RectArea::new(50, 50, 100, 100, None);
if win_ctx.contains(&rect) {
// safe to draw!
}
```

---

### 🧭 Window Quadrants

Each window is automatically split into four logical subregions — or quadrants — which can be useful for spatial partitioning, effects, or organizing your layout.

> A detailed explanation of `Quadrants` is covered in a dedicated section.

---

### 🧠 Tip: Explore the Source

Graph1 is open-source and well-documented. Core types like `WindowContext` and `GraphContext` expose their functionality clearly in code. For example:

- `resize()` — updates all window dimensions and regions
- `contains()` — checks if a rectangle fits in the window

Feel free to dig into the source — it's designed to be readable and educational.

---

### ⏭️ Up Next: `GraphContext`

In the next section, we’ll look at `GraphContext`, which ties everything together — including frame buffers, animation control, multithreading, and more.

