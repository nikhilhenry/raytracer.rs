use crate::vector::Vec3;
#[derive(Clone)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, dir: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            dir: dir.clone(),
        }
    }
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn at(&self, t: f32) -> Vec3 {
        (&self.dir * t) + &self.origin
    }
}
