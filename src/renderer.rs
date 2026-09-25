use crate::framebuffer::Framebuffer;

pub fn render_preview(framebuffer: &mut Framebuffer) {
    framebuffer.clear(0);
    let horizon = framebuffer.height * 2 / 3;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let color = if y < horizon {
                let t = y as f32 / horizon.max(1) as f32;
                let red = (7.0 + 12.0 * t) as u32;
                let green = (17.0 + 18.0 * t) as u32;
                let blue = (31.0 + 35.0 * t) as u32;
                (red << 16) | (green << 8) | blue
            } else {
                let cell_x = x / 16;
                let cell_y = (y - horizon) / 8;
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
