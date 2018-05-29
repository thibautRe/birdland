extern crate grid_2d;
extern crate noise;

use self::grid_2d::{Coord, Grid, Size};
use self::noise::{NoiseFn, Perlin, Seedable};

use super::tile::{Tile, TileObjects};

/// Size of a chunk
const _CHUNK_SIZE: u32 = 64;

/// Provides a cubic easing between 0 and 1
///
/// Returns 0 if the input value is below 0, and 1 if
/// the value is above 1. It will return 0.5 if the value
/// equals 0.5, and a smooth interpolation if the value
/// is between 0 and 1.
///
/// Example: https://easings.net/#easeInOutCubic
fn easing_cub_in_out(t: f64) -> f64 {
    if t < 0.0 {
        return 0.0;
    } else if t < 0.5 {
        return 4.0 * t * t * t;
    } else if t < 1.0 {
        return (t - 1.0) * (2.0 * t - 2.0) * (2.0 * t - 2.0) + 1.0;
    } else {
        1.0
    }
}

/// A Chunk holds informations about a size and position,
/// as well as informations for randomization
pub struct Chunk {
    pub size: u32,
    pub position: Coord,
    noises: Vec<Noise>,
    seed: u32,
    precision: u32,
}

impl Chunk {
    /// Creates a new chunk using a seed and an offset position
    pub fn new(seed: u32, position: Coord) -> Self {
        let mut noises = Vec::new();

        noises.push(Noise::new(seed, 0.05, 1.0));
        noises.push(Noise::new(seed, 0.1, 0.6));
        noises.push(Noise::new(seed, 0.5, 0.2));

        Self {
            position,
            size: _CHUNK_SIZE as u32,
            noises,
            seed,
            precision: 2,
        }
    }

    /// Returns a Grid with noised values
    pub fn get_grid(&self) -> Grid<Tile> {
        let mut grid = Grid::new_default(Size::new(_CHUNK_SIZE, _CHUNK_SIZE));
        let is_island_chunk = self.is_island_chunk();
        // let is_island_chunk = true;

        // The factor of the island determines roughly how big the island will be.
        let factor = self.noises[0].get(self.position) * 1.5 + 0.5;
        for coord in grid.coords() {
            let mut altitude = 0.0;
            // Calculate the resulting noise from the perlins
            // The real coord needs to be shifted by the origin
            // of the grid
            let moved_coord = coord + (self.position * _CHUNK_SIZE as i32);
            for noise in self.noises.iter() {
                altitude += noise.get(moved_coord) * noise.amplitude;
            }

            // If the chunk is an island then higher it!
            if is_island_chunk {
                altitude *= 1.0 + (self.get_island_factor(coord, factor) * 10.0);
            }

            // Round the altitude to the appropriate precision
            altitude = (altitude * 10u32.pow(self.precision) as f64).round()
                / 10u32.pow(self.precision) as f64;

            let tile_object = self.get_tile_object(coord, altitude);

            // Assign the noise altitude to the coord
            if let Some(x) = grid.get_mut(coord) {
                *x = Tile::new(altitude, tile_object);
            }
        }

        grid
    }

    /// Returns true if the chunk contains an island
    fn is_island_chunk(&self) -> bool {
        let noise = Noise::new(self.seed, 30.0, 1.0);

        noise.get(self.position).abs() < 0.5
    }

    /// Returns a value between 0 and 1 (0 at the edges of the chunk,
    /// in order to get a smooth transition between chunks)
    fn get_island_factor(&self, point: Coord, factor: f64) -> f64 {
        let center = (_CHUNK_SIZE / 2) as f64;
        let distance_from_center = factor * (-(point.x as f64 / center - 1.0).abs() + 1.0)
            * (-(point.y as f64 / center - 1.0).abs() + 1.0);
        easing_cub_in_out(distance_from_center as f64)
    }

    /// Returns the content of the tile.
    fn get_tile_object(&self, point: Coord, altitude: f64) -> Option<TileObjects> {
        // Create vegetation
        if altitude > 3.0 && altitude < 8.0 {
            // Create them randomly
            if self.noises[2].get(point) > 0.6 {
                Some(TileObjects::Tree)
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct Noise {
    amplitude: f64,
    scale: f64,
    generator: Box<NoiseFn<[f64; 2]>>,
}

impl Noise {
    /// Returns a Noise structure based on a seed
    fn new(seed: u32, target_scale: f64, target_amplitude: f64) -> Self {
        let generator = Perlin::new().set_seed(seed);

        // Generate amplitude and scale from the generator itself
        let scale = target_scale * (1.0 + generator.get([0.9, 0.9]));
        let amplitude = target_amplitude * (1.0 + generator.get([0.1, 0.1]));

        Self {
            amplitude,
            scale,
            generator: Box::new(generator),
        }
    }

    fn get(&self, point: Coord) -> f64 {
        let noise = self.generator
            .get([point.x as f64 * self.scale, point.y as f64 * self.scale]);
        // Normalize the value between 0 and 1 (the noise generator yields -1 and 1)
        (noise + 1.0) / 2.0
    }
}
