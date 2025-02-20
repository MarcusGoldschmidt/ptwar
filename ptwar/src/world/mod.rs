mod region;
mod tile;

use crate::system::Tick;
use crate::world::region::{Region, RegionNoise};
use hexx::{shapes, Hex, HexLayout, HexOrientation, Vec2};
use log::info;
use noise::{Fbm, NoiseFn, Perlin};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub struct World {
    pub last_save: Option<(Tick, Instant)>,

    hex_layout: HexLayout,
    regions: HashMap<Hex, Region>,
    seed: u32,
}

const DEFAULT_REGION_RADIUS: u32 = 30;

impl World {
    pub fn from_seed(seed: u32) -> Self {
        let layout = HexLayout {
            scale: Vec2::new(10.0, 5.0),
            orientation: HexOrientation::Flat,
            ..Default::default()
        };

        let noise_function = Fbm::<Perlin>::new(seed);

        let start = Instant::now();

        let regions: HashMap<Hex, Region> = shapes::rombus(Hex::from(layout.origin), 3, 6)
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
            "World generation took: {}ms generated {} tiles",
            start.elapsed().as_millis(),
            tiles_count
        );

        Self {
            last_save: None,
            hex_layout: layout,
            regions,
            seed,
        }
    }
}
