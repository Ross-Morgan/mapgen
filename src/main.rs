use mapgen::{builder::noise, prelude::*};
use ndarray::Ix2;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellContents {
    #[default]
    Land,
    Water,
}

impl MapTileContent for CellContents {
    fn to_char(&self) -> char {
        match *self {
            CellContents::Land => 'X',
            CellContents::Water => ' ',
        }
    }
}

fn main() {
    let map = TileMap::<usize, CellContents, Ix2, 2>::new_2d(32, 32);
    let noise = noise(32, 32, 2.0);

    println!("{}", map.to_string());
    println!("{}", noise.iter().map(|v| v.iter().map(|v| format!("{v:.2}")).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n"));
}
