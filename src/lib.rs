use std::ops;
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    fn length_squared(&self) -> f32 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) -> () {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) -> () {
        *self *= 1.0 / rhs
    }
}
