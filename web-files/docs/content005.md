# Colors

## RGBA model + Alpha Context<br />+ Color Adapters

### 🎨 RGBA

Graph1 supports basic [alpha blending](https://en.wikipedia.org/wiki/Alpha_compositing), which is the process of combining foreground and background colors based on the transparency ([alpha channel](https://en.wiktionary.org/wiki/alpha_channel)) of the foreground. The result is a composite color.

Graph1 uses the [RGBA color model](https://en.wikipedia.org/wiki/RGBA_color_model), where each color is a `u32` representing four 8-bit channels:
- **R**ed
- **G**reen
- **B**lue
- **A**lpha (opacity: 0 = fully transparent, 255 = fully opaque)

These are stored in-memory as `0xRRGGBBAA`. This layout makes it easy to do bit-shifting, blending, and cross-platform manipulation.

---

### 🧪 Alpha Context

`AlphaContext` is the part of `GraphContext` responsible for controlling alpha blending. It has two fields:
- `enabled: bool` — toggle alpha blending on or off
- `method: AlphaMethod` — choose how to compute the blend

If `enabled` is `false`, the alpha channel is ignored and colors simply overwrite.

#### Blend Methods
```rust
#[derive(Debug, Clone, Copy)]
enum AlphaMethod {
    Int,   // default: faster, integer math
    Float, // more accurate, uses floating-point arithmetic
}
```
- **Int** — integer math, faster and usually good enough
- **Float** — uses `f32` for better visual fidelity

You can switch methods at runtime:
```rust
use graph1::core::context::alpha::AlphaMethod;
ctx.alpha.enabled = true;
ctx.alpha.method = AlphaMethod::Float;
```

#### Example
```rust
let color: u32 = 0xff_33_00_85; // semi-transparent orange

draw::rectangle::filled(ctx, &RectArea::square(0, 0, 100, Some(color)));
```

This will draw a 100x100 square with 133 alpha (out of 255), blending over the background.

---

### 🔁 Color Adapters

Sometimes the renderer you use expects a different byte layout. For example:
- `minifb` wants `0RGB`
- Browsers might prefer `ABGR`

Graph1 includes a handful of utilities for converting RGBA to common formats in:
```rust
use graph1::utils::color::adapters::*;
```

#### Convert a buffer from RGBA to ABGR
```rust
let mut dst_buf: Vec<u32> = vec![0; ctx.frame_buf.len()];
rgba_to_abgr(&mut dst_buf, &ctx.frame_buf, true).unwrap();
```

#### Convert RGBA to 0RGB
```rust
use graph1::utils::color::adapters::rgba_to_0rgb;
rgba_to_0rgb(&mut dst_buf, &ctx.frame_buf, 4, false);
```

#### Convert a single color
```rust
let abgr = rgba_color_to_abgr(0xff_00_88_cc);
let _0rgb = rgba_color_to_0rgb(0xff_00_88_cc);
```

#### AdapterStatistics (optional)
Most buffer converters can compute simple stats like:
- average red/green/blue value
- average composite color

This is useful for diagnostics or previews. Just set `stats = true`.

---

📦 There are both safe and unsafe versions of each adapter, for cases where you need max speed and can guarantee buffer correctness. They follow the same function signature.<br /><br />
🧠 Don’t see your format? You can easily create your own adapter — check the `adapters/` module source. Contributions are welcome!

- - - -
💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_005_alpha.rs)

