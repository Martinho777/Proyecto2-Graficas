use crate::animation::bobbing_height;
use crate::app::Character;
use crate::cylinder::{Cylinder, VerticalCylinder};
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
        let radius = 4.45 - layer as f32 * 0.78;
        let y = -1.0 - layer as f32 * 0.65;
        for x in -5..=5 {
            for z in -5..=5 {
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
            Vec3::new(x, 0.75, -0.7),
            Vec3::new(1.0, 1.6, 1.25),
            MaterialId::Stone,
        ));
        for roof_layer in 0..3 {
            let shrink = roof_layer as f32 * 0.18;
            blocks.push(block(
                Vec3::new(x, 1.65 + roof_layer as f32 * 0.24, -0.7),
                Vec3::new(1.5 - shrink, 0.24, 1.75 - shrink),
                MaterialId::Lava,
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
            Vec3::new(0.0, 3.55 + roof_layer as f32 * 0.25, -0.7),
            Vec3::new(2.0 - shrink, 0.25, 2.1 - shrink),
            MaterialId::Lava,
        ));
    }

    blocks.push(block(
        Vec3::new(0.0, 4.92, -0.7),
        Vec3::new(0.1, 1.0, 0.1),
        MaterialId::Wood,
    ));
    blocks.push(block(
        Vec3::new(0.32, 5.12, -0.7),
        Vec3::new(0.65, 0.32, 0.08),
        MaterialId::Lava,
    ));

    blocks.push(block(
        Vec3::new(0.0, -0.05, 1.15),
        Vec3::new(0.65, 0.9, 0.12),
        MaterialId::Wood,
    ));

    for castle_block in &mut blocks {
        castle_block.center.x *= 1.2;
        castle_block.center.y *= 1.2;
        castle_block.size *= 1.2;
        castle_block.center.z -= 2.2;
    }

    blocks
}

fn build_path() -> Vec<Block> {
    let mut blocks = Vec::new();
    for z in [-0.2, 0.5, 1.2, 1.9, 2.6] {
        let width = 0.75 + (z + 0.2) * 0.12;
        blocks.push(block(
            Vec3::new(0.0, -0.595, z),
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
            Vec3::new(x, -0.35, z),
            Vec3::new(1.3, 0.65, 0.8),
            MaterialId::Leaves,
        ));
        blocks.push(block(
            Vec3::new(x, 0.20, z),
            Vec3::new(0.8, 0.45, 0.65),
            MaterialId::Leaves,
        ));
    }
    blocks
}

fn build_pipes() -> Vec<Block> {
    vec![
        block(
            Vec3::new(-3.0, 0.125, -0.3),
            Vec3::new(0.8, 1.6, 0.8),
            MaterialId::Leaves,
        ),
        block(
            Vec3::new(-3.0, 1.04, -0.3),
            Vec3::new(1.05, 0.22, 1.05),
            MaterialId::Leaves,
        ),
        block(
            Vec3::new(2.65, -0.05, 1.45),
            Vec3::new(0.7, 1.25, 0.7),
            MaterialId::Leaves,
        ),
        block(
            Vec3::new(2.65, 0.60, 1.45),
            Vec3::new(0.92, 0.2, 0.92),
            MaterialId::Leaves,
        ),
    ]
}

fn build_static_props(scene: &mut Scene, materials: &MaterialLibrary) {
    let question = materials.question();
    let question_mark = materials.question_mark();
    for (x, y, z) in [
        (-2.0, 0.18, 0.5),
        (1.8, 0.95, 1.0),
        (2.7, 0.35, -0.8),
        (-2.7, 1.25, 2.1),
    ] {
        let center = Vec3::new(x, y, z);
        scene.add_custom_block(center, Vec3::new(0.62, 0.62, 0.62), question.clone());
        let front_z = z + 0.325;
        scene.add_custom_block(
            Vec3::new(x, y + 0.16, front_z),
            Vec3::new(0.25, 0.07, 0.035),
            question_mark.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x + 0.1, y + 0.04, front_z),
            Vec3::new(0.07, 0.22, 0.035),
            question_mark.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x, y - 0.09, front_z),
            Vec3::new(0.2, 0.07, 0.035),
            question_mark.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x - 0.03, y - 0.2, front_z),
            Vec3::new(0.07, 0.08, 0.035),
            question_mark.clone(),
        );
    }
}

fn update_mario_animation(scene: &mut Scene, materials: &MaterialLibrary, time: f32) {
    let coin = materials.coin();
    for (x, y, z, phase) in [
        (-1.35, -0.15, 1.2, 0.0),
        (1.8, -0.05, 1.05, 1.1),
        (1.35, 0.05, 1.35, 2.2),
        (-2.2, -0.1, 2.45, 3.3),
    ] {
        scene.add_dynamic(Box::new(Cylinder {
            center: Vec3::new(x, y + bobbing_height(time, phase, 0.08, 1.8), z),
            radius: 0.32,
            height: 0.12,
            material: coin.clone(),
        }));
    }
}

fn build_mario_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x496A88);
    add_blocks(&mut scene, materials, build_floating_island());
    add_blocks(&mut scene, materials, build_castle());
    for x in [-1.98, 1.98] {
        scene.add_custom_block(
            Vec3::new(x, 0.66, -2.12),
            Vec3::new(0.28, 0.65, 0.12),
            materials.plain_crystal(),
        );
    }
    scene.add_custom_block(
        Vec3::new(0.0, 3.06, -1.88),
        Vec3::new(0.66, 1.02, 0.12),
        materials.stained_glass(),
    );
    add_blocks(&mut scene, materials, build_path());
    add_blocks(&mut scene, materials, build_bushes());
    let pipe = materials.pipe();
    for pipe_block in build_pipes() {
        scene.add_custom_block(pipe_block.center, pipe_block.size, pipe.clone());
    }
    build_static_props(&mut scene, materials);
    scene
}

fn add_kong_letter(scene: &mut Scene, materials: &MaterialLibrary, letter: &[&str], x: f32) {
    for (row, line) in letter.iter().enumerate() {
        for (column, pixel) in line.chars().enumerate() {
            if pixel == '#' {
                scene.add_custom_block(
                    Vec3::new(x + column as f32 * 0.12, 1.62 - row as f32 * 0.12, 0.52),
                    Vec3::new(0.1, 0.1, 0.05),
                    materials.banana(),
                );
            }
        }
    }
}

fn build_dk_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x63C9E8);
    add_blocks(&mut scene, materials, build_floating_island());

    let hut = materials.get(MaterialId::Wood);
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, 0.18, -0.85),
        radius: 1.55,
        height: 1.8,
        material: hut.clone(),
    }));

    let roof = materials.thatch();
    for (y, width, depth) in [(1.12, 3.5, 3.1), (1.34, 3.0, 2.7), (1.56, 2.45, 2.3)] {
        scene.add_custom_block(
            Vec3::new(0.0, y, -0.85),
            Vec3::new(width, 0.25, depth),
            roof.clone(),
        );
    }

    scene.add_custom_block(
        Vec3::new(0.0, -0.02, 0.62),
        Vec3::new(0.9, 1.25, 0.14),
        hut.clone(),
    );
    scene.add_custom_block(
        Vec3::new(0.0, 1.48, 0.42),
        Vec3::new(2.4, 0.62, 0.14),
        hut.clone(),
    );

    let glyphs: [&[&str]; 4] = [
        &["##.", "#.#", "##.", "#.#", "#.#"],
        &["###", "#.#", "#.#", "#.#", "###"],
        &["##.", "#.#", "##.", "#.#", "#.#"],
        &[".##", "#..", "#..", "#..", ".##"],
    ];
    for (index, glyph) in glyphs.iter().enumerate() {
        add_kong_letter(&mut scene, materials, glyph, -0.8 + index as f32 * 0.52);
    }

    for x in [-3.0, 3.0] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, 0.35, -0.1),
            radius: 0.16,
            height: 1.9,
            material: hut.clone(),
        }));
        for (dx, dz, size) in [(-0.55, 0.0, 1.15), (0.35, 0.28, 1.0), (0.25, -0.42, 0.9)] {
            scene.add_custom_block(
                Vec3::new(x + dx, 1.28, -0.1 + dz),
                Vec3::new(size, 0.16, 0.32),
                materials.get(MaterialId::Leaves),
            );
        }
    }

    let banana = materials.banana();
    for (x, z) in [(-2.0, 1.15), (-1.45, 1.75), (1.8, 1.2), (2.35, 0.55)] {
        scene.add_custom_block(
            Vec3::new(x, -0.58, z),
            Vec3::new(0.42, 0.12, 0.16),
            banana.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x + 0.16, -0.48, z + 0.06),
            Vec3::new(0.12, 0.12, 0.28),
            banana.clone(),
        );
    }

    scene
}

/// Actualiza solamente la animación correspondiente al stage seleccionado.
pub fn update_stage_animation(
    scene: &mut Scene,
    materials: &MaterialLibrary,
    character: Character,
    time: f32,
) {
    if character == Character::Mario {
        update_mario_animation(scene, materials, time);
    }
}

/// Stage provisional para personajes que todavía no tienen su mapa propio.
/// Mantiene la infraestructura visible mientras se construye cada diorama.
fn build_placeholder_stage(character: Character, materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(character.accent());
    add_blocks(&mut scene, materials, build_floating_island());
    scene
}

/// Selecciona el builder visual correspondiente al personaje.
pub fn build_stage(character: Character, materials: &MaterialLibrary) -> Scene {
    match character {
        Character::Mario => build_mario_stage(materials),
        Character::DonkeyKong => build_dk_stage(materials),
        Character::Link
        | Character::Samus
        | Character::Yoshi
        | Character::Kirby
        | Character::Fox
        | Character::Pikachu => build_placeholder_stage(character, materials),
    }
}

/// Stable animated entry point for the app state machine.
pub fn build_stage_at(character: Character, materials: &MaterialLibrary, time: f32) -> Scene {
    let mut scene = build_stage(character, materials);
    update_stage_animation(&mut scene, materials, character, time);
    scene
}
