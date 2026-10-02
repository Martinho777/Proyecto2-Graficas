use crate::animation::bobbing_height;
use crate::app::Character;
use crate::cube::Cube;
use crate::cylinder::{Cylinder, HalfCylinder, OrientedCylinder, VerticalCylinder};
use crate::material::{MaterialId, MaterialLibrary};
use crate::prism::{LongitudinalTriangularPrism, SlopedTriangularPrism, TriangularPrism};
use crate::scene::{Block, Scene};
use crate::sphere::{Ellipsoid, Sphere};
use crate::star::Star;
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

fn update_dk_animation(scene: &mut Scene, materials: &MaterialLibrary, time: f32) {
    let leaves = materials.get(MaterialId::Leaves);
    for (palm_index, (x, z)) in [(-2.8, 1.35), (2.8, 1.35), (-2.9, -2.1), (2.9, -2.0)]
        .into_iter()
        .enumerate()
    {
        let sway = (time * 1.8 + palm_index as f32 * 0.9).sin() * 0.16;
        let sway_depth = (time * 1.45 + palm_index as f32 * 0.7).cos() * 0.07;
        for (dx, dz, dy, size) in [
            (-0.55, 0.0, 0.0, 1.15),
            (0.35, 0.28, 0.12, 1.0),
            (0.25, -0.42, -0.06, 0.9),
        ] {
            scene.add_dynamic(Box::new(Cube::from_center_size(
                Vec3::new(x + dx + sway, 1.28 + dy, z + dz + sway_depth),
                Vec3::new(size, 0.16, 0.32),
                leaves.clone(),
            )));
        }
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

fn add_dk_letter(scene: &mut Scene, materials: &MaterialLibrary, glyph: &[&str], x: f32) {
    for (row, line) in glyph.iter().enumerate() {
        for (column, pixel) in line.chars().enumerate() {
            if pixel == '#' {
                scene.add_custom_block(
                    Vec3::new(x + column as f32 * 0.14, 1.86 - row as f32 * 0.14, 0.52),
                    Vec3::new(0.12, 0.12, 0.05),
                    materials.banana(),
                );
            }
        }
    }
}

fn build_dk_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0xD66B45);
    add_blocks(&mut scene, materials, build_floating_island());

    let hut = materials.bamboo();
    let door = materials.get(MaterialId::Wood);
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, 0.18, -0.85),
        radius: 1.55,
        height: 1.8,
        material: hut.clone(),
    }));

    let roof = materials.thatch();
    for (y, width, depth) in [
        (1.12, 3.5, 3.1),
        (1.34, 3.0, 2.7),
        (1.56, 2.45, 2.3),
        (1.78, 1.9, 1.85),
        (2.0, 1.35, 1.35),
        (2.22, 0.72, 0.72),
    ] {
        scene.add_custom_block(
            Vec3::new(0.0, y, -0.85),
            Vec3::new(width, 0.25, depth),
            roof.clone(),
        );
    }

    scene.add_custom_block(
        Vec3::new(0.0, -0.02, 0.8),
        Vec3::new(0.9, 1.25, 0.14),
        door.clone(),
    );
    scene.add_custom_block(
        Vec3::new(0.0, 1.55, 0.42),
        Vec3::new(2.4, 0.76, 0.14),
        door.clone(),
    );
    add_dk_letter(
        &mut scene,
        materials,
        &["####.", "#...#", "#...#", "#...#", "####."],
        -0.48,
    );
    add_dk_letter(
        &mut scene,
        materials,
        &["#...#", "#..#.", "###..", "#..#.", "#...#"],
        0.22,
    );

    for (x, z) in [(-2.8, 1.35), (2.8, 1.35), (-2.9, -2.1), (2.9, -2.0)] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, 0.18, z),
            radius: 0.16,
            height: 1.5,
            material: hut.clone(),
        }));
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x + if x < 0.0 { -0.08 } else { 0.08 }, 0.92, z),
            radius: 0.12,
            height: 0.65,
            material: hut.clone(),
        }));
    }

    let bush = materials.get(MaterialId::Leaves);
    for (x, z, scale) in [
        (-3.7, 0.45, 1.0),
        (-2.9, 2.25, 0.8),
        (2.9, 2.15, 0.9),
        (3.75, 0.25, 1.1),
        (-3.65, -1.65, 0.75),
        (3.55, -1.55, 0.8),
    ] {
        scene.add_custom_block(
            Vec3::new(x, -0.35, z),
            Vec3::new(0.9 * scale, 0.55, 0.8 * scale),
            bush.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x + 0.18 * scale, 0.04, z - 0.08),
            Vec3::new(0.62 * scale, 0.38, 0.58 * scale),
            bush.clone(),
        );
    }

    let banana = materials.banana();
    for (x, z) in [
        (-2.0, 1.15),
        (-1.45, 1.75),
        (1.8, 1.2),
        (2.35, 0.55),
        (-3.0, 0.65),
        (2.85, 1.9),
        (-2.55, -0.35),
        (-0.95, 2.25),
        (-0.45, 2.75),
        (0.55, 2.45),
        (1.05, 2.95),
        (-2.4, 2.55),
        (1.9, 2.35),
        (2.65, 2.8),
    ] {
        for (dx, dy, dz, sx, sz) in [
            (-0.2, 0.0, 0.0, 0.38, 0.12),
            (0.0, 0.015, 0.18, 0.12, 0.38),
            (0.2, 0.0, -0.04, 0.34, 0.12),
        ] {
            scene.add_custom_block(
                Vec3::new(x + dx, -0.58 + dy, z + dz),
                Vec3::new(sx, 0.12, sz),
                banana.clone(),
            );
        }
        scene.add_custom_block(
            Vec3::new(x, -0.4, z),
            Vec3::new(0.1, 0.3, 0.1),
            banana.clone(),
        );
    }

    scene
}

fn build_link_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x111B46);
    add_blocks(&mut scene, materials, build_floating_island());

    let bark = materials.deku_bark();
    let leaves = materials.get(MaterialId::Leaves);
    let stone = materials.get(MaterialId::Stone);

    // Camino de piedra que dirige la mirada hacia la espada y el árbol.
    for (index, z) in [2.8, 2.1, 1.4, 0.7, 0.0].into_iter().enumerate() {
        scene.add_custom_block(
            Vec3::new(0.0, -0.57, z),
            Vec3::new(0.82 + index as f32 * 0.08, 0.16, 0.5),
            stone.clone(),
        );
    }

    // Árbol Deku gigante al fondo.
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, 0.95, -1.75),
        radius: 1.35,
        height: 3.35,
        material: bark.clone(),
    }));
    for (x, z, width, depth) in [
        (-1.15, -0.75, 1.3, 0.75),
        (1.15, -0.8, 1.3, 0.75),
        (-0.8, -2.45, 1.1, 0.7),
        (0.9, -2.35, 1.1, 0.7),
    ] {
        scene.add_custom_block(
            Vec3::new(x, -0.28, z),
            Vec3::new(width, 0.5, depth),
            bark.clone(),
        );
    }
    for (x, y, z, size) in [
        (-1.7, 2.45, -1.65, 2.2),
        (1.7, 2.55, -1.75, 2.25),
        (0.0, 3.15, -1.75, 2.9),
        (-0.8, 3.75, -1.7, 2.1),
        (0.85, 3.8, -1.7, 2.0),
        (0.0, 4.3, -1.7, 1.45),
        (-1.15, 3.2, -0.7, 1.5),
        (1.15, 3.25, -2.75, 1.55),
        (-0.8, 4.05, -2.7, 1.35),
        (0.8, 4.1, -0.72, 1.3),
    ] {
        scene.add_custom_block(
            Vec3::new(x, y, z),
            Vec3::new(size, 0.9, size * 0.82),
            leaves.clone(),
        );
    }

    // Cara: ojos entrecerrados, cejas inclinadas y bigote diagonal.
    for x in [-0.38, 0.38] {
        scene.add_custom_block(
            Vec3::new(x, 1.45, -0.34),
            Vec3::new(0.36, 0.08, 0.12),
            materials.plain_crystal(),
        );
    }
    let face_mark = materials.face_mark();
    for (x, y, width) in [
        (-0.62, 1.68, 0.28),
        (-0.38, 1.61, 0.28),
        (0.38, 1.61, 0.28),
        (0.62, 1.68, 0.28),
        (-0.52, 1.18, 0.3),
        (-0.28, 1.1, 0.3),
        (0.28, 1.1, 0.3),
        (0.52, 1.18, 0.3),
    ] {
        scene.add_custom_block(
            Vec3::new(x, y, -0.36),
            Vec3::new(width, 0.1, 0.1),
            face_mark.clone(),
        );
    }
    scene.add_custom_block(
        Vec3::new(0.0, 1.3, -0.36),
        Vec3::new(0.18, 0.2, 0.12),
        bark.clone(),
    );

    // Master Sword clavada frente al árbol.
    let metal = materials.metal();
    scene.add_custom_block(
        Vec3::new(0.0, -0.5, 2.0),
        Vec3::new(1.9, 0.35, 1.55),
        stone.clone(),
    );
    scene.add_custom_block(
        Vec3::new(0.0, -0.27, 2.0),
        Vec3::new(1.45, 0.22, 1.15),
        stone.clone(),
    );
    scene.add_custom_block(
        Vec3::new(0.0, 0.42, 2.0),
        Vec3::new(0.16, 1.35, 0.12),
        metal,
    );
    scene.add_custom_block(
        Vec3::new(0.0, 1.15, 2.0),
        Vec3::new(0.72, 0.14, 0.2),
        materials.banana(),
    );
    scene.add_custom_block(Vec3::new(0.0, 1.42, 2.0), Vec3::new(0.18, 0.38, 0.18), bark);

    let shrub = materials.get(MaterialId::Leaves);
    for (x, z, scale) in [
        (-3.4, 1.5, 0.9),
        (3.25, 1.15, 0.8),
        (-3.0, -0.8, 0.75),
        (3.35, -1.1, 0.9),
    ] {
        scene.add_custom_block(
            Vec3::new(x, -0.52, z),
            Vec3::new(0.9 * scale, 0.55, 0.75 * scale),
            shrub.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x + 0.14, -0.02, z - 0.04),
            Vec3::new(0.58 * scale, 0.34, 0.5 * scale),
            shrub.clone(),
        );
    }

    scene
}

fn build_samus_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x04071A);
    let mut industrial_island = build_floating_island();
    for block in &mut industrial_island {
        if matches!(block.material, MaterialId::Leaves) {
            block.material = MaterialId::Wood;
        }
    }
    add_blocks(&mut scene, materials, industrial_island);
    let metal = materials.metal();
    let energy = materials.energy_blue();

    // Camino de tierra con balizas luminosas directamente sobre el piso.
    for (index, z) in [2.7, 2.1, 1.5, 0.9, 0.3].into_iter().enumerate() {
        scene.add_custom_block(
            Vec3::new(0.0, -0.37, z),
            Vec3::new(0.8 + index as f32 * 0.08, 0.12, 0.48),
            materials.get(MaterialId::Wood),
        );
        for x in [-0.7, 0.7] {
            scene.add_custom_block(
                Vec3::new(x, -0.635, z),
                Vec3::new(0.28, 0.08, 0.28),
                metal.clone(),
            );
            scene.add_custom_block(
                Vec3::new(x, -0.575, z),
                Vec3::new(0.20, 0.05, 0.20),
                energy.clone(),
            );
        }
    }

    // Barriles de acero con derrames fosforescentes sobre la tierra.
    for (x, z, spill_direction) in [(-2.55, 0.85, 1.0), (2.45, 1.55, -1.0)] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, -0.385, z),
            radius: 0.34,
            height: 0.58,
            material: metal.clone(),
        }));
        for y in [-0.595, -0.185] {
            scene.add_custom_block(
                Vec3::new(x, y, z),
                Vec3::new(0.72, 0.07, 0.72),
                metal.clone(),
            );
        }
        let _ = spill_direction;
    }

    // Solo tres energy towers, cada una con una punta azulada bien visible.
    for (x, z, height) in [(-1.45, -0.7, 0.95), (1.45, -0.7, 0.95), (0.0, -1.45, 1.15)] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, height * 0.5 - 0.675, z),
            radius: 0.40,
            height,
            material: metal.clone(),
        }));
        let _ = (x, z, height, energy.clone());
    }

    // Huevos alienígenas en parejas alrededor de la base.
    let alien_egg = materials.alien_egg();
    for (x, z) in [(-3.05, -1.35), (3.0, -0.95), (-2.85, 2.05), (2.75, 2.35)] {
        for (offset_x, offset_z) in [(-0.18, -0.10), (0.18, 0.10)] {
            scene.add(Box::new(Sphere {
                center: Vec3::new(x + offset_x, -0.455, z + offset_z),
                radius: 0.22,
                material: alien_egg.clone(),
            }));
        }
    }

    // Ridley gigante al fondo, construido como una silueta compacta y reconocible.
    let ridley = materials.alien_egg();
    let eye = materials.ridley_eye();
    scene.add(Box::new(Sphere {
        center: Vec3::new(0.0, 1.75, -3.8),
        radius: 0.92,
        material: ridley.clone(),
    }));
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, 2.65, -3.65),
        radius: 0.44,
        height: 1.35,
        material: ridley.clone(),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::new(0.0, 3.45, -3.5),
        radius: 0.76,
        material: ridley.clone(),
    }));
    scene.add_custom_block(
        Vec3::new(0.0, 3.18, -2.75),
        Vec3::new(0.9, 0.42, 0.78),
        ridley.clone(),
    );
    for x in [-0.42, 0.42] {
        scene.add(Box::new(Sphere {
            center: Vec3::new(x, 3.58, -2.62),
            radius: 0.10,
            material: eye.clone(),
        }));
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, 4.35, -3.55),
            radius: 0.12,
            height: 0.92,
            material: ridley.clone(),
        }));
    }
    // Patas delgadas y pies apoyados detrás de la isla.
    for x in [-0.34, 0.34] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, 0.12, -3.65),
            radius: 0.22,
            height: 1.55,
            material: ridley.clone(),
        }));
        scene.add_custom_block(
            Vec3::new(x, -0.585, -3.65),
            Vec3::new(0.48, 0.18, 0.78),
            ridley.clone(),
        );
    }

    // Cola segmentada, curvada hacia la parte posterior de la criatura.
    for (center, radius) in [
        (Vec3::new(0.0, 1.25, -4.75), 0.38),
        (Vec3::new(0.22, 1.02, -5.35), 0.28),
        (Vec3::new(0.48, 0.82, -5.85), 0.18),
    ] {
        scene.add(Box::new(Sphere {
            center,
            radius,
            material: ridley.clone(),
        }));
    }
    scene.add_custom_block(
        Vec3::new(0.64, 0.70, -6.15),
        Vec3::new(0.16, 0.16, 0.55),
        ridley.clone(),
    );

    scene
}

fn build_yoshi_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x7BCBFF);
    add_blocks(&mut scene, materials, build_floating_island());

    let nest = materials.get(MaterialId::Wood);
    let leaves = materials.get(MaterialId::Leaves);
    let egg = materials.yoshi_egg();

    // Nido circular de ramas en el centro de la isla.
    for (x, z) in [
        (-0.95, 0.0),
        (-0.68, -0.62),
        (0.0, -0.88),
        (0.68, -0.62),
        (0.95, 0.0),
        (0.68, 0.62),
        (0.0, 0.88),
        (-0.68, 0.62),
    ] {
        scene.add_custom_block(
            Vec3::new(x, -0.38, z),
            Vec3::new(0.62, 0.22, 0.34),
            nest.clone(),
        );
    }
    scene.add_custom_block(
        Vec3::new(0.0, -0.22, 0.0),
        Vec3::new(1.45, 0.18, 1.45),
        leaves.clone(),
    );

    // Huevo de Yoshi protagonista, con manchas verdes visibles al frente.
    let _ = egg;
    // Arbustos suaves para enmarcar el nido.
    for (x, z, scale) in [
        (-3.0, 1.5, 1.0),
        (3.0, 1.25, 0.9),
        (-2.8, -1.9, 0.85),
        (2.85, -2.0, 1.0),
    ] {
        scene.add_custom_block(
            Vec3::new(x, -0.37, z),
            Vec3::new(1.05 * scale, 0.58, 0.8 * scale),
            leaves.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x + 0.14, 0.08, z - 0.04),
            Vec3::new(0.68 * scale, 0.38, 0.58 * scale),
            leaves.clone(),
        );
    }

    // Flores coloridas alrededor del nido.
    let stem = materials.bamboo();
    let pink = materials.flower_pink();
    let yellow = materials.banana();
    for (x, z, material) in [
        (-1.85, 1.15, pink.clone()),
        (1.75, 1.35, yellow.clone()),
        (-1.85, -1.45, yellow.clone()),
        (1.85, -1.35, pink.clone()),
    ] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, -0.20, z),
            radius: 0.055,
            height: 0.95,
            material: stem.clone(),
        }));
        for (petal_x, petal_z) in [(-0.16, 0.0), (0.16, 0.0), (0.0, -0.16), (0.0, 0.16)] {
            scene.add(Box::new(Sphere {
                center: Vec3::new(x + petal_x, 0.3, z + petal_z),
                radius: 0.13,
                material: material.clone(),
            }));
        }
        scene.add(Box::new(Sphere {
            center: Vec3::new(x, 0.3, z),
            radius: 0.09,
            material: yellow.clone(),
        }));
    }

    // Relleno natural en el sector frontal derecho de la isla.
    for (x, z, scale) in [(2.35, 0.25, 0.82), (3.25, 0.45, 0.68)] {
        scene.add_custom_block(
            Vec3::new(x, -0.37, z),
            Vec3::new(1.05 * scale, 0.58, 0.78 * scale),
            leaves.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x - 0.12, 0.08, z - 0.05),
            Vec3::new(0.62 * scale, 0.36, 0.52 * scale),
            leaves.clone(),
        );
    }
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(2.65, -0.20, -0.72),
        radius: 0.055,
        height: 0.95,
        material: stem.clone(),
    }));
    for (petal_x, petal_z) in [(-0.16, 0.0), (0.16, 0.0), (0.0, -0.16), (0.0, 0.16)] {
        scene.add(Box::new(Sphere {
            center: Vec3::new(2.65 + petal_x, 0.3, -0.72 + petal_z),
            radius: 0.13,
            material: pink.clone(),
        }));
    }
    scene.add(Box::new(Sphere {
        center: Vec3::new(2.65, 0.3, -0.72),
        radius: 0.09,
        material: yellow.clone(),
    }));

    // Vegetación adicional para cerrar los espacios abiertos de la isla.
    for (x, z, scale) in [(-2.25, 2.35, 0.72), (2.2, 2.45, 0.62)] {
        scene.add_custom_block(
            Vec3::new(x, -0.37, z),
            Vec3::new(0.9 * scale, 0.5, 0.7 * scale),
            leaves.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x + 0.1, 0.05, z),
            Vec3::new(0.55 * scale, 0.3, 0.46 * scale),
            leaves.clone(),
        );
    }

    // Vegetación repartida por todo el perímetro, también detrás del nido.
    for (x, z, scale) in [
        (-3.45, -0.35, 0.62),
        (3.4, -0.25, 0.68),
        (-2.35, -2.45, 0.58),
        (2.35, -2.55, 0.62),
        (-1.2, 2.85, 0.55),
        (1.35, 2.9, 0.58),
    ] {
        scene.add_custom_block(
            Vec3::new(x, -0.37, z),
            Vec3::new(0.92 * scale, 0.5, 0.7 * scale),
            leaves.clone(),
        );
        scene.add_custom_block(
            Vec3::new(x + 0.1, 0.04, z - 0.04),
            Vec3::new(0.58 * scale, 0.3, 0.48 * scale),
            leaves.clone(),
        );
    }
    for (index, (x, z)) in [
        (-2.75, -0.65),
        (2.85, -0.55),
        (-1.55, -2.55),
        (1.55, -2.6),
        (-2.55, 2.55),
        (2.55, 2.65),
    ]
    .into_iter()
    .enumerate()
    {
        let petals = if index % 2 == 0 {
            pink.clone()
        } else {
            yellow.clone()
        };
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, -0.20, z),
            radius: 0.05,
            height: 0.9,
            material: stem.clone(),
        }));
        for (petal_x, petal_z) in [(-0.14, 0.0), (0.14, 0.0), (0.0, -0.14), (0.0, 0.14)] {
            scene.add(Box::new(Sphere {
                center: Vec3::new(x + petal_x, 0.28, z + petal_z),
                radius: 0.115,
                material: petals.clone(),
            }));
        }
        scene.add(Box::new(Sphere {
            center: Vec3::new(x, 0.28, z),
            radius: 0.075,
            material: yellow.clone(),
        }));
    }

    // Detalles finales cute: hongos y piedras alrededor del nido.
    let stone = materials.get(MaterialId::Stone);
    for (x, z, cap) in [
        (-2.25, 0.35, pink.clone()),
        (2.35, -0.05, yellow.clone()),
        (0.15, -2.55, pink.clone()),
    ] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, -0.525, z),
            radius: 0.12,
            height: 0.3,
            material: stone.clone(),
        }));
        scene.add(Box::new(Sphere {
            center: Vec3::new(x, -0.31, z),
            radius: 0.22,
            material: cap,
        }));
    }
    for (x, z, size) in [
        (-1.35, 2.4, Vec3::new(0.42, 0.18, 0.3)),
        (1.2, -2.35, Vec3::new(0.5, 0.2, 0.34)),
        (3.15, -1.15, Vec3::new(0.38, 0.16, 0.26)),
        (-3.1, 1.0, Vec3::new(0.36, 0.16, 0.28)),
    ] {
        scene.add_custom_block(Vec3::new(x, -0.57, z), size, stone.clone());
    }

    scene
}

fn build_kirby_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x111B46);
    let stone = materials.kirby_base();
    let crystal = materials.kirby_crystal();
    let water = materials.water();

    // Plataforma circular principal, inspirada en un escenario cósmico.
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, -0.72, 0.0),
        radius: 4.7,
        height: 0.82,
        material: stone.clone(),
    }));
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, -0.28, 0.0),
        radius: 4.58,
        height: 0.10,
        material: materials.kirby_base(),
    }));
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, -0.20, 0.0),
        radius: 4.5,
        height: 0.07,
        material: water,
    }));
    // Cristales verticales para enmarcar el agua sin saturar la escena.
    for (x, z, height, radius) in [
        (-2.8, -1.7, 1.8, 0.42),
        (2.7, -1.4, 2.2, 0.48),
        (-2.5, 1.8, 1.45, 0.36),
        (2.4, 2.0, 1.7, 0.4),
    ] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, height * 0.5 - 0.18, z),
            radius,
            height,
            material: crystal.clone(),
        }));
        // Punta escalonada tipo castillo en cada torre de la orilla.
        for layer in 0..3 {
            let layer_height = 0.16;
            scene.add(Box::new(VerticalCylinder {
                center: Vec3::new(
                    x,
                    height - 0.18 + layer_height * 0.5 + layer as f32 * layer_height,
                    z,
                ),
                radius: radius * (0.82 - layer as f32 * 0.22),
                height: layer_height,
                material: crystal.clone(),
            }));
        }
    }
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, 0.18, 0.0),
        radius: 0.45,
        height: 0.72,
        material: crystal,
    }));

    // Anillos luminosos sobre las cuatro torres exteriores.
    let ring = materials.navi();
    for (x, z, height, radius) in [
        (-2.8, -1.7, 1.8, 0.42),
        (2.7, -1.4, 2.2, 0.48),
        (-2.5, 1.8, 1.45, 0.36),
        (2.4, 2.0, 1.7, 0.4),
    ] {
        let ring_y = height + 0.34;
        for index in 0..8 {
            let angle = index as f32 * std::f32::consts::TAU / 8.0;
            scene.add(Box::new(Sphere {
                center: Vec3::new(
                    x + angle.cos() * radius * 0.92,
                    ring_y,
                    z + angle.sin() * radius * 0.92,
                ),
                radius: 0.095,
                material: ring.clone(),
            }));
        }
    }

    scene
}

fn build_fox_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x04071A);
    let hull = materials.fox_hull();
    let panel = materials.fox_panel();
    let engine = materials.fox_engine();
    let accent = materials.fox_accent();

    // Cuerpo central eliptico, con el volumen robusto aprobado anteriormente.
    scene.add(Box::new(Ellipsoid {
        center: Vec3::new(0.0, 0.55, 0.0),
        radii: Vec3::new(1.48, 0.78, 2.10),
        material: hull.clone(),
    }));

    // Nariz angular inferior y alas triangulares extruidas: no son simples
    // quads, tienen caras laterales y espesor visible.
    scene.add(Box::new(TriangularPrism {
        points: [
            Vec3::new(-1.15, 0.95, 0.65),
            Vec3::new(0.0, 2.50, 0.65),
            Vec3::new(1.15, 0.95, 0.65),
        ],
        center_y: 0.08,
        half_height: 0.22,
        material: panel.clone(),
    }));
    scene.add(Box::new(SlopedTriangularPrism {
        top_points: [
            Vec3::new(-0.58, 0.48, 0.95),
            Vec3::new(-4.25, -0.58, -1.35),
            Vec3::new(-1.00, 0.05, -1.58),
        ],
        thickness: 0.22,
        material: hull.clone(),
    }));
    scene.add(Box::new(SlopedTriangularPrism {
        top_points: [
            Vec3::new(0.58, 0.48, 0.95),
            Vec3::new(4.25, -0.58, -1.35),
            Vec3::new(1.00, 0.05, -1.58),
        ],
        thickness: 0.22,
        material: hull.clone(),
    }));

    // Mini alas superiores, ligeramente mas grandes y levantadas hacia las
    // puntas para darle al Arwing su segundo nivel de silueta.
    scene.add(Box::new(SlopedTriangularPrism {
        top_points: [
            Vec3::new(-0.54, 0.88, 0.62),
            Vec3::new(-2.15, 1.45, -0.28),
            Vec3::new(-0.88, 1.08, -0.78),
        ],
        thickness: 0.18,
        material: hull.clone(),
    }));
    scene.add(Box::new(SlopedTriangularPrism {
        top_points: [
            Vec3::new(0.54, 0.88, 0.62),
            Vec3::new(2.15, 1.45, -0.28),
            Vec3::new(0.88, 1.08, -0.78),
        ],
        thickness: 0.18,
        material: hull.clone(),
    }));

    // Ala dorsal rotada 90 grados sobre X: queda alineada con el fuselaje,
    // como una aleta de tiburon, y conserva el mismo material de las alas.
    scene.add(Box::new(LongitudinalTriangularPrism {
        points: [
            Vec3::new(0.0, 0.72, -1.58),
            Vec3::new(0.0, 2.72, -0.48),
            Vec3::new(0.0, 0.72, 0.18),
        ],
        center_x: 0.0,
        half_width: 0.18,
        material: hull.clone(),
    }));

    // Paneles rojos sobre las alas y aleta posterior para darle identidad al
    // Arwing sin depender de una textura externa.
    scene.add(Box::new(SlopedTriangularPrism {
        top_points: [
            Vec3::new(-0.82, 0.52, 0.82),
            Vec3::new(-2.55, -0.35, -0.72),
            Vec3::new(-1.05, 0.12, -0.90),
        ],
        thickness: 0.018,
        material: accent.clone(),
    }));
    scene.add(Box::new(SlopedTriangularPrism {
        top_points: [
            Vec3::new(0.82, 0.52, 0.82),
            Vec3::new(2.55, -0.35, -0.72),
            Vec3::new(1.05, 0.12, -0.90),
        ],
        thickness: 0.018,
        material: accent.clone(),
    }));

    scene.add_custom_block(
        Vec3::new(0.0, 0.82, -0.72),
        Vec3::new(0.18, 0.92, 1.08),
        panel.clone(),
    );
    scene.add_custom_block(
        Vec3::new(0.0, 1.28, -0.82),
        Vec3::new(0.14, 0.14, 0.68),
        accent.clone(),
    );

    // Cuello diagonal que nace en la parte frontal del fuselaje y avanza
    // hacia adelante antes de sostener la cabina.
    scene.add(Box::new(OrientedCylinder {
        center: Vec3::new(0.0, 1.00, 2.35),
        axis: Vec3::new(0.0, 0.50, 0.866),
        radius: 0.34,
        height: 1.85,
        material: hull.clone(),
    }));
    // Cabina temporalmente hecha del mismo material del fuselaje; el cristal
    // se puede reincorporar despues cuando la silueta quede aprobada.
    scene.add(Box::new(Ellipsoid {
        center: Vec3::new(0.0, 1.67, 3.25),
        radii: Vec3::new(0.56, 0.30, 0.62),
        material: hull.clone(),
    }));
    // Ventana frontal pequena en la cabeza de la nave.
    scene.add_custom_block(
        Vec3::new(0.0, 1.67, 3.875),
        Vec3::new(0.62, 0.18, 0.06),
        materials.plain_crystal(),
    );

    // Carcasas y anillos traseros de los dos motores.
    for x in [-1.42, 1.42] {
        scene.add(Box::new(Ellipsoid {
            center: Vec3::new(x, 0.25, -1.22),
            radii: Vec3::new(0.54, 0.48, 0.66),
            material: panel.clone(),
        }));
        scene.add(Box::new(Cylinder {
            center: Vec3::new(x, 0.25, -1.56),
            radius: 0.36,
            height: 0.28,
            material: hull.clone(),
        }));
        scene.add(Box::new(Cylinder {
            center: Vec3::new(x, 0.25, -1.72),
            radius: 0.25,
            height: 0.10,
            material: engine.clone(),
        }));
    }

    scene
}

fn build_pikachu_stage(materials: &MaterialLibrary) -> Scene {
    let mut scene = Scene::with_background(0x060A18);
    let metal = materials.stadium_metal();
    let field = materials.stadium_field();
    let line = materials.stadium_line();
    let dark_structure = materials.fox_panel();

    // Plataforma y arena principal inspiradas en Pokemon Stadium.
    scene.add_custom_block(
        Vec3::new(0.0, -1.10, 0.0),
        Vec3::new(8.8, 1.15, 6.5),
        metal.clone(),
    );
    scene.add_custom_block(
        Vec3::new(0.0, -0.48, 0.0),
        Vec3::new(7.45, 0.16, 5.25),
        field,
    );

    for (center, size) in [
        (Vec3::new(0.0, -0.34, -2.72), Vec3::new(8.0, 0.28, 0.30)),
        (Vec3::new(0.0, -0.34, 2.72), Vec3::new(8.0, 0.28, 0.30)),
        (Vec3::new(-3.90, -0.34, 0.0), Vec3::new(0.30, 0.28, 5.5)),
        (Vec3::new(3.90, -0.34, 0.0), Vec3::new(0.30, 0.28, 5.5)),
    ] {
        scene.add_custom_block(center, size, metal.clone());
    }

    // Unica marca del piso: Pokeball con dos semicirculos reales.
    scene.add(Box::new(HalfCylinder {
        center: Vec3::new(0.0, -0.375, 0.0),
        radius: 0.86,
        height: 0.06,
        upper_z_half: true,
        material: line.clone(),
    }));
    scene.add(Box::new(HalfCylinder {
        center: Vec3::new(0.0, -0.33, 0.0),
        radius: 0.86,
        height: 0.06,
        upper_z_half: false,
        material: materials.fox_accent(),
    }));
    scene.add_custom_block(
        Vec3::new(0.0, -0.255, 0.0),
        Vec3::new(1.78, 0.07, 0.10),
        materials.face_mark(),
    );
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, -0.20, 0.0),
        radius: 0.15,
        height: 0.08,
        material: materials.face_mark(),
    }));
    scene.add(Box::new(VerticalCylinder {
        center: Vec3::new(0.0, -0.145, 0.0),
        radius: 0.085,
        height: 0.045,
        material: line,
    }));

    // Pantalla, torres y luminarias del fondo.
    scene.add_custom_block(
        Vec3::new(0.0, 1.35, -3.20),
        Vec3::new(3.7, 1.65, 0.24),
        metal.clone(),
    );
    scene.add_custom_block(
        Vec3::new(0.0, 1.38, -3.06),
        Vec3::new(2.9, 1.05, 0.06),
        metal.clone(),
    );
    scene.add_custom_block(
        Vec3::new(0.0, 1.35, -3.52),
        Vec3::new(10.0, 3.8, 0.18),
        dark_structure,
    );
    for x in [-3.25, 3.25] {
        scene.add(Box::new(VerticalCylinder {
            center: Vec3::new(x, 0.45, -2.45),
            radius: 0.26,
            height: 1.75,
            material: metal.clone(),
        }));
        scene.add_custom_block(
            Vec3::new(x, 1.37, -2.45),
            Vec3::new(0.62, 0.16, 0.42),
            metal.clone(),
        );
    }
    for x in [-2.45, -1.55, 1.55, 2.45] {
        scene.add_custom_block(
            Vec3::new(x, 0.12, -2.55),
            Vec3::new(0.56, 0.12, 0.18),
            metal.clone(),
        );
    }

    scene
}

fn update_kirby_animation(scene: &mut Scene, materials: &MaterialLibrary, time: f32) {
    let star_y = 1.65 + (time * 1.15).sin() * 0.16;
    scene.navi_light_position = Some(Vec3::new(0.0, star_y, 0.0));
    scene.add_dynamic(Box::new(Star {
        center: Vec3::new(0.0, star_y, 0.0),
        outer_radius: 0.98,
        inner_radius: 0.43,
        depth: 0.22,
        material: materials.kirby_star(),
    }));

    // Cascadas animadas: flujo con ancho variable, gotas descendentes y salpicaduras ascendentes.
    for (cascade_index, (x, z)) in [(-2.35, 3.84), (0.0, 4.5), (2.35, 3.84)]
        .into_iter()
        .enumerate()
    {
        let pulse = 0.88 + 0.12 * (time * 2.0 + cascade_index as f32).sin();
        scene.add_dynamic(Box::new(VerticalCylinder {
            center: Vec3::new(x, -1.4, z),
            radius: 0.32 * pulse,
            height: 2.5,
            material: materials.water(),
        }));
        scene.add_dynamic(Box::new(VerticalCylinder {
            center: Vec3::new(x, -2.9, z),
            radius: 0.22 * pulse,
            height: 0.65,
            material: materials.water(),
        }));

        let fall_phase = (time * 0.9 + cascade_index as f32 * 0.37).rem_euclid(1.0);
        for drop_index in 0..3 {
            let phase = (fall_phase + drop_index as f32 * 0.31).rem_euclid(1.0);
            scene.add_dynamic(Box::new(Sphere {
                center: Vec3::new(
                    x + (drop_index as f32 - 1.0) * 0.12,
                    0.0 - phase * 2.65,
                    z + 0.2,
                ),
                radius: 0.07,
                material: materials.water(),
            }));
        }

        let foam = materials.foam();
        for offset in [-0.18, 0.0, 0.18] {
            scene.add_dynamic(Box::new(Sphere {
                center: Vec3::new(x + offset, -3.27, z),
                radius: 0.15,
                material: foam.clone(),
            }));
        }
        for splash_index in 0..2 {
            let splash_phase =
                (time * 1.2 + cascade_index as f32 * 0.4 + splash_index as f32 * 0.5)
                    .rem_euclid(1.0);
            scene.add_dynamic(Box::new(Sphere {
                center: Vec3::new(
                    x + (splash_index as f32 - 0.5) * 0.25,
                    -3.2 + splash_phase * 0.72,
                    z - 0.22,
                ),
                radius: 0.065,
                material: materials.water(),
            }));
        }
    }

    let wave = (time * 1.4).sin() * 0.16;
    for radius in [0.9 + wave, 1.55 - wave * 0.6] {
        scene.add_dynamic(Box::new(VerticalCylinder {
            center: Vec3::new(0.0, -0.145, 0.0),
            radius,
            height: 0.018,
            material: materials.water(),
        }));
    }
}

fn update_yoshi_animation(scene: &mut Scene, materials: &MaterialLibrary, time: f32) {
    let sway = (time * 1.35).sin();
    let center = Vec3::new(sway * 0.07, 0.55 + sway.abs() * 0.025, sway * 0.025);
    scene.add_dynamic(Box::new(Ellipsoid {
        center,
        radii: Vec3::new(0.68, 0.92, 0.68),
        material: materials.yoshi_egg(),
    }));
}

fn update_link_animation(scene: &mut Scene, materials: &MaterialLibrary, time: f32) {
    let navi = materials.navi();
    let center = Vec3::new(
        (time * 0.7).cos() * 3.15,
        1.15 + (time * 1.8).sin() * 0.18,
        0.1 + (time * 0.7).sin() * 3.05,
    );
    scene.navi_light_position = Some(center);
    scene.add_dynamic(Box::new(Sphere {
        center,
        radius: 0.24,
        material: navi,
    }));

    let rupee = materials.rupee();
    for (index, (x, z)) in [(-2.3, 1.9), (2.1, 1.75), (-2.9, 0.25), (2.8, -0.55)]
        .into_iter()
        .enumerate()
    {
        let bob = bobbing_height(time, index as f32 * 0.8, 0.1, 1.6);
        for (center, size) in [
            (Vec3::new(x, -0.4 + bob, z), Vec3::new(0.2, 0.5, 0.12)),
            (Vec3::new(x, -0.68 + bob, z), Vec3::new(0.12, 0.12, 0.1)),
            (Vec3::new(x, -0.12 + bob, z), Vec3::new(0.12, 0.12, 0.1)),
        ] {
            scene.add_dynamic(Box::new(Cube::from_center_size(
                center,
                size,
                rupee.clone(),
            )));
        }
    }
}

fn update_samus_animation(scene: &mut Scene, materials: &MaterialLibrary, time: f32) {
    let pulse = 0.28 + 0.72 * (0.5 + 0.5 * (time * 2.4).sin());
    let mut energy = materials.energy_blue();
    energy.emission = [0.08 * pulse, 0.45 * pulse, 1.4 * pulse];
    for (x, z, height) in [(-1.45, -0.7, 0.95), (1.45, -0.7, 0.95), (0.0, -1.45, 1.15)] {
        scene.add_dynamic(Box::new(Cube::from_center_size(
            Vec3::new(x, height - 0.595, z),
            Vec3::new(0.78, 0.16, 0.78),
            energy.clone(),
        )));
    }

    let mut phosphor = materials.phosphor_green();
    let green_pulse = 0.22 + 0.78 * (0.5 + 0.5 * (time * 3.0).sin());
    phosphor.emission = [0.35 * green_pulse, 2.2 * green_pulse, 0.18 * green_pulse];
    for (x, z, spill_direction) in [(-2.55, 0.85, 1.0), (2.45, 1.55, -1.0)] {
        for (index, distance) in [0.22, 0.58, 0.94].into_iter().enumerate() {
            scene.add_dynamic(Box::new(Cube::from_center_size(
                Vec3::new(
                    x + spill_direction * distance,
                    -0.645,
                    z + 0.06 * index as f32,
                ),
                Vec3::new(0.42 - index as f32 * 0.08, 0.06, 0.26),
                phosphor.clone(),
            )));
        }
    }

    let wing_lift = (time * 1.2).sin() * 0.24;
    let ridley = materials.alien_egg();
    for (x, z, size) in [
        (-1.45, -3.7, Vec3::new(2.4, 0.18, 1.35)),
        (1.45, -3.7, Vec3::new(2.4, 0.18, 1.35)),
        (-2.35, -3.45, Vec3::new(1.2, 0.16, 0.9)),
        (2.35, -3.45, Vec3::new(1.2, 0.16, 0.9)),
    ] {
        scene.add_dynamic(Box::new(Cube::from_center_size(
            Vec3::new(x, 2.15 + wing_lift * x.signum(), z),
            size,
            ridley.clone(),
        )));
    }
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
    } else if character == Character::DonkeyKong {
        update_dk_animation(scene, materials, time);
    } else if character == Character::Link {
        update_link_animation(scene, materials, time);
    } else if character == Character::Samus {
        update_samus_animation(scene, materials, time);
    } else if character == Character::Yoshi {
        update_yoshi_animation(scene, materials, time);
    } else if character == Character::Kirby {
        update_kirby_animation(scene, materials, time);
    } else if character == Character::Fox {
        update_fox_animation(scene, materials, time);
    } else if character == Character::Pikachu {
        update_pikachu_animation(scene, materials, time);
    }
}

fn update_fox_animation(scene: &mut Scene, materials: &MaterialLibrary, time: f32) {
    let pulse = 0.86 + 0.14 * (time * 6.0).sin();
    for x in [-1.42, 1.42] {
        let mut engine = materials.fox_engine();
        engine.emission = [2.2 * pulse, 0.24 * pulse, 0.015];
        scene.add_dynamic(Box::new(Ellipsoid {
            center: Vec3::new(x, 0.25, -2.02),
            radii: Vec3::new(0.28 * pulse, 0.28 * pulse, 0.52 * pulse),
            material: engine,
        }));
    }
}

fn update_pikachu_animation(scene: &mut Scene, materials: &MaterialLibrary, time: f32) {
    let pulse = 0.82 + 0.18 * (time * 5.0).sin().abs();
    let mut electric = materials.pika_electric();
    electric.emission = [3.2 * pulse, 1.8 * pulse, 0.08];

    // Rayos cortos en zigzag alrededor de la arena.
    for (bolt_index, (x, z)) in [(-1.35, 1.15), (2.75, 0.15)].into_iter().enumerate() {
        for segment in 0..5 {
            let offset_x = if segment % 2 == 0 { -0.14 } else { 0.14 };
            let triangle_x = x + offset_x;
            let triangle_z = z + bolt_index as f32 * 0.08;
            scene.add_dynamic(Box::new(TriangularPrism {
                points: [
                    Vec3::new(triangle_x - 0.15 * pulse, 0.0, triangle_z + 0.14),
                    Vec3::new(triangle_x + 0.15 * pulse, 0.0, triangle_z + 0.14),
                    Vec3::new(triangle_x, 0.0, triangle_z - 0.22),
                ],
                center_y: -0.06 + segment as f32 * 0.27,
                half_height: 0.06,
                material: electric.clone(),
            }));
        }
    }

    // Llamas compactas en las esquinas, reutilizando el mismo material
    // emisivo y la misma forma eliptica de los propulsores de Fox.
    for (index, (x, z)) in [(-2.75, 1.95), (2.75, -1.85)].into_iter().enumerate() {
        let flame_scale = 0.88 + 0.12 * (time * 4.0 + index as f32).sin();
        let mut fire = materials.fox_engine();
        fire.emission = [3.2 * flame_scale, 0.45 * flame_scale, 0.025];
        scene.add_dynamic(Box::new(Ellipsoid {
            center: Vec3::new(x, 0.10, z),
            radii: Vec3::new(0.22 * flame_scale, 0.46 * flame_scale, 0.22 * flame_scale),
            material: fire,
        }));
    }

    // Rocas del lado excavado: tiemblan en su sitio con desplazamientos
    // pequenos para sugerir que la tierra aun se esta moviendo.
    let excavated_rock = materials.get(MaterialId::Stone);
    for (index, (center, size)) in [
        (Vec3::new(2.25, -0.22, 0.62), Vec3::new(0.42, 0.34, 0.38)),
        (Vec3::new(2.62, -0.14, 0.76), Vec3::new(0.32, 0.48, 0.30)),
        (Vec3::new(2.35, -0.02, 1.04), Vec3::new(0.28, 0.34, 0.26)),
        (Vec3::new(2.85, -0.26, 0.58), Vec3::new(0.24, 0.28, 0.30)),
        (Vec3::new(1.92, -0.27, 0.70), Vec3::new(0.28, 0.24, 0.34)),
    ]
    .into_iter()
    .enumerate()
    {
        let shake = (time * 9.0 + index as f32 * 1.7).sin() * 0.035;
        let lift = (time * 12.0 + index as f32).sin().abs() * 0.018;
        scene.add_dynamic(Box::new(Cube::from_center_size(
            Vec3::new(center.x + shake, center.y + lift, center.z - shake * 0.5),
            size,
            excavated_rock.clone(),
        )));
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
        Character::Link => build_link_stage(materials),
        Character::Samus => build_samus_stage(materials),
        Character::Yoshi => build_yoshi_stage(materials),
        Character::Kirby => build_kirby_stage(materials),
        Character::Fox => build_fox_stage(materials),
        Character::Pikachu => build_pikachu_stage(materials),
    }
}

/// Stable animated entry point for the app state machine.
pub fn build_stage_at(character: Character, materials: &MaterialLibrary, time: f32) -> Scene {
    let mut scene = build_stage(character, materials);
    update_stage_animation(&mut scene, materials, character, time);
    scene
}
