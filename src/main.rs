extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate slab;

use crate::consts::window::{WINDOW_NAME, WINDOW_SIZE};
use crate::game::Game;
use crate::vec2d::Vec2d;
use crate::world::tile::Tile;
use crate::world::tile::Tile::{Air, Wall};
use crate::world::World;
use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::{ButtonEvent, EventSettings, Events, RenderEvent, UpdateEvent, WindowSettings};

pub mod angle;
pub mod color;
pub mod consts;
pub mod direction;
pub mod game;
pub mod player;
pub mod ray;
pub mod vec2d;
pub mod world;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(WINDOW_NAME, WINDOW_SIZE)
        .graphics_api(opengl)
        .resizable(false)
        .build()
        .expect("Unable to build window");

    let mut game = Game::from_world(
        opengl,
        Vec2d { x: 5.0, y: 5.0 },
        World::from_tiles(TEST_WORLD),
    );

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            game.render(&args);
        }

        if let Some(args) = event.update_args() {
            game.update(&args);
        }

        if let Some(args) = event.button_args() {
            game.button(&args);
        }
    }
}

const TEST_WORLD: [[Tile; 10]; 10] = [
    [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
    [Wall, Air, Air, Air, Air, Air, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Air, Air, Air, Wall],
    [Wall, Air, Wall, Air, Air, Air, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Air, Air, Air, Wall],
    [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
];
