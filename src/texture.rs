use std::fs;

#[derive(Clone, Debug)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[f32; 3]>,
}

impl Texture {
    pub fn solid(color: [f32; 3]) -> Self {
        Self {
            width: 1,
            height: 1,
            pixels: vec![color],
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
