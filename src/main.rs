mod camera;
mod cube;
mod framebuffer;
mod primitive;
mod ray;
mod renderer;
mod scene;
mod sphere;

use camera::Camera;
use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;

const FRAMEBUFFER_WIDTH: usize = 320;
const FRAMEBUFFER_HEIGHT: usize = 180;
const WINDOW_SCALE: usize = 3;
fn main() {
    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 7.0);
    let mut scene = scene::Scene::new();
    scene.add(Box::new(sphere::Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.5,
    }));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(0.0, -1.75, 0.0),
        Vec3::new(5.0, 0.8, 4.0),
    )));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(-1.7, -0.2, 0.0),
        Vec3::new(0.75, 2.5, 0.75),
    )));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(1.7, -0.2, 0.0),
        Vec3::new(0.75, 2.5, 0.75),
    )));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(0.0, 1.0, -0.4),
        Vec3::new(2.7, 0.6, 1.7),
    )));
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
