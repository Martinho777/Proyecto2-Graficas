use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::scene::Scene;

pub fn render_raytraced_scene(framebuffer: &mut Framebuffer, camera: &Camera, scene: &Scene) {
    framebuffer.clear(0);
    let aspect = framebuffer.width as f32 / framebuffer.height as f32;
    let fov = std::f32::consts::FRAC_PI_3;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = 2.0 * (x as f32 + 0.5) / framebuffer.width as f32 - 1.0;
            let screen_y = 1.0 - 2.0 * (y as f32 + 0.5) / framebuffer.height as f32;
            let direction = camera.ray_direction(screen_x, screen_y, aspect, fov);
            let ray = Ray::new(camera.eye(), direction);

            let color = if let Some(hit) = scene.intersect(&ray) {
                let red = ((hit.normal.x * 0.5 + 0.5) * 255.0) as u32;
                let green = ((hit.normal.y * 0.5 + 0.5) * 255.0) as u32;
                let blue = ((hit.normal.z * 0.5 + 0.5) * 255.0) as u32;
                (red << 16) | (green << 8) | blue
            } else {
                let t = screen_y * 0.5 + 0.5;
                let red = (8.0 + 14.0 * t) as u32;
                let green = (18.0 + 22.0 * t) as u32;
                let blue = (38.0 + 42.0 * t) as u32;
                (red << 16) | (green << 8) | blue
            };

            framebuffer.set_pixel(x, y, color);
        }
    }
}
