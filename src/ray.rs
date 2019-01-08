use crate::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
    t: f64,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, t: f64) -> Ray {
        Ray { a, b, t }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    pub fn time(&self) -> f64 {
        self.t
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}
