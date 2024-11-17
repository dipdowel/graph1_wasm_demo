# RGBA model + Alpha Context + Color Adapters

## RGBA
Graph1 supports basic [alpha blending](https://en.wikipedia.org/wiki/Alpha_compositing), which is the process of combining a foreground and background colors based on the transparency ([alpha channel](https://en.wiktionary.org/wiki/alpha_channel)) of the foreground, creating a composite color. <br /><br />

Graph1 internally uses the [RGBA color model](https://en.wikipedia.org/wiki/RGBA_color_model), where each color is composed of four channels: red, green, blue, and alpha. Each channel is 8 bits in size (ranging from 0 to 255), making the complete color a 32-bit integer. In Graph1, the `u32` type is almost always used to represent colors. The alpha channel controls the transparency level, enabling blending and opacity effects. <br /><br />

## Alpha Context
Let's now look at another subcontext called `AlphaContext`. It's a part of the `GraphContext` struct and is responsible for managing the alpha blending settings. It's probably the simplest subcontext in Graph1, as it only has two properties:
- `enabled` — a boolean value that turns alpha blending on or off.
- `method` — an enum that defines the blending method. 

### Blend methods 
As of 2024, Graph1 supports two blending methods:
- `AlphaMethod::Int` — uses only integers for all the calculations.
    - This method is somewhat faster, especially on systems where floating-point operations are expensive.
    - May introduce minor inaccuracies due to the limited precision of integer division.
    - Used by default. 
- `AlphaMethod::Float` — Uses `f32` floating-point arithmetic for calculations.
    - Provides higher precision and results in somewhat more accurate blending.
    - May be slower on some systems.
    - Use it if `AlphaMethod::Int` doesn't provide the desired results. 



 
```rust
use graph1::core::context::alpha::AlphaMethod;

// Enable alpha blending
ctx.alpha.enabled = true;

// Set the blending method
ctx.alpha.method = AlphaMethod::Int;

// Take an RGB-color `0xff_33_00` and
// set the alpha channel to `0x85` (133)
let color:u32 = 0xff_33_00_85;

// Draw a filled square at x=0, y=0, side length = 100 
// using `color`, which makes it partially transparent
draw::rectangle::filled(ctx, &RectArea::square(0, 0, 100, Some(color)));
```


## Color adapters
The rendering layer of your application may use a color model other than `RGBA`. For example, a rendering & window management library for native apps [minifb](https://github.com/emoon/minifb) uses `0RGBA`. Another example would be copying pixel data from WASM memory to an HTML Canvas in the browser. The `RGBA` bytes then often need to be re-arranged as `ABGR`. To simplify such conversions, Graph1 provides a set of color adapters in the `graph1::utils::color::adapters` module. <br /><br />

```rust
use graph1::utils::color::adapters::rgba_to_abgr;
 
let buf_len = ctx.frame_buf.len();

// Create a buffer for ABGR colors that will be read by the rendering layer
let mut canvas_buf_abgr: Vec<u32> = vec![0x00; buf_len];

// Convert the frame buffer from RGBA to ABGR 
// and write the result to `canvas_buf_abgr`
rgba_to_abgr(&mut canvas_buf_abgr, &ctx.frame_buf, true).unwrap();
```

If there is no color adapter for your specific use case, you can easily implement one yourself by using the provided adapters as a reference. You can also contribute your adapter to [Graph1 on Github](https://github.com/dipdowel/graph1/) or request it as a feature in the [discussions](https://github.com/dipdowel/graph1/discussions) or [open an issue](https://github.com/dipdowel/graph1/issues).

