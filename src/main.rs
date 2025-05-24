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
}

impl EventHandler for GameState {
fn update(&mut self, _ctx: &mut Context) -> GameResult {
    if self.game_over {
        return Ok(());
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
        canvas.draw(&score_text, dest);
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

struct GameOverState {
    score: u32,
}

impl EventHandler for GameOverState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // Get window dimensions
        let (window_width, window_height) = ctx.gfx.drawable_size();

        // Display "Game Over" message
        let mut game_over_text = graphics::Text::new("Game Over!");
        game_over_text.set_scale(50.0);
        let game_over_width = game_over_text.measure(ctx)?.x;
        let game_over_dest = ggez::glam::Vec2::new(
            (window_width - game_over_width) / 2.0,
            window_height / 3.0,
        );
        canvas.draw(&game_over_text, game_over_dest);

        // Display final score
        let mut score_text = graphics::Text::new(format!("Your Score: {}", self.score));
        score_text.set_scale(40.0);
        let score_width = score_text.measure(ctx)?.x;
        let score_dest = ggez::glam::Vec2::new(
            (window_width - score_width) / 2.0,
            window_height / 2.0,
        );
        canvas.draw(&score_text, score_dest);

        // Display restart instruction
        let mut restart_text = graphics::Text::new("Press R to restart");
        restart_text.set_scale(30.0);
        let restart_width = restart_text.measure(ctx)?.x;
        let restart_dest = ggez::glam::Vec2::new(
            (window_width - restart_width) / 2.0,
            window_height * 2.0 / 3.0,
        );
        canvas.draw(&restart_text, restart_dest);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: ggez::input::keyboard::KeyInput, _repeated: bool) -> GameResult {
        if let Some(key) = input.keycode {
            if key == KeyCode::R {
                // Restart the game
                let game = GameState::new();
                let (ctx, event_loop) = ContextBuilder::new("snake_game", "you")
                .window_setup(ggez::conf::WindowSetup::default().title("Snake Game in Rust"))
                .window_mode(
                    ggez::conf::WindowMode::default().dimensions(
                        GRID_SIZE * WIDTH as f32,
                        GRID_SIZE * HEIGHT as f32,
                    ),
                )
                .build().unwrap();
            
                event::run(ctx, event_loop, game);
            }
        }
        Ok(())
    }
}

enum AppState {
    Game(GameState),
    GameOver(GameOverState),
}

impl EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self {
            AppState::Game(game_state) => {
                game_state.update(ctx)?;
                if game_state.game_over {
                    // Transition to GameOverState
                    *self = AppState::GameOver(GameOverState {
                        score: game_state.score,
                    });
                }
            }
            AppState::GameOver(game_over_state) => {
                game_over_state.update(ctx)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match self {
            AppState::Game(game_state) => game_state.draw(ctx),
            AppState::GameOver(game_over_state) => game_over_state.draw(ctx),
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    ) -> GameResult {
        match self {
            AppState::Game(game_state) => game_state.key_down_event(ctx, input, repeated),
            AppState::GameOver(_game_over_state) => {
                if let Some(key) = input.keycode {
                    if key == KeyCode::R {
                        // Restart the game
                        *self = AppState::Game(GameState::new());
                    }
                }
                Ok(())
            }
        }
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

    let initial_state = AppState::Game(GameState::new());
    event::run(ctx, event_loop, initial_state)
}
