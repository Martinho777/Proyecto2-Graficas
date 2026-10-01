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
    pub dynamic_objects: Vec<Box<dyn Primitive>>,
    pub blocks: Vec<Block>,
    pub background: u32,
    pub navi_light_position: Option<Vec3>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            dynamic_objects: Vec::new(),
            blocks: Vec::new(),
            background: 0x07111F,
            navi_light_position: None,
        }
    }

    pub fn with_background(background: u32) -> Self {
        Self {
            objects: Vec::new(),
            dynamic_objects: Vec::new(),
            blocks: Vec::new(),
            background,
            navi_light_position: None,
        }
    }

    pub fn add(&mut self, object: Box<dyn Primitive>) {
        self.objects.push(object);
    }

    pub fn add_dynamic(&mut self, object: Box<dyn Primitive>) {
        self.dynamic_objects.push(object);
    }

    pub fn clear_dynamic(&mut self) {
        self.dynamic_objects.clear();
        self.navi_light_position = None;
    }

    pub fn add_block(&mut self, block: Block, materials: &MaterialLibrary) {
        self.add_custom_block(block.center, block.size, materials.get(block.material));

        self.blocks.push(block);
    }

    pub fn add_custom_block(
        &mut self,
        center: Vec3,
        size: Vec3,
        material: crate::material::Material,
    ) {
        self.objects
            .push(Box::new(Cube::from_center_size(center, size, material)));
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.objects
            .iter()
            .chain(self.dynamic_objects.iter())
            .filter_map(|object| object.intersect(ray))
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}
