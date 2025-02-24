use hexx::Hex;

pub enum Biome {
    Water,
    Desert,
    Plains,
    Forest,
    DenseForest,
    Hill,
    Mountain,
    City,
}

pub struct Tile {
    pub hex: Hex,
    pub biome: Biome,
    pub infrastructure_level: u8,
    pub height: f64,
    pub wight: u8,
    pub slots: u8,
    pub noise: f64,
}

impl Tile {
    pub fn from_noise(hex: Hex, noise: f64) -> Self {
        let biome = match noise {
            (-1.0..-0.2) => Biome::Water,
            (0.0..0.2) => Biome::Plains,
            (-0.2..0.0) => Biome::Desert,
            (0.2..0.5) => Biome::Forest,
            (0.5..0.6) => Biome::DenseForest,
            (0.6..0.7) => Biome::Hill,
            (0.7..0.8) => Biome::Mountain,
            (0.8..1.0) => Biome::City,
            _ => Biome::Water,
        };

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
