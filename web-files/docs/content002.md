# Basic concepts, pt.2
## Graph Context

The `GraphContext` struct is essential for all the drawing operations. 
- Let's look at the minimal setup to get a working `GraphContext` instance.
- Also, we'll draw two filled rectangles on the screen.
 
```rust
use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::plane::RectArea;
use graph1::utils::color::palettes::RetroNeon;

const WIN_WIDTH: u32 = 480;
const WIN_HEIGHT: u32 = 240;

fn main() {  
  // The window context
  let win_ctx = WindowContext::new(
    WIN_WIDTH,
    WIN_HEIGHT,
    Some(RetroNeon::CYBER_BLUE),
    Some(RetroNeon::LASER_LIME),
  );

  // Disable alpha blending when you don't need it. It improves performance.
  let use_alpha = false;

  // Disabled the draft buffer when you don't need it. It saves some memory.
  let use_draft_buf = false;

  // No user data needed for this demo
  let user_data = None;

  // Graph context
  let mut ctx: GraphContext =
          GraphContext::new(win_ctx, use_alpha, use_draft_buf, user_data);
    
  // rectangles have the same width  
  let width = ctx.win.w - 40;
    
  // Rectangle #1: 
  //  x = 20, y = 20, width = 440, height = 20, 
  // `None` =  the default window foreground color
  let rect_1 = RectArea::new(20, 20, width, 20, None);

  // Rectangle #2:
  // x = 20, y = 200, width = 440, height = 20, 
  // vibrant cyan color 
  let color_2 = Some(RetroNeon::VIBRANT_CYAN);
  let rect_2 = RectArea::new(20, ctx.win.h - 40, width, 20, color_2);

  // Draw both rectangles on the screen
  draw::rectangle::filled(&mut ctx, &rect_1);
  draw::rectangle::filled(&mut ctx, &rect_2);

  // The code that displays the text 
  // and renders the screen was omitted for brevity    
    
  
}

```
We'll cover what `user_data = None` means in the [next section](http://localhost:8080/?demo=3).

- - - - 
Also, see:
- [Alpha blending](http://localhost:8080/?demo=999) for details on `use_alpha = false`
- [Draft buffer](http://localhost:8080/?demo=999) for details on `use_draft_buf = false`

<br /><br />
- - - - 
- [Code of this demo on Github](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_002_basic_concepts_pt2.rs)
