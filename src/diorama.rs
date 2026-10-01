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
        let radius = 4.15 - layer as f32 * 0.75;
        let y = -1.0 - layer as f32 * 0.65;
        for x in -4..=4 {
            for z in -4..=4 {
                if (x as f32).hypot(z as f32) <= radius {
                    blocks.push(block(
                        Vec3::new(x as f32, y, z as f32),
                        Vec3::new(1.0, 0.65, 1.0),
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
        Vec3::new(0.0, 0.25, -0.7),
        Vec3::new(4.2, 1.8, 2.8),
        MaterialId::Stone,
    )];

    for x in [-1.65, 1.65] {
        blocks.push(block(
            Vec3::new(x, 1.55, -0.7),
            Vec3::new(1.0, 3.2, 1.25),
            MaterialId::Stone,
        ));
        for roof_layer in 0..3 {
            let shrink = roof_layer as f32 * 0.18;
            blocks.push(block(
                Vec3::new(x, 3.35 + roof_layer as f32 * 0.24, -0.7),
                Vec3::new(1.5 - shrink, 0.24, 1.75 - shrink),
                MaterialId::Wood,
            ));
        }
    }

    blocks.push(block(
        Vec3::new(0.0, 1.45, -0.7),
        Vec3::new(1.55, 4.0, 1.6),
        MaterialId::Stone,
    ));
    for roof_layer in 0..4 {
        let shrink = roof_layer as f32 * 0.2;
        blocks.push(block(
            Vec3::new(0.0, 3.7 + roof_layer as f32 * 0.25, -0.7),
            Vec3::new(2.0 - shrink, 0.25, 2.1 - shrink),
            MaterialId::Wood,
        ));
    }

    blocks.push(block(
        Vec3::new(0.0, -0.05, 0.75),
        Vec3::new(0.65, 0.9, 0.12),
        MaterialId::Wood,
    ));
    for x in [-1.65, 0.0, 1.65] {
        blocks.push(block(
            Vec3::new(x, 0.55, 0.73),
            Vec3::new(0.28, 0.65, 0.08),
            MaterialId::Crystal,
        ));
    }

    blocks
}

fn build_path() -> Vec<Block> {
    let mut blocks = Vec::new();
    for z in [-0.2, 0.5, 1.2, 1.9, 2.6] {
        let width = 0.75 + (z + 0.2) * 0.12;
        blocks.push(block(
            Vec3::new(0.0, -0.28, z),
            Vec3::new(width, 0.16, 0.5),
            MaterialId::Wood,
        ));
    }
    blocks
}

fn build_bushes() -> Vec<Block> {
    let mut blocks = Vec::new();
    for (x, z) in [(-3.0, 0.8), (2.8, 1.6), (-2.8, -2.1), (2.7, -2.2)] {
        blocks.push(block(
            Vec3::new(x, -0.05, z),
            Vec3::new(1.3, 0.65, 0.8),
            MaterialId::Leaves,
        ));
        blocks.push(block(
            Vec3::new(x, 0.38, z),
            Vec3::new(0.8, 0.45, 0.65),
            MaterialId::Leaves,
        ));
    }
    blocks
}

fn build_pipes() -> Vec<Block> {
    vec![
        block(
            Vec3::new(-3.0, 0.55, -0.3),
            Vec3::new(0.8, 1.6, 0.8),
            MaterialId::Leaves,
        ),
        block(
            Vec3::new(-3.0, 1.42, -0.3),
            Vec3::new(1.05, 0.22, 1.05),
            MaterialId::Leaves,
        ),
    ]
}

fn build_floating_props(scene: &mut Scene, materials: &MaterialLibrary) {
    let question = materials.question();
    let coin = materials.coin();
    for (x, y, z) in [(-2.0, 2.0, 0.5), (1.8, 2.55, 1.0), (2.7, 1.85, -0.8)] {
        scene.add_custom_block(
            Vec3::new(x, y, z),
            Vec3::new(0.7, 0.7, 0.7),
            question.clone(),
        );
    }
    for (x, y, z) in [(-1.35, 1.65, 1.2), (0.0, 2.6, 1.5), (1.35, 1.8, 1.35)] {
        scene.add_custom_block(Vec3::new(x, y, z), Vec3::new(0.36, 0.7, 0.12), coin.clone());
    }
}

fn build_island_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x496A88);
    add_blocks(&mut scene, materials, build_floating_island());
    add_blocks(&mut scene, materials, build_castle());
    add_blocks(&mut scene, materials, build_path());
    add_blocks(&mut scene, materials, build_bushes());
    add_blocks(&mut scene, materials, build_pipes());
    build_floating_props(&mut scene, materials);
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
