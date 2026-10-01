use nalgebra_glm::{cross, dot, normalize, Vec3};

use crate::material::Material;
use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

pub struct Star {
    pub center: Vec3,
    pub outer_radius: f32,
    pub inner_radius: f32,
    pub depth: f32,
    pub material: Material,
}

impl Star {
    fn vertices(&self) -> Vec<Vec3> {
        (0..10)
            .map(|index| {
                let angle =
                    std::f32::consts::FRAC_PI_2 + index as f32 * std::f32::consts::TAU / 10.0;
                let radius = if index % 2 == 0 {
                    self.outer_radius
                } else {
                    self.inner_radius
                };
                Vec3::new(
                    self.center.x + angle.cos() * radius,
                    self.center.y + angle.sin() * radius,
                    self.center.z,
                )
            })
            .collect()
    }

    fn contains_xy(&self, point: &Vec3, vertices: &[Vec3]) -> bool {
        let mut inside = false;
        for index in 0..vertices.len() {
            let current = &vertices[index];
            let next = &vertices[(index + 1) % vertices.len()];
            if (current.y > point.y) != (next.y > point.y) {
                let x =
                    (next.x - current.x) * (point.y - current.y) / (next.y - current.y) + current.x;
                if point.x < x {
                    inside = !inside;
                }
            }
        }
        inside
    }

    fn triangle_hit(&self, ray: &Ray, a: Vec3, b: Vec3, c: Vec3) -> Option<Hit> {
        let edge_one = b - a;
        let edge_two = c - a;
        let perpendicular = cross(&ray.direction, &edge_two);
        let determinant = dot(&edge_one, &perpendicular);
        if determinant.abs() < 1e-7 {
            return None;
        }
        let inverse = 1.0 / determinant;
        let to_origin = ray.origin - a;
        let u = dot(&to_origin, &perpendicular) * inverse;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = cross(&to_origin, &edge_one);
        let v = dot(&ray.direction, &q) * inverse;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let distance = dot(&edge_two, &q) * inverse;
        if distance <= 0.001 {
            return None;
        }
        let point = ray.at(distance);
        let normal = normalize(&cross(&edge_one, &edge_two));
        Some(Hit {
            distance,
            point,
            normal,
            uv: [
                (point.x - self.center.x) / (self.outer_radius * 2.0) + 0.5,
                (point.y - self.center.y) / (self.outer_radius * 2.0) + 0.5,
            ],
            material: self.material.clone(),
        })
    }
}

impl Primitive for Star {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let vertices = self.vertices();
        let front_z = self.center.z + self.depth * 0.5;
        let back_z = self.center.z - self.depth * 0.5;
        let mut hits = Vec::new();

        if ray.direction.z.abs() > 1e-7 {
            for (z, normal) in [
                (front_z, Vec3::new(0.0, 0.0, 1.0)),
                (back_z, Vec3::new(0.0, 0.0, -1.0)),
            ] {
                let distance = (z - ray.origin.z) / ray.direction.z;
                if distance > 0.001 {
                    let point = ray.at(distance);
                    if self.contains_xy(&point, &vertices) {
                        hits.push(Hit {
                            distance,
                            point,
                            normal,
                            uv: [
                                (point.x - self.center.x) / (self.outer_radius * 2.0) + 0.5,
                                (point.y - self.center.y) / (self.outer_radius * 2.0) + 0.5,
                            ],
                            material: self.material.clone(),
                        });
                    }
                }
            }
        }

        for index in 0..vertices.len() {
            let next = (index + 1) % vertices.len();
            let front_a = vertices[index] + Vec3::new(0.0, 0.0, self.depth * 0.5);
            let front_b = vertices[next] + Vec3::new(0.0, 0.0, self.depth * 0.5);
            let back_a = vertices[index] - Vec3::new(0.0, 0.0, self.depth * 0.5);
            let back_b = vertices[next] - Vec3::new(0.0, 0.0, self.depth * 0.5);
            if let Some(hit) = self.triangle_hit(ray, front_a, front_b, back_a) {
                hits.push(hit);
            }
            if let Some(hit) = self.triangle_hit(ray, front_b, back_b, back_a) {
                hits.push(hit);
            }
        }

        hits.into_iter()
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}
