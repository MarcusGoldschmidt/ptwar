use crate::game::resource::ResourceStorage;
use hexx::Hex;

pub enum Biome {
    Desert,
    Forest,
    Jungle,
    Mountain,
    Ocean,
    Plains,
    Swamp,
    BigCity,
    SmallCity,
    Village,
}

pub struct Tile {
    pub hex: Hex,
    pub biome: Biome,
    pub infrastructure_level: u32,
    pub resource_storage: ResourceStorage,
}

impl Tile {
    pub fn from_noise(hex: Hex, noise: f64) -> Self {
        let biome = match noise {
            _ => Biome::Plains,
        };

        Self {
            hex,
            biome,
            infrastructure_level: 0,
            resource_storage: Default::default(),
        }
    }
}
