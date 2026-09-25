use nalgebra_glm::Vec3;

use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}
