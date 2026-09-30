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
