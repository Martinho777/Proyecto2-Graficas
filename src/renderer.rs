use crate::camera::Camera;
use crate::framebuffer::Framebuffer;

pub fn render_preview(framebuffer: &mut Framebuffer, camera: &Camera) {
    framebuffer.clear(0);
    let horizon = framebuffer.height * 2 / 3;
    let eye = camera.eye();
    let horizon_shift = (eye.x * 5.0) as i32;
    let grid_shift_x = (eye.x * 4.0) as i32;
    let grid_shift_y = (eye.z * 3.0) as i32;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let color = if y < horizon {
                let t = y as f32 / horizon.max(1) as f32;
                let sky_wave = (((x as i32 + horizon_shift).rem_euclid(80)) as f32 / 80.0) * 3.0;
                let red = (7.0 + sky_wave + 12.0 * t) as u32;
                let green = (17.0 + sky_wave + 18.0 * t) as u32;
                let blue = (31.0 + sky_wave + 35.0 * t) as u32;
                (red << 16) | (green << 8) | blue
            } else {
                let shifted_x =
                    (x as i32 + grid_shift_x).rem_euclid(framebuffer.width as i32) as usize;
                let shifted_y = (y as i32 - horizon as i32 + grid_shift_y)
                    .rem_euclid((framebuffer.height - horizon) as i32)
                    as usize;
                let cell_x = shifted_x / 16;
                let cell_y = shifted_y / 8;
                if (cell_x + cell_y) % 2 == 0 {
                    0x3A3028
                } else {
                    0x4B4032
                }
            };

            framebuffer.set_pixel(x, y, color);
        }
    }
}
