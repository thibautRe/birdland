extern crate grid_2d;

#[derive(Serialize, Deserialize)]
pub enum TileObjects {
    Tree,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Tile {
    /// Altitude of the left-top point
    pub alt: f64,

    /// Content of the tile
    pub obj: Option<TileObjects>,
}

impl Tile {
    pub fn new(alt: f64, obj: Option<TileObjects>) -> Self {
        Tile { alt, obj }
    }
}
