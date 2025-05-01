# Basic Concepts, Pt.&nbsp;3

## Drawing Rectangles, Blending, and Going Retro

Alright, time to actually draw something! Rectangles are a great place to start — they're everywhere. Menus, tiles, backgrounds, fancy shader demos — it’s rectangles all the way down.

<br />
> ⚠️ **Note:** As of April 2025, multithreading works only in native builds. WebAssembly (WASM) builds still run single-threaded due to current browser and tooling limitations.

---

### 🧠 `GraphContext` — The Rendering Brain

This is the central struct that handles drawing, animation state, frame buffers, and more.

```rust
let mut ctx = GraphContext::new(
        WindowContext::default(), // your window setup
        true,                     // use alpha blending
        false,                    // don't use draft buffer
        None,                     // no custom user data
        4,                        // use 4 threads
        None                      // default line context
);
```

You’ll use `ctx` everywhere. It owns the frame buffer (`frame_buf`), handles blending, and manages per-frame animation data. You can also resize it later with `.resize(w, h)`.

---

### 🟥 Drawing a Filled Rectangle

To draw a filled rectangle, use:

```rust
use graph1::core::context::WindowContext;
use graph1::core::context::GraphContext;
use graph1::draw::rectangle;
use graph1::utils::color::palettes::RetroNeon;
use graph1::primitives::plane::RectArea;

let mut ctx = GraphContext::new(
        WindowContext::new(
                320,
                240,
                Some(0xff_00_00_00), // background color (black with full alpha)
                Some(0xff_ff_ff_ff), // foreground color (white with full alpha)
        ),
        true,   // enable alpha blending
        false,  // don't use draft buffer
        None,   // no user data
        4,      // number of threads
        None    // default line context
);

// Create a rectangle at (32, 32) with width=64 and height=64, color hot pink
let rect = RectArea::new(32, 32, 64, 64, Some(RetroNeon::HOT_PINK));

// Draw it
rectangle::filled(&mut ctx, &rect);
```

This will:

- Fill the specified area with the given color
- Use alpha blending if `ctx.alpha.enabled` is true
- Parallelize the work across `ctx.num_threads` (if >1 and worth it)


---

### 🌀 Alpha Blending: A Quick Note

If `ctx.alpha.enabled` is true, Graph1 uses the configured method:

- `AlphaMethod::Int` — faster, integer math (default)
- `AlphaMethod::Float` — smoother but more expensive

If disabled, pixels are simply overwritten.

You can switch methods manually:

```rust
ctx.alpha.method = AlphaMethod::Float;
```

---

### 🌈 Bonus: Color Palettes

Graph1 ships with a tasty set of color palettes for any mood or aesthetic — not just the neon variety.

Here are a few you can explore:

- `RetroNeon` — synthwave, vaporwave, and beyond
- `AutumnHarvest` — warm, earthy tones
- `OceanBreeze` — cool, aquatic colors
- `DesertDusk` — sandy sunsets and purple horizons
- `ForestMist` — lush greens and woodland neutrals
- `UrbanConcrete` — grayscale and industrial vibes
- `TropicalParadise` — bright, beachy hues
- `VintagePastel` — soft, faded nostalgia
- `WinterFrost` — icy and serene

Each palette is a simple struct with a bunch of named `u32` color constants, ready to use:

```rust
use graph1::utils::color::palettes::AutumnHarvest;
let leaf = AutumnHarvest::BURNT_ORANGE;
```

<br />
> 🎨 Want to browse all palettes visually? There's a dedicated palette explorer web tool. Just open [src/utils/color/palettes.html](/palettes.html) in your browser. It lets you preview each palette and even copy color constants to your clipboard by clicking on them!

<br />
- - - - 
💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_003_basic_concepts_pt3.rs)

