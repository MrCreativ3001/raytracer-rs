use std::cmp::max;

use crate::color::ColorLike;
use crate::consts::render::{FAR_PLANE, NEAR_PLANE};
use crate::direction::Direction;
use crate::ray::HitInfo;
use graphics::color::{BLACK, GREEN};
use graphics::types::Color;
use graphics::{Context, Graphics, Rectangle};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub enum Tile {
    #[default]
    Air,
    Wall,
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        match self {
            Tile::Air => false,
            Tile::Wall => true,
        }
    }
}

impl Tile {
    pub fn render_screen_slice<G>(
        &self,
        x: f64,
        width: f64,
        max_height: f64,
        hit_info: &HitInfo<Tile>,
        g: &mut G,
        ctx: &mut Context,
    ) where
        G: Graphics,
    {
        let distance = ((hit_info.distance - NEAR_PLANE) / FAR_PLANE).max(0.0);
        if distance > 1.0 {
            return;
        }
        let screen_y = max_height - (distance * max_height);
        let screen_height = (max_height * distance) - screen_y;

        match self {
            Tile::Wall => self.render_wall(
                x,
                screen_y,
                width,
                screen_height,
                distance,
                hit_info,
                g,
                ctx,
            ),
            _ => {}
        }
    }

    pub fn render_wall<G>(
        &self,
        screen_x: f64,
        screen_y: f64,
        screen_width: f64,
        screen_height: f64,
        distance: f64,
        hit_info: &HitInfo<Tile>,
        g: &mut G,
        ctx: &mut Context,
    ) where
        G: Graphics,
    {
        let rect = [screen_x, screen_y, screen_width, screen_height];

        let color = GREEN
            .mul_other(match hit_info.hit_direction {
                Direction::North | Direction::South => [0.9, 0.9, 0.9, 1.0],
                Direction::East | Direction::West => [0.95, 0.95, 0.95, 1.0],
            })
            .mul_other(Color::new_single(1.0 - distance as f32));

        g.rectangle(&Rectangle::new(color), rect, &ctx.draw_state, ctx.transform);
    }

    pub fn render_minimap<G>(&self, g: &mut G, ctx: &mut Context, rect: graphics::types::Rectangle)
    where
        G: Graphics,
    {
        match self {
            Tile::Air => {}
            Tile::Wall => g.rectangle(&Rectangle::new(BLACK), rect, &ctx.draw_state, ctx.transform),
        }
    }
}
