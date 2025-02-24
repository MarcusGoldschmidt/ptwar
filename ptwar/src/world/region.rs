use crate::game::resource::ResourceStorage;
use crate::world::region_noise::make_noise_fun;
use crate::world::tile::Tile;
use hexx::{shapes, Hex, HexLayout, HexOrientation, Vec2};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin, Turbulence, Worley};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::collections::HashMap;

pub struct RegionNoise {
    pub(crate) seed: u32,
    pub(crate) region_noise: f64,
    pub(crate) hex: Hex,
}

pub struct Region {
    pub name: String,
    pub region_noise: RegionNoise,
    pub tiles: HashMap<Hex, Tile>,
    // TODO: high memory usage, consider using a sparse data structure.
    pub storage: HashMap<Hex, ResourceStorage>,
}

impl Region {
    // TODO: Move this to a utility module with proper tests and names.
    pub fn random_name() -> String {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        s
    }

    pub fn new_with_noise(radius: u32, region_noise: RegionNoise) -> Self {
        let layout = HexLayout {
            scale: Vec2::new(radius as f32, radius as f32),
            orientation: HexOrientation::Flat,
            ..Default::default()
        };

        let mut tiles = HashMap::new();
        let mut storage = HashMap::new();

        let noise_function = make_noise_fun(region_noise.seed);

        for hex in shapes::hexagon(Hex::from(layout.origin), radius) {
            let noise = noise_function.get([
                hex.x as f64,
                hex.y as f64,
                region_noise.hex.x as f64,
                region_noise.hex.y as f64,
            ]);

            tiles.insert(hex, Tile::from_noise(hex, noise));
            storage.insert(hex, ResourceStorage::default());
        }

        Self {
            name: Self::random_name(),
            tiles,
            region_noise,
            storage,
        }
    }
}
