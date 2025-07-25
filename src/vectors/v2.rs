use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

trait Norm {
    fn norm(self) -> f32;
    fn sqr_norm(self) -> f32;
    fn normalized(self) -> Self;
    fn normalize(&mut self);
}

trait Distance {
    fn distance_to(self, other: V2) -> f32;
    fn sqr_distance_to(self, other: V2) -> f32;
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct V2 {
    x: f32,
    y: f32,
}
impl V2 {
    fn new(x: f32, y: f32) -> V2 {
        V2 {x: x, y: y}
    }
    fn zero() -> V2 {
        V2::new(0.0, 0.0)
    }
    fn up() -> V2 {
        V2::new(0.0, -1.0)
    }
    fn down() -> V2 {
        V2::new(0.0, 1.0)
    }
    fn left() -> V2 {
        V2::new(-1.0, 0.0)
    }
    fn right() -> V2 {
        V2::new(1.0, 1.0)
    }
}

impl Add for V2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for V2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl Sub for V2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign for V2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl Mul for V2 {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}
impl Mul<f32> for V2 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl MulAssign<f32> for V2 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}
impl Norm for V2 {
    fn norm(self) -> f32 {
        self.sqr_norm().sqrt()
    }
    fn sqr_norm(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    fn normalized(self) -> Self {
        if self.norm() > 0.0 {
            Self {
                x: self.x / self.norm(),
                y: self.y / self.norm(),
            }
        } else {
            self
        }
    }
    fn normalize(&mut self) {
        let vec = self.normalized();
        self.x = vec.x;
        self.y = vec.y;
    }
}
impl Distance for V2 {
    fn distance_to(self, other: V2) -> f32 {
        self.sqr_distance_to(other).sqrt()
    }
    fn sqr_distance_to(self, other: V2) -> f32 {
        let dist = self - other;
        dist.sqr_norm().abs()
    }
}

fn lerp_v2(a: V2, b:V2, t:f32) -> V2{
    if t > 1.0 {
        panic!("Vector linear interpolation out of range -> t must be in range 0..1");
    }
    if t < 0.0 {
        panic!("Vector linear interpolation out of range -> t must be in range 0..1");
    }
    (a * (1.0 - t)) + (b * t)
}