extern crate grid_2d;
extern crate noise;

pub mod chunk;

use grid_2d::Coord;
use chunk::Chunk;

fn main() {
    let chunk = Chunk::new(6, Coord::new(0, 0));
    let grid = chunk.get_grid();

    for coord in grid.coords() {
        let mut value = grid.get(coord).unwrap();
        let value = ((value + 1.0) * 5.0).round() as i32;
        if coord.x == 0 {
            print!("\n");
        }

        let character = match value {
            1 => ' ',
            2 => '.',
            3 => '-',
            4 => '=',
            5 => '+',
            6 => '*',
            7 => '#',
            8 => '%',
            9 => '@',
            10 => '▓',
            _ => ' ',
        };

        print!("{}", character);
    }
}
