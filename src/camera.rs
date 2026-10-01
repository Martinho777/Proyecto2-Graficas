use minifb::{Key, Window};
use nalgebra_glm::{cross, normalize, Vec3};

const MIN_PITCH: f32 = -1.35;
const MAX_PITCH: f32 = 1.35;
const MIN_DISTANCE: f32 = 2.0;
const MAX_DISTANCE: f32 = 16.0;

pub struct Camera {
    pub target: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

impl Camera {
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            yaw: 0.0,
            pitch: 0.35,
            distance: distance.clamp(MIN_DISTANCE, MAX_DISTANCE),
        }
    }

    pub fn eye(&self) -> Vec3 {
        let horizontal = self.distance * self.pitch.cos();

        self.target
            + Vec3::new(
                horizontal * self.yaw.sin(),
                self.distance * self.pitch.sin(),
                horizontal * self.yaw.cos(),
            )
    }

    pub fn ray_direction(&self, screen_x: f32, screen_y: f32, aspect: f32, fov: f32) -> Vec3 {
        let eye = self.eye();
        let forward = normalize(&(self.target - eye));
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let right = normalize(&cross(&forward, &world_up));
        let up = normalize(&cross(&right, &forward));
        let scale = (fov * 0.5).tan();

        normalize(&(forward + right * (screen_x * aspect * scale) + up * (screen_y * scale)))
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(MIN_PITCH, MAX_PITCH);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance + delta).clamp(MIN_DISTANCE, MAX_DISTANCE);
    }

    pub fn reset(&mut self) {
        self.yaw = 0.0;
        self.pitch = 0.35;
        self.distance = 8.5;
    }

    pub fn update_from_input(&mut self, window: &Window) -> bool {
        let mut changed = false;
        let orbit_speed = 0.035;
        let zoom_speed = 0.12;

        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            self.orbit(-orbit_speed, 0.0);
            changed = true;
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            self.orbit(orbit_speed, 0.0);
            changed = true;
        }
        if window.is_key_down(Key::Up) {
            self.orbit(0.0, orbit_speed);
            changed = true;
        }
        if window.is_key_down(Key::Down) {
            self.orbit(0.0, -orbit_speed);
            changed = true;
        }
        if window.is_key_down(Key::W) {
            self.zoom(-zoom_speed);
            changed = true;
        }
        if window.is_key_down(Key::S) {
            self.zoom(zoom_speed);
            changed = true;
        }
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            self.reset();
            changed = true;
        }

        changed
    }
}
