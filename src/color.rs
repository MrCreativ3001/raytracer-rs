use std::ops::{Add, Div, Mul, Sub};

use graphics::types::{Color, ColorComponent};

macro_rules! impl_do {
    ($fn_name:ident, $fn_name_other:ident, $fn_name_single:ident : $op:tt) => {
        fn $fn_name(&self, r: Self::N, g: Self::N, b: Self::N, a: Self::N) -> Self {
            Self::new(self.r() $op r, self.g() $op g, self.b() $op b, self.a() $op a)
        }
        fn $fn_name_other(&self, other: Self) -> Self {
            self.$fn_name(other.r(), other.g(), other.b(), other.a())
        }
        fn $fn_name_single(&self, v: Self::N) -> Self {
            self.$fn_name(v, v, v, v)
        }
    };
}

pub trait ColorLike: Sized + Copy {
    type N: Copy
        + Add<Self::N, Output = Self::N>
        + Sub<Self::N, Output = Self::N>
        + Mul<Self::N, Output = Self::N>
        + Div<Self::N, Output = Self::N>;

    fn new(r: Self::N, g: Self::N, b: Self::N, a: Self::N) -> Self;
    fn new_rgb(r: Self::N, g: Self::N, b: Self::N) -> Self;
    fn new_single(v: Self::N) -> Self {
        Self::new_rgb(v, v, v)
    }
    fn r(&self) -> Self::N;
    fn g(&self) -> Self::N;
    fn b(&self) -> Self::N;
    fn a(&self) -> Self::N;

    impl_do!(add, add_other, add_single: +);
    impl_do!(sub, sub_other, sub_single: -);
    impl_do!(mul, mul_other, mul_single: *);
    impl_do!(div, div_other, div_single: /);
}

impl ColorLike for Color {
    type N = ColorComponent;

    fn new(r: Self::N, g: Self::N, b: Self::N, a: Self::N) -> Self {
        [r, g, b, a]
    }
    fn new_rgb(r: Self::N, g: Self::N, b: Self::N) -> Self {
        [r, g, b, 1.0]
    }

    fn r(&self) -> Self::N {
        self[0]
    }
    fn g(&self) -> Self::N {
        self[1]
    }
    fn b(&self) -> Self::N {
        self[2]
    }
    fn a(&self) -> Self::N {
        self[3]
    }
}
