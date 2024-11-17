# User Data + Animation
## Bouncy
Whenever I start working with a new system capable of animation, I always like to build a demo that I call Bouncy. It's a square that bounces around the screen, just like the one in an episode of [The Office (American TV series)](https://en.wikipedia.org/wiki/The_Office_(American_TV_series)), where everyone was waiting for the 'DVD' logo to end up in a corner of the screen during a boring meeting. <br /><br />
![DVD Bouncy from The Office](/docs/media/dvd-bouncy.png)<br />

I, however, got inspired by early computer games and not by the series.
Anyway, Bouncy is a simple animation that takes very little code to implement. Nevertheless, it requires to maintain some application state, e.g. position of the floating square on the current frame needs to be updated and passed to the next frame. That's where `GraphContext::user_data` comes in handy.  

```rust
// What the `user_data` looks like in the `GraphContext` struct
pub struct GraphContext<UserData = Vec<i32>> {
    // properties omitted for brevity
    pub user_data: Box<UserData>,
}
```
`GraphContext` accepts a type parameter `UserData`, so you can configure the shape of the data in a way that suits your needs. 


#### Conceptual example of Bouncy
```rust
use graph1::utils::clear_screen;
use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::plane::RectArea;

/// Our bouncing hero!
struct Bouncy {    
    pub x:  i32, // starting position, x-coordinate    
    pub y:  i32, // starting position, y-coordinate    
    pub dx: i32, // starting direction and speed, x-axis    
    pub dy: i32, // starting direction and speed, y-axis
}

// The user data type must implement `Default` trait!
struct UserData { bouncy: Bouncy }

impl Default for UserData {
    fn default() -> Self {
        Self { bouncy: Bouncy { x: 10, y: 10, dx: 1, dy: 1 }
        }
    }
}

fn main() {
    let bouncy_user_data = Some(UserData::default());
    let mut ctx: GraphContext<UserData> =
        GraphContext::new(WindowContext::default(),
                          false,
                          false,
                          bouncy_user_data
        );

    // Here should be the animation loop, which calls `render_frame()`.
    // Please read on for details.
}

const SQUARE_SIDE_PX: i32 = 16;

/// Render a frame with Bouncy, who is just a square bouncing on the screen
pub fn render_frame(ctx: &mut GraphContext<UserData>) {
    
    // Read the animation values from the user data in the context
    let Bouncy { mut x, mut y, mut dx, mut dy } = ctx.user_data.bouncy;
    
    // Move Bouncy by 1 step.    
    x = x + dx;
    y = y + dy;

    // Write the updated animation values back to the context.
    ctx.user_data.bouncy.x = x;
    ctx.user_data.bouncy.y = y;

    clear_screen(ctx);
    
    // Finally, render Bouncy on the window surface
    let bouncy = RectArea::square(x as u32, y as u32, 20, None);    
    draw::rectangle::filled(ctx, &bouncy);
}

```
 
### Native application
The demos from this website can be compiled as a native application as well. I used [minifb](https://github.com/emoon/minifb) library  for window management and rendering. You may want to check out the [Graph1 minifb demo repo on Github](https://github.com/dipdowel/graph1_minifb_demo). 
- `NB:` The `minifb` application was tested only on Linux with X11.

<br /> If you got this far, it may be the time to grab [this whole project on Github](https://github.com/dipdowel/graph1_wasm_demo). It contains everything you see on this website, including the Bouncy demo. The project can be compiled for Web (WASM) and you can run it locally.

- - - - 
- [Code of this demo on Github](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_003_bouncy.rs)
- [`GraphContext` configuration and the animation loop](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/lib.rs)