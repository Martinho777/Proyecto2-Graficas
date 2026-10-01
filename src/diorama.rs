use nalgebra_glm::Vec3;

use crate::animation::bobbing_height;
use crate::app::Character;
use crate::material::{MaterialId, MaterialLibrary};
use crate::scene::{Block, Scene};
use crate::sphere::Sphere;

fn block(center: Vec3, size: Vec3, material: MaterialId) -> Block {
    Block {
        center: center * 2.0,
        size: size * 2.0,
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

pub fn build_bridge() -> Vec<Block> {
    let mut blocks = Vec::new();

    for z in 3..=6 {
        blocks.push(block(
            Vec3::new(0.0, -1.05, z as f32),
            Vec3::new(1.6, 0.35, 1.0),
            MaterialId::Wood,
        ));
        blocks.push(block(
            Vec3::new(-0.9, -0.55, z as f32),
            Vec3::new(0.25, 0.8, 1.0),
            MaterialId::Wood,
        ));
        blocks.push(block(
            Vec3::new(0.9, -0.55, z as f32),
            Vec3::new(0.25, 0.8, 1.0),
            MaterialId::Wood,
        ));
    }

    blocks
}

pub fn build_shrine() -> Vec<Block> {
    let mut blocks = Vec::new();

    blocks.push(block(
        Vec3::new(0.0, -0.95, -0.25),
        Vec3::new(3.8, 0.35, 3.0),
        MaterialId::Stone,
    ));

    for x in [-1.5, 1.5] {
        for z in [-1.15, 0.65] {
            blocks.push(block(
                Vec3::new(x, 0.15, z),
                Vec3::new(0.45, 2.2, 0.45),
                MaterialId::Wood,
            ));
        }
    }

    blocks.push(block(
        Vec3::new(0.0, 1.35, -0.25),
        Vec3::new(3.7, 0.4, 2.8),
        MaterialId::Leaves,
    ));

    blocks
}

pub fn build_pine_tree(base: Vec3, scale: f32) -> Vec<Block> {
    let mut blocks = Vec::new();
    let trunk_height = 2.0 * scale;

    blocks.push(block(
        base + Vec3::new(0.0, trunk_height * 0.5, 0.0),
        Vec3::new(0.55 * scale, trunk_height, 0.55 * scale),
        MaterialId::Wood,
    ));

    let foliage_layers = [(2.0, 2.7), (2.55, 2.1), (3.05, 1.5), (3.45, 0.8)];

    for (height, width) in foliage_layers {
        blocks.push(block(
            base + Vec3::new(0.0, height * scale, 0.0),
            Vec3::new(width * scale, 0.7 * scale, width * scale),
            MaterialId::Leaves,
        ));
    }

    blocks
}

pub fn build_forest() -> Vec<Block> {
    let mut blocks = Vec::new();
    let trees = [
        (Vec3::new(-1.65, -1.35, 1.45), 0.75),
        (Vec3::new(1.65, -1.35, 1.45), 0.9),
        (Vec3::new(-1.65, -1.35, -1.35), 0.65),
        (Vec3::new(1.65, -1.35, -1.35), 0.7),
    ];

    for (base, scale) in trees {
        blocks.extend(build_pine_tree(base, scale));
    }

    blocks
}

fn add_blocks(scene: &mut Scene, blocks: Vec<Block>, materials: &MaterialLibrary) {
    for block in blocks {
        scene.add_block(block, materials);
    }
}

fn add_mario_block(
    scene: &mut Scene,
    materials: &MaterialLibrary,
    center: Vec3,
    size: Vec3,
    material_id: MaterialId,
) {
    scene.add_custom_block(center, size, materials.get(material_id));
}

fn add_mario_gold_block(scene: &mut Scene, materials: &MaterialLibrary, center: Vec3, size: Vec3) {
    let mut gold = materials.get(MaterialId::Lava);
    gold.emission = [0.0, 0.0, 0.0];
    gold.albedo = [1.0, 0.9, 0.25];
    gold.specular = 48.0;
    add_mario_block(scene, materials, center, size, MaterialId::Lava);
    scene.objects.pop();
    scene.add_custom_block(center, size, gold);
}

pub fn build_forest_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x10152C);
    add_blocks(&mut scene, build_floating_island(), materials);
    add_blocks(&mut scene, build_forest(), materials);
    add_blocks(&mut scene, build_bridge(), materials);
    add_blocks(&mut scene, build_shrine(), materials);
    scene.add(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.5,
        material: materials.get(MaterialId::Crystal),
    }));
    scene
}

pub fn build_mario_stage(materials: &MaterialLibrary) -> Scene {
    build_mario_stage_at(materials, 0.0)
}

fn build_mario_hills() -> Vec<Block> {
    let mut blocks = Vec::new();
    for (x, height, width, z) in [
        (-3.5, 1.8, 2.8, 2.2),
        (-2.0, 2.5, 2.5, 2.5),
        (2.3, 2.2, 3.0, 2.3),
        (3.8, 1.5, 2.3, 2.0),
    ] {
        for layer in 0..3 {
            let layer_width = width - layer as f32 * 0.55;
            if layer_width > 0.0 {
                blocks.push(block(
                    Vec3::new(x, -0.35 + layer as f32 * 0.48, z),
                    Vec3::new(layer_width, 0.48, 0.7),
                    MaterialId::Leaves,
                ));
            }
        }
        blocks.push(block(
            Vec3::new(x, -0.35 + height * 0.18, z - 0.38),
            Vec3::new(width * 0.68, 0.18, 0.12),
            MaterialId::Leaves,
        ));
    }
    blocks
}

fn build_mario_pipe(x: f32, z: f32, height: i32) -> Vec<Block> {
    let mut blocks = Vec::new();
    for y in 0..height {
        blocks.push(block(
            Vec3::new(x, -0.12 + y as f32 * 0.62, z),
            Vec3::new(0.78, 0.62, 0.78),
            MaterialId::Leaves,
        ));
    }
    blocks.push(block(
        Vec3::new(x, -0.12 + height as f32 * 0.62, z),
        Vec3::new(1.05, 0.22, 1.05),
        MaterialId::Leaves,
    ));
    blocks
}

fn build_mario_castle() -> Vec<Block> {
    let mut blocks = Vec::new();
    blocks.push(block(
        Vec3::new(0.0, 0.9, -2.8),
        Vec3::new(4.8, 2.4, 1.9),
        MaterialId::Stone,
    ));
    for x in [-1.8, 1.8] {
        blocks.push(block(
            Vec3::new(x, 1.9, -2.8),
            Vec3::new(1.05, 3.2, 1.8),
            MaterialId::Stone,
        ));
        blocks.push(block(
            Vec3::new(x, 3.65, -2.8),
            Vec3::new(1.35, 0.35, 2.15),
            MaterialId::Wood,
        ));
    }
    blocks.push(block(
        Vec3::new(0.0, 2.35, -2.8),
        Vec3::new(1.6, 3.8, 1.8),
        MaterialId::Stone,
    ));
    blocks.push(block(
        Vec3::new(0.0, 4.45, -2.8),
        Vec3::new(1.95, 0.4, 2.15),
        MaterialId::Wood,
    ));
    blocks.push(block(
        Vec3::new(0.0, 5.2, -2.8),
        Vec3::new(0.14, 1.2, 0.14),
        MaterialId::Wood,
    ));
    blocks.push(block(
        Vec3::new(0.4, 5.42, -2.8),
        Vec3::new(0.7, 0.35, 0.12),
        MaterialId::Wood,
    ));
    blocks.push(block(
        Vec3::new(0.0, 0.4, -1.82),
        Vec3::new(0.65, 1.0, 0.12),
        MaterialId::Crystal,
    ));
    for x in [-1.8, 1.8] {
        blocks.push(block(
            Vec3::new(x, 1.8, -1.82),
            Vec3::new(0.32, 0.65, 0.12),
            MaterialId::Crystal,
        ));
    }
    blocks
}

fn build_mario_path() -> Vec<Block> {
    let mut blocks = Vec::new();
    for z in [-1.25, -0.5, 0.25, 1.0] {
        blocks.push(block(
            Vec3::new(0.0, -0.32, z),
            Vec3::new(1.15, 0.18, 0.55),
            MaterialId::Wood,
        ));
    }
    blocks
}

fn build_mario_island() -> Vec<Block> {
    let mut blocks = Vec::new();
    for layer in 0..5 {
        let radius = 4.6 - layer as f32 * 0.78;
        let y = -1.2 - layer as f32 * 0.72;
        let material = if layer == 0 {
            MaterialId::Stone
        } else {
            MaterialId::Wood
        };

        for x in -5..=5 {
            for z in -5..=5 {
                if (x as f32).hypot(z as f32) <= radius {
                    blocks.push(block(
                        Vec3::new(x as f32, y, z as f32),
                        Vec3::new(0.92, 0.72, 0.92),
                        material,
                    ));
                }
            }
        }
    }

    for x in -4..=4 {
        for z in -4..=4 {
            if (x as f32).hypot(z as f32) <= 4.35 {
                blocks.push(block(
                    Vec3::new(x as f32, -0.72, z as f32),
                    Vec3::new(0.94, 0.18, 0.94),
                    MaterialId::Leaves,
                ));
            }
        }
    }
    blocks
}

fn build_mario_blocks() -> Vec<Block> {
    let mut blocks = Vec::new();
    for x in [-1.8, 0.0, 1.8] {
        blocks.push(block(
            Vec3::new(x, 1.55, 0.25),
            Vec3::new(0.72, 0.72, 0.72),
            MaterialId::Lava,
        ));
    }
    for x in [-2.7, -1.8, -0.9, 0.9, 1.8, 2.7] {
        blocks.push(block(
            Vec3::new(x, 2.4, 0.35),
            Vec3::new(0.78, 0.45, 0.78),
            MaterialId::Wood,
        ));
    }
    blocks
}

pub fn build_mario_stage_at(materials: &MaterialLibrary, time: f32) -> Scene {
    let mut scene = Scene::with_background(0x6BC7F2);

    add_blocks(&mut scene, build_mario_hills(), materials);
    add_blocks(&mut scene, build_mario_pipe(-2.6, -0.4, 2), materials);
    add_blocks(&mut scene, build_mario_pipe(2.6, 0.0, 1), materials);
    for block in build_mario_blocks() {
        if matches!(block.material, MaterialId::Lava) {
            add_mario_gold_block(&mut scene, materials, block.center, block.size);
        } else {
            add_mario_block(
                &mut scene,
                materials,
                block.center,
                block.size,
                block.material,
            );
        }
    }

    add_blocks(&mut scene, build_mario_island(), materials);

    add_blocks(&mut scene, build_mario_path(), materials);

    for (x, y, z, width) in [
        (-2.0, 0.25, 0.5, 2.0),
        (1.5, 1.15, -0.5, 2.5),
        (-0.5, 2.0, -1.5, 1.5),
    ] {
        add_blocks(
            &mut scene,
            vec![block(
                Vec3::new(x, y, z),
                Vec3::new(width, 0.35, 0.8),
                MaterialId::Wood,
            )],
            materials,
        );
    }

    for x in -1..=1 {
        add_blocks(
            &mut scene,
            vec![block(
                Vec3::new(x as f32, 0.75, -2.0),
                Vec3::new(0.8, 0.8, 0.8),
                MaterialId::Stone,
            )],
            materials,
        );
    }

    add_blocks(&mut scene, build_mario_castle(), materials);
    add_blocks(
        &mut scene,
        vec![
            block(
                Vec3::new(-2.5, 3.0, -1.0),
                Vec3::new(2.0, 0.45, 0.7),
                MaterialId::Leaves,
            ),
            block(
                Vec3::new(2.4, 3.35, 0.2),
                Vec3::new(1.6, 0.45, 0.7),
                MaterialId::Leaves,
            ),
        ],
        materials,
    );

    for (x, y, z, phase) in [
        (-1.6, 1.0, 0.5, 0.0),
        (0.0, 2.65, -1.5, 1.4),
        (1.65, 1.9, -0.5, 2.8),
    ] {
        add_blocks(
            &mut scene,
            vec![block(
                Vec3::new(x, y + bobbing_height(time, phase, 0.28, 2.4), z),
                Vec3::new(0.42, 0.6, 0.18),
                MaterialId::Lava,
            )],
            materials,
        );
    }

    scene
}

pub fn build_stage(character: Character, materials: &MaterialLibrary) -> Scene {
    build_stage_at(character, materials, 0.0)
}

pub fn build_stage_at(character: Character, materials: &MaterialLibrary, time: f32) -> Scene {
    match character {
        Character::Mario => build_mario_stage_at(materials, time),
        _ => build_forest_stage(materials),
    }
}
