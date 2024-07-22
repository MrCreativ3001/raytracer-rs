use crate::angle::Angle;
use crate::consts::render::{
    CHANGE_ANGLE_PER_RAY, CHANGE_SCREEN_X_PER_RAY, FOV, MINIMAP_HEIGHT, MINIMAP_WIDTH,
    RAY_SCREEN_HEIGHT, RAY_SCREEN_WIDTH,
};
use crate::direction::Direction;
use crate::player::Player;
use crate::ray::{HitInfo, Ray, Raycastable};
use crate::vec2d::Vec2d;
use crate::world::tile::Tile;
use crate::world::World;
use conv::ConvAsUtil;
use graphics::color::{BLUE, RED, WHITE};
use graphics::types::Color;
use graphics::{Context, Graphics, Line, Rectangle};

impl World {
    pub fn render<G>(&self, g: &mut G, ctx: &mut Context, player: &Player)
    where
        G: Graphics,
    {
        let ray_screen_width = RAY_SCREEN_WIDTH;
        let ray_screen_height = RAY_SCREEN_HEIGHT;
        let mut ray_screen_x = 0.0;
        let mut degree = Angle::from_degree(0.0);
        while degree <= FOV {
            degree = Angle::from_degree(degree.degree() + CHANGE_ANGLE_PER_RAY.degree());
            let ray_angle = degree + player.direction;
            ray_screen_x += CHANGE_SCREEN_X_PER_RAY;

            let ray = Ray {
                pos: player.pos,
                angle: ray_angle - Angle::from_degree(FOV.degree() / 2.0),
            };
            if let Some(hit_info) = self.cast_ray(ray) {
                if let Some(tile) = hit_info.hit_object {
                    tile.render_screen_slice(
                        ray_screen_x,
                        ray_screen_width,
                        ray_screen_height,
                        &hit_info,
                        g,
                        ctx,
                    );
                    // // Debug for rendering rays on the minimap
                    // let tile_width = MINIMAP_WIDTH / World::WIDTH as f64;
                    // let tile_height = MINIMAP_HEIGHT / World::HEIGHT as f64;
                    // let player_screen_pos = Vec2d {
                    //     x: player.pos.x * tile_width,
                    //     y: player.pos.y * tile_height,
                    // };
                    // g.line(
                    //     &Line::new(WHITE, 0.5),
                    //     [
                    //         player_screen_pos.x,
                    //         player_screen_pos.y,
                    //         hit_info.hit.x * tile_width,
                    //         hit_info.hit.y * tile_height,
                    //     ],
                    //     &ctx.draw_state,
                    //     ctx.transform,
                    // );
                }
            }
        }
    }
}

impl Raycastable for World {
    type HitObject = Tile;

    fn cast_ray(&self, ray: Ray) -> Option<HitInfo<'_, Self::HitObject>> {
        self.cast_efficient(ray)
    }
}

impl World {
    fn cast_simple(&self, ray: Ray) -> Option<HitInfo<'_, Tile>> {
        const OFFSET: f64 = 0.1;
        let mut pos = ray.pos;
        let mut dist = 0.0;
        let off = ray.angle.normalize().vec().with_magnitude(OFFSET);
        let dist_off = off.magnitude();

        while let Some(tile) = self.tile_vec(pos) {
            if tile.is_solid() {
                return Some(HitInfo {
                    distance: dist,
                    hit: pos,
                    hit_object: Some(tile),
                    hit_direction: {
                        let next_boundary_x = pos.x.floor() + if off.x > 0.0 { 1.0 } else { 0.0 };
                        let next_boundary_y = pos.y.floor() + if off.y > 0.0 { 1.0 } else { 0.0 };

                        let boundary_dist = (pos
                            - Vec2d {
                                x: next_boundary_x,
                                y: next_boundary_y,
                            })
                        .abs();

                        let dist_x_bigger_y = boundary_dist.x > boundary_dist.y;
                        let x_positive = off.x > 0.0;
                        match (dist_x_bigger_y, x_positive) {
                            (true, true) => Direction::East,
                            (true, false) => Direction::West,
                            (false, true) => Direction::South,
                            (false, false) => Direction::North,
                        }
                    },
                });
            }
            pos += off;
            dist += dist_off;
        }
        None
    }

    fn cast_efficient(&self, ray: Ray) -> Option<HitInfo<'_, Tile>> {
        // casting on x axis:
        let x_hit = {
            let direction = ray.angle.vec();
            let direction_x = direction.x / direction.x.abs();
            let next_boundary_x = ray.pos.x.floor() + if direction.x > 0.0 { 1.0 } else { 0.0 };
            let per_x_tile_offset = direction / direction.x.abs();

            let next_boundary_dist_x = (ray.pos.x - next_boundary_x).abs();
            let mut next_boundary_hit =
                ray.pos + direction.scale_x(next_boundary_dist_x) * direction_x;
            // because of precision errors we need to sub a tiny value when going negative
            if direction_x < 0.0 {
                next_boundary_hit.x -= 0.0001;
            }

            let mut result = None;
            while let Some(tile) = self.tile_vec(next_boundary_hit) {
                if tile.is_solid() {
                    result = Some(HitInfo {
                        distance: ray.pos.distance_to(next_boundary_hit),
                        hit: next_boundary_hit,
                        hit_object: Some(tile),
                        hit_direction: if direction_x > 0.0 {
                            Direction::East
                        } else {
                            Direction::West
                        },
                    });
                    break;
                }

                next_boundary_hit += per_x_tile_offset;
            }
            result
        };
        // casting on y axis:
        let y_hit = {
            let direction = ray.angle.vec();
            let direction_y = direction.y / direction.y.abs();
            let next_boundary_y = ray.pos.y.floor() + if direction.y > 0.0 { 1.0 } else { 0.0 };
            let per_y_tile_offset = direction / direction.y.abs();

            let next_boundary_dist_y = (ray.pos.y - next_boundary_y).abs();
            let mut next_boundary_hit =
                ray.pos + direction.scale_y(next_boundary_dist_y) * direction_y;
            // because of precision errors we need to sub a tiny value when going negative
            if direction_y < 0.0 {
                next_boundary_hit.y -= 0.0001;
            }

            let mut result = None;
            while let Some(tile) = self.tile_vec(next_boundary_hit) {
                if tile.is_solid() {
                    result = Some(HitInfo {
                        distance: ray.pos.distance_to(next_boundary_hit),
                        hit: next_boundary_hit,
                        hit_object: Some(tile),
                        hit_direction: if direction_y > 0.0 {
                            Direction::North
                        } else {
                            Direction::South
                        },
                    });
                    break;
                }

                next_boundary_hit += per_y_tile_offset;
            }
            result
        };

        match (x_hit, y_hit) {
            (Some(x_hit), Some(y_hit)) => {
                if x_hit.distance < y_hit.distance {
                    Some(x_hit)
                } else {
                    Some(y_hit)
                }
            }
            (Some(x_hit), None) => Some(x_hit),
            (None, Some(y_hit)) => Some(y_hit),
            (None, None) => None,
        }
    }
}

impl World {
    pub fn render_mini_map<G>(&self, g: &mut G, ctx: &mut Context, player: &Player)
    where
        G: Graphics,
    {
        // Render background
        g.rectangle(
            &Rectangle::new(BLUE),
            [0.0, 0.0, MINIMAP_WIDTH, MINIMAP_HEIGHT],
            &ctx.draw_state,
            ctx.transform,
        );

        // Render world
        let tile_width = MINIMAP_WIDTH / World::WIDTH as f64;
        let tile_height = MINIMAP_HEIGHT / World::HEIGHT as f64;
        for x in 0..World::WIDTH {
            for y in 0..World::HEIGHT {
                let tile = self.tile(x, y);
                if let Some(tile) = tile {
                    tile.render_minimap(
                        g,
                        ctx,
                        [
                            (x as f64) * tile_width,
                            (y as f64) * tile_height,
                            tile_width,
                            tile_height,
                        ],
                    );
                }
            }
        }
        // Render player
        const PLAYER_SIZE: f64 = 5.0;
        let player_screen_pos = Vec2d {
            x: player.pos.x * tile_width,
            y: player.pos.y * tile_height,
        };
        g.rectangle(
            &Rectangle::new(WHITE),
            [
                player_screen_pos.x,
                player_screen_pos.y,
                PLAYER_SIZE,
                PLAYER_SIZE,
            ],
            &ctx.draw_state,
            ctx.transform,
        );
        let player_screen_pos = player_screen_pos + (PLAYER_SIZE / 2.0);
        let line_pos = player_screen_pos + player.direction.vec() * 10.0;
        g.line(
            &Line::new(RED, 1.0),
            [
                player_screen_pos.x,
                player_screen_pos.y,
                line_pos.x,
                line_pos.y,
            ],
            &ctx.draw_state,
            ctx.transform,
        );
    }
}
