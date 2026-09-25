mod camera;
mod cube;
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
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;

const FRAMEBUFFER_WIDTH: usize = 320;
const FRAMEBUFFER_HEIGHT: usize = 180;
const WINDOW_SCALE: usize = 3;
fn main() {
    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 7.0);
    let mut stone = material::Material::from_texture(
        texture::Texture::load_ppm("assets/textures/stone.ppm")
            .expect("textura de piedra invalida"),
    );
    stone.albedo = [0.85, 0.85, 0.85];

    let mut wood = material::Material::from_texture(
        texture::Texture::load_ppm("assets/textures/wood.ppm").expect("textura de madera invalida"),
    );
    wood.albedo = [0.95, 0.75, 0.55];
    wood.specular = 18.0;

    let mut metal = material::Material::from_texture(
        texture::Texture::load_ppm("assets/textures/metal.ppm").expect("textura de metal invalida"),
    );
    metal.albedo = [0.8, 0.9, 1.0];
    metal.specular = 96.0;
    metal.reflectivity = 0.35;

    let mut crystal = material::Material::from_texture(
        texture::Texture::load_ppm("assets/textures/crystal.ppm")
            .expect("textura de cristal invalida"),
    );
    crystal.albedo = [0.55, 0.8, 1.0];
    crystal.specular = 128.0;
    crystal.transparency = 0.7;
    crystal.refractive_index = 1.5;

    let mut lava = material::Material::from_texture(
        texture::Texture::load_ppm("assets/textures/lava.ppm").expect("textura de lava invalida"),
    );
    lava.albedo = [1.0, 0.75, 0.35];
    lava.specular = 24.0;
    lava.emission = [0.55, 0.12, 0.0];

    let mut scene = scene::Scene::new();
    scene.add(Box::new(sphere::Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.5,
        material: crystal,
    }));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(0.0, -1.75, 0.0),
        Vec3::new(5.0, 0.8, 4.0),
        stone.clone(),
    )));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(-1.7, -0.2, 0.0),
        Vec3::new(0.75, 2.5, 0.75),
        wood.clone(),
    )));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(1.7, -0.2, 0.0),
        Vec3::new(0.75, 2.5, 0.75),
        wood,
    )));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(0.0, 1.0, -0.4),
        Vec3::new(2.7, 0.6, 1.7),
        metal,
    )));
    scene.add(Box::new(cube::Cube::from_center_size(
        Vec3::new(0.0, 1.65, -0.4),
        Vec3::new(0.8, 0.35, 0.8),
        lava,
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
