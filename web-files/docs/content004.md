# Colors 
## Luminance and Intensity 
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
For instance, a **<span style="color:blue;">blue</span>** and a **<span style="color:yellow;">yellow</span>** pixels might have the same physical intensity, 
but we might want to calculate different luminance values for them, because the human eye perceives yellow as brighter!
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
Check [graph1::utils::color::properties::luminance](https://docs.rs/graph1/latest/graph1/utils/color/properties/fn.luminance.html) 
for all the available methods.

<br /><br />

###  Intensity
Intensity can be thought of as the average of the RGB components of a light source if we're treating them equally without
weighting for perception. Here’s one common approach:








