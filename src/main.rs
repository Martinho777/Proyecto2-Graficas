mod camera;
mod cube;
mod cylinder;
mod diorama;
mod framebuffer;
mod material;
mod primitive;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod texture;

use camera::Camera;
use framebuffer::Framebuffer;
use material::{MaterialId, MaterialLibrary};
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;

const FRAMEBUFFER_WIDTH: usize = 320;
const FRAMEBUFFER_HEIGHT: usize = 180;
const WINDOW_SCALE: usize = 3;
fn main() {
    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 7.0);
    let materials = MaterialLibrary::load().expect("no se pudieron cargar los materiales");
    let mut scene = scene::Scene::new();

    for block in diorama::build_floating_island() {
        scene.add_block(block, &materials);
    }
    for block in diorama::build_forest() {
        scene.add_block(block, &materials);
    }
    for block in diorama::build_bridge() {
        scene.add_block(block, &materials);
    }
    for block in diorama::build_shrine() {
        scene.add_block(block, &materials);
    }

    scene.add(Box::new(sphere::Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.5,
        material: materials.get(MaterialId::Crystal),
    }));
    let mut window = Window::new(
        "Proyecto 2 — Diorama con Raytracing",
        FRAMEBUFFER_WIDTH * WINDOW_SCALE,
        FRAMEBUFFER_HEIGHT * WINDOW_SCALE,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        camera.update_from_input(&window);
        renderer::render_raytraced_scene(&mut framebuffer, &camera, &scene);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .expect("No se pudo actualizar el framebuffer");
    }
}
