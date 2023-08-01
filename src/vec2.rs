use std::ops::{Add, Mul, Div, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn mul_scalar(self, mul: &T) -> Self {
        Vec2 {
            // i hate this
            x: self.x * *mul,
            y: self.y * *mul,
        }
    }

}

impl Vec2<f32> {
    pub fn len(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {

    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }

}

// TODO: impl Sub<T>, Add<T>, Mul<T> and Div<T> for Vec2<T>

impl<T: Sub<Output = T>> Sub for Vec2<T> {

    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {

    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}