use mapgen::prelude::*;
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
    let map = TileMap::<usize, CellContents, Ix2, 2>::new_2d(10, 10);

    println!("{}", map.to_string())
}