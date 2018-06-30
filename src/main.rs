extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

// for trig
use std::f64;

// some constants 
const SCREEN_WIDTH: f64 = 500.0;
const SCREEN_HEIGHT: f64 = 250.0;
const BAR_LENGTH: f64 = 25.0;
const BALL_SPEED: f64 = 200.0;
const BAR_SPEED: f64 = 100.0;
const BALL_START_POS: Point = Point{x: 250.0, y: 125.0};

struct Point {
    x: f64,
    y: f64
}

struct Ball {
    position: Point,
    direction: f64
}

struct Bar {
    position: Point,
    direction: VerticalDir,
    points: u8,

}

impl Bar {
    fn new() -> Bar {
        Bar {
            position: Point {
                x: 0.0,
                y: SCREEN_HEIGHT / 2.0
            },
            direction: VerticalDir::None,
            points: 0
        }
    }

    fn intersect(&self, ball: &Point) -> bool {
        ball.y <= self.position.y + BAR_LENGTH && ball.y >= self.position.y - BAR_LENGTH
    }
}

#[derive(PartialEq)]
enum VerticalDir {
    Up, 
    Down,
    None
}

pub struct App {
    glang: GlGraphics,
    ball: Ball,
    left: Bar,
    right: Bar,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::{ clear, rectangle, Transformed };

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(self.ball.position.x, self.ball.position.y, 5.0);
        let left_bar = rectangle::rectangle_by_corners(
            10.0, 
            self.left.position.y - BAR_LENGTH, 
            15.0, 
            self.left.position.y + BAR_LENGTH);
        let right_bar = rectangle::rectangle_by_corners(
            SCREEN_WIDTH - 15.0, 
            self.right.position.y - BAR_LENGTH, 
            SCREEN_WIDTH - 10.0, 
            self.right.position.y + BAR_LENGTH);

        self.glang.draw(args.viewport(), |cache, gl| {
            clear(BLACK, gl);
            // the position should be the center of the ball, not the edge
            // so offset by half the size of the ball
            let transform = cache.transform.trans(-2.5, -2.5);

            rectangle(WHITE, square, transform, gl);
            rectangle(WHITE, left_bar, cache.transform, gl);
            rectangle(WHITE, right_bar, cache.transform, gl);
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        // ball updates
        // magnitude of the hypo is 5.0 * td, which is seconds since last update
        let hypo = BALL_SPEED * args.dt;
        // sine = x, cos = y
        let dx = self.ball.direction.sin() * hypo;
        let dy = self.ball.direction.cos() * hypo;
        self.ball.position.x += dx;
        self.ball.position.y += dy;
        if self.ball.position.x >= SCREEN_WIDTH - 15.0 { // where the bar starts on the right
            if self.ball.position.x > SCREEN_WIDTH {
                self.ball.position = BALL_START_POS;
                // point for left
                self.left.points += 1;
                println!("Point for Left! Total: {}: {}", self.left.points, self.right.points);
            } else if self.right.intersect(&self.ball.position) {
                self.ball.direction += f64::consts::PI / 2.0;
            }
        } else if self.ball.position.x <= 15.0 {
            if self.ball.position.x < 0.0 {
                self.ball.position = BALL_START_POS;
                // point for right
                self.right.points += 1;
                println!("Point for Right! Total: {}, {}", self.left.points, self.right.points);
            } else if self.left.intersect(&self.ball.position) {
                self.ball.direction -= f64::consts::PI / 2.0;
            }
        }
        if self.ball.position.y >= SCREEN_HEIGHT {
            self.ball.direction += f64::consts::PI / 2.0;
        } else if self.ball.position.y <= 0.0 {
            self.ball.direction -= f64::consts::PI / 2.0;
        }
        // bar updates
        // left first
        if (self.left.position.y < 0.0 + BAR_LENGTH  && self.left.direction == VerticalDir::Up)
        || (self.left.position.y > SCREEN_HEIGHT - BAR_LENGTH && self.left.direction == VerticalDir::Down) {
            self.left.direction = VerticalDir::None;
        }
        match &self.left.direction {
            &VerticalDir::Up => self.left.position.y -= BAR_SPEED * args.dt,
            &VerticalDir::Down => self.left.position.y += BAR_SPEED * args.dt,
            &VerticalDir::None => {}
        }
        // now right
        if (self.right.position.y < 0.0 + BAR_LENGTH  && self.right.direction == VerticalDir::Up)
        || (self.right.position.y > SCREEN_HEIGHT - BAR_LENGTH && self.right.direction == VerticalDir::Down) {
            self.right.direction = VerticalDir::None;
        }
        match &self.right.direction {
            &VerticalDir::Up => self.right.position.y -= BAR_SPEED * args.dt,
            &VerticalDir::Down => self.right.position.y += BAR_SPEED * args.dt,
            &VerticalDir::None => {}
        }
    }

    fn new(opengl_v: OpenGL) -> App {
        App {
            glang: GlGraphics::new(opengl_v),
            ball: Ball {
                position: BALL_START_POS,
                direction: f64::consts::PI / 4.0
            },
            left: Bar::new(),
            right: Bar::new(),
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut win: Window = WindowSettings::new(
        "pong",
        [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32],
    ).opengl(opengl).exit_on_esc(true).build().unwrap();

    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut win) {
        use piston::input::{ Button, Key };
        if let Some(render_args) = event.render_args() {
            app.render(&render_args);
        }
        if let Some(update_args) = event.update_args() {
            app.update(&update_args);
        }
        if let Some(key_args) = event.press_args() {
            match key_args {
                Button::Keyboard(key) => match key {
                    Key::Up => app.right.direction = VerticalDir::Up,
                    Key::Down => app.right.direction = VerticalDir::Down,
                    Key::W => app.left.direction = VerticalDir::Up,
                    Key::S => app.left.direction = VerticalDir::Down,
                    _ => println!("Unknown key {:?}", key)
                },
            _ => println!("Mouse clicks unsupported")
            };
        }
        if let Some(key_args) = event.release_args() {
            match key_args {
                Button::Keyboard(key) => match key {
                    Key::Up | Key::Down => app.right.direction = VerticalDir::None,
                    Key::W | Key::S => app.left.direction = VerticalDir::None,
                    _ => {} // don't log on key up and down
                },
                _ => {}
            }
        }
    }
}
