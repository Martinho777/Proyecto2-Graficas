mod animation;
mod app;
mod camera;
mod cube;
mod cylinder;
mod diorama;
mod framebuffer;
mod material;
mod primitive;
mod prism;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod star;
mod texture;
mod ui;

use animation::AnimationClock;
use app::{AppState, Character};
use camera::Camera;
use framebuffer::Framebuffer;
use material::MaterialLibrary;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use nalgebra_glm::Vec3;
use texture::Texture;

const FRAMEBUFFER_WIDTH: usize = 320;
const FRAMEBUFFER_HEIGHT: usize = 180;
const WINDOW_SCALE: usize = 3;
fn main() {
    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 11.0);
    let mut state = AppState::Title;
    let mut selected = 0usize;
    let clock = AnimationClock::new();
    let materials = MaterialLibrary::load().expect("no se pudieron cargar los materiales");
    let title_screen = Texture::load_ppm("assets/textures/title_screen.ppm")
        .expect("no se pudo cargar la pantalla de inicio");
    let character_sheet = Texture::load_ppm("assets/textures/character_select.ppm")
        .expect("no se pudo cargar la pantalla de seleccion");
    let mut scene = diorama::build_stage(Character::Mario, &materials);
    let mut window = Window::new(
        "Proyecto 2 — Diorama con Raytracing",
        FRAMEBUFFER_WIDTH * WINDOW_SCALE,
        FRAMEBUFFER_HEIGHT * WINDOW_SCALE,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    window.set_target_fps(60);

    while window.is_open() {
        match state {
            AppState::Title => {
                if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
                    break;
                }
                if window.is_key_pressed(Key::Enter, KeyRepeat::No)
                    || window.is_key_pressed(Key::Space, KeyRepeat::No)
                {
                    state = AppState::CharacterSelect;
                }
                ui::render_state(
                    &mut framebuffer,
                    state,
                    selected,
                    &title_screen,
                    &character_sheet,
                );
            }
            AppState::CharacterSelect => {
                if window.is_key_pressed(Key::Left, KeyRepeat::No)
                    || window.is_key_pressed(Key::A, KeyRepeat::No)
                {
                    selected = (selected + Character::ALL.len() - 1) % Character::ALL.len();
                }
                if window.is_key_pressed(Key::Right, KeyRepeat::No)
                    || window.is_key_pressed(Key::D, KeyRepeat::No)
                {
                    selected = (selected + 1) % Character::ALL.len();
                }
                if window.is_key_pressed(Key::Up, KeyRepeat::No)
                    || window.is_key_pressed(Key::W, KeyRepeat::No)
                {
                    selected = (selected + Character::ALL.len() - 4) % Character::ALL.len();
                }
                if window.is_key_pressed(Key::Down, KeyRepeat::No)
                    || window.is_key_pressed(Key::S, KeyRepeat::No)
                {
                    selected = (selected + 4) % Character::ALL.len();
                }
                if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
                    let character = Character::ALL[selected];
                    scene = diorama::build_stage(character, &materials);
                    camera.reset();
                    state = AppState::Diorama(character);
                }
                if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
                    state = AppState::Title;
                }
                ui::render_state(
                    &mut framebuffer,
                    state,
                    selected,
                    &title_screen,
                    &character_sheet,
                );
            }
            AppState::Diorama(character) => {
                camera.update_from_input(&window);
                if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
                    state = AppState::CharacterSelect;
                }
                scene.clear_dynamic();
                diorama::update_stage_animation(&mut scene, &materials, character, clock.seconds());
                renderer::render_raytraced_scene(&mut framebuffer, &camera, &scene);
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .expect("No se pudo actualizar el framebuffer");
    }
}
