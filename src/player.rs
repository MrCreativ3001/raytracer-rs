use crate::angle::Angle;
use crate::consts::controls::*;
use crate::consts::player::{MOVE_SPEED, TURN_SPEED};
use crate::vec2d::Vec2d;
use crate::world::tile::Tile;
use crate::world::World;
use piston::{Button, ButtonArgs, ButtonState};

#[derive(Default, Debug)]
pub struct Player {
    pub pos: Vec2d,
    pub direction: Angle,
    pressed_buttons: [bool; 4],
}

const BTN_FORWARD: usize = 0;
const BTN_BACKWARD: usize = 1;
const BTN_TURN_RIGHT: usize = 2;
const BTN_TURN_LEFT: usize = 3;

impl Player {
    pub fn new(pos: Vec2d, direction: Angle) -> Self {
        Self {
            pos,
            direction,
            pressed_buttons: [false; 4],
        }
    }

    pub fn button(&mut self, args: &ButtonArgs) {
        let press = args.state == ButtonState::Press;
        match args.button {
            Button::Keyboard(FORWARD) => self.pressed_buttons[BTN_FORWARD] = press,
            Button::Keyboard(BACKWARD) => self.pressed_buttons[BTN_BACKWARD] = press,
            Button::Keyboard(TURN_RIGHT) => self.pressed_buttons[BTN_TURN_RIGHT] = press,
            Button::Keyboard(TURN_LEFT) => self.pressed_buttons[BTN_TURN_LEFT] = press,
            _ => {}
        }
    }

    pub fn update(&mut self, world: &World) {
        let prev_collision = Self::is_colliding(self.pos, world);

        let mut movement = Vec2d::default();
        if self.pressed_buttons[BTN_FORWARD] {
            let move_direction = self.direction.vec() * MOVE_SPEED;
            movement += move_direction;
        }
        if self.pressed_buttons[BTN_BACKWARD] {
            let move_direction = self.direction.vec() * -MOVE_SPEED;
            movement += move_direction;
        }

        // only check collisions when we're not in a solid (to allow the player to escape)
        if !prev_collision {
            let collision_only_x = Self::is_colliding(self.pos + movement.with_y(0.0), world);
            let collision_only_y = Self::is_colliding(self.pos + movement.with_x(0.0), world);
            let collision_both = Self::is_colliding(self.pos + movement, world);

            if !collision_both {
                self.pos += movement;
            } else if !collision_only_x {
                self.pos += movement.with_y(0.0);
            } else if !collision_only_y {
                self.pos += movement.with_x(0.0);
            }
        } else {
            self.pos += movement;
        }

        if self.pressed_buttons[BTN_TURN_RIGHT] {
            self.direction = self.direction + TURN_SPEED;
        }
        if self.pressed_buttons[BTN_TURN_LEFT] {
            self.direction = self.direction - TURN_SPEED;
        }
    }

    fn is_colliding(pos: Vec2d, world: &World) -> bool {
        world
            .tile_vec(pos)
            .map(|tile| tile.is_solid())
            .unwrap_or(true) // when we're out of the map this should be solid
    }
}
