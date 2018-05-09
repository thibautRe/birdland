extern crate grid_2d;
extern crate noise;

use self::grid_2d::{Coord, Grid, Size};
use self::noise::{NoiseFn, Perlin, Seedable};

/// Size of a chunk
const _CHUNK_SIZE: usize = 128;

/// A Chunk holds informations about a size and position,
/// as well as informations for randomization
pub struct Chunk {
    pub size: u32,
    pub position: Coord,
    noises: Vec<Noise>,
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
        }
    }

    /// Returns a Grid with noised values
    pub fn get_grid(&self) -> Grid<f64> {
        let mut grid = Grid::new_default(Size::new(_CHUNK_SIZE as u32, _CHUNK_SIZE as u32));
        for coord in grid.coords() {
            let mut value = 0.0;
            for noise in self.noises.iter() {
                value += noise.get(coord) * noise.amplitude;
            }

            if let Some(x) = grid.get_mut(coord) {
                *x = value;
            }
        }

        grid
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

        println!("scale {}, amplitude {}", scale, amplitude);

        Self {
            amplitude,
            scale,
            generator: Box::new(generator),
        }
    }

    fn get(&self, point: Coord) -> f64 {
        self.generator
            .get([point.x as f64 * self.scale, point.y as f64 * self.scale])
    }
}

#[test]
fn test_grid() {
    let chunk = Chunk::new(1, vec![10, 10]);
    let grid = chunk.get_grid();
    println!("{:?}", grid)
}
