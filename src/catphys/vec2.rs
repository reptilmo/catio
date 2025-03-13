use std::cmp;
use std::ops;

pub const EPSILON: f32 = f32::EPSILON * 10.0;

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    #[inline]
    pub fn nearly_zero(&self) -> bool {
        f32::abs(self.x) <= EPSILON && f32::abs(self.y) <= EPSILON
    }

    #[allow(dead_code)]
    #[inline]
    pub fn nearly_eq(&self, other: Vec2) -> bool {
        f32::abs(self.x - other.x) <= EPSILON && f32::abs(self.y - other.y) <= EPSILON
    }

    #[inline]
    pub fn magnitude2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.magnitude2())
    }

    #[inline]
    pub fn unit(&self) -> Vec2 {
        let magnitude = self.magnitude();

        if magnitude >= 0.0 {
            return *self * (1.0 / self.magnitude());
        }

        Vec2::new(0.0, 0.0)
    }

    #[inline]
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn reflect(&self, normal: Vec2) -> Vec2 {
        // direction (self) is pointing in, normal is pointing out
        *self - (normal * 2.0 * self.dot(normal))
    }

    pub fn rotate(&self, rads: f32) -> Vec2 {
        Vec2 {
            x: self.x * rads.cos() - self.y * rads.sin(),
            y: self.x * rads.sin() + self.y * rads.cos(),
        }
    }
}

impl cmp::PartialEq for Vec2 {
    #[inline]
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl cmp::Eq for Vec2 {}

impl ops::Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Vec2 {
        Vec2::new(self.x * other, self.y * other)
    }
}

impl ops::MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Vec2 {
        let factor = 1.0 / other;
        Vec2::new(self.x * factor, self.y * factor)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;
    #[test]
    fn add() {
        let a = Vec2::new(10.0, 25.0);
        let b = Vec2::new(12.0, 10.0);
        let c = Vec2::new(22.0, 35.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = Vec2::new(22.0, 35.0);
        let b = Vec2::new(12.0, 10.0);
        let c = Vec2::new(10.0, 25.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn nearly_equal() {
        let a = Vec2::new(1.2512, 1.5519);
        let b = Vec2::new(2.3701, 1.5501);
        let c = Vec2::new(3.6213, 3.102);
        assert!((a + b).nearly_eq(c));
    }

    #[test]
    fn mul() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(5.0, 10.0);
        assert_eq!(a * 5.0, b);
    }

    #[test]
    fn div() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(0.5, 1.0);
        assert_eq!(a / 2.0, b);
    }

    #[test]
    fn rotate() {
        let a = Vec2::new(1.0, 0.0);
        let b = a.rotate(std::f32::consts::PI);
        assert_eq!(b.x, -1.0);
        assert!(b.y.abs() <= std::f32::EPSILON); // Rust has nothing for this :/
        let c = a.rotate(std::f32::consts::PI * 0.5);
        assert!(c.x.abs() <= std::f32::EPSILON);
        assert_eq!(c.y, 1.0);
    }
}
