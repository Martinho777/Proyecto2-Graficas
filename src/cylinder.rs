use nalgebra_glm::{dot, normalize, Vec3};

use crate::material::Material;
use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

pub struct Cylinder {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub material: Material,
}

/// Vertical cylinder: axis aligned with world Y, used by DK's hut and palms.
pub struct VerticalCylinder {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub material: Material,
}

/// Thin half-cylinder used for split circular markings such as a Pokeball.
pub struct HalfCylinder {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub upper_z_half: bool,
    pub material: Material,
}

impl HalfCylinder {
    fn belongs_to_half(&self, point: Vec3) -> bool {
        let local_z = point.z - self.center.z;
        if self.upper_z_half {
            local_z >= -0.0001
        } else {
            local_z <= 0.0001
        }
    }
}

impl Primitive for HalfCylinder {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let relative = ray.origin - self.center;
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        let mut hits = Vec::new();

        if a.abs() > 1e-8 {
            let b = 2.0 * (relative.x * ray.direction.x + relative.z * ray.direction.z);
            let c = relative.x * relative.x + relative.z * relative.z - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;
            if discriminant >= 0.0 {
                let root = discriminant.sqrt();
                for distance in [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)] {
                    if distance <= 0.001 {
                        continue;
                    }
                    let point = ray.at(distance);
                    if point.y < self.center.y - self.height * 0.5
                        || point.y > self.center.y + self.height * 0.5
                        || !self.belongs_to_half(point)
                    {
                        continue;
                    }
                    let radial = Vec3::new(point.x - self.center.x, 0.0, point.z - self.center.z);
                    hits.push(Hit {
                        distance,
                        point,
                        normal: normalize(&radial),
                        uv: [0.5, 0.5],
                        material: self.material.clone(),
                    });
                }
            }
        }

        if ray.direction.y.abs() > 1e-8 {
            for (y, normal) in [
                (self.center.y + self.height * 0.5, Vec3::new(0.0, 1.0, 0.0)),
                (self.center.y - self.height * 0.5, Vec3::new(0.0, -1.0, 0.0)),
            ] {
                let distance = (y - ray.origin.y) / ray.direction.y;
                if distance <= 0.001 {
                    continue;
                }
                let point = ray.at(distance);
                let dx = point.x - self.center.x;
                let dz = point.z - self.center.z;
                if dx * dx + dz * dz <= self.radius * self.radius && self.belongs_to_half(point) {
                    hits.push(Hit {
                        distance,
                        point,
                        normal,
                        uv: [0.5, 0.5],
                        material: self.material.clone(),
                    });
                }
            }
        }

        // Flat diameter face closes the semicircle and gives the black band a
        // clean boundary instead of leaving a missing vertical surface.
        if ray.direction.z.abs() > 1e-8 {
            let distance = (self.center.z - ray.origin.z) / ray.direction.z;
            if distance > 0.001 {
                let point = ray.at(distance);
                if point.x >= self.center.x - self.radius
                    && point.x <= self.center.x + self.radius
                    && point.y >= self.center.y - self.height * 0.5
                    && point.y <= self.center.y + self.height * 0.5
                {
                    hits.push(Hit {
                        distance,
                        point,
                        normal: if ray.direction.z > 0.0 {
                            Vec3::new(0.0, 0.0, -1.0)
                        } else {
                            Vec3::new(0.0, 0.0, 1.0)
                        },
                        uv: [0.5, 0.5],
                        material: self.material.clone(),
                    });
                }
            }
        }

        hits.into_iter()
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}

/// Cylinder with an arbitrary axis, useful for diagonal supports and struts.
pub struct OrientedCylinder {
    pub center: Vec3,
    pub axis: Vec3,
    pub radius: f32,
    pub height: f32,
    pub material: Material,
}

impl Primitive for OrientedCylinder {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let axis = normalize(&self.axis);
        let relative = ray.origin - self.center;
        let ray_axis = dot(&ray.direction, &axis);
        let origin_axis = dot(&relative, &axis);
        let perpendicular_direction = ray.direction - axis * ray_axis;
        let perpendicular_origin = relative - axis * origin_axis;
        let a = dot(&perpendicular_direction, &perpendicular_direction);
        let b = 2.0 * dot(&perpendicular_origin, &perpendicular_direction);
        let c = dot(&perpendicular_origin, &perpendicular_origin) - self.radius * self.radius;
        let half_height = self.height * 0.5;
        let mut hits = Vec::new();

        if a.abs() > 1e-8 {
            let discriminant = b * b - 4.0 * a * c;
            if discriminant >= 0.0 {
                let root = discriminant.sqrt();
                for distance in [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)] {
                    if distance <= 0.001 {
                        continue;
                    }
                    let point = ray.at(distance);
                    let axial = dot(&(point - self.center), &axis);
                    if axial.abs() > half_height {
                        continue;
                    }
                    let radial = point - self.center - axis * axial;
                    hits.push(Hit {
                        distance,
                        point,
                        normal: normalize(&radial),
                        uv: [0.5, 0.5 + axial / self.height],
                        material: self.material.clone(),
                    });
                }
            }
        }

        if ray_axis.abs() > 1e-8 {
            for (axial, normal) in [(-half_height, -axis), (half_height, axis)] {
                let distance = (axial - origin_axis) / ray_axis;
                if distance <= 0.001 {
                    continue;
                }
                let point = ray.at(distance);
                let radial = point - self.center - axis * axial;
                if dot(&radial, &radial) <= self.radius * self.radius {
                    hits.push(Hit {
                        distance,
                        point,
                        normal,
                        uv: [0.5, 0.5],
                        material: self.material.clone(),
                    });
                }
            }
        }

        hits.into_iter()
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}

impl Primitive for VerticalCylinder {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let relative = ray.origin - self.center;
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        let mut hits = Vec::new();

        if a.abs() > 1e-8 {
            let b = 2.0 * (relative.x * ray.direction.x + relative.z * ray.direction.z);
            let c = relative.x * relative.x + relative.z * relative.z - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;
            if discriminant >= 0.0 {
                let root = discriminant.sqrt();
                for distance in [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)] {
                    if distance < 0.001 || distance == f32::INFINITY {
                        continue;
                    }
                    let point = ray.at(distance);
                    if point.y < self.center.y - self.height * 0.5
                        || point.y > self.center.y + self.height * 0.5
                    {
                        continue;
                    }
                    let radial = Vec3::new(point.x - self.center.x, 0.0, point.z - self.center.z);
                    hits.push(Hit {
                        distance,
                        point,
                        normal: normalize(&radial),
                        uv: [
                            0.5 + radial.z.atan2(radial.x) / (2.0 * std::f32::consts::PI),
                            (point.y - (self.center.y - self.height * 0.5)) / self.height,
                        ],
                        material: self.material.clone(),
                    });
                }
            }
        }

        for (y, normal) in [
            (self.center.y + self.height * 0.5, Vec3::new(0.0, 1.0, 0.0)),
            (self.center.y - self.height * 0.5, Vec3::new(0.0, -1.0, 0.0)),
        ] {
            if ray.direction.y.abs() < 1e-8 {
                continue;
            }
            let distance = (y - ray.origin.y) / ray.direction.y;
            if distance < 0.001 {
                continue;
            }
            let point = ray.at(distance);
            let dx = point.x - self.center.x;
            let dz = point.z - self.center.z;
            if dx * dx + dz * dz <= self.radius * self.radius {
                hits.push(Hit {
                    distance,
                    point,
                    normal,
                    uv: [
                        0.5 + dx / (2.0 * self.radius),
                        0.5 + dz / (2.0 * self.radius),
                    ],
                    material: self.material.clone(),
                });
            }
        }

        hits.into_iter()
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
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
