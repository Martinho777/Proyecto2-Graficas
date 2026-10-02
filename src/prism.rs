use nalgebra_glm::{cross, dot, normalize, Vec3};

use crate::material::Material;
use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

/// Prisma triangular extruido sobre el eje Y.
/// Los tres puntos describen el triangulo visto desde arriba (X/Z), mientras
/// que `half_height` agrega volumen real para que alas y paneles no sean planos.
pub struct TriangularPrism {
    pub points: [Vec3; 3],
    pub center_y: f32,
    pub half_height: f32,
    pub material: Material,
}

impl TriangularPrism {
    fn triangle_hit(ray: &Ray, a: Vec3, b: Vec3, c: Vec3, material: &Material) -> Option<Hit> {
        let edge_a = b - a;
        let edge_b = c - a;
        let perpendicular = cross(&ray.direction, &edge_b);
        let determinant = dot(&edge_a, &perpendicular);
        if determinant.abs() < 1e-7 {
            return None;
        }

        let inverse = 1.0 / determinant;
        let from_a = ray.origin - a;
        let barycentric_u = inverse * dot(&from_a, &perpendicular);
        if !(0.0..=1.0).contains(&barycentric_u) {
            return None;
        }

        let direction = cross(&from_a, &edge_a);
        let barycentric_v = inverse * dot(&ray.direction, &direction);
        if barycentric_v < 0.0 || barycentric_u + barycentric_v > 1.0 {
            return None;
        }

        let distance = inverse * dot(&edge_b, &direction);
        if distance <= 0.001 {
            return None;
        }

        let point = ray.at(distance);
        let mut normal = normalize(&cross(&edge_a, &edge_b));
        if dot(&normal, &ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(Hit {
            distance,
            point,
            normal,
            uv: [barycentric_u, barycentric_v],
            material: material.clone(),
        })
    }

    fn face_hit(&self, ray: &Ray, a: Vec3, b: Vec3, c: Vec3) -> Option<Hit> {
        Self::triangle_hit(ray, a, b, c, &self.material)
    }

    fn vertex(&self, index: usize, y: f32) -> Vec3 {
        Vec3::new(self.points[index].x, y, self.points[index].z)
    }
}

impl Primitive for TriangularPrism {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let bottom = self.center_y - self.half_height;
        let top = self.center_y + self.half_height;
        let vertices = [
            self.vertex(0, bottom),
            self.vertex(1, bottom),
            self.vertex(2, bottom),
            self.vertex(0, top),
            self.vertex(1, top),
            self.vertex(2, top),
        ];

        let mut hits = Vec::new();
        if let Some(hit) = self.face_hit(ray, vertices[3], vertices[4], vertices[5]) {
            hits.push(hit);
        }
        if let Some(hit) = self.face_hit(ray, vertices[2], vertices[1], vertices[0]) {
            hits.push(hit);
        }

        for (a, b, c, d) in [
            (vertices[0], vertices[1], vertices[4], vertices[5]),
            (vertices[1], vertices[2], vertices[5], vertices[4]),
            (vertices[2], vertices[0], vertices[3], vertices[5]),
        ] {
            if let Some(hit) = self.face_hit(ray, a, b, c) {
                hits.push(hit);
            }
            if let Some(hit) = self.face_hit(ray, a, c, d) {
                hits.push(hit);
            }
        }

        hits.into_iter()
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}
