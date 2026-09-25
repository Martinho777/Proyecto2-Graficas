mod framebuffer;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};

const FRAMEBUFFER_WIDTH: usize = 320;
const FRAMEBUFFER_HEIGHT: usize = 180;
const WINDOW_SCALE: usize = 3;
const BACKGROUND: u32 = 0x07111F;

fn main() {
    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    let mut window = Window::new(
        "Proyecto 2 — Diorama con Raytracing",
        FRAMEBUFFER_WIDTH * WINDOW_SCALE,
        FRAMEBUFFER_HEIGHT * WINDOW_SCALE,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear(BACKGROUND);

        window
            .update_with_buffer(
                &framebuffer.buffer,
                framebuffer.width,
                framebuffer.height,
            )
            .expect("No se pudo actualizar el framebuffer");
    }
}
