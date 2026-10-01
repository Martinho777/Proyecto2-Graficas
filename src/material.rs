use crate::texture::Texture;

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
}

impl MaterialLibrary {
    pub fn load() -> Result<Self, String> {
        let mut stone = Material::from_texture(Texture::solid([0.65, 0.65, 0.65]));
        stone.albedo = [0.95, 0.9, 0.78];

        let mut wood = Material::from_texture(Texture::solid([0.45, 0.25, 0.12]));
        wood.albedo = [0.78, 0.38, 0.16];
        wood.specular = 18.0;

        let mut leaves = Material::from_texture(Texture::solid([0.2, 0.65, 0.18]));
        leaves.albedo = [0.3, 0.86, 0.22];
        leaves.specular = 12.0;

        let mut crystal = Material::from_texture(Texture::solid([0.35, 0.7, 0.95]));
        crystal.albedo = [0.55, 0.8, 1.0];
        crystal.specular = 128.0;
        crystal.transparency = 0.7;
        crystal.refractive_index = 1.5;

        let mut lava = Material::from_texture(Texture::solid([0.9, 0.25, 0.05]));
        lava.albedo = [1.0, 0.72, 0.08];
        lava.specular = 24.0;
        lava.emission = [0.55, 0.12, 0.0];

        let mut coin = Material::from_texture(Texture::solid([1.0, 0.75, 0.05]));
        coin.albedo = [1.0, 0.82, 0.12];
        coin.specular = 96.0;
        coin.reflectivity = 0.35;

        let mut question = Material::from_texture(Texture::solid([1.0, 0.8, 0.08]));
        question.albedo = [1.0, 0.88, 0.18];
        question.specular = 48.0;

        Ok(Self {
            materials: [stone, wood, leaves, crystal, lava],
            coin,
            question,
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
