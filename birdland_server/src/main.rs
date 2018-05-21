#[macro_use]
extern crate nickel;

extern crate grid_2d;
extern crate noise;
extern crate serde_json;

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
            response.push_str(&serde_json::to_string(&grid).unwrap());
            response
        },
    );
    server.listen("127.0.0.1:6767").unwrap();
}
