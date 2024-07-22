use crate::angle::Angle;
use crate::consts::window::WINDOW_SIZE;
use crate::player::Player;
use crate::vec2d::Vec2d;
use crate::world::World;
use glutin_window::OpenGL;
use graphics::color::BLUE;
use graphics::{Context, Graphics, Rectangle};
use opengl_graphics::GlGraphics;
use piston::{ButtonArgs, RenderArgs, UpdateArgs};

pub struct Game {
    graphics: GlGraphics,
    pub player: Player,
    pub world: World,
}

impl Game {
    pub fn new(opengl: OpenGL) -> Self {
        Self {
            graphics: GlGraphics::new(opengl),
            player: Default::default(),
            world: World::default(),
        }
    }
    pub fn from_world(opengl: OpenGL, player_pos: Vec2d, world: World) -> Self {
        Self {
            graphics: GlGraphics::new(opengl),
            player: Player::new(player_pos, Angle::from_degree(0.0)),
            world,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.graphics
            .draw(args.viewport(), |mut ctx: Context, g: &mut GlGraphics| {
                // Clear screen with blue
                g.rectangle(
                    &Rectangle::new(BLUE),
                    [0.0, 0.0, WINDOW_SIZE[0], WINDOW_SIZE[1]],
                    &ctx.draw_state,
                    ctx.transform,
                );
                // render world
                self.world.render(g, &mut ctx, &self.player);
                // render minimap
                self.world.render_mini_map(g, &mut ctx, &self.player);
            });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.player.update(&self.world);
        self.world.update();
    }

    pub fn button(&mut self, args: &ButtonArgs) {
        self.player.button(args);
    }
}
