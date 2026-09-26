use nalgebra_glm::Vec3;

use crate::cube::Cube;
use crate::cylinder::Cylinder;
use crate::material::{MaterialId, MaterialLibrary};
use crate::primitive::{Hit, Primitive};
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Block {
    pub center: Vec3,
    pub size: Vec3,
    pub material: MaterialId,
    pub studs: bool,
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

        if block.studs {
            self.add_studs(&block, materials);
        }
        self.blocks.push(block);
    }

    fn add_studs(&mut self, block: &Block, materials: &MaterialLibrary) {
        let columns = ((block.size.x / 0.55).floor() as usize).clamp(1, 4);
        let rows = ((block.size.z / 0.55).floor() as usize).clamp(1, 4);
        let spacing = 0.45;
        let top_y = block.center.y + block.size.y * 0.5 + 0.08;

        for column in 0..columns {
            for row in 0..rows {
                let x = block.center.x + (column as f32 - (columns - 1) as f32 * 0.5) * spacing;
                let z = block.center.z + (row as f32 - (rows - 1) as f32 * 0.5) * spacing;
                self.objects.push(Box::new(Cylinder {
                    center: Vec3::new(x, top_y, z),
                    radius: 0.12,
                    height: 0.16,
                    material: materials.get(block.material),
                }));
            }
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.objects
            .iter()
            .filter_map(|object| object.intersect(ray))
            .min_by(|left, right| left.distance.total_cmp(&right.distance))
    }
}
