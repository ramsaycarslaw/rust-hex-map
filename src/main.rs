use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::input::mouse::MouseButton;
use ggez::conf;
use ggez::input::keyboard::{KeyInput, KeyCode};
use ggez::timer;

use std::env;
use std::path;

use rand::Rng;

mod hexagon;
mod hexgrid;

const FORESHORTENING_FACTOR: f32 = 0.5;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let backend = ggez::conf::Backend::default();

    let cb = ContextBuilder::new("hex_based_game", "ggez")
        .window_setup(conf::WindowSetup::default().title("Hex Based Game"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(800.0, 600.0)
                .resizable(true)
                .fullscreen_type(conf::FullscreenType::Windowed),
        )
        .add_resource_path(resource_dir)
        .backend(backend);

    // Context
    let (mut ctx, event_loop) = cb.build().expect("failed to create context.");
    // Create an instance of the event handler
    let game = Game::new(&mut ctx);

    // Run the event loop
    event::run(ctx, event_loop, game);
}

struct WindowSettings {
    toggle_fullscreen: bool,
    is_fullscreen: bool,
    is_resizable: bool,
}

struct Game {
    // state 
    hexgrid: hexgrid::Hexgrid,

    // viewport
    camera_offset: Point2<f32>,
    dragging: bool,
    last_mouse_pos: Point2<f32>,
    velocity: Point2<f32>,
    friction: f32,
    screen_coordinates: Point2<f32>,

    // window settings
    window_settings: WindowSettings,

    // zoom
    zoom: f32,

    // assets
    cityscapes: Vec<graphics::Image>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {

        ctx.fs.print_all();

        //let mut cityscape_images = Vec::new();

        //for i in 100..=200 {
        //    let image_path = format!("assets/hjm-cityscapes-v2/hjm-cityscape-v2_{}.png", i); 
        //    let image = graphics::Image::from_path(ctx, image_path).unwrap();
        //    cityscape_images.push(image);
        //}

        let cityscape_images = Vec::new();

        Game {
            hexgrid: hexgrid::Hexgrid::new(ctx, 21, 14, 100.0).unwrap(),

            camera_offset: Point2 { x: 0.0, y: 0.0 },
            dragging: false,
            last_mouse_pos: Point2 { x: 0.0, y: 0.0 },
            velocity: Point2 { x: 0.0, y: 0.0 },
            friction: 0.61,
            screen_coordinates: Point2 { x: 0.0, y: 0.0 },

            window_settings: WindowSettings {
                toggle_fullscreen: false,
                is_fullscreen: false,
                is_resizable: true,
            },

            zoom: 1.0,

            cityscapes: cityscape_images,
        }
    }
}

impl EventHandler for Game {
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        if button == MouseButton::Left {
            self.dragging = true;
            self.last_mouse_pos = Point2 { x, y };
            self.velocity = Point2 { x: 0.0, y: 0.0 };
        }

        if button == MouseButton::Right {
            let adjusted_x = (x - self.camera_offset.x) / self.zoom;
            let adjusted_y = (y - self.camera_offset.y) / self.zoom / FORESHORTENING_FACTOR;
            self.hexgrid.select(adjusted_x, adjusted_y);
        }

        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        if button == MouseButton::Left {
            self.dragging = false;
        }

        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> GameResult{
        if self.dragging {
            let current_pos = Point2 { x, y };
            let delta = Point2{x: current_pos.x - self.last_mouse_pos.x, y: current_pos.y - self.last_mouse_pos.y};

            self.velocity = Point2 { x: self.velocity.x + delta.x, y: self.velocity.y + delta.y };
            self.camera_offset = Point2 { x: self.camera_offset.x + delta.x, y: self.camera_offset.y + delta.y };
            self.last_mouse_pos = current_pos;

            //self.last_mouse_pos = Point2 { x: current_pos.x.clamp(-100.0, 100.0), 
            //    y: current_pos.y.clamp(-100.0, 100.0) }; 
        }

        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) -> GameResult{
        let zoom_change = 0.1;
        if y > 0.0 {
            self.zoom += zoom_change;
        } else if y < 0.0 {
            self.zoom -= zoom_change;
        }
        
        self.zoom = self.zoom.clamp(0.4, 2.0);

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: ggez::input::keyboard::KeyInput) -> GameResult{
        match input.keycode {
            Some(KeyCode::F) => {
                self.window_settings.toggle_fullscreen = true;
                self.window_settings.is_fullscreen = !self.window_settings.is_fullscreen;
            }

            Some(KeyCode::B) => {
                if let Some(selected_index) = self.hexgrid.selected_index {
                    let image_index = rand::random::<usize>() % self.cityscapes.len(); // Assuming rand is imported
                    let image = self.cityscapes[image_index].clone();
                    self.hexgrid.cells[selected_index].image = Some(image);
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //if !self.dragging {
        //    if (self.velocity.x + self.velocity.y) / 2.0 > 0.1 {
        //        self.camera_offset = Point2 { x: self.camera_offset.x + self.velocity.x, y: self.camera_offset.y + self.velocity.y };
        //        self.velocity = Point2 { x: self.velocity.x * self.friction, y: self.velocity.y * self.friction };
        //    } else {
        //        self.velocity = Point2 { x: 0.0, y: 0.0 };
        //    }
        //}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let adjusted_scale = Point2 {
            x: self.zoom,
            y: self.zoom * FORESHORTENING_FACTOR,
        };

        let draw_param = graphics::DrawParam::new()
            .dest(self.camera_offset)
            .scale(adjusted_scale);

        let _ = &self.hexgrid.draw(ctx, &mut canvas, draw_param)?;

        let fps = ctx.time.fps();
        let fps_display = graphics::Text::new(format!("FPS: {:.2}", fps));
        canvas.draw(&fps_display, graphics::DrawParam::default());

        // Draw code here...
        timer::yield_now();
        canvas.finish(ctx)
    }
}
