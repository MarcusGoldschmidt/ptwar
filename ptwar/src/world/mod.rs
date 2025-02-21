pub mod region;
pub mod tile;

use crate::system::Tick;
use crate::world::region::{Region, RegionNoise};
use hexx::{shapes, Hex, HexLayout, HexOrientation, Vec2};
use log::info;
use noise::{Fbm, NoiseFn, Perlin};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub struct PtWorld {
    pub last_save: Option<(Tick, Instant)>,
    pub regions: HashMap<Hex, Region>,
    pub seed: u32,
}

const DEFAULT_REGION_RADIUS: u32 = 30;

impl PtWorld {
    pub fn from_seed(seed: u32) -> Self {
        let noise_function = Fbm::<Perlin>::new(seed);

        let start = Instant::now();

        let regions: HashMap<Hex, Region> = shapes::hexagon(Hex::ZERO, 2)
            .collect::<Vec<Hex>>()
            .par_iter()
            .map(|hex| {
                let noise = noise_function.get([hex.x as f64, hex.y as f64]);

                let region = Region::new_with_noise(
                    DEFAULT_REGION_RADIUS,
                    RegionNoise {
                        seed,
                        region_noise: noise,
                    },
                );

                (*hex, region)
            })
            .collect();

        let tiles_count = regions
            .par_iter()
            .map(|region| region.1.tiles.len())
            .sum::<usize>();

        info!(
            "World generation took: {}ms generated {} total tiles for {} regions with {} tiles each",
            start.elapsed().as_millis(),
            tiles_count,
            regions.len(),
            tiles_count / regions.len()
        );

        Self {
            last_save: None,
            regions,
            seed,
        }
    }
}
