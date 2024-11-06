# Colors, Desaturation


| <!-- -->    | <!-- -->                                                                                                      |
|-------------|---------------------------------------------------------------------------------------------------------------|
| :bulb:         | `Desaturation` in the context of Graph1 is removing of color from an image, making it effectively grayscale. |

If you need to desaturate an image, a region of an image, or a single pixel, Graph1 provides two modules for that:
- `utils::color::desaturate::intensity` -- performant but less accurate
- `utils::color::desaturate::luminance` -- accurate but less performant

##  Intensity 
In the demo, colors of the first two columns  are **<span style="color:rgb(255, 0, 153);">NEON PINK</span>** and **<span style="color:rgb(0, 154, 255);">CYBER BLUE</span>**.
When desaturated using _intensity_, they are represented as the same shade of gray (see the lower grayscale region).
### Basic intensity
Intensity comes in two flavors, the fastest of the two is calculated with a simple formula and just integer numbers: 
```rust
let basic_intensity = (r + g + b) / 3.0;
```
### Physical intensity
Physical intensity of a color is a little bit slower as it involves calculating the root mean square (RMS) of RGB values:
```rust
let physical_intensity =((r * r + g * g + b * b) / 3.0).sqrt();
```
If you want to use the physical intensity, set `square` parameter to `true` when calling functions in the `intensity` module:
```rust
let squared = true;
rgba_region_intensity(ctx, &img_region, squared);
```

| Basic intensity | Physical intensity |
| :--------------: | :--------------: |
| ![basic-intensity](/docs/media/intensity-luminance/intensity-basic.png)   | ![basic-intensity](/docs/media/intensity-luminance/intensity-squared.png) |





vs. Luminance
Functions from `desaturate::intensity` module provide the most performant desaturation. It may be suitable for some purposes,
especially where the speed is crucial. 



However, the human eye perceives light differently.

For instance, colors of the first two columns are **<span style="color:rgb(255, 0, 153);">NEON PINK</span>** and **<span style="color:rgb(0, 154, 255);">CYBER BLUE</span>**. 
They are very similar in terms of physical intensity. The lower grayscale region represents them as the same shade of gray. 

however, it is not as accurate as the luminance method.

but we might want to apply some adjustments when converting both to grayscale (desaturating), because the human eye perceives yellow as brighter!









There are two approaches to desaturate an image (to convert a color pixels to grayscale):

<br />
[Luminance](https://en.wikipedia.org/wiki/Luminance) and intensity are both used in simulating lighting in computer graphics. 
They contribute to different aspects of how light interacts with objects and how brightness is perceived by humans. 
Both can be used for converting color images to grayscale, but they yield different results.
<br /><br />

In the demo above, we have 8 vertical color bars, they are intersected by two horizontal grayscale regions:
- The top grayscale region was computed with `rgba_region_luminance()`
- The bottom grayscale region was computed with `rgba_region_intensity()`

<br /><br />
### Luminance
In graphics, we use [luminance](https://en.wikipedia.org/wiki/Luminance) to mimic how humans perceive light in real-world scenes.
For instance, a **<span style="color:rgb(255, 0, 153);">NEON PINK</span>** and a **<span style="color:rgb(0, 154, 255);">CYBER BLUE</span>** pixels might have the same physical intensity, 
but we might want to apply some adjustments when converting both to grayscale (desaturating), because the human eye perceives yellow as brighter!
In `Graph1` we calculate luminance by performing the following multiplications on the RGB channels:
<br />
```rust
// Luma coefficients for RGB channels:

R * 0.2126 
G * 0.7152
B * 0.0722
```

Those constants are called [luma coefficients](https://en.wikipedia.org/wiki/Rec._709#Luma_coefficients). <br />
They are described in [Rec. 709 standard](https://en.wikipedia.org/wiki/Rec._709) for HDTV.
- - - - -
Check [graph1::utils::color::desaturate::luminance](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/desaturate/luminance_vs_intensity.rs)
<!-- Check [graph1::utils::color::properties::luminance](https://docs.rs/graph1/latest/graph1/utils/color/properties/fn.luminance.html) -->
for all the available methods.

<br /><br />

