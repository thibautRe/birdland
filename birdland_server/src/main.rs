#[macro_use]
extern crate nickel;

extern crate grid_2d;
extern crate noise;

mod chunk;

use chunk::Chunk;
use grid_2d::Coord;
use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();
    server.get("/", middleware!("Hello World"));
    server.get(
        "/chunk/:x/:y",
        middleware! {|request|
            let x = request.param("x").unwrap().parse().unwrap();
            let y = request.param("y").unwrap().parse().unwrap();

            let chunk = Chunk::new(6, Coord::new(x, y));
            let grid = chunk.get_grid();
            let mut response = String::new();
            for coord in grid.coords() {
                let mut value = grid.get(coord).unwrap();
                let value = value.round() as i32;
                if coord.x == 0 {
                    response.push_str("<br />");
                }

                let character = match value {
                    0 => ' ',
                    1 => '-',
                    2 => '=',
                    3 => '+',
                    4 => 'x',
                    5 => 'a',
                    6 => '%',
                    7 => '$',
                    8 => 'M',
                    9 => '@',
                    _ => ' ',
                };

                response.push(character);
            }
            response
        },
    );
    server.listen("127.0.0.1:6767").unwrap();
}
