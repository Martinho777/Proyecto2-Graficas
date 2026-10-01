use std::fs;

#[derive(Clone, Debug)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[f32; 3]>,
}

#[derive(Clone, Copy, Debug)]
pub enum TextureKind {
    Stone,
    Roof,
    Grass,
    Dirt,
    Pipe,
    Crystal,
    Coin,
    Question,
    Lava,
    Bamboo,
    Thatch,
    Bark,
    YoshiEgg,
}

impl Texture {
    pub fn solid(color: [f32; 3]) -> Self {
        Self {
            width: 1,
            height: 1,
            pixels: vec![color],
        }
    }

    pub fn procedural(kind: TextureKind) -> Self {
        let texture_size = if matches!(kind, TextureKind::YoshiEgg) {
            32
        } else {
            16
        };
        let width = texture_size;
        let height = texture_size;
        let mut pixels = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let noise = ((x * 13 + y * 7 + x * y * 3) % 17) as f32 / 16.0;
                let color = match kind {
                    TextureKind::Stone => {
                        let base = if (x / 4 + y / 4) % 2 == 0 { 0.82 } else { 0.9 };
                        [
                            base * (0.86 + noise * 0.14),
                            base * (0.84 + noise * 0.12),
                            base * 0.72,
                        ]
                    }
                    TextureKind::Roof => {
                        let base = if y % 4 == 0 { 0.5 } else { 0.82 };
                        [base, base * 0.08, base * 0.05]
                    }
                    TextureKind::Grass => [0.12 + noise * 0.08, 0.68 + noise * 0.2, 0.08],
                    TextureKind::Dirt => [0.48 + noise * 0.16, 0.27 + noise * 0.1, 0.1],
                    TextureKind::Pipe => [0.03, 0.25 + noise * 0.14, 0.04],
                    TextureKind::Crystal => [0.12 + noise * 0.1, 0.55 + noise * 0.2, 0.95],
                    TextureKind::Coin => {
                        let dx = x as f32 - 7.5;
                        let dy = y as f32 - 7.5;
                        if dx * dx + dy * dy < 42.0 {
                            [1.0, 0.65 + noise * 0.25, 0.03]
                        } else {
                            [0.45, 0.2, 0.01]
                        }
                    }
                    TextureKind::Question => {
                        let question = (y >= 2 && y <= 4 && x >= 5 && x <= 10)
                            || (y >= 4 && y <= 7 && x >= 9 && x <= 11)
                            || (y >= 7 && y <= 9 && x >= 6 && x <= 10)
                            || (y >= 9 && y <= 11 && x >= 7 && x <= 8)
                            || (y >= 13 && y <= 14 && x >= 7 && x <= 8);
                        if question {
                            [0.08, 0.02, 0.005]
                        } else {
                            [1.0, 0.68 + noise * 0.2, 0.03]
                        }
                    }
                    TextureKind::Lava => [0.8 + noise * 0.2, 0.08 + noise * 0.16, 0.01],
                    TextureKind::Bamboo => {
                        let band = if x % 5 == 0 {
                            0.28
                        } else {
                            0.52 + noise * 0.12
                        };
                        let joint = if y % 4 == 0 { 0.52 } else { 1.0 };
                        [
                            (0.3 + band * 0.42) * joint,
                            (0.14 + band * 0.28) * joint,
                            (0.035 + band * 0.08) * joint,
                        ]
                    }
                    TextureKind::Thatch => {
                        let strand = if (x + y * 2) % 5 == 0 {
                            0.42
                        } else {
                            0.72 + noise * 0.18
                        };
                        [strand, strand * 0.62, strand * 0.2]
                    }
                    TextureKind::Bark => {
                        let groove = if x % 4 == 0 { 0.46 } else { 1.0 };
                        let grain = 0.7 + noise * 0.25;
                        [0.24 * groove * grain, 0.1 * groove * grain, 0.035 * groove]
                    }
                    TextureKind::YoshiEgg => {
                        let spots = [(7.0, 8.0), (23.0, 10.0), (12.0, 23.0), (27.0, 25.0)];
                        let spotted = spots.iter().any(|(spot_x, spot_y)| {
                            let dx = x as f32 - spot_x;
                            let dy = y as f32 - spot_y;
                            dx * dx + dy * dy < 22.0
                        });
                        if spotted {
                            [0.10, 0.66 + noise * 0.12, 0.08]
                        } else {
                            [0.98, 0.98, 0.98]
                        }
                    }
                };
                pixels.push(color);
            }
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn load_ppm(path: &str) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|error| format!("{path}: {error}"))?;
        let text = String::from_utf8(bytes).map_err(|error| format!("{path}: {error}"))?;
        let mut tokens = text.split_whitespace();

        if tokens.next() != Some("P3") {
            return Err(format!("{path}: se esperaba un archivo P3"));
        }

        let width = tokens
            .next()
            .ok_or_else(|| format!("{path}: falta el ancho"))?
            .parse::<usize>()
            .map_err(|_| format!("{path}: ancho invalido"))?;
        let height = tokens
            .next()
            .ok_or_else(|| format!("{path}: falta el alto"))?
            .parse::<usize>()
            .map_err(|_| format!("{path}: alto invalido"))?;
        let max_value = tokens
            .next()
            .ok_or_else(|| format!("{path}: falta el valor maximo"))?
            .parse::<f32>()
            .map_err(|_| format!("{path}: valor maximo invalido"))?;

        let mut pixels = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            let red = tokens
                .next()
                .ok_or_else(|| format!("{path}: textura incompleta"))?;
            let green = tokens
                .next()
                .ok_or_else(|| format!("{path}: textura incompleta"))?;
            let blue = tokens
                .next()
                .ok_or_else(|| format!("{path}: textura incompleta"))?;
            pixels.push([
                red.parse::<f32>()
                    .map_err(|_| format!("{path}: rojo invalido"))?
                    / max_value,
                green
                    .parse::<f32>()
                    .map_err(|_| format!("{path}: verde invalido"))?
                    / max_value,
                blue.parse::<f32>()
                    .map_err(|_| format!("{path}: azul invalido"))?
                    / max_value,
            ]);
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn sample(&self, u: f32, v: f32) -> [f32; 3] {
        let u = u.rem_euclid(1.0);
        let v = v.rem_euclid(1.0);
        let x = (u * self.width as f32) as usize % self.width;
        let y = ((1.0 - v) * self.height as f32) as usize % self.height;
        self.pixels[y * self.width + x]
    }
}
