use crate::game::resource::ResourceStorage;
use crate::world::region_noise::NoiseGenerator;
use crate::world::tile::{Biome, Tile};
use hexx::algorithms::a_star;
use hexx::storage::{HexStore, HexagonalMap};
use hexx::{Hex, HexBounds};
use log::info;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::collections::HashSet;

pub struct RegionNoise {
    pub(crate) seed: u32,
    pub(crate) hex: Hex,
}

pub struct Region {
    pub name: String,
    pub region_noise: RegionNoise,
    pub tiles: HexagonalMap<(Tile, ResourceStorage)>,
    pub cities: Vec<HexBounds>,
}

#[derive(Debug)]
pub struct RenderChunk {
    pub biome: Biome,
    pub tiles: Vec<Hex>,
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
        let noise_function = NoiseGenerator::new(region_noise.seed);

        let mut hex_map = HexagonalMap::new(Hex::ZERO, radius, |hex| {
            let noise = noise_function.generate(
                hex.x as f64,
                hex.y as f64,
                region_noise.hex.x as f64,
                region_noise.hex.y as f64,
            );

            (Tile::from_noise(hex, noise), ResourceStorage::default())
        });

        let mut cities = Self::find_city_clusters(&hex_map);

        cities.sort_by(|a, b| a.radius.cmp(&b.radius));

        let mut have_road = HashSet::new();

        // Create roads between cities.
        for current_city in cities.iter() {
            // Mark as center
            // Debugging
            for x in current_city.all_coords() {
                if let Some((tile, _)) = hex_map.get_mut(x) {
                    tile.biome = Biome::CityCenter;
                }
            }

            let mut near_cities = cities
                .iter()
                .map(|c| (c, c.center.distance_to(current_city.center)))
                .collect::<Vec<_>>();

            near_cities.sort_by(|a, b| a.1.cmp(&b.1));
            near_cities.reverse();

            for target_city in near_cities.iter() {
                if current_city == target_city.0 {
                    continue;
                }

                if have_road.contains(&(current_city.center, target_city.0.center))
                    || have_road.contains(&(target_city.0.center, current_city.center))
                {
                    continue;
                }

                let path = a_star(current_city.center, target_city.0.center, |a, b| {
                    let tile_a = hex_map.get(a).map(|(t, _)| t)?;
                    let tile_b = hex_map.get(b).map(|(t, _)| t)?;

                    let height_diff = match tile_b.biome {
                        Biome::Water => 0.0,
                        _ => (tile_a.noise.height - tile_b.noise.height).abs() * 5.0,
                    };

                    let biome_b = hex_map.get(b).map(|(t, _)| t.biome).unwrap_or(Biome::City);

                    biome_b.move_cost().map(|cost| cost + height_diff as u32)
                });

                have_road.insert((current_city.center, target_city.0.center));

                if let Some(road) = path {
                    for hex in road {
                        if let Some((tile, _)) = hex_map.get_mut(hex) {
                            if tile.biome != Biome::City && tile.biome != Biome::CityCenter {
                                tile.biome = Biome::Road;
                            }
                        }
                    }
                }
            }
        }

        Self {
            name: Self::random_name(),
            tiles: hex_map,
            region_noise,
            cities,
        }
    }

    fn find_cluster_rec(
        hex: Hex,
        biome: Biome,
        deep: usize,
        visited: &mut HashSet<Hex>,
        map: &HexagonalMap<(Tile, ResourceStorage)>,
    ) -> Vec<Hex> {
        let mut cluster = Vec::new();

        if deep == 0 {
            return cluster;
        }

        for h in hex.all_diagonals() {
            if visited.contains(&h) {
                continue;
            }

            visited.insert(h);

            if let Some((t, _)) = map.get(h) {
                if t.biome == biome {
                    cluster.push(h);
                    cluster.extend(Self::find_cluster_rec(h, biome, deep, visited, map));
                } else {
                    // Find if the cluster is connected to a tine piece.
                    let response = Self::find_cluster_rec(h, biome, deep - 1, visited, map);
                    if response.is_empty() == false {
                        cluster.push(h);
                    }
                }
            }
        }

        cluster
    }

    fn find_city_clusters(map: &HexagonalMap<(Tile, ResourceStorage)>) -> Vec<HexBounds> {
        let mut city_clusters = Vec::new();

        let mut visited = HashSet::new();

        for (hex, (tile, _)) in map.iter() {
            if visited.contains(&hex) {
                continue;
            }

            if tile.biome == Biome::City {
                visited.insert(hex);

                let clusters = Self::find_cluster_rec(hex, Biome::City, 1, &mut visited, map);

                if clusters.len() > 1 {
                    city_clusters.push(HexBounds::from_iter(clusters));
                }
            }
        }

        city_clusters
    }

    fn get_same_biome_tiles_recur(
        &self,
        origin_hex: Hex,
        biome: Biome,
        hex_bounds: &HexBounds,
        visited: &mut HashSet<Hex>,
    ) -> Vec<Hex> {
        let mut cluster = Vec::new();

        for neighbor in origin_hex.all_neighbors() {
            if visited.contains(&neighbor) {
                continue;
            }

            if hex_bounds.is_in_bounds(neighbor) {
                self.tiles.get(neighbor).map(|(tile, _)| {
                    if tile.biome == biome {
                        visited.insert(neighbor);
                        // Recur call
                        cluster.push(neighbor);

                        cluster.extend(
                            self.get_same_biome_tiles_recur(neighbor, biome, hex_bounds, visited),
                        );
                    }
                });
            }
        }

        cluster
    }

    pub fn render_chunks(&self, chunk_center: &HexBounds) -> Vec<RenderChunk> {
        let mut chunks = Vec::new();
        let mut visited = HashSet::new();

        for x in chunk_center.all_coords() {
            if visited.contains(&x) {
                continue;
            }

            let origin = self.tiles.get(x).map(|(tile, _)| {
                let mut tiles =
                    self.get_same_biome_tiles_recur(x, tile.biome, chunk_center, &mut visited);

                tiles.push(x);

                (tile.biome, tiles)
            });

            if let Some((biome, tiles)) = origin {
                chunks.push(RenderChunk { biome, tiles });
            }
        }
        chunks
    }
}
