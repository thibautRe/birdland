extern crate grid_2d;
extern crate noise;

pub mod chunk;

use chunk::Chunk;
use grid_2d::Coord;

fn main() {
    // 3, 0 is an island
    let chunk = Chunk::new(6, Coord::new(4, 0));
    let grid = chunk.get_grid();

    for coord in grid.coords() {
        let mut value = grid.get(coord).unwrap();
        let value = value.round() as i32;
        if coord.x == 0 {
            print!("\n");
        }

        let character = match value {
            0 => ' ',
            1 => '-',
            2 => '=',
            3 => '+',
            4 => 'x',
            5 => 'å',
            6 => '%',
            7 => '$',
            8 => 'M',
            9 => '▓',
            _ => ' ',
        };

        print!("{}", character);
    }
}
