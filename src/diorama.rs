use nalgebra_glm::Vec3;

use crate::material::MaterialId;
use crate::scene::Block;

fn block(center: Vec3, size: Vec3, material: MaterialId) -> Block {
    Block {
        center,
        size,
        material,
    }
}

pub fn build_floating_island() -> Vec<Block> {
    let mut blocks = Vec::new();

    for x in -2..=2 {
        for z in -2..=2 {
            blocks.push(block(
                Vec3::new(x as f32, -1.75, z as f32),
                Vec3::new(1.0, 0.8, 1.0),
                MaterialId::Stone,
            ));
        }
    }

    for layer in 0..3 {
        let radius = 2.0 - layer as f32 * 0.55;
        let y = -2.45 - layer as f32 * 0.85;
        for x in -2..=2 {
            for z in -2..=2 {
                if (x as f32).hypot(z as f32) <= radius + 0.35 {
                    blocks.push(block(
                        Vec3::new(x as f32, y, z as f32),
                        Vec3::new(1.0, 0.85, 1.0),
                        MaterialId::Stone,
                    ));
                }
            }
        }
    }

    blocks
}
