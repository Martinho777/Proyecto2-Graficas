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
    question_mark: Material,
    plain_crystal: Material,
    stained_glass: Material,
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

        let mut stained_glass = Material::from_texture(Texture::load_ppm(
            "assets/textures/peach_stained_glass.ppm",
        )?);
        stained_glass.albedo = [0.75, 0.9, 1.0];
        stained_glass.specular = 128.0;
        stained_glass.transparency = 0.7;
        stained_glass.refractive_index = 1.5;

        let mut plain_crystal = Material::from_texture(Texture::procedural(TextureKind::Crystal));
        plain_crystal.albedo = [0.55, 0.8, 1.0];
        plain_crystal.specular = 128.0;
        plain_crystal.transparency = 0.7;
        plain_crystal.refractive_index = 1.5;

        let mut lava = Material::from_texture(Texture::procedural(TextureKind::Roof));
        lava.albedo = [1.0, 0.35, 0.22];
        lava.specular = 24.0;
        lava.emission = [0.0, 0.0, 0.0];

        let mut coin =
            Material::from_texture(Texture::load_ppm("assets/textures/coin_reference.ppm")?);
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

        let mut question_mark = Material::from_texture(Texture::solid([1.0, 1.0, 1.0]));
        question_mark.albedo = [1.0, 1.0, 1.0];
        question_mark.specular = 24.0;

        Ok(Self {
            materials: [stone, wood, leaves, plain_crystal.clone(), lava],
            coin,
            question,
            pipe,
            question_mark,
            plain_crystal,
            stained_glass,
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

    pub fn question_mark(&self) -> Material {
        self.question_mark.clone()
    }

    pub fn plain_crystal(&self) -> Material {
        self.plain_crystal.clone()
    }

    pub fn stained_glass(&self) -> Material {
        self.stained_glass.clone()
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
