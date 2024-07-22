use std::f64::consts::PI;

use crate::{angle::Angle, vec2d::Vec2d};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn vec(self) -> Vec2d {
        match self {
            Direction::North => Vec2d { x: 0.0, y: 1.0 },
            Direction::South => Vec2d { x: 0.0, y: -1.0 },
            Direction::East => Vec2d { x: 1.0, y: 0.0 },
            Direction::West => Vec2d { x: -1.0, y: 0.0 },
        }
    }

    pub fn angle(self) -> Angle {
        match self {
            Direction::North => Angle::from_radian(PI / 2.0),
            Direction::South => Angle::from_radian(PI * 1.5),
            Direction::East => Angle::from_radian(0.0),
            Direction::West => Angle::from_radian(PI),
        }
    }
}
