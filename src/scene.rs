use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

pub struct Scene {
    pub objects: Vec<Box<dyn Primitive>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Primitive>) {
        self.objects.push(object);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.objects
            .iter()
            .filter_map(|object| object.intersect(ray))
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}
