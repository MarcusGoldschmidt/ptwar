use crate::world::region_noise::MultiLayerNoiseValue;
use hexx::Hex;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Biome {
    Water,
    Desert,
    Plains,
    Forest,
    DenseForest,
    Hill,
    Mountain,
    City,
    CityCenter,
    Road,
}

impl Biome {
    pub fn move_cost(&self) -> Option<u32> {
        match self {
            Biome::Water => Some(20),
            Biome::Desert => Some(7),
            Biome::Plains => Some(5),
            Biome::Forest => Some(10),
            Biome::DenseForest => Some(14),
            Biome::Hill => Some(10),
            Biome::Mountain => Some(17),
            Biome::City => Some(10),
            Biome::CityCenter => Some(10),
            Biome::Road => Some(2),
        }
    }
    pub fn all() -> Vec<Biome> {
        vec![
            Biome::Water,
            Biome::Desert,
            Biome::Plains,
            Biome::Forest,
            Biome::DenseForest,
            Biome::Hill,
            Biome::Mountain,
            Biome::City,
            Biome::Road,
            Biome::CityCenter,
        ]
    }
}

pub struct Tile {
    pub hex: Hex,
    pub biome: Biome,
    pub infrastructure_level: u8,
    pub height: f64,
    pub wight: u8,
    pub slots: u8,
    pub noise: MultiLayerNoiseValue,
}

impl Tile {
    pub fn from_noise(hex: Hex, noise: MultiLayerNoiseValue) -> Self {
        let biome = noise.gen_biome();

        Self {
            hex,
            biome,
            infrastructure_level: 0,
            height: 0.0,
            wight: 100,
            slots: 3,
            noise,
        }
    }
}
