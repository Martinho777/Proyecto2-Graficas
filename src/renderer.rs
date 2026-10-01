use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::ray::Ray;
use crate::scene::Scene;
use nalgebra_glm::{dot, normalize, Vec3};

pub fn render_raytraced_scene(framebuffer: &mut Framebuffer, camera: &Camera, scene: &Scene) {
    framebuffer.clear(0);
    let aspect = framebuffer.width as f32 / framebuffer.height as f32;
    let fov = std::f32::consts::FRAC_PI_3;
    let background = scene.background;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = 2.0 * (x as f32 + 0.5) / framebuffer.width as f32 - 1.0;
            let screen_y = 1.0 - 2.0 * (y as f32 + 0.5) / framebuffer.height as f32;
            let direction = camera.ray_direction(screen_x, screen_y, aspect, fov);
            let ray = Ray::new(camera.eye(), direction);

            let color = if let Some(hit) = scene.intersect(&ray) {
                let light_direction = normalize(&Vec3::new(-0.6, 1.0, 0.8));
                let diffuse = dot(&hit.normal, &light_direction).max(0.0);
                let half_vector = normalize(&(light_direction - ray.direction));
                let specular = dot(&hit.normal, &half_vector)
                    .max(0.0)
                    .powf(hit.material.specular)
                    * 0.35;
                let texture = hit.material.texture.sample(hit.uv[0], hit.uv[1]);
                let red = ((texture[0] * hit.material.albedo[0] * (0.18 + diffuse)
                    + specular
                    + hit.material.emission[0])
                    .clamp(0.0, 1.0)
                    * 255.0) as u32;
                let green = ((texture[1] * hit.material.albedo[1] * (0.18 + diffuse)
                    + specular
                    + hit.material.emission[1])
                    .clamp(0.0, 1.0)
                    * 255.0) as u32;
                let blue = ((texture[2] * hit.material.albedo[2] * (0.18 + diffuse)
                    + specular
                    + hit.material.emission[2])
                    .clamp(0.0, 1.0)
                    * 255.0) as u32;
                (red << 16) | (green << 8) | blue
            } else {
                let t = screen_y * 0.5 + 0.5;
                let base_red = ((background >> 16) & 0xFF) as f32;
                let base_green = ((background >> 8) & 0xFF) as f32;
                let base_blue = (background & 0xFF) as f32;
                let red = (base_red * (0.75 + 0.25 * t)).clamp(0.0, 255.0) as u32;
                let green = (base_green * (0.75 + 0.25 * t)).clamp(0.0, 255.0) as u32;
                let blue = (base_blue * (0.75 + 0.25 * t)).clamp(0.0, 255.0) as u32;
                (red << 16) | (green << 8) | blue
            };

            framebuffer.set_pixel(x, y, color);
        }
    }
}
