extern crate vec_lib;
use vec_lib::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub a: Vector3,
    pub b: Vector3,
}

impl Ray {
    pub fn origin(&mut self) -> Vector3 {
        self.a
    }
    pub fn direction(&mut self) -> Vector3 {
        self.b
    }
    pub fn point_at_parameter(&mut self, t: f64) -> Vector3 {
        self.a + t*self.b
    }
}
