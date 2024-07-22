use crate::angle::Angle;
use crate::direction::Direction;
use crate::vec2d::Vec2d;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub pos: Vec2d,
    pub angle: Angle,
}

#[derive(PartialEq, Debug)]
pub struct HitInfo<'a, Obj> {
    pub distance: f64,
    pub hit: Vec2d,
    pub hit_object: Option<&'a Obj>,
    pub hit_direction: Direction,
}

pub trait Raycastable {
    type HitObject;
    fn cast_ray(&self, ray: Ray) -> Option<HitInfo<'_, Self::HitObject>>;
}
