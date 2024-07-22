use crate::vec2d::Vec2d;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Angle {
    degree: f64,
}

const RADIANS_TO_DEGREES: f64 = PI * 180.0;
const DEGREES_TO_RADIANS: f64 = PI / 180.0;
const DEGREE_FULL_CIRCLE: f64 = 360.0;

impl Angle {
    pub const fn from_degree(degree: f64) -> Self {
        Self { degree }
    }

    pub fn from_sin(sin: f64) -> Angle {
        Angle::from_radian(sin.asin())
    }
    pub fn from_cos(cos: f64) -> Angle {
        Angle::from_radian(cos.acos())
    }

    pub fn from_radian(radian: f64) -> Self {
        Self {
            degree: radian * RADIANS_TO_DEGREES,
        }
    }

    pub const fn degree(self) -> f64 {
        self.degree
    }

    pub fn radian(self) -> f64 {
        self.degree * DEGREES_TO_RADIANS
    }

    pub fn normalize(&self) -> Self {
        Self {
            degree: if self.degree >= 0.0 {
                self.degree % DEGREE_FULL_CIRCLE
            } else {
                DEGREE_FULL_CIRCLE + (self.degree % DEGREE_FULL_CIRCLE)
            },
        }
    }

    pub fn cos(self) -> f64 {
        self.radian().cos()
    }
    pub fn sin(self) -> f64 {
        self.radian().sin()
    }

    pub fn vec(&self) -> Vec2d {
        let rad = self.radian();
        Vec2d {
            x: rad.cos(),
            y: rad.sin(),
        }
    }
}

macro_rules! impl_op {
    ($operation: tt, $trait:ident($func:ident), $trait_assign:ident($func_assign:ident)) => {
        impl $trait<Angle> for Angle {
            type Output = Angle;
            fn $func(self, rhs: Angle) -> Self::Output {
                Self { degree: self.degree $operation rhs.degree }
            }
        }
        impl $trait_assign<Angle> for Angle {
            fn $func_assign(&mut self, rhs: Angle) {
                self.degree = self.degree $operation rhs.degree;
            }
        }
    };
}
impl_op!(+, Add(add), AddAssign(add_assign));
impl_op!(-, Sub(sub), SubAssign(sub_assign));
impl_op!(*, Mul(mul), MulAssign(mul_assign));
impl_op!(/, Div(div), DivAssign(div_assign));

#[cfg(test)]
mod test {
    use crate::angle::Angle;

    #[test]
    fn test_normalize() {
        assert_eq!(Angle::from_degree(361.0).normalize().degree(), 1.0);
        assert_eq!(Angle::from_degree(-1.0).normalize().degree(), 359.0);
        assert_eq!(Angle::from_degree(1.0).normalize().degree(), 1.0);
    }
}
