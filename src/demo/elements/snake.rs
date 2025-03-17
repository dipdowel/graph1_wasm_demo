use crate::demo::user_data::DemoUserData;
use graph1::core::context::{GraphContext, WindowContext};
use graph1::draw;
use graph1::primitives::math::MinMax;
use graph1::primitives::plane::RectArea;
use graph1::utils::clear_screen;
use graph1::utils::color::gradient;
use graph1::utils::color::palettes::RetroNeon;
use graph1::utils::math::rng::XorShiftRng;

/// Represents the four possible movement directions.
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Structure representing the snake.
pub struct Snake {
    body: Vec<(u32, u32)>, // Stores (x, y) coordinates of the snake body (tail is at index 0, head is at last)
    direction: Direction,  // Current movement direction
    length: u32, // Current snake length (intended length, may be > body.len() if snake is growing)
    direction_points: Vec<Direction>, // Queue of upcoming direction changes (not used in this implementation)
    width_tiles: u32,                 // Number of horizontal tiles on the screen
    height_tiles: u32,                // Number of vertical tiles on the screen
    rng: XorShiftRng,                 // Random number generator
    ctx: GraphContext,                // Context used for low-level operations (tile drawing)
    steps_limit: u32,                 // Number of steps to keep moving in the same direction
    tile_container_bg_color: u32,
    counter: u32,
}

impl Snake {
    /// Creates a new snake at the given starting position.
    pub fn new(
        window_context: WindowContext,
        start_x: u32,
        start_y: u32,
        initial_length: u32,
        width_tiles: u32,
        height_tiles: u32,
        seed: u32,
    ) -> Self {
        let mut body = Vec::new();
        // Initially, the snake occupies consecutive tiles vertically.
        // (tail at (start_x, start_y), head at (start_x, start_y + initial_length - 1))

        for i in 0..initial_length {
            body.push((start_x, start_y + i));
        }

        let mut ctx: GraphContext = GraphContext::new(window_context, true, true, None, 1);
        let rng = XorShiftRng::new(seed, seed as u64);

        let tile_container_bg_color =
            gradient::linear_step(0x00_00_00_ff, ctx.win.background_color, 200, 182);

        // clear screen
        draw::tools::fill::buffer(&mut ctx.frame_buf, tile_container_bg_color, 1);

        // FIXME: uncomment?
        // clear_screen(&mut ctx);

        for tile_y in 0..height_tiles {
            for tile_x in 0..width_tiles {
                let container_side = ctx.win.w / width_tiles;
                let tile_side = container_side - 2;
                let x = 1 + tile_x * container_side;
                let y = 1 + tile_y * container_side;
                let rect = RectArea::square(x, y, tile_side, Some(ctx.win.background_color));
                draw::rectangle::filled(&mut ctx, &rect);
            }
        }

        Self {
            body,
            direction: Direction::Up, // Initial movement direction
            length: initial_length,
            direction_points: Vec::new(),
            width_tiles,
            height_tiles,
            rng,
            ctx,
            steps_limit: 4, // starting steps limit
            tile_container_bg_color,
            counter:0
        }
    }

    pub fn get_frame_buffer(&self) -> &Vec<u32> {
        &self.ctx.frame_buf
    }

    /// Moves the snake one tile forward, updating the display.
    pub fn move_forward(&mut self) {


        self.counter += 1;
        if self.counter >100 &&  self.counter < 1000 && self.counter % 100 == 0 {
            self.grow();
        }


        // Determine the current head position.
        let head = *self.body.last().unwrap();
        // Compute the next tile based on the current direction.

        let next_tile = match self.direction {
            Direction::Up => {
                if head.1 == 0 {
                    None // at top edge
                } else {
                    Some((head.0, head.1 - 1))
                }
            }
            Direction::Down => {
                if head.1 >= self.height_tiles - 1 {
                    None // at bottom edge
                } else {
                    Some((head.0, head.1 + 1))
                }
            }
            Direction::Left => {
                if head.0 == 0 {
                    None // at left edge
                } else {
                    Some((head.0 - 1, head.1))
                }
            }
            Direction::Right => {
                if head.0 >= self.width_tiles - 1 {
                    None // at right edge
                } else {
                    Some((head.0 + 1, head.1))
                }
            }
        };

        // Check if a direction change is needed:
        let mut need_new_direction = false;
        if next_tile.is_none() {
            // Condition 1: The head is at the edge.
            need_new_direction = true;
        } else {
            let next = next_tile.unwrap();
            // For collision checking, if the snake is not growing then its tail will be removed.
            // So we ignore the tail (first element) in that case.
            let body_to_check = if self.body.len() == self.length as usize {
                &self.body[1..]
            } else {
                &self.body[..]
            };
            if body_to_check.contains(&next) {
                // Condition 2: The snake would bump into its body.
                need_new_direction = true;
            }
        }
        // Condition 3: steps_limit reached.
        if self.steps_limit == 0 {
            need_new_direction = true;
        }

        // If any condition applies, choose a new direction.
        if need_new_direction {
            self.choose_next_direction();
            // After choosing a new direction, compute the new head tile.
            let head = *self.body.last().unwrap();
            // (It is assumed that choose_next_direction selects only safe directions.)
            let new_head = match self.direction {
                Direction::Up => (head.0, head.1 - 1),
                Direction::Down => (head.0, head.1 + 1),
                Direction::Left => (head.0 - 1, head.1),
                Direction::Right => (head.0 + 1, head.1),
            };
            // Remove the tail only if the snake is not growing.
            if self.body.len() == self.length as usize {
                let tail = self.body.remove(0);
                self.reset_tile(tail.0, tail.1);
            }
            self.body.push(new_head);
            self.set_tile(new_head.0, new_head.1);
            // The steps_limit has been reset inside choose_next_direction.
            return;
        }

        // Otherwise, proceed with the current direction.
        let new_head = next_tile.unwrap();
        if self.body.len() == self.length as usize {
            let tail = self.body.remove(0);
            self.reset_tile(tail.0, tail.1);
        }
        self.body.push(new_head);
        self.set_tile(new_head.0, new_head.1);
        // Decrement steps_limit for continuing in the same direction.
        self.steps_limit = self.steps_limit.saturating_sub(1);
    }

    /// Increases the snake's length by one tile.
    pub fn grow(&mut self) {
        // Simply increment the intended length.
        self.length += 1;
    }

    /// Determines a new safe direction for the snake.
    fn choose_next_direction(&mut self) {
        let head = *self.body.last().unwrap();
        let mut candidates = Vec::new();

        // Consider all four directions.
        for &candidate in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            // Do not allow reversing the snake.
            if Self::is_opposite(self.direction, candidate) {
                continue;
            }
            // Compute the candidate's next head position.
            let next = match candidate {
                Direction::Up => {
                    if head.1 == 0 {
                        continue; // would hit top edge
                    } else {
                        (head.0, head.1 - 1)
                    }
                }
                Direction::Down => {
                    if head.1 >= self.height_tiles - 1 {
                        continue; // would hit bottom edge
                    } else {
                        (head.0, head.1 + 1)
                    }
                }
                Direction::Left => {
                    if head.0 == 0 {
                        continue; // would hit left edge
                    } else {
                        (head.0 - 1, head.1)
                    }
                }
                Direction::Right => {
                    if head.0 >= self.width_tiles - 1 {
                        continue; // would hit right edge
                    } else {
                        (head.0 + 1, head.1)
                    }
                }
            };
            // For collision checking, if the snake is not growing, ignore the tail.
            let body_to_check = if self.body.len() == self.length as usize {
                &self.body[1..]
            } else {
                &self.body[..]
            };
            if body_to_check.contains(&next) {
                continue; // candidate would lead to a collision
            }
            candidates.push(candidate);
        }

        // If one or more safe directions exist, choose one randomly.
        if !candidates.is_empty() {
            let index = self.rng.get_u32(&MinMax {
                min: 0,
                // max: (candidates.len() - 1) as u32,
                max: candidates.len() as u32,
            }) as usize;
            self.direction = candidates[index];
        }
        // Reinitialize steps_limit with a new random value.
        self.steps_limit = self.rng.get_u32(&MinMax {
            min: self.height_tiles / 4,
            max: self.width_tiles / 7*4,
        });
    }

    /// Returns true if `d2` is the opposite direction to `d1`.
    fn is_opposite(d1: Direction, d2: Direction) -> bool {
        matches!(
            (d1, d2),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }

    /// Draws a tile at the given position with the specified color.
    fn draw_tile(&mut self, tile_x: u32, tile_y: u32, color: u32) {
        let container_side = self.ctx.win.w / self.width_tiles;
        let tile_side = container_side - 2;
        let x = 1 + tile_x * container_side;
        let y = 1 + tile_y * container_side;
        let rect = RectArea::square(x, y, tile_side, Some(color));
        draw::rectangle::filled(&mut self.ctx, &rect);
    }

    /// Resets the tile at the given position to the default color.
    fn reset_tile(&mut self, tile_x: u32, tile_y: u32) {
        self.draw_tile(tile_x, tile_y, self.ctx.win.background_color);
    }

    /// Sets the tile at the given position to the snake color.
    fn set_tile(&mut self, tile_x: u32, tile_y: u32) {
        self.draw_tile(tile_x, tile_y, self.ctx.win.foreground_color);
    }
}
