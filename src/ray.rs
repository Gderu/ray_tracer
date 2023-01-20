pub use crate::vec3::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn zeros() -> Self {
        Ray {
            orig: Point3::zeros(),
            dir: Vec3::zeros(),
        }
    }

    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray {orig, dir}
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}