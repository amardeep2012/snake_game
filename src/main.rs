use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect},
    input::keyboard::KeyCode,
    Context, ContextBuilder, GameResult,
};
use rand::Rng;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const GRID_SIZE: f32 = 40.0;
const WIDTH: usize = 40;
const HEIGHT: usize = 40;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct GameState {
    snake: VecDeque<Point>,
    dir: Direction,
    food: Point,
    last_update: Instant,
    game_over: bool,
    score:u32,
}

impl GameState {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Point { x: 10, y: 10 });
        let food = GameState::random_food(&snake);

        GameState {
            snake,
            dir: Direction::Right,
            food,
            last_update: Instant::now(),
            game_over: false,
            score:0,
        }
    }

    fn random_food(snake: &VecDeque<Point>) -> Point {
        let mut rng = rand::thread_rng();
        loop {
            let p = Point {
                x: rng.gen_range(0..WIDTH),
                y: rng.gen_range(0..HEIGHT),
            };
            if !snake.contains(&p) {
                return p;
            }
        }
    }

    // fn update_snake(&mut self) {
    //     if self.game_over {
    //         return;
    //     }

    //     let head = self.snake.front().unwrap();
    //     let new_head = match self.dir {
    //         Direction::Up => Point {
    //             x: head.x,
    //             y: head.y.wrapping_sub(1),
    //         },
    //         Direction::Down => Point {
    //             x: head.x,
    //             y: head.y + 1,
    //         },
    //         Direction::Left => Point {
    //             x: head.x.wrapping_sub(1),
    //             y: head.y,
    //         },
        //     Direction::Right => Point {
        //         x: head.x + 1,
        //         y: head.y,
        //     },
        // };

        // // Check for collision with walls
        // if new_head.x < 0 || new_head.x >= WIDTH || new_head.y < 0 || new_head.y >= HEIGHT {
        //     self.game_over = true;
        //     return;
        // }

        // // Check for collision with self
        // if self.snake.contains(&new_head) {
        //     self.game_over = true;
        //     return;
        // }
        // self.snake.push_front(new_head);
        // if new_head == self.food {
        //     // self.snake.push_front(new_head);
        //     self.food = GameState::random_food(&self.snake);
        // } else {
    //         // self.snake.push_front(new_head);
    //         self.snake.pop_back();
    //     }
    // }
}

impl EventHandler for GameState {
fn update(&mut self, _ctx: &mut Context) -> GameResult {
    if self.game_over {
        return Ok(()); // Skip update
    }

    if self.last_update.elapsed() >= Duration::from_millis(100) {
        let head = self.snake.front().unwrap();
        let new_head = match self.dir {
            Direction::Up => Point {
                x: head.x,
                y: head.y.saturating_sub(1),
            },
            Direction::Down => Point {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Point {
                x: head.x.saturating_sub(1),
                y: head.y,
            },
            Direction::Right => Point {
                x: head.x + 1,
                y: head.y,
            },
        };

        // Collision: wall
        if new_head.x >= WIDTH || new_head.y >= HEIGHT {
            self.game_over = true;
            return Ok(());
        }

        // Collision: self
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return Ok(());
        }

        if new_head == self.food {
            self.snake.push_front(new_head);
            self.food = GameState::random_food(&self.snake);
            self.score += 1; // Increment score when food is eaten
        } else {
            self.snake.push_front(new_head);
            self.snake.pop_back();
        }

        self.last_update = Instant::now();
    }
    Ok(())
}


    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // Draw snake

        for segment in &self.snake {
            let rect = Rect::new(
                (segment.x as f32) * GRID_SIZE,
                (segment.y as f32) * GRID_SIZE,
                GRID_SIZE,
                GRID_SIZE,
            );
            let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?;
            canvas.draw(&mesh, DrawParam::default());
        }

        // Draw food
        let food_rect = Rect::new(
            (self.food.x as f32) * GRID_SIZE,
            (self.food.y as f32) * GRID_SIZE,
            GRID_SIZE,
            GRID_SIZE,
        );
        let food_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), food_rect, Color::RED)?;
        canvas.draw(&food_mesh, DrawParam::default());

        
        // Draw Score
        let mut  score_text = graphics::Text::new(format!("Score: {}", self.score));
        score_text.set_scale(30.0);
        let dest = ggez::glam::Vec2::new(20.0, 20.0);
        canvas.draw(&score_text, dest);;
        if self.game_over {
            let mut text = graphics::Text::new("Game Over! Press R to restart");
            text.set_scale(30.0);
            let dest = ggez::glam::Vec2::new(50.0, GRID_SIZE * HEIGHT as f32 / 2.0);
            canvas.draw( &text, dest);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: ggez::input::keyboard::KeyInput, _repeated: bool) -> GameResult {
        if let Some(key) = input.keycode {
            if self.game_over && key == KeyCode::R {
                // Reset the game state
                *self = GameState::new();
                return Ok(());
            }

            if !self.game_over {
                self.dir = match key {
                    KeyCode::Up if self.dir != Direction::Down => Direction::Up,
                    KeyCode::Down if self.dir != Direction::Up => Direction::Down,
                    KeyCode::Left if self.dir != Direction::Right => Direction::Left,
                    KeyCode::Right if self.dir != Direction::Left => Direction::Right,
                    _ => self.dir,
                };
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("snake_game", "you")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game in Rust"))
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(
                GRID_SIZE * WIDTH as f32,
                GRID_SIZE * HEIGHT as f32,
            ),
        )
        .build()?;

    let game = GameState::new();
    event::run(ctx, event_loop, game)
}
