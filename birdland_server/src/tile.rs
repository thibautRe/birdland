extern crate grid_2d;

#[derive(Default, Serialize, Deserialize)]
pub struct Tile {
    // Altitude of the left-top point
    pub alt: f64,
}

impl Tile {
    pub fn new(alt: f64) -> Self {
        Tile { alt }
    }
}
