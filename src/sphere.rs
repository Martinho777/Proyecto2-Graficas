use nalgebra_glm::{dot, normalize, Vec3};

use crate::material::Material;
use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let offset = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&offset, &ray.direction);
        let c = dot(&offset, &offset) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let near = (-b - root) / (2.0 * a);
        let far = (-b + root) / (2.0 * a);
        let distance = if near > 0.001 {
            near
        } else if far > 0.001 {
            far
        } else {
            return None;
        };

        let point = ray.at(distance);
        let normal = normalize(&(point - self.center));
        let u = 0.5 + normal.z.atan2(normal.x) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - normal.y.asin() / std::f32::consts::PI;

        Some(Hit {
            distance,
            point,
            normal,
            uv: [u, v],
            material: self.material.clone(),
        })
    }
}
