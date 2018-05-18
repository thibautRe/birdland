extern crate grid_2d;
extern crate noise;

use self::grid_2d::{Coord, Grid, Size};
use self::noise::{NoiseFn, Perlin, Seedable};

/// Size of a chunk
const _CHUNK_SIZE: usize = 64;

/// A Chunk holds informations about a size and position,
/// as well as informations for randomization
pub struct Chunk {
    pub size: u32,
    pub position: Coord,
    noises: Vec<Noise>,
    seed: u32,
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
        }
    }

    /// Returns a Grid with noised values
    pub fn get_grid(&self) -> Grid<f64> {
        let mut grid = Grid::new_default(Size::new(_CHUNK_SIZE as u32, _CHUNK_SIZE as u32));
        let is_island_chunk = self.is_island_chunk();
        for coord in grid.coords() {
            let mut value = 0.0;
            // Calculate the resulting noise from the perlins
            // The real coord needs to be shifted by the origin
            // of the grid
            let moved_coord = coord + (self.position * _CHUNK_SIZE as i32);
            for noise in self.noises.iter() {
                value += noise.get(moved_coord) * noise.amplitude;
            }

            // If the chunk is an island then higher it!
            if is_island_chunk {
                value *= 1.0 + (self.get_island_factor(coord) * 10.0);
            }

            // Assign the noise value to the coord
            if let Some(x) = grid.get_mut(coord) {
                *x = value;
            }
        }

        grid
    }

    /// Returns true if the chunk contains an island
    fn is_island_chunk(&self) -> bool {
        let noise = Noise::new(self.seed, 30.0, 1.0);

        println!("Island : {}", noise.get(self.position).abs() < 0.5);
        noise.get(self.position).abs() < 0.5
    }

    /// Returns a value between 0 and 1 (0 at the edges of the chunk,
    /// in order to get a smooth transition between chunks)
    fn get_island_factor(&self, point: Coord) -> f64 {
        // lower it closer to 1 for more rough edges
        // increase it to make it narrower
        let factor: f64 = 100.0;

        // Formulae : e^(-1/x + 1/(x-CHUNK))
        // TODO: Probably replace that by something else, gives too squarish results
        let factor_x =
            factor.powf(-1.0 / point.x as f64 + 1.0 / (point.x as f64 - _CHUNK_SIZE as f64));
        let factor_y =
            factor.powf(-1.0 / point.y as f64 + 1.0 / (point.y as f64 - _CHUNK_SIZE as f64));

        // The edge factor is 0 at the edges and around 1 in the center
        let edge_factor = factor_x * factor_y * 1.4;

        edge_factor
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

#[test]
fn test_grid() {
    let chunk = Chunk::new(1, vec![10, 10]);
    let grid = chunk.get_grid();
    println!("{:?}", grid)
}
