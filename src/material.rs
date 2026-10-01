use crate::texture::{Texture, TextureKind};
use std::sync::Arc;

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
    pub texture: Arc<Texture>,
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
            texture: Arc::new(texture),
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
    thatch: Material,
    banana: Material,
    bamboo: Material,
    metal: Material,
    navi: Material,
    energy_blue: Material,
    phosphor_green: Material,
    alien_egg: Material,
    ridley_skin: Material,
    ridley_eye: Material,
    rupee: Material,
    face_mark: Material,
    deku_bark: Material,
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
        stained_glass.transparency = 0.58;
        stained_glass.reflectivity = 0.16;
        stained_glass.refractive_index = 1.5;
        stained_glass.emission = [0.035, 0.025, 0.02];

        let mut plain_crystal = Material::from_texture(Texture::procedural(TextureKind::Crystal));
        plain_crystal.albedo = [0.55, 0.8, 1.0];
        plain_crystal.specular = 128.0;
        plain_crystal.transparency = 0.42;
        plain_crystal.reflectivity = 0.12;
        plain_crystal.refractive_index = 1.5;

        let mut lava = Material::from_texture(Texture::procedural(TextureKind::Roof));
        lava.albedo = [1.0, 0.35, 0.22];
        lava.specular = 24.0;
        lava.emission = [0.0, 0.0, 0.0];

        let mut coin =
            Material::from_texture(Texture::load_ppm("assets/textures/coin_reference.ppm")?);
        coin.albedo = [1.0, 0.82, 0.12];
        coin.specular = 64.0;
        coin.reflectivity = 0.12;
        coin.transparency = 0.0;

        let mut question = Material::from_texture(Texture::procedural(TextureKind::Question));
        question.albedo = [1.0, 1.0, 1.0];
        question.specular = 48.0;

        let mut pipe = Material::from_texture(Texture::procedural(TextureKind::Pipe));
        pipe.albedo = [0.12, 0.5, 0.08];
        pipe.specular = 52.0;
        pipe.reflectivity = 0.12;
        pipe.transparency = 0.0;

        let mut question_mark = Material::from_texture(Texture::solid([1.0, 1.0, 1.0]));
        question_mark.albedo = [1.0, 1.0, 1.0];
        question_mark.specular = 24.0;

        let mut thatch = Material::from_texture(Texture::procedural(TextureKind::Thatch));
        thatch.albedo = [0.95, 0.68, 0.25];
        thatch.specular = 8.0;

        let mut banana = Material::from_texture(Texture::solid([0.95, 0.82, 0.08]));
        banana.albedo = [1.0, 0.82, 0.06];
        banana.specular = 20.0;

        let mut bamboo = Material::from_texture(Texture::procedural(TextureKind::Bamboo));
        bamboo.albedo = [0.9, 0.62, 0.28];
        bamboo.specular = 10.0;

        let mut metal = Material::from_texture(Texture::procedural(TextureKind::Stone));
        metal.albedo = [0.72, 0.78, 0.86];
        metal.specular = 128.0;
        metal.reflectivity = 0.28;

        let mut deku_bark = Material::from_texture(Texture::procedural(TextureKind::Bark));
        deku_bark.albedo = [1.0, 0.72, 0.42];
        deku_bark.specular = 14.0;

        let mut navi = Material::from_texture(Texture::solid([0.2, 0.85, 1.0]));
        navi.albedo = [0.25, 0.9, 1.0];
        navi.specular = 96.0;
        navi.reflectivity = 0.12;
        navi.emission = [1.8, 2.4, 3.2];

        let mut energy_blue = Material::from_texture(Texture::solid([0.04, 0.28, 0.95]));
        energy_blue.albedo = [0.04, 0.3, 1.0];
        energy_blue.specular = 128.0;
        energy_blue.reflectivity = 0.08;
        energy_blue.emission = [0.08, 0.45, 1.4];

        let mut phosphor_green = Material::from_texture(Texture::solid([0.08, 1.0, 0.12]));
        phosphor_green.albedo = [0.08, 1.0, 0.12];
        phosphor_green.specular = 96.0;
        phosphor_green.emission = [0.35, 2.2, 0.18];

        let mut alien_egg = Material::from_texture(Texture::solid([0.62, 0.08, 0.92]));
        alien_egg.albedo = [0.62, 0.08, 0.92];
        alien_egg.specular = 112.0;
        alien_egg.reflectivity = 0.16;
        alien_egg.emission = [0.18, 0.02, 0.28];

        let mut ridley_skin = Material::from_texture(Texture::solid([0.22, 0.07, 0.18]));
        ridley_skin.albedo = [0.3, 0.08, 0.18];
        ridley_skin.specular = 32.0;
        ridley_skin.reflectivity = 0.08;

        let mut ridley_eye = Material::from_texture(Texture::solid([1.0, 0.02, 0.01]));
        ridley_eye.albedo = [1.0, 0.02, 0.01];
        ridley_eye.specular = 96.0;
        ridley_eye.emission = [2.4, 0.02, 0.01];

        let mut rupee = Material::from_texture(Texture::procedural(TextureKind::Crystal));
        rupee.albedo = [0.08, 0.45, 1.0];
        rupee.specular = 128.0;
        rupee.reflectivity = 0.28;
        rupee.transparency = 0.12;
        rupee.refractive_index = 1.45;
        rupee.emission = [0.02, 0.12, 0.35];

        let mut face_mark = Material::from_texture(Texture::solid([0.035, 0.025, 0.02]));
        face_mark.albedo = [0.18, 0.12, 0.08];
        face_mark.specular = 8.0;

        Ok(Self {
            materials: [stone, wood, leaves, plain_crystal.clone(), lava],
            coin,
            question,
            pipe,
            question_mark,
            plain_crystal,
            stained_glass,
            thatch,
            banana,
            bamboo,
            metal,
            navi,
            energy_blue,
            phosphor_green,
            alien_egg,
            ridley_skin,
            ridley_eye,
            rupee,
            face_mark,
            deku_bark,
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

    pub fn thatch(&self) -> Material {
        self.thatch.clone()
    }

    pub fn banana(&self) -> Material {
        self.banana.clone()
    }

    pub fn bamboo(&self) -> Material {
        self.bamboo.clone()
    }

    pub fn metal(&self) -> Material {
        self.metal.clone()
    }

    pub fn navi(&self) -> Material {
        self.navi.clone()
    }

    pub fn energy_blue(&self) -> Material {
        self.energy_blue.clone()
    }

    pub fn phosphor_green(&self) -> Material {
        self.phosphor_green.clone()
    }

    pub fn alien_egg(&self) -> Material {
        self.alien_egg.clone()
    }

    pub fn ridley_skin(&self) -> Material {
        self.ridley_skin.clone()
    }

    pub fn ridley_eye(&self) -> Material {
        self.ridley_eye.clone()
    }

    pub fn deku_bark(&self) -> Material {
        self.deku_bark.clone()
    }

    pub fn rupee(&self) -> Material {
        self.rupee.clone()
    }

    pub fn face_mark(&self) -> Material {
        self.face_mark.clone()
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
