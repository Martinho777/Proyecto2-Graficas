use nalgebra_glm::Vec3;

use crate::material::Material;
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Hit {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub uv: [f32; 2],
    pub material: Material,
}

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}
