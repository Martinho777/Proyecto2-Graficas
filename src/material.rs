use crate::texture::{Texture, TextureKind};

#[derive(Clone, Copy, Debug)]
pub enum MaterialId {
    Stone,
    Wood,
    Leaves,
    Crystal,
    Lava,
}

#[derive(Clone, Debug)]
pub struct Material {
    pub texture: Texture,
    pub albedo: [f32; 3],
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub refractive_index: f32,
    pub emission: [f32; 3],
}

impl Material {
    pub fn from_texture(texture: Texture) -> Self {
        Self {
            texture,
            albedo: [0.8, 0.8, 0.8],
            specular: 32.0,
            transparency: 0.0,
            reflectivity: 0.0,
            refractive_index: 1.0,
            emission: [0.0, 0.0, 0.0],
        }
    }
}

pub struct MaterialLibrary {
    materials: [Material; 5],
    coin: Material,
    question: Material,
    pipe: Material,
}

impl MaterialLibrary {
    pub fn load() -> Result<Self, String> {
        let mut stone = Material::from_texture(Texture::procedural(TextureKind::Stone));
        stone.albedo = [0.95, 0.9, 0.78];

        let mut wood = Material::from_texture(Texture::procedural(TextureKind::Dirt));
        wood.albedo = [0.78, 0.38, 0.16];
        wood.specular = 18.0;

        let mut leaves = Material::from_texture(Texture::procedural(TextureKind::Grass));
        leaves.albedo = [0.3, 0.86, 0.22];
        leaves.specular = 12.0;

        let mut crystal = Material::from_texture(Texture::procedural(TextureKind::Crystal));
        crystal.albedo = [0.55, 0.8, 1.0];
        crystal.specular = 128.0;
        crystal.transparency = 0.7;
        crystal.refractive_index = 1.5;

        let mut lava = Material::from_texture(Texture::procedural(TextureKind::Roof));
        lava.albedo = [1.0, 0.35, 0.22];
        lava.specular = 24.0;
        lava.emission = [0.0, 0.0, 0.0];

        let mut coin = Material::from_texture(Texture::procedural(TextureKind::Coin));
        coin.albedo = [1.0, 0.82, 0.12];
        coin.specular = 96.0;
        coin.reflectivity = 0.35;

        let mut question = Material::from_texture(Texture::procedural(TextureKind::Question));
        question.albedo = [1.0, 1.0, 1.0];
        question.specular = 48.0;

        let mut pipe = Material::from_texture(Texture::procedural(TextureKind::Pipe));
        pipe.albedo = [0.12, 0.5, 0.08];
        pipe.specular = 96.0;
        pipe.reflectivity = 0.4;

        Ok(Self {
            materials: [stone, wood, leaves, crystal, lava],
            coin,
            question,
            pipe,
        })
    }

    pub fn get(&self, id: MaterialId) -> Material {
        self.materials[id.index()].clone()
    }

    pub fn coin(&self) -> Material {
        self.coin.clone()
    }

    pub fn question(&self) -> Material {
        self.question.clone()
    }

    pub fn pipe(&self) -> Material {
        self.pipe.clone()
    }
}

impl MaterialId {
    fn index(self) -> usize {
        match self {
            Self::Stone => 0,
            Self::Wood => 1,
            Self::Leaves => 2,
            Self::Crystal => 3,
            Self::Lava => 4,
        }
    }
}
