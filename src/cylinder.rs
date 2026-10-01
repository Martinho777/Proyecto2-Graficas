use nalgebra_glm::{normalize, Vec3};

use crate::material::Material;
use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

pub struct Cylinder {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub material: Material,
}

impl Cylinder {
    fn hit_side(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let relative = ray.origin - self.center;
        let a = ray.direction.x * ray.direction.x + ray.direction.y * ray.direction.y;
        if a.abs() < 1e-8 {
            return None;
        }

        let b = 2.0 * (relative.x * ray.direction.x + relative.y * ray.direction.y);
        let c = relative.x * relative.x + relative.y * relative.y - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let roots = [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)];
        for distance in roots {
            if distance < t_min || distance > t_max {
                continue;
            }
            let point = ray.at(distance);
            let half_height = self.height * 0.5;
            if point.z < self.center.z - half_height || point.z > self.center.z + half_height {
                continue;
            }
            let radial = Vec3::new(point.x - self.center.x, point.y - self.center.y, 0.0);
            let normal = normalize(&radial);
            let u = 0.5 + radial.y.atan2(radial.x) / (2.0 * std::f32::consts::PI);
            let v = (point.z - (self.center.z - half_height)) / self.height;
            return Some(Hit {
                distance,
                point,
                normal,
                uv: [u, v],
                material: self.material.clone(),
            });
        }

        None
    }

    fn hit_cap(&self, ray: &Ray, z: f32, normal: Vec3, t_min: f32, t_max: f32) -> Option<Hit> {
        if ray.direction.z.abs() < 1e-8 {
            return None;
        }
        let distance = (z - ray.origin.z) / ray.direction.z;
        if distance < t_min || distance > t_max {
            return None;
        }
        let point = ray.at(distance);
        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;
        if dx * dx + dy * dy > self.radius * self.radius {
            return None;
        }
        Some(Hit {
            distance,
            point,
            normal,
            uv: [
                0.5 + dx / (2.0 * self.radius),
                0.5 + dy / (2.0 * self.radius),
            ],
            material: self.material.clone(),
        })
    }
}

impl Primitive for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let half_height = self.height * 0.5;
        let top = self.center.z + half_height;
        let bottom = self.center.z - half_height;
        let side = self.hit_side(ray, 0.001, f32::INFINITY);
        let top_hit = self.hit_cap(ray, top, Vec3::new(0.0, 0.0, 1.0), 0.001, f32::INFINITY);
        let bottom_hit = self.hit_cap(ray, bottom, Vec3::new(0.0, 0.0, -1.0), 0.001, f32::INFINITY);

        [side, top_hit, bottom_hit]
            .into_iter()
            .flatten()
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}
