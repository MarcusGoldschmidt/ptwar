use crate::world::tile::Biome;
use noise::*;
use std::ops::{Div, Mul};
const CONTINENT_FREQUENCY: f64 = 1.0;

const CONTINENT_LACUNARITY: f64 = 2.108984375;

fn base_noise_fn(seed: u32) -> impl NoiseFn<f64, 2> {
    let base = Fbm::<Perlin>::new(seed);

    let base_continent_def_fb1 = Fbm::<Perlin>::new(seed + 1)
        .set_frequency(CONTINENT_FREQUENCY)
        .set_persistence(0.5)
        .set_lacunarity(CONTINENT_LACUNARITY)
        .set_octaves(9);

    Add::new(base, base_continent_def_fb1)
}

pub fn height_noise_fn(seed: u32) -> impl NoiseFn<f64, 2> {
    let base = Fbm::<Perlin>::new(seed)
        .set_frequency(0.34375)
        .set_persistence(0.5)
        .set_lacunarity(CONTINENT_LACUNARITY)
        .set_octaves(8);

    Exponent::new(Abs::new(base)).set_exponent(1.4)
}

#[derive(Clone, Debug)]
pub struct MultiLayerNoiseValue {
    pub height: f64,
    pub temperature: f64,
    pub humidity: f64,
    pub special: f64,
}

pub struct NoiseGenerator {
    seed: u32,
    height_noise: Box<dyn NoiseFn<f64, 2>>,
    temperature_noise: Box<dyn NoiseFn<f64, 2>>,
    humidity_noise: Box<dyn NoiseFn<f64, 2>>,
    special_noise: Box<dyn NoiseFn<f64, 2>>,
}

const WORLD_REGION_NOISE: f64 = 0.3124;
const INNER_REGION_NOISE: f64 = 30.345158;
const SPECIAL_CLUSTER_NOISE: f64 = 17.51231;
impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        let height_noise = Box::new(height_noise_fn(seed));
        let temperature_noise = Box::new(base_noise_fn(seed + 1));
        let humidity_noise = Box::new(base_noise_fn(seed + 2));
        let special_noise = Box::new(base_noise_fn(seed + 3));

        Self {
            seed,
            height_noise,
            temperature_noise,
            humidity_noise,
            special_noise,
        }
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }

    pub fn generate(&self, x: f64, y: f64, z: f64, w: f64) -> MultiLayerNoiseValue {
        let z = z.abs() + 1.0;
        let w = w.abs() + 1.0;

        let x_c = (x / INNER_REGION_NOISE) * z;
        let y_c = (y / INNER_REGION_NOISE) * w;

        MultiLayerNoiseValue {
            height: self.height_noise.get([x_c, y_c]),
            temperature: self.temperature_noise.get([x_c, y_c]),
            humidity: self.humidity_noise.get([x_c, y_c]),
            // No cluster for special
            special: self.special_noise.get([
                (x + z) / SPECIAL_CLUSTER_NOISE,
                (y + w) / SPECIAL_CLUSTER_NOISE,
            ]),
        }
    }
}

impl MultiLayerNoiseValue {
    pub fn gen_biome(&self) -> Biome {
        if self.height < -0.2 {
            return Biome::Water;
        }

        if self.special > 0.75 {
            return Biome::City;
        }

        if self.height > 0.6 {
            return Biome::Mountain;
        }

        if self.height > 0.4 {
            return Biome::Hill;
        }

        if self.temperature > 0.0 {
            if self.humidity > 0.6 {
                return Biome::DenseForest;
            }

            if self.humidity > 0.3 {
                return Biome::Forest;
            }

            return Biome::Desert;
        }

        Biome::Plains
    }
}
