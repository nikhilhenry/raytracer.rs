use std::ops;
#[derive(Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[macro_export]
macro_rules! print_vec {
    ($a:item) => {
        println!("{} {} {}", item.x, item.y, item.z)
    };
}

#[macro_export]
macro_rules! write_color {
    ($a:expr) => {
        const COLOR_VALUE: f32 = 255.999;
        println!(
            "{} {} {}",
            ($a['x'] * COLOR_VALUE) as u32,
            ($a['y'] * COLOR_VALUE) as u32,
            ($a['z'] * COLOR_VALUE) as u32,
        )
    };
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
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

impl ops::Index<char> for Vec3 {
    type Output = f32;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.x,
            'y' => &self.y,
            'z' => &self.z,
            _ => panic!("Incorrect vec index"),
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        return Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}
impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        return Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}
impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[inline]
pub fn dot(v: &Vec3, u: &Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}
#[inline]
pub fn cross(v: &Vec3, u: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}
