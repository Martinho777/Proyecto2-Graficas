use crate::app::{AppState, Character};
use crate::framebuffer::Framebuffer;
use crate::texture::Texture;

fn glyph(character: char) -> [u8; 5] {
    match character {
        'A' => [0b010, 0b101, 0b111, 0b101, 0b101],
        'B' => [0b110, 0b101, 0b110, 0b101, 0b110],
        'C' => [0b011, 0b100, 0b100, 0b100, 0b011],
        'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
        'E' => [0b111, 0b100, 0b110, 0b100, 0b111],
        'F' => [0b111, 0b100, 0b110, 0b100, 0b100],
        'G' => [0b011, 0b100, 0b101, 0b101, 0b011],
        'H' => [0b101, 0b101, 0b111, 0b101, 0b101],
        'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
        'J' => [0b001, 0b001, 0b001, 0b101, 0b010],
        'K' => [0b101, 0b110, 0b100, 0b110, 0b101],
        'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
        'M' => [0b101, 0b111, 0b111, 0b101, 0b101],
        'N' => [0b101, 0b111, 0b111, 0b111, 0b101],
        'O' => [0b010, 0b101, 0b101, 0b101, 0b010],
        'P' => [0b110, 0b101, 0b110, 0b100, 0b100],
        'Q' => [0b010, 0b101, 0b101, 0b111, 0b011],
        'R' => [0b110, 0b101, 0b110, 0b101, 0b101],
        'S' => [0b011, 0b100, 0b010, 0b001, 0b110],
        'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
        'U' => [0b101, 0b101, 0b101, 0b101, 0b111],
        'V' => [0b101, 0b101, 0b101, 0b101, 0b010],
        'W' => [0b101, 0b101, 0b111, 0b111, 0b101],
        'X' => [0b101, 0b101, 0b010, 0b101, 0b101],
        'Y' => [0b101, 0b101, 0b010, 0b010, 0b010],
        'Z' => [0b111, 0b001, 0b010, 0b100, 0b111],
        '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
        '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
        '2' => [0b110, 0b001, 0b010, 0b100, 0b111],
        '3' => [0b110, 0b001, 0b010, 0b001, 0b110],
        '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
        '5' => [0b111, 0b100, 0b110, 0b001, 0b110],
        '6' => [0b011, 0b100, 0b111, 0b101, 0b111],
        '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
        '8' => [0b111, 0b101, 0b111, 0b101, 0b111],
        '9' => [0b111, 0b101, 0b111, 0b001, 0b110],
        _ => [0, 0, 0, 0, 0],
    }
}

fn fill_rect(
    framebuffer: &mut Framebuffer,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    for py in y..y.saturating_add(height).min(framebuffer.height) {
        for px in x..x.saturating_add(width).min(framebuffer.width) {
            framebuffer.set_pixel(px, py, color);
        }
    }
}

fn outline_rect(
    framebuffer: &mut Framebuffer,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    for px in x..x.saturating_add(width).min(framebuffer.width) {
        framebuffer.set_pixel(px, y, color);
        framebuffer.set_pixel(px, y.saturating_add(height.saturating_sub(1)), color);
    }
    for py in y..y.saturating_add(height).min(framebuffer.height) {
        framebuffer.set_pixel(x, py, color);
        framebuffer.set_pixel(x.saturating_add(width.saturating_sub(1)), py, color);
    }
}

fn draw_text(
    framebuffer: &mut Framebuffer,
    text: &str,
    x: usize,
    y: usize,
    scale: usize,
    color: u32,
) {
    let mut cursor = x;
    for character in text.chars() {
        if character == ' ' {
            cursor += 4 * scale;
            continue;
        }
        for (row, bits) in glyph(character).into_iter().enumerate() {
            for column in 0..3 {
                if bits & (1 << (2 - column)) != 0 {
                    fill_rect(
                        framebuffer,
                        cursor + column * scale,
                        y + row * scale,
                        scale,
                        scale,
                        color,
                    );
                }
            }
        }
        cursor += 4 * scale;
    }
}

fn draw_centered(framebuffer: &mut Framebuffer, text: &str, y: usize, scale: usize, color: u32) {
    let width = text
        .chars()
        .map(|character| if character == ' ' { 4 } else { 4 })
        .sum::<usize>()
        * scale;
    draw_text(
        framebuffer,
        text,
        framebuffer.width.saturating_sub(width) / 2,
        y,
        scale,
        color,
    );
}

pub fn render_title(framebuffer: &mut Framebuffer) {
    fill_rect(
        framebuffer,
        0,
        0,
        framebuffer.width,
        framebuffer.height,
        0x10152C,
    );
    fill_rect(framebuffer, 0, 0, framebuffer.width, 28, 0x20294B);
    fill_rect(
        framebuffer,
        0,
        framebuffer.height.saturating_sub(10),
        framebuffer.width,
        10,
        0xD52C35,
    );

    draw_centered(framebuffer, "SUPER", 38, 4, 0xF2F2F2);
    draw_centered(framebuffer, "SMASH BROS", 62, 3, 0xF2F2F2);
    draw_centered(framebuffer, "RUST", 88, 4, 0xE4454C);
    draw_centered(framebuffer, "PRESS SPACE", 145, 2, 0xFFFFFF);
}

pub fn render_character_select(framebuffer: &mut Framebuffer, selected: usize) {
    fill_rect(
        framebuffer,
        0,
        0,
        framebuffer.width,
        framebuffer.height,
        0x171717,
    );
    draw_centered(framebuffer, "SELECT CHARACTER", 7, 2, 0xF1D36A);

    let card_width = 74;
    let card_height = 62;
    let gap = 4;
    let start_x = 5;
    let start_y = 28;

    for (index, character) in Character::ALL.into_iter().enumerate() {
        let column = index % 4;
        let row = index / 4;
        let x = start_x + column * (card_width + gap);
        let y = start_y + row * (card_height + gap);
        fill_rect(
            framebuffer,
            x,
            y,
            card_width,
            card_height,
            character.accent(),
        );
        fill_rect(framebuffer, x + 3, y + 3, card_width - 6, 42, 0x282828);
        fill_rect(framebuffer, x + 25, y + 10, 24, 24, character.accent());
        fill_rect(framebuffer, x + 30, y + 15, 4, 4, 0xFFFFFF);
        fill_rect(framebuffer, x + 40, y + 15, 4, 4, 0xFFFFFF);
        let label = character.name();
        let text_width = label.chars().count() * 4;
        draw_text(
            framebuffer,
            label,
            x + card_width.saturating_sub(text_width) / 2,
            y + 50,
            1,
            0xFFFFFF,
        );
        if index == selected {
            outline_rect(
                framebuffer,
                x.saturating_sub(2),
                y.saturating_sub(2),
                card_width + 4,
                card_height + 4,
                0xFFFFFF,
            );
            outline_rect(
                framebuffer,
                x.saturating_sub(4),
                y.saturating_sub(4),
                card_width + 8,
                card_height + 8,
                0xF1D36A,
            );
        }
    }

    draw_text(framebuffer, "ARROWS MOVE", 8, 166, 1, 0xA0A0A0);
    draw_text(framebuffer, "ENTER SELECT", 218, 166, 1, 0xA0A0A0);
}

fn draw_texture_nearest(
    framebuffer: &mut Framebuffer,
    texture: &Texture,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) {
    for destination_y in 0..height {
        let source_y = destination_y * texture.height / height;
        for destination_x in 0..width {
            let source_x = destination_x * texture.width / width;
            let pixel = texture.pixels[source_y * texture.width + source_x];
            let red = (pixel[0].clamp(0.0, 1.0) * 255.0) as u32;
            let green = (pixel[1].clamp(0.0, 1.0) * 255.0) as u32;
            let blue = (pixel[2].clamp(0.0, 1.0) * 255.0) as u32;
            framebuffer.set_pixel(
                destination_x + x,
                destination_y + y,
                (red << 16) | (green << 8) | blue,
            );
        }
    }
}

pub fn render_character_select_with_sheet(
    framebuffer: &mut Framebuffer,
    selected: usize,
    character_sheet: &Texture,
) {
    fill_rect(
        framebuffer,
        0,
        0,
        framebuffer.width,
        framebuffer.height,
        0x12121C,
    );
    draw_centered(framebuffer, "1 PLAYER GAME", 5, 2, 0xF1D36A);

    let sheet_x = 6;
    let sheet_y = 22;
    let sheet_width = 308;
    let sheet_height = 146;
    draw_texture_nearest(
        framebuffer,
        character_sheet,
        sheet_x,
        sheet_y,
        sheet_width,
        sheet_height,
    );

    let card_width = sheet_width / 4;
    let card_height = sheet_height / 2;
    let column = selected % 4;
    let row = selected / 4;
    let card_x = sheet_x + column * card_width;
    let card_y = sheet_y + row * card_height;
    outline_rect(
        framebuffer,
        card_x.saturating_sub(2),
        card_y.saturating_sub(2),
        card_width + 4,
        card_height + 4,
        0xFFFFFF,
    );
    outline_rect(
        framebuffer,
        card_x.saturating_sub(4),
        card_y.saturating_sub(4),
        card_width + 8,
        card_height + 8,
        0xF1D36A,
    );

    draw_text(framebuffer, "ARROWS MOVE", 8, 171, 1, 0xA0A0A0);
    draw_text(framebuffer, "ENTER SELECT", 218, 171, 1, 0xA0A0A0);
}

pub fn render_state(
    framebuffer: &mut Framebuffer,
    state: AppState,
    selected: usize,
    character_sheet: &Texture,
) {
    match state {
        AppState::Title => render_title(framebuffer),
        AppState::CharacterSelect => {
            render_character_select_with_sheet(framebuffer, selected, character_sheet)
        }
        AppState::Diorama(_) => {}
    }
}
