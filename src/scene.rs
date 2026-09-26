use nalgebra_glm::Vec3;

use crate::cube::Cube;
use crate::material::{MaterialId, MaterialLibrary};
use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Block {
    pub center: Vec3,
    pub size: Vec3,
    pub material: MaterialId,
}

pub struct Scene {
    pub objects: Vec<Box<dyn Primitive>>,
    pub blocks: Vec<Block>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Primitive>) {
        self.objects.push(object);
    }

    pub fn add_block(&mut self, block: Block, materials: &MaterialLibrary) {
        self.objects.push(Box::new(Cube::from_center_size(
            block.center,
            block.size,
            materials.get(block.material),
        )));
        self.blocks.push(block);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.objects
            .iter()
            .filter_map(|object| object.intersect(ray))
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}
