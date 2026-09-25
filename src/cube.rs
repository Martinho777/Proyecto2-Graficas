use nalgebra_glm::Vec3;

use crate::material::Material;
use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn from_center_size(center: Vec3, size: Vec3, material: Material) -> Self {
        let half = size * 0.5;
        Self {
            min: center - half,
            max: center + half,
            material,
        }
    }

    fn axis_values(&self, axis: usize) -> (f32, f32) {
        match axis {
            0 => (self.min.x, self.max.x),
            1 => (self.min.y, self.max.y),
            _ => (self.min.z, self.max.z),
        }
    }

    fn normal_at(&self, point: &Vec3) -> Vec3 {
        let epsilon = 0.001;
        if (point.x - self.min.x).abs() < epsilon {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (point.x - self.max.x).abs() < epsilon {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (point.y - self.min.y).abs() < epsilon {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (point.y - self.max.y).abs() < epsilon {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (point.z - self.min.z).abs() < epsilon {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        }
    }

    fn uv_at(&self, point: &Vec3, normal: &Vec3) -> [f32; 2] {
        let size = self.max - self.min;
        if normal.x.abs() > 0.5 {
            [
                ((point.z - self.min.z) / size.z),
                ((point.y - self.min.y) / size.y),
            ]
        } else if normal.y.abs() > 0.5 {
            [
                ((point.x - self.min.x) / size.x),
                ((point.z - self.min.z) / size.z),
            ]
        } else {
            [
                ((point.x - self.min.x) / size.x),
                ((point.y - self.min.y) / size.y),
            ]
        }
    }
}

impl Primitive for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let origins = [ray.origin.x, ray.origin.y, ray.origin.z];
        let directions = [ray.direction.x, ray.direction.y, ray.direction.z];
        let mut t_min: f32 = 0.001;
        let mut t_max: f32 = f32::INFINITY;

        for axis in 0..3 {
            let (min_value, max_value) = self.axis_values(axis);
            if directions[axis].abs() < 1e-8 {
                if origins[axis] < min_value || origins[axis] > max_value {
                    return None;
                }
                continue;
            }

            let mut near = (min_value - origins[axis]) / directions[axis];
            let mut far = (max_value - origins[axis]) / directions[axis];
            if near > far {
                std::mem::swap(&mut near, &mut far);
            }
            t_min = t_min.max(near);
            t_max = t_max.min(far);
            if t_min > t_max {
                return None;
            }
        }

        let point = ray.at(t_min);
        let normal = self.normal_at(&point);
        Some(Hit {
            distance: t_min,
            point,
            uv: self.uv_at(&point, &normal),
            normal,
            material: self.material.clone(),
        })
    }
}
