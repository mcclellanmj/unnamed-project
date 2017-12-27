extern crate ggez;
extern crate nalgebra;

use ggez::audio;
use ggez::conf;
use ggez::event::*;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::timer;
use ggez::event::Keycode;

use std::env;
use std::path;
use std::collections::HashSet;
use std::time::Duration;

/// *********************************************************************
/// Basic stuff, make some helpers for vector functions.
/// ggez includes the nalgebra math library to provide lots of
/// math stuff, we just fill in a couple gaps.
/// **********************************************************************
use nalgebra as na;

struct MainState {
    player: Player,
    screen_width: u32,
    screen_height: u32,
    active_keys: HashSet<Keycode>,
}

#[derive(Debug)]
enum Target {
    Mouse(i32, i32),
    Angle(f32)
}

struct Player {
    position: na::Vector2<f64>,
    angle: na::Rotation2<f64>,
    target: Target,
    image: graphics::Image,
}

fn create_player(ctx: &mut Context) -> Player {
    Player {
        position: na::Vector2::new(20.0, 20.0),
        angle: na::Rotation2::new(270.0_f64.to_radians()),
        target: Target::Angle(90.0_f32.to_radians()),
        image: graphics::Image::new(ctx, "/placeholder-character.png").unwrap()
    }
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (10, 40, 30, 255).into());

        println!("Game resource path: {:?}", ctx.filesystem);

        let s = MainState {
            player: create_player(ctx),
            active_keys: HashSet::new(),
            screen_width: ctx.conf.window_width,
            screen_height: ctx.conf.window_height,
        };

        println!("ctx init, screen size w: {}, h: {}", s.screen_width, s.screen_height);

        Ok(s)
    }

    fn calc_actions(active_keys: &HashSet<Keycode>) -> na::Vector2<f64> {
        let mut velocity: na::Vector2<f64> = na::Vector2::new(0.0, 0.0);
        for &key in active_keys {
            match key {
                Keycode::W => velocity = velocity + na::Vector2::new(0.0, -1.0),
                Keycode::S => velocity = velocity + na::Vector2::new(0.0, 1.0),
                Keycode::A => velocity = velocity + na::Vector2::new(-1.0, 0.0),
                Keycode::D => velocity = velocity + na::Vector2::new(1.0, 0.0),
                _ => {}
            }
        }

        if velocity == na::Vector2::zeros() {
            velocity
        } else {
            na::Unit::new_normalize(velocity).unwrap() * 50_f64
        }
    }

    fn determine_angle_diff(player_position: &na::Vector2<f64>, player_rotation: &na::Rotation2<f64>, angle_target: &Target) -> na::Rotation2<f64> {
        match angle_target {
            &Target::Angle(x) => {
                let target_angle = &na::Rotation2::new(x as f64);
                player_rotation.rotation_to(target_angle)
            },
            &Target::Mouse(x, y) => {
                let target_angle = (x as f64 - player_position[0]).atan2(player_position[1] - y as f64);
                player_rotation.rotation_to(&na::Rotation2::new(target_angle))
            }
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let one_nano: f64 = 1_000_000_000.0;
        let nanos: f64 = dt.as_secs() as f64 + (dt.subsec_nanos() as f64 / one_nano);

        let input_velocity = MainState::calc_actions(&self.active_keys);

        // input_velocity.normalize();

        self.player.position = self.player.position + (self.player.angle * (input_velocity * nanos));

        let angle_diff = MainState::determine_angle_diff(&self.player.position, &self.player.angle, &self.player.target);

        let change = if angle_diff.angle() < 0_f64 {
            -3_f64 * nanos
        } else {
            3_f64 * nanos
        };

        self.player.angle = na::Rotation2::new(self.player.angle.angle() + change);

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        if !repeat {
            self.active_keys.insert(keycode);
        }
    }
    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        if !repeat {
            self.active_keys.remove(&keycode);
        }
    }

    fn mouse_motion_event(&mut self, _state: MouseState, x: i32, y: i32,
                          _xrel: i32, _yrel: i32) {
        self.player.target = Target::Mouse(x, y);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let rect = graphics::Rect::new(self.player.position.data[0] as f32,
                                       self.player.position.data[1] as f32, 3.0, 3.0);

        let player_point = graphics::Point::new(self.player.position[0] as f32, self.player.position[1] as f32);
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.player.image, player_point, self.player.angle.angle() as f32)?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect);
        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let mut config = conf::Conf::new();
    config.window_title = "Unnamed Project".to_string();
    config.window_width = 800;
    config.window_height = 600;

    let ctx = &mut Context::load_from_conf("unnamedProject", "ggez", config).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    /*
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
        println!("Adding path {:?}", path);
    } else {
        println!("aie?");
    }
    */

    match MainState::new(ctx) {
        Err(e) => {
            println!("Failed to start the game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);

            if let Err(e) = result {
                println!("Error encountered while running the game: {}", e);
            } else {
                println!("Game exited normally.")
            }
        }
    }
}
