use conv::{ApproxFrom, ApproxInto, ValueFrom, ValueInto};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Debug)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn with_magnitude(self, magnitude: f64) -> Self {
        let current_magnitude = self.magnitude();
        let scalar = magnitude / current_magnitude;
        self * scalar
    }

    pub fn distance_to(self, to: Self) -> f64 {
        (self - to).magnitude()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        self / magnitude
    }

    pub fn scale_x(self, new_x: f64) -> Self {
        let factor = new_x / self.x;
        self * factor
    }
    pub fn scale_y(self, new_y: f64) -> Self {
        let factor = new_y / self.y;
        self * factor
    }

    pub fn with_x(self, x: f64) -> Self {
        Self { x, y: self.y }
    }
    pub fn with_y(self, y: f64) -> Self {
        Self { x: self.x, y }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
}

macro_rules! impl_op {
    ($operation: tt, $trait:ident($func:ident), $trait_assign:ident($func_assign:ident)) => {
        impl $trait<Vec2d> for Vec2d {
            type Output = Vec2d;
            fn $func(self, rhs: Vec2d) -> Self::Output {
                Self {
                    x: self.x $operation rhs.x,
                    y: self.y $operation rhs.y,
                }
            }
        }
        impl $trait_assign<Vec2d> for Vec2d {
            fn $func_assign(&mut self, rhs: Vec2d) {
                self.x = self.x $operation rhs.x;
                self.y = self.y $operation rhs.y;
            }
        }
    };
    ($operation: tt, $trait:ident($func:ident), $trait_assign:ident($func_assign:ident), $rhs:ty) => {
        impl $trait<$rhs> for Vec2d {
            type Output = Vec2d;
            fn $func(self, rhs: $rhs) -> Self::Output {
                Self {
                    x: self.x $operation rhs,
                    y: self.y $operation rhs,
                }
            }
        }
        impl $trait_assign<$rhs> for Vec2d {
            fn $func_assign(&mut self, rhs: $rhs) {
                self.x = self.x $operation rhs;
                self.y = self.y $operation rhs;
            }
        }
    };
}

impl_op!(+, Add(add), AddAssign(add_assign));
impl_op!(+, Add(add), AddAssign(add_assign), f64);
impl_op!(-, Sub(sub), SubAssign(sub_assign));
impl_op!(-, Sub(sub), SubAssign(sub_assign), f64);
impl_op!(*, Mul(mul), MulAssign(mul_assign));
impl_op!(*, Mul(mul), MulAssign(mul_assign), f64);
impl_op!(/, Div(div), DivAssign(div_assign));
impl_op!(/, Div(div), DivAssign(div_assign), f64);

impl From<[f64; 2]> for Vec2d {
    fn from(value: [f64; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl Into<[f64; 2]> for Vec2d {
    fn into(self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl<N> ValueFrom<[N; 2]> for Vec2d
where
    N: ValueInto<f64>,
{
    type Err = N::Err;

    fn value_from([x, y]: [N; 2]) -> Result<Self, Self::Err> {
        Ok(Self {
            x: x.value_into()?,
            y: y.value_into()?,
        })
    }
}

impl<N> ValueInto<[N; 2]> for Vec2d
where
    N: ValueFrom<f64>,
{
    type Err = N::Err;

    fn value_into(self) -> Result<[N; 2], Self::Err> {
        Ok([self.x.value_into()?, self.y.value_into()?])
    }
}

impl<N> ApproxFrom<[N; 2]> for Vec2d
where
    N: ApproxInto<f64>,
{
    type Err = N::Err;

    fn approx_from([x, y]: [N; 2]) -> Result<Self, Self::Err> {
        Ok(Self {
            x: x.approx_into()?,
            y: y.approx_into()?,
        })
    }
}

impl<N> ApproxInto<[N; 2]> for Vec2d
where
    N: ApproxFrom<f64>,
{
    type Err = N::Err;

    fn approx_into(self) -> Result<[N; 2], Self::Err> {
        Ok([self.x.approx_into()?, self.y.approx_into()?])
    }
}
