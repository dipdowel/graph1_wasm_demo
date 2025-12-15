# Text Rendering

Graph1 provides a text rendering system, **pixel fonts** in [CBF file format](https://github.com/dipdowel/compact-bitmap-font), and text layout capabilities. You can design and use your own CBF-fonts as well.

---

## 📦 Embedded Fonts

Graph1 ships with 4 embedded pixel fonts in CBF format (CBF = Compact Bitmap Font). Font color, scaling, alignment, and other attributes are configurable. 

### 1. Matriks Uaxactun font family

**Matriks Uaxactun fonts** were designed by [Marcel van Deijl](https://designisfijn.com/) specifically for `Graph1`.
As of December 2025, there are two variants available:
- `EmbeddedFonts::MatriksUaxactun` — Regular (proportional) variant
- `EmbeddedFonts::MatriksUaxactunMono` — Monospaced variant
- Fonts contain 225 characters, so most EU-languages can be rendered.
- The demo above uses both of the fonts: The regular one for the title scroller, and the monospaced one for the terminal-like text.

### 2. C&C Red Alert fonts
- `EmbeddedFonts::CCRedAlertInet` — INET variant
- `EmbeddedFonts::CCRedAlertLan` — LAN variant
- Created by [N3tRunn3r](https://forums.cncnet.org/topic/78-the-red-alert-fonts/) in 2008 after pixel fonts from the classic RTS game Command & Conquer: Red Alert.

---

## ⚡ Quick Start

```rust
// 1. Instantiate font
let font = instantiate_embedded_font(
    EmbeddedFonts::MatriksUaxactunMono,
    2,                          // scale: 1, 2, 3, 4,...
    None,
    None                        // default char
);

// 2. Setup color
let props = printer::ColorProperties {
    color: Some(0x00ff00ff),   // green
    color_transformer: None,
    data: None,
};

// 3. Print single line
let dims = printer::print_line(ctx, &Point::new(10, 10), &font, &props, &"Hello!");

// 4. Print multiple lines
let lines = ["Line 1", "Line 2"];
printer::print(ctx, &Point::new(10, 30), &font, &props, &lines, Align::Center);

// 5. Create character grid (monospaced only!)
let grid = make_monospaced_char_grid(
    &font, 
    Variant::Primary(&lines), 
    Point::new(0, 0), 
    Some(0xff0000ff)
)?;
```
---

## 🎨 Basic Text Rendering

### Creating a Font Instance

Use `instantiate_embedded_font()` to create a `PixelFont` with custom scaling and spacing:

```rust
use graph1::text::font_embedder::{instantiate_embedded_font, EmbeddedFonts};
use graph1::text::font::Spacing;

let font = instantiate_embedded_font(
    EmbeddedFonts::MatriksUaxactunMono,
    2,                   // font scale factor: 1, 2, 3, 4, etc.
    Some(Spacing {
        kerning_px: 3,   // horizontal spacing between characters
        leading_px: 4,   // vertical spacing between lines
    }),
    None,               // default char (fallback for missing glyphs)           
);
```

**Parameters:**
- `font_name`: One of the `EmbeddedFonts` enum variants
- `font_scale_factor`: How much to scale the font up (1 = native size, 2 = double size, 3 = triple size, etc.)
- `spacing`: Optional custom kerning and leading (defaults to font's native spacing)
- `default_char`: Optional fallback character to render when the font does not contain a request character. `None` results in the font's built-in default char (e.g. `?`).

### External Fonts

You can also load fonts from external CBF files using `instantiate_external_font()`:

```rust
use graph1::text::font_embedder::instantiate_external_font;

let font_data: &[u8] = include_bytes!("path/to/custom.cbf");
let font = instantiate_external_font(font_data, 2, None, None);
```

### Printing Text

The `printer` module provides two main functions for rendering text:

#### Single Line: `print_line()`

Renders a single line of text and returns its dimensions:

```rust
use graph1::text::printer;
use graph1::primitives::point::Point;
use graph1::primitives::plane::Dimensions2d;

let color_props = printer::ColorProperties {
    color: Some(0x6fff43ff),   // green color in RGBA
    color_transformer: None,   // color transformer function
    data: None,                // data to be passed to `color_transformer()`
};

let dimensions: Dimensions2d = printer::print_line(
    ctx,                        // the context
    &Point { x: 50, y: 50 },    // starting position
    &font,                      // the font instance
    &color_props,               // color properties
    &"Hello, World!",           // text to render
);
```

**Returns:** `Dimensions2d` with the width and height of the rendered text.

#### Measurement: `get_line_dimensions()`

Measure text dimensions without performing actual rendering:

```rust
let dims = printer::get_line_dimensions(&font, "Text to measure");
println!("Width: {}, Height: {}", dims.w, dims.h);
```
Knowing exact dimensions of the text to be printed can be useful for layout calculations, effects, animations, etc.

#### Multi-line Text: `print()`

Render multiple lines with alignment support:

```rust
use graph1::text::printer::Align;

let lines = [
    "Graph1 is a pixel graphics library",
    "for creative coding in Rust.",
    "Have fun!",
];

let dimensions = printer::print(
    ctx,
    &Point { x: 26, y: 19 },
    &font,
    &color_props,
    &lines,
    Align::Left,  // or Align::Center, Align::Right
);
```

**Alignment behavior:**
- `Align::Left`: All lines start at the same x-coordinate
- `Align::Right`: All lines end at the same x-coordinate (aligned to longest line)
- `Align::Center`: All lines centered relative to the longest line

**Returns:** `Dimensions2d` with total width (longest line) and height (all lines + leading).

#### Multi-line Measurement: `get_text_dimensions()`

Measure dimensions of multi-line text without rendering:
```rust
let dims = printer::get_text_dimensions(&font, &lines, Align::Center);
```

---

## 🔤 Character Grids

Function `make_monospaced_char_grid()` returns a grid (`UniformGrid<u32>`) representing positions of each character cell
of a given text. This is useful for terminal emulators, cursor animation and control, or any scenario where you need 
to manipulate text at the character-cell level.

> ⚠️ **Important:** This function only works with **monospaced fonts**. Use `is_font_monospaced()` to check if a font is monospaced.

```rust
use graph1::text::char_grid::make_monospaced_char_grid;
use graph1::text::helpers::is_font_monospaced;
use graph1::primitives::data_structs::variant::Variant;

// Verify font is monospaced
if !is_font_monospaced(&font) {
    panic!("This font is not monospaced!");
}

let text = ["Line 1", "Line 2", "Line 3"];
let dimensions = Variant::Primary(&text);

let char_grid = make_monospaced_char_grid(
    &font,
    dimensions,
    Point { x: 26, y: 19 },
    Some(0x11bb05ff),  // optional color for cells
)?;
```

### Using Character Grids

The returned `UniformGrid<u32>` provides powerful grid operations:

```rust
// Access individual cells
let cell = char_grid.get_cell(row, col)?;

// Iterate through all cells
for cell in char_grid.cells() {
    // Each cell is a Region with dimensions and position
    draw::rectangle::filled(ctx, cell);
}

// Get grid dimensions
let (rows, cols) = (char_grid.rows(), char_grid.cols());

// Access the prototype cell (template for all cells)
let proto = &char_grid.proto_cell;
```

### Input Variants

You can specify dimensions in two ways:

1. **From text array** (auto-calculates dimensions):
   ```rust
   Variant::Primary(&["Line 1", "Longer line 2!"])
   // Creates grid: 2 rows × 14 cols (width of longest line)
   ```

2. **Explicit dimensions**:
   ```rust
   Variant::Secondary(Dimensions2d { w: 40, h: 10 })
   ```

---

## ✨ Text color

### Color Properties

The `ColorProperties` struct offers two ways to control text color:

```rust
pub struct ColorProperties<'a> {
    pub color: Option<u32>,
    pub color_transformer: Option<PixelColorTransformerFn>,
    pub data: Option<&'a Vec<u32>>,
}
```

### 1. Solid color – the simplest approach:
   ```rust
   ColorProperties {
       color: Some(0xff00ffff),  // magenta
       color_transformer: None,
       data: None,
   }
   ```

### 2. Color transformer – apply custom per-pixel transformation:
   ```rust
   ColorProperties {
       color: None,
       color_transformer: Some(my_transformer_fn),
       data: Some(&my_data),
   }
   ```
The effect below was achieved by using a custom color transformer function:<br /><br />![ABCDEFG](/docs/media/color-transform.png)<br />

See type [PixelColorTransformerFn](https://github.com/dipdowel/graph1/blob/develop/src/primitives/helper_types.rs) for more details.

 
### Typography: Spacing Control

Fine-tune text appearance with the `Spacing` struct:

```rust
pub struct Spacing {
    pub kerning_px: u8,  // horizontal space between characters
    pub leading_px: u8,  // vertical space between lines
}
```

**Kerning** (horizontal spacing):
- Applied between characters in a line
- Final line width = sum of glyph widths + (kerning × (num_chars - 1))

**Leading** (vertical spacing):
- Applied between lines of text
- Total text height = (line_height + leading) × num_lines

Each font has native spacing values, but you can override them:
```rust
// Use font's native spacing
let font = instantiate_embedded_font(font_name, scale, None, None);

// Override spacing
let custom_spacing = Spacing { kerning_px: 5, leading_px: 10 };
let font = instantiate_embedded_font(font_name, scale, Some(custom_spacing), None);
```
--- 

### Multi-language Support
The Matriks Uaxactun fonts contain 225 characters, covering:
- Latin alphabet (upper and lower case)
- Numbers and common symbols
- Accented characters (é, ñ, ü, etc.)
- Special typography symbols (—, –, •, °, etc.)

---

## 📦 CBF Format (Compact Bitmap Font)

Graph1 uses a custom binary format called [CBF](https://github.com/dipdowel/compact-bitmap-font), for storing pixel fonts efficiently. Here are some features of CBF:
- **Compact**: 1-bit bitmap encoding (black/white)
- **Embedded**: Compiled into binary via `include_bytes!`
- **Metadata-rich**: Includes author, version, date
- **Fast loading**: Direct memory mapping, no parsing overhead

All the embedded fonts are stored as `.cbf` files in `src/text/cbf_data/`.
 

💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_013_text.rs)

