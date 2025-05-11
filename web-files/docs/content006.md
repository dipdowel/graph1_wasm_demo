# Colors
## Desaturation

| <!-- -->    | <!-- -->                                                                                                           |
|-------------|--------------------------------------------------------------------------------------------------------------------|
| 💡         | In the context of Graph1 `Desaturation` means removing colors from <br/>an image, making it effectively grayscale. |

If you need to desaturate an image or a part thereof, Graph1 provides two modules for that:
- [utils::color::desaturate::intensity](https://github.com/dipdowel/graph1/blob/rgba/src/utils/color/desaturate/intensity.rs)
- [utils::color::desaturate::luminance](https://github.com/dipdowel/graph1/blob/rgba/src/utils/color/desaturate/luminance.rs)

The demo above shows 4 ghosts of different colors moving along colorful lanes.  
The 3 vertical grayscale regions illustrate 3 ways of desaturation.

<br />
### 🧪 Intensity vs. Luminance
|                           (1) Basic intensity                           |                             (2) Quadratic intensity                             |                                   (3) Luminance                                    
|:-----------------------------------------------------------------------:|:-------------------------------------------------------------------------------:|:--------------:|
| ![basic-intensity](/docs/media/intensity-luminance/intensity-basic.png) | ![quadratic-intensity](/docs/media/intensity-luminance/intensity-quadratic.png) | ![luminance](/docs/media/intensity-luminance/luminance.png) |

Colors <span style="font-weight:bold;color:rgb(255, 0, 153);">NEON PINK</span> and <span style="font-weight:bold;color:rgb(0, 154, 255);">CYBER BLUE</span>  have a similar physical intensity, so when desaturated using _intensity_, they result in similar shades of gray. At the same time, the _luminance_ method provides two distinct shades of gray for these colors, which is closer to how the human eye perceives light.

### ⚙️ Basic intensity
- See the left-most grayscale region in the animated demo above.
  The fastest desaturation, calculated with a simple formula:
```rust
let basic_intensity = (r + g + b) / 3;
```

### 📐 Quadratic intensity
- See the middle grayscale region in the animated demo above.

A bit slower than the basic intensity, but more "physically" accurate. It involves calculating the root mean square (RMS) of RGB values:
```rust
let quadratic_intensity =((r * r + g * g + b * b) / 3.0).sqrt();
```

### 🌕 Luminance
- See the right-most grayscale region in the animated demo above.
  In graphics, we use [luminance](https://en.wikipedia.org/wiki/Luminance) to mimic how humans perceive light in real-world scenes, so this way of desaturating an image provides the most human-eye-friendly results. In Graph1 we calculate luminance by performing the following multiplications on the RGB channels:

```rust
let luminance = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
```

The constants `0.299`, `0.587` and `0.114` are called [luma coefficients](https://en.wikipedia.org/wiki/Rec._709#Luma_coefficients).  They are described in [Rec. 709 standard](https://en.wikipedia.org/wiki/Rec._709) for HDTV.
<br /><br />
- - - - 
💻 [Code of this demo on Github](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_006_luminance_vs_intensity.rs)

<!-- Check [graph1::utils::color::properties::luminance](https://docs.rs/graph1/latest/graph1/utils/color/properties/fn.luminance.html) -->

