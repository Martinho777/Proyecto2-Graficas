use crate::app::Character;
use crate::material::{MaterialId, MaterialLibrary};
use crate::scene::{Block, Scene};
use nalgebra_glm::Vec3;

fn block(center: Vec3, size: Vec3, material: MaterialId) -> Block {
    Block {
        center,
        size,
        material,
    }
}

fn add_blocks(scene: &mut Scene, materials: &MaterialLibrary, blocks: Vec<Block>) {
    for block in blocks {
        scene.add_block(block, materials);
    }
}

fn build_floating_island() -> Vec<Block> {
    let mut blocks = Vec::new();

    for layer in 0..4 {
        let radius = 3.8 - layer as f32 * 0.7;
        let y = -1.0 - layer as f32 * 0.65;
        for x in -4..=4 {
            for z in -4..=4 {
                if (x as f32).hypot(z as f32) <= radius {
                    blocks.push(block(
                        Vec3::new(x as f32, y, z as f32),
                        Vec3::new(0.9, 0.65, 0.9),
                        if layer == 0 {
                            MaterialId::Leaves
                        } else {
                            MaterialId::Wood
                        },
                    ));
                }
            }
        }
    }

    blocks
}

fn build_castle() -> Vec<Block> {
    let mut blocks = vec![block(
        Vec3::new(0.0, 0.2, -0.5),
        Vec3::new(3.4, 1.8, 2.2),
        MaterialId::Stone,
    )];

    for x in [-1.35, 1.35] {
        blocks.push(block(
            Vec3::new(x, 1.45, -0.5),
            Vec3::new(0.9, 3.0, 1.0),
            MaterialId::Stone,
        ));
        blocks.push(block(
            Vec3::new(x, 3.15, -0.5),
            Vec3::new(1.2, 0.35, 1.3),
            MaterialId::Wood,
        ));
    }

    blocks.push(block(
        Vec3::new(0.0, 1.35, -0.5),
        Vec3::new(1.25, 3.8, 1.35),
        MaterialId::Stone,
    ));
    blocks.push(block(
        Vec3::new(0.0, 3.55, -0.5),
        Vec3::new(1.6, 0.35, 1.7),
        MaterialId::Wood,
    ));

    for x in [-1.35, 0.0, 1.35] {
        blocks.push(block(
            Vec3::new(x, 0.35, 0.63),
            Vec3::new(0.28, 0.65, 0.08),
            MaterialId::Crystal,
        ));
    }

    blocks
}

fn build_island_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x496A88);
    add_blocks(&mut scene, materials, build_floating_island());
    add_blocks(&mut scene, materials, build_castle());
    scene
}

/// Empty visual scaffold. The new diorama will be designed here from scratch.
pub fn build_stage(_character: Character, _materials: &MaterialLibrary) -> Scene {
    build_island_stage(_materials)
}

/// Stable animated entry point for the app state machine.
pub fn build_stage_at(character: Character, materials: &MaterialLibrary, _time: f32) -> Scene {
    build_stage(character, materials)
}
